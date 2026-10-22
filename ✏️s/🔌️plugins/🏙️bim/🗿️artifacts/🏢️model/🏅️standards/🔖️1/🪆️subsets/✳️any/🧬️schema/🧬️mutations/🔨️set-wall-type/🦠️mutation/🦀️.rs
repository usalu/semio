//! 🔨️ `set-wall-type` payload. Patches exactly the provided fields of a wall type; the layer stack is one field and replaces the whole stack.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::WallTypePatch;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallType {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl SetWallType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> WallTypePatch {
        WallTypePatch { name: self.name.clone(), layers: self.layers.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: WallTypePatch) -> Self {
        Self { id, name: patch.name, layers: patch.layers }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall-type", kind: "set-wall-type", record: "SetWallType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit wall type \"{}\"", self.id), &format!("Wandtyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetWallType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetWallType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_type::inverse::inverse(self, base))
    }
}
