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
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct BitmapColorDsl {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
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

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "wfc.bitmap", layout = "lines")]
pub(crate) struct BitmapSnapshotDsl {
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}



pub(crate) fn bitmap_document_to_dsl(document: &BitmapSnapshot) -> BitmapSnapshotDsl {
    BitmapSnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        input_width: document.input.width,
        input_height: document.input.height,
        input_pixels: encode_base64(&document.input.pixels),
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

pub(crate) fn bitmap_document_from_dsl(parsed: BitmapSnapshotDsl) -> Result<BitmapSnapshot, semio_framework_value::ValueError> {
    Ok(BitmapSnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        input: BitmapInput { width: parsed.input_width, height: parsed.input_height, palette: parsed.palette.iter().map(color_from_dsl).collect(), pixels: decode_pixel_text(&parsed.input_pixels)? },
        output: BitmapOutputSpec { width: parsed.output_width, height: parsed.output_height, periodic: parsed.output_periodic },
        model: BitmapOverlappingModel { pattern_size: parsed.pattern_size, symmetry: parsed.symmetry, periodic_input: parsed.periodic_input, ground: parsed.ground },
        pinned: parsed.pinned.iter().map(pin_from_dsl).collect(),
    })
}

/// 📍️ Positions a native refusal at the document origin while keeping its refusal kind.


/// 🛬️ Moves the authored bitmap grammar into persisted fields under one native ownership control.
pub(crate) fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<BitmapSnapshot,semio_framework_value::ValueError>{
    let maximum_rows=control.limits().max_rows;
    let snapshot=store::decode_sqlite_snapshot_record_native(payload,<BitmapSnapshot as store::ArtifactDsl>::envelope_id(),BitmapSnapshotDsl::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {construct_native_record(record,native,maximum_rows)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)?;snapshot.admit_sqlite_values(control,store::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative)?;Ok(snapshot)
}

/// 🛫️ Admits every owned bitmap native field before its canonical record and physical output.
pub(crate) fn encode_sqlite_snapshot_native(document:&BitmapSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{
    let rows=4usize.checked_add(document.input.palette.len()).and_then(|n|n.checked_add(document.pinned.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"bitmap native entity count overflow"))?;
    control.check_rows(rows)?;document.admit_sqlite_values(control,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative)?;
    store::encode_sqlite_snapshot_record_native(encoding,<BitmapSnapshot as store::ArtifactDsl>::envelope_id(),BitmapSnapshotDsl::__dsl_spec_producer(),|native|project_native_record(document,native),control,native_owner)
}

impl store::ArtifactDsl for BitmapSnapshot {
    const EXTENSION: &'static str = "wfcbitmap";
    fn envelope_id() -> &'static str {
        "wfc.bitmap"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        bitmap_document_from_dsl(<BitmapSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?).map_err(|e|semio_framework_diagnostic::TextError::new(e.kind,e.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    fn print_dsl(&self) -> String {
        <BitmapSnapshotDsl as store::ArtifactDsl>::print_dsl(&bitmap_document_to_dsl(self))
    }
}


//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.wfcbitmap` DSL text into a `BitmapSnapshot`.
pub fn parse_dsl(text: &str) -> Result<BitmapSnapshot, semio_framework_diagnostic::TextError> {
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

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};








}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::add_palette_color::add_palette_color;
use crate::standards::v1::subsets::any::schema::mutations::change_model::change_model;
use crate::standards::v1::subsets::any::schema::mutations::change_palette_color::change_palette_color;
use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::standards::v1::subsets::any::schema::mutations::paint_input_stroke::{paint_input_stroke, stroke_cells, stroke_extent, BitmapStrokePoint, BITMAP_STROKE_MAXIMUM_POINTS};
use crate::standards::v1::subsets::any::schema::mutations::pin_pixel::pin_pixel;
use crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::remove_palette_color;
use crate::standards::v1::subsets::any::schema::mutations::resize_input::resize_input;
use crate::standards::v1::subsets::any::schema::mutations::resize_output::resize_output;
use crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::set_input_pixels;
use crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::unpin_pixel;

/// 🔁️ Decodes one snapshot through this subset's production JSON codec and re-encodes it — the subject half of the
/// case's `identity-round-trip` scenario.
pub fn bitmap_snapshot_json_round_trip(text: &str) -> Result<String, String> {
    let snapshot: BitmapSnapshot = crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error| error.to_string())?;
    Ok(crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&snapshot))
}
}
pub use mutations_codec::*;

