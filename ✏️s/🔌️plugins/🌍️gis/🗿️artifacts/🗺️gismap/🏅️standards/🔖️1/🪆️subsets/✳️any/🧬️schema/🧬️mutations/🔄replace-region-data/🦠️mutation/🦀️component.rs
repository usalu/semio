//! 🔁️ `replace-region-data` mutation payload — whole-value swaps a region
//! feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change`
//! isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule).
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔁️ Replaces the `data` payload of the `regions` entry addressed by `id`. Diff/inverse
/// delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-region-data")]
pub struct ReplaceRegionData {
    pub id: String,
    pub new_data: dsl::DslValue,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReplaceRegionData {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "region-data", kind: "replace-region-data", record: "ReplacedRegionData" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace region \"{}\" data", self.id)
    }
}
//#endregion 🔹Payload
