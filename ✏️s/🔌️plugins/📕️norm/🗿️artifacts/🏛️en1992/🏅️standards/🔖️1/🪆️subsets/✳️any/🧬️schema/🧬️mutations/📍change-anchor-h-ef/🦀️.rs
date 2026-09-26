//! 🔧 `change-anchor-h-ef`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnchorHEf {
    pub anchor_id: String,
    pub new_value: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorHEf {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-h-ef", kind: "change-anchor-h-ef", record: "ChangedChangeAnchorHEf" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change {} of {}", "h_ef", self.anchor_id), &format!("{} von {} ändern", "h_ef", self.anchor_id))
    }
    fn target(&self) -> Vec<String> { vec![self.anchor_id.clone()] }
}
