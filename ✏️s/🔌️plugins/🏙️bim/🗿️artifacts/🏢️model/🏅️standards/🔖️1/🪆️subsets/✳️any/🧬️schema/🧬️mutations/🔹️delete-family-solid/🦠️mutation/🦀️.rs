//! 🔹️ `delete-family-solid` payload. Removes one solid from its family; the parameters stay untouched.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteFamilySolid {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteFamilySolid
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteFamilySolid> + From<crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "family-solid", kind: "delete-family-solid", record: "DeleteFamilySolid" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete family solid \"{}\"", self.id), &format!("Familienkörper \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteFamilySolid
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteFamilySolid> + From<crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::inverse::inverse(self, base))
    }
}
