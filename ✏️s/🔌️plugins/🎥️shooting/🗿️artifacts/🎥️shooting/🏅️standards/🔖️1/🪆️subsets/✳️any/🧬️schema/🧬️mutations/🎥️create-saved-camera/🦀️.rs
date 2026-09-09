//! 🎥 Shooting mutation payload — `CreateSavedCamera`. Brings a new saved camera into existence (append-only apply).

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingSavedCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSavedCamera {
    pub saved_camera: ShootingSavedCamera,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateSavedCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "saved-camera", kind: "create-saved-camera", record: "CreatedSavedCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
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
