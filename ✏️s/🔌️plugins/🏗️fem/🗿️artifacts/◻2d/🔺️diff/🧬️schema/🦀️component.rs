//! 🧬️ Fem2d diff schema — sparse field delta over the artifact.

use crate::artifacts::fem2d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, FemRegion, };
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the fem2d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::fem2d::schema::Fem2dArtifact>>,
    #[state(persistent)] pub nodes: Option<Fem2dNodesDelta>,
    #[state(persistent)] pub elements: Option<Fem2dElementsDelta>,
    #[state(persistent)] pub regions: Option<Fem2dRegionsDelta>,
    #[state(persistent)] pub materials: Option<Fem2dMaterialsDelta>,
    #[state(persistent)] pub sections: Option<Fem2dSectionsDelta>,
    #[state(persistent)] pub supports: Option<Fem2dSupportsDelta>,
    #[state(persistent)] pub load_cases: Option<Fem2dLoadCasesDelta>,
    #[state(persistent)] pub combinations: Option<Fem2dCombinationsDelta>,
    #[state(persistent)] pub analysis: Option<FemAnalysisSettings>,
    #[state(shared_ui)] pub result_source_id: Option<Option<String>>,
    #[state(shared_ui)] pub result_mode: Option<String>,
    #[state(shared_ui)] pub result_mode_index: Option<u32>,
    #[state(local_ui)] pub camera: Option<FemCamera>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub solver_results_json: Option<String>,
    #[state(preview)] pub mesh_preview_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dNodesDelta {
    pub added: Vec<FemNode>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dNodesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `nodes` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dNodesPatchEntry {
    pub id: String,
    pub item: FemNode,
}

/// 🧩 Identified-collection delta for `elements`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dElementsDelta {
    pub added: Vec<FemElement>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dElementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `elements` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dElementsPatchEntry {
    pub id: String,
    pub item: FemElement,
}

/// 🧩 Identified-collection delta for `regions`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dRegionsDelta {
    pub added: Vec<FemRegion>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dRegionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `regions` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dRegionsPatchEntry {
    pub id: String,
    pub item: FemRegion,
}

/// 🧩 Identified-collection delta for `materials`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dMaterialsDelta {
    pub added: Vec<FemMaterial>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dMaterialsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `materials` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dMaterialsPatchEntry {
    pub id: String,
    pub item: FemMaterial,
}

/// 🧩 Identified-collection delta for `sections`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dSectionsDelta {
    pub added: Vec<FemSection>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dSectionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `sections` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dSectionsPatchEntry {
    pub id: String,
    pub item: FemSection,
}

/// 🧩 Identified-collection delta for `supports`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dSupportsDelta {
    pub added: Vec<FemSupport>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dSupportsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `supports` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dSupportsPatchEntry {
    pub id: String,
    pub item: FemSupport,
}

/// 🧩 Identified-collection delta for `loadCases`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dLoadCasesDelta {
    pub added: Vec<FemLoadCase>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dLoadCasesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `loadCases` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dLoadCasesPatchEntry {
    pub id: String,
    pub item: FemLoadCase,
}

/// 🧩 Identified-collection delta for `combinations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem2dCombinationsDelta {
    pub added: Vec<FemCombination>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem2dCombinationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `combinations` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dCombinationsPatchEntry {
    pub id: String,
    pub item: FemCombination,
}

//#endregion 🔖️DeltaHelpers
