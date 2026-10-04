# Actual Ordinary Text Preamble Syntax Admission

The admitted exact producer is ordinary split_text_preamble, whose only refusal comes from empty text or parse_preamble_line's explicit InvalidPreamble cases. Its source has no control parameter, DecodingControl construction, UnknownEnvelope or AmbiguousEnvelope branch. These actual ordinary syntax sites receive explicit InvalidValue; controlled split/unwrap APIs are excluded. Full immediate Semio owner source is captured with these producers. Native higher consumer admission remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-preamble-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs

```rust
//! 🔌️ Native Wires Text uses literal shallow records for full intrinsic ownership.
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str=concat!(module_path!(),"::📖️.grammar.semio");
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT:&str=include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
use crate::WiresSnapshot;
impl store::ArtifactDsl for WiresSnapshot{
 const EXTENSION:&'static str="wires";
 fn envelope_id()->&'static str{"reasoning.wires"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Wires Text envelope differs",semio_framework_diagnostic::TextSpan::at(1,1)))}super::binary::parse_pack_record_text(body)}
 fn print_dsl(&self)->String{let body=super::binary::print_pack_record_text(self);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("Wires owner envelope");store::semio_format::wrap_text(&envelope,&body)}
}
pub fn parse_dsl(text:&str)->Result<WiresSnapshot,semio_framework_diagnostic::TextError>{<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)}
pub fn print_dsl(snapshot:&WiresSnapshot)->String{store::ArtifactDsl::print_dsl(snapshot)}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ JpgSnapshot schema — complete JFIF 1.01 semantic model, real codecs. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `RasterImage`-shaped stub (`image: RasterImage{width,height,rgba}` only, no JFIF/SOF/DQT/DHT
//! typing at all) with a typed JFIF APP0 header, typed SOF (frame) + id-keyed DQT/DHT tables,
//! `DRI` restart interval, verbatim-retained other APPn/COM segments, and decoded pixels —
//! `## Snapshot completeness spec`'s jpg row. `RasterImage` itself dies per the ticket's explicit
//! kill directive (W0: "shared verbatim across jpg/png/tiff, png already killed its own copy") —
//! `width`/`height`/`pixels` are first-class fields here, no shared wrapper type.

use crate::STDIO_JPG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::text as owned_text;

//#region Jfif
/// 📏️ JFIF APP0 `units` byte (ITU-T T.871 / JFIF 1.02 §). `Aspect` means `x_density`/
/// `y_density` are merely a pixel aspect ratio, not an absolute resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JfifDensityUnits {
    #[default]
    Aspect,
    PixelsPerInch,
    PixelsPerCm,
}

impl JfifDensityUnits {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JfifDensityUnits::Aspect),
            1 => Ok(JfifDensityUnits::PixelsPerInch),
            2 => Ok(JfifDensityUnits::PixelsPerCm),
            _ => Err(format!("jfif: unsupported density unit {v}")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u8(self) -> u8 {
        match self {
            JfifDensityUnits::Aspect => 0,
            JfifDensityUnits::PixelsPerInch => 1,
            JfifDensityUnits::PixelsPerCm => 2,
        }
    }
}

/// 🖼️ JFIF APP0's optional embedded thumbnail — uncompressed 24-bit RGB, `width * height * 3`
/// bytes, row-major. A weak value (whole-value replaced in diffs, never sub-diffed).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct JfifThumbnail {
    pub width: u8,
    pub height: u8,
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub rgb_data: Vec<u8>,
}
//#endregion Jfif

//#region FrameScanModel
/// 🧩 One SOF0 frame component descriptor: id, H/V sampling factors, and which of the (up to 4)
/// DQT tables it dequantizes against. Id-keyed within `JpgFrameHeader.components`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameComponent {
    pub id: u8,
    pub h_sampling: u8,
    pub v_sampling: u8,
    pub quant_table_id: u8,
}

/// 🖼️ Baseline (SOF0) frame header — sample precision, dimensions, and the per-component
/// sampling/quant-table layout the entropy-coded scan follows.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgFrameHeader {
    pub precision: u8,
    pub width: u16,
    pub height: u16,
    pub components: Vec<JpgFrameComponent>,
}

