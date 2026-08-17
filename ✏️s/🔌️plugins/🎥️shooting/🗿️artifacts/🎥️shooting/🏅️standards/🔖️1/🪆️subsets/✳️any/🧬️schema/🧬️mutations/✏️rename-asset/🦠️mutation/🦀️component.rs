//! ✏️ Shooting mutation payload — `RenameAsset`. Changes an asset's identity `name` field.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameAsset {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameAsset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "asset", kind: "rename-asset", record: "RenamedAsset" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename asset to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
