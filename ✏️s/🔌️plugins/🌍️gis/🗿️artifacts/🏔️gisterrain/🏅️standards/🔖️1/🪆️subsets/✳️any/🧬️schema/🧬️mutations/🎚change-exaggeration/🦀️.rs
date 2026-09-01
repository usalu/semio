//! 🎚️ Direct `change-exaggeration` mutation owner.
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🎚️ Sets `GisTerrainSnapshot::exaggeration` to `new_exaggeration`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-exaggeration")]
pub struct ChangeExaggeration {
    pub new_exaggeration: f64,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeExaggeration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "exaggeration", kind: "change-exaggeration", record: "ChangedExaggeration" };

    fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change terrain exaggeration to {}", self.new_exaggeration)
    }
}
//#endregion 🔹Payload