const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// 🔤️ RFC 4648 standard base64, padded — authored here rather than taken from
/// `semio-framework-io-base64` because that crate carries no `[workspace.dependencies]` alias and
/// this artifact must not widen the workspace manifest for forty lines of table lookup.
pub fn encode_base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(BASE64_ALPHABET[(triple >> 18) as usize & 63] as char);
        out.push(BASE64_ALPHABET[(triple >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { BASE64_ALPHABET[(triple >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { BASE64_ALPHABET[triple as usize & 63] as char } else { '=' });
    }
    out
}
pub(crate) fn base64_value(byte: u8) -> Option<u32> {
    match byte {
        b'A'..=b'Z' => Some(u32::from(byte - b'A')),
        b'a'..=b'z' => Some(u32::from(byte - b'a') + 26),
        b'0'..=b'9' => Some(u32::from(byte - b'0') + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}
/// 🔤️ The exact inverse of [`encode_base64`]; a malformed buffer answers `None` rather than a
/// silently truncated one, because a pixel buffer that does not decode is a fatal mutation outcome,
/// never a best-effort render.
pub fn decode_base64(text: &str) -> Option<Vec<u8>> {
    let bytes = text.as_bytes();
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for chunk in bytes.chunks(4) {
        let pad = chunk.iter().filter(|byte| **byte == b'=').count();
        if pad > 2 || (pad > 0 && chunk[3] != b'=') || (pad == 2 && chunk[2] != b'=') {
            return None;
        }
        let mut triple = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            let value = if *byte == b'=' { 0 } else { base64_value(*byte)? };
            if *byte == b'=' && index < 4 - pad {
                return None;
            }
            triple |= value << (18 - 6 * index);
        }
        out.push((triple >> 16) as u8);
        if pad < 2 {
            out.push((triple >> 8) as u8);
        }
        if pad < 1 {
            out.push(triple as u8);
        }
    }
    if encode_base64(&out) != text { return None; }
    Some(out)
}

/// 🖼️ Decodes the physical base64 spelling into intrinsic palette octets.
pub fn decode_pixel_text(text:&str)->Result<Vec<u8>,semio_framework_value::ValueError>{decode_base64(text).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Bitmap pixels require canonical base64"))}

/// 🫴️ Binds the literal bitmap fields under original native admission.
pub(crate)fn construct_native_record(record:&semio_framework_dsl_record::RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize)->Result<BitmapSnapshot,semio_framework_value::ValueError>{
        let parsed=BitmapSnapshotDsl::__dsl_from_record_controlled(record,native)?;
        let entities=parsed.palette.len().checked_add(parsed.pinned.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("bitmap native entity count overflow").to_string()))?;
        if entities.checked_add(4).is_none_or(|rows|rows>maximum_rows){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("bitmap native entity count exceeds caller limit").to_string()));}
        native.scoped_stage(|native|{
            native.begin_stage(entities)?;
            let mut palette=native.allocate_vec::<BitmapColor>(parsed.palette.len())?;
            let mut pinned=native.allocate_vec::<BitmapPinnedPixel>(parsed.pinned.len())?;
            for color in parsed.palette{palette.push(BitmapColor{r:color.r,g:color.g,b:color.b,a:color.a});native.step()?;}
            for pin in parsed.pinned{pinned.push(BitmapPinnedPixel{x:pin.x,y:pin.y,color:pin.color});native.step()?;}
            native.checkpoint()?;
            native.charge(parsed.input_pixels.len()/4*3)?;
            let pixels=decode_pixel_text(&parsed.input_pixels)?;
            Ok(BitmapSnapshot{schema:parsed.schema,seed:parsed.seed,input:BitmapInput{width:parsed.input_width,height:parsed.input_height,palette,pixels},output:BitmapOutputSpec{width:parsed.output_width,height:parsed.output_height,periodic:parsed.output_periodic},model:BitmapOverlappingModel{pattern_size:parsed.pattern_size,symmetry:parsed.symmetry,periodic_input:parsed.periodic_input,ground:parsed.ground},pinned})
        })
    }

/// 🫴️ Binds the literal bitmap fields under original native admission.
pub(crate)fn project_native_record(document:&BitmapSnapshot,native:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
        let schema=native.copy_text(&document.schema)?;
        native.charge(document.input.pixels.len().div_ceil(3)*4)?;
        let input_pixels=encode_base64(&document.input.pixels);
        let palette=native.scoped_stage(|native|->Result<Vec<BitmapColorDsl>,semio_framework_value::ValueError>{
            native.begin_stage(document.input.palette.len())?;
            let mut values=native.allocate_vec::<BitmapColorDsl>(document.input.palette.len())?;
            for color in &document.input.palette{values.push(BitmapColorDsl{r:color.r,g:color.g,b:color.b,a:color.a});native.step()?;}
            native.checkpoint()?;Ok(values)
        })?;
        let pinned=native.scoped_stage(|native|->Result<Vec<BitmapPinnedPixelDsl>,semio_framework_value::ValueError>{
            native.begin_stage(document.pinned.len())?;
            let mut values=native.allocate_vec::<BitmapPinnedPixelDsl>(document.pinned.len())?;
            for pin in &document.pinned{values.push(BitmapPinnedPixelDsl{x:pin.x,y:pin.y,color:pin.color});native.step()?;}
            native.checkpoint()?;Ok(values)
        })?;
        BitmapSnapshotDsl{schema,seed:document.seed,input_width:document.input.width,input_height:document.input.height,input_pixels,output_width:document.output.width,output_height:document.output.height,output_periodic:document.output.periodic,pattern_size:document.model.pattern_size,symmetry:document.model.symmetry,periodic_input:document.model.periodic_input,ground:document.model.ground,palette,pinned}.__dsl_to_record_controlled(native)
    }
