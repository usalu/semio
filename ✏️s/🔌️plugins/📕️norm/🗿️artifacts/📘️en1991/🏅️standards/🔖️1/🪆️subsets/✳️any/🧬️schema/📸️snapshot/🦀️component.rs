//! 🧬️ En1991 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::{AnnexChoice, ImposedCategory};
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
pub mod part_1_2 {
    pub use crate::artifacts::en1991::part_1_2::FireCurve;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1991", layout = "lines")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Snapshot {
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub area_m2: f64,
    #[state(artifact)]
    pub category: ImposedCategory,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub self_weight_material: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub self_weight_thickness_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(artifact)]
    pub assumed_g_k_kn_m2: f64,
    #[state(artifact)]
    pub fire_curve: part_1_2::FireCurve,
    #[state(artifact)]
    pub fire_resistance_min: f64,
    #[state(artifact)]
    pub fire_member_capacity_c: f64,
    #[state(artifact)]
    pub snow_zone: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub snow_altitude_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(artifact)]
    pub en_s_k_kn_m2: f64,
    #[state(artifact)]
    pub wind_zone: u8,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub en_v_b_m_s: f64,
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub delta_t_k: f64,
    #[state(artifact)]
    pub construction_activity: String,
    #[dsl(unit = "t")]
    #[state(artifact)]
    pub accidental_mass_t: f64,
    #[state(artifact)]
    pub accidental_speed_km_h: f64,
    #[state(artifact)]
    pub bridge_lane: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_span_m: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_lane_width_m: f64,
    #[state(artifact)]
    pub bridge_moment_resistance_knm: f64,
    #[state(artifact)]
    pub crane_class: String,
    #[state(artifact)]
    pub hoist_class: String,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub hoisting_speed_m_s: f64,
    #[state(artifact)]
    pub silo_bulk_density_kn_m3: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_height_m: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_hydraulic_radius_m: f64,
    #[state(artifact)]
    pub silo_mu: f64,
    #[state(artifact)]
    pub silo_k: f64,
    #[state(artifact)]
    pub c_s: f64,
    #[state(artifact)]
    pub c_d: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1991Snapshot, extension = "en1991", envelope_id = "norm.en1991");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1991Snapshot {
    fn default() -> Self {
        Self {
            area_m2: 50.0,
            category: ImposedCategory::B,
            annex: AnnexChoice::De,
            self_weight_material: "reinforced_concrete".into(),
            self_weight_thickness_m: 0.2,
            assumed_g_k_kn_m2: 6.0,
            fire_curve: part_1_2::FireCurve::Standard,
            fire_resistance_min: 30.0,
            fire_member_capacity_c: 900.0,
            snow_zone: 2,
            snow_altitude_m: 150.0,
            en_s_k_kn_m2: 0.85,
            wind_zone: 2,
            en_v_b_m_s: 25.0,
            delta_t_k: 30.0,
            construction_activity: "scaffolding".into(),
            accidental_mass_t: 30.0,
            accidental_speed_km_h: 80.0,
            bridge_lane: 1,
            bridge_span_m: 20.0,
            bridge_lane_width_m: 3.0,
            bridge_moment_resistance_knm: 3000.0,
            crane_class: "HC2".into(),
            hoist_class: "HC2".into(),
            hoisting_speed_m_s: 0.5,
            silo_bulk_density_kn_m3: 8.0,
            silo_height_m: 12.0,
            silo_hydraulic_radius_m: 1.5,
            silo_mu: 0.4,
            silo_k: 0.4,
            c_s: 1.0,
            c_d: 1.0,
        }
    }
}
//#endregion 🔖️Snapshot
