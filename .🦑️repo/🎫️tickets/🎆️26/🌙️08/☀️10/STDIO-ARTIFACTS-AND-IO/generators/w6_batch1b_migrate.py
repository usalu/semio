#!/usr/bin/env python3
"""W6 batch1b: draw, raster, forms, layout, playbook stdio absorb."""
from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
TOK = json.loads((TICKET / "🧪tokens.json").read_text(encoding="utf-8"))
OT = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))
REF_JSON = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json"
NOTE_ART = ROOT / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note"

TEXT, BIN = TOK["text"], TOK["binary"]
BUILDER, DECOMPOSER = TOK["builder"], TOK["decomposer"]
DESER, SER = TOK["deserializers"], TOK["serializers"]

PLUGIN_KEYS = ["🖍️draw", "🖨️raster", "📋️forms", "📏️layout", "📖️playbook"]
STDIO_DIRS = {k: v["dir"] for k, v in OT["stdio_roster"].items()}

CFG = {
    "🖍️draw": {
        "mod": "draw",
        "emoji": "🖍️draw",
        "prefix": "Draw",
        "mutation": "DrawMutation",
        "diff": "DrawDiff",
        "schema_const": "DRAW_DOCUMENT_SCHEMA",
        "apply_fn": "apply_draw_edit_mutation",
        "apply_mut": False,
        "dwg_fn": "draw_document_json_from_dwg",
        "op_use": "pub use crate::artifacts::draw::schema::mutations::{apply_draw_edit_mutation, DrawMutation};",
        "crate": "semio-s-plugin-draw",
    },
    "🖨️raster": {
        "mod": "raster",
        "emoji": "🖨️raster",
        "prefix": "Raster",
        "mutation": "RasterMutation",
        "diff": "RasterDiff",
        "schema_const": "RASTER_DOCUMENT_SCHEMA",
        "apply_fn": "apply_raster_mutation",
        "apply_mut": False,
        "dwg_fn": "raster_document_json_from_dwg",
        "op_use": "pub use crate::artifacts::raster::schema::mutations::{apply_raster_mutation, RasterMutation};",
        "crate": "semio-s-plugin-raster",
    },
    "📏️layout": {
        "mod": "layout",
        "emoji": "📏️layout",
        "prefix": "Layout",
        "mutation": "LayoutMutation",
        "diff": "LayoutDiff",
        "schema_const": "LAYOUT_DOCUMENT_SCHEMA",
        "apply_fn": "apply_layout_mutation",
        "apply_mut": True,
        "dwg_fn": "layout_document_json_from_dwg",
        "op_use": "pub use crate::artifacts::layout::schema::mutations::{apply_layout_mutation, LayoutMutation};",
        "crate": "semio-s-plugin-layout",
    },
    "📋️forms": {
        "mod": "forms",
        "emoji": "📋️forms",
        "prefix": "Forms",
        "mutation": "FormMutation",
        "diff": "FormsDiff",
        "schema_const": "FORMS_DOCUMENT_SCHEMA",
        "apply_fn": "apply_form_edit_mutation",
        "apply_mut": False,
        "dwg_fn": None,
        "op_use": "pub use crate::artifacts::forms::schema::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation};",
        "crate": "semio-s-plugin-forms",
    },
    "📖️playbook": {
        "mod": "playbook",
        "emoji": "📖️playbook",
        "prefix": "Playbook",
        "mutation": "PlaybookMutation",
        "diff": "PlaybookDiff",
        "schema_const": "PLAYBOOK_DOCUMENT_SCHEMA",
        "apply_fn": "apply_playbook_mutation",
        "apply_mut": False,
        "dwg_fn": None,
        "op_use": "pub use crate::artifacts::playbook::schema::mutations::{apply_playbook_mutation, PlaybookMutation};",
        "crate": "semio-s-plugin-playbook",
    },
}


def owner_row(emoji: str) -> dict:
    for row in OT["owners"]:
        if row["plugin"] == emoji:
            return row
    raise KeyError(emoji)


