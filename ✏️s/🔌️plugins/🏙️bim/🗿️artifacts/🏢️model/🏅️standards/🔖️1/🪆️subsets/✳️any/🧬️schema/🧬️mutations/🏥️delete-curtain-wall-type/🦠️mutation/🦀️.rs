//! 🏥️ `delete-curtain-wall-type` payload. Removes a curtain wall type that no curtain wall uses.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteCurtainWallType {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteCurtainWallType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteCurtainWallType> + From<crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "curtain-wall-type", kind: "delete-curtain-wall-type", record: "DeleteCurtainWallType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete curtain wall type \"{}\"", self.id), &format!("Fassadentyp \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteCurtainWallType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteCurtainWallType> + From<crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::inverse::inverse(self, base))
    }
}
