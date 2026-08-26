//! 🧹 `change-bm` payload — changes the En1997 document's `b_m` (footing width B [m]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBM {
    pub new_b_m: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeBM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bm", kind: "change-bm", record: "ChangedBM" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_b_m::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_b_m::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change footing width B [m] to {}", self.new_b_m)
    }
}
//#endregion 🔖️ChangeBM
