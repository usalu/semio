//! 🔁️ `replace-position-data` mutation payload — whole-value swaps a position
//! feature's opaque payload (`MapFeature::data` is deliberately untyped, so a partial `change`
//! isn't expressible — this is a `replace`, per the taxonomy's "large structured sub-payload" rule).
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔁️ Replaces the `data` payload of the `positions` entry addressed by `id`. Diff/inverse
/// delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-position-data")]
pub struct ReplacePositionData {
    pub id: String,
    pub new_data: dsl::DslValue,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReplacePositionData {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "position-data", kind: "replace-position-data", record: "ReplacedPositionData" };

    async fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Replace position \"{}\" data", self.id)
    }
}
//#endregion 🔹Payload
