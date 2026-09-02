//! 🔧 `change-cooling-gains-kwh` payload — changes the Din16798 document's `cooling_gains_kwh` (cooling internal gains).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_cooling_gains_kwh::ChangeCoolingGainsKwh;

//#region 🔖️ChangeCoolingGainsKwh
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCoolingGainsKwh {
    pub new_cooling_gains_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCoolingGainsKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cooling-gains-kwh", kind: "change-cooling-gains-kwh", record: "ChangedCoolingGainsKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling internal gains to {}", self.new_cooling_gains_kwh)
    }
}
//#endregion 🔖️ChangeCoolingGainsKwh