/// 🎯 One SOS scan component: which DC/AC Huffman table (of up to 4 each) it decodes with.
/// Transient decode/encode state — not persisted on `JpgSnapshot` (the persisted per-component
/// table binding is `JpgFrameComponent.quant_table_id` plus `JpgSnapshot.huffman_tables`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JpgScanComponent {
    pub id: u8,
    pub dc_table_id: u8,
    pub ac_table_id: u8,
}
//#endregion FrameScanModel

//#region QuantHuffmanTables
/// 📊️ One `DQT` table (id-keyed within `JpgSnapshot.quant_tables`). `values` is retained in the
/// EXACT zigzag scan order the DQT segment stores on disk (T.81 Annex B §B.2.4.1) — never
/// reindexed to natural/row-major order, so a decoded table round-trips byte-for-byte.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgQuantTable {
    pub id: u8,
    /// 🔢️ DQT `Pq` nibble: `0` = 8-bit values, `1` = 16-bit values.
    pub precision: u8,
    pub values: [u16; 64],
}

/// 🌳️ `DHT` table class — DC (differential prediction) or AC (run-length coefficients).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JpgHuffmanClass {
    #[default]
    Dc,
    Ac,
}

impl JpgHuffmanClass {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JpgHuffmanClass::Dc),
            1 => Ok(JpgHuffmanClass::Ac),
            _ => Err(format!("jpg: unsupported huffman class {v}")),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_u8(self) -> u8 {
        match self {
            JpgHuffmanClass::Dc => 0,
            JpgHuffmanClass::Ac => 1,
        }
    }
}

/// 🌳️ One `DHT` table, keyed by `(class, id)` within `JpgSnapshot.huffman_tables` (DC id=0 and
/// AC id=0 are DIFFERENT tables — the compound key is load-bearing). `bits`/`values` are the raw
/// canonical-code counts-per-length and symbol-value bytes exactly as the DHT segment stores them.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgHuffmanTable {
    pub id: u8,
    pub class: JpgHuffmanClass,
    pub bits: [u8; 16],
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub values: Vec<u8>,
}
//#endregion QuantHuffmanTables

//#region OtherSegments
/// 🗃️ An APPn (other than a recognized JFIF APP0)/COM segment the codec doesn't specifically
/// model, retained VERBATIM (typed raw-retention — "nothing real on disk silently dropped").
/// Index-keyed (not marker-keyed): duplicate COM/APPn markers are legal, so position is the only
/// safe stable identity within one decode (mirrors png's `PngTextChunk` reasoning).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegment {
    pub marker: u8,
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub data: Vec<u8>,
}
//#endregion OtherSegments

