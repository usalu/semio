//! 💡️ Shooting mutation payload — `ChangeSceneSunIntensity`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunIntensity {
    pub new_intensity: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunIntensity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-intensity", kind: "change-scene-sun-intensity", record: "ChangedSceneSunIntensity" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun intensity to {}", self.new_intensity)
    }
}
