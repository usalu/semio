#!/usr/bin/env python3
"""Generate 🚪️io facets for all 54 artifacts, wire glue.rs, and update engines."""
from __future__ import annotations

import json
import re
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]  # may be wrong — fix below
# Resolve repo root by walking up until we find ✏️s/🔌️plugins
HERE = Path(__file__).resolve().parent
ROOT = HERE
while ROOT != ROOT.parent and not (ROOT / "✏️s" / "🔌️plugins").exists():
    ROOT = ROOT.parent
assert (ROOT / "✏️s" / "🔌️plugins").exists(), ROOT

TICKET = HERE
owners_doc = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
catalog = owners_doc["catalog_formats"]
owners_by_artifact = {o["artifact"]: o for o in owners_doc["owners"]}

VARIANT = {
    "glb": "Glb", "gltf": "Gltf", "stl": "Stl", "obj": "Obj", "ply": "Ply", "las": "Las",
    "step": "Step", "ifc": "Ifc", "dwg": "Dwg", "dxf": "Dxf", "svg": "Svg", "png": "Png",
    "jpg": "Jpg", "gif": "Gif", "bmp": "Bmp", "tiff": "Tiff", "pdf": "Pdf", "docx": "Docx",
    "pptx": "Pptx", "csv": "Csv", "xlsx": "Xlsx", "md": "Md", "txt": "Txt", "zip": "Zip",
    "bcf": "Bcf", "json": "Json",
}

ASCII_OVERRIDES = {
    ("🔱️trinity", "♻️rewrite"): "rewrite",
    ("🔱️trinity", "🔌️jack"): "jack",
    ("🖍️draw", "🖍️draw"): "draw",
}

KIND_OVERRIDES = {
    "📐️cad": "3d.cad",
    "🎬️present": "2d.present",
}


def discover() -> list[dict]:
    rows = []
    plugins = ROOT / "✏️s" / "🔌️plugins"
    for plugin_dir in sorted(plugins.iterdir()):
        if not plugin_dir.is_dir():
            continue
        preferred = plugin_dir / "📦️packages" / "🦀️rust" / "📦️glue.rs"
        if preferred.exists():
            glue = preferred
        else:
            glues = [p for p in plugin_dir.glob("**/📦️glue.rs") if "🔨️modules" not in p.parts]
            if not glues:
                glues = list(plugin_dir.glob("**/📦️glue.rs"))
            if not glues:
                continue
            glue = sorted(glues, key=lambda p: len(p.parts))[0]
        text = glue.read_text(encoding="utf-8")
        arts = plugin_dir / "🗿️artifacts"
        if not arts.exists():
            continue
        for art in sorted(arts.iterdir()):
            if not art.is_dir():
                continue
            ascii_name = ASCII_OVERRIDES.get((plugin_dir.name, art.name))
            if ascii_name is None:
                needle = f"🗿️artifacts/{art.name}/"
                idx = text.find(needle)
                if idx >= 0:
                    before = text[:idx]
                    mods = list(re.finditer(r"pub\s+mod\s+(\w+)\s*\{", before))
                    if mods:
                        ascii_name = mods[-1].group(1)
            snap = None
            snap_path = art / "📸️snapshot" / "🧬️schema" / "🦀️component.rs"
            if snap_path.exists():
                mt = re.search(r"pub\s+struct\s+(\w+Snapshot)\b", snap_path.read_text(encoding="utf-8"))
                if mt:
                    snap = mt.group(1)
            owner = owners_by_artifact.get(art.name, {})
            kind = KIND_OVERRIDES.get(art.name) or owner.get("kind_id")
            if not kind:
                # parse artifact_kind id
                comp = art / "🦀️component.rs"
                if comp.exists():
                    m = re.search(r'fn\s+\w*artifact_kind\w*\s*\([^)]*\)[^{]*\{([\s\S]*?)\n\}', comp.read_text(encoding="utf-8"))
                    if m:
                        mm = re.search(r'\bid:\s*"([^"]+)"', m.group(1))
                        if mm:
                            kind = mm.group(1)
            if not kind:
                kind = f"artifact.{ascii_name or art.name}"
            formats = owner.get("formats") or ["json"]
            rows.append({
                "plugin": plugin_dir.name,
                "artifact": art.name,
                "ascii": ascii_name,
                "snapshot": snap,
                "kind_id": kind,
                "formats": formats,
                "glue": str(glue.relative_to(ROOT)),
                "art_dir": str(art.relative_to(ROOT)),
            })
    return rows


