//! 🧭️ create-option-group authored mutation.
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, OptionGroup};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateOptionGroup {
    pub id: String,
    pub option_group: OptionGroup,
}
impl<Op> MutationKind<ModelSnapshot, Op> for CreateOptionGroup
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateOptionGroup> + From<crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "option-group", kind: "create-option-group", record: "CreateOptionGroup" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create option group", "Anlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateOptionGroup
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateOptionGroup> + From<crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::create_option_group::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::create_option_group::inverse::inverse(self, base)) }
}
