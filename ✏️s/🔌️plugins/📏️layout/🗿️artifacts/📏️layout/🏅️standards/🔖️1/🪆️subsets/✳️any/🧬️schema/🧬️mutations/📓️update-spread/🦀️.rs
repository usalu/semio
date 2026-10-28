//! 📓 `update-spread` — renames a spread.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::{LayoutSpreadsDelta, SpreadPatch};
use semio_s_artifact_layout_layout_diff::LayoutSpreadsModification;
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
pub struct UpdateSpread {
    pub id: String,
    pub name: String,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateSpread where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_spread::UpdateSpread> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "spread", kind: "update-spread", record: "UpdatedSpread" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update spread \"{}\"", self.name), &format!("Druckbogen \"{}\" aktualisieren", self.name)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for UpdateSpread where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_spread::UpdateSpread> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_spread(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_update_spread(self, base)? 
    })
}
}

pub fn diff_update_spread(payload: &UpdateSpread, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(spread) = base.spreads.iter().find(|spread| spread.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Spread \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A spread needs a name.", std::iter::empty::<String>());
    }
    if spread.name == payload.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Spread is already named that.");
    }
    protocol::MutationOutcome::new(LayoutDiff { spreads: Some(LayoutSpreadsDelta { modified: vec![LayoutSpreadsModification { id: payload.id.clone(), patch: SpreadPatch { name: Some(payload.name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_update_spread<Op: From<crate::update_spread::UpdateSpread>>(payload: &UpdateSpread, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(spread) = base.spreads.iter().find(|spread| spread.id == payload.id) else { return Vec::new() };
    vec![Op::from(UpdateSpread { id: spread.id.clone(), name: spread.name.clone() })]

    })())
}

