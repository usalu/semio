//! 🧭️ create-option-group authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateOptionGroup {
    pub id: String,
    pub option_group: OptionGroup,
}
impl MutationKind<ModelSnapshot, ModelMutation> for CreateOptionGroup {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "option-group", kind: "create-option-group", record: "CreateOptionGroup" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create option group", "Anlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
