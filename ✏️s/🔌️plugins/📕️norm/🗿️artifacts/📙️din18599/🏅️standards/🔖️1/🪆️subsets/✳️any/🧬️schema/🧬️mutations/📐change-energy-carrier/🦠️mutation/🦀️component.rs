//! 📐 `change-energy-carrier` payload — changes the Din18599 document's `energy_carrier` (energy carrier).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeEnergyCarrier
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEnergyCarrier {
    pub new_energy_carrier: String,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeEnergyCarrier {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "energy-carrier", kind: "change-energy-carrier", record: "ChangedEnergyCarrier" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        crate::artifacts::din18599::mutations::change_energy_carrier::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::change_energy_carrier::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change energy carrier to \"{}\"", self.new_energy_carrier)
    }
}
//#endregion 🔖️ChangeEnergyCarrier
