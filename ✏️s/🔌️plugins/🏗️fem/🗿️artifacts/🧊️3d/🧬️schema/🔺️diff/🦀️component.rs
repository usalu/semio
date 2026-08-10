//! 🧬️ Fem3d diff schema — sparse field delta over the artifact.

use crate::artifacts::fem3d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSupport, FemSolid, };
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the fem3d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::fem3d::schema::Fem3dArtifact>>,
    #[state(persistent)] pub nodes: Option<Fem3dNodesDelta>,
    #[state(persistent)] pub elements: Option<Fem3dElementsDelta>,
    #[state(persistent)] pub materials: Option<Fem3dMaterialsDelta>,
    #[state(persistent)] pub sections: Option<Fem3dSectionsDelta>,
    #[state(persistent)] pub solids: Option<Fem3dSolidsDelta>,
    #[state(persistent)] pub supports: Option<Fem3dSupportsDelta>,
    #[state(persistent)] pub load_cases: Option<Fem3dLoadCasesDelta>,
    #[state(persistent)] pub combinations: Option<Fem3dCombinationsDelta>,
    #[state(persistent)] pub analysis: Option<FemAnalysisSettings>,
    #[state(shared_ui)] pub result_source_id: Option<Option<String>>,
    #[state(shared_ui)] pub result_mode: Option<String>,
    #[state(shared_ui)] pub result_mode_index: Option<u32>,
    #[state(local_ui)] pub camera: Option<FemCamera>,
    #[state(preview)] pub solver_results_json: Option<String>,
    #[state(preview)] pub mesh_preview_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dNodesDelta {
    pub added: Vec<FemNode>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dNodesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `nodes` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dNodesPatchEntry {
    pub id: String,
    pub item: FemNode,
}

/// 🧩 Identified-collection delta for `elements`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dElementsDelta {
    pub added: Vec<FemElement>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dElementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `elements` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dElementsPatchEntry {
    pub id: String,
    pub item: FemElement,
}

/// 🧩 Identified-collection delta for `materials`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dMaterialsDelta {
    pub added: Vec<FemMaterial>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dMaterialsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `materials` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dMaterialsPatchEntry {
    pub id: String,
    pub item: FemMaterial,
}

/// 🧩 Identified-collection delta for `sections`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dSectionsDelta {
    pub added: Vec<FemSection>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSectionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `sections` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dSectionsPatchEntry {
    pub id: String,
    pub item: FemSection,
}

/// 🧩 Identified-collection delta for `solids`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dSolidsDelta {
    pub added: Vec<FemSolid>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSolidsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `solids` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dSolidsPatchEntry {
    pub id: String,
    pub item: FemSolid,
}

/// 🧩 Identified-collection delta for `supports`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dSupportsDelta {
    pub added: Vec<FemSupport>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSupportsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `supports` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dSupportsPatchEntry {
    pub id: String,
    pub item: FemSupport,
}

/// 🧩 Identified-collection delta for `loadCases`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dLoadCasesDelta {
    pub added: Vec<FemLoadCase>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dLoadCasesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `loadCases` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dLoadCasesPatchEntry {
    pub id: String,
    pub item: FemLoadCase,
}

/// 🧩 Identified-collection delta for `combinations`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Fem3dCombinationsDelta {
    pub added: Vec<FemCombination>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dCombinationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `combinations` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dCombinationsPatchEntry {
    pub id: String,
    pub item: FemCombination,
}

//#endregion 🔖️DeltaHelpers
