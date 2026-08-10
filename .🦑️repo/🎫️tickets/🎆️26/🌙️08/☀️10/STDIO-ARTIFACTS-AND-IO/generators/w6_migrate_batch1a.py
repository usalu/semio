#!/usr/bin/env python3
"""W6 batch1a: writer, mathematical, flow, vcs, dag — stdio artifact absorb + glue."""
from __future__ import annotations

import importlib.util
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))["stdio_roster"]
PLUGINS_ROOT = ROOT / "✏️s/🔌️plugins"
REF = PLUGINS_ROOT / TOK["stdio_plugin"] / "🗿️artifacts" / ROSTER["json"]["dir"]
NOTE_ART = PLUGINS_ROOT / "🗒️note/🗿️artifacts/🗒️note"

BUILDER = TOK["builder"]
DECOMPOSER = TOK["decomposer"]
TEXT = TOK["text"]
BINARY = TOK["binary"]
DESER = TOK["deserializers"]
SER = TOK["serializers"]
TS_LEAF = "🟦️component.ts"
RS_LEAF = "🦀️component.rs"

BATCH = [
    {
        "plugin": "✒️writer",
        "artifact": "✒️writer",
        "mod": "writer",
        "prefix": "Writer",
        "crate": "semio-s-plugin-writer",
        "schema_const": "WRITER_DOCUMENT_SCHEMA",
        "stdio": ["docx", "json", "md", "pdf", "txt"],
        "mutation": "WriterMutation",
        "diff": "WriterDiff",
        "apply": "apply_writer_mutation",
        "mut_mut": True,
        "op_reexport": "WriterMutation",
    },
    {
        "plugin": "➗️mathematical",
        "artifact": "➗️mathematical",
        "mod": "mathematical",
        "prefix": "Mathematical",
        "crate": "semio-s-plugin-mathematical",
        "schema_const": "MATH_DOCUMENT_SCHEMA",
        "stdio": ["csv", "json", "md"],
        "mutation": "MathematicalMutation",
        "diff": "MathematicalDiff",
        "apply": "apply_mathematical_mutation",
        "mut_mut": True,
        "op_reexport": "MathematicalMutation",
    },
    {
        "plugin": "🌊️flow",
        "artifact": "🌊️flow",
        "mod": "flow",
        "prefix": "Flow",
        "crate": "semio-s-plugin-flow",
        "schema_const": "FLOW_DOCUMENT_SCHEMA",
        "stdio": ["csv", "json", "md"],
        "mutation": "FlowMutation",
        "diff": "FlowDiff",
        "apply": "apply_flow_mutation",
        "mut_mut": True,
        "op_reexport": "FlowMutation",
    },
    {
        "plugin": "🌿️vcs",
        "artifact": "🌿️vcs",
        "mod": "vcs",
        "prefix": "Vcs",
        "crate": "semio-s-plugin-vcs",
        "schema_const": "VCS_DOCUMENT_SCHEMA",
        "stdio": ["csv", "json", "xlsx", "zip"],
        "mutation": "VcsDemoMutation",
        "diff": "VcsDiff",
        "apply": "apply_vcs_demo_mutation",
        "mut_mut": True,
        "op_reexport": "VcsDemoMutation",
    },
    {
        "plugin": "🕸️dag",
        "artifact": "🕸️dag",
        "mod": "dag",
        "prefix": "Dag",
        "crate": "semio-s-plugin-dag",
        "schema_const": "DAG_DOCUMENT_SCHEMA",
        "stdio": ["csv", "json", "md", "png", "svg"],
        "mutation": "DagMutation",
        "diff": "DagDiff",
        "apply": "apply_dag_mutation",
        "mut_mut": True,
        "op_reexport": "DagMutation",
    },
]

STDIO_DIRS = {slug: ROSTER[slug]["dir"] for slug in ROSTER}

W5 = TICKET / "generators" / "w5_migrate_artifact.py"
spec = importlib.util.spec_from_file_location("w5_migrate_artifact", W5)
w5 = importlib.util.module_from_spec(spec)
assert spec.loader
spec.loader.exec_module(w5)


def art_path(cfg: dict) -> Path:
    return PLUGINS_ROOT / cfg["plugin"] / "🗿️artifacts" / cfg["artifact"]


def plugin_path(cfg: dict) -> Path:
    return PLUGINS_ROOT / cfg["plugin"]


def mut_mod(dirname: str) -> str:
    slug = "".join(c if c.isascii() and (c.isalnum() or c == "-") else "" for c in dirname)
    return slug.replace("-", "_")


def ensure_tree(dst: Path) -> None:
    w5.ensure_tree(dst)


def absorb(art: Path) -> None:
    w5.absorb(art)


def copy_missing_leaves(dst: Path) -> None:
    mapping = [
        f"🧬️schema/📸️snapshot/{TEXT}",
        f"🧬️schema/📸️snapshot/{BINARY}",
        f"🧬️schema/🔺️diff/{TEXT}",
        f"🧬️schema/🔺️diff/{BINARY}",
        f"🧬️schema/🧬️mutations/{TEXT}",
        f"🧬️schema/🧬️mutations/{BINARY}",
        BUILDER,
        DECOMPOSER,
    ]
    text_leaves = [
        "📖️component.grammar.semio",
        "🔤️component.ebnf",
        "🅰️component.g4",
        "🔗️component.graphql",
        "🔣️component.json",
        "🛰️component.proto",
        RS_LEAF,
        TS_LEAF,
    ]
    bin_leaves = [
        "📡️component.protocol.semio",
        "🔠️component.abnf",
        "🥋️component.ksy",
        "🌶️component.spicy",
        RS_LEAF,
        TS_LEAF,
    ]
    for rel in mapping:
        (dst / rel).mkdir(parents=True, exist_ok=True)
        ref = REF / rel if (REF / rel).exists() else NOTE_ART / rel
        if not ref.exists():
            continue
        for f in ref.rglob("*"):
            if f.is_file():
                out = dst / rel / f.relative_to(ref)
                out.parent.mkdir(parents=True, exist_ok=True)
                if not out.exists():
                    shutil.copy2(f, out)
    for leaf in text_leaves:
        for base in (f"🧬️schema/📸️snapshot/{TEXT}", f"🧬️schema/🔺️diff/{TEXT}", f"🧬️schema/🧬️mutations/{TEXT}"):
            p = dst / base / leaf
            if not p.exists() and (NOTE_ART / base / leaf).exists():
                shutil.copy2(NOTE_ART / base / leaf, p)
    for leaf in bin_leaves:
        for base in (f"🧬️schema/📸️snapshot/{BINARY}", f"🧬️schema/🔺️diff/{BINARY}", f"🧬️schema/🧬️mutations/{BINARY}"):
            p = dst / base / leaf
            if not p.exists() and (NOTE_ART / base / leaf).exists():
                shutil.copy2(NOTE_ART / base / leaf, p)


def fix_dsl_example_paths(art: Path) -> None:
    text_rs = art / "🧬️schema/📸️snapshot" / TEXT / RS_LEAF
    if not text_rs.exists():
        return
    t = text_rs.read_text(encoding="utf-8")
    t2 = re.sub(
        r'include_str!\("\.\./📚️examples/',
        'include_str!("../../../📚️examples/',
        t,
    )
    t2 = re.sub(
        r'include_str!\("\.\./\.\./📚️examples/',
        'include_str!("../../../📚️examples/',
        t2,
    )
    if t != t2:
        text_rs.write_text(t2, encoding="utf-8")


def write_builder(art: Path, cfg: dict) -> None:
    m, p, snap, mut_, diff, apply = (
        cfg["mod"],
        cfg["prefix"],
        f"{cfg['prefix']}Snapshot",
        cfg["mutation"],
        cfg["diff"],
        cfg["apply"],
    )
    mutate_body = (
        f"        crate::artifacts::{m}::schema::mutations::{apply}(&mut self.snapshot, &mutation);\n        self"
        if cfg["mut_mut"]
        else f"        self.snapshot = crate::artifacts::{m}::schema::mutations::{apply}(&self.snapshot, &mutation);\n        self"
    )
    rs = f"""//! {p}Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{m}::{{{diff}, {mut_}, {snap}}};

#[derive(Clone, Debug, Default)]
pub struct {p}Builder {{
    snapshot: {snap},
    diagnostics: Vec<dsl::Diagnostic>,
}}

impl ArtifactBuilder for {p}Builder {{
    type Snapshot = {snap};
    type Mutation = {mut_};
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
{mutate_body}
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
    (art / BUILDER / RS_LEAF).write_text(rs, encoding="utf-8")
    ts = f"/** {p}Builder */\nexport interface {p}Builder {{ build(): {{ schema: string }}; }}\n"
    (art / BUILDER / TS_LEAF).write_text(ts, encoding="utf-8")


def write_decomposer(art: Path, cfg: dict) -> None:
    m, p, snap = cfg["mod"], cfg["prefix"], f"{cfg['prefix']}Snapshot"
    rs = f"""//! {p}Decomposer
use semio_framework_plugin::{{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource}};
use crate::artifacts::{m}::{snap};

#[derive(Clone, Debug, Default)]
pub struct {p}Parts {{ pub snapshot: Option<{snap}> }}

pub struct {p}Decomposer;

impl ArtifactDecomposer for {p}Decomposer {{
    type Snapshot = {snap};
    type Parts = {p}Parts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {{
        let mut parts = {p}Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {{
            match source {{
                DecomposeSource::Text(text) => match <{snap} as store::DocumentDsl>::parse_dsl(text) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{m}.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
                DecomposeSource::Binary(bytes) => match <{snap} as store::DocumentPack>::decode_pack(bytes) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{m}.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
            }}
        }}
        Decomposition {{ parts, confidence, diagnostics }}
    }}
}}
"""
    (art / DECOMPOSER / RS_LEAF).write_text(rs, encoding="utf-8")
    ts = f"""/** {p}Decomposer */
export interface Decomposition<T> {{ parts: T; confidence: 'high'|'medium'|'low'; diagnostics: unknown[]; }}
"""
    (art / DECOMPOSER / TS_LEAF).write_text(ts, encoding="utf-8")


def stdio_rs_name(slug: str) -> str:
    return {"csv": "csv", "json": "json", "md": "md", "pdf": "pdf", "txt": "txt", "docx": "docx",
            "png": "png", "svg": "svg", "zip": "zip", "xlsx": "xlsx"}[slug]


def stdio_type_prefix(slug: str) -> str:
    return {"csv": "Csv", "json": "Json", "md": "Md", "pdf": "Pdf", "txt": "Txt", "docx": "Docx",
            "png": "Png", "svg": "Svg", "zip": "Zip", "xlsx": "Xlsx"}[slug]


def write_deser(art: Path, cfg: dict, slug: str) -> None:
    m, snap, p = cfg["mod"], f"{cfg['prefix']}Snapshot", cfg["prefix"]
    sp = stdio_type_prefix(slug)
    sn = stdio_rs_name(slug)
    schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
    base = art / "🚪️io/📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS[slug]
    base.mkdir(parents=True, exist_ok=True)
    wire = f"""
use crate::artifacts::{m}::io::{{{m}_from_wire, pack_err_as_text}};""" if slug in ("png", "svg") else ""
    body = {
        "json": f"""//! Deserialize {m} via stdio.json.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &JsonSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    serde_json::from_value(from.value.clone()).map_err(|e| store::TextError::new(format!("{m}<-json: {{e}}"), dsl::TextSpan::at(1, 1)))
}}

pub fn deserialize_text(text: &str) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentDsl>::parse_dsl(text)
}}
""",
        "md": f"""//! Deserialize {m} via stdio.md.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::md::{{MdSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &MdSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    <{snap} as store::DocumentDsl>::parse_dsl(&from.body)
}}

pub fn deserialize_text(text: &str) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentDsl>::parse_dsl(text)
}}
""",
        "txt": f"""//! Deserialize {m} via stdio.txt.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::txt::{{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &TxtSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.text)
}}

pub fn deserialize_text(text: &str) -> Result<{snap}, store::TextError> {{
    <{snap} as store::DocumentDsl>::parse_dsl(text)
}}
""",
        "csv": f"""//! Deserialize {m} via stdio.csv.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &CsvSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{m}<-csv: {{e}}"), dsl::TextSpan::at(1, 1)))
}}
""",
        "pdf": f"""//! Deserialize {m} via stdio.pdf.
use crate::artifacts::{m}::{{{snap}, {cfg['schema_const']}}};
use semio_s_plugin_stdio::artifacts::pdf::{{PdfSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &PdfSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    Ok({snap} {{
        schema: {cfg['schema_const']}.into(),
        id: "pdf-import".into(),
        language_id: "plain".into(),
        uri: "writer://pdf-import".into(),
        text: from.page.text.clone(),
    }})
}}
""" if m == "writer" else f"""//! Deserialize {m} via stdio.pdf.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::pdf::{{PdfSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &PdfSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    <{snap} as store::DocumentDsl>::parse_dsl(&from.page.text)
}}
""",
        "docx": f"""//! Deserialize {m} via stdio.docx.
use crate::artifacts::{m}::{{{snap}, {cfg['schema_const']}}};
use semio_s_plugin_stdio::artifacts::docx::{{DocxSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &DocxSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    let body = from.entries.iter().filter(|e| e.name.ends_with(".xml") || e.name.contains("document"))
        .filter_map(|e| std::str::from_utf8(&e.data).ok()).collect::<Vec<_>>().join("\\n");
    Ok({snap} {{
        schema: {cfg['schema_const']}.into(),
        id: "docx-import".into(),
        language_id: "plain".into(),
        uri: "writer://docx-import".into(),
        text: body,
    }})
}}
""",
        "png": f"""//! Deserialize {m} via stdio.png.{wire}
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::png::{{PngSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &PngSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{m}<-png: {{e}}"), dsl::TextSpan::at(1, 1)))
}}
""",
        "svg": f"""//! Deserialize {m} via stdio.svg.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::svg::{{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &SvgSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    <{snap} as store::DocumentDsl>::parse_dsl(&from.body)
}}
""",
        "zip": f"""//! Deserialize {m} via stdio.zip.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::zip::{{ZipSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &ZipSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{m}<-zip: {{e}}"), dsl::TextSpan::at(1, 1)))
}}
""",
        "xlsx": f"""//! Deserialize {m} via stdio.xlsx.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::xlsx::{{XlsxSnapshot, {schema}}};

pub fn register() {{}}

pub fn deserialize(from: &XlsxSnapshot) -> Result<{snap}, store::TextError> {{
    let _ = {schema};
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("{m}<-xlsx: {{e}}"), dsl::TextSpan::at(1, 1)))
}}
""",
    }[slug]
    (base / RS_LEAF).write_text(body, encoding="utf-8")
    (base / TS_LEAF).write_text(f"/** {m} import {slug} */\nexport function register(): void {{}}\n", encoding="utf-8")


def write_ser(art: Path, cfg: dict, slug: str) -> None:
    m, snap = cfg["mod"], f"{cfg['prefix']}Snapshot"
    schema = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA"
    base = art / "🚪️io/📤️export" / SER / "🗿️artifacts" / STDIO_DIRS[slug]
    base.mkdir(parents=True, exist_ok=True)
    if slug == "json":
        body = f"""//! Serialize {m} to stdio.json.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::json::{{JsonSnapshot, {schema}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<JsonSnapshot, store::PackError> {{
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(JsonSnapshot {{ schema: {schema}.into(), value }})
}}

pub fn serialize_text(from: &{snap}) -> Result<String, store::PackError> {{
    Ok(<{snap} as store::DocumentDsl>::print_dsl(from))
}}
"""
    elif slug in ("md", "txt"):
        sn, sc = ("Md", "STDIO_MD_DOCUMENT_SCHEMA") if slug == "md" else ("Txt", "STDIO_TXT_DOCUMENT_SCHEMA")
        field = "body" if slug == "md" else "text"
        body = f"""//! Serialize {m} to stdio.{slug}.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{sn}Snapshot, {sc}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<{sn}Snapshot, store::PackError> {{
    Ok({sn}Snapshot {{ schema: {sc}.into(), {field}: <{snap} as store::DocumentDsl>::print_dsl(from) }})
}}

pub fn serialize_text(from: &{snap}) -> Result<String, store::PackError> {{
    Ok(<{snap} as store::DocumentDsl>::print_dsl(from))
}}
"""
    elif slug == "csv":
        body = f"""//! Serialize {m} to stdio.csv.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::csv::{{CsvSnapshot, {schema}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<CsvSnapshot, store::PackError> {{
    serde_json::from_value(serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?)
        .map_err(|e| store::PackError::Schema(e.to_string()))
}}
"""
    elif slug == "pdf" and m == "writer":
        body = f"""//! Serialize {m} to stdio.pdf.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::pdf::{{PageDoc, PdfSnapshot, {schema}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<PdfSnapshot, store::PackError> {{
    Ok(PdfSnapshot {{
        schema: {schema}.into(),
        page: PageDoc {{ width: 612.0, height: 792.0, text: from.text.clone() }},
    }})
}}
"""
    elif slug == "docx" and m == "writer":
        body = f"""//! Serialize {m} to stdio.docx.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::docx::{{DocxEntry, DocxSnapshot, {schema}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<DocxSnapshot, store::PackError> {{
    Ok(DocxSnapshot {{
        schema: {schema}.into(),
        entries: vec![DocxEntry {{ name: "document.txt".into(), data: from.text.clone().into_bytes() }}],
    }})
}}
"""
    else:
        sp = stdio_type_prefix(slug)
        body = f"""//! Serialize {m} to stdio.{slug}.
use crate::artifacts::{m}::{snap};
use semio_s_plugin_stdio::artifacts::{slug}::{{{sp}Snapshot, {schema}}};

pub fn register() {{}}

pub fn serialize(from: &{snap}) -> Result<{sp}Snapshot, store::PackError> {{
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}}
"""
    (base / RS_LEAF).write_text(body, encoding="utf-8")
    (base / TS_LEAF).write_text(f"/** {m} export {slug} */\nexport function register(): void {{}}\n", encoding="utf-8")


def write_io_root(art: Path, cfg: dict) -> None:
    m = cfg["mod"]
    slugs = cfg["stdio"]
    lines = [f"//! {m} IO stdio matrix", "pub fn register() {"]
    for slug in slugs:
        lines.append(f"    crate::artifacts::{m}::io::import::deserializers::artifacts::{slug}::register();")
        lines.append(f"    crate::artifacts::{m}::io::export::serializers::artifacts::{slug}::register();")
    lines.append("}")
    imp = ", ".join(f'"stdio.{s}"' for s in slugs)
    lines.append(f"pub fn import_stdio_kinds() -> &'static [&'static str] {{ &[{imp}] }}")
    lines.append(f"pub fn export_stdio_kinds() -> &'static [&'static str] {{ &[{imp}] }}")
    wire = f"""
pub fn {m}_to_wire(from: &crate::artifacts::{m}::{cfg['prefix']}Snapshot) -> Vec<u8> {{
    store::DocumentPack::encode_pack(from)
}}
pub fn {m}_from_wire(bytes: &[u8]) -> Result<crate::artifacts::{m}::{cfg['prefix']}Snapshot, store::PackError> {{
    <crate::artifacts::{m}::{cfg['prefix']}Snapshot as store::DocumentPack>::decode_pack(bytes)
}}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {{
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}}
"""
    (art / "🚪️io" / RS_LEAF).write_text("\n".join(lines) + wire, encoding="utf-8")
    io_ts = f"/** {m} io */\nexport function register(): void {{}}\n"
    (art / "🚪️io" / TS_LEAF).write_text(io_ts, encoding="utf-8")


def patch_glue(cfg: dict) -> None:
    art_e, mod_, plug = cfg["artifact"], cfg["mod"], plugin_path(cfg)
    art = art_path(cfg)
    glue = plug / "📦️packages/🦀️rust/📦️glue.rs"
    muts = art / "🧬️schema/🧬️mutations"
    mut_dirs = [c.name for c in sorted(muts.iterdir()) if c.is_dir() and c.name not in (TEXT, BINARY)] if muts.exists() else []
    ap = f"../../🗿️artifacts/{art_e}"
    parts = ["//#region 🗿️Artifacts", '#[path = "."]', "pub mod artifacts {", '    #[path = "."]', f"    pub mod {mod_} {{", f'        #[path = "{ap}/🦀️component.rs"]', "        mod component;", "        pub use component::*;", ""]
    parts += ['        #[path = "."]', "        pub mod schema {", f'            #[path = "{ap}/🧬️schema/🦀️component.rs"]', "            mod component;", "            pub use component::*;", '            #[path = "."]', "            pub mod snapshot {", f'                #[path = "{ap}/🧬️schema/📸️snapshot/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs"]', "                pub mod text;", f'                #[path = "{ap}/🧬️schema/📸️snapshot/{BINARY}/🦀️component.rs"]', "                pub mod binary;", "            }", '            #[path = "."]', "            pub mod diff {", f'                #[path = "{ap}/🧬️schema/🔺️diff/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"]', "                pub mod text;", f'                #[path = "{ap}/🧬️schema/🔺️diff/{BINARY}/🦀️component.rs"]', "                pub mod binary;", "            }", '            #[path = "."]', "            pub mod mutations {", f'                #[path = "{ap}/🧬️schema/🧬️mutations/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs"]', "                pub mod text;", f'                #[path = "{ap}/🧬️schema/🧬️mutations/{BINARY}/🦀️component.rs"]', "                pub mod binary;"]
    for d in mut_dirs:
        mm = mut_mod(d)
        base = f"{ap}/🧬️schema/🧬️mutations/{d}"
        parts += ['                #[path = "."]', f"                pub mod {mm} {{", f'                    #[path = "{base}/🦠️mutation/🦀️component.rs"]', "                    pub mod mutation;", f'                    #[path = "{base}/🔺️diff/🦀️component.rs"]', "                    pub mod diff;", f'                    #[path = "{base}/↩️inverse/🦀️component.rs"]', "                    pub mod inverse;", "                }"]
    parts += ["            }", "        }", ""]
    parts += [f'        pub mod op {{ pub use crate::artifacts::{mod_}::schema::mutations::text::*; pub use crate::artifacts::{mod_}::schema::mutations::{cfg["mutation"]}; }}', f'        pub mod dsl {{ pub use crate::artifacts::{mod_}::schema::snapshot::text::*; }}', f'        pub mod spr {{ pub use crate::artifacts::{mod_}::schema::mutations::binary::*; }}', f'        pub mod diff {{ pub use crate::artifacts::{mod_}::schema::diff::*; pub mod schema {{ pub use crate::artifacts::{mod_}::schema::diff::*; }} }}', f'        pub mod mutations {{ pub use crate::artifacts::{mod_}::schema::mutations::*; }}', '        #[path = "."]', "        pub mod snapshot {", f'            pub mod schema {{ pub use crate::artifacts::{mod_}::schema::snapshot::*; }}', f'            pub mod pack {{ pub use crate::artifacts::{mod_}::schema::snapshot::binary::*; }}', "        }", f'        #[path = "{ap}/{BUILDER}/🦀️component.rs"]', "        pub mod builder;", f'        #[path = "{ap}/{DECOMPOSER}/🦀️component.rs"]', "        pub mod decomposer;", '        #[path = "."]', "        pub mod io {", f'            #[path = "{ap}/🚪️io/🦀️component.rs"]', "            mod component;", "            pub use component::*;", '            #[path = "."]', "            pub mod import {", '                #[path = "."]', f"                pub mod deserializers {{", '                    #[path = "."]', "                    pub mod artifacts {"]
    for slug in cfg["stdio"]:
        dname = STDIO_DIRS[slug]
        parts += ['                        #[path = "."]', f"                        pub mod {slug} {{", f'                            #[path = "{ap}/🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs"]', "                            mod component;", "                            pub use component::*;", "                        }"]
    parts += ["                    }", "                }", "            }", '            #[path = "."]', "            pub mod export {", '                #[path = "."]', f"                pub mod serializers {{", '                    #[path = "."]', "                    pub mod artifacts {"]
    for slug in cfg["stdio"]:
        dname = STDIO_DIRS[slug]
        parts += ['                        #[path = "."]', f"                        pub mod {slug} {{", f'                            #[path = "{ap}/🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs"]', "                            mod component;", "                            pub use component::*;", "                        }"]
    parts += ["                    }", "                }", "            }"]
    for slug in cfg["stdio"]:
        parts += ['            #[path = "."]', f"            pub mod {slug} {{", '                #[path = "."]', "                pub mod export {", f"                    pub use crate::artifacts::{mod_}::io::export::serializers::artifacts::{slug}::*;", "                }", '                #[path = "."]', "                pub mod import {", f"                    pub use crate::artifacts::{mod_}::io::import::deserializers::artifacts::{slug}::*;", "                }", "            }"]
    parts += [f'            #[path = "{ap}/⚙️engine/🦀️component.rs"]', "            pub mod engine;", "        }", "    }", "}", ""]
    new_region = "\n".join(parts) + "\n"
    text = glue.read_text(encoding="utf-8")
    start = text.find("//#region 🗿️Artifacts")
    end = text.find("//#endregion 🗿️Artifacts")
    assert start >= 0 and end > start, glue
    glue.write_text(text[:start] + new_region + text[end:], encoding="utf-8")
    cargo = plug / "📦️packages/🦀️rust/Cargo.toml"
    c = cargo.read_text(encoding="utf-8")
    if "semio-s-plugin-stdio" not in c:
        c = c.replace("[dependencies]\n", '[dependencies]\nsemio-s-plugin-stdio = { path = "../../../🗄️stdio/📦️packages/🦀️rust", package = "semio-s-plugin-stdio" }\n', 1)
        cargo.write_text(c, encoding="utf-8")


def patch_ts(cfg: dict) -> None:
    art_e, mod_, plug = cfg["artifact"], cfg["mod"], plugin_path(cfg)
    ap = f"../../🗿️artifacts/{art_e}"
    ts = plug / "📦️packages/🟦️typescript/📦️index.ts"
    lines = [f"/** {mod_} facet WASM facades */", f'export * as {mod_}_schema from "{ap}/🧬️schema/{TS_LEAF}";', f'export * as {mod_}_snapshot from "{ap}/🧬️schema/📸️snapshot/{TS_LEAF}";', f'export * as {mod_}_snapshot_text from "{ap}/🧬️schema/📸️snapshot/{TEXT}/{TS_LEAF}";', f'export * as {mod_}_snapshot_binary from "{ap}/🧬️schema/📸️snapshot/{BINARY}/{TS_LEAF}";', f'export * as {mod_}_diff from "{ap}/🧬️schema/🔺️diff/{TS_LEAF}";', f'export * as {mod_}_diff_text from "{ap}/🧬️schema/🔺️diff/{TEXT}/{TS_LEAF}";', f'export * as {mod_}_diff_binary from "{ap}/🧬️schema/🔺️diff/{BINARY}/{TS_LEAF}";', f'export * as {mod_}_mutations from "{ap}/🧬️schema/🧬️mutations/{TS_LEAF}";', f'export * as {mod_}_mutations_text from "{ap}/🧬️schema/🧬️mutations/{TEXT}/{TS_LEAF}";', f'export * as {mod_}_mutations_binary from "{ap}/🧬️schema/🧬️mutations/{BINARY}/{TS_LEAF}";', f'export * as {mod_}_io from "{ap}/🚪️io/{TS_LEAF}";', f'export * as {mod_}_builder from "{ap}/{BUILDER}/{TS_LEAF}";', f'export * as {mod_}_decomposer from "{ap}/{DECOMPOSER}/{TS_LEAF}";', ""]
    ts.write_text("\n".join(lines), encoding="utf-8")


def verify_old_gone(art: Path) -> list[str]:
    return w5.verify(art)


def migrate_one(cfg: dict) -> dict:
    art = art_path(cfg)
    absorb(art)
    copy_missing_leaves(art)
    fix_dsl_example_paths(art)
    write_builder(art, cfg)
    write_decomposer(art, cfg)
    for slug in cfg["stdio"]:
        write_deser(art, cfg, slug)
        write_ser(art, cfg, slug)
    write_io_root(art, cfg)
    patch_glue(cfg)
    patch_ts(cfg)
    return {"art": str(art), "verify": verify_old_gone(art)}


def main() -> int:
    results = []
    for cfg in BATCH:
        print("migrate", cfg["plugin"], flush=True)
        results.append({"plugin": cfg["plugin"], "crate": cfg["crate"], **migrate_one(cfg)})
    out = TICKET / "generators/w6_batch1a_migrate.json"
    out.write_text(json.dumps(results, indent=2), encoding="utf-8")
    print("wrote", out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
