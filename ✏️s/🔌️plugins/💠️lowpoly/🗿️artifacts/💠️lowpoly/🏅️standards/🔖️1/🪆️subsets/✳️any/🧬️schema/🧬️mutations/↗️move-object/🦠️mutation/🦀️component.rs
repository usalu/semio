//! ↗️️ `move-object` — absolute spatial reposition of an object's transform.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveObject {
    pub id: String,
    pub new_position: [f32; 3],
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for MoveObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "object", kind: "move-object", record: "MovedObject" };

    fn diff(&self, base: &LowpolySnapshot) -> <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move object \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
