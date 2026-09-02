//! 🔁️ Fem3d mutation — `ReplaceElement` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemElement, element_id};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta, Fem3dElementsPatchEntry};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing element's payload (`Bar`/`Frame` fields, including a possible
/// variant change) — structured, no field-by-field editor gesture exists for elements.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-element")]
pub struct ReplaceElement {
    pub id: String,
    #[dsl(statements)]
    pub new_element: Box<FemElement>,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "element", kind: "replace-element", record: "ReplacedElement" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace element \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
