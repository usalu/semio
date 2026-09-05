//! 🧬️ EN 1997 diff schema — sparse field delta.

use schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1997::schema::En1997Artifact>>,
    #[state(artifact)]
    pub v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub h_ed_kn: Option<f64>,
    #[state(artifact)]
    pub footing_area_m2: Option<f64>,
    #[state(artifact)]
    pub phi_deg: Option<f64>,
    #[state(artifact)]
    pub c_kpa: Option<f64>,
    #[state(artifact)]
    pub gamma_kn_m3: Option<f64>,
    #[state(artifact)]
    pub b_m: Option<f64>,
    #[state(artifact)]
    pub d_f_m: Option<f64>,
    #[state(artifact)]
    pub e_s_mpa: Option<f64>,
    #[state(artifact)]
    pub nu: Option<f64>,
    #[state(artifact)]
    pub design_approach: Option<String>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub settlement_limit_mm: Option<f64>,
    #[state(artifact)]
    pub n_pile_ed_kn: Option<f64>,
    #[state(artifact)]
    pub alpha_s: Option<f64>,
    #[state(artifact)]
    pub pile_d_m: Option<f64>,
    #[state(artifact)]
    pub q_s_kpa: Option<f64>,
    #[state(artifact)]
    pub pile_l_m: Option<f64>,
    #[state(artifact)]
    pub q_b_kpa: Option<f64>,
    #[state(artifact)]
    pub pile_base_area_m2: Option<f64>,
    #[state(artifact)]
    pub pile_n_profiles: Option<u32>,
    #[state(artifact)]
    pub z_investigated_m: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
