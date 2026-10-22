//! 🎚️ `set-column-type` payload. Changes exactly the provided fields of a column type; every element of the type follows by inference.

use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::ColumnTypePatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Profile;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetColumnType {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl SetColumnType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ColumnTypePatch {
        ColumnTypePatch { name: self.name.clone(), profile: self.profile.clone(), material: self.material.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ColumnTypePatch) -> Self {
        Self { id, name: patch.name, profile: patch.profile, material: patch.material }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetColumnType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetColumnType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "column-type", kind: "set-column-type", record: "SetColumnType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change column type \"{}\"", self.id), &format!("Stützentyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetColumnType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetColumnType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_column_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_column_type::inverse::inverse(self, base))
    }
}
