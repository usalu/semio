//! 📚️ `create-classification-system` payload. Brings a new classification system into the model library: its name, edition and the authored table of its entries (code, title, parent). Elements are classified through `set-element-classification`.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ClassificationSystem, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateClassificationSystem {
    pub id: String,
    pub system: ClassificationSystem,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateClassificationSystem
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateClassificationSystem> + From<crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "classification-system", kind: "create-classification-system", record: "CreateClassificationSystem" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create classification system \"{}\"", self.system.name), &format!("Klassifikationssystem \"{}\" anlegen", self.system.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateClassificationSystem
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateClassificationSystem> + From<crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_classification_system::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_classification_system::inverse::inverse(self, base))
    }
}
