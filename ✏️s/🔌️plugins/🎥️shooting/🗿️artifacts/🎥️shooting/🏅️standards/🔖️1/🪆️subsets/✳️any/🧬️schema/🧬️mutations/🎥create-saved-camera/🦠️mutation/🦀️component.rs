//! 🎥 Shooting mutation payload — `CreateSavedCamera`. Brings a new saved camera into existence (append-only apply).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::ShootingSavedCamera;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSavedCamera {
    pub saved_camera: ShootingSavedCamera,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "saved-camera", kind: "create-saved-camera", record: "CreatedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> ShootingDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create saved camera \"{}\"", self.saved_camera.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.saved_camera.id.clone()]
    }
}
