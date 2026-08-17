//! 🧽 `change-c-kpa` payload — changes the En1997 document's `c_kpa` (cohesion c [kPa]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCKpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCKpa {
    pub new_c_kpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeCKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "c-kpa", kind: "change-c-kpa", record: "ChangedCKpa" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_c_kpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_c_kpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cohesion c [kPa] to {}", self.new_c_kpa)
    }
}
//#endregion 🔖️ChangeCKpa
