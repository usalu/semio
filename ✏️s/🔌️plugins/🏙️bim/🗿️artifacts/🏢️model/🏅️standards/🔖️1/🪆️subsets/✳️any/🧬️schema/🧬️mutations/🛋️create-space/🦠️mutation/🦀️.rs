//! 🛋️ `create-space` payload. Brings a new room onto a storey; a bounded space stores only its seed point, its outline, area and volume are inferred from the walls around it.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Space};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSpace {
    pub id: String,
    pub space: Space,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSpace
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSpace> + From<semio_s_artifact_bim_model_mut_any_64::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "space", kind: "create-space", record: "CreateSpace" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_space::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_space::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create space \"{}\"", self.space.name), &format!("Raum \"{}\" anlegen", self.space.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
