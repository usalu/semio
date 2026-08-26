//! 🗻 `change-annex` payload — changes the En1998 document's `annex` (national annex).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_annex::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_annex::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to \"{}\"", self.new_annex)
    }
}
//#endregion 🔖️ChangeAnnex
