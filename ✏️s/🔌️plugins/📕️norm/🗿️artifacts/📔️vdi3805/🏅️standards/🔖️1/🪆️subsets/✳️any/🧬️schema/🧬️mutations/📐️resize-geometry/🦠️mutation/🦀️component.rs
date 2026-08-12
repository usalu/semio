//! 📐️ `resize-geometry` — changes a geometry's bounding-box extent, addressed by id.

use crate::artifacts::vdi3805::{BoundingBox, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeGeometry {
    pub id: String,
    pub new_bbox: BoundingBox,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ResizeGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "geometry", kind: "resize-geometry", record: "ResizedGeometry" };

    fn diff(&self, base: &Vdi3805Snapshot) -> <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize geometry \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
