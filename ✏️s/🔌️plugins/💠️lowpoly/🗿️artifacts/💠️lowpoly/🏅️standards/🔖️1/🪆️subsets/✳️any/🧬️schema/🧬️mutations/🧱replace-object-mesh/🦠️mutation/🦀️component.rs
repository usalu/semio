//! 🧱️ `replace-object-mesh` — whole-value swap of the object's mesh geometry payload.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceObjectMesh {
    pub id: String,
    pub new_mesh_json: String,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ReplaceObjectMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "object-mesh", kind: "replace-object-mesh", record: "ReplacedObjectMesh" };

    fn diff(&self, base: &LowpolySnapshot) -> <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace mesh on object \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
