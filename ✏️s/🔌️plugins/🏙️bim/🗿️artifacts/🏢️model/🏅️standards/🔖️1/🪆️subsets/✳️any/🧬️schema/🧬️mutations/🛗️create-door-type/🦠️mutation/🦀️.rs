//! 🛗️ `create-door-type` payload. Brings a new door type into the library; it needs a free id, an existing material and valid dimensions.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{DoorType, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateDoorType {
    pub id: String,
    pub door_type: DoorType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateDoorType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateDoorType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "door-type", kind: "create-door-type", record: "CreateDoorType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create door type \"{}\"", self.door_type.name), &format!("Türtyp \"{}\" anlegen", self.door_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateDoorType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateDoorType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_door_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_door_type::inverse::inverse(self, base))
    }
}
