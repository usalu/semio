//! 🧬️ EN 1999 diff schema — sparse field delta.

use crate::artifacts::en1999::schema::En1999Artifact as EnArtifact;
use crate::artifacts::en1999::En1999Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1999::schema::En1999Artifact>>,
    #[state(persistent)] pub n_ed_kn: Option<f64>,
    #[state(persistent)] pub m_ed_knm: Option<f64>,
    #[state(persistent)] pub a_mm2: Option<f64>,
    #[state(persistent)] pub w_el_mm3: Option<f64>,
    #[state(persistent)] pub alloy: Option<String>,
    #[state(persistent)] pub chi: Option<f64>,
    #[state(persistent)] pub i_t_mm4: Option<f64>,
    #[state(persistent)] pub l_cr_mm: Option<f64>,
    #[state(persistent)] pub theta_c: Option<f64>,
    #[state(persistent)] pub delta_sigma_ed: Option<f64>,
    #[state(persistent)] pub delta_sigma_c: Option<f64>,
    #[state(persistent)] pub fatigue_m: Option<f64>,
    #[state(persistent)] pub n_cycles: Option<f64>,
    #[state(persistent)] pub v_weld_ed_kn: Option<f64>,
    #[state(persistent)] pub weld_throat_mm: Option<f64>,
    #[state(persistent)] pub weld_length_mm: Option<f64>,
    #[state(persistent)] pub beta_w: Option<f64>,
    #[state(persistent)] pub sheet_b_mm: Option<f64>,
    #[state(persistent)] pub sheet_t_mm: Option<f64>,
    #[state(persistent)] pub sheet_k_sigma: Option<f64>,
    #[state(persistent)] pub sheet_w_el_mm3: Option<f64>,
    #[state(persistent)] pub sheet_m_ed_knm: Option<f64>,
    #[state(persistent)] pub shell_t_mm: Option<f64>,
    #[state(persistent)] pub shell_r_mm: Option<f64>,
    #[state(persistent)] pub sigma_ed_shell_mpa: Option<f64>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
