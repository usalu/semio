//! 🌋️ EN 1998 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1998 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1998", layout = "lines")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Snapshot {
    #[state(persistent)]
    pub seismic_zone: u8,
    #[state(persistent)]
    pub ground_type: String,
    #[state(persistent)]
    pub importance_class: String,
    #[state(persistent)]
    pub structural_system: String,
    #[state(persistent)]
    pub t1_s: f64,
    #[state(persistent)]
    pub mass_t: f64,
    #[state(persistent)]
    pub v_rd_kn: f64,
    #[state(persistent)]
    pub drift_mm: f64,
    #[state(persistent)]
    pub height_m: f64,
    #[state(persistent)]
    pub multiple_resisting_systems: bool,
    #[state(persistent)]
    pub annex: String,
    #[state(persistent)]
    pub en_a_gr: f64,
    #[state(persistent)]
    pub en_ground_type: String,
    #[state(persistent)]
    pub en_spectrum_type: String,
    #[state(persistent)]
    pub period_ratio: f64,
    #[state(persistent)]
    pub bridge_v_rd_kn: f64,
    #[state(persistent)]
    pub bearing_d_ed_mm: f64,
    #[state(persistent)]
    pub bearing_d_rd_mm: f64,
    #[state(persistent)]
    pub retrofit_knowledge_level: String,
    #[state(persistent)]
    pub retrofit_limit_state: String,
    #[state(persistent)]
    pub retrofit_e_d_kn: f64,
    #[state(persistent)]
    pub retrofit_r_k_kn: f64,
    #[state(persistent)]
    pub retrofit_gamma_el: f64,
    #[state(persistent)]
    pub silo_height_m: f64,
    #[state(persistent)]
    pub silo_radius_m: f64,
    #[state(persistent)]
    pub silo_n_rd_kn: f64,
    #[state(persistent)]
    pub silo_v_ed_kn: f64,
    #[state(persistent)]
    pub silo_v_rd_kn: f64,
    #[state(persistent)]
    pub silo_q_nominal: f64,
    #[state(persistent)]
    pub tank_height_m: f64,
    #[state(persistent)]
    pub tank_radius_m: f64,
    #[state(persistent)]
    pub tank_mass_t: f64,
    #[state(persistent)]
    pub tank_v_rd_kn: f64,
    #[state(persistent)]
    pub tower_m_ed_knm: f64,
    #[state(persistent)]
    pub tower_m_rd_knm: f64,
    #[state(persistent)]
    pub tower_is_chimney: bool,
    #[state(persistent)]
    pub tower_q_nominal: f64,
    #[state(persistent)]
    pub tower_mass_t: f64,
    #[state(persistent)]
    pub foundation_area_m2: f64,
    #[state(persistent)]
    pub foundation_p_rd_kpa: f64,
    #[state(persistent)]
    pub foundation_h_ed_kn: f64,
    #[state(persistent)]
    pub foundation_h_rd_kn: f64,
    #[state(persistent)]
    pub k_foundation: f64,
    #[state(persistent)]
    pub k_soil: f64,
    #[state(persistent)]
    pub wall_height_m: f64,
    #[state(persistent)]
    pub wall_phi_deg: f64,
    #[state(persistent)]
    pub wall_soil_gamma_kn_m3: f64,
    #[state(persistent)]
    pub wall_r: f64,
    #[state(persistent)]
    pub wall_h_rd_kn: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedEn1998SnapshotCodecs
/// ✉️ P6 handcrafted En1998SnapshotDsl/En1998SnapshotPack (derive no longer emits these traits).
impl store::En1998SnapshotDsl for En1998Snapshot {
    const EXTENSION: &'static str = "en1998";
    fn envelope_id() -> &'static str { "norm.en1998" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::En1998Snapshot },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::En1998Snapshot);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::En1998SnapshotDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::En1998SnapshotPack for En1998Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::En1998SnapshotDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::En1998SnapshotDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::En1998SnapshotDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedEn1998SnapshotCodecs


impl Default for En1998Snapshot {
    fn default() -> Self {
        Self {
            seismic_zone: 2,
            ground_type: "b".into(),
            importance_class: "cc2".into(),
            structural_system: "moment_frame_dch".into(),
            t1_s: 0.3,
            mass_t: 500.0,
            v_rd_kn: 800.0,
            drift_mm: 20.0,
            height_m: 12.0,
            multiple_resisting_systems: true,
            annex: "de".into(),
            en_a_gr: 0.15,
            en_ground_type: "b".into(),
            en_spectrum_type: "type1".into(),
            period_ratio: 2.0,
            bridge_v_rd_kn: 600.0,
            bearing_d_ed_mm: 120.0,
            bearing_d_rd_mm: 250.0,
            retrofit_knowledge_level: "kl2".into(),
            retrofit_limit_state: "significant_damage".into(),
            retrofit_e_d_kn: 250.0,
            retrofit_r_k_kn: 400.0,
            retrofit_gamma_el: 1.0,
            silo_height_m: 10.0,
            silo_radius_m: 5.0,
            silo_n_rd_kn: 500.0,
            silo_v_ed_kn: 180.0,
            silo_v_rd_kn: 300.0,
            silo_q_nominal: 2.0,
            tank_height_m: 8.0,
            tank_radius_m: 4.0,
            tank_mass_t: 300.0,
            tank_v_rd_kn: 400.0,
            tower_m_ed_knm: 1200.0,
            tower_m_rd_knm: 2500.0,
            tower_is_chimney: true,
            tower_q_nominal: 2.5,
            tower_mass_t: 80.0,
            foundation_area_m2: 100.0,
            foundation_p_rd_kpa: 500.0,
            foundation_h_ed_kn: 150.0,
            foundation_h_rd_kn: 400.0,
            k_foundation: 500_000.0,
            k_soil: 200_000.0,
            wall_height_m: 4.0,
            wall_phi_deg: 30.0,
            wall_soil_gamma_kn_m3: 18.0,
            wall_r: 1.5,
            wall_h_rd_kn: 150.0,
        }
    }
}
