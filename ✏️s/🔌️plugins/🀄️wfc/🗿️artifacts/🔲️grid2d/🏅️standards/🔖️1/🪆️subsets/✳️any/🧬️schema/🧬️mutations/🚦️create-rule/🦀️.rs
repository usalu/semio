//! 🚦 Declares whether tile B may sit in one direction of tile A. Unspecified pairs default to FORBIDDEN, so the rule set is the complete whitelist.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateRule {
    pub rule: WfcAdjacencyRule2d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_rule(rule: WfcAdjacencyRule2d) -> Grid2dMutation {
    Grid2dMutation::CreateRule(CreateRule { rule })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for CreateRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "rule", kind: "create-rule", record: "CreatedRule" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create rule \"{}\"", self.rule.id), &format!("Regel \"{}\" erstellen", self.rule.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.rule.id.clone()]
    }
}
//#endregion 🔖️CreateRule