//#region Snapshot
/// 🧬️ Complete `stdio.jpg` jfif-1.01 semantic snapshot. `schema` is an identity field, never
/// diffed. `frame`/`sof_marker`/`arithmetic` are populated by `engine::decode_jpg` at successful
/// decode (`None`/`0`/`false` only for a snapshot that has never round-tripped through a real
/// JPEG byte stream) — retained under those exact names/shapes because
/// `🧱️baseline::analyzer::check_baseline_conformance` depends on them (ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgSnapshot {
    #[state(artifact)]
    pub schema: String,

    // Decoded raster payload — legitimate `Vec<u8>` exception (the format's payload IS pixels):
    // canonical 8-bit-per-channel RGBA, `width * height * 4` bytes, row-major, top-to-bottom.
    // `width`/`height` here are the CANONICAL raster dimensions a caller wants encoded — distinct
    // from `frame.width`/`frame.height` (u16 on-disk SOF values, only present after a real
    // decode/encode); the two agree for any engine-produced snapshot but a freshly hand-authored
    // one (via `SetPixels`) has no `frame` yet and still needs its own dimensions.
    #[state(artifact)]
    #[value(default)]
    pub width: u32,
    #[state(artifact)]
    #[value(default)]
    pub height: u32,
    #[state(artifact)]
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub pixels: Vec<u8>,
    /// 🎚️ Quality parameter `engine::encode_jpg` scales the Annex K quantization tables by
    /// (IJG convention, `1..=100`). `None` = the engine's own default (90).
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub re_encode_quality: Option<u8>,

    // JFIF APP0 (ITU-T T.871 / JFIF 1.02). Always first-class (non-optional): every JFIF file
    // carries exactly one of these; a never-decoded snapshot keeps the spec's own defaults
    // (version 1.01, aspect-ratio units, 1x1 density, no thumbnail) — `engine::encode_jpg`
    // writes them out unconditionally, matching every real JFIF encoder.
    #[state(artifact)]
    #[value(default)]
    pub jfif_version: (u8, u8),
    #[state(artifact)]
    #[value(default)]
    pub jfif_density_units: JfifDensityUnits,
    #[state(artifact)]
    #[value(default)]
    pub jfif_x_density: u16,
    #[state(artifact)]
    #[value(default)]
    pub jfif_y_density: u16,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_thumbnail: Option<JfifThumbnail>,

    // SOF (T.81 §B.2.2) — see the struct doc for why `frame`/`sof_marker`/`arithmetic` keep
    // their pre-existing shapes/names.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<JpgFrameHeader>,
    #[state(artifact)]
    #[value(default)]
    pub sof_marker: u8,
    #[state(artifact)]
    #[value(default)]
    pub arithmetic: bool,

    // DQT (T.81 §B.2.4.1) / DHT (T.81 §B.2.4.2) — id-keyed (DHT compound-keyed by class+id).
    #[state(artifact)]
    #[value(default)]
    pub quant_tables: Vec<JpgQuantTable>,
    #[state(artifact)]
    #[value(default)]
    pub huffman_tables: Vec<JpgHuffmanTable>,

    // DRI (T.81 §B.2.4.4) — `None` = no restart interval segment was present.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub restart_interval: Option<u16>,

    // Verbatim-retained other APPn/COM segments, in encounter order (§`JpgSegment` doc).
    #[state(artifact)]
    #[value(default)]
    pub other_segments: Vec<JpgSegment>,
}

impl Default for JpgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_JPG_DOCUMENT_SCHEMA.into(),
            width: 0,
            height: 0,
            pixels: Vec::new(),
            re_encode_quality: None,
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: None,
            sof_marker: 0,
            arithmetic: false,
            quant_tables: Vec::new(),
            huffman_tables: Vec::new(),
            restart_interval: None,
            other_segments: Vec::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for JpgSnapshot {
    const EXTENSION: &'static str = "jpg";
    fn envelope_id() -> &'static str {
        "stdio.jpg"
    }

    fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
        let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
        if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JPG owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)));}
        owned_text::from_record(dsl::schema::parse_exact(body,&owned_text::spec(),&dsl::ParseOptions::default())?)
    }
    fn print_dsl(&self)->String{
        let body=dsl::schema::print(&owned_text::to_record(self),&owned_text::spec(),dsl::JoinMode::Document);
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared JPG envelope");store::semio_format::wrap_text(&envelope,&body)
    }
}

impl store::ArtifactPack for JpgSnapshot {
    fn record_spec()->Option<dsl::RecordSpec>{Some(owned_text::spec())}
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
        let body=store::pack_rt::encode_document(&owned_text::spec(),&owned_text::to_record(self),options)?;
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
    }
    fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
        let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("JPG owned Pack envelope mismatch".into()));}
        owned_text::from_record(store::pack_rt::decode_document(&body,&owned_text::spec(),options)?.0).map_err(|error|store::PackError::Schema(error.to_string()))
    }
}
//#endregion HandcraftedArtifactCodecs

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ GifSnapshot schema (87a) — complete per GIF87a §18-24: logical screen descriptor + global
//! color table + an ordered sequence of images (GIF87a legally permits more than one Image
//! Descriptor per file even without any extension block — there is simply no per-image timing or
//! disposal metadata, since GCE is an 89a-only feature). Palette indices are stored losslessly
//! (never decoded to RGBA) — `rgba()` is a derived accessor, matching the plan's "lossless-payload
//! exception" for indexed pixel buffers. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use crate::STDIO_GIF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region ColorTable
/// 🎨️ One color table entry (GCT/LCT), stored exactly as read from disk — including any
/// power-of-two padding entries past the meaningful palette, since those are real on-disk bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 🎨️ A Global or Local Color Table. `colors.len()` must be a power of two in `2..=256` on encode
/// (the on-disk "size" field is `log2(len)-1`). `sorted` mirrors the packed byte's sort flag
/// (decreasing importance ordering — rarely used in practice, but real on-disk state).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifColorTable {
    #[value(default)]
    pub sorted: bool,
    #[value(default)]
    pub colors: Vec<GifRgb>,
}
//#endregion ColorTable

