//! 🔨 `change-annex` payload — changes the En1996 document's `annex` (national annex).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: crate::document::AnnexChoice,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_annex::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_annex::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change national annex to {:?}", self.new_annex)
    }
}
//#endregion 🔖️ChangeAnnex
