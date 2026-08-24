//! 🆕️ `create-position` mutation payload — adds a new position feature to `positions`.
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🆕️ Inserts `item` into `positions` at `index` (FINAL-state, per the taxonomy's index-addressing
/// law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-position")]
pub struct CreatePosition {
    pub index: usize,
    #[dsl(block)]
    pub item: MapFeature,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for CreatePosition {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "position", kind: "create-position", record: "CreatedPosition" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create position \"{}\"", self.item.id)
    }
}
//#endregion 🔹Payload
