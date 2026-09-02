//! 🦥 `change-solar-gains-kwh` payload — changes the Din18599 document's `solar_gains_kwh` (solar heat gains [kWh]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::mutations::change_solar_gains_kwh::ChangeSolarGainsKwh;

//#region 🔖️ChangeSolarGainsKwh
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSolarGainsKwh {
    pub new_solar_gains_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeSolarGainsKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-gains-kwh", kind: "change-solar-gains-kwh", record: "ChangedSolarGainsKwh" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change solar heat gains [kWh] to {}", self.new_solar_gains_kwh)
    }
}
//#endregion 🔖️ChangeSolarGainsKwh
