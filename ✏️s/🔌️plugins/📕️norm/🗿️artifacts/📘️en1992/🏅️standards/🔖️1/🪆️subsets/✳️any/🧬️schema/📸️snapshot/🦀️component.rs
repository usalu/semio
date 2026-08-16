//! 🧬️ En1992 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
pub mod part_1_2 {
    pub use crate::artifacts::en1992::part_1_2::FireRating;
}
pub mod part_3 {
    pub use crate::artifacts::en1992::part_3::TightnessClass;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1992", layout = "lines")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub f_ck: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub b_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub d_mm: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub a_s_mm2: f64,
    #[state(artifact)]
    pub f_yk: f64,
    #[state(artifact)]
    pub rho_l: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub p_kn: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub a_c_mm2: f64,
    #[state(artifact)]
    pub use_fem: bool,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub span_m: f64,
    #[state(artifact)]
    pub udl_kn_m: f64,
    #[state(artifact)]
    pub fire_rating: part_1_2::FireRating,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub provided_axis_distance_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub bridge_sigma_c_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub bridge_delta_sigma_s_mpa: f64,
    #[state(artifact)]
    pub tightness_class: part_3::TightnessClass,
    #[state(artifact)]
    pub hd_over_h: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub liquid_sigma_s_mpa: f64,
    #[state(artifact)]
    pub liquid_rho_p_eff: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub liquid_f_ct_eff_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub liquid_e_s_mpa: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub liquid_s_r_max_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub anchor_h_ef_mm: f64,
    #[state(artifact)]
    pub anchor_cracked: bool,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub anchor_f_uk_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub anchor_f_yk_mpa: f64,
    #[dsl(unit = "mm2")]
    #[state(artifact)]
    pub anchor_a_s_mm2: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub anchor_d_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub anchor_c1_mm: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub anchor_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub anchor_v_ed_kn: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1992Snapshot, extension = "en1992", envelope_id = "norm.en1992");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1992Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 120.0,
            v_ed_kn: 80.0,
            f_ck: 30.0,
            b_mm: 300.0,
            d_mm: 450.0,
            a_s_mm2: 1200.0,
            f_yk: 500.0,
            rho_l: 0.01,
            n_ed_kn: 0.0,
            p_kn: 0.0,
            a_c_mm2: 135_000.0,
            use_fem: false,
            span_m: 6.0,
            udl_kn_m: 20.0,
            fire_rating: part_1_2::FireRating::R60,
            provided_axis_distance_mm: 30.0,
            bridge_sigma_c_mpa: 12.0,
            bridge_delta_sigma_s_mpa: 100.0,
            tightness_class: part_3::TightnessClass::Tc1,
            hd_over_h: 10.0,
            liquid_sigma_s_mpa: 200.0,
            liquid_rho_p_eff: 0.01,
            liquid_f_ct_eff_mpa: 2.9,
            liquid_e_s_mpa: 200_000.0,
            liquid_s_r_max_mm: 250.0,
            anchor_h_ef_mm: 80.0,
            anchor_cracked: false,
            anchor_f_uk_mpa: 800.0,
            anchor_f_yk_mpa: 640.0,
            anchor_a_s_mm2: 84.3,
            anchor_d_mm: 12.0,
            anchor_c1_mm: 100.0,
            anchor_n_ed_kn: 10.0,
            anchor_v_ed_kn: 5.0,
        }
    }
}
//#endregion 🔖️Snapshot
