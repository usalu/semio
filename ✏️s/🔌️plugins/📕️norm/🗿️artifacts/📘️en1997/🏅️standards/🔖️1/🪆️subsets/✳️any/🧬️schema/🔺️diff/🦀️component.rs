//! 🧬️ EN 1997 diff schema — sparse field delta.

use crate::artifacts::en1997::schema::En1997Artifact as EnArtifact;
use crate::artifacts::en1997::En1997Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1997::schema::En1997Artifact>>,
    #[state(persistent)] pub v_ed_kn: Option<f64>,
    #[state(persistent)] pub h_ed_kn: Option<f64>,
    #[state(persistent)] pub footing_area_m2: Option<f64>,
    #[state(persistent)] pub phi_deg: Option<f64>,
    #[state(persistent)] pub c_kpa: Option<f64>,
    #[state(persistent)] pub gamma_kn_m3: Option<f64>,
    #[state(persistent)] pub b_m: Option<f64>,
    #[state(persistent)] pub d_f_m: Option<f64>,
    #[state(persistent)] pub e_s_mpa: Option<f64>,
    #[state(persistent)] pub nu: Option<f64>,
    #[state(persistent)] pub design_approach: Option<String>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub settlement_limit_mm: Option<f64>,
    #[state(persistent)] pub n_pile_ed_kn: Option<f64>,
    #[state(persistent)] pub alpha_s: Option<f64>,
    #[state(persistent)] pub pile_d_m: Option<f64>,
    #[state(persistent)] pub q_s_kpa: Option<f64>,
    #[state(persistent)] pub pile_l_m: Option<f64>,
    #[state(persistent)] pub q_b_kpa: Option<f64>,
    #[state(persistent)] pub pile_base_area_m2: Option<f64>,
    #[state(persistent)] pub pile_n_profiles: Option<u32>,
    #[state(persistent)] pub z_investigated_m: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
