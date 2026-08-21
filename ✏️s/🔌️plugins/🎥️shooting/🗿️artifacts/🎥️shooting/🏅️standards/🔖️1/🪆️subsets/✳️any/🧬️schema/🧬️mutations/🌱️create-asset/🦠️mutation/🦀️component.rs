//! 🌱 Shooting mutation payload — `CreateAsset`. Brings a new asset into existence. `index` is descriptive of authoring intent (the append-only apply always pushes onto the end of the list).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingAsset;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAsset {
    pub asset: ShootingAsset,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "asset", kind: "create-asset", record: "CreatedAsset" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create asset \"{}\"", self.asset.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.asset.id.clone()]
    }
}
