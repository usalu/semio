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
    #[state(persistent)]
    pub area_m2: f64,
    #[state(persistent)]
    pub category: ImposedCategory,
    #[state(persistent)]
    pub annex: AnnexChoice,
    #[state(persistent)]
    pub self_weight_material: String,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub self_weight_thickness_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(persistent)]
    pub assumed_g_k_kn_m2: f64,
    #[state(persistent)]
    pub fire_curve: part_1_2::FireCurve,
    #[state(persistent)]
    pub fire_resistance_min: f64,
    #[state(persistent)]
    pub fire_member_capacity_c: f64,
    #[state(persistent)]
    pub snow_zone: u8,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub snow_altitude_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(persistent)]
    pub en_s_k_kn_m2: f64,
    #[state(persistent)]
    pub wind_zone: u8,
    #[dsl(unit = "m/s")]
    #[state(persistent)]
    pub en_v_b_m_s: f64,
    #[dsl(unit = "K")]
    #[state(persistent)]
    pub delta_t_k: f64,
    #[state(persistent)]
    pub construction_activity: String,
    #[dsl(unit = "t")]
    #[state(persistent)]
    pub accidental_mass_t: f64,
    #[state(persistent)]
    pub accidental_speed_km_h: f64,
    #[state(persistent)]
    pub bridge_lane: u8,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub bridge_span_m: f64,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub bridge_lane_width_m: f64,
    #[state(persistent)]
    pub bridge_moment_resistance_knm: f64,
    #[state(persistent)]
    pub crane_class: String,
    #[state(persistent)]
    pub hoist_class: String,
    #[dsl(unit = "m/s")]
    #[state(persistent)]
    pub hoisting_speed_m_s: f64,
    #[state(persistent)]
    pub silo_bulk_density_kn_m3: f64,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub silo_height_m: f64,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub silo_hydraulic_radius_m: f64,
    #[state(persistent)]
    pub silo_mu: f64,
    #[state(persistent)]
    pub silo_k: f64,
    #[state(persistent)]
    pub c_s: f64,
    #[state(persistent)]
    pub c_d: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted En1991SnapshotDsl/En1991SnapshotPack (derive no longer emits these traits).
impl store::ArtifactDsl for En1991Snapshot {
    const EXTENSION: &'static str = "en1991";
    fn envelope_id() -> &'static str { "norm.en1991" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for En1991Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
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
