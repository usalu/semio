//! 🗑️ `delete-page` — removes a {@link Page} by id; inverse recreates it via `create-page`.

use crate::create_page;
use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, LayoutSpreadsDelta, LayoutSpreadsModification, SpreadPatch};
use semio_s_artifact_layout_layout_diff::LayoutPageRemoval;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeletePage
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeletePage {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeletePage where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_page::CreatePage> + From<crate::set_page_spread::SetPageSpread> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "page", kind: "delete-page", record: "DeletedPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete page \"{}\"", self.id), &format!("Seite \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for DeletePage where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::create_page::CreatePage> + From<crate::set_page_spread::SetPageSpread> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_page(self, base)?
    
    })
}
}
//#endregion 🗑️DeletePage

//#region 🗑️DeletePage
pub fn diff_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.pages.iter().position(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let page = &base.pages[at];
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::validate_frame_removal(base, page.frames.iter().map(|frame| frame.id())) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.id.clone()]);
    }
    let Some(spread) = base.spreads.iter().find(|spread| spread.id == page.spread_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The owning spread does not exist.", [page.spread_id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id: payload.id.clone(), index: at }], ..Default::default() }),
        spreads: Some(LayoutSpreadsDelta { modified: vec![LayoutSpreadsModification { id: spread.id.clone(), patch: SpreadPatch { page_ids: Some(spread.page_ids.iter().filter(|id| **id != payload.id).cloned().collect()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🗑️DeletePage

//#region 🗑️DeletePage
pub fn inverse_delete_page<Op: From<crate::create_page::CreatePage> + From<crate::set_page_spread::SetPageSpread>>(payload: &DeletePage, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_delete_page(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    let index = base.pages.iter().position(|page| page.id == payload.id).expect("validated page");
    let page = &base.pages[index];
    let spread = base.spreads.iter().find(|spread| spread.id == page.spread_id).expect("validated spread");
    let spread_index = spread.page_ids.iter().position(|id| id == &page.id);
    let mut inverse = Vec::new();
    if spread_index != spread.page_ids.len().checked_sub(1) {
        inverse.push(Op::from(crate::set_page_spread::SetPageSpread { id: page.id.clone(), spread_id: page.spread_id.clone(), index: spread_index }));
    }
    inverse.push(Op::from(create_page::CreatePage { page: page.clone(), index: Some(index) }));
    Ok(inverse)
}
//#endregion 🗑️DeletePage
