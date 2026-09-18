//! 🚦 `s.wfc.grid3d` mutation — `CreateRule`: admits or denies one ordered tile pair across one of the
//! six face directions. A pair no rule mentions for a direction is NOT allowed: the rule set is a
//! closed allow-list, never a deny-list.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateRule {
    pub rule: Grid3dRule,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(rule: Grid3dRule) -> Grid3dMutation {
    Grid3dMutation::CreateRule(CreateRule { rule })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
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
