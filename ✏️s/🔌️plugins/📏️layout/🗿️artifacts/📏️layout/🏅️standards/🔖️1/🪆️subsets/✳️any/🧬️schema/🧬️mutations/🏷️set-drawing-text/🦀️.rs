//! 🏷️ `set-drawing-text` — replaces one text label on the imported plan.

use semio_s_artifact_layout_layout_model::drawing_labels;
#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutDrawingTextRow;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDrawingText {
    pub index: u32,
    pub text: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for SetDrawingText where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::set_drawing_text::SetDrawingText> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "drawing-text", kind: "set-drawing-text", record: "SetDrawingText" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_drawing_text(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_set_drawing_text(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Set drawing text to \"{}\"", self.text), &format!("Zeichnungstext auf \"{}\" setzen", self.text)) }
    fn target(&self) -> Vec<String> { vec!["background-drawing".into()] }
}

pub fn is_drawing_kind(kind: &str) -> bool {
    matches!(kind, "drawing" | "dwg" | "dxf" | "svg" | "cad" | "s.draw.drawing" | "s.stdio.dwg" | "s.stdio.dxf" | "s.stdio.svg" | "s.cad.cad")
}

pub fn first_drawing_text(snapshot: &LayoutSnapshot) -> Option<String> {
    drawing_labels(snapshot).into_iter().next()
}


pub fn drawing_text_field(index: usize) -> Option<&'static str> {
    const KEYS: [&str; 32] = ["drawingText", "drawingText.1", "drawingText.2", "drawingText.3", "drawingText.4", "drawingText.5", "drawingText.6", "drawingText.7", "drawingText.8", "drawingText.9", "drawingText.10", "drawingText.11", "drawingText.12", "drawingText.13", "drawingText.14", "drawingText.15", "drawingText.16", "drawingText.17", "drawingText.18", "drawingText.19", "drawingText.20", "drawingText.21", "drawingText.22", "drawingText.23", "drawingText.24", "drawingText.25", "drawingText.26", "drawingText.27", "drawingText.28", "drawingText.29", "drawingText.30", "drawingText.31"];
    KEYS.get(index).copied()
}

pub fn drawing_text_index(field: &str) -> Option<u32> {
    let index = if field == "drawingText" { 0 } else { field.strip_prefix("drawingText.")?.parse().ok()? };
    drawing_text_field(index as usize)?;
    Some(index)
}


pub fn diff_set_drawing_text(payload: &SetDrawingText, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if payload.text.chars().count() > 256 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Drawing text is at most 256 characters.", std::iter::empty::<String>());
    }
    if base.background_drawing.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The document has no imported plan.", std::iter::empty::<String>());
    }
    let Some(previous) = drawing_labels(base).into_iter().nth(payload.index as usize) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The imported plan has no text at that index.", std::iter::empty::<String>());
    };
    if previous == payload.text {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The drawing text is already that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { drawing_texts: vec![LayoutDrawingTextRow { index: payload.index, text: payload.text.clone() }], ..Default::default() })
}

pub fn inverse_set_drawing_text<Op: From<crate::set_drawing_text::SetDrawingText>>(payload: &SetDrawingText, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(previous) = drawing_labels(base).get(payload.index as usize).cloned() else { return Vec::new() };
    if previous == payload.text {
        return Vec::new();
    }
    vec![Op::from(SetDrawingText { index: payload.index, text: previous })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
