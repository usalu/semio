//! 🗑️ Fem3d mutation — `DeleteElement` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, element_id};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, create_element};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🗑️ Removes an existing element by id, capturing nothing itself (the removed payload is recovered
/// from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-element")]
pub struct DeleteElement {
    pub id: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for DeleteElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "element", kind: "delete-element", record: "DeletedElement" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete element \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
