//! 🗑️ `delete-generation` payload — removes an id-keyed [`FormGeneration`] entry.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteGeneration
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteGeneration {
    pub id: String,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for DeleteGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::delete_generation::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::delete_generation::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete generation \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteGeneration
