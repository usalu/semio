//! 🧰️ `create-property-template` payload. Brings a new property set template into the model library: the property set name, the element and type kinds it applies to and the definition of each of its properties. It changes no element; the effective properties are inferred.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, PropertyTemplate};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreatePropertyTemplate {
    pub id: String,
    pub template: PropertyTemplate,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreatePropertyTemplate
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreatePropertyTemplate> + From<crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "property-template", kind: "create-property-template", record: "CreatePropertyTemplate" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_property_template::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_property_template::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create property template \"{}\"", self.template.name), &format!("Eigenschaftsvorlage \"{}\" anlegen", self.template.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
