//! 📝 VCS mutation — `ChangeNotes`: sets the document's `notes` scalar to a new value.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 📝 `change-notes` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-notes")]
pub struct ChangeNotes {
    pub new_notes: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_notes(new_notes: String) -> VcsDemoMutation {
    VcsDemoMutation::ChangeNotes(ChangeNotes { new_notes })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for ChangeNotes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vcs", kind: "change-notes", record: "ChangedVcsNotes" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change notes to \"{}\"", self.new_notes)
    }
}
//#endregion 🔖️Mutation
