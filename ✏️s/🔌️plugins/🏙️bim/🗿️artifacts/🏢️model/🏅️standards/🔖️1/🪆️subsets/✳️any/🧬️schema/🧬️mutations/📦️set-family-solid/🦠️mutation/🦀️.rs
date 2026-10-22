//! 📦️ `set-family-solid` payload. Sets exactly the provided fields of a family solid: name, shape, material, visibility and offset; absent fields stay untouched.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{ExprPoint3, SolidShape};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::FamilySolidPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFamilySolid {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<SolidShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<ExprPoint3>,
}

impl SetFamilySolid {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> FamilySolidPatch {
        FamilySolidPatch { name: self.name.clone(), shape: self.shape.clone(), material: self.material.clone(), visible: self.visible.clone(), offset: self.offset.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: FamilySolidPatch) -> Self {
        Self { id, name: patch.name, shape: patch.shape, material: patch.material, visible: patch.visible, offset: patch.offset }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetFamilySolid
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetFamilySolid>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "family-solid", kind: "set-family-solid", record: "SetFamilySolid" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change family solid \"{}\"", self.id), &format!("Familienkörper \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetFamilySolid
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetFamilySolid>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_family_solid::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_family_solid::inverse::inverse(self, base))
    }
}
