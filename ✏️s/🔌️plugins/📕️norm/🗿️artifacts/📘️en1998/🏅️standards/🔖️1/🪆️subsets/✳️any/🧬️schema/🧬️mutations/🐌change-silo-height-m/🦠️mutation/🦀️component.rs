//! 🐌 `change-silo-height-m` payload — changes the En1998 document's `silo_height_m` (silo height [m]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSiloHeightM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSiloHeightM {
    pub new_silo_height_m: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSiloHeightM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-height-m", kind: "change-silo-height-m", record: "ChangedSiloHeightM" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_silo_height_m::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_silo_height_m::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change silo height [m] to {}", self.new_silo_height_m)
    }
}
//#endregion 🔖️ChangeSiloHeightM
