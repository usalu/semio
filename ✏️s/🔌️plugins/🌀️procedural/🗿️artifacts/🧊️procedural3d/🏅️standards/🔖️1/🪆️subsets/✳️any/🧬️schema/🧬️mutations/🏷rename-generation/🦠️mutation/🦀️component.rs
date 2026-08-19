//! 🏷️ `rename-generation` payload — changes a generation's identity `name` field.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️RenameGeneration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameGeneration {
    pub id: String,
    pub new_name: String}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for RenameGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "generation", kind: "rename-generation", record: "RenamedGeneration" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::rename_generation::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::rename_generation::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Rename generation \"{}\" to \"{}\"", self.id, self.new_name)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️RenameGeneration
