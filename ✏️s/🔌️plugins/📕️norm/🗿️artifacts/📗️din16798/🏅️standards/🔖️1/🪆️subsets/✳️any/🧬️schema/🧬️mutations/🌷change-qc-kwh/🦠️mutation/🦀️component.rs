//! 🔧 `change-qc-kwh` payload — changes the Din16798 document's `q_c_kwh` (cooling energy demand).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeQCKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeQCKwh {
    pub new_q_c_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeQCKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "qc-kwh", kind: "change-qc-kwh", record: "ChangedQCKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_q_c_kwh::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_q_c_kwh::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling energy demand to {}", self.new_q_c_kwh)
    }
}
//#endregion 🔖️ChangeQCKwh
