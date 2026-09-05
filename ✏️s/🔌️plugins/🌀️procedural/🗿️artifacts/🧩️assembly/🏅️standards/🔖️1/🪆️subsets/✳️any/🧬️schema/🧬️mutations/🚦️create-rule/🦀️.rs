//! 🌱 Assembly mutation — `CreateRule`: brings a new id-keyed adjacency rule into existence at a
//! FINAL-state insertion index.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::{AssemblyRule, AssemblySnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRule {
    pub index: usize,
    pub rule: AssemblyRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(index: usize, rule: AssemblyRule) -> AssemblyMutation {
    AssemblyMutation::CreateRule(CreateRule { index, rule })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create rule \"{}\"", self.rule.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️CreateRule
