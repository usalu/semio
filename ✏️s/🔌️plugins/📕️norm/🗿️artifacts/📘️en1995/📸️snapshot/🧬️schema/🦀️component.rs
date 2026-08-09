//! 🪵️ EN 1995 snapshot schema — persistent fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1995 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1995", layout = "lines")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Snapshot {
    #[state(persistent)]
    pub annex: crate::document::AnnexChoice,
    #[state(persistent)]
    pub m_ed_knm: f64,
    #[state(persistent)]
    pub n_ed_kn: f64,
    #[state(persistent)]
    pub v_ed_kn: f64,
    #[state(persistent)]
    pub w_mm3: f64,
    #[state(persistent)]
    pub a_mm2: f64,
    #[state(persistent)]
    pub b_mm: f64,
    #[state(persistent)]
    pub h_mm: f64,
    #[state(persistent)]
    pub f_m_k: f64,
    #[state(persistent)]
    pub f_c_0_k: f64,
    #[state(persistent)]
    pub service_class: String,
    #[state(persistent)]
    pub load_duration: String,
    #[state(persistent)]
    pub m_crit_knm: f64,
    #[state(persistent)]
    pub f_ed_kn: f64,
    #[state(persistent)]
    pub a_ef_mm2: f64,
    #[state(persistent)]
    pub f_v_k: f64,
    #[state(persistent)]
    pub fire_duration_min: f64,
    #[state(persistent)]
    pub section_depth_mm: f64,
    #[state(persistent)]
    pub a_vert_m_s2: f64,
    #[state(persistent)]
    pub n_cycles_bridge: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for En1995Snapshot {
    const EXTENSION: &'static str = "en1995";
    fn envelope_id() -> &'static str { "norm.en1995" }
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

impl store::DocumentPack for En1995Snapshot {
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


impl Default for En1995Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 25.0,
            n_ed_kn: 50.0,
            v_ed_kn: 15.0,
            w_mm3: 1_000_000.0,
            a_mm2: 20_000.0,
            b_mm: 200.0,
            h_mm: 300.0,
            f_m_k: 24.0,
            f_c_0_k: 21.0,
            service_class: "sc1".into(),
            load_duration: "medium".into(),
            m_crit_knm: 80.0,
            f_ed_kn: 18.0,
            a_ef_mm2: 12_000.0,
            f_v_k: 4.0,
            fire_duration_min: 30.0,
            section_depth_mm: 300.0,
            a_vert_m_s2: 0.3,
            n_cycles_bridge: 500_000.0,
        }
    }
}