//#region ImageModel
/// 🖼️ One Table-Based Image (GIF87a §20-22): its own screen sub-rectangle, optional Local Color
/// Table, interlace flag, and losslessly-retained palette indices (NOT decoded RGBA).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifImage {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[value(default)]
    pub interlace: bool,
    #[dsl(block)]
    #[value(default)]
    pub lct: Option<GifColorTable>,
    /// 🎞️ Palette indices, row-major, natural (non-interlaced) order — length must equal
    /// `width * height`. The lossless-payload exception: this is the format's actual pixel data.
    #[dsl(base64)]
    #[value(default)]
    pub indices: Vec<u8>,
}

impl GifImage {
    /// 🖌️ Derived RGBA accessor — decodes `indices` through `lct` (falling back to `gct` when this
    /// image has no local table). `rgba()` is intentionally NOT a stored field: GIF87a has no
    /// transparency concept at all, so every pixel is fully opaque.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rgba(&self, gct: Option<&GifColorTable>) -> Vec<u8> {
        let table = self.lct.as_ref().or(gct);
        let mut out = Vec::with_capacity(self.indices.len() * 4);
        for &idx in &self.indices {
            let rgb = table.and_then(|t| t.colors.get(idx as usize)).copied().unwrap_or_default();
            out.extend_from_slice(&[rgb.r, rgb.g, rgb.b, 255]);
        }
        out
    }
}
//#endregion ImageModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    #[dsl(block)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[value(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[value(default)]
    pub pixel_aspect_ratio: u8,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    fn envelope_id() -> &'static str {
        "stdio.gif"
    }

    fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
        let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
        if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "GIF owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)));}
        Self::__dsl_from_record(&dsl::schema::parse_exact(body,&Self::__dsl_spec(),&dsl::ParseOptions::default())?)
    }
    fn print_dsl(&self)->String{
        let body=dsl::schema::print(&self.__dsl_to_record(),&Self::__dsl_spec(),dsl::JoinMode::Document);
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared GIF envelope");store::semio_format::wrap_text(&envelope,&body)
    }
}

impl store::ArtifactPack for GifSnapshot {
    fn record_spec()->Option<dsl::RecordSpec>{Some(Self::__dsl_spec())}
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
        let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
    }
    fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
        let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("GIF owned Pack envelope mismatch".into()));}
        Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(|error|store::PackError::Schema(error.to_string()))
    }
}
//#endregion HandcraftedArtifactCodecs

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ GifSnapshot schema (89a) — complete per GIF89a §18-27: logical screen descriptor + optional
//! Global Color Table + an ordered sequence of frames, each with its own Graphic Control Extension
//! fields (delay/disposal/transparent-index/user-input), optional Local Color Table, interlace
//! flag, and losslessly-retained palette indices (never decoded RGBA — the lossless-payload
//! exception). Also carries NETSCAPE2.0 loop count, comment extensions, plain-text extensions, and
//! any OTHER application extension verbatim (`GifAppExtension`) — nothing real on disk is silently
//! dropped. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: a REAL
//! rewrite of the prior decoded-rgba stub, which dropped palettes and every extension but GCE/loop.
//! Distinct from 87a's `GifImage`-shaped `GifSnapshot` (no GCE/animation concept at all) — the two
//! standards genuinely differ in shape, which is why 87a→89a is the plan's "Tier 2"
//! (snapshot-type-changing) evolution pilot rather than a same-type dialect move.

use framework_schema::ArtifactSchema;

