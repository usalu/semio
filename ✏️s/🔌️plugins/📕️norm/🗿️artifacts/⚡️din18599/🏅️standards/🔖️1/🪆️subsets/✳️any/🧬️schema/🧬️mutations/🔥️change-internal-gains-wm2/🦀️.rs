//! 🦘 `change-internal-gains-wm2` payload — changes the Din18599 document's `internal_gains_w_m2` (internal heat gains [W/m2]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
//#region 🔖️ChangeInternalGainsWM2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeInternalGainsWM2 {
    pub new_internal_gains_w_m2: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeInternalGainsWM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "internal-gains-wm2", kind: "change-internal-gains-wm2", record: "ChangedInternalGainsWM2" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change internal heat gains [W/m2] to {}", self.new_internal_gains_w_m2)
    }
}
//#endregion 🔖️ChangeInternalGainsWM2
