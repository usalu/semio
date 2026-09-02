//! 🔀️ `reorder-regions` mutation payload — repositions a region feature within `regions`
//! by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair).

use crate::artifacts::gismap::GisMapSnapshot;
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🔀️ Moves the `regions` entry addressed by `id` to `to_index`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-regions")]
pub struct ReorderRegions {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReorderRegions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "regions", kind: "reorder-regions", record: "ReorderedRegions" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder region \"{}\" to {}", self.id, self.to_index)
    }
}
//#endregion 🔹Payload
