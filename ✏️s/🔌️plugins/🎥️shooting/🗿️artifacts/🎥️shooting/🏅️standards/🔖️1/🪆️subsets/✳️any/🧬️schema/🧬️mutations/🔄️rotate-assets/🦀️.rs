//! 🔄 Shooting mutation payload — `RotateAssets`. The bulk axis-angle rotation gesture. Composes an `(ax, ay, az, angle)` axis-angle quaternion onto every asset in `asset_ids`.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateAssets {
    pub asset_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RotateAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "assets", kind: "rotate-assets", record: "RotatedAssets" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rotate {} asset(s)", self.asset_ids.len())
    }
    async fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
