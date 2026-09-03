//! 🗑️ VCS mutation — `RemoveTag`: detaches a set-like tag member from the document.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 🗑️ `remove-tag` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "remove-tag")]
pub struct RemoveTag {
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_tag(tag: String) -> VcsDemoMutation {
    VcsDemoMutation::RemoveTag(RemoveTag { tag })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for RemoveTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "tag", kind: "remove-tag", record: "RemovedTag" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove tag \"{}\"", self.tag)
    }
    fn target(&self) -> Vec<String> {
        vec![self.tag.clone()]
    }
}
//#endregion 🔖️Mutation
