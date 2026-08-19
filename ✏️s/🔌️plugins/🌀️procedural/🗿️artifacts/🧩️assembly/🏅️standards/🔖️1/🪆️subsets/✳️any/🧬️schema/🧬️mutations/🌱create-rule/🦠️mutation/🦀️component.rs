//! 🌱 Assembly mutation — `CreateRule`: brings a new id-keyed adjacency rule into existence at a
//! FINAL-state insertion index.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::{AssemblyRule, AssemblySnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRule {
    pub index: usize,
    pub rule: AssemblyRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_rule(index: usize, rule: AssemblyRule) -> AssemblyMutation {
    AssemblyMutation::CreateRule(CreateRule { index, rule })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    async fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create rule \"{}\"", self.rule.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️CreateRule
