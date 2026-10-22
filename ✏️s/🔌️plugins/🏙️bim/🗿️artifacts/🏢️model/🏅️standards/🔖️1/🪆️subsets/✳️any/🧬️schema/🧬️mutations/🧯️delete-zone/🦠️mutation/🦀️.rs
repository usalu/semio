//! 🧯️ `delete-zone` payload. Removes a zone with its properties and classifications and clears the zone of every space that belonged to it; refuses while an area scheme still counts it.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteZone {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteZone
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteZone> + From<semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space::SetSpace> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "zone", kind: "delete-zone", record: "DeleteZone" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete zone \"{}\"", self.id), &format!("Zone \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteZone
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteZone> + From<semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space::SetSpace> + semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_zone::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_zone::inverse::inverse(self, base))
    }
}
