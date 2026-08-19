//! ➕️ `insert-point` — places a new point into the geometry playground's anonymous, index-keyed
//! point cloud. `index` is FINAL-state, per the addressing convention for index-keyed collections.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertPoint {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for InsertPoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "point", kind: "insert-point", record: "InsertedPoint" };

    async fn diff(&self, base: &MathematicalSnapshot) -> protocol::MutationOutcome<<MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Insert point at {}", self.index)
    }
    async fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
