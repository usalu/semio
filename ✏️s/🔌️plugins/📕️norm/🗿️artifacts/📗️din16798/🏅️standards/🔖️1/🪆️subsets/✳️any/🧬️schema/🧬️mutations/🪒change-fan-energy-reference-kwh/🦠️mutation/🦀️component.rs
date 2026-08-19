//! 🔧 `change-fan-energy-reference-kwh` payload — changes the Din16798 document's `fan_energy_reference_kwh` (fan energy reference).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFanEnergyReferenceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFanEnergyReferenceKwh {
    pub new_fan_energy_reference_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeFanEnergyReferenceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fan-energy-reference-kwh", kind: "change-fan-energy-reference-kwh", record: "ChangedFanEnergyReferenceKwh" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_fan_energy_reference_kwh::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_fan_energy_reference_kwh::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change fan energy reference to {}", self.new_fan_energy_reference_kwh)
    }
}
//#endregion 🔖️ChangeFanEnergyReferenceKwh
