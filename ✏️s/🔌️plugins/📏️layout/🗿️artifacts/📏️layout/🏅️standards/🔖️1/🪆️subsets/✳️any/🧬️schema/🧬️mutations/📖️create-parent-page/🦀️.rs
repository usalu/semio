//! 🏗️ create-parent-page produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot, ParentPage};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateParentPage {
    pub page: ParentPage,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreateParentPage
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_parent_page::DeleteParentPage> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "parent-page", kind: "create-parent-page", record: "CreatedParentPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create parent page", "Mustervorlage erstellen") }
    fn target(&self) -> Vec<String> { vec![self.page.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for CreateParentPage
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_parent_page::DeleteParentPage> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_parent_page(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_create_parent_page(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_create_parent_page(payload: &CreateParentPage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.parent_pages.iter().any(|row| row.id == payload.page.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The parent page id already exists.", [payload.page.id.clone()]);
    }
    let mut page = payload.page.clone();
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::normalize_parent_content(base, &mut page) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [page.id.clone()]);
    }
    let at = payload.index.unwrap_or(base.parent_pages.len());
    if at > base.parent_pages.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Insertion index is beyond the list.", [payload.page.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { parent_pages: Some(LayoutParentPagesDelta { inserted: vec![LayoutParentPageInsertion { index: at, row: page }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_create_parent_page<Op: From<crate::delete_parent_page::DeleteParentPage>>(payload: &CreateParentPage, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_create_parent_page(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    Ok(vec![Op::from(crate::delete_parent_page::DeleteParentPage { id: payload.page.id.clone() })])
}
