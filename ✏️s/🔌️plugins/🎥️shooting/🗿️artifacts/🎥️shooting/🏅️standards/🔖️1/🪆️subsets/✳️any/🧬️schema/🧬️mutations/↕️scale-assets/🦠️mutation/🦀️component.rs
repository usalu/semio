//! ↕️ Shooting mutation payload — `ScaleAssets`. The bulk multiplicative-scale gesture. Multiplies every asset in `asset_ids`' current per-axis scale by `(sx, sy, sz)`.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAssets {
    pub asset_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ScaleAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "assets", kind: "scale-assets", record: "ScaledAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale {} asset(s)", self.asset_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
