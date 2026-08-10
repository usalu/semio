#!/usr/bin/env python3
"""W6 HEAVY: migrate multi-artifact plugins to new stdio shape."""
from __future__ import annotations

import importlib.util
import subprocess
import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
BATCH = json.loads((TICKET / "generators" / "w6-heavy.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))

BUILDER = TOK["builder"]
DECOMPOSER = TOK["decomposer"]
TEXT = TOK["text"]
BINARY = TOK["binary"]
DESER = TOK["deserializers"]
SER = TOK["serializers"]
TS_LEAF = "🟦️component.ts"

_SPEC = importlib.util.spec_from_file_location(
    "w6_batch1c_migrate", TICKET / "generators" / "w6_batch1c_migrate.py"
)
b1c = importlib.util.module_from_spec(_SPEC)
assert _SPEC.loader
_SPEC.loader.exec_module(b1c)


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))



def fixup_io(art: Path, rust_mod: str, snap: str, slugs: list[str]) -> None:
    """Apply pack-based IO templates used by batch1c fixup (engine-free)."""
    roster = OWNER["stdio_roster"]
    text_slugs = {"json", "csv", "md", "txt"}
    for slug in slugs:
        dname = roster[slug]["dir"]
        stdio_snap = b1c.pascal(slug) + "Snapshot"
        stdio_schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
        if slug == "json":
            imp = f"""//! {rust_mod} <- json
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
"""
            exp = f"""//! {rust_mod} -> json
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
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
        elif slug in ("csv", "md", "txt"):
            # keep batch1c write_io output for these
            continue
        else:
            imp = f"""//! {rust_mod} <- {slug}
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
            exp = f"""//! {rust_mod} -> {slug}
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
        (art / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs").write_text(imp, encoding="utf-8")
        (art / f"🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs").write_text(exp, encoding="utf-8")

    # example include path fix
    for p in art.rglob(f"{TEXT}/🦀️component.rs"):
        t = p.read_text(encoding="utf-8")
        t2 = t.replace('include_str!("../📚️examples/', 'include_str!("../../../📚️examples/')
        if t2 != t:
            p.write_text(t2, encoding="utf-8")


def migrate_one(entry: dict) -> dict:
    row = owner_row(entry["plugin"], entry["artifact"])
    art = ROOT / row["path"]
    plugin_path = art.parent.parent
    slugs = entry.get("stdio") or row.get("import") or row.get("stdio_artifacts") or []
    if not art.exists():
        raise FileNotFoundError(art)
    b1c.absorb(art)
    b1c.scaffold_leaves(art)
    snap, mut, diff, schema_const = b1c.sniff_types(art, entry["rust_mod"])
    b1c.write_builder(art, entry["rust_mod"], snap, mut, diff)
    b1c.write_io(art, entry["rust_mod"], snap, schema_const, slugs)
    fixup_io(art, entry["rust_mod"], snap, slugs)
    b1c.fix_schema_includes(art)
    b1c.ensure_cargo_dep(plugin_path)
    b1c.patch_artifact_kind(art, slugs)
    errs = b1c.verify_tree(art)
    return {
        "plugin": entry["plugin"],
        "artifact": entry["artifact"],
        "rust_mod": entry["rust_mod"],
        "crate": entry["crate"],
        "errors": errs,
        "slugs": slugs,
        "path": str(art),
        "builder": (art / BUILDER).exists(),
        "decomposer": (art / DECOMPOSER).exists(),
        "old_dsl": (art / "🗣️dsl").exists(),
        "snap": snap,
        "mut": mut,
        "diff": diff,
        "schema_const": schema_const,
    }


def patch_plugin_glue(plugin: str, entries: list[dict]) -> None:
    plugin_path = ROOT / "✏️s/🔌️plugins" / plugin
    glue = plugin_path / "📦️packages/🦀️rust/📦️glue.rs"
    text = glue.read_text(encoding="utf-8")
    start = -1
    end = -1
    for begin_mark, end_mark in (
        ("//#region 🗿️Artifacts", "//#endregion 🗿️Artifacts"),
        ("//#region 🔖️Artifacts", "//#endregion 🔖️Artifacts"),
    ):
        s = text.find(begin_mark)
        e = text.find(end_mark)
        if s >= 0 and e > s:
            start, end = s, e
            break
    if start < 0 or end < 0:
        raise RuntimeError(f"glue region missing: {glue}")
    nl = text.find("\n", start) + 1
    head = text[:nl]
    tail = text[end:]
    blocks = []
    for entry in entries:
        row = owner_row(entry["plugin"], entry["artifact"])
        art = ROOT / row["path"]
        slugs = entry.get("stdio") or row.get("import") or row.get("stdio_artifacts") or []
        block = b1c.glue_artifact_block(
            entry["plugin"],
            entry["artifact"],
            entry["rust_mod"],
            entry["artifact"],
            slugs,
            entry.get("extras") or [],
            art,
        )
        if not block.lstrip().startswith("#[path"):
            block = '    #[path = "."]\n' + block
        snap, mut, diff, _schema = b1c.sniff_types(art, entry["rust_mod"])
        inject = (
            f"        pub use crate::artifacts::{entry['rust_mod']}::schema::snapshot::{snap};\n"
            f"        pub use crate::artifacts::{entry['rust_mod']}::schema::mutations::{mut};\n"
            f"        pub use crate::artifacts::{entry['rust_mod']}::schema::diff::{diff};\n"
        )
        marker = "        pub use component::*;\n"
        if marker in block and inject not in block:
            block = block.replace(marker, marker + inject, 1)
        blocks.append(block)
    body = '#[path = "."]\npub mod artifacts {\n' + "".join(blocks) + "}\n"
    glue.write_text(head + body + tail, encoding="utf-8")
    b1c.ensure_cargo_dep(plugin_path)


def patch_plugin_ts(plugin: str, entries: list[dict]) -> None:
    plugin_path = ROOT / "✏️s/🔌️plugins" / plugin
    ts_path = plugin_path / "📦️packages/🟦️typescript/📦️index.ts"
    lines = ["/** heavy plugin facet WASM facades */"]
    for entry in entries:
        artifact = entry["artifact"]
        rust_mod = entry["rust_mod"]
        ap = f"../../🗿️artifacts/{artifact}"
        lines.extend([
            f'export * as {rust_mod}_schema from "{ap}/🧬️schema/{TS_LEAF}";',
            f'export * as {rust_mod}_snapshot from "{ap}/🧬️schema/📸️snapshot/{TS_LEAF}";',
            f'export * as {rust_mod}_snapshot_text from "{ap}/🧬️schema/📸️snapshot/{TEXT}/{TS_LEAF}";',
            f'export * as {rust_mod}_snapshot_binary from "{ap}/🧬️schema/📸️snapshot/{BINARY}/{TS_LEAF}";',
            f'export * as {rust_mod}_diff from "{ap}/🧬️schema/🔺️diff/{TS_LEAF}";',
            f'export * as {rust_mod}_diff_text from "{ap}/🧬️schema/🔺️diff/{TEXT}/{TS_LEAF}";',
            f'export * as {rust_mod}_diff_binary from "{ap}/🧬️schema/🔺️diff/{BINARY}/{TS_LEAF}";',
            f'export * as {rust_mod}_mutations from "{ap}/🧬️schema/🧬️mutations/{TS_LEAF}";',
            f'export * as {rust_mod}_mutations_text from "{ap}/🧬️schema/🧬️mutations/{TEXT}/{TS_LEAF}";',
            f'export * as {rust_mod}_mutations_binary from "{ap}/🧬️schema/🧬️mutations/{BINARY}/{TS_LEAF}";',
            f'export * as {rust_mod}_io from "{ap}/🚪️io/{TS_LEAF}";',
            f'export * as {rust_mod}_builder from "{ap}/{BUILDER}/{TS_LEAF}";',
            f'export * as {rust_mod}_decomposer from "{ap}/{DECOMPOSER}/{TS_LEAF}";',
        ])
    ts_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_report(results: list[dict], checks: dict) -> Path:
    lines = ["# W6 Heavy Report", "", "Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`", ""]
    required = [
        BUILDER,
        DECOMPOSER,
        f"🧬️schema/📸️snapshot/{TEXT}",
        f"🧬️schema/📸️snapshot/{BINARY}",
        f"🚪️io/📥️import/{DESER}/🗿️artifacts",
        f"🚪️io/📤️export/{SER}/🗿️artifacts",
    ]
    olds = ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr"]
    for r in results:
        art = Path(r["path"])
        lines.append(f"## {r['plugin']} / {r['artifact']} (`{r['rust_mod']}`)")
        lines.append("")
        crate = r["crate"]
        ck = checks.get(crate, {})
        mark = "✅ green" if ck.get("ok") else "❌ FAIL"
        lines.append(f"- **cargo check `-p {crate}`**: {mark}")
        if ck.get("tail"):
            last = ck["tail"].strip().splitlines()[-1] if ck["tail"].strip() else ""
            lines.append(f"  - last line: `{last}`")
        lines.append("- **Path.exists verification**:")
        for rel in required:
            lines.append(f"  - `{rel}`: `{(art / rel).exists()}`")
        for old in olds:
            lines.append(f"  - old `{old}` gone: `{not (art / old).exists()}` (`{(art / old).exists()}`)")
        root_mut = art / "🧬️mutations"
        lines.append(f"  - old root `🧬️mutations` gone: `{not root_mut.exists()}` (`{root_mut.exists()}`)")
        for slug in r["slugs"]:
            d = b1c.stdio_dir(slug)
            imp = art / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{d}/🦀️component.rs"
            exp = art / f"🚪️io/📤️export/{SER}/🗿️artifacts/{d}/🦀️component.rs"
            lines.append(f"  - io `{slug}` import/export: `{imp.exists()}` / `{exp.exists()}`")
        if r["errors"]:
            lines.append(f"- **verify errors**: `{r['errors']}`")
        lines.append("")
    lines.append("## Cargo summary")
    lines.append("")
    for crate, ck in checks.items():
        lines.append(f"- `{crate}`: {'✅' if ck['ok'] else '❌'}")
    report = TICKET / "🧪w6-heavy-report.md"
    report.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return report


def main() -> int:
    glue_only = "--glue-only" in sys.argv
    results = []
    by_plugin: dict[str, list[dict]] = defaultdict(list)
    for entry in BATCH:
        by_plugin[entry["plugin"]].append(entry)
        if glue_only:
            row = owner_row(entry["plugin"], entry["artifact"])
            art = ROOT / row["path"]
            results.append({
                "plugin": entry["plugin"],
                "artifact": entry["artifact"],
                "rust_mod": entry["rust_mod"],
                "crate": entry["crate"],
                "errors": b1c.verify_tree(art),
                "slugs": entry.get("stdio") or row.get("import") or [],
                "path": str(art),
            })
            continue
        print("migrate", entry["plugin"], entry["artifact"], entry["rust_mod"], flush=True)
        results.append(migrate_one(entry))

    for plugin, entries in by_plugin.items():
        print("patch glue/ts", plugin, len(entries), flush=True)
        patch_plugin_glue(plugin, entries)
        patch_plugin_ts(plugin, entries)

    (TICKET / "generators/w6-heavy-migrate-report.json").write_text(
        json.dumps(results, indent=2, ensure_ascii=False), encoding="utf-8"
    )

    checks = {}
    crates = []
    for entry in BATCH:
        if entry["crate"] not in crates:
            crates.append(entry["crate"])
    for crate in crates:
        print("cargo check", crate, flush=True)
        r = subprocess.run(
            ["cargo", "check", "-p", crate],
            cwd=str(ROOT),
            capture_output=True,
            text=True,
        )
        tail = (r.stdout or "") + (r.stderr or "")
        ok = r.returncode == 0
        checks[crate] = {"ok": ok, "tail": tail[-12000:]}
        (TICKET / f"🧪w6-heavy-{crate}.log").write_text(tail, encoding="utf-8")
        print(" ->", "OK" if ok else "FAIL", flush=True)

    (TICKET / "generators/w6-heavy-cargo.json").write_text(
        json.dumps({k: {"ok": v["ok"], "tail": v["tail"][-2000:]} for k, v in checks.items()}, indent=2),
        encoding="utf-8",
    )
    report = write_report(results, checks)
    print("wrote", report)
    print(json.dumps({k: v["ok"] for k, v in checks.items()}, indent=2))

    return 0 if all(v["ok"] for v in checks.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
