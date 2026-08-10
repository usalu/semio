//! ✨️ EN 1999 snapshot schema — persistent fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1999 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1999", layout = "lines")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Snapshot {
    #[state(persistent)]
    pub n_ed_kn: f64,
    #[state(persistent)]
    pub m_ed_knm: f64,
    #[state(persistent)]
    pub a_mm2: f64,
    #[state(persistent)]
    pub w_el_mm3: f64,
    #[state(persistent)]
    pub alloy: String,
    #[state(persistent)]
    pub chi: f64,
    #[state(persistent)]
    pub i_t_mm4: f64,
    #[state(persistent)]
    pub l_cr_mm: f64,
    #[state(persistent)]
    pub theta_c: f64,
    #[state(persistent)]
    pub delta_sigma_ed: f64,
    #[state(persistent)]
    pub delta_sigma_c: f64,
    #[state(persistent)]
    pub fatigue_m: f64,
    #[state(persistent)]
    pub n_cycles: f64,
    #[state(persistent)]
    pub v_weld_ed_kn: f64,
    #[state(persistent)]
    pub weld_throat_mm: f64,
    #[state(persistent)]
    pub weld_length_mm: f64,
    #[state(persistent)]
    pub beta_w: f64,
    #[state(persistent)]
    pub sheet_b_mm: f64,
    #[state(persistent)]
    pub sheet_t_mm: f64,
    #[state(persistent)]
    pub sheet_k_sigma: f64,
    #[state(persistent)]
    pub sheet_w_el_mm3: f64,
    #[state(persistent)]
    pub sheet_m_ed_knm: f64,
    #[state(persistent)]
    pub shell_t_mm: f64,
    #[state(persistent)]
    pub shell_r_mm: f64,
    #[state(persistent)]
    pub sigma_ed_shell_mpa: f64,
    #[state(persistent)]
    pub annex: crate::document::AnnexChoice,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for En1999Snapshot {
    const EXTENSION: &'static str = "en1999";
    fn envelope_id() -> &'static str { "norm.en1999" }
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

impl store::ArtifactPack for En1999Snapshot {
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


impl Default for En1999Snapshot {
    fn default() -> Self {
        Self {
            n_ed_kn: 80.0,
            m_ed_knm: 4.0,
            a_mm2: 1200.0,
            w_el_mm3: 24_000.0,
            alloy: "aw6060t6".into(),
            chi: 0.85,
            i_t_mm4: 5000.0,
            l_cr_mm: 3000.0,
            theta_c: 200.0,
            delta_sigma_ed: 45.0,
            delta_sigma_c: 71.0,
            fatigue_m: 8.0,
            n_cycles: 500_000.0,
            v_weld_ed_kn: 25.0,
            weld_throat_mm: 4.0,
            weld_length_mm: 120.0,
            beta_w: 0.63,
            sheet_b_mm: 200.0,
            sheet_t_mm: 2.0,
            sheet_k_sigma: 4.0,
            sheet_w_el_mm3: 8000.0,
            sheet_m_ed_knm: 0.5,
            shell_t_mm: 4.0,
            shell_r_mm: 500.0,
            sigma_ed_shell_mpa: 150.0,
            annex: AnnexChoice::De,
        }
    }
}
