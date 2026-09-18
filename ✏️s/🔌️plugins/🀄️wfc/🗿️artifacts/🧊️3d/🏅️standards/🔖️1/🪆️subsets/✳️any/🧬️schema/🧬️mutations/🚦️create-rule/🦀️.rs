//! 🚦️ `wfc3d` mutation — `CreateRule`: brings a new id-keyed adjacency rule into existence at a
//! FINAL-state insertion index.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{GraphRule, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRule {
    pub index: usize,
    pub rule: GraphRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(index: usize, rule: GraphRule) -> Wfc3dMutation {
    Wfc3dMutation::CreateRule(CreateRule { index, rule })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
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
