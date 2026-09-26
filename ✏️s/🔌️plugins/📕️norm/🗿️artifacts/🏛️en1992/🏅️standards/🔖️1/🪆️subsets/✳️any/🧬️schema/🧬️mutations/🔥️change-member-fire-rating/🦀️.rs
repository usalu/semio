//! 🔥️ `change-member-fire-rating`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::part_1_2::FireRating;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberFireRating {
    pub member_id: String,
    pub new_rating: FireRating,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeMemberFireRating {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-fire-rating", kind: "change-member-fire-rating", record: "ChangedMemberFireRating" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change fire rating of {}", self.member_id), &format!("Feuerwiderstand von {} ändern", self.member_id))
    }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}
