//! 🖌️ `apply-paint-stroke` — one brush or eraser stroke stated as its INTENT rather than as the pixels it left
//! behind: the object and paint layer it lands on, the brush (colour, radius, hardness, opacity, eraser) and the UV
//! points of its dabs in drawing order. The pixels are derived on every application from the base layer, so the
//! stroke replays onto whatever the layer holds by then and every brush parameter stays editable in history
//! (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).

use crate::mutations::PixelRun;
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
    pub color: [u8; 4],
    pub radius: f32,
    pub hardness: f32,
    pub opacity: f32,
    pub points: Vec<[f32; 2]>,
}

impl ApplyPaintStroke {
    /// 🧪️ The payload's own invariant, independent of any document: a positive finite radius, hardness and opacity
    /// within `[0, 1]`, and at least one dab, every one of them on the texture (`u`, `v` within `[0, 1]`).
    pub fn invariant_violation(&self) -> Option<String> {
        let unit = |value: f32| value.is_finite() && (0.0..=1.0).contains(&value);
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
        for point in &self.points {
            crate::schema::stamp_brush_on(&mut after, side, point[0], point[1], self.radius, self.color, self.hardness, self.opacity, self.eraser);
        }
        Some(crate::schema::pixel_runs_from_diff(&before, &after).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect())
    }

