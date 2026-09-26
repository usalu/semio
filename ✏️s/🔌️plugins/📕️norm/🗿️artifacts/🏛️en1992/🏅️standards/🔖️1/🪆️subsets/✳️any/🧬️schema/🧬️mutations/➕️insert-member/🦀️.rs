//! ➕️ `insert-member`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::{En1992Snapshot, RcMember};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertMember {
    pub index: usize,
    pub member: RcMember,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for InsertMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "member", kind: "insert-member", record: "InsertedMember" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Insert member {}", self.member.id), &format!("Bauteil {} einfügen", self.member.id))
    }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}
