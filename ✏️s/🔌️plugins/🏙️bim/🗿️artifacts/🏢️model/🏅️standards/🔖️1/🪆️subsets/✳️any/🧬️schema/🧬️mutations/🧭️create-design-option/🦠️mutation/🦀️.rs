//! 🧭️ create-design-option authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateDesignOption {
    pub id: String,
    pub design_option: DesignOption,
}
impl MutationKind<ModelSnapshot, ModelMutation> for CreateDesignOption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "design-option", kind: "create-design-option", record: "CreateDesignOption" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create design option", "Anlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
