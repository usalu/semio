//! 🏷️ VCS mutation — `AddTag`: attaches a set-like tag member to the document.
use crate::mutations::VcsDemoMutation;
use crate::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 🏷️ `add-tag` payload.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-tag")]
pub struct AddTag {
    pub tag: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none", default))]
    pub index: Option<u32>,
}

/// 🏗️ Builder — appends the tag after the existing ones.
pub fn add_tag(tag: String) -> VcsDemoMutation {
    VcsDemoMutation::AddTag(AddTag { tag, index: None })
}

/// 📍️ Builder — inserts the tag at `index` (an index past the end appends).
pub fn add_tag_at(tag: String, index: u32) -> VcsDemoMutation {
    VcsDemoMutation::AddTag(AddTag { tag, index: Some(index) })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for AddTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "tag", kind: "add-tag", record: "AddedTagToVcs" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Result<Vec<VcsDemoMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add tag \"{}\"", self.tag), &format!("Tag \"{}\" hinzufügen", self.tag))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tag.clone()]
    }
}
//#endregion 🔖️Mutation