def art_path(emoji: str) -> Path:
    return ROOT / owner_row(emoji)["path"]


def plugin_root(emoji: str) -> Path:
    return ROOT / "✏️s/🔌️plugins" / emoji


def ensure_tree(dst: Path):
    mapping = [
        f"🧬️schema/📸️snapshot/{TEXT}",
        f"🧬️schema/📸️snapshot/{BIN}",
        f"🧬️schema/🔺️diff/{TEXT}",
        f"🧬️schema/🔺️diff/{BIN}",
        f"🧬️schema/🧬️mutations/{TEXT}",
        f"🧬️schema/🧬️mutations/{BIN}",
        BUILDER,
        DECOMPOSER,
        f"🚪️io/📥️import/{DESER}/🗿️artifacts",
        f"🚪️io/📤️export/{SER}/🗿️artifacts",
    ]
    for rel in mapping:
        (dst / rel).mkdir(parents=True, exist_ok=True)
        ref = REF_JSON / rel.replace("🚪️io/📥️import/" + DESER + "/🗿️artifacts", "").replace("🚪️io/📤️export/" + SER + "/🗿️artifacts", "")
        if not ref.exists():
            ref = REF_JSON
        for f in REF_JSON.rglob("*"):
            if not f.is_file():
                continue
            relf = f.relative_to(REF_JSON)
            if "🚪️io" in str(relf):
                continue
            out = dst / relf
            if "📸️snapshot" in str(relf) or "🔺️diff" in str(relf) or "🧬️mutations" in str(relf):
                out.parent.mkdir(parents=True, exist_ok=True)
                if not out.exists() and f.suffix in {".json", ".graphql", ".proto", ".ts", ".abnf", ".ksy", ".spicy", ".ebnf", ".g4", ".grammar.semio", ".protocol.semio"}:
                    shutil.copy2(f, out)


def move_if(src: Path, dst: Path):
    if not src.exists():
        return
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.exists() and src.is_dir():
        for f in src.rglob("*"):
            if f.is_file():
                t = dst / f.relative_to(src)
                t.parent.mkdir(parents=True, exist_ok=True)
                if not t.exists():
                    shutil.copy2(f, t)
        shutil.rmtree(src)
        return
    if not dst.exists():
        shutil.move(str(src), str(dst))


def absorb(art: Path):
    ensure_tree(art)
    move_if(art / "🗣️dsl", art / f"🧬️schema/📸️snapshot/{TEXT}")
    move_if(art / "📸️snapshot/🎒️pack", art / f"🧬️schema/📸️snapshot/{BIN}")
    snap_schema = art / "📸️snapshot/🧬️schema"
    if snap_schema.exists():
        for f in snap_schema.iterdir():
            if f.is_file():
                t = art / "🧬️schema/📸️snapshot" / f.name
                if not t.exists():
                    shutil.copy2(f, t)
        shutil.rmtree(snap_schema)
    snap = art / "📸️snapshot"
    if snap.exists():
        for f in snap.iterdir():
            if f.is_file():
                t = art / "🧬️schema/📸️snapshot" / f.name
                if not t.exists():
                    shutil.copy2(f, t)
            elif f.is_dir() and f.name not in {"🎒️pack", "🧬️schema"}:
                t = art / "🧬️schema/📸️snapshot" / f.name
                if not t.exists():
                    shutil.copytree(f, t)
        shutil.rmtree(snap, ignore_errors=True)
    diff = art / "🔺️diff"
    if diff.exists():
        ds = diff / "🧬️schema"
        if ds.exists():
            for f in ds.iterdir():
                if f.is_file():
                    t = art / "🧬️schema/🔺️diff" / f.name
                    if not t.exists():
                        shutil.copy2(f, t)
            shutil.rmtree(ds)
        for f in list(diff.iterdir()):
            if f.is_file():
                t = art / f"🧬️schema/🔺️diff/{TEXT}" / f.name
                t.parent.mkdir(parents=True, exist_ok=True)
                if not t.exists():
                    shutil.copy2(f, t)
        shutil.rmtree(diff, ignore_errors=True)
    move_if(art / "🔧️op", art / f"🧬️schema/🧬️mutations/{TEXT}")
    move_if(art / "📡️spr", art / f"🧬️schema/🧬️mutations/{BIN}")
    mut = art / "🧬️mutations"
    if mut.exists() and mut.parent == art:
        dest = art / "🧬️schema/🧬️mutations"
        dest.mkdir(parents=True, exist_ok=True)
        for child in list(mut.iterdir()):
            t = dest / child.name
            if not t.exists():
                shutil.move(str(child), str(t))
            elif child.is_dir():
                shutil.rmtree(child)
            else:
                child.unlink()
        shutil.rmtree(mut, ignore_errors=True)
    io = art / "🚪️io"
    if io.exists():
        for fmt in list(io.iterdir()):
            if not fmt.is_dir() or fmt.name in {"📥️import", "📤️export"}:
                continue
            for direction, bucket in [("📥️import", DESER), ("📤️export", SER)]:
                old = fmt / direction
                if not old.exists():
                    continue
                new = io / direction / bucket / "🗿️artifacts" / fmt.name
                new.mkdir(parents=True, exist_ok=True)
                for f in old.rglob("*"):
                    if f.is_file():
                        t = new / f.relative_to(old)
                        t.parent.mkdir(parents=True, exist_ok=True)
                        if not t.exists():
                            shutil.copy2(f, t)
            shutil.rmtree(fmt)


def scaffold_builder(cfg: dict, art: Path):
    mod = cfg["mod"]
    p, m, d, s = cfg["prefix"], cfg["mutation"], cfg["diff"], cfg["schema_const"]
    apply_fn = cfg["apply_fn"]
    if cfg["apply_mut"]:
        mutate = f"        {apply_fn}(&mut self.snapshot, &mutation);\n        self"
    else:
        mutate = f"        self.snapshot = crate::artifacts::{mod}::schema::mutations::{apply_fn}(&self.snapshot, &mutation);\n        self"
    rs = f"""//! {p}Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::{mod}::{{{d}, {m}, {p}Snapshot}};

#[derive(Clone, Debug, Default)]
pub struct {p}Builder {{
    snapshot: {p}Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}}

impl ArtifactBuilder for {p}Builder {{
    type Snapshot = {p}Snapshot;
    type Mutation = {m};
    type Diff = {d};
    fn empty() -> Self {{ Self {{ snapshot: {p}Snapshot::default(), diagnostics: Vec::new() }} }}
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {{ Self {{ snapshot, diagnostics: Vec::new() }} }}
    fn from_text(text: &str) -> Result<Self, store::TextError> {{
        Ok(Self::from_snapshot(<{p}Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }}
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {{
        Ok(Self::from_snapshot(<{p}Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }}
    fn mutate(mut self, mutation: Self::Mutation) -> Self {{
{mutate}
    }}
    fn absorb(mut self, diff: Self::Diff) -> Self {{
        self.snapshot = <{d} as protocol::MutationDiff<{p}Snapshot>>::apply(&diff, &self.snapshot);
        self
    }}
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {{
        if self.diagnostics.is_empty() {{ Ok(self.snapshot) }} else {{ Err(self.diagnostics) }}
    }}
}}
"""
    ts = f"/** {p}Builder */\nexport interface {p}Builder {{ build(): {{ schema: string }}; }}\n"
    (art / BUILDER / "🦀️component.rs").write_text(rs, encoding="utf-8")
    (art / BUILDER / "🟦️component.ts").write_text(ts, encoding="utf-8")


def scaffold_decomposer(cfg: dict, art: Path):
    mod, p = cfg["mod"], cfg["prefix"]
    rs = f"""//! {p}Decomposer
use semio_framework_plugin::{{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource}};
use crate::artifacts::{mod}::{p}Snapshot;

#[derive(Clone, Debug, Default)]
pub struct {p}Parts {{ pub snapshot: Option<{p}Snapshot> }}

pub struct {p}Decomposer;

impl ArtifactDecomposer for {p}Decomposer {{
    type Snapshot = {p}Snapshot;
    type Parts = {p}Parts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {{
        let mut parts = {p}Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {{
            match source {{
                DecomposeSource::Text(text) => match <{p}Snapshot as store::DocumentDsl>::parse_dsl(text) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{mod}.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
                DecomposeSource::Binary(bytes) => match <{p}Snapshot as store::DocumentPack>::decode_pack(bytes) {{
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {{
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("{mod}.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }}
                }},
            }}
        }}
        Decomposition {{ parts, confidence, diagnostics }}
    }}
}}
"""
    ts = f"/** {p}Decomposer */\nexport interface Decomposition<T> {{ parts: T; confidence: 'high'|'medium'|'low'; diagnostics: unknown[]; }}\n"
    (art / DECOMPOSER / "🦀️component.rs").write_text(rs, encoding="utf-8")
    (art / DECOMPOSER / "🟦️component.ts").write_text(ts, encoding="utf-8")


def patch_root_reexports(cfg: dict, art: Path):
    comp = art / "🦀️component.rs"
    t = comp.read_text(encoding="utf-8")
    mod, p, m, d = cfg["mod"], cfg["prefix"], cfg["mutation"], cfg["diff"]
    new = f"pub use crate::artifacts::{mod}::schema::snapshot::{p}Snapshot;\npub use crate::artifacts::{mod}::schema::diff::{d};\npub use crate::artifacts::{mod}::schema::mutations::{m};\n"
    t2 = re.sub(
        r"pub use crate::artifacts::" + mod + r"\.[^;]+;\n(?:pub use crate::artifacts::" + mod + r"\.[^;]+;\n)*",
        new,
        t,
        count=1,
    )
    if t2 == t:
        if f"schema::snapshot::{p}Snapshot" not in t:
            t2 = t.replace("//#endregion 🔖️Domain", new + "\n//#endregion 🔖️Domain")
    comp.write_text(t2, encoding="utf-8")


def note_codec_substitute(text: str, cfg: dict) -> str:
    mod, p = cfg["mod"], cfg["prefix"]
    repl = [
        ("note", mod),
        ("Note", p),
        ("NOTE_DOCUMENT_SCHEMA", cfg["schema_const"]),
        ("note_document_json_from_dwg", cfg.get("dwg_fn") or "note_document_json_from_dwg"),
        ("create_note_id", f"create_{mod}_id" if mod != "forms" else "create_forms_id"),
        ("empty_note_snapshot", f"empty_{mod}_snapshot" if mod != "playbook" else "empty_playbook_snapshot"),
    ]
    for a, b in repl:
        if b:
            text = text.replace(a, b)
    return text


def write_json_codec(cfg: dict, art: Path, slug: str, direction: str):
    mod, p, s = cfg["mod"], cfg["prefix"], cfg["schema_const"]
    stdio = STDIO_DIRS[slug]
    if direction == "import":
        body = f"""//! {mod} <- {slug}
use crate::artifacts::{mod}::{{{p}Snapshot, {s}}};
use semio_s_plugin_stdio::artifacts::{slug}.{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};
pub fn register() {{}}
pub fn deserialize(from: &JsonSnapshot) -> Result<{p}Snapshot, String> {{
    let mut snap: {p}Snapshot = serde_json::from_value(from.value.clone()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() {{ snap.schema = {s}.into(); }}
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(snap)
}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{p}Snapshot, String> {{
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }})
}}
"""
        if slug != "json":
            note_path = NOTE_ART / f"🚪️io/📥️import/{DESER}/🗿️artifacts/{stdio}/🦀️component.rs"
            if note_path.exists():
                body = note_codec_substitute(note_path.read_text(encoding="utf-8"), cfg)
                body = re.sub(r"//! note <- .*", f"//! {mod} <- {slug}", body, count=1)
    else:
        body = f"""//! {mod} -> {slug}
use crate::artifacts::{mod}::{p}Snapshot;
use semio_s_plugin_stdio::artifacts::{slug}.{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};
pub fn register() {{}}
pub fn serialize(snapshot: &{p}Snapshot) -> Result<JsonSnapshot, String> {{
    Ok(JsonSnapshot {{ schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value: serde_json::to_value(snapshot).map_err(|e| e.to_string())? }})
}}
pub fn serialize_bytes(snapshot: &{p}Snapshot) -> Result<Vec<u8>, String> {{
    serde_json::to_vec_pretty(&serialize(snapshot)?.value).map_err(|e| e.to_string())
}}
"""
        if slug != "json":
            note_path = NOTE_ART / f"🚪️io/📤️export/{SER}/🗿️artifacts/{stdio}/🦀️component.rs"
            if note_path.exists():
                body = note_codec_substitute(note_path.read_text(encoding="utf-8"), cfg)
                body = re.sub(r"//! note -> .*", f"//! {mod} -> {slug}", body, count=1)
    bucket = DESER if direction == "import" else SER
    out = art / f"🚪️io/{('📥️import' if direction == 'import' else '📤️export')}/{bucket}/🗿️artifacts/{stdio}/🦀️component.rs"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")


def write_text_dsl_codec(cfg: dict, art: Path, slug: str, direction: str):
    mod, p = cfg["mod"], cfg["prefix"]
    stdio = STDIO_DIRS[slug]
    snap_type = f"{p}Snapshot"
    stdio_mod = slug
    schema_const = f"STDIO_{slug.upper()}_DOCUMENT_SCHEMA" if slug != "md" else "STDIO_MD_DOCUMENT_SCHEMA"
    if slug == "txt":
        schema_const = "STDIO_TXT_DOCUMENT_SCHEMA"
    if direction == "import":
        body = f"""//! {mod} <- {slug}
use crate::artifacts::{mod}::{snap_type};
use semio_s_plugin_stdio::artifacts::{stdio_mod}::{{{p}Snapshot as StioSnap, {schema_const}}};
pub fn register() {{}}
pub fn deserialize(from: &StioSnap) -> Result<{snap_type}, String> {{
    let text = serde_json::to_string(from).ok();
    if let Some(t) = text {{
        return <{snap_type} as store::DocumentDsl>::parse_dsl(&t).map_err(|e| e.to_string());
    }}
    <{snap_type} as store::DocumentDsl>::parse_dsl("").map_err(|e| e.to_string())
}}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<{snap_type}, String> {{
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    <{snap_type} as store::DocumentDsl>::parse_dsl(text).map_err(|e| e.to_string())
}}
"""
    else:
        body = f"""//! {mod} -> {slug}
use crate::artifacts::{mod}::{snap_type};
pub fn register() {{}}
pub fn serialize(snapshot: &{snap_type}) -> Result<Vec<u8>, String> {{
    Ok(<{snap_type} as store::DocumentDsl>::render_dsl(snapshot).into_bytes())
}}
pub fn serialize_bytes(snapshot: &{snap_type}) -> Result<Vec<u8>, String> {{ serialize(snapshot) }}
"""
    bucket = DESER if direction == "import" else SER
    dire = "📥️import" if direction == "import" else "📤️export"
    out = art / f"🚪️io/{dire}/{bucket}/🗿️artifacts/{stdio}/🦀️component.rs"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")


def write_codecs(cfg: dict, art: Path, formats: list[str]):
    for slug in formats:
        if slug in {"json", "dwg", "dxf", "pdf", "png", "svg"}:
            write_json_codec(cfg, art, slug, "import")
            write_json_codec(cfg, art, slug, "export")
        elif slug in {"md", "txt"}:
            write_text_dsl_codec(cfg, art, slug, "import")
            write_text_dsl_codec(cfg, art, slug, "export")
        else:
            write_json_codec(cfg, art, "json", "import")
            write_json_codec(cfg, art, "json", "export")


def write_io_register(cfg: dict, art: Path, formats: list[str]):
    mod = cfg["mod"]
    lines = [f"//! {mod} IO stdio matrix", "pub fn register() {"]
    for slug in formats:
        lines.append(f"    crate::artifacts::{mod}::io::import::deserializers::artifacts::{slug}::register();")
        lines.append(f"    crate::artifacts::{mod}::io::export::serializers::artifacts::{slug}::register();")
    lines.append("}")
    kinds = ", ".join(f'"stdio.{s}"' for s in formats)
    lines.append(f"pub fn import_stdio_kinds() -> &'static [&'static str] {{ &[{kinds}] }}")
    lines.append(f"pub fn export_stdio_kinds() -> &'static [&'static str] {{ &[{kinds}] }}")
    (art / "🚪️io/🦀️component.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")


def mut_mod(dirname: str) -> str:
    slug = "".join(c if c.isascii() and (c.isalnum() or c == "-") else "" for c in dirname)
    return slug.replace("-", "_")


def build_glue_region(cfg: dict, art: Path) -> str:
    mod, emoji, p = cfg["mod"], cfg["emoji"], cfg["prefix"]
    muts = art / "🧬️schema/🧬️mutations"
    mut_dirs = [
        c.name
        for c in sorted(muts.iterdir())
        if c.is_dir()
        and c.name not in (TEXT, BIN)
        and (c / "🦠️mutation" / "🦀️component.rs").exists()
    ]
    formats = owner_row(emoji)["stdio_artifacts"]
    parts = ["//#region 🗿️Artifacts", "#[path = \".\"]", "pub mod artifacts {", "    #[path = \".\"]", f"    pub mod {mod} {{"]
    parts.append(f'        #[path = "../../🗿️artifacts/{emoji}/🦀️component.rs"]')
    parts.append("        mod component;")
    parts.append("        pub use component::*;")
    parts.append('        #[path = "."]')
    parts.append("        pub mod schema {")
    parts.append(f'            #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🦀️component.rs"]')
    parts.append("            mod component;")
    parts.append("            pub use component::*;")
    parts.append('            #[path = "."]')
    parts.append("            pub mod snapshot {")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/🦀️component.rs"]')
    parts.append("                mod component;")
    parts.append("                pub use component::*;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs"]')
    parts.append("                pub mod text;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/{BIN}/🦀️component.rs"]')
    parts.append("                pub mod binary;")
    parts.append("            }")
    parts.append('            #[path = "."]')
    parts.append("            pub mod diff {")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/🦀️component.rs"]')
    parts.append("                mod component;")
    parts.append("                pub use component::*;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"]')
    parts.append("                pub mod text;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/{BIN}/🦀️component.rs"]')
    parts.append("                pub mod binary;")
    parts.append("            }")
    parts.append('            #[path = "."]')
    parts.append("            pub mod mutations {")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/🦀️component.rs"]')
    parts.append("                mod component;")
    parts.append("                pub use component::*;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs"]')
    parts.append("                pub mod text;")
    parts.append(f'                #[path = "../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{BIN}/🦀️component.rs"]')
    parts.append("                pub mod binary;")
    for d in mut_dirs:
        mm = mut_mod(d)
        base = f"../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{d}"
        parts.append('                #[path = "."]')
        parts.append(f"                pub mod {mm} {{")
        parts.append(f'                    #[path = "{base}/🦠️mutation/🦀️component.rs"]')
        parts.append("                    pub mod mutation;")
        parts.append(f'                    #[path = "{base}/🔺️diff/🦀️component.rs"]')
        parts.append("                    pub mod diff;")
        parts.append(f'                    #[path = "{base}/↩️inverse/🦀️component.rs"]')
        parts.append("                    pub mod inverse;")
        parts.append("                }")
    parts.append("            }")
    parts.append("        }")
    parts.append(f"        pub mod op {{ {cfg['op_use']} }}")
    parts.append(f"        pub mod dsl {{ pub use crate::artifacts::{mod}::schema::snapshot::text::*; }}")
    parts.append(f"        pub mod spr {{ pub use crate::artifacts::{mod}::schema::mutations::binary::*; }}")
    parts.append(
        f"        pub mod diff {{ pub use crate::artifacts::{mod}::schema::diff::*; pub mod schema {{ pub use crate::artifacts::{mod}::schema::diff::*; }} pub mod text {{ pub use crate::artifacts::{mod}::schema::diff::text::*; }} }}"
    )
    parts.append(f"        pub mod mutations {{ pub use crate::artifacts::{mod}::schema::mutations::*; }}")
    parts.append(
        f"        pub mod snapshot {{ pub mod schema {{ pub use crate::artifacts::{mod}::schema::snapshot::*; }} pub mod pack {{ pub use crate::artifacts::{mod}::schema::snapshot::binary::*; }} }}"
    )
    parts.append(f'        #[path = "../../🗿️artifacts/{emoji}/{BUILDER}/🦀️component.rs"]')
    parts.append("        pub mod builder;")
    parts.append(f'        #[path = "../../🗿️artifacts/{emoji}/{DECOMPOSER}/🦀️component.rs"]')
    parts.append("        pub mod decomposer;")
    parts.append('        #[path = "."]')
    parts.append("        pub mod io {")
    parts.append(f'            #[path = "../../🗿️artifacts/{emoji}/🚪️io/🦀️component.rs"]')
    parts.append("            mod component;")
    parts.append("            pub use component::*;")
    parts.append('            #[path = "."]')
    parts.append("            pub mod import {")
    parts.append('                #[path = "."]')
    parts.append("                pub mod deserializers {")
    parts.append('                    #[path = "."]')
    parts.append("                    pub mod artifacts {")
    for slug in formats:
        dname = STDIO_DIRS[slug]
        parts.append('                        #[path = "."]')
        parts.append(f"                        pub mod {slug} {{")
        parts.append(
            f'                            #[path = "../../🗿️artifacts/{emoji}/🚪️io/📥️import/{DESER}/🗿️artifacts/{dname}/🦀️component.rs"]'
        )
        parts.append("                            mod component;")
        parts.append("                            pub use component::*;")
        parts.append("                        }")
    parts.append("                    }")
    parts.append("                }")
    parts.append("            }")
    parts.append('            #[path = "."]')
    parts.append("            pub mod export {")
    parts.append('                #[path = "."]')
    parts.append("                pub mod serializers {")
    parts.append('                    #[path = "."]')
    parts.append("                    pub mod artifacts {")
    for slug in formats:
        dname = STDIO_DIRS[slug]
        parts.append('                        #[path = "."]')
        parts.append(f"                        pub mod {slug} {{")
        parts.append(
            f'                            #[path = "../../🗿️artifacts/{emoji}/🚪️io/📤️export/{SER}/🗿️artifacts/{dname}/🦀️component.rs"]'
        )
        parts.append("                            mod component;")
        parts.append("                            pub use component::*;")
        parts.append("                        }")
    parts.append("                    }")
    parts.append("                }")
    parts.append("            }")
    for slug in formats:
        parts.append('            #[path = "."]')
        parts.append(f"            pub mod {slug} {{")
        parts.append('                #[path = "."]')
        parts.append("                pub mod export {")
        parts.append(f"                    pub use crate::artifacts::{mod}::io::export::serializers::artifacts::{slug}::*;")
        parts.append("                }")
        parts.append('                #[path = "."]')
        parts.append("                pub mod import {")
        parts.append(f"                    pub use crate::artifacts::{mod}::io::import::deserializers::artifacts::{slug}::*;")
        parts.append("                }")
        parts.append("            }")
    parts.append("        }")
    parts.append(f'        #[path = "../../🗿️artifacts/{emoji}/⚙️engine/🦀️component.rs"]')
    parts.append("        pub mod engine;")
    parts.append("    }")
    parts.append("}")
    parts.append("")
    return "\n".join(parts) + "\n"


