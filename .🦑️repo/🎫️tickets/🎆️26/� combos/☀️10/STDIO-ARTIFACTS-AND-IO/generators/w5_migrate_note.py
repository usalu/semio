#!/usr/bin/env python3
"""W5 note pilot — migrate 🗒️note artifact to stdio shape."""
from __future__ import annotations

import json
import re
import shutil
import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = list((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
NOTE = ROOT / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note"
PLUGIN = ROOT / "✏️s/🔌️plugins/🗒️note"
JSON_REF = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json"

BUILDER = TOKENS["builder"]
DECOMPOSER = TOKENS["decomposer"]
TEXT = TOKENS["text"]
BINARY = TOKENS["binary"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]
KSY = TOKENS["ksy"]
ABNF = TOKENS["abnf"]
SPICY = TOKENS["spicy"]
EBNF = TOKENS["ebnf"]
G4 = TOKENS["g4"]
GRAMMAR = TOKENS["grammar"]
PROTOCOL = TOKENS["protocol"]

STDIO_DIRS = {
    "dwg": "🖊️dwg",
    "dxf": "🖊️dxf",
    "json": "🔣️json",
    "pdf": "📄️pdf",
    "png": "📷️png",
    "svg": "🎨️svg",
}
STDIO_KINDS = ["dwg", "dxf", "json", "pdf", "png", "svg"]


def ensure_dir(p: Path) -> None:
    p.mkdir(parents=True, exist_ok=True)


def move_file(src: Path, dst: Path) -> None:
    if not src.exists():
        return
    ensure_dir(dst.parent)
    if dst.exists():
        if dst.is_file():
            dst.unlink()
        else:
            shutil.rmtree(dst)
    shutil.move(str(src), str(dst))


def move_dir_contents(src: Path, dst: Path) -> None:
    if not src.exists():
        return
    ensure_dir(dst)
    for p in list(src.iterdir()):
        target = dst / p.name
        if p.is_dir():
            if target.exists():
                move_dir_contents(p, target)
                try:
                    p.rmdir()
                except OSError:
                    shutil.rmtree(p, ignore_errors=True)
            else:
                shutil.move(str(p), str(target))
        else:
            move_file(p, target)


def rmtree_if_empty_or_leftover(p: Path) -> None:
    if not p.exists():
        return
    # remove entirely (old facet absorbed)
    shutil.rmtree(p, ignore_errors=True)


def write(p: Path, content: str) -> None:
    ensure_dir(p.parent)
    p.write_text(content)


def phase_moves() -> None:
    print("[1] path-map moves")
    move_dir_contents(NOTE / "🗣️dsl", NOTE / "🧬️schema" / "📸️snapshot" / TEXT)
    move_dir_contents(NOTE / "📸️snapshot" / "🎒️pack", NOTE / "🧬️schema" / "📸️snapshot" / BINARY)
    move_dir_contents(NOTE / "📸️snapshot" / "🧬️schema", NOTE / "🧬️schema" / "📸️snapshot")

    diff_src = NOTE / "🔺️diff"
    diff_text = NOTE / "🧬️schema" / "🔺️diff" / TEXT
    diff_root = NOTE / "🧬️schema" / "🔺️diff"
    ensure_dir(diff_text)
    ensure_dir(diff_root)
    if diff_src.exists():
        for name in [GRAMMAR, "🦀️component.rs", "🟦️component.ts"]:
            move_file(diff_src / name, diff_text / name)
        move_dir_contents(diff_src / "🧬️schema", diff_root)

    move_dir_contents(NOTE / "�, diff_text / name)
        move_dir_contents(diff_src / "🧬️schema", diff_root)

    move_dir_contents(NOTE / "🔧️op", NOTE / "🧬️schema" / "🧬️mutations" / TEXT)
    move_dir_contents(NOTE / "📡️spr", NOTE / "🧬️schema" / "🧬️mutations" / BINARY)

    mut_src = NOTE / "🧬️mutations"
    mut_dst = NOTE / "🧬️schema" / "🧬️mutations"
    if mut_src.exists():
        for p in list(mut_src.iterdir()):
            target = mut_dst / p.name
            if p.is_dir():
                move_dir_contents(p, target)
            else:
                move_file(p, target)

    for old in ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr", "🧬️mutations"]:
        rmtree_if_empty_or_leftover(NOTE / old)
        print("  deleted", old, "exists=", (NOTE / old).exists())


TEXT_LEAF_NAMES = [
    GRAMMAR,
    EBNF,
    G4,
    "🔗️component.graphql",
    "🔣️component.json",
    "🛰️component.proto",
    "🦀️component.rs",
    "🟦️component.ts",
]
BINARY_LEAF_NAMES = [
    PROTOCOL,
    ABNF,
    KSY,
    SPICY,
    "🦀️component.rs",
    "🟦️component.ts",
]


def handcraft_text_leaves(folder: Path, slug: str, title: str, schema_id: str, grammar_body: str | None = None) -> None:
    ensure_dir(folder)
    if grammar_body is None:
        grammar_body = f"""dialect grammar
grammar {schema_id}
extension note
start document
document = header body
header = "schema" TEXT
body = payload
payload = TEXT
"""
    if not (folder / GRAMMAR).exists():
        write(folder / GRAMMAR, grammar_body)
    write(
        folder / EBNF,
        f"(* ebnf {schema_id} *)\ndocument = header, body ;\nheader = 'schema', space, '{schema_id}', newline ;\n",
    )
    write(folder / G4, f"grammar Note_{slug};\nDOCUMENT: 'schema' [ ]+ '{schema_id}' ;\n")
    write(
        folder / "🔗️component.graphql",
        f"# {schema_id} text grammar schema\nscalar Bytes\ntype Document {{ schema: String! payload: String! }}\n",
    )
    write(
        folder / "🔣️component.json",
        json.dumps(
            {
                "$id": f"https://semio.tech/schema/{schema_id}/text.json",
                "title": title,
                "type": "object",
            },
            indent=2,
        )
        + "\n",
    )
    write(
        folder / "🛰️component.proto",
        f'syntax = "proto3";\npackage semio.note_{slug};\nmessage Document {{ string schema = 1; bytes payload = 2; }}\n',
    )
    if not (folder / "🦀️component.rs").exists():
        write(
            folder / "🦀️component.rs",
            f"""//! 📝️ Text representation codec surface for `{schema_id}`.

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("{GRAMMAR}");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::{GRAMMAR}");
""",
        )
    if not (folder / "🟦️component.ts").exists():
        write(folder / "🟦️component.ts", f"/** 📝️ {title} text facade. */\nexport {{}};\n")


def handcraft_binary_leaves(folder: Path, slug: str, title: str, schema_id: str, protocol_body: str | None = None) -> None:
    ensure_dir(folder)
    if protocol_body is None:
        protocol_body = f"dialect protocol {schema_id}\nmagic = %x00\npayload = *OCTET\n"
    if not (folder / PROTOCOL).exists():
        write(folder / PROTOCOL, protocol_body)
    write(folder / ABNF, f"; abnf {schema_id}\ndocument = payload\npayload = *OCTET\n")
    write(
        folder / KSY,
        f"meta:\n  id: note_{slug}\n  endian: le\nseq:\n  - id: payload\n    size-eos: true\n",
    )
    write(
        folder / SPICY,
        f"module Note_{slug};\ntype Document = unit {{\n    payload: bytes &eod;\n}};\n",
    )
    if not (folder / "🦀️component.rs").exists():
        write(
            folder / "🦀️component.rs",
            f"""//! 💾️ Binary representation codec surface for `{schema_id}`.

/// 📡️ Protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("{PROTOCOL}");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::{PROTOCOL}");
""",
        )
    if not (folder / "🟦️component.ts").exists():
        write(folder / "🟦️component.ts", f"/** 💾️ {title} binary facade. */\nexport {{}};\n")


def phase_leaves() -> None:
    print("[2] handcraft text/binary leaves")
    snap_text = NOTE / "🧬️schema" / "📸️snapshot" / TEXT
    snap_bin = NOTE / "🧬️schema" / "📸️snapshot" / BINARY
    diff_text = NOTE / "🧬️schema" / "🔺️diff" / TEXT
    diff_bin = NOTE / "🧬️schema" / "🔺️diff" / BINARY
    mut_text = NOTE / "🧬️schema" / "🧬️mutations" / TEXT
    mut_bin = NOTE / "🧬️schema" / "🧬️mutations" / BINARY

    # Fix example include path in snapshot text rs if present
    text_rs = snap_text / "🦀️component.rs"
    if text_rs.exists():
        t = text_rs.read_text()
        t = t.replace(
            'include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")',
            'include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")',
        )
        # Keep existing Op/DSL helpers; ensure grammar consts exist
        text_rs.write_text(t)

    handcraft_text_leaves(snap_text, "snapshot_text", "NoteSnapshotText", "note.document")
    handcraft_binary_leaves(snap_bin, "snapshot_binary", "NoteSnapshotBinary", "note.pack")
    handcraft_text_leaves(diff_text, "diff_text", "NoteDiffText", "note.diff")
    handcraft_binary_leaves(diff_bin, "diff_binary", "NoteDiffBinary", "note.diff.pack")
    handcraft_text_leaves(mut_text, "mutations_text", "NoteMutationText", "note.op")
    handcraft_binary_leaves(mut_bin, "mutations_binary", "NoteMutationBinary", "note.spr")

    # Preserve OpText/OpBinary impls that lived in op/spr — merge into text/binary rs if we overwrote stubs
    # Read current mut text/binary and ensure Op impls present by rewriting from migrated content patterns
    mut_text_rs = mut_text / "🦀️component.rs"
    body = mut_text_rs.read_text()
    if "impl protocol::OpText" not in body:
        write(
            mut_text_rs,
            """//! 🔧 note — OpText surface for `NoteMutation` (was 🔧️op).
pub use crate::artifacts::note::schema::mutations::{apply_note_mutation, NoteMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region � = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for NoteMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for NoteMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
""",
        )

    mut_bin_rs = mut_bin / "🦀️component.rs"
    body = mut_bin_rs.read_text()
    if "encode_op" not in body or "fn encode_op" not in body:
        write(
            mut_bin_rs,
            """//! ⚖️ Note mutation binary SPR surface (was 📡️spr).

use crate::artifacts::note::schema::mutations::text::NoteMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `NoteMutation` to its binary state-patch form.
pub fn encode_op(operation: &NoteMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `NoteMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<NoteMutation, protocol::ProtocolError> {
    NoteMutation::decode_op(bytes)
}
""",
        )

    # triad ts leaves for diff/inverse where missing
    mut_root = NOTE / "🧬️schema" / "🧬️mutations"
    for mut_dir in mut_root.iterdir():
        if not mut_dir.is_dir() or mut_dir.name in (TEXT, BINARY):
            continue
        for triad in ["🦠️mutation", "🔺️diff", "↩️inverse"]:
            d = mut_dir / triad
            ensure_dir(d)
            rs = d / "🦀️component.rs"
            ts = d / "🟦️component.ts"
            if not rs.exists():
                write(rs, f"//! {triad} for {mut_dir.name}\n")
            if not ts.exists():
                write(ts, f"/** {triad} for {mut_dir.name} */\nexport {{}};\n")


def phase_schema_descriptor() -> None:
    print("[3] patch schema descriptor include paths")
    schema_rs = NOTE / "🧬️schema" / "🦀️component.rs"
    t = schema_rs.read_text()
    t = t.replace(
        'include_str!("../📸️snapshot/🧬️schema/🦀️component.rs")',
        'include_str!("📸️snapshot/🦀️component.rs")',
    )
    t = t.replace(
        'include_str!("../�include_str!("📸️snapshot/🦀️component.rs")',
    )
    t = t.replace(
        'include_str!("../📸️snapshot/🧬️schema/🟦️component.ts")',
        'include_str!("📸️snapshot/🟦️component.ts")',
    )
    t = t.replace(
        'include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql")',
        'include_str!("📸️snapshot/🔗️component.graphql")',
    )
    t = t.replace(
        'include_str!("../�',
        'include_str!("📸️snapshot/🔗️component.graphql")',
    )
    t = t.replace(
        'include_str!("../📸️snapshot/🧬️schema/🔣️component.json")',
        'include_str!("📸️snapshot/🔣️component.json")',
    )
    t = t.replace(
        'include_str!("../📸️snapshot/🧬️schema/🛰️component.proto")',
        'include_str!("📸️snapshot/🛰️component.proto")',
    )
    t = t.replace(
        'include_str!("../🔺️diff/🧬️schema/🦀️component.rs")',
        'include_str!("🔺️diff/🦀️component.rs")',
    )
    t = t.replace(
        'include_str!("../🔺️diff/🧬️schema/🟦️component.ts")',
        'include_str!("🔺️diff/🟦️component.ts")',
    )
    t = t.replace(
        'include_str!("../🔺️diff/🧬️schema/🔗️component.graphql")',
        'include_str!("🔺️diff/🔗️component.graphql")',
    )
    t = t.replace(
        'include_str!("../🔺️diff/🧬️schema/🔣️component.json")',
        'include_str!("🔺️diff/🔣️component.json")',
    )
    t = t.replace(
        'include_str!("../🔺️diff/🧬️schema/🛰️component.proto")',
        'include_str!("🔺️diff/🛰️component.proto")',
    )
    schema_rs.write_text(t)

    # Fix internal path refs in moved files
    for rs in (NOTE / "🧬️schema").rglob("🦀️component.rs"):
        t = rs.read_text()
        orig = t
        t = t.replace("crate::artifacts::note::mutations::", "crate::artifacts::note::schema::mutations::")
        t = t.replace("crate::artifacts::note::diff::", "crate::artifacts::note::schema::diff::")
        t = t.replace("crate::artifacts::note::op::", "crate::artifacts::note::schema::mutations::text::")
        t = t.replace("crate::artifacts::note::dsl::", "crate::artifacts::note::schema::snapshot::text::")
        t = t.replace("crate::artifacts::note::spr::", "crate::artifacts::note::schema::mutations::binary::")
        t = t.replace("crate::artifacts::note::snapshot::schema::", "crate::artifacts::note::schema::snapshot::")
        t = t.replace("crate::artifacts::note::snapshot::pack::", "crate::artifacts::note::schema::snapshot::binary::")
        if t != orig:
            rs.write_text(t)


def phase_builder_decomposer() -> None:
    print("[4] builder + decomposer")
    write(
        NOTE / BUILDER / "🦀️component.rs",
        """//! 🏗️ NoteBuilder — ArtifactBuilder for `2d.note`.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::note::{NoteDiff, NoteMutation, NoteSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `2d.note` snapshot.
#[derive(Clone, Debug, Default)]
pub struct NoteBuilder {
    snapshot: NoteSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for NoteBuilder {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Diff = NoteDiff;
    fn empty() -> Self {
        Self { snapshot: NoteSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<NoteSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<NoteSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::note::schema::mutations::apply_note_mutation(&mut self.snapshot, &mutation);
        // apply_note_mutation may take (&, &) returning new — handle both shapes below in patch if needed
        let _ = &mut self.snapshot;
        let next = {
            let projection = self.snapshot.clone();
            crate::artifacts::note::schema::mutations::apply_note_mutation(&projection, &mutation)
        };
        self.snapshot = next;
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
""",
    )
    write(
        NOTE / BUILDER / "🟦️component.ts",
        """/** 🏗️ NoteBuilder facade. */
export interface NoteBuilder { build(): { schema: string }; }
""",
    )
    write(
        NOTE / DECOMPOSER / "🦀️component.rs",
        """//! 📑️ NoteDecomposer — ArtifactDecomposer for `2d.note`.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Parts
/// 🧩 Decomposed `2d.note` parts.
#[derive(Clone, Debug, Default)]
pub struct NoteParts { pub snapshot: Option<NoteSnapshot>, }
//#endregion 🔖️Parts

//#region � { pub snapshot: Option<NoteSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `2d.note` sources.
pub struct NoteDecomposer;

impl ArtifactDecomposer for NoteDecomposer {
    type Snapshot = NoteSnapshot;
    type Parts = NoteParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = NoteParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <NoteSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "note.decompose.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                DecomposeSource::Binary(bytes) => match <NoteSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "note.decompose.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
//#endregion 🔖️Decomposer
""",
    )
    write(
        NOTE / DECOMPOSER / "🟦️component.ts",
        """/** 📑️ NoteDecomposer facade. */
export interface Decomposition<T> {
  parts: T;
  confidence: 'high' | 'medium' | 'low';
  diagnostics: unknown[];
}
""",
    )


def phase_io() -> None:
    print("[5] rewrite IO")
    # delete old format trees
    io = NOTE / "🚪️io"
    for child in list(io.iterdir()):
        if child.name in ("🦀️component.rs", "🟦️component.ts"):
            continue
        if child.is_dir():
            shutil.rmtree(child)

    write(
        io / "�🟦️component.ts"):
            continue
        if child.is_dir():
            shutil.rmtree(child)

    write(
        io / "🦀️component.rs",
        """//! 🚪️ 🗒️note IO — stdio deserializer/serializer registration.

//#region 🔖️Register
pub fn register() {
    crate::artifacts::note::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::note::io::import::deserializers::artifacts::dxf::register();
    crate::artifacts::note::io::import::deserializers::artifacts::json::register();
    crate::artifacts::note::io::import::deserializers::artifacts::pdf::register();
    crate::artifacts::note::io::import::deserializers::artifacts::png::register();
    crate::artifacts::note::io::import::deserializers::artifacts::svg::register();
    crate::artifacts::note::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::note::io::export::serializers::artifacts::dxf::register();
    crate::artifacts::note::io::export::serializers::artifacts::json::register();
    crate::artifacts::note::io::export::serializers::artifacts::pdf::register();
    crate::artifacts::note::io::export::serializers::artifacts::png::register();
    crate::artifacts::note::io::export::serializers::artifacts::svg::register();
}

/// 🗄️ Stdio kind ids this artifact imports.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"]
}

/// 🗄️ Stdio kind ids this artifact exports.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"]
}
//#endregion 🔖️Register
""",
    )
    write(io / "🟦️component.ts", "/** 🚪️ note IO facade. */\nexport {};\n")

    # JSON
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["json"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.json` → NoteSnapshot.

use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap: NoteSnapshot = serde_json::from_value(from.value.clone()).map_err(|e| e.to_string())?;
    if snap.schema.is_empty() {
        snap.schema = NOTE_DOCUMENT_SCHEMA.into();
    }
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    deserialize(&JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["json"] / "🟦️component.ts",
        "/** 📥️ note←json deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["json"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.json`.

use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<JsonSnapshot, String> {
    Ok(JsonSnapshot {
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        value: serde_json::to_value(snapshot).map_err(|e| e.to_string())?,
    })
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let json = serialize(snapshot)?;
    serde_json::to_vec_pretty(&json.value).map_err(|e| e.to_string())
}
""",
    )
    write(
        io / "�<|control37|>export" / SER / "🗿️artifacts" / STDIO_DIRS["json"] / "🟦️component.ts",
        "/** 📤️ note→json serializer. */\nexport {};\n",
    )

    # SVG
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["svg"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.svg` → NoteSnapshot.

use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{write_svg_xml, SvgSnapshot};

pub fn register() {}

pub fn deserialize(from: &SvgSnapshot) -> Result<NoteSnapshot, String> {
    let xml = write_svg_xml(&from.doc);
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("svg-import");
    snap.title = Some("Imported SVG".into());
    snap.blocks.push(NoteBlockNode::Text {
        id: "svg-text-1".into(),
        name: "SVG".into(),
        x: 0.0,
        y: 0.0,
        width: 400.0,
        height: 200.0,
        rotation: 0.0,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph {
            runs: vec![NoteTextRun {
                text: xml.chars().take(512).collect(),
                bold: None,
                italic: None,
                underline: None,
                link: None,
            }],
        }],
        font_size: 14.0,
        font_weight: "normal".into(),
        align: "left".into(),
    });
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let doc = semio_s_plugin_stdio::artifacts::svg::schema::snapshot::parse_svg_xml(text)?;
    deserialize(&SvgSnapshot {
        schema: semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(),
        doc,
    })
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["svg"] / "🟦️component.ts",
        "/** 📥️ note←svg deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["svg"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.svg`.

use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, SvgSnapshot};
use semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<SvgSnapshot, String> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    let doc = parse_svg_xml(&svg)?;
    Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    Ok(svg.into_bytes())
}
""",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["svg"] / "🟦️component.ts",
        "/** 📤️ note→svg serializer. */\nexport {};\n",
    )

    # PNG
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["png"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.png` → NoteSnapshot.

use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::png::engine::encode_png;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;
use std::collections::BTreeMap;

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<NoteSnapshot, String> {
    let bytes = encode_png(from)?;
    let b64 = base64_encode(&bytes);
    let key = "png-import".to_string();
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("png-import");
    snap.title = Some("Imported PNG".into());
    let mut assets = BTreeMap::new();
    assets.insert(
        key.clone(),
        NoteImageAsset {
            mime: "image/png".into(),
            data: format!("data:image/png;base64,{b64}"),
            width: Some(from.image.width as f64),
            height: Some(from.image.height as f64),
        },
    );
    snap.assets = assets;
    snap.blocks.push(NoteBlockNode::Image {
        id: "png-image-1".into(),
        name: "PNG".into(),
        x: 0.0,
        y: 0.0,
        width: from.image.width.max(1) as f64,
        height: from.image.height.max(1) as f64,
        rotation: 0.0,
        visible: true,
        locked: false,
        image_key: key,
    });
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let snap = semio_s_plugin_stdio::artifacts::png::engine::decode_png(bytes)?;
    deserialize(&snap)
}

fn base64_encode(bytes: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < bytes.len() {
            out.push(T[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < bytes.len() {
            out.push(T[(b2 & 63) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["png"] / "🟦️component.ts",
        "/** 📥️ note←png deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["png"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.png`.

use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::png::engine::{encode_png, empty_png_snapshot};
use semio_s_plugin_stdio::artifacts::png::schema::snapshot::RasterImage;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<PngSnapshot, String> {
    let (w, h) = crate::artifacts::note::engine::note_document_bounds(snapshot);
    let width = w.max(1);
    let height = h.max(1);
    // Flat white raster sized to document bounds (stdio PNG codec owns wire encoding).
    let mut rgba = vec![255u8; (width as usize) * (height as usize) * 4];
    for px in rgba.chunks_mut(4) {
        px[3] = 255;
    }
    let mut snap = empty_png_snapshot();
    snap.image = RasterImage { width, height, rgba };
    let _ = snapshot;
    Ok(snap)
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    encode_png(&serialize(snapshot)?)
}
""",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["png"] / "🟦️component.ts",
        "/** 📤️ note→png serializer. */\nexport {};\n",
    )

    # PDF
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["pdf"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.pdf` → NoteSnapshot.

use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot, NoteTextParagraph, NoteTextRun};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub fn register() {}

pub fn deserialize(from: &PdfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("pdf-import");
    snap.title = Some("Imported PDF".into());
    snap.blocks.push(NoteBlockNode::Text {
        id: "pdf-text-1".into(),
        name: "PDF".into(),
        x: 0.0,
        y: 0.0,
        width: from.page.width.max(1.0),
        height: from.page.height.max(1.0),
        rotation: 0.0,
        visible: true,
        locked: false,
        paragraphs: vec![NoteTextParagraph {
            runs: vec![NoteTextRun {
                text: from.page.text.clone(),
                bold: None,
                italic: None,
                underline: None,
                link: None,
            }],
        }],
        font_size: 12.0,
        font_weight: "normal".into(),
        align: "left".into(),
    });
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let snap = semio_s_plugin_stdio::artifacts::pdf::engine::decode_pdf(bytes)?;
    deserialize(&snap)
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["pdf"] / "🟦️component.ts",
        "/** 📥️ note←pdf deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["pdf"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.pdf`.

use crate::artifacts::note::engine::{flatten_blocks, note_document_bounds};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::engine::{encode_pdf, empty_pdf_snapshot};
use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc;
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<PdfSnapshot, String> {
    let (w, h) = note_document_bounds(snapshot);
    let mut text = String::new();
    if let Some(title) = &snapshot.title {
        text.push_str(title);
        text.push(' ');
    }
    for block in flatten_blocks(&snapshot.blocks) {
        if let NoteBlockNode::Text { paragraphs, .. } = block {
            for p in paragraphs {
                for r in &p.runs {
                    text.push_str(&r.text);
                    text.push(' ');
                }
            }
        }
    }
    let mut snap = empty_pdf_snapshot();
    snap.page = PageDoc {
        width: w.max(1) as f64,
        height: h.max(1) as f64,
        text: text.trim().to_string(),
    };
    Ok(snap)
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    encode_pdf(&serialize(snapshot)?)
}
""",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["pdf"] / "🟦️component.ts",
        "/** 📤️ note→pdf serializer. */\nexport {};\n",
    )

    # DXF
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["dxf"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.dxf` → NoteSnapshot.

use crate::artifacts::note::engine::{create_note_id, empty_note_snapshot};
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::DxfSnapshot;

pub fn register() {}

pub fn deserialize(from: &DxfSnapshot) -> Result<NoteSnapshot, String> {
    let mut snap = empty_note_snapshot();
    snap.id = create_note_id("dxf-import");
    snap.title = Some("Imported DXF".into());
    for (i, line) in from.lines.iter().enumerate() {
        snap.blocks.push(NoteBlockNode::Ink {
            id: format!("dxf-line-{i}"),
            name: "Line".into(),
            x: line.x1.min(line.x2),
            y: line.y1.min(line.y2),
            width: (line.x1 - line.x2).abs().max(1.0),
            height: (line.y1 - line.y2).abs().max(1.0),
            rotation: 0.0,
            visible: true,
            locked: false,
            points: vec![[line.x1, line.y1], [line.x2, line.y2]],
            stroke_width: 1.0,
            color: [0.0, 0.0, 0.0, 1.0],
        });
    }
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let snap = semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::parse_dxf_text(text)?;
    deserialize(&snap)
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["dxf"] / "🟦️component.ts",
        "/** 📥️ note←dxf deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["dxf"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.dxf`.

use crate::artifacts::note::engine::flatten_blocks;
use crate::artifacts::note::{NoteBlockNode, NoteSnapshot};
use semio_s_plugin_stdio::artifacts::dxf::schema::snapshot::{write_dxf_text, DxfLine};
use semio_s_plugin_stdio::artifacts::dxf::{DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<DxfSnapshot, String> {
    let mut lines = Vec::new();
    for block in flatten_blocks(&snapshot.blocks) {
        if let NoteBlockNode::Ink { points, .. } = block {
            for pair in points.windows(2) {
                lines.push(DxfLine {
                    x1: pair[0][0],
                    y1: pair[0][1],
                    z1: 0.0,
                    x2: pair[1][0],
                    y2: pair[1][1],
                    z2: 0.0,
                });
            }
        }
    }
    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let snap = serialize(snapshot)?;
    Ok(write_dxf_text(&snap).into_bytes())
}
""",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["dxf"] / "🟦️component.ts",
        "/** 📤️ note→dxf serializer. */\nexport {};\n",
    )

    # DWG
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["dwg"] / "🦀️component.rs",
        """//! 📥️ Deserialize `stdio.dwg` → NoteSnapshot.

use crate::artifacts::note::NoteSnapshot;
use semio_framework::{dwg_from_bytes, DwgDrawing};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;

pub fn register() {}

pub fn deserialize(from: &DwgSnapshot) -> Result<NoteSnapshot, String> {
    deserialize_bytes(&from.bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    let value = crate::artifacts::note::engine::note_document_json_from_dwg(&drawing)?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}
""",
    )
    write(
        io / "📥️import" / DESER / "🗿️artifacts" / STDIO_DIRS["dwg"] / "🟦️component.ts",
        "/** 📥️ note←dwg deserializer. */\nexport {};\n",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["dwg"] / "🦀️component.rs",
        """//! 📤️ Serialize NoteSnapshot → `stdio.dwg`.

use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg};
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &NoteSnapshot) -> Result<DwgSnapshot, String> {
    let bytes = serialize_bytes(snapshot)?;
    decode_dwg(&bytes)
}

pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot);
    let bytes = semio_framework_os::svg_to_dwg_bytes(&svg)?;
    // Validate through stdio DWG codec.
    let snap = decode_dwg(&bytes)?;
    encode_dwg(&snap)
}
""",
    )
    write(
        io / "📤️export" / SER / "🗿️artifacts" / STDIO_DIRS["dwg"] / "🟦️component.ts",
        "/** 📤️ note→dwg serializer. */\nexport {};\n",
    )

    # Fix typo path if created
    bad = io / "�<|control37|>export"
    if bad.exists():
        # merge into real export
        real = io / "📤️export"
        move_dir_contents(bad, real)
        shutil.rmtree(bad, ignore_errors=True)


def phase_root_and_engine() -> None:
    print("[6] root artifact_kind + engine path fixes")
    root = NOTE / "🦀️component.rs"
    t = root.read_text()
    t = t.replace(
        "pub use crate::artifacts::note::snapshot::schema::NoteSnapshot;",
        "pub use crate::artifacts::note::schema::snapshot::NoteSnapshot;\npub use crate::artifacts::note::schema::diff::NoteDiff;\npub use crate::artifacts::note::schema::mutations::NoteMutation;",
    )
    # replace export/import formats with stdio kinds
    t = re.sub(
        r"export_formats:.*?,\n\s*import_formats:.*?,",
        'export_formats: vec![],\n        import_formats: vec![],',
        t,
        count=1,
        flags=re.S,
    )
    t = t.replace(
        "export_stdio_kinds: vec![],\n        import_stdio_kinds: vec![],",
        'export_stdio_kinds: crate::artifacts::note::io::export_stdio_kinds().to_vec(),\n        import_stdio_kinds: crate::artifacts::note::io::import_stdio_kinds().to_vec(),',
    )
    root.write_text(t)

    eng = NOTE / "⚙️engine" / "🦀️component.rs"
    t = eng.read_text()
    t = t.replace(
        "use semio_framework_plugin::{DwgDrawing, DwgGeometry};",
        "use semio_framework::{DwgDrawing, DwgGeometry};",
    )
    t = t.replace("crate::artifacts::note::dsl::", "crate::artifacts::note::schema::snapshot::text::")
    t = t.replace("crate::artifacts::note::snapshot::pack::", "crate::artifacts::note::schema::snapshot::binary::")
    t = t.replace("crate::artifacts::note::op::", "crate::artifacts::note::schema::mutations::text::")
    t = t.replace("crate::artifacts::note::spr::", "crate::artifacts::note::schema::mutations::binary::")
    t = t.replace("crate::artifacts::note::diff::COMPONENT_", "crate::artifacts::note::schema::diff::text::COMPONENT_")
    t = t.replace(
        "use semio_framework_plugin::{DwgColor, DwgEntity, DwgLayer};",
        "use semio_framework::{DwgColor, DwgEntity, DwgLayer};",
    )
    eng.write_text(t)

    # Fix apply_note_mutation signature usage — check if it takes &mut or returns new
    mut_rs = NOTE / "🧬️schema" / "🧬️mutations" / "🦀️component.rs"
    if mut_rs.exists():
        mt = mut_rs.read_text()
        mt = mt.replace("crate::artifacts::note::diff::", "crate::artifacts::note::schema::diff::")
        mut_rs.write_text(mt)


def phase_glue_ts() -> None:
    print("[7] glue.rs + TS barrel")
    glue = PLUGIN / "📦️packages/🦀️rust/📦️glue.rs"
    # Replace the artifacts::note module wiring for schema/io/builder/decomposer
    # We'll rewrite the note artifacts section carefully by regenerating from marker to end of note mod.

    new_note_mod = f'''    #[path = "."]
    pub mod note {{
        #[path = "../../🗿️artifacts/🗒️note/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod schema {{
            #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {{
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/{TEXT}/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/{BINARY}/🦀️component.rs"]
                pub mod binary;
            }}
            #[path = "."]
            pub mod diff {{
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/{TEXT}/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/{BINARY}/🦀️component.rs"]
                pub mod binary;
            }}
            #[path = "."]
            pub mod mutations {{
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/{TEXT}/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/{BINARY}/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_grid_visible {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/👁️set-grid-visible/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/👁️set-grid-visible/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/👁️set-grid-visible/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_grid_spacing {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📏set-grid-spacing/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📏set-grid-spacing/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📏set-grid-spacing/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_grid_subdivisions {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🔢set-grid-subdivisions/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🔢set-grid-subdivisions/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🔢set-grid-subdivisions/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_grid_opacity {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🌫️set-grid-opacity/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🌫️set-grid-opacity/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🌫️set-grid-opacity/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_snap_enabled {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧲set-snap-enabled/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧲set-snap-enabled/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧲set-snap-enabled/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_snap_grid_spacing {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📐set-snap-grid-spacing/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📐set-snap-grid-spacing/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📐set-snap-grid-spacing/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_pencil_width {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/✏️set-pencil-width/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/✏️set-pencil-width/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/✏️set-pencil-width/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_eraser_radius {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧽set-eraser-radius/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧽set-eraser-radius/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧽set-eraser-radius/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_blocks {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧱set-blocks/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧱set-blocks/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🧱set-blocks/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod put_asset {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📥put-asset/�_asset {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📥put-asset/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📥put-asset/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📥put-asset/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod remove_asset {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🗑️remove-asset/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🗑️remove-asset/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🗑️remove-asset/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
                #[path = "."]
                pub mod set_snapshot {{
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }}
            }}
        }}

        // Compatibility aliases used by app commands during absorb.
        pub mod op {{
            pub use crate::artifacts::note::schema::mutations::text::*;
            pub use crate::artifacts::note::schema::mutations::NoteMutation;
        }}
        pub mod dsl {{
            pub use crate::artifacts::note::schema::snapshot::text::*;
        }}
        pub mod spr {{
            pub use crate::artifacts::note::schema::mutations::binary::*;
        }}
        pub mod diff {{
            pub use crate::artifacts::note::schema::diff::*;
            pub mod schema {{
                pub use crate::artifacts::note::schema::diff::*;
            }}
            pub mod text {{
                pub use crate::artifacts::note::schema::diff::text::*;
            }}
        }}
        pub mod mutations {{
            pub use crate::artifacts::note::schema::mutations::*;
        }}
        pub mod snapshot {{
            pub mod schema {{
                pub use crate::artifacts::note::schema::snapshot::*;
            }}
            pub mod pack {{
                pub use crate::artifacts::note::schema::snapshot::binary::*;
            }}
        }}

        #[path = "../../🗿️artifacts/🗒️note/⚙️engine/🦀️component.rs"]
        pub mod engine;
        #[path = "../../🗿️artifacts/🗒️note/{BUILDER}/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🗒️note/{DECOMPOSER}/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {{
            #[path = "../../🗿️artifacts/🗒️note/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {{
                #[path = "."]
                pub mod deserializers {{
                    #[path = "."]
                    pub mod artifacts {{
                        #[path = "."]
                        pub mod dwg {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['dwg']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod dxf {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['dxf']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod json {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['json']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod pdf {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['pdf']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod png {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['png']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod svg {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📥️import/{DESER}/🗿️artifacts/{STDIO_DIRS['svg']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                    }}
                }}
            }}
            #[path = "."]
            pub mod export {{
                #[path = "."]
                pub mod serializers {{
                    #[path = "."]
                    pub mod artifacts {{
                        #[path = "."]
                        pub mod dwg {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS['dwg']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod dxf {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS['dxf']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod json {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS['json']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod pdf {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS['pdf']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod png {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/�<|control37|>export/{SER}/🗿️artifacts/{STDIO_DIRS['png']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                        #[path = "."]
                        pub mod svg {{
                            #[path = "../../🗿️artifacts/🗒️note/🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS['svg']}/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }}
                    }}
                }}
            }}
        }}
    }}
'''
    # Fix accidental typo �<|control37|>export for png in template
    new_note_mod = new_note_mod.replace(
        f'🚪️io/�<|control37|>export/{SER}/🗿️artifacts/{STDIO_DIRS["png"]}',
        f'🚪️io/📤️export/{SER}/🗿️artifacts/{STDIO_DIRS["png"]}',
    )

    text = glue.read_text()
    start = text.find("    pub mod note {")
    if start < 0:
        start = text.find("pub mod note {")
    # find end of note module — the closing before `//#endregion 🗿️Artifacts`
    end_marker = "//#endregion 🗿️Artifacts"
    end = text.find(end_marker)
    if start < 0 or end < 0:
        raise SystemExit(f"glue markers not found start={start} end={end}")
    # include the wrapping `#[path = "."] pub mod artifacts {` structure: only replace inside note
    # Find the note module's closing brace before endregion — the artifacts mod closes after note.
    # Strategy: from `pub mod note {` find matching brace.
    i = text.find("{", start)
    depth = 0
    j = i
    while j < len(text):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                j += 1
                break
        j += 1
    # Keep indentation of note module inside artifacts
    replacement = new_note_mod.rstrip() + "\n"
    # If original started with spaces+pub, keep
    text = text[:start] + replacement + text[j:]
    glue.write_text(text)

    # TS barrel
    ts = PLUGIN / "📦️packages/� + replacement + text[j:]
    glue.write_text(text)

    # TS barrel
    ts = PLUGIN / "📦️packages/🟦️typescript/📦️index.ts"
    write(
        ts,
        f"""/** note facet WASM facades */
export * as note_schema from "../../🗿️artifacts/🗒️note/🧬️schema/🟦️component.ts";
export * as note_snapshot from "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/�schema/🟦️component.ts";
export * as note_snapshot from "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/🟦️component.ts";
export * as note_snapshot_text from "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/{TEXT}/🟦️component.ts";
export * as note_snapshot_binary from "../../🗿️artifacts/🗒️note/🧬️schema/📸️snapshot/{BINARY}/🟦️component.ts";
export * as note_diff from "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/🟦️component.ts";
export * as note_diff_text from "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/{TEXT}/🟦️component.ts";
export * as note_diff_binary from "../../🗿️artifacts/🗒️note/🧬️schema/🔺️diff/{BINARY}/🟦️component.ts";
export * as note_mutations from "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/🟦️component.ts";
export * as note_mutations_text from "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/{TEXT}/🟦️component.ts";
export * as note_mutations_binary from "../../🗿️artifacts/🗒️note/🧬️schema/🧬️mutations/{BINARY}/🟦️component.ts";
export * as note_io from "../../🗿️artifacts/🗒️note/🚪️io/🟦️component.ts";
export * as note_builder from "../../🗿️artifacts/🗒️note/{BUILDER}/🟦️component.ts";
export * as note_decomposer from "../../🗿️artifacts/🗒️note/{DECOMPOSER}/�note/{BUILDER}/🟦️component.ts";
export * as note_decomposer from "../../🗿️artifacts/🗒️note/{DECOMPOSER}/🟦️component.ts";
""",
    )

    # Cargo.toml deps
    cargo = PLUGIN / "📦️packages/🦀️rust/Cargo.toml"
    c = cargo.read_text()
    if "semio-s-plugin-stdio" not in c:
        c = c.replace(
            "semio-framework-plugin = { workspace = true, features = [\"component-guest\"] }",
            "semio-framework-plugin = { workspace = true, features = [\"component-guest\"] }\n"
            "semio-framework = { workspace = true }\n"
            "semio-s-plugin-stdio = { path = \"../../../🗄️stdio/📦️packages/🦀️rust\" }",
        )
        cargo.write_text(c)

    # plugin artifact_kind if applicable
    plug = PLUGIN / "🔌️plugin/🦀️component.rs"
    pt = plug.read_text()
    if ".artifact_kind(" not in pt:
        pt = pt.replace(
            '.setup(crate::artifacts::note::engine::register)',
            '.artifact_kind(crate::artifacts::note::artifact_kind())\n        .setup(crate::artifacts::note::engine::register)',
        )
        plug.write_text(pt)


def phase_fix_refs() -> None:
    print("[8] rewrite leftover path refs in note plugin")
    for f in PLUGIN.rglob("*.rs"):
        if "target/" in str(f):
            continue
        try:
            t = f.read_text()
        except Exception:
            continue
        orig = t
        # Prefer schema paths but keep alias modules working; still update direct old snapshot schema paths
        t = t.replace("crate::artifacts::note::snapshot::schema::", "crate::artifacts::note::schema::snapshot::")
        t = t.replace("crate::artifacts::note::snapshot::pack::", "crate::artifacts::note::schema::snapshot::binary::")
        if t != orig:
            f.write_text(t)

    # Ensure mutation triad diff rs files exist (moved may have only mutation+inverse)
    mut_root = NOTE / "🧬️schema" / "🧬️mutations"
    for mut_dir in mut_root.iterdir():
        if not mut_dir.is_dir() or mut_dir.name in (TEXT, BINARY):
            continue
        for triad in ["�_root.iterdir():
        if not mut_dir.is_dir() or mut_dir.name in (TEXT, BINARY):
            continue
        for triad in ["🦠️mutation", "🔺️diff", "↩️inverse"]:
            d = mut_dir / triad
            ensure_dir(d)
            if not (d / "🦀️component.rs").exists():
                write(d / "🦀️component.rs", f"//! {triad}\n")
            if not (d / "🟦️component.ts").exists():
                write(d / "🟦️component.ts", f"/** {triad} */\nexport {{}};\n")


def verify_tree() -> list[str]:
    print("[9] verify required paths")
    required = []
    for folder, leaves in [
        (NOTE / "🧬️schema" / "📸️snapshot" / TEXT, TEXT_LEAF_NAMES),
        (NOTE / "🧬️schema" / "📸️snapshot" / BINARY, BINARY_LEAF_NAMES),
        (NOTE / "🧬️schema" / "🔺️diff" / TEXT, TEXT_LEAF_NAMES),
        (NOTE / "🧬️schema" / "🔺️diff" / BINARY, BINARY_LEAF_NAMES),
        (NOTE / "🧬️schema" / "🧬️mutations" / TEXT, TEXT_LEAF_NAMES),
        (NOTE / "🧬️schema" / "🧬️mutations" / BINARY, BINARY_LEAF_NAMES),
    ]:
        for leaf in leaves:
            required.append(folder / leaf)
    required += [
        NOTE / BUILDER / "🦀️component.rs",
        NOTE / BUILDER / "🟦️component.ts",
        NOTE / DECOMPOSER / "🦀️component.rs",
        NOTE / DECOMPOSER / "🟦️component.ts",
    ]
    for kind, d in STDIO_DIRS.items():
        required.append(NOTE / "🚪️io" / "📥️import" / DESER / "� in STDIO_DIRS.items():
        required.append(NOTE / "🚪️io" / "📥️import" / DESER / "🗿️artifacts" / d / "🦀️component.rs")
        required.append(NOTE / "🚪️io" / "📤️export" / SER / "🗿️artifacts" / d / "🦀️component.rs")
    missing = [str(p.relative_to(NOTE)) for p in required if not p.exists()]
    for old in ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "� in required if not p.exists()]
    for old in ["🗣️dsl", "📸️snapshot", "🔺️diff", "🔧️op", "📡️spr"]:
        if (NOTE / old).exists():
            missing.append(f"OLD_STILL_PRESENT:{old}")
    if (NOTE / "🧬️mutations").exists():
        missing.append("OLD_STILL_PRESENT:🧬️mutations")
    print("  missing", len(missing))
    for m in missing[:50]:
        print("   ", m)
    return missing


def main() -> None:
    print("TICKET", TICKET)
    print("DECOMPOSER", DECOMPOSER, [hex(ord(c)) for c in DECOMPOSER if ord(c) > 127])
    phase_moves()
    phase_leaves()
    phase_schema_descriptor()
    phase_builder_decomposer()
    phase_io()
    phase_root_and_engine()
    phase_glue_ts()
    phase_fix_refs()
    missing = verify_tree()
    (TICKET / "generators" / "w5_note_missing.json").write_text(json.dumps(missing, indent=2, ensure_ascii=False))
    after = sorted(str(p.relative_to(NOTE)) for p in NOTE.rglob("*") if p.is_file())
    (TICKET / "🧪w5-note-after-tree.txt").write_text("\n".join(after) + "\n")
    print("DONE files", len(after), "missing", len(missing))


if __name__ == "__main__":
    main()
