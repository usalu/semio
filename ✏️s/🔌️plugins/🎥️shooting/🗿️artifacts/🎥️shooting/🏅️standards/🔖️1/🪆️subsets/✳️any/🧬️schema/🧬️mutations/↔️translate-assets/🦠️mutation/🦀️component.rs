//! ↔ Shooting mutation payload — `DragAssets`, the bulk relative-offset gesture over multiple
//! assets (gumball drag). Delegates `diff`/`inverse` to the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ↔️DragAssets
/// ↔️ Relative `(dx, dy, dz)` offset applied to every asset in `asset_ids` — the taxonomy's plural
/// bulk-drag gesture (never a bare `Vec` arg bolted onto a singular verb).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragAssets {
    pub asset_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DragAssets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "assets", kind: "drag-assets", record: "DraggedAssets" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_drag_assets(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_drag_assets(self, base)
    }
    fn label(&self) -> String {
        format!("Drag {} asset(s)", self.asset_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.asset_ids.clone()
    }
}
//#endregion ↔️DragAssets
