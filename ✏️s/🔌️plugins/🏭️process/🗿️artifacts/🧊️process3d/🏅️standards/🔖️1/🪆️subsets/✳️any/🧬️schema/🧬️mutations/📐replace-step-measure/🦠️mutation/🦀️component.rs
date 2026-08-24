//! 📄 Process3d mutation — `ReplaceStepMeasure` (repurposes the pre-migration `📄set-snapshot/`
//! triad dir — glue.rs path-includes this exact directory outside this facet's writable boundary,
//! so the directory name stays `📄set-snapshot`; see the migration report's `sharedFileRequests`
//! for the rename once a later pass can touch `📦️glue.rs`).
//!
//! Whole-document snapshot replacement is BANNED by
//! `📓️taxonomy.md`/`📓️derivation-rules.md` — file-open/import/load-example now goes through
//! `store::ArtifactStore::reset`, entirely outside this `Mutation` enum. This slot's real semantic
//! replacement is `replace-step-measure`: a large structured sub-payload (`ProcessMeasure`, the
//! tool/pose geometry a step performs) swapped wholesale on one addressed step.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessMeasure};
use serde::{Deserialize, Serialize};

//#region 🔖️ReplaceStepMeasure
/// 📄 Whole-value swap of one [`ProcessStep`](crate::artifacts::process3d::ProcessStep)'s
/// `measure` — the cut/drill/attach tool geometry and pose the step performs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStepMeasure {
    pub id: String,
    pub new_measure: ProcessMeasure,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReplaceStepMeasure {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "step", kind: "replace-step-measure", record: "ReplacedStepMeasure" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::replace_step_measure::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::replace_step_measure::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace measure of step \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ReplaceStepMeasure
