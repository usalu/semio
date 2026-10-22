//! 🧭️ set-workset authored mutation.
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::diff::patches::hand_written::WorksetPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWorkset {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_visible: Option<bool>,
}
impl SetWorkset {
    pub fn patch(&self) -> WorksetPatch { WorksetPatch { name: self.name.clone(), default_visible: self.default_visible.clone() } }
    pub fn from_patch(id: String, patch: WorksetPatch) -> Self { Self { id, name: patch.name, default_visible: patch.default_visible } }
}
impl<Op> MutationKind<ModelSnapshot, Op> for SetWorkset
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWorkset>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "workset", kind: "set-workset", record: "SetWorkset" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set workset", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetWorkset
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetWorkset>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_workset::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_workset::inverse::inverse(self, base)) }
}
