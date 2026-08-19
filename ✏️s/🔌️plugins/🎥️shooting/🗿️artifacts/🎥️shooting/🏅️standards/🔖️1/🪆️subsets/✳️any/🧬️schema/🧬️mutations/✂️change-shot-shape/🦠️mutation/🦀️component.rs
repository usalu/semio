//! ✂️ Shooting mutation payload — `ChangeShotShape`. Sets a shot's crop `shape`.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeShotShape {
    pub id: String,
    pub new_shape: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotShape {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-shape", kind: "change-shot-shape", record: "ChangedShotShape" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change shot \"{}\" shape", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
