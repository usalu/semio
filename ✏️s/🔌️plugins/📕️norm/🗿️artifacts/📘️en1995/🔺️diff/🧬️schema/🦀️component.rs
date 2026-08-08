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
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1995::schema::En1995Artifact>>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub m_ed_knm: Option<f64>,
    #[state(persistent)] pub n_ed_kn: Option<f64>,
    #[state(persistent)] pub v_ed_kn: Option<f64>,
    #[state(persistent)] pub w_mm3: Option<f64>,
    #[state(persistent)] pub a_mm2: Option<f64>,
    #[state(persistent)] pub b_mm: Option<f64>,
    #[state(persistent)] pub h_mm: Option<f64>,
    #[state(persistent)] pub f_m_k: Option<f64>,
    #[state(persistent)] pub f_c_0_k: Option<f64>,
    #[state(persistent)] pub service_class: Option<String>,
    #[state(persistent)] pub load_duration: Option<String>,
    #[state(persistent)] pub m_crit_knm: Option<f64>,
    #[state(persistent)] pub f_ed_kn: Option<f64>,
    #[state(persistent)] pub a_ef_mm2: Option<f64>,
    #[state(persistent)] pub f_v_k: Option<f64>,
    #[state(persistent)] pub fire_duration_min: Option<f64>,
    #[state(persistent)] pub section_depth_mm: Option<f64>,
    #[state(persistent)] pub a_vert_m_s2: Option<f64>,
    #[state(persistent)] pub n_cycles_bridge: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
