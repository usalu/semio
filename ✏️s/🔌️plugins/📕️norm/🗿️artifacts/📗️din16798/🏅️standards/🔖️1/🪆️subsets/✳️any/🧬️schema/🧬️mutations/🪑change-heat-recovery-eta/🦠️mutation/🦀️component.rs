//! 🔧 `change-heat-recovery-eta` payload — changes the Din16798 document's `heat_recovery_eta` (heat recovery efficiency).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHeatRecoveryEta
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHeatRecoveryEta {
    pub new_heat_recovery_eta: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHeatRecoveryEta {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "heat-recovery-eta", kind: "change-heat-recovery-eta", record: "ChangedHeatRecoveryEta" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_heat_recovery_eta::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_heat_recovery_eta::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change heat recovery efficiency to {}", self.new_heat_recovery_eta)
    }
}
//#endregion 🔖️ChangeHeatRecoveryEta
