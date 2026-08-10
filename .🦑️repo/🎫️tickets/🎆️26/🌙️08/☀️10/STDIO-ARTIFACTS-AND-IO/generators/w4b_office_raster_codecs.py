#!/usr/bin/env python3
"""Patch w4b office+raster artifacts with real codecs and IO."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
ROSTER = json.loads((TICKET / "🧪tokens.json").read_text()) if False else json.loads(
    (list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0] / "🧪owner-table.json").read_text()
)
TOKENS = json.loads((list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0] / "🧪tokens.json").read_text())
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
BINARY = ROSTER["binary"]["dir"]

RASTER_SNAPSHOT = '''//! 🧬️ {Name}Snapshot schema — persistent fields + real codecs.

use crate::artifacts::{mid}::STDIO_{MID}_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region RasterModel
/// 🖼️ RGBA raster (`width` × `height` × 4 bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RasterImage {{
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub rgba: Vec<u8>,
}}
//#endregion RasterModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {Name}Snapshot {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub image: RasterImage,
}}

impl Default for {Name}Snapshot {{
    fn default() -> Self {{
        Self {{ schema: STDIO_{MID}_DOCUMENT_SCHEMA.into(), image: RasterImage::default() }}
    }}
}}
//#endregion Snapshot

//#region HandcraftedDocumentCodecs
impl store::DocumentDsl for {Name}Snapshot {{
    const EXTENSION: &'static str = "{ext}";
    fn envelope_id() -> &'static str {{ "stdio.{mid}" }}

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {{
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }}
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {{
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {{
                store::TextError::new(format!("invalid hex: {{e}}"), dsl::TextSpan::at(1, 1))
            }})?;
            bytes.push(byte);
            i += 2;
        }}
        crate::artifacts::{mid}::engine::decode_{mid}(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }}

    fn print_dsl(&self) -> String {{
        let bytes = crate::artifacts::{mid}::engine::encode_{mid}(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{{b:02x}}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl store::DocumentPack for {Name}Snapshot {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let _ = options;
        let raw = crate::artifacts::{mid}::engine::encode_{mid}(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }}

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let _ = options;
        crate::artifacts::{mid}::engine::decode_{mid}(&inner).map_err(|e| store::PackError::Schema(e))
    }}
}}
//#endregion HandcraftedDocumentCodecs
'''

IO_BINARY_IMP = '''//! Deserialize stdio.{mid} from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::{mid}::{{{Name}Snapshot, STDIO_{MID}_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &BinarySnapshot) -> Result<{Name}Snapshot, store::PackError> {{
    let mut snap = crate::artifacts::{mid}::engine::decode_{mid}(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_{MID}_DOCUMENT_SCHEMA.into();
    Ok(snap)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{Name}Snapshot, store::PackError> {{
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}}
'''

IO_BINARY_SER = '''//! Serialize stdio.{mid} to stdio.binary.

use crate::artifacts::binary::{{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{Name}Snapshot;

pub fn register() {{}}

pub fn serialize(from: &{Name}Snapshot) -> Result<BinarySnapshot, store::PackError> {{
    let bytes = crate::artifacts::{mid}::engine::encode_{mid}(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot {{ schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes }})
}}
'''

IO_DEFLATE_IMP = '''//! Deserialize stdio.{mid} from stdio.deflate (raw file bytes in deflate snapshot).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::{mid}::{{{Name}Snapshot, STDIO_{MID}_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &DeflateSnapshot) -> Result<{Name}Snapshot, store::PackError> {{
    let mut snap = crate::artifacts::{mid}::engine::decode_{mid}(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_{MID}_DOCUMENT_SCHEMA.into();
    Ok(snap)
}}
'''

IO_DEFLATE_SER = '''//! Serialize stdio.{mid} to stdio.deflate.

use crate::artifacts::deflate::{{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{Name}Snapshot;

pub fn register() {{}}

pub fn serialize(from: &{Name}Snapshot) -> Result<DeflateSnapshot, store::PackError> {{
    let bytes = crate::artifacts::{mid}::engine::encode_{mid}(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {{ schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), bytes }})
}}
'''

RASTER_ENGINE_HEADER = '''//! ⚙️ {Name}Engine — real {mid} codec.

use crate::artifacts::{mid}::{{schema::snapshot::RasterImage, {Name}Artifact, {Name}Diff, {Name}Mutation, {Name}Snapshot, STDIO_{MID}_DOCUMENT_SCHEMA}};

fn png_crc32(data: &[u8]) -> u32 {{
    crate::artifacts::zip::engine::crc32(data)
}}

fn write_chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {{
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ty);
    out.extend_from_slice(data);
    let mut crc_in = Vec::new();
    crc_in.extend_from_slice(ty);
    crc_in.extend_from_slice(data);
    out.extend_from_slice(&png_crc32(&crc_in).to_be_bytes());
}}

pub fn rgba_len(img: &RasterImage) -> Result<usize, String> {{
    let n = (img.width as usize).checked_mul(img.height as usize).and_then(|p| p.checked_mul(4))
        .ok_or("dimensions overflow")?;
    if img.rgba.len() != n {{ return Err("rgba length mismatch".into()); }}
    Ok(n)
}}
'''

# Format-specific encode/decode appended per mid in CODECS dict below

CODECS: dict[str, str] = {}

CODECS["png"] = RASTER_ENGINE_HEADER + '''
pub fn encode_png(snap: &PngSnapshot) -> Result<Vec<u8>, String> {
    let img = &snap.image;
    rgba_len(img)?;
    let mut idat = Vec::new();
    let row = (img.width as usize) * 4;
    for y in 0..img.height as usize {
        idat.push(0);
        let start = y * row;
        idat.extend_from_slice(&img.rgba[start..start + row]);
    }
    let compressed = crate::artifacts::deflate::engine::zlib_compress(&idat)?;
    let mut out = Vec::new();
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&img.width.to_be_bytes());
    ihdr.extend_from_slice(&img.height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    Ok(out)
}

pub fn decode_png(data: &[u8]) -> Result<PngSnapshot, String> {
    if data.len() < 8 || &data[0..8] != [137, 80, 78, 71, 13, 10, 26, 10] {
        return Err("not a png".into());
    }
    let mut pos = 8usize;
    let mut width = 0u32;
    let mut height = 0u32;
    let mut idat = Vec::new();
    while pos + 12 <= data.len() {
        let len = u32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        let ty = &data[pos+4..pos+8];
        let start = pos + 8;
        let end = start + len;
        if end + 4 > data.len() { break; }
        let chunk = &data[start..end];
        if ty == b"IHDR" && chunk.len() >= 8 {
            width = u32::from_be_bytes([chunk[0],chunk[1],chunk[2],chunk[3]]);
            height = u32::from_be_bytes([chunk[4],chunk[5],chunk[6],chunk[7]]);
        } else if ty == b"IDAT" {
            idat.extend_from_slice(chunk);
        }
        pos = end + 4;
        if ty == b"IEND" { break; }
    }
    let raw = crate::artifacts::deflate::engine::zlib_decompress(&idat)?;
    let row = (width as usize) * 4;
    let mut rgba = Vec::with_capacity(row * height as usize);
    let mut p = 0usize;
    for _ in 0..height {
        if p >= raw.len() { break; }
        p += 1;
        if p + row > raw.len() { return Err("truncated png scanlines".into()); }
        rgba.extend_from_slice(&raw[p..p+row]);
        p += row;
    }
    Ok(PngSnapshot { schema: STDIO_PNG_DOCUMENT_SCHEMA.into(), image: RasterImage { width, height, rgba } })
}

pub fn empty_png_snapshot() -> PngSnapshot { PngSnapshot::default() }

pub fn register() {
    crate::artifacts::png::io::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::png::schema::png_artifact_schema_descriptor());
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.png", extension: Some("png"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::png::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::png::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.png"),
    });
    store::register_document_codec(store::DocumentCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA));
}

pub struct PngEngine { artifact_state: PngArtifact, snapshot_state: PngSnapshot }
impl PngEngine {
    pub fn new(snapshot: PngSnapshot) -> Self {
        Self { artifact_state: PngArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for PngEngine {
    type Artifact = PngArtifact; type Snapshot = PngSnapshot; type Mutation = PngMutation; type Diff = PngDiff;
    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
'''

# Due to size, jpg/gif/tiff/pdf/office/glb engines written to separate files in ticket/codecs/
CODEC_DIR = TICKET / "generators" / "codecs"
for name in ("jpg", "gif", "tiff", "pdf", "office", "glb"):
    p = CODEC_DIR / f"w4b_{name}_engine.rs"
    if p.exists():
        CODECS[name if name != "office" else "docx"] = p.read_text(encoding="utf-8")

RASTER_MIDS = [
    ("png", "Png", "Png", "png"),
    ("jpg", "Jpg", "Jpg", "jpg"),
    ("gif", "Gif", "Gif", "gif"),
    ("tiff", "Tiff", "Tiff", "tiff"),
]

def write_raster(mid: str, name: str, mid_upper: str, ext: str) -> None:
    art = PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"]
    snap = art / "🧬️schema/📸️snapshot/🦀️component.rs"
    snap.write_text(RASTER_SNAPSHOT.format(mid=mid, Name=name, MID=mid_upper, ext=ext), encoding="utf-8")
    eng = art / "⚙️engine/🦀️component.rs"
    body = CODECS.get(mid)
    if not body:
        raise SystemExit(f"missing codec body for {mid}")
    eng.write_text(body.format(mid=mid, Name=name, MID=mid_upper) if "{" in body and "Name" in body else body, encoding="utf-8")
    imp = art / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BINARY}/🦀️component.rs"
    ser = art / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BINARY}/🦀️component.rs"
    imp.write_text(IO_BINARY_IMP.format(mid=mid, Name=name, MID=mid_upper), encoding="utf-8")
    ser.write_text(IO_BINARY_SER.format(mid=mid, Name=name, MID=mid_upper), encoding="utf-8")
    if mid in ("png", "pdf"):
        defl = ROSTER["deflate"]["dir"]
        impd = art / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{defl}/🦀️component.rs"
        serd = art / f"🚪️io/📤️export/{SER}/🗿️artifacts/{defl}/🦀️component.rs"
        impd.write_text(IO_DEFLATE_IMP.format(mid=mid, Name=name, MID=mid_upper), encoding="utf-8")
        serd.write_text(IO_DEFLATE_SER.format(mid=mid, Name=name, MID=mid_upper), encoding="utf-8")

for row in RASTER_MIDS:
    if row[0] in CODECS or row[0] == "png":
        write_raster(*row)

print("codecs: need engine bodies for jpg gif tiff pdf office glb — run after writing codec files")
