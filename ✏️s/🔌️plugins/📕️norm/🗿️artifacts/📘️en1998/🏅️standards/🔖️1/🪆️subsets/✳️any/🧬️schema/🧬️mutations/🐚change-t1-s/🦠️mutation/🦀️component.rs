//! 🐚 `change-t1-s` payload — changes the En1998 document's `t1_s` (fundamental period T1 [s]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeT1S
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeT1S {
    pub new_t1_s: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeT1S {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t1-s", kind: "change-t1-s", record: "ChangedT1S" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_t1_s::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_t1_s::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change fundamental period T1 [s] to {}", self.new_t1_s)
    }
}
//#endregion 🔖️ChangeT1S
