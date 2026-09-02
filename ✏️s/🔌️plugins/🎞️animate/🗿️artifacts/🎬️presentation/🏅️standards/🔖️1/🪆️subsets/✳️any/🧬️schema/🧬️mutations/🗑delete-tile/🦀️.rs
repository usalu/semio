//! 🗑️ `delete-tile` mutation payload — removes a figure tile crop from `tiles` by id.

use crate::artifacts::presentation::PresentationSnapshot;
use crate::artifacts::presentation::diff::PresentationDiff;
use crate::artifacts::presentation::mutations::PresentationMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🗑️ Removes the `tiles` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-tile")]
pub struct DeleteTile {
    pub id: String,
}

impl MutationKind<PresentationSnapshot, PresentationMutation> for DeleteTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn diff(&self, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete tile \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone()]
    }
}
//#endregion 🔹Payload
