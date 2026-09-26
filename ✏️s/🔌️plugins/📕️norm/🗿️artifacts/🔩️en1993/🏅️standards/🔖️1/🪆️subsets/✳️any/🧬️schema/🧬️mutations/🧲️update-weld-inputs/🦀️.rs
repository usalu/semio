//! `upsert-member-action` — upsert a `MemberAction` by id into `member_actions`.

use crate::{MemberAction, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateWeldInputs {
    pub member_action: MemberAction,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateWeldInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "memberAction", kind: "update-weld-inputs", record: "UpdatedMemberAction" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(
            &format!("Upsert member action {}", self.member_action.id),
            &format!("Bauteilbeanspruchung setzen {}", self.member_action.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.member_action.id.clone()]
    }
}
//#endregion 🔖️Payload
