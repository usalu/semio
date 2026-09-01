//! 🏟️ `change-retrofit-knowledge-level` payload — changes the En1998 document's `retrofit_knowledge_level` (retrofit knowledge level).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRetrofitKnowledgeLevel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRetrofitKnowledgeLevel {
    pub new_retrofit_knowledge_level: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitKnowledgeLevel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-knowledge-level", kind: "change-retrofit-knowledge-level", record: "ChangedRetrofitKnowledgeLevel" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit knowledge level to \"{}\"", self.new_retrofit_knowledge_level)
    }
}
//#endregion 🔖️ChangeRetrofitKnowledgeLevel
