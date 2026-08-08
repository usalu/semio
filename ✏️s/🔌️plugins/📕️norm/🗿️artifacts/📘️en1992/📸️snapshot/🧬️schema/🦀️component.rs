//! 🧬️ En1992 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }
}
pub mod part_3 {
    use super::*;

    /// 💧️ Tightness class per EN 1992-3 Table 7.105: required degree of protection against leakage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum TightnessClass {
        Tc0,
        Tc1,
        Tc2,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1992", layout = "lines")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Snapshot {
    pub annex: AnnexChoice,
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    pub v_ed_kn: f64,
    pub f_ck: f64,
    #[dsl(unit = "mm")]
    pub b_mm: f64,
    #[dsl(unit = "mm")]
    pub d_mm: f64,
    #[dsl(unit = "mm2")]
    pub a_s_mm2: f64,
    pub f_yk: f64,
    pub rho_l: f64,
    #[dsl(unit = "kN")]
    pub n_ed_kn: f64,
    #[dsl(unit = "kN")]
    pub p_kn: f64,
    #[dsl(unit = "mm2")]
    pub a_c_mm2: f64,
    pub use_fem: bool,
    #[dsl(unit = "m")]
    pub span_m: f64,
    pub udl_kn_m: f64,
    pub fire_rating: part_1_2::FireRating,
    #[dsl(unit = "mm")]
    pub provided_axis_distance_mm: f64,
    #[dsl(unit = "MPa")]
    pub bridge_sigma_c_mpa: f64,
    #[dsl(unit = "MPa")]
    pub bridge_delta_sigma_s_mpa: f64,
    pub tightness_class: part_3::TightnessClass,
    pub hd_over_h: f64,
    #[dsl(unit = "MPa")]
    pub liquid_sigma_s_mpa: f64,
    pub liquid_rho_p_eff: f64,
    #[dsl(unit = "MPa")]
    pub liquid_f_ct_eff_mpa: f64,
    #[dsl(unit = "MPa")]
    pub liquid_e_s_mpa: f64,
    #[dsl(unit = "mm")]
    pub liquid_s_r_max_mm: f64,
    #[dsl(unit = "mm")]
    pub anchor_h_ef_mm: f64,
    pub anchor_cracked: bool,
    #[dsl(unit = "MPa")]
    pub anchor_f_uk_mpa: f64,
    #[dsl(unit = "MPa")]
    pub anchor_f_yk_mpa: f64,
    #[dsl(unit = "mm2")]
    pub anchor_a_s_mm2: f64,
    #[dsl(unit = "mm")]
    pub anchor_d_mm: f64,
    #[dsl(unit = "mm")]
    pub anchor_c1_mm: f64,
    #[dsl(unit = "kN")]
    pub anchor_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    pub anchor_v_ed_kn: f64,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted En1992SnapshotDsl/En1992SnapshotPack (derive no longer emits these traits).
impl store::DocumentDsl for En1992Snapshot {
    const EXTENSION: &'static str = "en1992";
    fn envelope_id() -> &'static str { "norm.en1992" }
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
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for En1992Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs

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
