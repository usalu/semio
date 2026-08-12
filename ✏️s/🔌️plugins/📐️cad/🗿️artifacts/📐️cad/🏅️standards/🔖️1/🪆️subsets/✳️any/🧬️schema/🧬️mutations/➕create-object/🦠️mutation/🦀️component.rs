//! ➕️ CAD mutation — `CreateObject` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadObject, CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➕️ Brings a new [`CadObject`] into existence inside `pane`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-object")]
pub struct CreateObject {
    pub pane: CadPaneId,
    #[dsl(block)]
    pub object: CadObject,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create object \"{}\"", self.object.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object.id.clone()]
    }
}
//#endregion 🔖️Mutation
