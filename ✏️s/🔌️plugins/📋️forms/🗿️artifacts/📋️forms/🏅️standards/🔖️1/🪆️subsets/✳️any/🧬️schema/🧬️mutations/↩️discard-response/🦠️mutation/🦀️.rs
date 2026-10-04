//! 📨️ DiscardResponse persists one semantic response event.
use crate::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DiscardResponse {
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DiscardResponse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "discard", entity: "response", kind: "discard-response", record: "DiscardedResponse" };
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Retract Response", "Antwort zurückziehen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
