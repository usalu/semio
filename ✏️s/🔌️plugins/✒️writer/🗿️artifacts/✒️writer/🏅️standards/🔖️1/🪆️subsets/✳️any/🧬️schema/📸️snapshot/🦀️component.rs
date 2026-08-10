//! 🧬️ Writer snapshot schema — persistent fields only.

use crate::artifacts::writer::{WRITER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted writer document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "writer.writer", layout = "lines")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[state(persistent)]
    pub language_id: String,
    #[state(persistent)]
    #[serde(default = "crate::artifacts::writer::default_uri")]
    pub uri: String,
    #[state(persistent)]
    #[serde(default)]
    #[dsl(lang_from = "language_id")]
    pub text: String,
}
//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for WriterSnapshot {
    const EXTENSION: &'static str = "writer";
    fn envelope_id() -> &'static str {
        "writer.writer"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {
                limits: dsl::Limits::default(),
                mode: dsl::SourceMode::Document,
            },
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

impl store::ArtifactPack for WriterSnapshot {
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

impl Default for WriterSnapshot {
    fn default() -> Self {
        Self {
            schema: WRITER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            language_id: "plaintext".into(),
            uri: crate::artifacts::writer::default_uri(),
            text: String::new(),
        }
    }
}
//#endregion 🔖️Snapshot