//#region Ids
/// 🏷️ Document schema / DSL envelope id — distinct from 87a's `"stdio.gif"` so the two
/// standards' document codecs never collide in the shared `store::document_codec_registry`
/// (still keyed by a flat schema string pre-D4; see `engine::register`).
pub const STDIO_GIF89A_DOCUMENT_SCHEMA: &str = "stdio.gif.89a";
/// 🧬️ Artifact schema descriptor id — distinct from 87a's `"s.stdio.gif"` for the same reason.
pub const GIF89A_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif.89a";
//#endregion Ids

//#region ColorTable
/// 🎨️ One color table entry (GCT/LCT), stored exactly as read from disk — including any
/// power-of-two padding entries past the meaningful palette, since those are real on-disk bytes.
/// 🧪️ F6-PILOT: `dsl::DslRecord` throughout this file — gives every nested snapshot/strong-entity
/// type `DslField` so `#[derive(dsl::DslOps)]` (on `GifMutation`) and `#[derive(dsl::DslDiff)]`
/// (on `GifDiff`, `GifFrameDiff`, ...) can embed them as variant/field payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 🎨️ A Global or Local Color Table. `colors.len()` must be a power of two in `2..=256` on encode.
/// `sorted` mirrors the packed byte's sort flag (decreasing importance ordering).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifColorTable {
    #[value(default)]
    pub sorted: bool,
    #[value(default)]
    pub colors: Vec<GifRgb>,
}
//#endregion ColorTable

//#region DisposalModel
/// 🗑️ GIF89a §23.c.4 disposal method: how the decoder should treat this frame's canvas region
/// before rendering the next one.
/// 🧪️ F6-PILOT: `dsl::DslScalar` — a plain unit-variant enum binds as `DslField` directly (no
/// `DslVariants`/`Statements` needed; this is the "enum but not a mutation-shaped one" case).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum GifDisposal {
    #[default]
    Unspecified,
    DoNotDispose,
    RestoreToBackground,
    RestoreToPrevious,
}

impl GifDisposal {
    /// 📐️ Decodes the GCE packed byte's 3-bit disposal field (values 4-7 are spec-reserved and
    /// fold back to `Unspecified` rather than erroring).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            1 => GifDisposal::DoNotDispose,
            2 => GifDisposal::RestoreToBackground,
            3 => GifDisposal::RestoreToPrevious,
            _ => GifDisposal::Unspecified,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_bits(self) -> u8 {
        match self {
            GifDisposal::Unspecified => 0,
            GifDisposal::DoNotDispose => 1,
            GifDisposal::RestoreToBackground => 2,
            GifDisposal::RestoreToPrevious => 3,
        }
    }
}
//#endregion DisposalModel

//#region PlainText
/// 📝️ Plain Text Extension (GIF89a §25) — a Graphic-Rendering Block alternative to a Table-Based
/// Image. Modeled as an optional companion on [`GifFrame`] per this ticket's design: a frame whose
/// `plain_text` is `Some` and `width == 0` IS a plain-text-only block (no image data); a frame with
/// both real image data and `plain_text` is a rare-but-legal combo the codec does not encode (a
/// documented deviation — see `engine::encode_gif`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifPlainText {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub cell_width: u8,
    pub cell_height: u8,
    pub fg_color_index: u8,
    pub bg_color_index: u8,
    #[value(default)]
    pub text: String,
}
//#endregion PlainText

//#region AppExtension
/// 🧩️ Any application extension OTHER than NETSCAPE2.0 (which is modeled separately via
/// `GifSnapshot::loop_count`, to avoid representing the same on-disk bytes twice), retained
/// verbatim — typed raw-retention for a spec-real-but-semantically-opaque region.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[derive(Default)]
pub struct GifAppExtension {
    pub identifier: [u8; 8],
    pub auth_code: [u8; 3],
    #[value(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
}

//#endregion AppExtension

