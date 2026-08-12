//! 🧭️ Shooting mutation payload — `ChangeSceneSunAzimuth`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSceneSunAzimuth {
    pub new_azimuth: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunAzimuth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-azimuth", kind: "change-scene-sun-azimuth", record: "ChangedSceneSunAzimuth" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change sun azimuth to {}", self.new_azimuth)
    }
}
