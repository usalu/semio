//! ❌ `s.wfc.grid3d` mutation — `DeleteRule`: drops one adjacency rule. Because the rule set is a
//! closed allow-list, deleting an `allowed` rule NARROWS the solve rather than widening it.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rule(id: String) -> Grid3dMutation {
    Grid3dMutation::DeleteRule(DeleteRule { id })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for DeleteRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "rule", kind: "delete-rule", record: "DeletedRule" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Delete rule \"{}\"", self.id), &format!("Regel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteRule
