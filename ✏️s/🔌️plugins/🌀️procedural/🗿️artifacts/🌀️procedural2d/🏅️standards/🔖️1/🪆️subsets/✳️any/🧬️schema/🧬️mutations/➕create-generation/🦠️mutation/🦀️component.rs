//! 🦠️ `➕create-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::playbook::FormGeneration;
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGeneration {
    pub generation: FormGeneration,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_generation(generation: FormGeneration) -> Procedural2dMutation {
    Procedural2dMutation::CreateGeneration(CreateGeneration { generation })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for CreateGeneration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create generation \"{}\"", self.generation.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️Mutation
