//! 🎐️ `set-ceiling-type` payload. Patches exactly the provided fields of a ceiling type; the layer stack is one field and replaces the whole stack.

use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::CeilingTypePatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCeilingType {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl SetCeilingType {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CeilingTypePatch {
        CeilingTypePatch { name: self.name.clone(), layers: self.layers.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CeilingTypePatch) -> Self {
        Self { id, name: patch.name, layers: patch.layers }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCeilingType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCeilingType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ceiling-type", kind: "set-ceiling-type", record: "SetCeilingType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit ceiling type \"{}\"", self.id), &format!("Unterdeckentyp \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetCeilingType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetCeilingType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_ceiling_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_ceiling_type::inverse::inverse(self, base))
    }
}
