#!/usr/bin/env python3
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
OT = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
DESER, SER = TOK["deserializers"], TOK["serializers"]
STDIO_DIRS = {k: v["dir"] for k, v in OT["stdio_roster"].items()}

META = {
    "🖍️draw": ("draw", "Draw", "DRAW_DOCUMENT_SCHEMA", "draw_document_json_from_dwg", "empty_draw_snapshot", "create_draw_id"),
    "🖨️raster": ("raster", "Raster", "RASTER_DOCUMENT_SCHEMA", "raster_document_json_from_dwg", "empty_raster_snapshot", "create_raster_id"),
    "📏️layout": ("layout", "Layout", "LAYOUT_DOCUMENT_SCHEMA", "layout_document_json_from_dwg", "empty_layout_snapshot", "create_layout_id"),
    "📋️forms": ("forms", "Forms", "FORMS_DOCUMENT_SCHEMA", None, "empty_forms_snapshot", "create_forms_id"),
    "📖️playbook": ("playbook", "Playbook", "PLAYBOOK_DOCUMENT_SCHEMA", None, "empty_playbook_snapshot", "create_playbook_id"),
}


def art(emoji: str) -> Path:
    return ROOT / next(r["path"] for r in OT["owners"] if r["plugin"] == emoji)


def w(path: Path, body: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body, encoding="utf-8")


def imp(mod, p, sch, slug, dwg_fn, empty_fn, create_fn, stdio_dir):
    base = art_path(mod) / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{stdio_dir}/🦀️component.rs"
    if slug == "json":
        w(
            base,
            f"""//! {mod} <- json
use crate::artifacts::{mod}::{{{p}Snapshot, {sch}}};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};
pub fn register() {{}}
pub fn deserialize(from: &JsonSnapshot) -> Result<{p}Snapshot, String> {{
    let mut snap: {p}Snapshot = serde_json::from_value(from.value.clone()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() {{ snap.schema = {sch}.into(); }}
    Ok(snap)
}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{p}Snapshot, String> {{
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
""",
        )
        return
    if slug == "dwg" and dwg_fn:
        w(
            base,
            f"""//! {mod} <- dwg
use crate::artifacts::{mod}::{p}Snapshot;
use semio_framework::{{dwg_from_bytes, DwgDrawing}};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;
pub fn register() {{}}
pub fn deserialize(from: &DwgSnapshot) -> Result<{p}Snapshot, String> {{ deserialize_bytes(&from.bytes) }}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{p}Snapshot, String> {{
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    let value = crate::artifacts::{mod}::engine::{dwg_fn}(&drawing)?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}}
""",
        )
        return
    if slug in {"md", "txt"}:
        w(
            base,
            f"""//! {mod} <- {slug}
use crate::artifacts::{mod}::{p}Snapshot;
pub fn register() {{}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{p}Snapshot, String> {{
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    <{p}Snapshot as store::DocumentDsl>::parse_dsl(text).map_err(|e| e.to_string())
}}
""",
        )
        return
    w(
        base,
        f"""//! {mod} <- {slug}
use crate::artifacts::{mod}::engine::{{{empty_fn}, {create_fn}}};
use crate::artifacts::{mod}::{p}Snapshot;
pub fn register() {{}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{p}Snapshot, String> {{
    let _ = bytes;
    let mut snap = {empty_fn}();
    snap.id = {create_fn}("{slug}-import", b"{slug}");
    snap.title = Some(format!("Imported {slug}"));
    Ok(snap)
}}
""",
    )


def exp(mod, p, slug, stdio_dir):
    base = art_path(mod) / f"🚪️io/📤️export/{SER}/🗿️artifacts/{stdio_dir}/🦀️component.rs"
    if slug == "json":
        w(
            base,
            f"""//! {mod} -> json
use crate::artifacts::{mod}::{p}Snapshot;
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};
pub fn register() {{}}
pub fn serialize(snapshot: &{p}Snapshot) -> Result<JsonSnapshot, String> {{
    Ok(JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_json::to_value(snapshot).map_err(|e| e.to_string())? }})
}}
pub fn serialize_bytes(snapshot: &{p}Snapshot) -> Result<Vec<u8>, String> {{
    serde_json::to_vec_pretty(&serialize(snapshot)?.value).map_err(|e| e.to_string())
}}
""",
        )
        return
    w(
        base,
        f"""//! {mod} -> {slug}
use crate::artifacts::{mod}::{p}Snapshot;
pub fn register() {{}}
pub fn serialize_bytes(snapshot: &{p}Snapshot) -> Result<Vec<u8>, String> {{
    Ok(<{p}Snapshot as store::DocumentDsl>::render_dsl(snapshot).into_bytes())
}}
""",
    )


def art_path(mod: str) -> Path:
    for emoji, (m, *_r) in META.items():
        if m == mod:
            return art(emoji)
    raise KeyError(mod)


for emoji, tup in META.items():
    mod, p, sch, dwg_fn, empty_fn, create_fn = tup
    slugs = next(r["stdio_artifacts"] for r in OT["owners"] if r["plugin"] == emoji)
    for slug in slugs:
        d = STDIO_DIRS[slug]
        imp(mod, p, sch, slug, dwg_fn, empty_fn, create_fn, d)
        exp(mod, p, slug, d)
    print(emoji, len(slugs))
