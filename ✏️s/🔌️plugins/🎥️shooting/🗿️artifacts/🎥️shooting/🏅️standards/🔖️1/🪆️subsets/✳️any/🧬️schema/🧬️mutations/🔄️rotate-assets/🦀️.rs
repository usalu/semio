//! 🔄 Shooting mutation payload — `RotateAssets`. The bulk axis-angle rotation gesture. Composes an `(ax, ay, az, angle)` axis-angle quaternion onto every asset in `asset_ids`.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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
