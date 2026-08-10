//! 🌍️ EN 1997 snapshot schema — persistent fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1997 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1997", layout = "lines")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Snapshot {
    #[state(persistent)]
    pub v_ed_kn: f64,
    #[state(persistent)]
    pub h_ed_kn: f64,
    #[state(persistent)]
    pub footing_area_m2: f64,
    #[state(persistent)]
    pub phi_deg: f64,
    #[state(persistent)]
    pub c_kpa: f64,
    #[state(persistent)]
    pub gamma_kn_m3: f64,
    #[state(persistent)]
    pub b_m: f64,
    #[state(persistent)]
    pub d_f_m: f64,
    #[state(persistent)]
    pub e_s_mpa: f64,
    #[state(persistent)]
    pub nu: f64,
    #[state(persistent)]
    pub design_approach: String,
    #[state(persistent)]
    pub annex: crate::document::AnnexChoice,
    #[state(persistent)]
    pub settlement_limit_mm: f64,
    #[state(persistent)]
    pub n_pile_ed_kn: f64,
    #[state(persistent)]
    pub alpha_s: f64,
    #[state(persistent)]
    pub pile_d_m: f64,
    #[state(persistent)]
    pub q_s_kpa: f64,
    #[state(persistent)]
    pub pile_l_m: f64,
    #[state(persistent)]
    pub q_b_kpa: f64,
    #[state(persistent)]
    pub pile_base_area_m2: f64,
    #[state(persistent)]
    pub pile_n_profiles: u32,
    #[state(persistent)]
    pub z_investigated_m: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for En1997Snapshot {
    const EXTENSION: &'static str = "en1997";
    fn envelope_id() -> &'static str { "norm.en1997" }
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

impl store::ArtifactPack for En1997Snapshot {
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


impl Default for En1997Snapshot {
    fn default() -> Self {
        Self {
            v_ed_kn: 500.0,
            h_ed_kn: 80.0,
            footing_area_m2: 2.0,
            phi_deg: 30.0,
            c_kpa: 0.0,
            gamma_kn_m3: 18.0,
            b_m: 2.0,
            d_f_m: 1.5,
            e_s_mpa: 30_000.0,
            nu: 0.3,
            design_approach: "da1str".into(),
            annex: AnnexChoice::De,
            settlement_limit_mm: 25.0,
            n_pile_ed_kn: 800.0,
            alpha_s: 0.7,
            pile_d_m: 0.6,
            q_s_kpa: 80.0,
            pile_l_m: 12.0,
            q_b_kpa: 2500.0,
            pile_base_area_m2: 0.28,
            pile_n_profiles: 1,
            z_investigated_m: 8.0,
        }
    }
}
