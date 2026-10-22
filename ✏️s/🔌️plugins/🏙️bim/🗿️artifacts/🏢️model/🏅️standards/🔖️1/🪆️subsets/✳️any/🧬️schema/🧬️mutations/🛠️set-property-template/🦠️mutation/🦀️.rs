//! 🛠️ `set-property-template` payload. Sets any of a property set template's name, applicable kinds and property definitions; absent fields stay untouched and a list replaces the whole list.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::PropertyTemplatePatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{PropertyDef, TemplateTarget};
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

impl<Op> MutationKind<ModelSnapshot, Op> for SetPropertyTemplate
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetPropertyTemplate>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "property-template", kind: "set-property-template", record: "SetPropertyTemplate" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit property template \"{}\"", self.id), &format!("Eigenschaftsvorlage \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetPropertyTemplate
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetPropertyTemplate>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_property_template::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_property_template::inverse::inverse(self, base))
    }
}
