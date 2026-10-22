//! 🛏️ `create-component` payload. Places an instance of a family: a storey, a family, a plan position with an elevation above the storey, a rotation, a mirror flag, an optional host wall and an optional system for a terminal.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Component, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateComponent {
    pub id: String,
    pub component: Component,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateComponent
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateComponent> + From<semio_s_artifact_bim_model_mut_any_122::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "component", kind: "create-component", record: "CreateComponent" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place component \"{}\"", self.component.name), &format!("Komponente \"{}\" platzieren", self.component.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateComponent
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateComponent> + From<semio_s_artifact_bim_model_mut_any_122::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_component::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_component::inverse::inverse(self, base))
    }
}
