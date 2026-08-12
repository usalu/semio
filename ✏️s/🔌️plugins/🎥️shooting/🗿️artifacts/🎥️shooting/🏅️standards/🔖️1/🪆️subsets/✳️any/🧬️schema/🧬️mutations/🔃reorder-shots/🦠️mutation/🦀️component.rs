//! 🔃 Shooting mutation payload — `ReorderShots`. Repositions a shot within the display-ordered `shots` list.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderShots {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReorderShots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "shots", kind: "reorder-shots", record: "ReorderedShots" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder shot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
