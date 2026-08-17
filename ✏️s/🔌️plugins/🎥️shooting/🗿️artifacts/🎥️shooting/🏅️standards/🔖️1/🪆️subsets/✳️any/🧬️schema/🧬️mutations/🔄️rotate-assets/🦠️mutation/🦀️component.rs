//! 🔄 Shooting mutation payload — `RotateAssets`. The bulk axis-angle rotation gesture. Composes an `(ax, ay, az, angle)` axis-angle quaternion onto every asset in `asset_ids`.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RotateAssets {
    pub asset_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RotateAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "assets", kind: "rotate-assets", record: "RotatedAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate {} asset(s)", self.asset_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
