//! 📸 Shooting mutation payload — `CreateShot`. Brings a new shot into existence (append-only apply).

use crate::artifacts::shooting::{ShootingShot, ShootingSnapshot};
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateShot {
    pub shot: ShootingShot,
    pub index: Option<usize>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for CreateShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "shot", kind: "create-shot", record: "CreatedShot" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create shot \"{}\"", self.shot.label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.shot.id.clone()]
    }
}
