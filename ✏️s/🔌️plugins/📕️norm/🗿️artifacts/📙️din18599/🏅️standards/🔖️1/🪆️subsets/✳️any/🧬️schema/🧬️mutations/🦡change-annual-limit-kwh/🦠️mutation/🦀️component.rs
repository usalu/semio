//! 🦡 `change-annual-limit-kwh` payload — changes the Din18599 document's `annual_limit_kwh` (annual primary energy limit [kWh]).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnualLimitKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnualLimitKwh {
    pub new_annual_limit_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeAnnualLimitKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annual-limit-kwh", kind: "change-annual-limit-kwh", record: "ChangedAnnualLimitKwh" };

    async fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        crate::artifacts::din18599::mutations::change_annual_limit_kwh::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::change_annual_limit_kwh::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change annual primary energy limit [kWh] to {}", self.new_annual_limit_kwh)
    }
}
//#endregion 🔖️ChangeAnnualLimitKwh
