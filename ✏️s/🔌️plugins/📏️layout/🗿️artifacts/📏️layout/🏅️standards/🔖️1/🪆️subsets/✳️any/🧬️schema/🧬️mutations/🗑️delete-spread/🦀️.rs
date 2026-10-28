//! 🏗️ delete-spread produces a structural diff and retains its semantic inverse.

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
pub struct DeleteSpread {
    pub id: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for DeleteSpread
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::create_spread::CreateSpread> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "spread", kind: "delete-spread", record: "DeletedSpread" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Delete spread", "Druckbogen löschen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for DeleteSpread
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::create_spread::CreateSpread> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_delete_spread(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_delete_spread(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_delete_spread(payload: &DeleteSpread, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.spreads.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The spread does not exist.", [payload.id.clone()]);
    };
    if !base.spreads[at].page_ids.is_empty() || base.pages.iter().any(|page| page.spread_id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-referenced", "The spread is still referenced.", [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { spreads: Some(LayoutSpreadsDelta { removed: vec![LayoutSpreadRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_delete_spread<Op: From<crate::create_spread::CreateSpread>>(payload: &DeleteSpread, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_delete_spread(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    let at = base.spreads.iter().position(|row| row.id == payload.id).expect("validated target");
    Ok(vec![Op::from(crate::create_spread::CreateSpread { spread: base.spreads[at].clone(), index: Some(at) })])
}
