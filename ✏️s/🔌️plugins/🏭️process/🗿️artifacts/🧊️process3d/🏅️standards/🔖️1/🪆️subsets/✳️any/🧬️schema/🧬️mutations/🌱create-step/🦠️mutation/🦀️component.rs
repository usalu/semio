//! 📋 Process3d mutation — `CreateStep` (repurposes the pre-migration `📋steps/` triad dir; glue.rs
//! path-includes this exact directory and this facet's writable boundary excludes glue.rs, so the
//! directory name stays `📋steps` — see the migration report's `sharedFileRequests` for the rename
//! once a later pass can touch `📦️glue.rs`).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStep};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateStep
/// 📋 Full initial payload for a new [`ProcessStep`] appended to the document's ordered timeline.
/// `index` is carried for label/provenance purposes only — the underlying `Process3dStepsDelta`
/// engine (`apply_steps_delta`) always appends `added` entries, matching this facet's
/// pre-migration generic-add behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStep {
    pub index: usize,
    pub step: ProcessStep,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::create_step::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::create_step::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.label)
    }

    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🔖️CreateStep
