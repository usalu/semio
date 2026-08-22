//! 🌱️ Fem2d mutation — `CreateElement` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{element_id, Fem2dSnapshot, FemElement};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemElement`] (`Bar`/`Beam`) into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-element")]
pub struct CreateElement {
    #[dsl(statements)]
    pub element: Box<FemElement>,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "element", kind: "create-element", record: "CreatedElement" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
