//! ↔️ Shooting mutation payload — `DragAssets`. The bulk relative-offset gesture over multiple assets (gumball drag). Relative `(dx, dy, dz)` offset applied to every asset in `asset_ids` — the taxonomy's plural bulk-drag gesture.

use crate::ShootingSnapshot;
use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DragAssets {
    pub asset_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DragAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "assets", kind: "drag-assets", record: "DraggedAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Drag {} asset(s)", self.asset_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
