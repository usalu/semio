//! 🔧 `change-load-duration` payload — changes the En1995 document's `load_duration` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLoadDuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLoadDuration {
    pub new_load_duration: String,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeLoadDuration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "load-duration", kind: "change-load-duration", record: "ChangedLoadDuration" };

    async fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_load_duration::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_load_duration::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change load duration to {:?}", self.new_load_duration)
    }
}
//#endregion 🔖️ChangeLoadDuration
