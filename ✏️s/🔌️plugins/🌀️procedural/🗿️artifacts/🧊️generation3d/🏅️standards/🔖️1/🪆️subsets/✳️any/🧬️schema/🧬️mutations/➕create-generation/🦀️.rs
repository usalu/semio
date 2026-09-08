//! ➕ `create-generation` payload — brings a new id-keyed [`FormGeneration`] into existence.
//! Delegates to `semio_framework_artifact_playbook_playbook`'s existing `GenerationMutation::Add` engine (framework territory,
//! out of this facet's writable boundary) via the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateGeneration
/// ➕ Full initial payload for a new generation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateGeneration {
    pub generation: FormGeneration,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for CreateGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_generation::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::standards::v1::subsets::any::schema::mutations::create_generation::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create generation \"{}\"", self.generation.name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️CreateGeneration
