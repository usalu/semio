//! 🆕️ `create-route` mutation payload — adds a new route feature to `routes`.

use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🆕️ Inserts `item` into `routes` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-route")]
pub struct CreateRoute {
    pub index: usize,
    #[dsl(block)]
    pub item: MapFeature,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for CreateRoute {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "route", kind: "create-route", record: "CreatedRoute" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create route \"{}\"", self.item.id)
    }
}
//#endregion 🔹Payload
