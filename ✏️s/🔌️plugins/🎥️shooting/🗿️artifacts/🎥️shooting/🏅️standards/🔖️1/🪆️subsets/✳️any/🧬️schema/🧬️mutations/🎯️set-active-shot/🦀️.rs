//! 🎯 Shooting mutation payload — `SetActiveShot`. A narrow addressed single-field setter on the document root (taxonomy's `set` verb; NOT the banned whole-document `set-snapshot`).

use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::mutations::ShootingMutation;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActiveShot {
    pub shot_id: Option<String>,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for SetActiveShot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "active-shot", kind: "set-active-shot", record: "SetActiveShot" };
    async fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        match &self.shot_id {
            Some(id) => format!("Set active shot to \"{id}\""),
            None => "Clear active shot".into(),
        }
    }
}
