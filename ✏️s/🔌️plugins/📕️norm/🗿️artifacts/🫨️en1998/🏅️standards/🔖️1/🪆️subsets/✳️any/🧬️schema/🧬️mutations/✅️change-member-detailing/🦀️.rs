//! ✅️ `change-member-detailing` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberDetailing {
    pub building_index: usize,
    pub member_index: usize,
    pub new_detailing_compatible_with_q: bool,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeMemberDetailing {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-detailing",
        kind: "change-member-detailing",
        record: "ChangeMemberDetailing",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change member detailing conformity", "Konformität der konstruktiven Durchbildung ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-member-detailing".into()]
    }
}
