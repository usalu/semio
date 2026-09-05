//! 🔧 `change-heat-recovery-eta` payload — changes the Din16798 document's `heat_recovery_eta` (heat recovery efficiency).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeHeatRecoveryEta
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHeatRecoveryEta {
    pub new_heat_recovery_eta: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHeatRecoveryEta {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "heat-recovery-eta", kind: "change-heat-recovery-eta", record: "ChangedHeatRecoveryEta" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heat recovery efficiency to {}", self.new_heat_recovery_eta)
    }
}
//#endregion 🔖️ChangeHeatRecoveryEta
