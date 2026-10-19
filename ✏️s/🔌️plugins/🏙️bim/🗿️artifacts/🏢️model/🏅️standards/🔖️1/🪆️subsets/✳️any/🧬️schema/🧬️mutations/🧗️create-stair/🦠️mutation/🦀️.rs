//! 🧗️ `create-stair` payload. Brings a stair run onto a storey; risers and treads are never stored, they are inferred from the storey levels and the top constraint.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Stair};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateStair {
    pub id: String,
    pub stair: Stair,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateStair
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateStair> + From<semio_s_artifact_bim_model_mut_any_80::standards::v1::subsets::any::schema::mutations::delete_stair::DeleteStair>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "stair", kind: "create-stair", record: "CreateStair" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_stair::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_stair::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create stair \"{}\"", self.stair.name), &format!("Treppe \"{}\" anlegen", self.stair.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
