//! 🧬️ Iso16757 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Iso16757 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::iso16757::schema::Iso16757Artifact>>,
    #[state(persistent)] pub catalogue: Option<crate::artifacts::iso16757::part_1::Catalogue>,
    #[state(persistent)] pub dictionary: Option<crate::artifacts::iso16757::part_4::Dictionary>,
    #[state(persistent)] pub geometry: Option<crate::artifacts::iso16757::part_2::GeometryCatalogue>,
    #[state(persistent)] pub selection: Option<crate::artifacts::iso16757::part_1::SelectionRequest>,
    #[state(persistent)] pub part_number_rule: Option<crate::artifacts::iso16757::part_5::PartNumberRule>,
    #[state(persistent)] pub part_number_inputs: Option<BTreeMap<String, CatalogueValue>>,
    #[state(persistent)] pub script_limits: Option<crate::artifacts::iso16757::part_5::ScriptLimits>,
    #[state(persistent)] pub exchange_process: Option<crate::artifacts::iso16757::part_5::ExchangeProcess>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Iso16757StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
