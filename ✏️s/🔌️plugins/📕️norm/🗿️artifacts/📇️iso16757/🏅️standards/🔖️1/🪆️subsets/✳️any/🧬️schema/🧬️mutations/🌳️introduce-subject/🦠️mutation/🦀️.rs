//! 🆕️ `introduce-subject` — brings a new id-keyed dictionary subject into existence.

use crate::{part_4::Subject, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct IntroduceSubject {
    pub subject: Subject,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroduceSubject {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "subject", kind: "introduce-subject", record: "IntroducedSubject" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create dictionary subject \"{}\"", self.subject.names.preferred.text), &format!("Wörterbuchthema \"{}\" erstellen", self.subject.names.preferred.text))
    }
    fn target(&self) -> Vec<String> {
        vec![self.subject.id.clone()]
    }
}
//#endregion 🔖️Payload
