//! 🆕️ `create-route` mutation payload — adds a new route feature to `routes`.
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🆕️ Inserts `item` into `routes` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-route")]
pub struct CreateRoute {
    pub index: usize,
    #[dsl(block)]
    pub item: MapFeature,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for CreateRoute {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "route", kind: "create-route", record: "CreatedRoute" };

    fn diff(&self, base: &GisMapSnapshot) -> GisMapDiff {
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
