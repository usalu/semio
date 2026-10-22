//! 💥️ `delete-wall` payload. Removes a wall together with the openings it hosts and the properties and classifications of both.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWall {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteWall> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "wall", kind: "delete-wall", record: "DeleteWall" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete wall \"{}\" with its openings", self.id), &format!("Wand \"{}\" samt Öffnungen löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteWall
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteWall> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_wall::inverse::inverse(self, base))
    }
}
