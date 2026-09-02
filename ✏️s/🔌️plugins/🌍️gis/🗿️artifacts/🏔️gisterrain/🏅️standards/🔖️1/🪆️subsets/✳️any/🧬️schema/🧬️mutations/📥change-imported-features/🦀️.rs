//! 📥️ Direct `change-imported-features` mutation owner — sets the terrain's last-imported `2d.map`
//! descriptor JSON (the `map:in` insertion point).
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 📥️ Sets `GisTerrainSnapshot::imported_features_json` to `new_imported_features_json`. Diff/
/// inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-imported-features")]
pub struct ChangeImportedFeatures {
    #[dsl(key = "new-imported-features-json")]
    pub new_imported_features_json: String,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeImportedFeatures {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "imported-features", kind: "change-imported-features", record: "ChangedImportedFeatures" };

    fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Change imported terrain features".to_string()
    }
}
//#endregion 🔹Payload
