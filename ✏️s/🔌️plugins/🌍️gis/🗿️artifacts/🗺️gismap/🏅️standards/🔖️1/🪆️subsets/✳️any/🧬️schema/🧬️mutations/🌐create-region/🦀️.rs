//! 🆕️ `create-region` mutation payload — adds a new region feature to `regions`.

use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🆕️ Inserts `item` into `regions` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-region")]
pub struct CreateRegion {
    pub index: usize,
    #[dsl(block)]
    pub item: MapFeature,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for CreateRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "region", kind: "create-region", record: "CreatedRegion" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create region \"{}\"", self.item.id)
    }
}
//#endregion 🔹Payload
