//! 🗜️ `delete-property-template` payload. Removes a property set template from the library. Templates own no entries: the property sets of elements stay as they are, they merely lose their definition.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeletePropertyTemplate {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeletePropertyTemplate
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeletePropertyTemplate> + From<crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "property-template", kind: "delete-property-template", record: "DeletePropertyTemplate" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_property_template::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_property_template::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete property template \"{}\"", self.id), &format!("Eigenschaftsvorlage \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
