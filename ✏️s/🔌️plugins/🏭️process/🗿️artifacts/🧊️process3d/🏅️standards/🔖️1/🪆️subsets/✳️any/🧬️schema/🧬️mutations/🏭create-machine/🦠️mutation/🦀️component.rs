//! 🛠️ Process3d mutation — `CreateMachine` (repurposes the pre-migration `🛠️machines/` triad dir —
//! glue.rs path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `🛠️machines`; see the migration report's `sharedFileRequests` for the
//! rename once a later pass can touch `📦️glue.rs`).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, WorkshopMachine};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateMachine
/// 🛠️ Full initial payload for a new [`WorkshopMachine`] installed into the document's workshop.
/// `index` is carried for label/provenance purposes only — the workshop's `machines` list has no
/// user-meaningful order, so the diff always appends.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMachine {
    pub index: usize,
    pub machine: WorkshopMachine,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for CreateMachine {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "machine", kind: "create-machine", record: "CreatedMachine" };

    async fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::create_machine::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::create_machine::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Create machine \"{}\"", self.machine.label)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.machine.id.clone()]
    }
}
//#endregion 🔖️CreateMachine
