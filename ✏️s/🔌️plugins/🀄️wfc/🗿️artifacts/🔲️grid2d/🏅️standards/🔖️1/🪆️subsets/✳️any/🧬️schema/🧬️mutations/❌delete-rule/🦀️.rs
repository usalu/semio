//! ❌ Removes one adjacency rule — the pair falls back to the FORBIDDEN default for that direction.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteRule
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteRule {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_rule(id: String) -> Grid2dMutation {
    Grid2dMutation::DeleteRule(DeleteRule { id })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for DeleteRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "rule", kind: "delete-rule", record: "DeletedRule" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Grid2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete rule \"{}\"", self.id), &format!("Regel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteRule
