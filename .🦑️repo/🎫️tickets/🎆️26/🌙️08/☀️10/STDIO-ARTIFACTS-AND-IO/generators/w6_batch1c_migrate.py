#!/usr/bin/env python3
"""W6 batch1c: stdio facet absorb + glue/io/builder for 12 plugins."""
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parents[1]
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
ROSTER = OWNER["stdio_roster"]
BATCH = json.loads((TICKET / "generators" / "w6-batch1c.json").read_text(encoding="utf-8"))
REF_NOTE = ROOT / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note"
REF_STDIO = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json"

BUILDER = TOK["builder"]
DECOMPOSER = TOK["decomposer"]
TEXT = TOK["text"]
BINARY = TOK["binary"]
DESER = TOK["deserializers"]
SER = TOK["serializers"]

TEXT_FORMATS = {"json", "csv", "md", "txt", "xml", "svg", "obj", "stl", "step", "ifc", "dxf"}
BINARY_WIRE = {"dwg", "glb", "gltf", "png", "jpg", "gif", "bmp", "tiff", "pdf", "las", "ply", "zip", "xlsx", "pptx", "docx", "bcf"}


def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))


def absorb(art: Path) -> None:
    sys.path.insert(0, str(TICKET / "generators"))
    import w5_migrate_artifact as w5

    w5.absorb(art)


def copy_tree_missing(src: Path, dst: Path) -> None:
    if not src.exists():
        return
    for f in src.rglob("*"):
        if f.is_file():
            rel = f.relative_to(src)
            out = dst / rel
            out.parent.mkdir(parents=True, exist_ok=True)
            if not out.exists():
                shutil.copy2(f, out)


def scaffold_leaves(art: Path) -> None:
    note_snap_t = REF_NOTE / "🧬️schema/📸️snapshot" / TEXT
    note_snap_b = REF_NOTE / "🧬️schema/📸️snapshot" / BINARY
    note_diff_t = REF_NOTE / "🧬️schema/🔺️diff" / TEXT
    note_diff_b = REF_NOTE / "🧬️schema/🔺️diff" / BINARY
    note_mut_t = REF_NOTE / "🧬️schema/🧬️mutations" / TEXT
    note_mut_b = REF_NOTE / "🧬️schema/🧬️mutations" / BINARY
    for src, rel in [
        (note_snap_t, f"🧬️schema/📸️snapshot/{TEXT}"),
        (note_snap_b, f"🧬️schema/📸️snapshot/{BINARY}"),
        (note_diff_t, f"🧬️schema/🔺️diff/{TEXT}"),
        (note_diff_b, f"🧬️schema/🔺️diff/{BINARY}"),
        (note_mut_t, f"🧬️schema/🧬️mutations/{TEXT}"),
        (note_mut_b, f"🧬️schema/🧬️mutations/{BINARY}"),
        (REF_NOTE / BUILDER, BUILDER),
        (REF_NOTE / DECOMPOSER, DECOMPOSER),
    ]:
        copy_tree_missing(src, art / rel)
    ref_b = REF_STDIO / BUILDER
    ref_d = REF_STDIO / DECOMPOSER
    copy_tree_missing(ref_b, art / BUILDER)
    copy_tree_missing(ref_d, art / DECOMPOSER)


def pascal(slug: str) -> str:
    parts = re.split(r"[^a-zA-Z0-9]+", slug)
    return "".join(p[:1].upper() + p[1:] for p in parts if p)


def sniff_types(art: Path, rust_mod: str) -> tuple[str, str, str, str]:
    root = art / "🦀️component.rs"
    text = root.read_text(encoding="utf-8") if root.exists() else ""
    m = re.search(r"pub struct (\w+Snapshot)", text)
    snap = m.group(1) if m else pascal(rust_mod) + "Snapshot"
    m = re.search(r"pub struct (\w+Mutation)", text)
    mut = m.group(1) if m else pascal(rust_mod) + "Mutation"
    m = re.search(r"pub struct (\w+Diff)", text)
    diff = m.group(1) if m else pascal(rust_mod) + "Diff"
    m = re.search(r"pub const (\w+_DOCUMENT_SCHEMA)", text)
    schema = m.group(1) if m else f"{rust_mod.upper()}_DOCUMENT_SCHEMA"
    return snap, mut, diff, schema


def mutate_body(rust_mod: str, mut: str, snap: str, diff: str) -> str:
    mut_path = f"crate::artifacts::{rust_mod}::schema::mutations"
    for line in [
        f"pub fn apply_{rust_mod}_mutation",
        f"apply_{rust_mod}_mutation",
        f"{mut_path}::apply_{rust_mod}_mutation",
    ]:
        pass
    art_mut = Path(ROOT / "✏️s/🔌️plugins")  # dummy
    return f"""        let d = <{mut} as protocol::Mutation<{snap}>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);"""


def write_builder(art: Path, rust_mod: str, snap: str, mut: str, diff: str) -> None:
    bname = pascal(rust_mod) + "Builder"
    dname = pascal(rust_mod) + "Decomposer"
    parts = pascal(rust_mod) + "Parts"
    mut_block = mutate_body(rust_mod, mut, snap, diff)
    builder = f"""//! {bname}
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{rust_mod}::schema::diff::{diff};
use crate::artifacts::{rust_mod}::schema::mutations::{mut};
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};

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
{mut_block}
        self
    }}
    fn absorb(mut self, diff: Self::Diff) -> Self {{
        self.snapshot = <{diff} as protocol::MutationDiff<{snap}>>::apply(&diff, &self.snapshot);
        self
    }}
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {{
        if self.diagnostics.is_empty() {{ Ok(self.snapshot) }} else {{ Err(self.diagnostics) }}
    }}
}}
"""
    decomposer = f"""//! {dname}
use semio_framework_plugin::{{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource}};
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};

#[derive(Clone, Debug, Default)]
pub struct {parts} {{ pub snapshot: Option<{snap}> }}

pub struct {dname};

impl ArtifactDecomposer for {dname} {{
    type Snapshot = {snap};
    type Parts = {parts};
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {{
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
"""
    (art / BUILDER / "🦀️component.rs").write_text(builder, encoding="utf-8")
    (art / DECOMPOSER / "🦀️component.rs").write_text(decomposer, encoding="utf-8")
    for leaf in ("🟦️component.ts",):
        for facet in (BUILDER, DECOMPOSER):
            p = art / facet / leaf
            if not p.exists():
                p.write_text("export {};\n", encoding="utf-8")


def stdio_dir(slug: str) -> str:
    return ROSTER[slug]["dir"]


def io_import_rs(rust_mod: str, snap: str, schema_const: str, slug: str) -> str:
    mid = slug.replace("-", "_")
    stdio_mod = slug if slug[0].isalpha() else slug
    stdio_snap = pascal(slug) + "Snapshot"
    stdio_schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
    err = "store::TextError::new"
    if slug == "json":
        return f"""//! {rust_mod} <- json
use crate::artifacts::{rust_mod}::{{{snap}, {schema_const}}};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &JsonSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let out: {snap} = serde_json::from_value(from.value.clone())
        .map_err(|e| {err}(format!("{rust_mod}<-json: {{e}}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    let text = std::str::from_utf8(bytes).map_err(|e| {err}(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| {err}(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
"""
    if slug == "csv":
        return f"""//! {rust_mod} <- csv
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &CsvSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = serde_json::json!({{ "headers": from.headers, "rows": from.rows }});
    serde_json::from_value(value).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    deserialize(&<CsvSnapshot as store::DocumentPack>::decode_pack(bytes)?)
}}
"""
    if slug == "md":
        return f"""//! {rust_mod} <- md
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &MdSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.body)
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    deserialize(&<MdSnapshot as store::DocumentPack>::decode_pack(bytes)?)
}}
"""
    if slug in TEXT_FORMATS:
        return f"""//! {rust_mod} <- {slug}
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn deserialize(from: &{stdio_snap}) -> Result<{snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = semio_s_plugin_stdio::artifacts::{slug}::engine::encode_{slug}(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <{snap} as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <{snap} as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    deserialize(&semio_s_plugin_stdio::artifacts::{slug}::engine::decode_{slug}(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}}
"""
    return f"""//! {rust_mod} <- {slug}
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn deserialize(from: &{stdio_snap}) -> Result<{snap}, store::TextError> {{
    let _ = {stdio_schema};
    let bytes = semio_s_plugin_stdio::artifacts::{slug}::engine::encode_{slug}(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <{snap} as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <{snap} as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap}, store::TextError> {{
    deserialize(&semio_s_plugin_stdio::artifacts::{slug}::engine::decode_{slug}(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}}
"""


