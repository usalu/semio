//! 🔧 `change-delta-c-dev` payload.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeDeltaCDev {
    pub new_delta_c_dev: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeDeltaCDev {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-c-dev", kind: "change-delta-c-dev", record: "ChangedDeltaCDev" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change delta-c-dev to {}", self.new_delta_c_dev), &format!("delta-c-dev auf {} ändern", self.new_delta_c_dev))
    }
}
