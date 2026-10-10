//! 🧭️ create-workset authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWorkset {
    pub id: String,
    pub workset: Workset,
}
impl MutationKind<ModelSnapshot, ModelMutation> for CreateWorkset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "workset", kind: "create-workset", record: "CreateWorkset" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create workset", "Anlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
