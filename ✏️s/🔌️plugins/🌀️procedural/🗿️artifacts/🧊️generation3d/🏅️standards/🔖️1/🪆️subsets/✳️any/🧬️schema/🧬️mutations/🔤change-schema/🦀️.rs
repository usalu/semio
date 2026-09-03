//! 🔧 `change-schema` payload — document-level scalar: the fixture's own schema version string
//! (`📓️derivation-rules.md` rule 1's `change-<field>` per remaining scalar).
//!
//! Directory kept at its pre-migration `🎛set-schema` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSchema
/// 🔧 Whole-artifact scope — the fixture has exactly one schema field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::change_schema::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::change_schema::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change schema to \"{}\"", self.new_schema)
    }
}
//#endregion 🔖️ChangeSchema
