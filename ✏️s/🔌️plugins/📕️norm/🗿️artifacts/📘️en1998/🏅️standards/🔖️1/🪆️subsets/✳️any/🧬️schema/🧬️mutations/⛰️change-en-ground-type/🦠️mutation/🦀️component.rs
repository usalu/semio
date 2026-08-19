//! ⛰️ `change-en-ground-type` payload — changes the En1998 document's `en_ground_type` (EN ground type).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeEnGroundType
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEnGroundType {
    pub new_en_ground_type: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeEnGroundType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-ground-type", kind: "change-en-ground-type", record: "ChangedEnGroundType" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_en_ground_type::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_en_ground_type::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change EN ground type to \"{}\"", self.new_en_ground_type)
    }
}
//#endregion 🔖️ChangeEnGroundType
