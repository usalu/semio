//! 🔀️ `reorder-positions` mutation payload — repositions a position feature within `positions`
//! by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair).

use crate::GisMapSnapshot;
use crate::diff::GisMapDiff;
use crate::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🔀️ Moves the `positions` entry addressed by `id` to `to_index`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-positions")]
pub struct ReorderPositions {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReorderPositions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "positions", kind: "reorder-positions", record: "ReorderedPositions" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder position \"{}\" to {}", self.id, self.to_index)
    }
}
//#endregion 🔹Payload
