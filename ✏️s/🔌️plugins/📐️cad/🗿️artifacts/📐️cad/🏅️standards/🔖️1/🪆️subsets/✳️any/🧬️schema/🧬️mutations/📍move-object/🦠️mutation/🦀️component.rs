//! 📍️ CAD mutation — `MoveObject` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍️ Move one object's `origin` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-object")]
pub struct MoveObject {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_origin: [f64; 3],
}

impl MutationKind<CadSnapshot, CadMutation> for MoveObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "object", kind: "move-object", record: "MovedObject" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
