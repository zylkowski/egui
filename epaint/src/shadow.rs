use super::*;

/// The color and fuzziness of a fuzzy shape.
/// Can be used for a rectangular shadow with a soft penumbra.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Shadow {
    /// The shadow extends this much outside the rect.
    /// The size of the fuzzy penumbra.
    pub extrusion: f32,

    /// Color of the opaque center of the shadow.
    pub color: Rgba,
}

impl Shadow {
    /// Tooltips, menus, ...
    pub fn small_dark() -> Self {
        Self {
            extrusion: 16.0,
            color: Color32::from_black_alpha(96).into(),
        }
    }

    /// Tooltips, menus, ...
    pub fn small_light() -> Self {
        Self {
            extrusion: 16.0,
            color: Color32::from_black_alpha(32).into(),
        }
    }

    /// Subtle and nice on dark backgrounds
    pub fn big_dark() -> Self {
        Self {
            extrusion: 32.0,
            color: Color32::from_black_alpha(96).into(),
        }
    }

    /// Subtle and nice on white backgrounds
    pub fn big_light() -> Self {
        Self {
            extrusion: 32.0,
            color: Color32::from_black_alpha(40).into(),
        }
    }

    pub fn tessellate(&self, rect: emath::Rect, corner_radius: f32) -> Mesh {
        // tessellator.clip_rect = clip_rect; // TODO: culling

        let Self { extrusion, color } = *self;

        use crate::tessellator::*;
        let rect = PaintRect {
            rect: rect.expand(0.5 * extrusion),
            corner_radius: corner_radius + 0.5 * extrusion,
            fill: color,
            stroke: Default::default(),
        };
        let mut tessellator = Tessellator::from_options(TessellationOptions {
            aa_size: extrusion,
            anti_alias: true,
            ..Default::default()
        });
        let mut mesh = Mesh::default();
        tessellator.tessellate_rect(&rect, &mut mesh);
        mesh
    }
}
