//! 🍄 `change-ground-type` payload — changes the En1998 document's `ground_type` (ground type).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeGroundType
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeGroundType {
    pub new_ground_type: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeGroundType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground-type", kind: "change-ground-type", record: "ChangedGroundType" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_ground_type::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_ground_type::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ground type to \"{}\"", self.new_ground_type)
    }
}
//#endregion 🔖️ChangeGroundType
