//! 🏗️ create-spread produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot, Spread};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateSpread {
    pub spread: Spread,
    pub index: Option<usize>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for CreateSpread
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_spread::DeleteSpread> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "spread", kind: "create-spread", record: "CreatedSpread" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create spread", "Druckbogen erstellen") }
    fn target(&self) -> Vec<String> { vec![self.spread.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for CreateSpread
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::delete_spread::DeleteSpread> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_create_spread(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_create_spread(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_create_spread(payload: &CreateSpread, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if payload.spread.id.trim().is_empty() || payload.spread.name.trim().is_empty() || !payload.spread.page_ids.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The new spread has invalid fields.", std::iter::empty::<String>());
    }
    if base.spreads.iter().any(|row| row.id == payload.spread.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "The spread id already exists.", [payload.spread.id.clone()]);
    }
    let at = payload.index.unwrap_or(base.spreads.len());
    if at > base.spreads.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Insertion index is beyond the list.", [payload.spread.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { spreads: Some(LayoutSpreadsDelta { inserted: vec![LayoutSpreadInsertion { index: at, row: payload.spread.clone() }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_create_spread<Op: From<crate::delete_spread::DeleteSpread>>(payload: &CreateSpread, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_create_spread(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    Ok(vec![Op::from(crate::delete_spread::DeleteSpread { id: payload.spread.id.clone() })])
}
