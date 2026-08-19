//! 🗑️ VCS mutation — `RemoveTag`: detaches a set-like tag member from the document.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `remove-tag` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-tag")]
pub struct RemoveTag {
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn remove_tag(tag: String) -> VcsDemoMutation {
    VcsDemoMutation::RemoveTag(RemoveTag { tag })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for RemoveTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "tag", kind: "remove-tag", record: "RemovedTag" };

    async fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove tag \"{}\"", self.tag)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.tag.clone()]
    }
}
//#endregion 🔖️Mutation
