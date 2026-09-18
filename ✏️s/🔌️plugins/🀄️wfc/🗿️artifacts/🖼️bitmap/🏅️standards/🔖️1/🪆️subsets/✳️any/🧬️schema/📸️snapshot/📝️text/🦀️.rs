//! 📜️ Bitmap artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records rather than
//! `#[dsl]` attributes on those records directly, for the reason the sibling assembly artifact
//! states in its own text facet: the schema file says WHAT the document is and must stay
//! representation-free, while this file says how it is SPELLED. The twins carry the same primitive
//! fields, so the bridge is mechanical in both directions and there is no second authority.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::{BitmapColor, BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, BitmapPinnedPixel, BitmapSnapshot};

//#region 🔖️Examples
/// 📄️ The two authored bitmap problems this subset ships, in their own `.wfcbitmap` DSL.
pub const BITMAP_EXAMPLE_ROOMS_TEXT: &str = include_str!("../../../📚️examples/🚪️rooms-16/🖼️assets/🚪️rooms-16/🗣️.dsl.semio");
pub const BITMAP_EXAMPLE_FLOWERS_TEXT: &str = include_str!("../../../📚️examples/🌸️flowers-24/🖼️assets/🌸️flowers-24/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct BitmapColorDsl {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct BitmapPinnedPixelDsl {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

pub fn color_to_dsl(color: &BitmapColor) -> BitmapColorDsl {
    BitmapColorDsl { r: color.r, g: color.g, b: color.b, a: color.a }
}

pub fn color_from_dsl(color: &BitmapColorDsl) -> BitmapColor {
    BitmapColor { r: color.r, g: color.g, b: color.b, a: color.a }
}

pub fn pin_to_dsl(pin: &BitmapPinnedPixel) -> BitmapPinnedPixelDsl {
    BitmapPinnedPixelDsl { x: pin.x, y: pin.y, color: pin.color }
}

pub fn pin_from_dsl(pin: &BitmapPinnedPixelDsl) -> BitmapPinnedPixel {
    BitmapPinnedPixel { x: pin.x, y: pin.y, color: pin.color }
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "wfc.bitmap", layout = "lines")]
struct BitmapSnapshotDsl {
    schema: String,
    seed: u64,
    input_width: u32,
    input_height: u32,
    input_pixels: String,
    output_width: u32,
    output_height: u32,
    output_periodic: bool,
    pattern_size: u32,
    symmetry: u32,
    periodic_input: bool,
    ground: Option<u32>,
    #[dsl(table)]
    palette: Vec<BitmapColorDsl>,
    #[dsl(table)]
    pinned: Vec<BitmapPinnedPixelDsl>,
}

impl Default for BitmapSnapshotDsl {
    fn default() -> Self {
        let base = BitmapSnapshot::default();
        bitmap_document_to_dsl(&base)
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive stopped emitting these traits, so every
/// artifact states its own envelope discipline.
impl store::ArtifactDsl for BitmapSnapshotDsl {
    const EXTENSION: &'static str = "wfcbitmap";
    fn envelope_id() -> &'static str {
        "wfc.bitmap"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BitmapSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
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

fn bitmap_document_to_dsl(document: &BitmapSnapshot) -> BitmapSnapshotDsl {
    BitmapSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        input_width: document.input.width,
        input_height: document.input.height,
        input_pixels: document.input.pixels.clone(),
        output_width: document.output.width,
        output_height: document.output.height,
        output_periodic: document.output.periodic,
        pattern_size: document.model.pattern_size,
        symmetry: document.model.symmetry,
        periodic_input: document.model.periodic_input,
        ground: document.model.ground,
        palette: document.input.palette.iter().map(color_to_dsl).collect(),
        pinned: document.pinned.iter().map(pin_to_dsl).collect(),
    }
}

fn bitmap_document_from_dsl(parsed: BitmapSnapshotDsl) -> BitmapSnapshot {
    BitmapSnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        input: BitmapInput { width: parsed.input_width, height: parsed.input_height, palette: parsed.palette.iter().map(color_from_dsl).collect(), pixels: parsed.input_pixels },
        output: BitmapOutputSpec { width: parsed.output_width, height: parsed.output_height, periodic: parsed.output_periodic },
        model: BitmapOverlappingModel { pattern_size: parsed.pattern_size, symmetry: parsed.symmetry, periodic_input: parsed.periodic_input, ground: parsed.ground },
        pinned: parsed.pinned.iter().map(pin_from_dsl).collect(),
    }
}

impl store::ArtifactDsl for BitmapSnapshot {
    const EXTENSION: &'static str = "wfcbitmap";
    fn envelope_id() -> &'static str {
        "wfc.bitmap"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        Ok(bitmap_document_from_dsl(<BitmapSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn print_dsl(&self) -> String {
        <BitmapSnapshotDsl as store::ArtifactDsl>::print_dsl(&bitmap_document_to_dsl(self))
    }
}

impl store::ArtifactPack for BitmapSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <BitmapSnapshotDsl as store::ArtifactPack>::encode_pack_with(&bitmap_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Ok(bitmap_document_from_dsl(<BitmapSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfcbitmap` DSL text into a `BitmapSnapshot`.
pub fn parse_dsl(text: &str) -> Result<BitmapSnapshot, store::TextError> {
    <BitmapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `BitmapSnapshot` back to `.wfcbitmap` DSL text.
pub fn print_dsl(document: &BitmapSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type BitmapSnapshotText = String;
//#endregion 🚚️Carrier
