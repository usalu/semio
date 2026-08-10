//! 🧬️ Layout snapshot schema — persistent document fields only.

use crate::artifacts::layout::{
    CharacterStyle, Frame, GridSettings, ImageLink, Layer, Page, PageColumns, PageMargins, PageOverride, ParagraphStyle,
    ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted layout document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslArtifact)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout")]
#[dsl(extension = "layout", layout = "lines")]
pub struct LayoutSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub name: String,
    #[state(persistent)]
    #[dsl(block)]
    pub grid: GridSettings,
    #[state(persistent)]
    #[serde(rename = "paragraphStyles")]
    #[dsl(table)]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(persistent)]
    #[serde(rename = "characterStyles")]
    #[dsl(table)]
    pub character_styles: Vec<CharacterStyle>,
    #[state(persistent)]
    #[dsl(table)]
    pub stories: Vec<TextStory>,
    #[state(persistent)]
    #[dsl(table)]
    pub links: Vec<ImageLink>,
    #[state(persistent)]
    #[serde(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    #[state(persistent)]
    #[dsl(table)]
    pub spreads: Vec<Spread>,
    #[state(persistent)]
    pub pages: Vec<Page>,
    #[state(persistent)]
    #[serde(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(persistent)]
    #[serde(rename = "dataFieldsJson", default, skip_serializing_if = "Option::is_none")]
    pub data_fields_json: Option<String>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for LayoutSnapshot {
    const EXTENSION: &'static str = "layout";
    fn envelope_id() -> &'static str {
        LAYOUT_DOCUMENT_SCHEMA
    }
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
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LayoutSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
