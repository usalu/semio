#!/usr/bin/env python3
"""Apply w4b codecs, IO, glue, plugin, index."""
from __future__ import annotations

import json
import re
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
CODEC = TICKET / "generators" / "codecs"
GLUE = PLUGIN / "📦️packages/🦀️rust/📦️glue.rs"
PLUGIN_RS = PLUGIN / "🔌️plugin/🦀️component.rs"
INDEX_TS = PLUGIN / "📦️packages/🟦️typescript/📦️index.ts"
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
BINARY = ROSTER["binary"]["dir"]
DEFLATE = ROSTER["deflate"]["dir"]


def read(p: Path) -> str:
    return p.read_text(encoding="utf-8")

RASTER_SNAPSHOT = read(TICKET / "generators" / "w4b_office_raster_codecs.py").split("RASTER_SNAPSHOT = '''")[1].split("'''")[0]
IO_BINARY_IMP = read(TICKET / "generators" / "w4b_office_raster_codecs.py").split("IO_BINARY_IMP = '''")[1].split("'''")[0]
IO_BINARY_SER = read(TICKET / "generators" / "w4b_office_raster_codecs.py").split("IO_BINARY_SER = '''")[1].split("'''")[0]
IO_DEFLATE_IMP = read(TICKET / "generators" / "w4b_office_raster_codecs.py").split("IO_DEFLATE_IMP = '''")[1].split("'''")[0]
IO_DEFLATE_SER = read(TICKET / "generators" / "w4b_office_raster_codecs.py").split("IO_DEFLATE_SER = '''")[1].split("'''")[0]

OFFICE_SNAPSHOT = '''//! 🧬️ {Name}Snapshot — OOXML zip parts.

use crate::artifacts::{mid}::STDIO_{MID}_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {Name}Entry {{
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
}}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {Name}Snapshot {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<{Name}Entry>,
}}

impl Default for {Name}Snapshot {{
    fn default() -> Self {{
        Self {{ schema: STDIO_{MID}_DOCUMENT_SCHEMA.into(), entries: Vec::new() }}
    }}
}}

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
        for i in (0..hex.len()).step_by(2) {{
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {{
                store::TextError::new(format!("invalid hex: {{e}}"), dsl::TextSpan::at(1, 1))
            }})?);
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
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }}
        let _ = options;
        crate::artifacts::{mid}::engine::decode_{mid}(&inner).map_err(|e| store::PackError::Schema(e))
    }}
}}
'''

PDF_SNAPSHOT = '''//! 🧬️ PdfSnapshot schema.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDoc {
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub page: PageDoc,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: String::new() } }
    }
}

impl store::DocumentDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str { "stdio.pdf" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) { Ok((_, r)) => r, Err(_) => text };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i+2], 16).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1,1)))?);
        }
        crate::artifacts::pdf::engine::decode_pdf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1,1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::engine::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.pdf", store::semio_format::Component::Dsl, 1).unwrap();
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::engine::encode_pdf(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.pdf", store::semio_format::Component::Pack, 1)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != "stdio.pdf" { return Err(store::PackError::Schema("envelope mismatch".into())); }
        let _ = options;
        crate::artifacts::pdf::engine::decode_pdf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
'''

GLB_SNAPSHOT = '''//! 🧬️ GlbSnapshot schema.

use crate::artifacts::glb::STDIO_GLB_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlbPayload {
    #[serde(default)]
    pub gltf_json: String,
    #[serde(default)]
    pub bin: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb")]
pub struct GlbSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub payload: GlbPayload,
}

impl Default for GlbSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: GlbPayload { gltf_json: r#"{"asset":{"version":"2.0"}}"#.into(), bin: Vec::new() } }
    }
}

impl store::DocumentDsl for GlbSnapshot {
    const EXTENSION: &'static str = "glb";
    fn envelope_id() -> &'static str { "stdio.glb" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) { Ok((_, r)) => r, Err(_) => text };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i+2], 16).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1,1)))?);
        }
        crate::artifacts::glb::engine::decode_glb(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1,1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::glb::engine::encode_glb(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.glb", store::semio_format::Component::Dsl, 1).unwrap();
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for GlbSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::glb::engine::encode_glb(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id("stdio.glb", store::semio_format::Component::Pack, 1)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != "stdio.glb" { return Err(store::PackError::Schema("envelope mismatch".into())); }
        let _ = options;
        crate::artifacts::glb::engine::decode_glb(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
'''

PNG_ENGINE = read(CODEC / "w4b_png_engine.rs") if (CODEC / "w4b_png_engine.rs").exists() else ""

def art(mid: str) -> Path:
    return PLUGIN / "🗿️artifacts" / ROSTER[mid]["dir"]

