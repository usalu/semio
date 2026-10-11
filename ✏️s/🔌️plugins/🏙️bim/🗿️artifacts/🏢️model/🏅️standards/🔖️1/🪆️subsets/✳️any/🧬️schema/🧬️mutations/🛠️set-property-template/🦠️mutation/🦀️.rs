//! 🛠️ `set-property-template` payload. Sets any of a property set template's name, applicable kinds and property definitions; absent fields stay untouched and a list replaces the whole list.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, PropertyDef, PropertyTemplatePatch, TemplateTarget};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPropertyTemplate {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<TemplateTarget>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PropertyDef>>,
}

impl SetPropertyTemplate {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> PropertyTemplatePatch {
        PropertyTemplatePatch { name: self.name.clone(), applies_to: self.applies_to.clone(), properties: self.properties.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: PropertyTemplatePatch) -> Self {
        Self { id, name: patch.name, applies_to: patch.applies_to, properties: patch.properties }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetPropertyTemplate {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "property-template", kind: "set-property-template", record: "SetPropertyTemplate" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit property template \"{}\"", self.id), &format!("Eigenschaftsvorlage \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
