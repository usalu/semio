//! ⏱️ Process3d mutation — `ChangeCursor` (repurposes the pre-migration `⏱️set-cursor/` triad dir —
//! glue.rs path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `⏱️set-cursor`; see the migration report's `sharedFileRequests` for the
//! rename once a later pass can touch `📦️glue.rs`).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCursor
/// ⏱️ Document-level scalar setter for the "resolved up to" playback cursor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCursor {
    pub new_resolved_up_to: Option<usize>,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeCursor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cursor", kind: "change-cursor", record: "ChangedCursor" };

    async fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::change_cursor::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::change_cursor::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        match self.new_resolved_up_to {
            Some(cursor) => format!("Move cursor to step {cursor}"),
            None => "Clear cursor".to_string(),
        }
    }
}
//#endregion 🔖️ChangeCursor
