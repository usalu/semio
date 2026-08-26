//! 🔧 `change-annex` payload — changes the En1992 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `📦️glue.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the directory
//! name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see this ticket's wave2
//! report `sharedFileRequests` for the rename once a later pass can touch `📦️glue.rs`.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::set_snapshot::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::set_snapshot::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to {:?}", self.new_annex)
    }
}
//#endregion 🔖️ChangeAnnex
