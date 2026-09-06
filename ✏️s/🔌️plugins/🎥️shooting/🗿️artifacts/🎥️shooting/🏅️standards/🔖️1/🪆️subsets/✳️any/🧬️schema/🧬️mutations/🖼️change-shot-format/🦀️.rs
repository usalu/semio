//! 🖼️ Shooting mutation payload — `ChangeShotFormat`. Sets a shot's export `format`.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeShotFormat {
    pub id: String,
    pub new_format: String,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeShotFormat {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "shot-format", kind: "change-shot-format", record: "ChangedShotFormat" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change shot \"{}\" format", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
