//! 🔧 `change-n-cycles-bridge` payload — changes the En1995 document's `n_cycles_bridge` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNCyclesBridge
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNCyclesBridge {
    pub new_n_cycles_bridge: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeNCyclesBridge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-cycles-bridge", kind: "change-n-cycles-bridge", record: "ChangedNCyclesBridge" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_n_cycles_bridge::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_n_cycles_bridge::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change n cycles bridge to {:?}", self.new_n_cycles_bridge)
    }
}
//#endregion 🔖️ChangeNCyclesBridge
