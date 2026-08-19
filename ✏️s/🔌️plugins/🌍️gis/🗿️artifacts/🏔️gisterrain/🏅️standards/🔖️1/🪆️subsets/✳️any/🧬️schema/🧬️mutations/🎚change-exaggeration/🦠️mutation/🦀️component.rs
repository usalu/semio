//! 🎚️ `change-exaggeration` mutation payload — sets the terrain's vertical exaggeration scalar.
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🎚️ Sets `GisTerrainSnapshot::exaggeration` to `new_exaggeration`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-exaggeration")]
pub struct ChangeExaggeration {
    pub new_exaggeration: f64,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeExaggeration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "exaggeration", kind: "change-exaggeration", record: "ChangedExaggeration" };

    async fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change terrain exaggeration to {}", self.new_exaggeration)
    }
}
//#endregion 🔹Payload
