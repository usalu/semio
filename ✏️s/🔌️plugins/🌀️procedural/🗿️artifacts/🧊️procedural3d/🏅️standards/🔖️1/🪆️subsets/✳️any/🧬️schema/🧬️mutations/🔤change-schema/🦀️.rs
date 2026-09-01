//! 🔧 `change-schema` payload — document-level scalar: the fixture's own schema version string
//! (`📓️derivation-rules.md` rule 1's `change-<field>` per remaining scalar).
//!
//! Directory kept at its pre-migration `🎛set-schema` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSchema
/// 🔧 Whole-artifact scope — the fixture has exactly one schema field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::change_schema::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::change_schema::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change schema to \"{}\"", self.new_schema)
    }
}
//#endregion 🔖️ChangeSchema
