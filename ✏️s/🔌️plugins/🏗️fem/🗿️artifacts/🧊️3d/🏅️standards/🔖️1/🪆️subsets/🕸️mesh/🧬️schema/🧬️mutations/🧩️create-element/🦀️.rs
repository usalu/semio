//! 🌱️ Fem3d mutation — `CreateElement` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemElement, element_id};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, delete_element};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemElement`] (`Bar`/`Frame`) into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-element")]
pub struct CreateElement {
    #[dsl(statements)]
    pub element: Box<FemElement>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "element", kind: "create-element", record: "CreatedElement" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create element \"{}\"", element_id(&self.element))
    }
    fn target(&self) -> Vec<String> {
        vec![element_id(&self.element).to_string()]
    }
}
//#endregion 🔖️Mutation
