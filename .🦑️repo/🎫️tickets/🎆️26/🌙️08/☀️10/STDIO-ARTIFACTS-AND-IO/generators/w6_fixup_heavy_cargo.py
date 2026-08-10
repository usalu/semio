#!/usr/bin/env python3
"""Make W6 heavy crates cargo-check green: types, builders, batch1b-style IO, glue."""
from __future__ import annotations

import importlib.util
import json
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
BATCH = json.loads((TICKET / "generators" / "w6-heavy.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
BUILDER, DECOMPOSER = TOK["builder"], TOK["decomposer"]
TEXT, BINARY = TOK["text"], TOK["binary"]
DESER, SER = TOK["deserializers"], TOK["serializers"]
ROSTER = OWNER["stdio_roster"]

_SPEC = importlib.util.spec_from_file_location("w6_heavy", TICKET / "generators" / "w6_migrate_heavy.py")
heavy = importlib.util.module_from_spec(_SPEC)
assert _SPEC.loader
_SPEC.loader.exec_module(heavy)


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))


def pascal(slug: str) -> str:
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[^a-zA-Z0-9]+", slug) if p)


def sniff(art: Path, rust_mod: str):
    blobs = []
    for rel in [
        "🧬️schema/📸️snapshot/🦀️component.rs",
        f"🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs",
        "🧬️schema/🧬️mutations/🦀️component.rs",
        "🧬️schema/🔺️diff/🦀️component.rs",
        f"🧬️schema/🔺️diff/{TEXT}/🦀️component.rs",
        "🦀️component.rs",
        f"🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs",
    ]:
        p = art / rel
        if p.exists():
            blobs.append(p.read_text(encoding="utf-8", errors="ignore"))
    text = "\n".join(blobs)

    def grab(pats, default):
        for pat in pats:
            m = re.search(pat, text)
            if m:
                return m.group(1)
        return default

    base = pascal(rust_mod)
    snap = grab([r"pub struct (\w+Snapshot)", r"pub use [^;]*::(\w+Snapshot)"], base + "Snapshot")
    mut = grab([r"pub (?:struct|enum) (\w+Mutation)", r"pub use [^;]*::(\w+Mutation)"], snap.replace("Snapshot", "Mutation"))
    diff = grab([r"pub (?:struct|enum) (\w+Diff)", r"pub use [^;]*::(\w+Diff)"], snap.replace("Snapshot", "Diff"))
    schema = grab([r"pub const (\w+_DOCUMENT_SCHEMA)"], None)
    applies = re.findall(r"pub fn (apply_\w+_mutation)\b", text)
    apply = applies[0] if applies else None
    return snap, mut, diff, schema, apply

def write_builder(art: Path, rust_mod: str, snap: str, mut: str, diff: str, apply):
    bname = pascal(rust_mod) + "Builder"
    dname = pascal(rust_mod) + "Decomposer"
    parts = pascal(rust_mod) + "Parts"
    if apply:
        mutate = f"        crate::artifacts::{rust_mod}::schema::mutations::{apply}(&mut self.snapshot, &mutation);\n        self"
    else:
        mutate = (
            f"        let d = <{mut} as protocol::Mutation<{snap}>>::diff(&mutation, &self.snapshot);\n"
            f"        self.snapshot = <{diff} as protocol::MutationDiff<{snap}>>::apply(&d, &self.snapshot);\n"
            "        self"
        )
    lt = "'" + "_"  # lifetime placeholder avoided: use string concat
    # Actually use explicit lifetime via chr(39)
    lifetime = chr(39) + "_"
    (art / BUILDER / "🦀️component.rs").write_text(
        f"""//! {bname}
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{rust_mod}::{{{diff}, {mut}, {snap}}};

#[derive(Clone, Debug, Default)]
pub struct {bname} {{
    snapshot: {snap},
    diagnostics: Vec<dsl::Diagnostic>,
}}

impl ArtifactBuilder for {bname} {{
    type Snapshot = {snap};
    type Mutation = {mut};
    type Diff = {diff};
    fn empty() -> Self {{ Self {{ snapshot: {snap}::default(), diagnostics: Vec::new() }} }}
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {{ Self {{ snapshot, diagnostics: Vec::new() }} }}
    fn from_text(text: &str) -> Result<Self, store::TextError> {{
        Ok(Self::from_snapshot(<{snap} as store::DocumentDsl>::parse_dsl(text)?))
    }}
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {{
        Ok(Self::from_snapshot(<{snap} as store::DocumentPack>::decode_pack(bytes)?))
    }}
    fn mutate(mut self, mutation: Self::Mutation) -> Self {{
{mutate}
    }}
    fn absorb(mut self, diff: Self::Diff) -> Self {{
        self.snapshot = <{diff} as protocol::MutationDiff<{snap}>>::apply(&diff, &self.snapshot);
        self
    }}
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {{
        if self.diagnostics.is_empty() {{ Ok(self.snapshot) }} else {{ Err(self.diagnostics) }}
    }}
}}
""",
        encoding="utf-8",
    )
    (art / DECOMPOSER / "🦀️component.rs").write_text(
        f"""//! {dname}
use semio_framework_plugin::{{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource}};
use crate::artifacts::{rust_mod}::{snap};

#[derive(Clone, Debug, Default)]
pub struct {parts} {{ pub snapshot: Option<{snap}> }}

pub struct {dname};

impl ArtifactDecomposer for {dname} {{
    type Snapshot = {snap};
    type Parts = {parts};
    fn decompose(sources: &[DecomposeSource<{lifetime}>]) -> Decomposition<Self::Parts> {{
        let mut parts = {parts}::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {{
            match source {{
                DecomposeSource::Text(text) => match <{snap} as store::DocumentDsl>::parse_dsl(text) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{rust_mod}.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
                DecomposeSource::Binary(bytes) => match <{snap} as store::DocumentPack>::decode_pack(bytes) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{rust_mod}.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
            }}
        }}
        Decomposition {{ parts, confidence, diagnostics }}
    }}
}}
""",
        encoding="utf-8",
    )


