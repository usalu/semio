//! 🔧 `change-cooling-delta-th` payload — changes the Din16798 document's `cooling_delta_t_h` (cooling degree hours).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCoolingDeltaTH
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCoolingDeltaTH {
    pub new_cooling_delta_t_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCoolingDeltaTH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cooling-delta-th", kind: "change-cooling-delta-th", record: "ChangedCoolingDeltaTH" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_cooling_delta_t_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_cooling_delta_t_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling degree hours to {}", self.new_cooling_delta_t_h)
    }
}
//#endregion 🔖️ChangeCoolingDeltaTH
