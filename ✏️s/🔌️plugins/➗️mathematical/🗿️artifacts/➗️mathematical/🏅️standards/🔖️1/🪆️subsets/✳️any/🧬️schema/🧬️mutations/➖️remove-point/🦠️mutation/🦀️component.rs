//! ➖️ `remove-point` — takes a point out of the geometry playground's point cloud. `index` is
//! BASE-state, per the addressing convention for index-keyed collections.

use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemovePoint {
    pub index: usize,
}

impl protocol::MutationKind<MathematicalSnapshot, MathematicalMutation> for RemovePoint {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "point", kind: "remove-point", record: "RemovedPoint" };

    fn diff(&self, base: &MathematicalSnapshot) -> <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove point at {}", self.index)
    }
    fn target(&self) -> Vec<String> {
        vec!["geometry".into(), "points".into(), self.index.to_string()]
    }
}
//#endregion 🔖️Payload