def patch_raster(mid: str, name: str, ext: str) -> None:
    mid_u = mid.upper()
    (art(mid) / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(
        RASTER_SNAPSHOT.format(mid=mid, Name=name, MID=mid_u, ext=ext), encoding="utf-8"
    )
    eng_src = CODEC / f"w4b_{mid}_engine.rs"
    if not eng_src.exists():
        raise SystemExit(f"missing engine {eng_src}")
    shutil.copy(eng_src, art(mid) / "⚙️engine/🦀️component.rs")
    imp = art(mid) / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BINARY}/🦀️component.rs"
    ser = art(mid) / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BINARY}/🦀️component.rs"
    imp.write_text(IO_BINARY_IMP.format(mid=mid, Name=name, MID=mid_u), encoding="utf-8")
    ser.write_text(IO_BINARY_SER.format(mid=mid, Name=name, MID=mid_u), encoding="utf-8")
    if mid in ("png", "pdf"):
        (art(mid) / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{DEFLATE}/🦀️component.rs").write_text(
            IO_DEFLATE_IMP.format(mid=mid, Name=name, MID=mid_u), encoding="utf-8"
        )
        (art(mid) / f"🚪️io/📤️export/{SER}/🗿️artifacts/{DEFLATE}/🦀️component.rs").write_text(
            IO_DEFLATE_SER.format(mid=mid, Name=name, MID=mid_u), encoding="utf-8"
        )

office_tpl = read(CODEC / "w4b_office_engine.tpl.rs")

for mid, name, ext in [
    ("png", "Png", "png"), ("jpg", "Jpg", "jpg"), ("gif", "Gif", "gif"), ("tiff", "Tiff", "tiff"),
]:
    patch_raster(mid, name, ext)

