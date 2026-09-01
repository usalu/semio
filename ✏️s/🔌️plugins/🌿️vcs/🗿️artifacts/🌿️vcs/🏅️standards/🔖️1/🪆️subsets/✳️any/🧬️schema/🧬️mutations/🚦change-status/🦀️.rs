//! 🚦 VCS mutation — `ChangeStatus`: sets the document's `status` scalar to a new value.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🚦 `change-status` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-status")]
pub struct ChangeStatus {
    pub new_status: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_status(new_status: String) -> VcsDemoMutation {
    VcsDemoMutation::ChangeStatus(ChangeStatus { new_status })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for ChangeStatus {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vcs", kind: "change-status", record: "ChangedVcsStatus" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change status to \"{}\"", self.new_status)
    }
}
//#endregion 🔖️Mutation
