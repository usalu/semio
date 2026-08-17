#!/usr/bin/env python3
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parents[1]
BATCH = json.loads((TICKET / "generators/w6-batch1c.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
TEXT = TOK["text"]


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))


def fix_example_includes(art: Path) -> None:
    for p in art.rglob(f"{TEXT}/🦀️component.rs"):
        t = p.read_text(encoding="utf-8")
        t2 = t.replace('include_str!("../📚️examples/', 'include_str!("../../../📚️examples/')
        if t2 != t:
            p.write_text(t2, encoding="utf-8")


def fix_diff_imports(art: Path, rust_mod: str) -> None:
    p = art / "🧬️schema/🔺️diff" / TEXT / "🦀️component.rs"
    if not p.exists():
        return
    t = p.read_text(encoding="utf-8")
    use_line = f"use crate::artifacts::{rust_mod}::schema::diff::*;\n"
    if use_line not in t:
        t = re.sub(
            r"(//#endregion 📖️SemioGrammar\n)\n",
            r"\1\n" + use_line + "\n",
            t,
            count=1,
        )
        p.write_text(t, encoding="utf-8")


PACK_IO_IMP = """//! {rust_mod} <- {slug}
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn deserialize(from: &{stdio_snap}) -> Result<{snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = <{stdio_snap} as store::DocumentPack>::encode_pack(from)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize_bytes(&bytes)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentPack>::decode_pack(bytes).or_else(|_| {{
        <{snap} as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    }})
}}
"""

PACK_IO_EXP = """//! {rust_mod} -> {slug}
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<{stdio_snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = <{snap} as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<{snap} as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    <{stdio_snap} as store::DocumentPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<{stdio_snap} as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
"""

TEXT_SLUGS = {"json", "csv", "md"}


def pascal(slug: str) -> str:
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[^a-zA-Z0-9]+", slug) if p)


def fix_binary_io(art: Path, rust_mod: str, slugs: list[str]) -> None:
    snap = pascal(rust_mod) + "Snapshot"
    if rust_mod == "process3d":
        snap = "Process3dSnapshot"
    elif rust_mod == "curate":
        snap = "CurateSnapshot"
    deser = TOK["deserializers"]
    ser = TOK["serializers"]
    roster = OWNER["stdio_roster"]
    for slug in slugs:
        if slug in TEXT_SLUGS:
            continue
        dname = roster[slug]["dir"]
        stdio_snap = pascal(slug) + "Snapshot"
        stdio_schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
        body = PACK_IO_IMP.format(
            rust_mod=rust_mod, slug=slug, snap=snap, stdio_snap=stdio_snap, stdio_schema=stdio_schema
        )
        (art / "🚪️io/📥️import" / deser / "🗿️artifacts" / dname / "🦀️component.rs").write_text(body, encoding="utf-8")
        body = PACK_IO_EXP.format(
            rust_mod=rust_mod, slug=slug, snap=snap, stdio_snap=stdio_snap, stdio_schema=stdio_schema
        )
        (art / "🚪️io/📤️export" / ser / "🗿️artifacts" / dname / "🦀️component.rs").write_text(body, encoding="utf-8")


def fix_json_io(art: Path, rust_mod: str) -> None:
    snap = pascal(rust_mod) + "Snapshot"
    if rust_mod == "process3d":
        snap = "Process3dSnapshot"
    elif rust_mod == "curate":
        snap = "CurateSnapshot"
    imp = art / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"
    if imp.exists():
        imp.write_text(
            f"""//! {rust_mod} <- json
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &JsonSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.value.clone()).map_err(|e| store::TextError::new(format!("{rust_mod}<-json: {{e}}"), dsl::TextSpan::at(1, 1)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
""",
            encoding="utf-8",
        )


def fix_glue_op_line(plugin_path: Path) -> None:
    glue = plugin_path / "📦️packages/🦀️rust/📦️glue.rs"
    t = glue.read_text(encoding="utf-8")
    t2 = re.sub(
        r"pub mod op \{ pub use crate::artifacts::\w+::schema::mutations::text::\*; pub use crate::artifacts::\w+::schema::mutations::\w+Mutation; \}",
        lambda m: m.group(0).split("; pub use")[0] + "; }",
        t,
    )
    glue.write_text(t2, encoding="utf-8")


def main() -> None:
    for entry in BATCH:
        row = owner_row(entry["plugin"], entry["artifact"])
        art = ROOT / row["path"]
        slugs = row.get("import") or []
        fix_example_includes(art)
        fix_diff_imports(art, entry["rust_mod"])
        fix_json_io(art, entry["rust_mod"])
        fix_binary_io(art, entry["rust_mod"], slugs)
        fix_glue_op_line(art.parent.parent)
        print("fixup", entry["artifact"])


if __name__ == "__main__":
    main()
