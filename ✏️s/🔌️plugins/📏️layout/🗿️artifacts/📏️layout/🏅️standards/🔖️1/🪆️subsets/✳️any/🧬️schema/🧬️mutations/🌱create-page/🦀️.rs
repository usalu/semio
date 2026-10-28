//! 🌱️ `create-page` — brings a new {@link Page} into existence in the id-keyed `pages` collection.

use crate::delete_page;
use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, LayoutSpreadsDelta, LayoutSpreadsModification, SpreadPatch};
use semio_s_artifact_layout_layout_diff::LayoutPageInsertion;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::Page;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🌱️CreatePage
/// 🌱️ `index` inserts among pages; absence appends. Spread membership is appended atomically.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreatePage {
    pub page: Page,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreatePage where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::delete_page::DeletePage> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "page", kind: "create-page", record: "CreatedPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create page \"{}\"", self.page.name), &format!("Seite \"{}\" erstellen", self.page.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for CreatePage where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::delete_page::DeletePage> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_page(self, base)?
    
    })
}
}
//#endregion 🌱️CreatePage

//#region 🌱️CreatePage
pub fn diff_create_page(payload: &CreatePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.pages.iter().any(|page| page.id == payload.page.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A page with id \"{}\" already exists.", payload.page.id), [payload.page.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.pages.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.pages.len()), [&payload.page.id.to_string()]);
    }
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::validate_page_creation(base, &payload.page) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.page.id.clone()]);
    }
    let spread = base.spreads.iter().find(|spread| spread.id == payload.page.spread_id).expect("validated spread");
    let mut page_ids = spread.page_ids.clone();
    page_ids.push(payload.page.id.clone());
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { inserted: vec![LayoutPageInsertion { index: payload.index.unwrap_or(base.pages.len()), row: payload.page.clone() }], ..Default::default() }),
        spreads: Some(LayoutSpreadsDelta { modified: vec![LayoutSpreadsModification { id: spread.id.clone(), patch: SpreadPatch { page_ids: Some(page_ids), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🌱️CreatePage

//#region 🌱️CreatePage
pub fn inverse_create_page<Op: From<crate::delete_page::DeletePage>>(payload: &CreatePage, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_create_page(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    Ok((|| {
    vec![Op::from(delete_page::DeletePage { id: payload.page.id.clone() })]

    })())
}
//#endregion 🌱️CreatePage

