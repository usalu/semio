//! 🏗️ set-page-spread produces a structural diff and retains its semantic inverse.

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
pub struct SetPageSpread {
    pub id: String,
    pub spread_id: String,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for SetPageSpread
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::set_page_spread::SetPageSpread> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-spread", kind: "set-page-spread", record: "SetPageSpread" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Assign page spread", "Seite einem Druckbogen zuweisen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for SetPageSpread
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::set_page_spread::SetPageSpread> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_page_spread(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_set_page_spread(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_set_page_spread(payload: &SetPageSpread, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The page does not exist.", [payload.id.clone()]);
    };
    let Some(target) = base.spreads.iter().find(|spread| spread.id == payload.spread_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The spread does not exist.", [payload.spread_id.clone()]);
    };
    let mut ids: Vec<String> = target.page_ids.iter().filter(|id| **id != payload.id).cloned().collect();
    let at = payload.index.unwrap_or_else(|| if page.spread_id == payload.spread_id { target.page_ids.iter().position(|id| id == &payload.id).unwrap_or(ids.len()) } else { ids.len() });
    if at > ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The spread position is beyond the list.", [payload.id.clone()]);
    }
    ids.insert(at, payload.id.clone());
    if page.spread_id == payload.spread_id && ids == target.page_ids {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The page is already at that spread position.");
    }
    let mut modified = vec![LayoutSpreadsModification { id: target.id.clone(), patch: SpreadPatch { page_ids: Some(ids), ..Default::default() } }];
    if page.spread_id != payload.spread_id {
        if let Some(previous) = base.spreads.iter().find(|spread| spread.id == page.spread_id) {
            modified.push(LayoutSpreadsModification { id: previous.id.clone(), patch: SpreadPatch { page_ids: Some(previous.page_ids.iter().filter(|id| **id != payload.id).cloned().collect()), ..Default::default() } });
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { spread_id: Some(payload.spread_id.clone()), ..Default::default() } }], ..Default::default() }),
        spreads: Some(LayoutSpreadsDelta { modified, ..Default::default() }), ..Default::default()
    })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_set_page_spread<Op: From<crate::set_page_spread::SetPageSpread>>(payload: &SetPageSpread, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_set_page_spread(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    let page = base.pages.iter().find(|page| page.id == payload.id).expect("validated target");
    let index = base.spreads.iter().find(|spread| spread.id == page.spread_id).and_then(|spread| spread.page_ids.iter().position(|id| id == &page.id));
    Ok(vec![Op::from(SetPageSpread { id: page.id.clone(), spread_id: page.spread_id.clone(), index })])
}
