//! 📋 Process3d mutation — `CreateStep` (repurposes the pre-migration `📋steps/` triad dir; glue.rs
//! path-includes this exact directory and this facet's writable boundary excludes glue.rs, so the
//! directory name stays `📋steps` — see the migration report's `sharedFileRequests` for the rename
//! once a later pass can touch `🦀️.rs`).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStep};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateStep
/// 📋 Full initial payload for a new [`ProcessStep`] inserted into the document's ordered
/// `step_payloads` timeline. `index` is FINAL-state, clamped to the timeline length (same
/// insert-at-index convention `📥️insert-array-element`/`🔀reorder-steps` already use) — steps are
/// order-meaningful, unlike the unordered `workshop.machines` set.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateStep {
    pub index: usize,
    pub step: ProcessStep,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.label)
    }

    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🔖️CreateStep
