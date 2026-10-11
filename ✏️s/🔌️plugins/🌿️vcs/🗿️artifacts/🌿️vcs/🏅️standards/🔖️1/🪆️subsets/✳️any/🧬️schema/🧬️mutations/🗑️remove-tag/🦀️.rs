//! 🗑️ VCS mutation — `RemoveTag`: detaches a set-like tag member from the document.
use crate::mutations::VcsDemoMutation;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 🗑️ `remove-tag` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn inverse(&self, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove tag \"{}\"", self.tag), &format!("Tag \"{}\" entfernen", self.tag))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tag.clone()]
    }
}
//#endregion 🔖️Mutation
