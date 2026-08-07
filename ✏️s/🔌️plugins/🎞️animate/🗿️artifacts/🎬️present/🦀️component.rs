//! 🎞️ Animate present artifact — document entities + `ArtifactKindSpec` (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Domain
/// 📐️ Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
/// {@link FigureTileSource}/{@link FigureTileDraft}), so it declares no `#[dsl(keyword)]` of its own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    #[dsl(block)]
    pub frame: FigureTileFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub crop: FigureTileFrame,
}

/// 📜️ `.present` textual document: `schema=... \n source { ... } \n tiles [ ... ]` (see
/// {@link store::DocumentDsl}).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[dsl(extension = "present", layout = "lines")]
#[serde(rename_all = "camelCase")]
pub struct PresentDeck {
    pub schema: String,
    #[dsl(block)]
    pub source: FigureTileSource,
    #[dsl(table)]
    pub tiles: Vec<FigureTileDraft>,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for PresentDeck {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
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
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for PresentDeck {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec


pub const PRESENT_DECK_SCHEMA: &str = "animate.present.deck";

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: "/🖼️bauteilbörse.png".into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(1222.0 / 896.0), pdf_page: None }
}

pub fn default_present_deck() -> PresentDeck {
    PresentDeck { schema: PRESENT_DECK_SCHEMA.into(), source: default_figure_tile_source(), tiles: Vec::new() }
}
//#endregion 🔖️Domain

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::present::create_animate_present_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: PRESENT_DECK_SCHEMA.into(),
        name: "Animate Present Deck".into(),
        source_format: PRESENT_DECK_SCHEMA.into(),
        component_kind: "panel".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
        schema: PRESENT_DECK_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
/// 🪪️ Orphan-rule anchor: `Identified`/`Patchable` (from `protocol`) can only be implemented for
/// `FigureTileDraft` in the crate that defines it — this is that crate.
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<FigureTileDraftPatch> {
        Some(FigureTileDraftPatch { name: (self.name != other.name).then(|| other.name.clone()), crop: (self.crop != other.crop).then(|| other.crop.clone()) })
    }
}
//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_deck_schema_is_animate_present() {
        assert_eq!(default_present_deck().schema, PRESENT_DECK_SCHEMA);
    }

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` matches the store envelope schema for this
    /// artifact (unlike flow, animate never split these two).
    #[test]
    fn artifact_kind_matches_the_store_schema() {
        assert_eq!(artifact_kind().schema, PRESENT_DECK_SCHEMA);
        assert_eq!(artifact_kind().id, PRESENT_DECK_SCHEMA);
    }
}
//#endregion 🧪️Tests
