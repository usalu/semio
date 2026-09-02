//! 🏷️ Shooting mutation payload — `RenameShot`. Changes a shot's identity `label` field.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameShot {
    pub id: String,
    pub new_label: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for RenameShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "shot", kind: "rename-shot", record: "RenamedShot" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename shot to \"{}\"", self.new_label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
