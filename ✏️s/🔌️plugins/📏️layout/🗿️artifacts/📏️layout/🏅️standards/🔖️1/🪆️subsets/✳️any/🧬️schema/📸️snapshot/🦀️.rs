//! 🧬️ Layout snapshot schema — artifact-lane fields only.

#[cfg(test)]
use crate::Frame;
use crate::{CharacterStyle, GridSettings, ImageLink, LayoutDrawingChild, Page, ParagraphStyle, ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persisted layout document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `background_drawing` composes stdio's real
/// `s.stdio.semio/v1/drawing` subset as a genuine child slot (see the artifact root's
/// `🔖️ComposedTypes` region doc for the full before/after); `referenced_model` is a forward
/// `ArtifactLink` reference slot, both new. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack are the
/// derived spec-driven encodings of the one `dsl::DslRecord` spec, composed child and link slot included.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "layout")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    #[value(rename = "paragraphStyles")]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    #[value(rename = "characterStyles")]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    #[value(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    pub pages: Vec<Page>,
    #[state(artifact)]
    #[value(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[value(rename = "dataFieldsJson", default, skip_serializing_if = "Option::is_none")]
    pub data_fields_json: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(rename = "backgroundDrawing", default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingChild>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[value(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

/// 🧷️ Real "empty" constructor used as the parse/decode starting point (mirrors cad's
/// `empty_cad_snapshot`) — `default_document()` at `crate::schema` seeds a full
/// demo document instead, so this can't reuse a `Default` impl (this type has none).
pub(crate) fn empty_layout_snapshot() -> LayoutSnapshot {
    LayoutSnapshot {
        schema: String::new(),
        name: String::new(),
        grid: GridSettings { baseline_grid: 0.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` over the one derived record spec.
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
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LayoutSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;
//#endregion 🧪️Tests
