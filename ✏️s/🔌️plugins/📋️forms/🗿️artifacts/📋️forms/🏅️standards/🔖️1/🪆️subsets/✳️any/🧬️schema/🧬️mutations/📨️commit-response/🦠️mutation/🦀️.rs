//! 📨️ CommitResponse persists one semantic response event.
use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CommitResponse {
    pub response: crate::schema::response::FormsResponse,
    pub index: Option<usize>,
}

impl<Op> MutationKind<FormsSnapshot, Op> for CommitResponse
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<CommitResponse>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "commit", entity: "response", kind: "commit-response", record: "CommittedResponse" };
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> { crate::standards::v1::subsets::any::schema::mutations::component::commit_response::diff::diff(self, base) }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::standards::v1::subsets::any::schema::mutations::component::commit_response::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Submit Response", "Antwort absenden") }
    fn target(&self) -> Vec<String> { vec![self.response.id.clone()] }
}
