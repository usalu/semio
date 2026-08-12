//! 🕸️ `create-mesh` — sets an object's `mesh` CHILD slot to a new owned handle (overwrite-aware,
//! same convention as stdio's `✳️object` `create-mesh`), and syncs `mesh_workspace` alongside it —
//! a kernel-edit commit touches both together. Replaces the old whole-value `replace-object-mesh`,
//! gone now that `LowpolyObject.mesh` is a real `store::ArtifactChild<SemioMeshSnapshot>` handle
//! instead of an opaque `mesh_json: String` (`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`).

use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMesh {
    pub id: String,
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
    pub mesh_workspace: String,
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for CreateMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn diff(&self, base: &LowpolySnapshot) -> <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create mesh on object \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
