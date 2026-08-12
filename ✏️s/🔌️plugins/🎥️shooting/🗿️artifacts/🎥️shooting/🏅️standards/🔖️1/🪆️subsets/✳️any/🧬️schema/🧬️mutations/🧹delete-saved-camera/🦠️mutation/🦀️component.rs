//! 🧹 Shooting mutation payload — `DeleteSavedCamera`. Removes a saved camera by id; inverse recreates it.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSavedCamera {
    pub id: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for DeleteSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "saved-camera", kind: "delete-saved-camera", record: "DeletedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete saved camera \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
