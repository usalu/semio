//! 🌡️ `change-unit` payload — changes the En1996 document's `unit` (masonry unit material).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeUnit
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUnit {
    pub new_unit: String,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeUnit {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "unit", kind: "change-unit", record: "ChangedUnit" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_unit::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_unit::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change masonry unit material to \"{}\"", self.new_unit)
    }
}
//#endregion 🔖️ChangeUnit
