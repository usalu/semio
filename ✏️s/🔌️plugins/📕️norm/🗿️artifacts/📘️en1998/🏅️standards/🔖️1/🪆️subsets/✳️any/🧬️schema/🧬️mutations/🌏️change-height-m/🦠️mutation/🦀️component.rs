//! 🌏️ `change-height-m` payload — changes the En1998 document's `height_m` (building height [m]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHeightM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHeightM {
    pub new_height_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeHeightM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "height-m", kind: "change-height-m", record: "ChangedHeightM" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_height_m::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_height_m::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change building height [m] to {}", self.new_height_m)
    }
}
//#endregion 🔖️ChangeHeightM
