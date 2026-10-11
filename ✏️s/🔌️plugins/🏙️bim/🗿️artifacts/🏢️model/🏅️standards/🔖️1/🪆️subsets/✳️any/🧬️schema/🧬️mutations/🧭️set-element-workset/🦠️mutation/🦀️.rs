//! 🧭️ set-element-workset authored mutation.
use crate::*;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementWorkset {
    pub id: String,
    pub workset: Option<String>,
}
impl MutationKind<ModelSnapshot, ModelMutation> for SetElementWorkset {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-workset", kind: "set-element-workset", record: "SetElementWorkset" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set element workset", "Zuordnen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
