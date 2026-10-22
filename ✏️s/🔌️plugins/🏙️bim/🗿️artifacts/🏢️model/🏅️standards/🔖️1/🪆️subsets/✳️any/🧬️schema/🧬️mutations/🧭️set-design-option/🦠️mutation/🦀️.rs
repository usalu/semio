//! 🧭️ set-design-option authored mutation.
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::diff::patches::hand_written::DesignOptionPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetDesignOption {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
impl SetDesignOption {
    pub fn patch(&self) -> DesignOptionPatch { DesignOptionPatch { group: self.group.clone(), name: self.name.clone(), primary: self.primary.clone() } }
    pub fn from_patch(id: String, patch: DesignOptionPatch) -> Self { Self { id, group: patch.group, name: patch.name, primary: patch.primary } }
}
impl<Op> MutationKind<ModelSnapshot, Op> for SetDesignOption
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetDesignOption>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "design-option", kind: "set-design-option", record: "SetDesignOption" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set design option", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetDesignOption
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetDesignOption>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_design_option::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_design_option::inverse::inverse(self, base)) }
}
