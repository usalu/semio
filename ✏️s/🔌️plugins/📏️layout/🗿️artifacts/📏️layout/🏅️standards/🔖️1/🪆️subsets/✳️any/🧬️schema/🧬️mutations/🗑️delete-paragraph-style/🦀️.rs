//! 🏗️ delete-paragraph-style produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteParagraphStyle {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeleteParagraphStyle
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::create_paragraph_style::CreateParagraphStyle> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "paragraph-style", kind: "delete-paragraph-style", record: "DeletedParagraphStyle" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Delete paragraph style", "Absatzformat löschen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for DeleteParagraphStyle
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::create_paragraph_style::CreateParagraphStyle> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_delete_paragraph_style(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_delete_paragraph_style(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_delete_paragraph_style(payload: &DeleteParagraphStyle, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.paragraph_styles.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The paragraph style does not exist.", [payload.id.clone()]);
    };
    if base.stories.iter().any(|story| story.style_runs.iter().any(|run| run.paragraph_style_id.as_deref() == Some(payload.id.as_str()))) {
        return protocol::MutationOutcome::error("mutation.target-referenced", "The paragraph style is still referenced.", [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { paragraph_styles: Some(LayoutParagraphStylesDelta { removed: vec![LayoutParagraphStyleRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_delete_paragraph_style<Op: From<crate::create_paragraph_style::CreateParagraphStyle>>(payload: &DeleteParagraphStyle, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_delete_paragraph_style(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    let at = base.paragraph_styles.iter().position(|row| row.id == payload.id).expect("validated target");
    Ok(vec![Op::from(crate::create_paragraph_style::CreateParagraphStyle { style: base.paragraph_styles[at].clone(), index: Some(at) })])
}