//#region FrameModel
/// 🎞️ One animation frame: its own region of the logical screen (real GIFs commonly only redraw
/// the changed sub-rectangle per frame, confirmed against the `dancing.gif` fixture), an optional
/// Local Color Table, interlace flag, losslessly-retained palette indices (NOT decoded RGBA — the
/// lossless-payload exception), and the Graphic Control Extension fields that preceded it.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifFrame {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[value(default)]
    pub interlace: bool,
    #[value(default)]
    #[dsl(block)]
    pub lct: Option<GifColorTable>,
    /// 🎞️ Palette indices, row-major, natural (non-interlaced) order — length must equal
    /// `width * height` for a real-image frame (empty for a plain-text-only frame).
    #[value(default)]
    #[dsl(base64)]
    pub indices: Vec<u8>,
    /// ⏱️ GCE delay time in 1/100s units.
    #[value(default)]
    pub delay_cs: u16,
    #[value(default)]
    pub disposal: GifDisposal,
    /// 👁️ GCE transparent color index — `None` when the transparent-color flag is clear.
    #[value(default)]
    pub transparent_index: Option<u8>,
    /// ⌨️ GCE user-input flag.
    #[value(default)]
    pub user_input: bool,
    #[value(default)]
    #[dsl(block)]
    pub plain_text: Option<GifPlainText>,
}

impl GifFrame {
    /// 🖌️ Derived RGBA accessor — decodes `indices` through `lct` (falling back to `gct`).
    /// `transparent_index`-matching pixels normalize to `[0,0,0,0]`. NOT a stored field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rgba(&self, gct: Option<&GifColorTable>) -> Vec<u8> {
        let table = self.lct.as_ref().or(gct);
        let mut out = Vec::with_capacity(self.indices.len() * 4);
        for &idx in &self.indices {
            if Some(idx) == self.transparent_index {
                out.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            let rgb = table.and_then(|t| t.colors.get(idx as usize)).copied().unwrap_or_default();
            out.extend_from_slice(&[rgb.r, rgb.g, rgb.b, 255]);
        }
        out
    }
}
//#endregion FrameModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
pub struct GifSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    #[dsl(block)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[value(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[value(default)]
    pub pixel_aspect_ratio: u8,
    /// 🔁️ NETSCAPE2.0 application extension loop count: `None` = no looping extension present
    /// (plays once); `Some(0)` = loop forever; `Some(n)` = loop `n` additional times.
    #[state(artifact)]
    #[value(default)]
    pub loop_count: Option<u16>,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<GifFrame>,
    /// 💬️ Comment Extension bodies, in file order (positionally normalized to appear right after
    /// the screen descriptor on re-encode — see `engine::encode_gif`'s documented normal form).
    #[state(artifact)]
    #[value(default)]
    pub comments: Vec<String>,
    /// 🧩️ Every application extension OTHER than NETSCAPE2.0, verbatim.
    #[state(artifact)]
    #[value(default)]
    pub app_extensions: Vec<GifAppExtension>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, loop_count: None, frames: Vec::new(), comments: Vec::new(), app_extensions: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    fn envelope_id() -> &'static str {
        STDIO_GIF89A_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
        let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
        if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "GIF owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)));}
        Self::__dsl_from_record(&dsl::schema::parse_exact(body,&Self::__dsl_spec(),&dsl::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:32*1024*1024,..semio_framework_diagnostic::Limits::default()},..dsl::ParseOptions::default()})?)
    }
    fn print_dsl(&self)->String{
        let body=dsl::schema::print(&self.__dsl_to_record(),&Self::__dsl_spec(),dsl::JoinMode::Document);
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared GIF envelope");store::semio_format::wrap_text(&envelope,&body)
    }
}

impl store::ArtifactPack for GifSnapshot {
    fn record_spec()->Option<dsl::RecordSpec>{Some(Self::__dsl_spec())}
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
        let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;
        let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))
    }
    fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
        let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("GIF owned Pack envelope mismatch".into()));}
        Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(|error|store::PackError::Schema(error.to_string()))
    }
}
//#endregion HandcraftedArtifactCodecs

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

