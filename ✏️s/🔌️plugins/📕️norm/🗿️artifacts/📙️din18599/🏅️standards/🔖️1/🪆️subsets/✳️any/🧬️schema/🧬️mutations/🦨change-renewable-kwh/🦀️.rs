//! 🦨 `change-renewable-kwh` payload — changes the Din18599 document's `renewable_kwh` (renewable energy contribution [kWh]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::mutations::change_renewable_kwh::ChangeRenewableKwh;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRenewableKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRenewableKwh {
    pub new_renewable_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeRenewableKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "renewable-kwh", kind: "change-renewable-kwh", record: "ChangedRenewableKwh" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change renewable energy contribution [kWh] to {}", self.new_renewable_kwh)
    }
}
//#endregion 🔖️ChangeRenewableKwh
