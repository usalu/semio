//! 🔘️ `change-object-smooth-shading` — flips the mesh's smooth/flat shading flag.

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeObjectSmoothShading {
    pub id: String,
    pub new_smooth_shading: bool,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for ChangeObjectSmoothShading {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-smooth-shading", record: "ChangedObjectSmoothShading" };

    async fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Set object \"{}\" smooth shading to {}", self.id, self.new_smooth_shading)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