def patch_glue(cfg: dict):
    emoji = cfg["emoji"]
    mod = cfg["mod"]
    glue = plugin_root(emoji) / "📦️packages/🦀️rust/📦️glue.rs"
    art = art_path(emoji)
    text = glue.read_text(encoding="utf-8")
    start = text.find("//#region 🗿️Artifacts")
    end = text.find("//#endregion 🗿️Artifacts")
    region = build_glue_region(cfg, art)
    glue.write_text(text[:start] + region + text[end:], encoding="utf-8")
    cargo = plugin_root(emoji) / "📦️packages/🦀️rust/Cargo.toml"
    c = cargo.read_text(encoding="utf-8")
    if "semio-s-plugin-stdio" not in c:
        c = c.replace(
            "[dependencies]\n",
            "[dependencies]\nsemio-s-plugin-stdio = { path = \"../../../🗄️stdio/📦️packages/🦀️rust\", package = \"semio-s-plugin-stdio\" }\n",
            1,
        )
        cargo.write_text(c, encoding="utf-8")
    ts_leaf = "🟦️component.ts"
    ts_lines = [
        f"/** {mod} facet WASM facades */",
        f"export * as {mod}_schema from \"../../🗿️artifacts/{emoji}/🧬️schema/{ts_leaf}\";",
        f"export * as {mod}_snapshot from \"../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/{ts_leaf}\";",
        f"export * as {mod}_snapshot_text from \"../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/{TEXT}/{ts_leaf}\";",
        f"export * as {mod}_snapshot_binary from \"../../🗿️artifacts/{emoji}/🧬️schema/📸️snapshot/{BIN}/{ts_leaf}\";",
        f"export * as {mod}_diff from \"../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/{ts_leaf}\";",
        f"export * as {mod}_diff_text from \"../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/{TEXT}/{ts_leaf}\";",
        f"export * as {mod}_diff_binary from \"../../🗿️artifacts/{emoji}/🧬️schema/🔺️diff/{BIN}/{ts_leaf}\";",
        f"export * as {mod}_mutations from \"../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{ts_leaf}\";",
        f"export * as {mod}_mutations_text from \"../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{TEXT}/{ts_leaf}\";",
        f"export * as {mod}_mutations_binary from \"../../🗿️artifacts/{emoji}/🧬️schema/🧬️mutations/{BIN}/{ts_leaf}\";",
        f"export * as {mod}_io from \"../../🗿️artifacts/{emoji}/🚪️io/{ts_leaf}\";",
        f"export * as {mod}_builder from \"../../🗿️artifacts/{emoji}/{BUILDER}/{ts_leaf}\";",
        f"export * as {mod}_decomposer from \"../../🗿️artifacts/{emoji}/{DECOMPOSER}/{ts_leaf}\";",
    ]
    (plugin_root(emoji) / "📦️packages/🟦️typescript/📦️index.ts").write_text("\n".join(ts_lines) + "\n", encoding="utf-8")


