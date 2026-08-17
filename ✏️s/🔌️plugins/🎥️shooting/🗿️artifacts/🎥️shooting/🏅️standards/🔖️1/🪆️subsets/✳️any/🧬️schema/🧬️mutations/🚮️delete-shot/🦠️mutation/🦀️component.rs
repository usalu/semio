//! 🚮 Shooting mutation payload — `DeleteShot`. Removes a shot by id; inverse recreates it.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteShot {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "shot", kind: "delete-shot", record: "DeletedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete shot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
