//! `upsert-member` — upsert a `SteelMember` by id into `members`.

use crate::{SteelMember, En1993Mutation, En1993Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateMemberProperties {
    pub member: SteelMember,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateMemberProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "member", kind: "update-member-properties", record: "UpdatedMember" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(
            &format!("Update member {}", self.member.id),
            &format!("Bauteil {} aktualisieren", self.member.id),
        )
    }
    fn target(&self) -> Vec<String> {
        vec![self.member.id.clone()]
    }
}
//#endregion 🔖️Payload