    /// ➕️ This stroke followed by `tick` as ONE stroke — the dabs appended — when both land on the same layer with the
    /// same brush; `None` otherwise.
    pub fn then(&self, tick: &ApplyPaintStroke) -> Option<ApplyPaintStroke> {
        let brush = |stroke: &ApplyPaintStroke| (stroke.object_id.clone(), stroke.layer_index, stroke.eraser, stroke.color, stroke.radius.to_bits(), stroke.hardness.to_bits(), stroke.opacity.to_bits());
        (brush(self) == brush(tick)).then(|| ApplyPaintStroke { points: self.points.iter().chain(&tick.points).copied().collect(), ..self.clone() })
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ApplyPaintStroke {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "paint-stroke", kind: "apply-paint-stroke", record: "AppliedPaintStroke" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let count = self.points.len();
        let (dabs, tupfer) = if count == 1 { ("1 dab".to_string(), "1 Tupfer".to_string()) } else { (format!("{count} dabs"), format!("{count} Tupfern")) };
        if self.eraser {
            protocol::LocalizedLabel::native(&format!("Erase a stroke of {dabs} on layer {} of \"{}\"", self.layer_index, self.object_id), &format!("Einen Strich aus {tupfer} auf Ebene {} von \"{}\" radieren", self.layer_index, self.object_id))
        } else {
            protocol::LocalizedLabel::native(&format!("Paint a stroke of {dabs} on layer {} of \"{}\"", self.layer_index, self.object_id), &format!("Einen Strich aus {tupfer} auf Ebene {} von \"{}\" malen", self.layer_index, self.object_id))
        }
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🧪️Laws
/// ⚖️ The laws every committed `apply-paint-stroke` scenario holds, written once beside the leaf and called by each
/// scenario's `🧪️tests/<scenario>/🦀️.rs` with its committed quintet.
#[cfg(test)]
pub mod laws {
    use crate::{LowpolyDiff, LowpolyMutation, LowpolySnapshot};

    fn from_json<T: dsl::FromValue>(text: &str) -> T {
        let parsed: serde_json::Value = serde_json::from_str(text).expect("fixture json parses");
        dsl::FromValue::from_value(dsl::DslValue::from(parsed)).expect("fixture json decodes")
    }

    fn to_json<T: dsl::ToValue>(value: &T) -> serde_json::Value {
        dsl::ToValue::to_value(value).into()
    }

    fn produced(mutation: &LowpolyMutation, before: &LowpolySnapshot) -> Vec<(String, String)> {
        <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(mutation, before)
            .messages()
            .iter()
            .map(|message| (to_json(&message.level).as_str().unwrap_or_default().to_string(), message.code.0.clone()))
            .collect()
    }

    /// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
    pub fn forward(before: &str, mutation: &str, after: &str, diff: &str) {
        let (before, mutation, after): (LowpolySnapshot, LowpolyMutation, LowpolySnapshot) = (from_json(before), from_json(mutation), from_json(after));
        let raised = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &before);
        assert_eq!(to_json(raised.diff()), serde_json::from_str::<serde_json::Value>(diff).expect("diff parses"), "the produced delta is the committed 🔺️diff");
        let (applied, _) = protocol::apply_mutation(&before, &mutation).expect("the stroke applies");
        assert_eq!(applied, after, "the applied layer is the committed after-snapshot");
        let committed: LowpolyDiff = from_json(diff);
        assert_eq!(<LowpolyDiff as protocol::MutationDiff<LowpolySnapshot>>::apply(&committed, &before).expect("the committed diff applies"), after, "the committed diff alone carries before to after");
    }

    /// ↩️ The computed inverse — the overwritten pixels written back — restores `before` exactly.
    pub fn inverse_restores(before: &str, mutation: &str) {
        let (base, mutation): (LowpolySnapshot, LowpolyMutation) = (from_json(before), from_json(mutation));
        let inverse = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::inverse(&mutation, &base);
        assert_eq!(inverse.len(), 1, "a stroke inverts to ONE pixel write: {inverse:?}");
        let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
        for step in &inverse {
            snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
        }
        assert_eq!(snapshot, base, "the inverse restores the before-snapshot");
    }

    /// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
    pub fn declared_outcome(before: &str, mutation: &str, outcome: &str) {
        let (before, mutation): (LowpolySnapshot, LowpolyMutation) = (from_json(before), from_json(mutation));
        let outcome: serde_json::Value = serde_json::from_str(outcome).expect("outcome parses");
        let declared: Vec<(String, String)> = match outcome["status"].as_str() {
            Some("rejected") => vec![(if outcome["code"] == "mutation.invariant" { "fatal" } else { "error" }.to_string(), outcome["code"].as_str().unwrap_or_default().to_string())],
            _ => outcome["messages"].as_array().map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default(),
        };
        assert_eq!(produced(&mutation, &before), declared, "the raised diagnostics are the committed 🎯️outcome");
    }

    /// ⛔️ A refused or no-op stroke leaves the document byte-identical, emits the declared diagnostic and inverts to
    /// nothing that would move the document.
    pub fn refusal(before: &str, mutation: &str, after: &str, outcome: &str) {
        declared_outcome(before, mutation, outcome);
        let (base, mutation, after): (LowpolySnapshot, LowpolyMutation, LowpolySnapshot) = (from_json(before), from_json(mutation), from_json(after));
        assert_eq!(after, base, "the committed after-snapshot is the before-snapshot");
        let snapshot = protocol::apply_mutation(&base, &mutation).map_or_else(|_| base.clone(), |(next, _)| next);
        assert_eq!(snapshot, base, "a refused or no-op stroke leaves the document untouched");
    }

    /// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
    pub fn canonical(before: &str, after: &str, mutation: &str, diff: Option<&str>) {
        for text in [before, after] {
            assert_eq!(to_json(&from_json::<LowpolySnapshot>(text)), serde_json::from_str::<serde_json::Value>(text).expect("snapshot parses"), "a committed snapshot is canonical");
        }
        assert_eq!(to_json(&from_json::<LowpolyMutation>(mutation)), serde_json::from_str::<serde_json::Value>(mutation).expect("mutation parses"), "the committed mutation is canonical");
        if let Some(diff) = diff {
            assert_eq!(to_json(&from_json::<LowpolyDiff>(diff)), serde_json::from_str::<serde_json::Value>(diff).expect("diff parses"), "the committed diff is canonical");
        }
    }
}
//#endregion 🧪️Laws
