//! ➕ `create-generation` payload — brings a new id-keyed [`FormGeneration`] into existence.
//! Delegates to `flow::playbook`'s existing `GenerationMutation::Add` engine (framework territory,
//! out of this facet's writable boundary) via the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::playbook::FormGeneration;
use serde::{Deserialize, Serialize};

//#region 🔖️CreateGeneration
/// ➕ Full initial payload for a new generation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGeneration {
    pub generation: FormGeneration,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for CreateGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::create_generation::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::create_generation::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Create generation \"{}\"", self.generation.name)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️CreateGeneration