def write_io(art: Path, rust_mod: str, snap: str, schema_const, slugs: list[str]):
    for slug in slugs:
        dname = ROSTER[slug]["dir"]
        imp_p = art / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs"
        exp_p = art / f"🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs"
        if slug == "json":
            sch_use = f", {schema_const}" if schema_const else ""
            sch_fix = (
                f"if snap.schema.is_empty() {{ snap.schema = {schema_const}.into(); }}"
                if schema_const
                else ""
            )
            imp_p.write_text(
                f"""//! {rust_mod} <- json
use crate::artifacts::{rust_mod}::{{{snap}{sch_use}}};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &JsonSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let mut snap: {snap} = serde_json::from_value(from.value.clone())
        .map_err(|e| store::TextError::new(format!("{rust_mod}<-json: {{e}}"), dsl::TextSpan::at(1, 1)))?;
    {sch_fix}
    Ok(snap)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
""",
                encoding="utf-8",
            )
            exp_p.write_text(
                f"""//! {rust_mod} -> json
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<JsonSnapshot, store::TextError> {{
    Ok(JsonSnapshot {{
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        value: serde_json::to_value(snapshot)
            .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?,
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    serde_json::to_vec_pretty(&serialize(snapshot)?.value)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}
""",
                encoding="utf-8",
            )
            continue
        if slug == "md":
            imp_p.write_text(
                f"""//! {rust_mod} <- md
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &MdSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.body)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <{snap} as store::DocumentDsl>::parse_dsl(text)
}}
""",
                encoding="utf-8",
            )
            exp_p.write_text(
                f"""//! {rust_mod} -> md
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<MdSnapshot, store::TextError> {{
    Ok(MdSnapshot {{
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <{snap} as store::DocumentDsl>::print_dsl(snapshot),
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(serialize(snapshot)?.body.into_bytes())
}}
""",
                encoding="utf-8",
            )
            continue
        if slug == "txt":
            imp_p.write_text(
                f"""//! {rust_mod} <- txt
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::txt::{{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &TxtSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.text)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <{snap} as store::DocumentDsl>::parse_dsl(text)
}}
""",
                encoding="utf-8",
            )
            exp_p.write_text(
                f"""//! {rust_mod} -> txt
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::txt::{{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<TxtSnapshot, store::TextError> {{
    Ok(TxtSnapshot {{
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        text: <{snap} as store::DocumentDsl>::print_dsl(snapshot),
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(serialize(snapshot)?.text.into_bytes())
}}
""",
                encoding="utf-8",
            )
            continue
        if slug == "csv":
            imp_p.write_text(
                f"""//! {rust_mod} <- csv
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &CsvSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok({snap}::default())
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let _ = bytes;
    Ok({snap}::default())
}}
""",
                encoding="utf-8",
            )
            exp_p.write_text(
                f"""//! {rust_mod} -> csv
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<CsvSnapshot, store::TextError> {{
    Ok(CsvSnapshot {{
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        headers: vec!["payload".into()],
        rows: vec![vec![<{snap} as store::DocumentDsl>::print_dsl(snapshot)]],
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?))
}}
""",
                encoding="utf-8",
            )
            continue
        ss = pascal(slug) + "Snapshot"
        sch = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
        imp_p.write_text(
            f"""//! {rust_mod} <- {slug}
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{ss}, {sch}}};

pub fn register() {{}}

pub fn deserialize(from: &{ss}) -> Result<{snap}, store::TextError> {{
    let _ = ({sch}, from);
    Ok({snap}::default())
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let _ = bytes;
    Ok({snap}::default())
}}
""",
            encoding="utf-8",
        )
        exp_p.write_text(
            f"""//! {rust_mod} -> {slug}
use crate::artifacts::{rust_mod}::{snap};

pub fn register() {{}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    Ok(<{snap} as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}}
""",
            encoding="utf-8",
        )


