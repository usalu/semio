//! 📌 Shooting mutation payload — `SetActiveAsset`, a narrow addressed single-field setter on the
//! document root (taxonomy's `set` verb).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 📌️SetActiveAsset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetActiveAsset {
    pub asset_id: Option<String>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for SetActiveAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "active-asset", kind: "set-active-asset", record: "SetActiveAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff_set_active_asset(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse_set_active_asset(self, base)
    }
    fn label(&self) -> String {
        match &self.asset_id {
            Some(id) => format!("Set active asset to \"{id}\""),
            None => "Clear active asset".into(),
        }
    }
}
//#endregion 📌️SetActiveAsset
