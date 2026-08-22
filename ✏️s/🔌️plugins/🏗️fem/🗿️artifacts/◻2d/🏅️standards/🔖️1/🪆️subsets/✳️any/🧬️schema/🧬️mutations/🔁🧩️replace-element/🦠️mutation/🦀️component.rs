//! 🔁️ Fem2d mutation — `ReplaceElement` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemElement};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing element's payload (`Bar`/`Beam` fields, including a possible
/// variant change) — structured, no field-by-field editor gesture exists for elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-element")]
pub struct ReplaceElement {
    pub id: String,
    #[dsl(statements)]
    pub new_element: Box<FemElement>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "element", kind: "replace-element", record: "ReplacedElement" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
