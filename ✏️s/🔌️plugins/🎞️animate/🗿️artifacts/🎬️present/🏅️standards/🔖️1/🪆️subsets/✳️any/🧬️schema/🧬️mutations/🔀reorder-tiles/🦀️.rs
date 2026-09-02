//! 🔀️ `reorder-tiles` mutation payload — repositions a figure tile within `tiles` by id (id-keyed
//! collection, so addressing is `id`+`to_index`, not a bare index pair).

use crate::artifacts::present::PresentSnapshot;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🔀️ Moves the `tiles` entry addressed by `id` to `to_index`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-tiles")]
pub struct ReorderTiles {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<PresentSnapshot, PresentMutation> for ReorderTiles {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "tiles", kind: "reorder-tiles", record: "ReorderedTiles" };

    fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder tile \"{}\" to position {}", self.id, self.to_index)
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone()]
    }
}
//#endregion 🔹Payload
