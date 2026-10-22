//! 🟥️ `delete-slab-type` payload. Removes a slab type that no slab uses.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSlabType {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteSlabType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteSlabType> + From<crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "slab-type", kind: "delete-slab-type", record: "DeleteSlabType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete slab type \"{}\"", self.id), &format!("Deckentyp \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteSlabType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteSlabType> + From<crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::inverse::inverse(self, base))
    }
}
