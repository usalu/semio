//! 🕷️ `change-tower-is-chimney` payload — changes the En1998 document's `tower_is_chimney` (tower-is-chimney flag).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTowerIsChimney
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTowerIsChimney {
    pub new_tower_is_chimney: bool,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTowerIsChimney {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tower-is-chimney", kind: "change-tower-is-chimney", record: "ChangedTowerIsChimney" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_tower_is_chimney::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_tower_is_chimney::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tower-is-chimney flag to {}", self.new_tower_is_chimney)
    }
}
//#endregion 🔖️ChangeTowerIsChimney