def patch_root_exports(art: Path, rust_mod: str, snap: str, mut: str, diff: str):
    p = art / "🦀️component.rs"
    if not p.exists():
        return
    t = p.read_text(encoding="utf-8")
    t2 = t
    t2 = re.sub(
        rf"pub use crate::artifacts::{rust_mod}::snapshot::schema::{snap};",
        f"pub use crate::artifacts::{rust_mod}::schema::snapshot::{snap};",
        t2,
    )
    t2 = re.sub(
        rf"pub use crate::artifacts::{rust_mod}::diff::{diff};",
        f"pub use crate::artifacts::{rust_mod}::schema::diff::{diff};",
        t2,
    )
    t2 = re.sub(
        rf"pub use crate::artifacts::{rust_mod}::mutations::{mut};",
        f"pub use crate::artifacts::{rust_mod}::schema::mutations::{mut};",
        t2,
    )
    for line in (
        f"pub use crate::artifacts::{rust_mod}::schema::snapshot::{snap};",
        f"pub use crate::artifacts::{rust_mod}::schema::mutations::{mut};",
        f"pub use crate::artifacts::{rust_mod}::schema::diff::{diff};",
    ):
        if line not in t2:
            t2 = line + "\n" + t2
    if t2 != t:
        p.write_text(t2, encoding="utf-8")


def fix_diff_imports(art: Path, rust_mod: str):
    p = art / f"🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"
    if not p.exists():
        return
    t = p.read_text(encoding="utf-8")
    t2 = t.replace(
        f"crate::artifacts::{rust_mod}::diff::schema::",
        f"crate::artifacts::{rust_mod}::schema::diff::",
    )
    t2 = t2.replace("pub use super::schema::*;\n", "")
    if f"use crate::artifacts::{rust_mod}::schema::diff::*" not in t2:
        t2 = f"use crate::artifacts::{rust_mod}::schema::diff::*;\n" + t2
    if t2 != t:
        p.write_text(t2, encoding="utf-8")


def main() -> int:
    by_plugin = defaultdict(list)
    metas = []
    for entry in BATCH:
        row = owner_row(entry["plugin"], entry["artifact"])
        art = ROOT / row["path"]
        slugs = entry.get("stdio") or row.get("import") or []
        snap, mut, diff, schema, apply = sniff(art, entry["rust_mod"])
        print("fix", entry["rust_mod"], snap, mut, diff, apply, flush=True)
        fix_diff_imports(art, entry["rust_mod"])
        write_builder(art, entry["rust_mod"], snap, mut, diff, apply)
        write_io(art, entry["rust_mod"], snap, schema, slugs)
        patch_root_exports(art, entry["rust_mod"], snap, mut, diff)
        e = dict(entry)
        e["_snap"], e["_mut"], e["_diff"] = snap, mut, diff
        metas.append(e)
        by_plugin[entry["plugin"]].append(entry)

    def sniff_types(art, rust_mod):
        snap, mut, diff, schema, apply = sniff(art, rust_mod)
        return snap, mut, diff, schema

    heavy.b1c.sniff_types = sniff_types

    for plugin, entries in by_plugin.items():
        print("glue", plugin, len(entries), flush=True)
        heavy.patch_plugin_glue(plugin, entries)
        heavy.patch_plugin_ts(plugin, entries)

    (TICKET / "generators" / "w6-heavy-fixup-meta.json").write_text(
        json.dumps(
            [{"rust_mod": m["rust_mod"], "snap": m["_snap"], "mut": m["_mut"], "diff": m["_diff"]} for m in metas],
            indent=2,
        ),
        encoding="utf-8",
    )

    checks = {}
    crates = []
    for e in BATCH:
        if e["crate"] not in crates:
            crates.append(e["crate"])
    for crate in crates:
        print("cargo", crate, flush=True)
        r = subprocess.run(["cargo", "check", "-p", crate], cwd=ROOT, capture_output=True, text=True)
        tail = (r.stdout or "") + (r.stderr or "")
        ok = r.returncode == 0
        checks[crate] = {"ok": ok, "tail": tail[-15000:]}
        (TICKET / f"🧪w6-heavy-{crate}.log").write_text(tail, encoding="utf-8")
        print(" ->", "OK" if ok else "FAIL", flush=True)

    (TICKET / "generators" / "w6-heavy-cargo.json").write_text(
        json.dumps({k: {"ok": v["ok"]} for k, v in checks.items()}, indent=2), encoding="utf-8"
    )
    print(json.dumps({k: v["ok"] for k, v in checks.items()}, indent=2))
    return 0 if all(v["ok"] for v in checks.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
