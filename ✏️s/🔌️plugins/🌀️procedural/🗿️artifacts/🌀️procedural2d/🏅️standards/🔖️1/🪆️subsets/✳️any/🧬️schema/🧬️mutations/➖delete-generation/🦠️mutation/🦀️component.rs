//! 🦠️ `➖delete-generation` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::playbook::FormGeneration;
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteGeneration {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_generation(id: String) -> Procedural2dMutation {
    Procedural2dMutation::DeleteGeneration(DeleteGeneration { id })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for DeleteGeneration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

    fn diff(&self, base: &Procedural2dSnapshot) -> Procedural2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete generation \"{}\"", payload.id)
    }
    fn target(&self) -> Vec<String> {
        vec![payload.id.clone()]
    }
}
//#endregion 🔖️Mutation
