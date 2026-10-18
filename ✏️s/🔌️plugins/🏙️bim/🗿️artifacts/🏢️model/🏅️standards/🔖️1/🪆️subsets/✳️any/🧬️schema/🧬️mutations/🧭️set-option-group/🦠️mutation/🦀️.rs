//! 🧭️ set-option-group authored mutation.
use crate::standards::v1::subsets::any::schema::{diff::{ModelDiff, OptionGroupPatch}, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetOptionGroup {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SetOptionGroup {
    pub fn patch(&self) -> OptionGroupPatch { OptionGroupPatch { name: self.name.clone() } }
    pub fn from_patch(id: String, patch: OptionGroupPatch) -> Self { Self { id, name: patch.name } }
}
impl<Op> MutationKind<ModelSnapshot, Op> for SetOptionGroup
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetOptionGroup>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "option-group", kind: "set-option-group", record: "SetOptionGroup" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_option_group::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_option_group::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set option group", "Ändern") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
