//! 🔧 `change-heat-recovery-eta-min` payload — changes the Din16798 document's `heat_recovery_eta_min` (minimum heat recovery efficiency).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_heat_recovery_eta_min::ChangeHeatRecoveryEtaMin;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHeatRecoveryEtaMin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHeatRecoveryEtaMin {
    pub new_heat_recovery_eta_min: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHeatRecoveryEtaMin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "heat-recovery-eta-min", kind: "change-heat-recovery-eta-min", record: "ChangedHeatRecoveryEtaMin" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change minimum heat recovery efficiency to {}", self.new_heat_recovery_eta_min)
    }
}
//#endregion 🔖️ChangeHeatRecoveryEtaMin
