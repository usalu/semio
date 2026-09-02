//! 🧭️ Shooting mutation payload — `ChangeSceneSunAzimuth`. One of the scene's independently-settable fields.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSceneSunAzimuth {
    pub new_azimuth: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunAzimuth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-azimuth", kind: "change-scene-sun-azimuth", record: "ChangedSceneSunAzimuth" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change sun azimuth to {}", self.new_azimuth)
    }
}
