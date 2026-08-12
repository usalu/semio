//! 🔧 `change-fan-qvm3-s` payload — changes the Din16798 document's `fan_q_v_m3_s` (fan volume flow).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFanQVM3S
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFanQVM3S {
    pub new_fan_q_v_m3_s: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeFanQVM3S {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fan-qvm3-s", kind: "change-fan-qvm3-s", record: "ChangedFanQVM3S" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_fan_q_v_m3_s::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_fan_q_v_m3_s::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change fan volume flow to {}", self.new_fan_q_v_m3_s)
    }
}
//#endregion 🔖️ChangeFanQVM3S
