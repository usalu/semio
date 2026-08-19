//! 🔧 `change-t-op-c` payload — changes the Din16798 document's `t_op_c` (operative temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTOpC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTOpC {
    pub new_t_op_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeTOpC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t-op-c", kind: "change-t-op-c", record: "ChangedTOpC" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_t_op_c::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_t_op_c::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change operative temperature to {}", self.new_t_op_c)
    }
}
//#endregion 🔖️ChangeTOpC
