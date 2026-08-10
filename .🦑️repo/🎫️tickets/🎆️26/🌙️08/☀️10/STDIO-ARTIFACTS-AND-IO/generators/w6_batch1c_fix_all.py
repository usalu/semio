#!/usr/bin/env python3
"""Fix batch1c IO + known symbol/syntax issues (batch1a/b patterns)."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = Path(__file__).resolve().parents[1]
BATCH = json.loads((TICKET / "generators/w6-batch1c.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
DESER, SER = TOK["deserializers"], TOK["serializers"]
STDIO_DIRS = {k: v["dir"] for k, v in OWNER["stdio_roster"].items()}

SNAP = {
    "remodel": ("RemodelSnapshot", "REMODEL_DOCUMENT_SCHEMA", "crate::artifacts::remodel::"),
    "playground": ("PlaygroundSnapshot", "PLAYGROUND_DOCUMENT_SCHEMA", "crate::artifacts::playground::"),
    "present": ("PresentSnapshot", "PRESENT_DOCUMENT_SCHEMA", "crate::artifacts::present::"),
    "shooting": ("ShootingSnapshot", "SHOOTING_DOCUMENT_SCHEMA", "crate::artifacts::shooting::"),
    "program": ("ProgramSnapshot", "ARCHITECT_PROGRAM_SCHEMA", "crate::artifacts::program::"),
    "process3d": ("Process3dSnapshot", "PROCESS_3D_SCHEMA", "crate::artifacts::process3d::"),
    "lowpoly": ("LowpolySnapshot", "LOWPOLY_DOCUMENT_SCHEMA", "crate::artifacts::lowpoly::"),
    "wires": ("WiresSnapshot", "MINDMAP_WIRES_SCHEMA", "crate::artifacts::wires::"),
    "home": ("SHomeSnapshot", "S_HOME_DOCUMENT_SCHEMA", "crate::artifacts::home::"),
    "curate": ("CurateSnapshot", "SOURCING_CURATE_SCHEMA", "crate::artifacts::curate::"),
}

JSON_SER = """//! {mod} -> json
use {root}{snap};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<JsonSnapshot, store::TextError> {{
    Ok(JsonSnapshot {{
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        value: serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?,
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    serde_json::to_vec_pretty(&serialize(snapshot)?.value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}
"""

HAS_SCHEMA = frozenset({"remodel", "program", "shooting", "present", "playground", "home", "lowpoly"})

JSON_IMP = """//! {mod} <- json
use {root}{snap};
use {root}{sch};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &JsonSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {sch};
    let mut out: {snap} = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("{label}<-json: {{e}}"), dsl::TextSpan::at(1, 1)))?;
{schema_fixup}    Ok(out)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
"""

MD_SER = """//! {mod} -> md
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<MdSnapshot, store::TextError> {{
    Ok(MdSnapshot {{
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <{snap} as store::DocumentDsl>::print_dsl(snapshot),
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<MdSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
"""

MD_IMP = """//! {mod} <- md
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &MdSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.body)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let md = <MdSnapshot as store::DocumentPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&md)
}}
"""

CSV_SER = """//! {mod} -> csv
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<CsvSnapshot, store::TextError> {{
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let headers = value.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    let rows = value.get("rows").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Ok(CsvSnapshot {{ schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
"""

CSV_IMP = """//! {mod} <- csv
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &CsvSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = serde_json::json!({{ "headers": from.headers, "rows": from.rows }});
    serde_json::from_value(value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}
"""

WIRE_SER = """//! {mod} -> {slug}
use {root}{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<{stdio_snap}, store::TextError> {{
    let _ = {stdio_schema};
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{label}->{slug}: {{e}}"), dsl::TextSpan::at(1, 1)))
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<{stdio_snap} as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
"""

WIRE_IMP = """//! {mod} <- {slug}
use {root}{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn deserialize(from: &{stdio_snap}) -> Result<{snap}, store::TextError> {{
    let _ = {stdio_schema};
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{label}<-{slug}: {{e}}"), dsl::TextSpan::at(1, 1)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let wire = <{stdio_snap} as store::DocumentPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}}
"""

PACK_SER = """//! {mod} -> {slug}
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<{stdio_snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = <{snap} as store::DocumentPack>::encode_pack(snapshot);
    <{stdio_snap} as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<{stdio_snap} as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
"""

PACK_IMP = """//! {mod} <- {slug}
use {root}schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn deserialize(from: &{stdio_snap}) -> Result<{snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = <{stdio_snap} as store::DocumentPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentPack>::decode_pack(bytes).or_else(|_| {{
        <{snap} as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    }})
}}
"""


def pascal(slug: str) -> str:
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[^a-zA-Z0-9]+", slug) if p)


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))


def write_io(mod: str, art: Path, slugs: list[str]) -> None:
    snap, sch, root = SNAP[mod]
    for slug in slugs:
        d = STDIO_DIRS[slug]
        stdio_snap = pascal(slug) + "Snapshot"
        stdio_schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
        schema_fixup = (
            f"    if out.schema.is_empty() {{\n        out.schema = {sch}.into();\n    }}\n"
            if mod in HAS_SCHEMA
            else ""
        )
        ctx = dict(
            mod=mod,
            label=mod,
            snap=snap,
            sch=sch,
            root=root,
            slug=slug,
            stdio_snap=stdio_snap,
            stdio_schema=stdio_schema,
            schema_fixup=schema_fixup,
        )
        if slug == "json":
            imp, exp = JSON_IMP.format(**ctx), JSON_SER.format(**ctx)
        elif slug == "md":
            imp, exp = MD_IMP.format(**ctx), MD_SER.format(**ctx)
        elif slug == "csv":
            imp, exp = CSV_IMP.format(**ctx), CSV_SER.format(**ctx)
        elif slug in {"zip", "xlsx", "docx", "pdf", "pptx", "txt"}:
            imp, exp = WIRE_IMP.format(**ctx), WIRE_SER.format(**ctx)
        else:
            imp, exp = PACK_IMP.format(**ctx), PACK_SER.format(**ctx)
        imp_p = art / "🚪️io/📥️import" / DESER / "🗿️artifacts" / d / "🦀️component.rs"
        exp_p = art / "🚪️io/📤️export" / SER / "🗿️artifacts" / d / "🦀️component.rs"
        imp_p.parent.mkdir(parents=True, exist_ok=True)
        exp_p.parent.mkdir(parents=True, exist_ok=True)
        imp_p.write_text(imp, encoding="utf-8")
        exp_p.write_text(exp, encoding="utf-8")


def fix_remodel_builder(art: Path) -> None:
    for rel in ("🏗️builder/🦀️component.rs", "🪓️decomposer/🦀️component.rs"):
        p = art / rel
        t = p.read_text(encoding="utf-8")
        t = t.replace("WatertightReportSnapshot", "RemodelSnapshot")
        p.write_text(t, encoding="utf-8")
    for p in art.rglob("🚪️io/**/🦀️component.rs"):
        t = p.read_text(encoding="utf-8")
        t2 = t.replace("WatertightReportSnapshot", "RemodelSnapshot")
        if t2 != t:
            p.write_text(t2, encoding="utf-8")


def fix_sourcing_builder(art: Path) -> None:
    p = art / "🏗️builder/🦀️component.rs"
    t = p.read_text(encoding="utf-8")
    t = t.replace("CurateMutation", "SourcingMutation")
    p.write_text(t, encoding="utf-8")


def fix_procedural_comma() -> None:
    for name in ("🌀️procedural2d", "🧊️procedural3d"):
        p = ROOT / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts" / name / "🦀️component.rs"
        t = p.read_text(encoding="utf-8")
        t2 = t.replace("import_formats: vec![]        export_stdio_kinds", "import_formats: vec![],\n        export_stdio_kinds")
        p.write_text(t2, encoding="utf-8")


def fix_lowpoly_engine_reexports() -> None:
    p = ROOT / "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/⚙️engine/🦀️component.rs"
    t = p.read_text(encoding="utf-8")
    block = """pub use crate::artifacts::lowpoly::engine::paint::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from, stamp_brush};
pub use crate::artifacts::lowpoly::engine::media::{
    lowpoly_document_from_mesh,
    lowpoly_mesh_from_document,
    mesh_data_from_transfer,
    mesh_document_from_mesh,
    mesh_from_mesh_document,
};
"""
    t = re.sub(r"pub use paint::\{[^}]+\};\n+", "", t)
    t = re.sub(
        r"//#endregion ⚠️ Errors\n\n(?:pub use crate::artifacts::lowpoly::engine::(?:paint|media)::[^\n]+\n)+",
        f"//#endregion ⚠️ Errors\n\n{block}\n",
        t,
        count=1,
    )
    if block.strip() not in t:
        t = t.replace("//#endregion ⚠️ Errors\n", f"//#endregion ⚠️ Errors\n\n{block}\n", 1)
    p.write_text(t, encoding="utf-8")


def fix_animate_glue() -> None:
    p = ROOT / "✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs"
    t = p.read_text(encoding="utf-8")
    old = """            pub mod animate {
                pub use super::animation::*;
                pub use super::scene::*;
                pub use super::geometry::*;
                pub use super::camera::*;
                pub use super::text::*;
                pub use super::rate::*;
                pub use super::config::*;
            }"""
    new = """            pub mod animate {
                pub use super::animation::animation::*;
                pub use super::animation::animations_catalog::*;
                pub use super::scene::scene::*;
                pub use super::scene::section::*;
                pub use super::scene::sobject::*;
                pub use super::geometry::geometry::*;
                pub use super::geometry::three_d::*;
                pub use super::geometry::axes::*;
                pub use super::camera::camera::*;
                pub use super::camera::matrix::*;
                pub use super::text::color::*;
                pub use super::text::text::*;
                pub use super::rate::rate::*;
                pub use super::rate::updater::*;
                pub use super::config::config::*;
                pub use super::config::hash::*;
                pub use super::config::graph::*;
            }"""
    if old in t:
        t = t.replace(old, new)
        p.write_text(t, encoding="utf-8")


def fix_json_schema_field(mod: str) -> None:
    if mod != "home":
        return
    snap, sch, _ = SNAP[mod]
    # SHomeSnapshot uses schema field — json import already sets if empty; skip


def main() -> None:
    fix_procedural_comma()
    fix_lowpoly_engine_reexports()
    fix_animate_glue()
    for entry in BATCH:
        mod = entry["rust_mod"]
        if mod in ("imperative", "sequence"):
            continue
        row = owner_row(entry["plugin"], entry["artifact"])
        art = ROOT / row["path"]
        slugs = row.get("import") or row.get("stdio_artifacts") or []
        write_io(mod, art, slugs)
        if mod == "remodel":
            fix_remodel_builder(art)
        if mod == "curate":
            fix_sourcing_builder(art)
        fix_json_schema_field(mod)
        print("fixed", entry["crate"])


if __name__ == "__main__":
    main()
