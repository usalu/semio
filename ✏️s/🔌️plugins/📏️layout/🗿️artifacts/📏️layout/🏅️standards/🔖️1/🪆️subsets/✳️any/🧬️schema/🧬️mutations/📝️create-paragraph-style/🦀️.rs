//! 🏗️ create-paragraph-style produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot, ParagraphStyle};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateParagraphStyle {
    pub style: ParagraphStyle,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreateParagraphStyle
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_paragraph_style::DeleteParagraphStyle> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "paragraph-style", kind: "create-paragraph-style", record: "CreatedParagraphStyle" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create paragraph style", "Absatzformat erstellen") }
    fn target(&self) -> Vec<String> { vec![self.style.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for CreateParagraphStyle
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_paragraph_style::DeleteParagraphStyle> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_paragraph_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_create_paragraph_style(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_create_paragraph_style(payload: &CreateParagraphStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if payload.style.id.trim().is_empty() || payload.style.name.trim().is_empty() || payload.style.font_family.trim().is_empty() || !payload.style.font_size.is_finite() || payload.style.font_size <= 0.0 || !payload.style.leading.is_finite() || payload.style.leading <= 0.0 || !payload.style.tracking.is_finite() || payload.style.font_weight == 0 || !matches!(payload.style.alignment.as_str(), "left" | "center" | "right" | "justify") {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The new paragraph style has invalid fields.", std::iter::empty::<String>());
    }
    if base.paragraph_styles.iter().any(|row| row.id == payload.style.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The paragraph style id already exists.", [payload.style.id.clone()]);
    }
    let at = payload.index.unwrap_or(base.paragraph_styles.len());
    if at > base.paragraph_styles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Insertion index is beyond the list.", [payload.style.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { paragraph_styles: Some(LayoutParagraphStylesDelta { inserted: vec![LayoutParagraphStyleInsertion { index: at, row: payload.style.clone() }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_create_paragraph_style<Op: From<crate::delete_paragraph_style::DeleteParagraphStyle>>(payload: &CreateParagraphStyle, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_create_paragraph_style(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    Ok(vec![Op::from(crate::delete_paragraph_style::DeleteParagraphStyle { id: payload.style.id.clone() })])
}
