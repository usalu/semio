//! ↔️ `create-dimension` payload. Brings a new dimension onto a storey plan: two or more anchors (free points or elements of the model) measured along a direction; its values and text are inferred from the geometry of the anchors.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Dimension, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateDimension {
    pub id: String,
    pub dimension: Dimension,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateDimension
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateDimension> + From<semio_s_artifact_bim_model_mut_any_95::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "dimension", kind: "create-dimension", record: "CreateDimension" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create dimension \"{}\"", self.dimension.name), &format!("Bemaßung \"{}\" anlegen", self.dimension.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateDimension
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateDimension> + From<semio_s_artifact_bim_model_mut_any_95::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_dimension::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_dimension::inverse::inverse(self, base))
    }
}