```

## ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🎥️ Persisted Shooting fields and literal composed emblem ownership.
use crate::{ShootingAsset,ShootingEmblemChild,ShootingSavedCamera,ShootingSceneLighting,ShootingShot,SHOOTING_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
/// 📸️ Complete Shooting document snapshot, with ordered records and optional child reference.
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,ArtifactSchema,dsl::DslRecord)]
#[value(rename_all="camelCase")]
#[artifact_schema(id="s.shooting.shooting")]
#[dsl(extension="shooting")]
#[dsl(layout="lines")]
pub struct ShootingSnapshot{
 #[state(artifact)]
 pub schema:String,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub assets:Vec<ShootingAsset>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub saved_cameras:Vec<ShootingSavedCamera>,
 #[state(artifact)]
 #[value(default)]
 #[dsl(block)]
 pub scene:ShootingSceneLighting,
 #[state(artifact)]
 #[value(default)]
 #[dsl(table)]
 pub shots:Vec<ShootingShot>,
 #[state(artifact)]
 #[value(default)]
 pub active_shot_id:String,
 #[state(artifact)]
 #[value(default)]
 pub active_asset_id:String,
 #[state(artifact)]
 #[child(kind="s.stdio.semio")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub emblem:Option<ShootingEmblemChild>,
}
impl Default for ShootingSnapshot{
 fn default()->Self{Self{schema:SHOOTING_DOCUMENT_SCHEMA.into(),assets:Vec::new(),saved_cameras:Vec::new(),scene:ShootingSceneLighting::default(),shots:Vec::new(),active_shot_id:String::new(),active_asset_id:String::new(),emblem:None}}
}
impl store::ArtifactDsl for ShootingSnapshot{
 const EXTENSION:&'static str="shooting";
 fn envelope_id()->&'static str{"shooting.shooting"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|semio_framework_diagnostic::TextError::new(e.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
  if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Shooting native text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}
  let record=dsl::parse(body,&Self::__dsl_spec(),&dsl::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:dsl::SourceMode::Document})?;Self::__dsl_from_record(&record)
 }
 fn print_dsl(&self)->String{
  let body=dsl::print(&self.__dsl_to_record(),&Self::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("valid Shooting envelope");store::semio_format::wrap_text(&envelope,&body)
 }
}
impl store::ArtifactPack for ShootingSnapshot{
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let inner=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|store::PackError::Schema(e.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::Schema(e.to_string()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("Shooting native pack envelope mismatch".into()))}
  let(record,_)=store::pack_rt::decode_document(&inner,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
 }
 fn record_spec()->Option<dsl::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
}
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;
#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

```

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::CurationSnapshot;

/// 📄️ The `demo` example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = crate::examples::demo::PRIMARY_TEXT;

/// 📄️ The empty curation the shell's "no example" loads — empty stock and curated table. `catalog`'s
/// handle is content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurationSnapshot::default()` mints).
pub const EMPTY_CURATION_TEXT: &str = r#"semio curation.curation.dsl v1
catalog=child_id=catalog-4f53cda18c2baa0c target=artifact-id=catalog-4f53cda18c2baa0c artifact-kind=s.stdio.semio standard=v1 subset=kit stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curation` DSL text into a `CurationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurationSnapshot, semio_framework_diagnostic::TextError> {
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurationSnapshot` back to `.curation` DSL text.
pub fn print_dsl(document: &CurationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl store::ArtifactDsl for CurationSnapshot {
    const EXTENSION: &'static str = "curation";
    fn envelope_id() -> &'static str { "curation.curation" }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = if text.trim_start().starts_with("semio ") {
            let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|error| semio_framework_diagnostic::TextError::new(error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Curation text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
            body
        } else { text };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        let result = Self::__dsl_from_record(&record)?;
        result.validate().map_err(|message| semio_framework_diagnostic::TextError::new(message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(result)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("Curation envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📥 Parses the document's native text representation.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> { parse_dsl(text).map_err(|error| format!("{error:?}")) }
/// 📤 Emits the document's native text representation.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String { print_dsl(snapshot) }

```


## Actual Mount Receipt

The selected Curation caller had already changed through this lane's prior hexadecimal/shape cut. Its immediate original was refreshed before mutation and the admitted ordinary preamble call was re-located; all six actual mounted source hashes have zero gaps. No higher native consumer proof is claimed.
