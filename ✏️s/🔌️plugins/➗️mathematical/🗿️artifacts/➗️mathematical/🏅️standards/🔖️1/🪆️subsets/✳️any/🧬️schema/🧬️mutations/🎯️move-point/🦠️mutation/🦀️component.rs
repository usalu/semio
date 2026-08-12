//! 🎯️ `move-point` — absolute spatial reposition of a point in the geometry playground's point
//! cloud, addressed by its BASE-state index.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovePoint {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for MovePoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "point", kind: "move-point", record: "MovedPoint" };

    fn diff(&self, base: &MathematicalSnapshot) -> <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move point at {}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
