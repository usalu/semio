//! 📝 VCS mutation — `ChangeNotes`: sets the document's `notes` scalar to a new value.
use crate::mutations::VcsDemoMutation;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 📝 `change-notes` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
    fn inverse(&self, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change notes to \"{}\"", self.new_notes), &format!("Notizen auf \"{}\" ändern", self.new_notes))
    }
}
//#endregion 🔖️Mutation
