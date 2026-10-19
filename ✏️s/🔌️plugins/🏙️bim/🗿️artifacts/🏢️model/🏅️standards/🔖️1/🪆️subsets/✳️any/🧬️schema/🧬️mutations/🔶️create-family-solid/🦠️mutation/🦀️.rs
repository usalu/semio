//! 🔶️ `create-family-solid` payload. Adds a solid to a family: an extrusion, revolution, sweep or cuboid whose dimensions, material and visibility are formulas of the family parameters.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{FamilySolid, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateFamilySolid {
    pub id: String,
    pub solid: FamilySolid,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateFamilySolid
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateFamilySolid> + From<crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "family-solid", kind: "create-family-solid", record: "CreateFamilySolid" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_family_solid::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_family_solid::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create family solid \"{}\"", self.solid.name), &format!("Familienkörper \"{}\" anlegen", self.solid.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
