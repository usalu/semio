//! 🏷️ Shooting mutation payload — `RenameShot`. Changes a shot's identity `label` field.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameShot {
    pub id: String,
    pub new_label: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "shot", kind: "rename-shot", record: "RenamedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename shot to \"{}\"", self.new_label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
