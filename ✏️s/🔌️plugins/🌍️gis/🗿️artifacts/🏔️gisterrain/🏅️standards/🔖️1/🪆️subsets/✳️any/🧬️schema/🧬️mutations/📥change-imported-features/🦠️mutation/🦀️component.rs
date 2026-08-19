//! 📥️ `change-imported-features` mutation payload — sets the terrain's last-imported `2d.map`
//! descriptor JSON (the `map:in` insertion point).
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 📥️ Sets `GisTerrainSnapshot::imported_features_json` to `new_imported_features_json`. Diff/
/// inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-imported-features")]
pub struct ChangeImportedFeatures {
    #[dsl(key = "new-imported-features-json")]
    pub new_imported_features_json: String,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeImportedFeatures {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "imported-features", kind: "change-imported-features", record: "ChangedImportedFeatures" };

    async fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        "Change imported terrain features".to_string()
    }
}
//#endregion 🔹Payload
