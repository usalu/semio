//! ✨️ EN 1999 app — document entities (constitutional: general).

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

// #region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1999", layout = "lines")]
pub struct Document {
    #[dsl(unit = "kN")]
    pub n_ed_kn: f64,
    pub m_ed_knm: f64,
    #[dsl(unit = "mm2")]
    pub a_mm2: f64,
    pub w_el_mm3: f64,
    pub alloy: String,
    pub chi: f64,
    #[dsl(unit = "mm4")]
    pub i_t_mm4: f64,
    #[dsl(unit = "mm")]
    pub l_cr_mm: f64,
    pub theta_c: f64,
    pub delta_sigma_ed: f64,
    pub delta_sigma_c: f64,
    pub fatigue_m: f64,
    pub n_cycles: f64,
    #[dsl(unit = "kN")]
    pub v_weld_ed_kn: f64,
    #[dsl(unit = "mm")]
    pub weld_throat_mm: f64,
    #[dsl(unit = "mm")]
    pub weld_length_mm: f64,
    pub beta_w: f64,
    #[dsl(unit = "mm")]
    pub sheet_b_mm: f64,
    #[dsl(unit = "mm")]
    pub sheet_t_mm: f64,
    pub sheet_k_sigma: f64,
    pub sheet_w_el_mm3: f64,
    pub sheet_m_ed_knm: f64,
    #[dsl(unit = "mm")]
    pub shell_t_mm: f64,
    #[dsl(unit = "mm")]
    pub shell_r_mm: f64,
    #[dsl(unit = "MPa")]
    pub sigma_ed_shell_mpa: f64,
    pub annex: AnnexChoice,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Document {
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
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for Document {
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




impl Default for Document {
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
// #endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1999", "EN 1999")
}
//#endregion 🔖️ArtifactKind
