//! 🔢 VCS mutation — `ChangeCounter`: sets the document's `counter` scalar to a new value.
use crate::mutations::VcsDemoMutation;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 🔢 `change-counter` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-counter")]
pub struct ChangeCounter {
    pub new_counter: i64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_counter(new_counter: i64) -> VcsDemoMutation {
    VcsDemoMutation::ChangeCounter(ChangeCounter { new_counter })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for ChangeCounter {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "vcs", kind: "change-counter", record: "ChangedVcsCounter" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change counter to {}", self.new_counter), &format!("Zähler auf {} ändern", self.new_counter))
    }
}
//#endregion 🔖️Mutation
