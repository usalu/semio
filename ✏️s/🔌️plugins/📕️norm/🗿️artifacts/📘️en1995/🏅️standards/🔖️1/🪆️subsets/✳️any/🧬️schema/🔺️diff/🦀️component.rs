//! 🧬️ EN 1995 diff schema — sparse field delta.

use crate::artifacts::en1995::schema::En1995Artifact as EnArtifact;
use crate::artifacts::en1995::En1995Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1995::schema::En1995Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub n_ed_kn: Option<f64>,
    #[state(artifact)]
    pub v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub w_mm3: Option<f64>,
    #[state(artifact)]
    pub a_mm2: Option<f64>,
    #[state(artifact)]
    pub b_mm: Option<f64>,
    #[state(artifact)]
    pub h_mm: Option<f64>,
    #[state(artifact)]
    pub f_m_k: Option<f64>,
    #[state(artifact)]
    pub f_c_0_k: Option<f64>,
    #[state(artifact)]
    pub service_class: Option<String>,
    #[state(artifact)]
    pub load_duration: Option<String>,
    #[state(artifact)]
    pub m_crit_knm: Option<f64>,
    #[state(artifact)]
    pub f_ed_kn: Option<f64>,
    #[state(artifact)]
    pub a_ef_mm2: Option<f64>,
    #[state(artifact)]
    pub f_v_k: Option<f64>,
    #[state(artifact)]
    pub fire_duration_min: Option<f64>,
    #[state(artifact)]
    pub section_depth_mm: Option<f64>,
    #[state(artifact)]
    pub a_vert_m_s2: Option<f64>,
    #[state(artifact)]
    pub n_cycles_bridge: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
