//! 🧬️ En1990 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use crate::artifacts::en1990::En1990QkEntry;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1990 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1990::schema::En1990Artifact>>,
    #[state(persistent)] pub g_k: Option<f64>,
    #[state(persistent)] pub q_k: Option<En1990QkList>,
    #[state(persistent)] pub resistance_kn: Option<f64>,
    #[state(persistent)] pub consequence_class: Option<u8>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub seismic_a_ed_kn: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1990StringList { pub values: Vec<String> }

/// 📋 Qk table wrapper for optional list diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1990QkList { pub values: Vec<En1990QkEntry> }
//#endregion 🔖️DeltaHelpers
