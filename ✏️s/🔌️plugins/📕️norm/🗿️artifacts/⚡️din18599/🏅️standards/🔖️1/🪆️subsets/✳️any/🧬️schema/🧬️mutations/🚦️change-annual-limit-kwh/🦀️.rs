//! 🦡 `change-annual-limit-kwh` payload — changes the Din18599 document's `annual_limit_kwh` (annual primary energy limit [kWh]).

use crate::diff::Din18599Diff;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;
//#region 🔖️ChangeAnnualLimitKwh
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnnualLimitKwh {
    pub new_annual_limit_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeAnnualLimitKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annual-limit-kwh", kind: "change-annual-limit-kwh", record: "ChangedAnnualLimitKwh" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change annual primary energy limit [kWh] to {}", self.new_annual_limit_kwh)
    }
}
//#endregion 🔖️ChangeAnnualLimitKwh
