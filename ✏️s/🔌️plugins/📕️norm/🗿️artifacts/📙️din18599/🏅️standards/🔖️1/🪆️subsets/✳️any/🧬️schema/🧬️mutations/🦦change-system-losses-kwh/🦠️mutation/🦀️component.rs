//! 🦦 `change-system-losses-kwh` payload — changes the Din18599 document's `system_losses_kwh` (system losses [kWh]).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSystemLossesKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSystemLossesKwh {
    pub new_system_losses_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeSystemLossesKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "system-losses-kwh", kind: "change-system-losses-kwh", record: "ChangedSystemLossesKwh" };

    fn diff(&self, base: &Din18599Snapshot) -> Din18599Diff {
        crate::artifacts::din18599::mutations::change_system_losses_kwh::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::change_system_losses_kwh::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change system losses [kWh] to {}", self.new_system_losses_kwh)
    }
}
//#endregion 🔖️ChangeSystemLossesKwh
