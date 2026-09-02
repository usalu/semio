//! 🔧 `change-anchor-h-ef-mm` payload — changes the En1992 document's `anchor_h_ef_mm` (EN 1992 input).


use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::mutations::change_anchor_h_ef_mm::ChangeAnchorHEfMm;

//#region 🔖️ChangeAnchorHEfMm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAnchorHEfMm {
    pub new_anchor_h_ef_mm: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeAnchorHEfMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "anchor-h-ef-mm", kind: "change-anchor-h-ef-mm", record: "ChangedAnchorHEfMm" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change anchor h ef mm to {:?}", self.new_anchor_h_ef_mm)
    }
}
//#endregion 🔖️ChangeAnchorHEfMm
