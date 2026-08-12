//! ⬅️ `change-area-mm2` payload — changes the En1996 document's `area_mm2` (cross-section area [mm2]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAreaMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAreaMm2 {
    pub new_area_mm2: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeAreaMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "area-mm2", kind: "change-area-mm2", record: "ChangedAreaMm2" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_area_mm2::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_area_mm2::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cross-section area [mm2] to {}", self.new_area_mm2)
    }
}
//#endregion 🔖️ChangeAreaMm2