def write(path: Path, contents: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(contents, encoding="utf-8")


def export_rs(ascii_name: str, snapshot: str, kind: str, fmt: str) -> str:
    var = VARIANT[fmt]
    return f'''//! 📤️ Export {snapshot} as .{fmt}.

use semio_framework_plugin::{{DocumentCodec, IoError, JsonCodec, MediaFormat}};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::{ascii_name}::{snapshot}) -> Result<Vec<u8>, IoError> {{
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    JsonCodec.export(&value)
}}

pub fn register() {{
    let kind = "{kind}";
    let format = MediaFormat::{var};
    semio_framework_os::register_os_media_export_handler(kind, format, |doc| {{
        let snapshot: crate::artifacts::{ascii_name}::{snapshot} =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        let stem = kind.replace('.', "_");
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, &stem)
    }});
}}
//#endregion 🔖️Export
'''


def import_rs(ascii_name: str, snapshot: str, kind: str, fmt: str) -> str:
    var = VARIANT[fmt]
    return f'''//! 📥️ Import .{fmt} into {snapshot}.

use semio_framework_plugin::{{DocumentCodec, IoError, JsonCodec, MediaFormat}};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::{ascii_name}::{snapshot}, IoError> {{
    let value = match JsonCodec.import(bytes) {{
        Ok(v) => v,
        Err(_) if format == MediaFormat::{var} => {{
            // Non-JSON bytes: wrap as a JSON envelope so snapshot serde can still be attempted later.
            let _ = format;
            serde_json::json!({{
                "schema": "{kind}",
                "importedFormat": "{fmt}",
                "raw": String::from_utf8_lossy(bytes),
            }})
        }}
        Err(e) => return Err(e),
    }};
    // Prefer full snapshot decode; fall back to default-ish JSON object merge is not available —
    // require JSON snapshots for interchange.
    serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()))
}}

pub fn register() {{
    let kind = "{kind}";
    let format = MediaFormat::{var};
    semio_framework_os::register_os_media_import_handler(kind, format, |bytes| {{
        let snapshot = import(bytes).map_err(|e| e.to_string())?;
        serde_json::to_value(snapshot).map_err(|e| e.to_string())
    }});
}}
//#endregion 🔖️Import
'''


def import_rs_fixed(ascii_name: str, snapshot: str, kind: str, fmt: str) -> str:
    var = VARIANT[fmt]
    return f'''//! 📥️ Import .{fmt} into {snapshot}.

use semio_framework_plugin::{{DocumentCodec, IoError, JsonCodec, MediaFormat}};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::{ascii_name}::{snapshot}, IoError> {{
    match JsonCodec.import(bytes) {{
        Ok(value) => serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string())),
        Err(primary) => {{
            // Textual formats may arrive as raw UTF-8; attempt JSON parse of the string body first.
            if let Ok(text) = std::str::from_utf8(bytes) {{
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {{
                    return serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()));
                }}
            }}
            let _ = MediaFormat::{var};
            Err(primary)
        }}
    }}
}}

pub fn register() {{
    let kind = "{kind}";
    let format = MediaFormat::{var};
    semio_framework_os::register_os_media_import_handler(kind, format, |bytes| {{
        let snapshot = import(bytes).map_err(|e| e.to_string())?;
        serde_json::to_value(snapshot).map_err(|e| e.to_string())
    }});
}}
//#endregion 🔖️Import
'''


def root_rs(ascii_name: str, artifact: str, formats: list[str]) -> str:
    specs = ",\n".join(
        f"        IoFormatSpec {{ format: MediaFormat::{VARIANT[f]}, import: true, export: true }}"
        for f in formats
    )
    regs = "\n".join(
        f"        super::{f}::export::register();\n        super::{f}::import::register();"
        for f in formats
    )
    return f'''//! 🚪️ {artifact} IO facet — declared MediaFormat table + OS handler registration.

use semio_framework_plugin::{{ArtifactIo, IoFormatSpec, MediaFormat}};

//#region 🔖️Formats
pub fn format_specs() -> &'static [IoFormatSpec] {{
    &[
{specs}
    ]
}}
//#endregion 🔖️Formats

//#region 🔖️ArtifactIo
/// 🚪️ {ascii_name} artifact IO registration surface.
pub struct Io;

impl ArtifactIo for Io {{
    fn formats() -> &'static [IoFormatSpec] {{ format_specs() }}
    fn register() {{
{regs}
    }}
}}

pub fn register() {{
    <Io as ArtifactIo>::register();
}}
//#endregion 🔖️ArtifactIo
'''


def note_special_export(fmt: str) -> str | None:
    """Pilot-quality converters for note specialized formats."""
    if fmt == "svg":
        return '''//! 📤️ Export NoteSnapshot as .svg.

use semio_framework_plugin::{IoError, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::note::NoteSnapshot) -> Result<Vec<u8>, IoError> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    Ok(svg.into_bytes())
}

pub fn register() {
    let kind = "2d.note";
    let format = MediaFormat::Svg;
    semio_framework_os::register_os_media_export_handler(kind, format, |doc| {
        let snapshot: crate::artifacts::note::NoteSnapshot =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, "note")
    });
}
//#endregion 🔖️Export
'''
    if fmt == "json":
        return None  # use generic
    if fmt == "dwg":
        return '''//! 📤️ Export NoteSnapshot as .dwg.

use semio_framework_plugin::{IoError, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::note::NoteSnapshot) -> Result<Vec<u8>, IoError> {
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    // Reuse the OS 2D→DWG bridge via SVG intermediate.
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    let _ = value;
    semio_framework_os::svg_to_dwg_bytes(&svg).map_err(|e| IoError::Payload(e))
}

pub fn register() {
    let kind = "2d.note";
    let format = MediaFormat::Dwg;
    semio_framework_os::register_os_media_export_handler(kind, format, |doc| {
        let snapshot: crate::artifacts::note::NoteSnapshot =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, "note")
    });
}
//#endregion 🔖️Export
'''
    return None


def note_special_import(fmt: str) -> str | None:
    if fmt == "dwg":
        return '''//! 📥️ Import .dwg into NoteSnapshot.

use semio_framework_plugin::{IoError, MediaFormat};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::note::NoteSnapshot, IoError> {
    let drawing = semio_framework_plugin::dwg_from_bytes(bytes).map_err(|e| IoError::Payload(e))?;
    let value = crate::artifacts::note::engine::note_document_json_from_dwg(&drawing)
        .map_err(|e| IoError::Payload(e))?;
    serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()))
}

pub fn register() {
    let kind = "2d.note";
    let format = MediaFormat::Dwg;
    semio_framework_os::register_os_media_import_handler(kind, format, |bytes| {
        let snapshot = import(bytes).map_err(|e| e.to_string())?;
        serde_json::to_value(snapshot).map_err(|e| e.to_string())
    });
}
//#endregion 🔖️Import
'''
    return None


def generate_artifact(row: dict) -> None:
    ascii_name = row["ascii"]
    snapshot = row["snapshot"]
    kind = row["kind_id"]
    formats = row["formats"]
    art = ROOT / row["art_dir"]
    io_root = art / "🚪️io"
    if io_root.exists():
        shutil.rmtree(io_root)

    write(io_root / "🦀️component.rs", root_rs(ascii_name, row["artifact"], formats))
    write(io_root / "🟦️component.ts", "/** 🚪️ IO facet barrel — WASM facades land in W7. */\nexport {};\n")

    for fmt in formats:
        fdir = catalog[fmt]["dir"]
        exp = None
        imp = None
        if row["artifact"] == "🗒️note":
            exp = note_special_export(fmt)
            imp = note_special_import(fmt)
        if exp is None:
            exp = export_rs(ascii_name, snapshot, kind, fmt)
        if imp is None:
            imp = import_rs_fixed(ascii_name, snapshot, kind, fmt)
        write(io_root / fdir / "📤️export" / "🦀️component.rs", exp)
        write(io_root / fdir / "📤️export" / "🟦️component.ts", "export {};\n")
        write(io_root / fdir / "📥️import" / "🦀️component.rs", imp)
        write(io_root / fdir / "📥️import" / "🟦️component.ts", "export {};\n")


def glue_io_block(art_emoji: str, ascii_name: str, formats: list[str], indent: str = "        ") -> str:
    lines = [
        f"{indent}#[path = \".\"]",
        f"{indent}pub mod io {{",
        f"{indent}    #[path = \"../../🗿️artifacts/{art_emoji}/🚪️io/🦀️component.rs\"]",
        f"{indent}    mod component;",
        f"{indent}    pub use component::*;",
    ]
    for fmt in formats:
        fdir = catalog[fmt]["dir"]
        lines += [
            f"{indent}    #[path = \".\"]",
            f"{indent}    pub mod {fmt} {{",
            f"{indent}        #[path = \".\"]",
            f"{indent}        pub mod export {{",
            f"{indent}            #[path = \"../../🗿️artifacts/{art_emoji}/🚪️io/{fdir}/📤️export/🦀️component.rs\"]",
            f"{indent}            mod component;",
            f"{indent}            pub use component::*;",
            f"{indent}        }}",
            f"{indent}        #[path = \".\"]",
            f"{indent}        pub mod import {{",
            f"{indent}            #[path = \"../../🗿️artifacts/{art_emoji}/🚪️io/{fdir}/📥️import/🦀️component.rs\"]",
            f"{indent}            mod component;",
            f"{indent}            pub use component::*;",
            f"{indent}        }}",
            f"{indent}    }}",
        ]
    lines.append(f"{indent}}}")
    return "\n".join(lines) + "\n"


def patch_glue(row: dict) -> None:
    glue_path = ROOT / row["glue"]
    text = glue_path.read_text(encoding="utf-8")
    ascii_name = row["ascii"]
    art = row["artifact"]
    # Remove any existing io block for this artifact (between pub mod io and matching close is hard);
    # detect by path fragment.
    marker = f"🗿️artifacts/{art}/🚪️io/"
    if marker in text:
        # strip previous generated io module: find `pub mod io {` whose window contains marker
        pattern = re.compile(
            rf'[ \t]*#\[path = "\."\]\s*\n[ \t]*pub mod io \{{[\s\S]*?🗿️artifacts/{re.escape(art)}/🚪️io/[\s\S]*?\n[ \t]*\}}\n',
            re.M,
        )
        text2, n = pattern.subn("", text, count=1)
        if n:
            text = text2
        else:
            # fallback: leave and skip insert if still present after failed strip
            pass

    block = glue_io_block(art, ascii_name, row["formats"])
    # Insert before `pub mod engine` for this artifact, or before closing of ascii mod
    engine_path = f'🗿️artifacts/{art}/⚙️engine/🦀️component.rs'
    eng_idx = text.find(engine_path)
    if eng_idx >= 0:
        # find start of the #[path] line for engine
        line_start = text.rfind("\n", 0, eng_idx) + 1
        # include preceding #[path = "."] pub mod if any — insert before the engine path attribute
        # Walk back to include #[path = "...engine..."] line only; insert block just before it.
        insert_at = line_start
        # If previous non-empty is `pub mod engine` path attr only — also check for `#[path = "."] pub
        # mod snapshot` style. Insert immediately before engine's #[path].
        text = text[:insert_at] + block + text[insert_at:]
    else:
        # insert before the closing of the ascii artifact mod: find `pub mod {ascii}` then its engine-less end
        # Fallback: before `pub mod dsl` path for this artifact
        for facet in ("🗣️dsl", "📡️spr", "⚙️engine"):
            needle = f"🗿️artifacts/{art}/{facet}/"
            idx = text.find(needle)
            if idx >= 0:
                line_start = text.rfind("\n", 0, idx) + 1
                text = text[:line_start] + block + text[line_start:]
                break
        else:
            raise RuntimeError(f"cannot find insert point in glue for {art}")

    glue_path.write_text(text, encoding="utf-8")


OLD_REG_PATTERNS = [
    re.compile(r"^[ \t]*semio_framework_os::register_2d_export_handlers\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_dwg_import_handler\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_mesh_exporter\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_mesh_importer\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_solid_exporter\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_solid_importer\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_mesh_dwg_export_handler\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_os_media_export_handler\([^;]+;\n", re.M),
    re.compile(r"^[ \t]*semio_framework_os::register_os_media_import_handler\([^;]+;\n", re.M),
]


def patch_engine(row: dict) -> None:
    eng = ROOT / row["art_dir"] / "⚙️engine" / "🦀️component.rs"
    if not eng.exists():
        return
    text = eng.read_text(encoding="utf-8")
    original = text
    for pat in OLD_REG_PATTERNS:
        text = pat.sub("", text)
    ascii_name = row["ascii"]
    call = f"    crate::artifacts::{ascii_name}::io::register();\n"
    if f"::io::register()" in text:
        # already has a call
        pass
    else:
        # Insert into pub fn register() body after opening brace
        m = re.search(r"pub fn register\s*\(\s*\)\s*\{", text)
        if m:
            insert_at = m.end()
            text = text[:insert_at] + "\n" + call + text[insert_at:]
        else:
            # append a register shim
            text += f"\n//#region 🔖️IoFacet\npub fn register_io() {{\n{call}}}\n//#endregion 🔖️IoFacet\n"
    if text != original:
        eng.write_text(text, encoding="utf-8")


def main() -> None:
    rows = discover()
    missing = [r for r in rows if not r["ascii"] or not r["snapshot"]]
    if missing:
        raise SystemExit(f"missing ascii/snapshot: {missing}")
    (TICKET / "🧪discovered-artifacts.json").write_text(json.dumps(rows, indent=2, ensure_ascii=False), encoding="utf-8")
    for row in rows:
        print(f"gen {row['plugin']}/{row['artifact']} ascii={row['ascii']} formats={len(row['formats'])}")
        generate_artifact(row)
        patch_glue(row)
        patch_engine(row)
    print(f"done {len(rows)} artifacts")


if __name__ == "__main__":
    main()
