//! 🔧 `change-co2-ppm` payload — changes the Din16798 document's `co2_ppm` (CO2 concentration).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_co2_ppm::ChangeCo2Ppm;

//#region 🔖️ChangeCo2Ppm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCo2Ppm {
    pub new_co2_ppm: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCo2Ppm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "co2-ppm", kind: "change-co2-ppm", record: "ChangedCo2Ppm" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change CO2 concentration to {}", self.new_co2_ppm)
    }
}
//#endregion 🔖️ChangeCo2Ppm