(art("pdf") / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(PDF_SNAPSHOT, encoding="utf-8")
shutil.copy(CODEC / "w4b_pdf_engine.rs", art("pdf") / "⚙️engine/🦀️component.rs")
patch_raster("pdf", "Pdf", "pdf")

(art("glb") / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(GLB_SNAPSHOT, encoding="utf-8")
shutil.copy(CODEC / "w4b_glb_engine.rs", art("glb") / "⚙️engine/🦀️component.rs")
imp = art("glb") / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{BINARY}/🦀️component.rs"
imp.write_text(IO_BINARY_IMP.format(mid="glb", Name="Glb", MID="GLB"), encoding="utf-8")
(art("glb") / f"🚪️io/📤️export/{SER}/🗿️artifacts/{BINARY}/🦀️component.rs").write_text(
    IO_BINARY_SER.format(mid="glb", Name="Glb", MID="GLB"), encoding="utf-8"
)
json_dir = ROSTER["json"]["dir"]
for side, folder in (("import", DESER), ("export", SER)):
    p = art("glb") / f"🚪️io/📥️import/{folder}/🗿️artifacts/{json_dir}/🦀️component.rs" if side == "import" else art("glb") / f"🚪️io/📤️export/{folder}/🗿️artifacts/{json_dir}/🦀️component.rs"
    if p.exists():
        t = p.read_text(encoding="utf-8")
        t = t.replace("decode_pack(bytes)", "decode_glb_from_json(from)")
        t = t.replace("BinarySnapshot", "JsonSnapshot")
        if "decode_glb_from_json" in t and "fn decode_glb_from_json" not in t:
            t = t.replace(
                "pub fn deserialize(from: &JsonSnapshot)",
                "pub fn decode_glb_from_json(from: &JsonSnapshot) -> Result<GlbSnapshot, store::PackError> {\n    let snap = GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: crate::artifacts::glb::schema::snapshot::GlbPayload { gltf_json: serde_json::to_string(&from.value).unwrap_or_default(), bin: Vec::new() } };\n    Ok(snap)\n}\n\npub fn deserialize(from: &JsonSnapshot)",
            )
        p.write_text(t, encoding="utf-8")

for mid, name, ext in [("docx", "Docx", "docx"), ("pptx", "Pptx", "pptx"), ("xlsx", "Xlsx", "xlsx"), ("bcf", "Bcf", "bcf")]:
    mid_u = mid.upper()
    cap = name
    (art(mid) / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(
        OFFICE_SNAPSHOT.format(mid=mid, Name=name, MID=mid_u, ext=ext), encoding="utf-8"
    )
    eng = office_tpl.replace("{mid}", mid).replace("{Name}", name).replace("{MID}", mid_u).replace("{mid_cap}", cap)
    (art(mid) / "⚙️engine/🦀️component.rs").write_text(eng, encoding="utf-8")
    zd = ROSTER["zip"]["dir"]
    impz = art(mid) / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{zd}/🦀️component.rs"
    if impz.exists():
        t = impz.read_text(encoding="utf-8")
        t = t.replace("ZipSnapshot", f"{name}Snapshot")
        t = t.replace("STDIO_ZIP_DOCUMENT_SCHEMA", f"STDIO_{mid_u}_DOCUMENT_SCHEMA")
        t = t.replace(f"crate::artifacts::{mid}::engine::decode_zip", f"crate::artifacts::{mid}::engine::decode_{mid}")
        t = t.replace(f"crate::artifacts::zip::engine::decode_zip", f"crate::artifacts::{mid}::engine::decode_{mid}")
        impz.write_text(t, encoding="utf-8")

# fix bmp wrap_binary
bmp_snap = art("bmp") / "🧬️schema/📸️snapshot/🦀️component.rs"
if bmp_snap.exists():
    t = bmp_snap.read_text(encoding="utf-8").replace("wrap_bmp", "wrap_binary").replace("unwrap_bmp", "unwrap_binary")
    bmp_snap.write_text(t, encoding="utf-8")

# glue
glue = GLUE.read_text(encoding="utf-8")
zip_start = glue.index("    pub mod zip {")
zip_end = glue.index("    pub mod step {")
zip_tpl = glue[glue.rfind('    #[path = "."]', 0, zip_start) : zip_end]
def_start = glue.index("    pub mod deflate {")
def_end = glue.index("    pub mod zip {", def_start)
def_tpl = glue[glue.rfind('    #[path = "."]', 0, def_start) : def_end]
end_pos = glue.rfind("\n}\n//#endregion Artifacts")
if end_pos < 0:
    raise SystemExit("glue end missing")

def zip_block(mid: str, emoji: str) -> str:
    b = zip_tpl.replace("pub mod zip", f"pub mod {mid}").replace("🎒️zip", emoji).replace("/zip/", f"/{mid}/")
    return b

def def_block(mid: str, emoji: str) -> str:
    b = def_tpl.replace("pub mod deflate", f"pub mod {mid}").replace("🗜️deflate", emoji).replace("/deflate/", f"/{mid}/")
    return b

def office_block(mid: str, emoji: str) -> str:
    b = zip_tpl.replace("pub mod zip", f"pub mod {mid}").replace("🎒️zip", emoji).replace("/zip/", f"/{mid}/")
    b = b.replace("pub mod deflate", "pub mod xml").replace("🗜️deflate", "📰xml").replace("/deflate/", "/xml/")
    b = b.replace("deserializers::artifacts::binary", "deserializers::artifacts::zip")
    b = b.replace("serializers::artifacts::binary", "serializers::artifacts::zip")
    b = b.replace("🗿️artifacts/💾️binary/🚪️io", f"🗿️artifacts/{ROSTER['zip']['dir']}/🚪️io")
    return b

def glb_block(mid: str, emoji: str) -> str:
    b = zip_tpl.replace("pub mod zip", f"pub mod {mid}").replace("🎒️zip", emoji).replace("/zip/", f"/{mid}/")
    b = b.replace("pub mod deflate", "pub mod json").replace("🗜️deflate", "🔣️json").replace("/deflate/", "/json/")
    return b

new_blocks = []
for mid in ("png", "pdf"):
    if f"pub mod {mid}" not in glue:
        new_blocks.append(zip_block(mid, ROSTER[mid]["dir"]))
for mid in ("jpg", "gif", "tiff"):
    if f"pub mod {mid}" not in glue:
        new_blocks.append(def_block(mid, ROSTER[mid]["dir"]))
for mid in ("docx", "pptx", "xlsx", "bcf"):
    if f"pub mod {mid}" not in glue:
        new_blocks.append(office_block(mid, ROSTER[mid]["dir"]))
if "pub mod glb" not in glue:
    new_blocks.append(glb_block("glb", ROSTER["glb"]["dir"]))

if new_blocks:
    glue = glue[:end_pos] + "".join(new_blocks) + glue[end_pos:]
    GLUE.write_text(glue, encoding="utf-8")

mids = ["png", "jpg", "gif", "tiff", "pdf", "docx", "pptx", "xlsx", "bcf", "glb"]
pt = PLUGIN_RS.read_text(encoding="utf-8")
for mid in mids:
    reg = f"    crate::artifacts::{mid}::engine::register();"
    kind = f"        .artifact_kind(crate::artifacts::{mid}::artifact_kind())"
    if reg not in pt:
        pt = pt.replace("    crate::artifacts::deflate::engine::register();", "    crate::artifacts::deflate::engine::register();\n" + reg)
    if kind not in pt:
        pt = pt.replace("        .artifact_kind(crate::artifacts::deflate::artifact_kind())", "        .artifact_kind(crate::artifacts::deflate::artifact_kind())\n" + kind)
PLUGIN_RS.write_text(pt, encoding="utf-8")

lines = INDEX_TS.read_text(encoding="utf-8").strip().splitlines()
for mid in mids:
    line = f'export * as {mid} from "../../🗿️artifacts/{ROSTER[mid]["dir"]}/🟦️component.ts";'
    if line not in lines:
        lines.append(line)
INDEX_TS.write_text("\n".join(lines) + "\n", encoding="utf-8")
print("applied w4b codecs + glue")
