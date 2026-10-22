//! 🪨️ `create-material` payload. Brings a new material into the library; physical values are finite and non-negative and every colour component lies in 0..1.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Material, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMaterial {
    pub id: String,
    pub material: Material,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateMaterial
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateMaterial> + From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreateMaterial" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create material \"{}\"", self.material.name), &format!("Material \"{}\" anlegen", self.material.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateMaterial
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateMaterial> + From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_material::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_material::inverse::inverse(self, base))
    }
}
