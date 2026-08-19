//! 🗑️ Assembly mutation — `DeleteRule`: removes an id-addressed adjacency rule.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteRule
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_rule(id: String) -> AssemblyMutation {
    AssemblyMutation::DeleteRule(DeleteRule { id })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for DeleteRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "rule", kind: "delete-rule", record: "DeletedRule" };

    async fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete rule \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteRule
