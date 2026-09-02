//! 🔧 `change-cooling-utilization-factor` payload — changes the Din16798 document's `cooling_utilization_factor` (cooling gain utilization factor).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_cooling_utilization_factor::ChangeCoolingUtilizationFactor;

//#region 🔖️ChangeCoolingUtilizationFactor
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCoolingUtilizationFactor {
    pub new_cooling_utilization_factor: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCoolingUtilizationFactor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cooling-utilization-factor", kind: "change-cooling-utilization-factor", record: "ChangedCoolingUtilizationFactor" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling gain utilization factor to {}", self.new_cooling_utilization_factor)
    }
}
//#endregion 🔖️ChangeCoolingUtilizationFactor
