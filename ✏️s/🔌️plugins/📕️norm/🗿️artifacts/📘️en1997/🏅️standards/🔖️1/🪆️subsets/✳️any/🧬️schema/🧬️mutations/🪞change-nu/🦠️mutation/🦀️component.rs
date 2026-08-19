//! 🪞 `change-nu` payload — changes the En1997 document's `nu` (Poisson's ratio nu).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNu
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNu {
    pub new_nu: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeNu {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "nu", kind: "change-nu", record: "ChangedNu" };

    async fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_nu::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_nu::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change Poisson's ratio nu to {}", self.new_nu)
    }
}
//#endregion 🔖️ChangeNu
