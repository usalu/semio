//! 🧬️ Present snapshot schema — persistent fields only.

use crate::artifacts::present::{
    FigureTileDraft, FigureTileSource, PRESENT_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted present document snapshot (shared source + tile crops).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub source: FigureTileSource,
    #[state(persistent)]
    #[serde(default)]
    pub tiles: Vec<FigureTileDraft>,
}

impl Default for PresentSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> PresentSnapshot {
    PresentSnapshot {
        schema: PRESENT_DOCUMENT_SCHEMA.into(),
        source: crate::artifacts::present::default_figure_tile_source(),
        tiles: Vec::new(),
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "present", layout = "lines")]
pub(crate) struct PresentSnapshotDsl {
    schema: String,
    #[dsl(block)]
    source: FigureTileSource,
    #[dsl(table)]
    tiles: Vec<FigureTileDraft>,
}

pub(crate) fn present_snapshot_to_dsl(snapshot: &PresentSnapshot) -> PresentSnapshotDsl {
    PresentSnapshotDsl {
        schema: snapshot.schema.clone(),
        source: snapshot.source.clone(),
        tiles: snapshot.tiles.clone(),
    }
}

pub(crate) fn present_snapshot_from_dsl(dsl_snapshot: PresentSnapshotDsl) -> PresentSnapshot {
    PresentSnapshot {
        schema: dsl_snapshot.schema,
        source: dsl_snapshot.source,
        tiles: dsl_snapshot.tiles,
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PresentSnapshotDsl {
    const EXTENSION: &'static str = "present";
    fn envelope_id() -> &'static str {
        PRESENT_DOCUMENT_SCHEMA
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

impl store::ArtifactPack for PresentSnapshotDsl {
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

impl store::ArtifactDsl for PresentSnapshot {
    const EXTENSION: &'static str = "present";
    fn envelope_id() -> &'static str {
        PRESENT_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <PresentSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        Ok(present_snapshot_from_dsl(parsed))
    }
    fn print_dsl(&self) -> String {
        <PresentSnapshotDsl as store::ArtifactDsl>::print_dsl(&present_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for PresentSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <PresentSnapshotDsl as store::ArtifactPack>::encode_pack_with(&present_snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <PresentSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        Ok(present_snapshot_from_dsl(parsed))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
