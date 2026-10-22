//! 📽️ `create-view` payload. Brings a new view into a building: a plan or ceiling plan of one of its storeys, a section or elevation through a vertical plane, or a camera. Each kind owns exactly its fields; the linework it draws is inferred.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, View};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateView {
    pub id: String,
    pub view: View,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateView
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateView> + From<semio_s_artifact_bim_model_mut_any_93::standards::v1::subsets::any::schema::mutations::delete_view::DeleteView>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "view", kind: "create-view", record: "CreateView" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create view \"{}\"", self.view.name), &format!("Ansicht \"{}\" anlegen", self.view.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateView
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateView> + From<semio_s_artifact_bim_model_mut_any_93::standards::v1::subsets::any::schema::mutations::delete_view::DeleteView>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_view::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_view::inverse::inverse(self, base))
    }
}
