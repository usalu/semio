//! 🧬️ `change-delta-u-wb` mutation leaf.

use crate::diff::Din18599Diff;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeDeltaUWb {
    pub new_delta_u_wb_w_m2k: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeDeltaUWb {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "delta-u-wb",
        kind: "change-delta-u-wb",
        record: "ChangedDeltaUWb",
    };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change delta-u-wb", "delta-u-wb ändern")
    }
}
