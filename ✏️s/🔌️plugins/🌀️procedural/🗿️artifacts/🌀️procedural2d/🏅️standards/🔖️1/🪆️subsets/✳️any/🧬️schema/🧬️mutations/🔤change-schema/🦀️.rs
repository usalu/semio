//! 🏷️ Procedural2d mutation — `ChangeSchema`: sets the fixture's document-level schema field to a
//! new value.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSchema
/// 🏷️ `change-schema` payload — the fixture's new schema id.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSchema {
    pub schema: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_schema(schema: String) -> Procedural2dMutation {
    Procedural2dMutation::ChangeSchema(ChangeSchema { schema })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ChangeSchema {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "fixture", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change schema to \"{}\"", self.schema)
    }
}
//#endregion 🔖️ChangeSchema
