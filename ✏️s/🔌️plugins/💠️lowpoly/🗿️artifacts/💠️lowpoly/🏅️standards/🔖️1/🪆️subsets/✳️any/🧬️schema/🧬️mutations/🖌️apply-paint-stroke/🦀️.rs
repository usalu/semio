//! 🖌️ `apply-paint-stroke` — one brush or eraser stroke stated as its INTENT rather than as the pixels it left
//! behind: the object and paint layer it lands on, the brush (sRGB colour, radius, hardness, opacity, eraser) and the UV
//! points of its dabs in drawing order. The pixels are derived on every application from the base layer, so the
//! stroke replays onto whatever the layer holds by then and every brush parameter stays editable in history
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).

use crate::schema::PixelRun;
use crate::{LowpolyMutation, LowpolySnapshot, LOWPOLY_PAINT_TEXTURE_SIZE};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ApplyPaintStroke {
    pub object_id: String,
    pub layer_index: usize,
    pub eraser: bool,
    pub color: [f32; 3],
    pub radius: f32,
    pub hardness: f32,
    pub opacity: f32,
    pub points: Vec<[f32; 2]>,
}

impl ApplyPaintStroke {
    /// 🧪️ The payload's own invariant, independent of any document: every colour channel, the hardness and the opacity
    /// within `[0, 1]`, a positive finite radius, and at least one dab, every one of them on the texture (`u`, `v` within
    /// `[0, 1]`).
    pub fn invariant_violation(&self) -> Option<String> {
        let unit = |value: f32| value.is_finite() && (0.0..=1.0).contains(&value);
        if !self.color.iter().all(|channel| unit(*channel)) {
            return Some(format!("Every colour channel must lie within [0, 1], got {:?}.", self.color));
        }
        if !(self.radius.is_finite() && self.radius > 0.0) {
            return Some(format!("The brush radius must be positive and finite, got {}.", self.radius));
        }
        if !unit(self.hardness) || !unit(self.opacity) {
            return Some(format!("Brush hardness and opacity must lie within [0, 1], got {} and {}.", self.hardness, self.opacity));
        }
        if self.points.is_empty() {
            return Some("A stroke needs at least one dab.".into());
        }
        self.points.iter().find(|point| !unit(point[0]) || !unit(point[1])).map(|point| format!("Every dab must lie on the texture, got ({}, {}).", point[0], point[1]))
    }

    /// 🎨️ The brush colour as the bytes it writes: every channel scaled to `0..=255` and rounded half away from zero.
    pub fn color_bytes(&self) -> [u8; 3] {
        self.color.map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    /// 🎨️ The pixel runs this stroke writes into `base`'s layer: every dab stamped in order onto the layer's buffer (a
    /// never-painted layer reads as the opaque-white texture), then diffed per RGBA pixel against the untouched
    /// buffer. `None` when the object or layer is missing or the buffer is not a square RGBA texture.
    pub fn runs(&self, base: &LowpolySnapshot) -> Option<Vec<PixelRun>> {
        let layer = base.objects.iter().find(|object| object.id == self.object_id)?.paint_layers.get(self.layer_index)?;
        let before = layer.materialized_pixels();
        let side = if layer.pixels.is_empty() { LOWPOLY_PAINT_TEXTURE_SIZE } else { (before.len() / 4).isqrt() };
        if before.len() != side * side * 4 {
            return None;
        }
        let mut after = before.clone();
        let [red, green, blue] = self.color_bytes();
        for point in &self.points {
            crate::schema::stamp_brush_on(&mut after, side, point[0], point[1], self.radius, [red, green, blue, u8::MAX], self.hardness, self.opacity, self.eraser);
        }
        Some(crate::schema::pixel_runs_from_diff(&before, &after).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect())
    }

    /// ➕️ This stroke followed by `tick` as ONE stroke — the dabs appended — when both land on the same layer with the
    /// same brush; `None` otherwise.
    pub fn then(&self, tick: &ApplyPaintStroke) -> Option<ApplyPaintStroke> {
        let brush = |stroke: &ApplyPaintStroke| (stroke.object_id.clone(), stroke.layer_index, stroke.eraser, stroke.color.map(f32::to_bits), stroke.radius.to_bits(), stroke.hardness.to_bits(), stroke.opacity.to_bits());
        (brush(self) == brush(tick)).then(|| ApplyPaintStroke { points: self.points.iter().chain(&tick.points).copied().collect(), ..self.clone() })
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ApplyPaintStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "paint-stroke", kind: "apply-paint-stroke", record: "AppliedPaintStroke" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let count = self.points.len();
        let (dabs, tupfer) = if count == 1 { ("1 dab".to_string(), "1 Tupfer".to_string()) } else { (format!("{count} dabs"), format!("{count} Tupfern")) };
        if self.eraser {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Erase a stroke of {dabs} on layer {} of \"{}\"", self.layer_index, self.object_id), &format!("Einen Strich aus {tupfer} auf Ebene {} von \"{}\" radieren", self.layer_index, self.object_id))
        } else {
            semio_framework_ui_locale::LocalizedLabel::native(&format!("Paint a stroke of {dabs} on layer {} of \"{}\"", self.layer_index, self.object_id), &format!("Einen Strich aus {tupfer} auf Ebene {} von \"{}\" malen", self.layer_index, self.object_id))
        }
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🧪️Laws
#[cfg(test)]
pub use crate::mutations::laws;
//#endregion 🧪️Laws
