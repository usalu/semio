//! 🦎 `change-member-buckling-length` mutation leaf.

use crate::diff::En1999Diff;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;


#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberBucklingLength {
    pub member_id: String,
    pub axis: String,
    pub new_length: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeMemberBucklingLength {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "member-buckling-length",
        kind: "change-member-buckling-length",
        record: "ChangedMemberBucklingLength",
    };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change member buckling length", "Knicklänge des Bauteils ändern")
    }
}
