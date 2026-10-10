//! 🧭️ set-element-option authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementOption {
    pub id: String,
    pub option: Option<String>,
}
impl MutationKind<ModelSnapshot, ModelMutation> for SetElementOption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-option", kind: "set-element-option", record: "SetElementOption" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set element option", "Zuordnen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
