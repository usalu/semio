//! 🔧 `change-fan-energy-reference-kwh` payload — changes the Din16798 document's `fan_energy_reference_kwh` (fan energy reference).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_fan_energy_reference_kwh::ChangeFanEnergyReferenceKwh;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFanEnergyReferenceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFanEnergyReferenceKwh {
    pub new_fan_energy_reference_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeFanEnergyReferenceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fan-energy-reference-kwh", kind: "change-fan-energy-reference-kwh", record: "ChangedFanEnergyReferenceKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fan energy reference to {}", self.new_fan_energy_reference_kwh)
    }
}
//#endregion 🔖️ChangeFanEnergyReferenceKwh
