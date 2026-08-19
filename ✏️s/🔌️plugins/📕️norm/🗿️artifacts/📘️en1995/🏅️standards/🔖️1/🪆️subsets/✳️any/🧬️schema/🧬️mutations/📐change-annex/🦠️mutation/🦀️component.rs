//! 🔧 `change-annex` payload — changes the En1995 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `📦️glue.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the directory
//! name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see this ticket's wave2
//! report `sharedFileRequests` for the rename once a later pass can touch `📦️glue.rs` (mirrors the
//! en1990/en1992 precedent).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    async fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::set_snapshot::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::set_snapshot::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change national annex to {}", self.new_annex.label())
    }
}
//#endregion 🔖️ChangeAnnex
