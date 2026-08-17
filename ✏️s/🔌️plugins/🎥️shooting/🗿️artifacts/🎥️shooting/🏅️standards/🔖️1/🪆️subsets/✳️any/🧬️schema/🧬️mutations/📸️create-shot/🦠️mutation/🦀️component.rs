//! 📸 Shooting mutation payload — `CreateShot`. Brings a new shot into existence (append-only apply).

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::ShootingShot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateShot {
    pub shot: ShootingShot,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "shot", kind: "create-shot", record: "CreatedShot" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create shot \"{}\"", self.shot.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot.id.clone()]
    }
}
