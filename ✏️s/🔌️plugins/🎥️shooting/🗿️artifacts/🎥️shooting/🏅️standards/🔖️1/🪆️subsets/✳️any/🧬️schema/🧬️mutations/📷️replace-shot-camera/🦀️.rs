//! 📷 Shooting mutation payload — `ReplaceShotCamera`. Overwrites the *saved* camera `shot_id` references with a new pose — a no-op (empty diff) when that shot has no saved camera. The free/live viewport camera is session-only runtime state and never reaches this mutation.

use crate::{ShootingCamera, ShootingSnapshot};
use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceShotCamera {
    pub shot_id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceShotCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "shot-camera", kind: "replace-shot-camera", record: "ReplacedShotCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace shot \"{}\" camera", self.shot_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot_id.clone()]
    }
}