def io_export_rs(rust_mod: str, snap: str, slug: str) -> str:
    stdio_snap = pascal(slug) + "Snapshot"
    stdio_schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
    if slug == "json":
        return f"""//! {rust_mod} -> json
use crate::artifacts::{rust_mod}::{snap};
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
    if slug == "csv":
        return f"""//! {rust_mod} -> csv
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<CsvSnapshot, store::TextError> {{
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let headers = value.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    let rows = value.get("rows").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Ok(CsvSnapshot {{ schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    <CsvSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?)
}}
"""
    if slug == "md":
        return f"""//! {rust_mod} -> md
use crate::artifacts::{rust_mod}::schema::snapshot::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<MdSnapshot, store::TextError> {{
    Ok(MdSnapshot {{
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <{snap} as store::DocumentDsl>::print_dsl(snapshot),
    }})
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    <MdSnapshot as store::DocumentPack>::encode_pack(&serialize(snapshot)?)
}}
"""
    return f"""//! {rust_mod} -> {slug}
use crate::artifacts::{rust_mod}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{stdio_snap}, {stdio_schema}}};

pub fn register() {{}}

pub fn serialize(snapshot: &{snap}) -> Result<{stdio_snap}, store::TextError> {{
    let bytes = <{snap} as store::DocumentPack>::encode_pack(snapshot)
        .or_else(|_| Ok(<{snap} as store::DocumentDsl>::print_dsl(snapshot).into_bytes()))?;
    semio_s_plugin_stdio::artifacts::{slug}::engine::decode_{slug}(&bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}}

pub fn serialize_bytes(snapshot: &{snap}) -> Result<Vec<u8>, store::TextError> {{
    semio_s_plugin_stdio::artifacts::{slug}::engine::encode_{slug}(&serialize(snapshot)?)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}}
"""


def write_io(art: Path, rust_mod: str, snap: str, schema_const: str, slugs: list[str]) -> None:
    imp_base = art / "🚪️io/📥️import" / DESER / "🗿️artifacts"
    exp_base = art / "🚪️io/📤️export" / SER / "🗿️artifacts"
    for slug in slugs:
        dname = stdio_dir(slug)
        imp_dir = imp_base / dname
        exp_dir = exp_base / dname
        imp_dir.mkdir(parents=True, exist_ok=True)
        exp_dir.mkdir(parents=True, exist_ok=True)
        (imp_dir / "🦀️component.rs").write_text(io_import_rs(rust_mod, snap, schema_const, slug), encoding="utf-8")
        (exp_dir / "🦀️component.rs").write_text(io_export_rs(rust_mod, snap, slug), encoding="utf-8")
        for leaf in ("🟦️component.ts",):
            for d in (imp_dir, exp_dir):
                p = d / leaf
                if not p.exists():
                    p.write_text("export {};\n", encoding="utf-8")
    kinds = ", ".join(f'"stdio.{s}"' for s in slugs)
    reg_imp = "\n    ".join(f"crate::artifacts::{rust_mod}::io::import::deserializers::artifacts::{s}::register();" for s in slugs)
    reg_exp = "\n    ".join(f"crate::artifacts::{rust_mod}::io::export::serializers::artifacts::{s}::register();" for s in slugs)
    io_root = f"""//! {rust_mod} IO stdio matrix
pub fn register() {{
    {reg_imp}
    {reg_exp}
}}
pub fn import_stdio_kinds() -> &'static [&'static str] {{
    &[{kinds}]
}}
pub fn export_stdio_kinds() -> &'static [&'static str] {{
    &[{kinds}]
}}
"""
    (art / "🚪️io/🦀️component.rs").write_text(io_root, encoding="utf-8")


def mut_mod_name(dirname: str) -> str:
    slug = "".join(c if c.isascii() and (c.isalnum() or c == "-") else "" for c in dirname)
    return slug.replace("-", "_")


def list_mutations(art: Path) -> list[str]:
    muts = art / "🧬️schema" / "🧬️mutations"
    if not muts.exists():
        return []
    return sorted(
        c.name
        for c in muts.iterdir()
        if c.is_dir() and c.name not in (TEXT, BINARY)
    )


def list_engine_children(art: Path) -> list[str]:
    eng = art / "⚙️engine"
    if not eng.exists():
        return []
    return sorted(
        c.name
        for c in eng.iterdir()
        if c.is_dir() and (c / "🦀️component.rs").exists()
    )


def glue_artifact_block(
    plugin: str,
    artifact: str,
    rust_mod: str,
    art_rel: str,
    slugs: list[str],
    extras: list[str],
    art: Path,
) -> str:
    mut_dirs = list_mutations(art)
    eng_children = list_engine_children(art)
    lines: list[str] = []
    a = f"../../🗿️artifacts/{artifact}"

    def L(s: str = "") -> None:
        lines.append(s)

    L(f"    pub mod {rust_mod} {{")
    L(f'        #[path = "{a}/🦀️component.rs"]')
    L("        mod component;")
    L("        pub use component::*;")
    extra_paths = {"kernel": "🧱️kernel", "registers": "🗄️registers"}
    for ex in extras:
        sub = extra_paths.get(ex, ex)
        L(f'        #[path = "{a}/🧬️schema/{sub}/🦀️component.rs"]')
        L(f"        pub mod {ex};")
    L('        #[path = "."]')
    L("        pub mod schema {")
    L(f'            #[path = "{a}/🧬️schema/🦀️component.rs"]')
    L("            mod component;")
    L("            pub use component::*;")
    L('            #[path = "."]')
    L("            pub mod snapshot {")
    L(f'                #[path = "{a}/🧬️schema/📸️snapshot/🦀️component.rs"]')
    L("                mod component;")
    L("                pub use component::*;")
    L(f'                #[path = "{a}/🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs"]')
    L("                pub mod text;")
    L(f'                #[path = "{a}/🧬️schema/📸️snapshot/{BINARY}/🦀️component.rs"]')
    L("                pub mod binary;")
    L("            }")
    L('            #[path = "."]')
    L("            pub mod diff {")
    L(f'                #[path = "{a}/🧬️schema/🔺️diff/🦀️component.rs"]')
    L("                mod component;")
    L("                pub use component::*;")
    L(f'                #[path = "{a}/🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"]')
    L("                pub mod text;")
    L(f'                #[path = "{a}/🧬️schema/🔺️diff/{BINARY}/🦀️component.rs"]')
    L("                pub mod binary;")
    L("            }")
    L('            #[path = "."]')
    L("            pub mod mutations {")
    L(f'                #[path = "{a}/🧬️schema/🧬️mutations/🦀️component.rs"]')
    L("                mod component;")
    L("                pub use component::*;")
    L(f'                #[path = "{a}/🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs"]')
    L("                pub mod text;")
    L(f'                #[path = "{a}/🧬️schema/🧬️mutations/{BINARY}/🦀️component.rs"]')
    L("                pub mod binary;")
    for d in mut_dirs:
        mod = mut_mod_name(d)
        base = f"{a}/🧬️schema/🧬️mutations/{d}"
        L('                #[path = "."]')
        L(f"                pub mod {mod} {{")
        L(f'                    #[path = "{base}/🦠️mutation/🦀️component.rs"]')
        L("                    pub mod mutation;")
        L(f'                    #[path = "{base}/🔺️diff/🦀️component.rs"]')
        L("                    pub mod diff;")
        L(f'                    #[path = "{base}/↩️inverse/🦀️component.rs"]')
        L("                    pub mod inverse;")
        L("                }")
    L("            }")
    L("        }")
    L(f'        pub mod op {{ pub use crate::artifacts::{rust_mod}::schema::mutations::text::*; pub use crate::artifacts::{rust_mod}::schema::mutations::{pascal(rust_mod)}Mutation; }}')
    L(f'        pub mod dsl {{ pub use crate::artifacts::{rust_mod}::schema::snapshot::text::*; }}')
    L(f'        pub mod spr {{ pub use crate::artifacts::{rust_mod}::schema::mutations::binary::*; }}')
    L(f'        pub mod diff {{ pub use crate::artifacts::{rust_mod}::schema::diff::*; pub use crate::artifacts::{rust_mod}::schema::diff::text::*; pub mod schema {{ pub use crate::artifacts::{rust_mod}::schema::diff::*; }} pub mod text {{ pub use crate::artifacts::{rust_mod}::schema::diff::text::*; }} }}')
    L(f'        pub mod mutations {{ pub use crate::artifacts::{rust_mod}::schema::mutations::*; }}')
    L(f'        pub mod snapshot {{ pub mod schema {{ pub use crate::artifacts::{rust_mod}::schema::snapshot::*; }} pub mod pack {{ pub use crate::artifacts::{rust_mod}::schema::snapshot::binary::*; }} }}')
    L(f'        #[path = "{a}/{BUILDER}/🦀️component.rs"]')
    L("        pub mod builder;")
    L(f'        #[path = "{a}/{DECOMPOSER}/🦀️component.rs"]')
    L("        pub mod decomposer;")
    L('        #[path = "."]')
    L("        pub mod io {")
    L(f'            #[path = "{a}/🚪️io/🦀️component.rs"]')
    L("            mod component;")
    L("            pub use component::*;")
    L('            #[path = "."]')
    L("            pub mod import {")
    L('                #[path = "."]')
    L("                pub mod deserializers {")
    L('                    #[path = "."]')
    L("                    pub mod artifacts {")
    for slug in slugs:
        dname = stdio_dir(slug)
        L('                        #[path = "."]')
        L(f"                        pub mod {slug} {{")
        L(f'                            #[path = "{a}/🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs"]')
        L("                            mod component;")
        L("                            pub use component::*;")
        L("                        }")
    L("                    }")
    L("                }")
    L("            }")
    L('            #[path = "."]')
    L("            pub mod export {")
    L('                #[path = "."]')
    L("                pub mod serializers {")
    L('                    #[path = "."]')
    L("                    pub mod artifacts {")
    for slug in slugs:
        dname = stdio_dir(slug)
        L('                        #[path = "."]')
        L(f"                        pub mod {slug} {{")
        L(f'                            #[path = "{a}/🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs"]')
        L("                            mod component;")
        L("                            pub use component::*;")
        L("                        }")
    L("                    }")
    L("                }")
    L("            }")
    for slug in slugs:
        L('            #[path = "."]')
        L(f"            pub mod {slug} {{")
        L('                #[path = "."]')
        L("                pub mod export {")
        L(f"                    pub use crate::artifacts::{rust_mod}::io::export::serializers::artifacts::{slug}::*;")
        L("                }")
        L('                #[path = "."]')
        L("                pub mod import {")
        L(f"                    pub use crate::artifacts::{rust_mod}::io::import::deserializers::artifacts::{slug}::*;")
        L("                }")
        L("            }")
    L("        }")
    if (art / "⚙️engine/🦀️component.rs").exists():
        L('        #[path = "."]')
        L("        pub mod engine {")
        L(f'            #[path = "{a}/⚙️engine/🦀️component.rs"]')
        L("            mod component;")
        L("            pub use component::*;")
        for ch in eng_children:
            mod = mut_mod_name(ch)
            L(f'            #[path = "{a}/⚙️engine/{ch}/🦀️component.rs"]')
            L(f"            pub mod {mod};")
        L("        }")
    L("    }")
    return "\n".join(lines) + "\n"


def patch_glue(plugin_path: Path, artifact: str, rust_mod: str, slugs: list[str], extras: list[str], art: Path) -> None:
    glue = plugin_path / "📦️packages/🦀️rust/📦️glue.rs"
    text = glue.read_text(encoding="utf-8")
    start = text.find("//#region 🗿️Artifacts")
    end = text.find("//#endregion 🗿️Artifacts")
    if start < 0 or end < 0:
        raise RuntimeError(f"glue region missing: {glue}")
    head = text[: start + len("//#region 🗿️Artifacts\n")]
    tail = text[end:]
    block = glue_artifact_block("", artifact, rust_mod, artifact, slugs, extras, art)
    new_region = head + "#[path = \".\"]\npub mod artifacts {\n    #[path = \".\"]\n" + block + "}\n" + tail
    glue.write_text(new_region, encoding="utf-8")


def patch_ts_barrel(plugin_path: Path, artifact: str, rust_mod: str) -> None:
    barrel = plugin_path / "📦️packages/🟦️typescript/📦️index.ts"
    if not barrel.exists():
        return
    ts = barrel.read_text(encoding="utf-8")
    prefix = f"{rust_mod}_"
    lines = [
        ln
        for ln in ts.splitlines()
        if not ln.strip().startswith(f"export * as {prefix}")
        and f"/🗿️artifacts/{artifact}/" not in ln
    ]
    a = f"../../🗿️artifacts/{artifact}"
    add = [
        f'export * as {prefix}schema from "{a}/🧬️schema/🟦️component.ts";',
        f'export * as {prefix}builder from "{a}/{BUILDER}/🟦️component.ts";',
        f'export * as {prefix}decomposer from "{a}/{DECOMPOSER}/🟦️component.ts";',
        f'export * as {prefix}io from "{a}/🚪️io/🟦️component.ts";',
    ]
    barrel.write_text("\n".join(lines + add) + "\n", encoding="utf-8")


def ensure_cargo_dep(plugin_path: Path) -> None:
    cargo = plugin_path / "📦️packages/🦀️rust/Cargo.toml"
    t = cargo.read_text(encoding="utf-8")
    if "semio-s-plugin-stdio" not in t:
        t = t.replace(
            "[dependencies]\n",
            '[dependencies]\nsemio-s-plugin-stdio = { path = "../../../🗄️stdio/📦️packages/🦀️rust", package = "semio-s-plugin-stdio" }\n',
            1,
        )
        cargo.write_text(t, encoding="utf-8")


def fix_schema_includes(art: Path) -> None:
    schema_rs = art / "🧬️schema/🦀️component.rs"
    if not schema_rs.exists():
        return
    t = schema_rs.read_text(encoding="utf-8")
    snap = art / "🧬️schema/📸️snapshot"
    diff = art / "🧬️schema/🔺️diff"
    t2 = re.sub(
        r'include_str!\("\.\./[^"]*snapshot[^"]*schema/([^"]+)"\)',
        lambda m: f'include_str!("📸️snapshot/{m.group(1)}")',
        t,
    )
    t2 = re.sub(
        r'include_str!\("\.\./[^"]*diff[^"]*schema/([^"]+)"\)',
        lambda m: f'include_str!("🔺️diff/{m.group(1)}")',
        t2,
    )
    if t2 != t:
        schema_rs.write_text(t2, encoding="utf-8")
    text_rs = snap / TEXT / "🦀️component.rs"
    if text_rs.exists():
        tt = text_rs.read_text(encoding="utf-8")
        ex = art / "📚️examples"
        if ex.exists():
            for dsl in ex.rglob("*example.dsl*"):
                rel = Path(os_relpath(art, dsl))
                tt2 = re.sub(
                    r'include_str!\("([^"]*example\.dsl[^"]*)"\)',
                    f'include_str!("{rel.as_posix()}")',
                    tt,
                )
                if tt2 != tt:
                    text_rs.write_text(tt2, encoding="utf-8")
                break


def os_relpath(base: Path, target: Path) -> str:
    return Path(os.path.relpath(target, base)).as_posix()


def patch_artifact_kind(art: Path, slugs: list[str]) -> None:
    p = art / "🦀️component.rs"
    t = p.read_text(encoding="utf-8")
    t = re.sub(r",\s*MediaFormat::\w+", "", t)
    t = re.sub(r"MediaFormat,?\s*", "", t)
    t = re.sub(r"DocumentCodec,?\s*", "", t)
    t = re.sub(r"JsonCodec,?\s*", "", t)
    t = re.sub(r"IoFormatSpec[^;]*;", "", t)
    kinds = ", ".join(f'"stdio.{s}"' for s in slugs)
    t = re.sub(
        r"export_stdio_kinds:\s*vec!\[[^\]]*\]",
        f"export_stdio_kinds: vec![{kinds}]",
        t,
    )
    t = re.sub(
        r"import_stdio_kinds:\s*vec!\[[^\]]*\]",
        f"import_stdio_kinds: vec![{kinds}]",
        t,
    )
    t = re.sub(r"export_formats:\s*vec!\[[^\]]*\]", "export_formats: vec![]", t)
    t = re.sub(r"import_formats:\s*vec!\[[^\]]*\]", "import_formats: vec![]", t)
    p.write_text(t, encoding="utf-8")


def verify_tree(art: Path) -> list[str]:
    errs = []
    for rel in [
        BUILDER,
        DECOMPOSER,
        f"🧬️schema/📸️snapshot/{TEXT}",
        f"🧬️schema/📸️snapshot/{BINARY}",
        f"🚪️io/📥️import/{DESER}/🗿️artifacts",
        f"🚪️io/📤️export/{SER}/🗿️artifacts",
    ]:
        if not (art / rel).exists():
            errs.append(f"missing {rel}")
    for old in ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr"]:
        if (art / old).exists():
            errs.append(f"old facet {old}")
    if (art / "🧬️mutations").exists():
        errs.append("root mutations")
    return errs


def migrate_one(entry: dict) -> dict:
    row = owner_row(entry["plugin"], entry["artifact"])
    art = ROOT / row["path"]
    plugin_path = art.parent.parent
    slugs = row.get("import") or row.get("stdio_artifacts") or []
    absorb(art)
    scaffold_leaves(art)
    snap, mut, diff, schema_const = sniff_types(art, entry["rust_mod"])
    write_builder(art, entry["rust_mod"], snap, mut, diff)
    write_io(art, entry["rust_mod"], snap, schema_const, slugs)
    fix_schema_includes(art)
    patch_glue(plugin_path, entry["artifact"], entry["rust_mod"], slugs, entry.get("extras") or [], art)
    patch_ts_barrel(plugin_path, entry["artifact"], entry["rust_mod"])
    ensure_cargo_dep(plugin_path)
    patch_artifact_kind(art, slugs)
    errs = verify_tree(art)
    return {"plugin": entry["plugin"], "artifact": entry["artifact"], "crate": entry["crate"], "errors": errs, "slugs": slugs}


def cargo_check(crate: str) -> tuple[bool, str]:
    log = TICKET / f"🧪w6-batch1c-{crate}.log"
    r = subprocess.run(
        ["cargo", "check", "-p", crate],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    log.write_text(r.stdout + r.stderr, encoding="utf-8")
    return r.returncode == 0, r.stdout.splitlines()[-3:] if r.stdout else []


def main() -> None:
    results = []
    checks = {}
    for entry in BATCH:
        print("migrate", entry["plugin"], entry["artifact"])
        results.append(migrate_one(entry))
    (TICKET / "generators/w6-batch1c-migrate-report.json").write_text(
        json.dumps(results, indent=2), encoding="utf-8"
    )
    for entry in BATCH:
        ok, tail = cargo_check(entry["crate"])
        checks[entry["crate"]] = {"ok": ok, "tail": tail}
    (TICKET / "generators/w6-batch1c-cargo.json").write_text(json.dumps(checks, indent=2), encoding="utf-8")
    print(json.dumps(checks, indent=2))
    sys.exit(0 if all(v["ok"] for v in checks.values()) else 1)


if __name__ == "__main__":
    main()
