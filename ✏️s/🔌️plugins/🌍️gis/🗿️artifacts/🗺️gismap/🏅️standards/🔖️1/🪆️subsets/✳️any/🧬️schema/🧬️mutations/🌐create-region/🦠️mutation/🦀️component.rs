//! 🆕️ `create-region` mutation payload — adds a new region feature to `regions`.
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🆕️ Inserts `item` into `regions` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-region")]
pub struct CreateRegion {
    pub index: usize,
    #[dsl(block)]
    pub item: MapFeature,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for CreateRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "region", kind: "create-region", record: "CreatedRegion" };

    async fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Create region \"{}\"", self.item.id)
    }
}
//#endregion 🔹Payload