def verify_old(art: Path) -> list[str]:
    errs = []
    for old in ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr"]:
        if (art / old).exists():
            errs.append(old)
    if (art / "🧬️mutations").exists() and (art / "🧬️mutations").parent == art:
        errs.append("root_mutations")
    return errs


def migrate_one(emoji: str):
    cfg = CFG[emoji]
    art = art_path(emoji)
    row = owner_row(emoji)
    formats = row["stdio_artifacts"]
    print(f"migrate {emoji}")
    absorb(art)
    scaffold_builder(cfg, art)
    scaffold_decomposer(cfg, art)
    patch_root_reexports(cfg, art)
    write_codecs(cfg, art, formats)
    write_io_register(cfg, art, formats)
    patch_glue(cfg)
    errs = verify_old(art)
    print(f"  old facets: {errs}")


def cargo_check(crate: str) -> tuple[int, str]:
    r = subprocess.run(
        ["cargo", "check", "-p", crate],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    log = (r.stdout or "") + (r.stderr or "")
    (TICKET / f"🧪w6-batch1b-{crate}.log").write_text(log, encoding="utf-8")
    return r.returncode, log


def main():
    for emoji in PLUGIN_KEYS:
        migrate_one(emoji)
    results = {}
    for emoji in PLUGIN_KEYS:
        code, _ = cargo_check(CFG[emoji]["crate"])
        results[emoji] = code
    print(json.dumps(results, indent=2))
    return 0 if all(c == 0 for c in results.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
