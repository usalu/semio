//! 🔧 `change-fire-duration-min` payload — changes the En1995 document's `fire_duration_min` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFireDurationMin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFireDurationMin {
    pub new_fire_duration_min: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeFireDurationMin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-duration-min", kind: "change-fire-duration-min", record: "ChangedFireDurationMin" };

    async fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_fire_duration_min::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_fire_duration_min::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change fire duration min to {:?}", self.new_fire_duration_min)
    }
}
//#endregion 🔖️ChangeFireDurationMin
