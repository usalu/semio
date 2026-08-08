#!/usr/bin/env python3
"""Wire snapshot codecs, slim artifact roots, mutations, pack, dsl references."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
APPS = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎛️apps")

ARTIFACTS = [
    ("en1995", "En1995", "🪵️", "EN 1995"),
    ("en1996", "En1996", "🧱️", "EN 1996"),
    ("en1997", "En1997", "🌍️", "EN 1997"),
    ("en1998", "En1998", "🌋️", "EN 1998"),
    ("en1999", "En1999", "✨️", "EN 1999"),
]


def extract_codecs_and_default(src: str, prefix: str) -> str:
    text = src.replace("Document", prefix + "Snapshot")
    text = text.replace(prefix + "SnapshotDsl", "DocumentDsl")  # fix over-replace
    text = text.replace("DocumentDsl", "DocumentDsl")
    m_default = re.search(
        r"impl Default for " + prefix + r"Snapshot \{.*?\n\}\n",
        text,
        re.S,
    )
    m_codec = re.search(
        r"//#region 🔖️HandcraftedDocumentCodecs.*?//#endregion 🔖️HandcraftedDocumentCodecs",
        text,
        re.S,
    )
    parts = []
    if m_codec:
        parts.append(m_codec.group(0))
    if m_default:
        parts.append(m_default.group(0))
    return "\n\n\n".join(parts)


def slim_root(key: str, prefix: str, emoji: str, label: str, keep_preamble: str) -> str:
    return f"""//! {emoji} {label} artifact root — snapshot re-export and facet modules.

{keep_preamble}
pub use crate::artifacts::{key}::snapshot::schema::{prefix}Snapshot;

#[path = "./🧬️schema/🦀️component.rs"]
pub mod schema;

pub mod snapshot {{
    #[path = "./📸️snapshot/🧬️schema/🦀️component.rs"]
    pub mod schema;
    #[path = "./📸️snapshot/🎒️pack/🦀️component.rs"]
    pub mod pack;
}}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {{
    crate::app_surface::artifact_kind_spec("{key}", "{label}")
}}
"""


def preamble_en1996(src: str) -> str:
    m = re.search(r"^//!.*?\n\n(.*?)(# //region 🔖️Types|// #region 🔖️Types)", src, re.S)
    if not m:
        return ""
    block = m.group(1)
    # keep through part_2 module and before Document struct
    end = block.find("pub struct Document")
    if end > 0:
        block = block[:end]
    return block.rstrip() + "\n\n"


def wire_snapshot(key: str, prefix: str) -> None:
    art = ROOT / f"📘️{key}" / "🦀️component.rs"
    src = art.read_text()
    snap_path = ROOT / f"📘️{key}" / "📸️snapshot/🧬️schema/🦀️component.rs"
    snap = snap_path.read_text()
    extra = extract_codecs_and_default(src, prefix)
    if extra and "impl store::DocumentDsl" not in snap:
        snap_path.write_text(snap.rstrip() + "\n\n" + extra + "\n")
    # fix artifact default
    schema_path = ROOT / f"📘️{key}" / "🧬️schema/🦀️component.rs"
    sch = schema_path.read_text()
    sch = sch.replace(
        f"            ..crate::artifacts::{key}::{prefix}Snapshot::default().into()",
        f"            selected_check_index: None,\n            ..{prefix}Artifact::from_snapshot({prefix}Snapshot::default())",
    )
    sch = re.sub(
        rf"impl Default for {prefix}Artifact \{{[^}}]+\}}",
        f"""impl Default for {prefix}Artifact {{
    fn default() -> Self {{
        Self::from_snapshot({prefix}Snapshot::default())
    }}
}}""",
        sch,
        count=1,
    )
    schema_path.write_text(sch)


def wire_root(key: str, prefix: str, emoji: str, label: str) -> None:
    art = ROOT / f"📘️{key}" / "🦀️component.rs"
    src = art.read_text()
    pre = preamble_en1996(src) if key == "en1996" else ""
    art.write_text(slim_root(key, prefix, emoji, label, pre))


def replace_in_tree(tree: Path, mapping: dict[str, str]) -> None:
    for p in tree.rglob("*"):
        if p.is_file() and p.suffix in {".rs", ".ts"}:
            text = p.read_text()
            new = text
            for a, b in mapping.items():
                new = new.replace(a, b)
            if new != text:
                p.write_text(new)


def wire_mutations(key: str, prefix: str) -> None:
    mut = ROOT / f"📘️{key}" / "🧬️mutations/🦀️component.rs"
    content = f"""//! 🧬️ {prefix} artifact — document mutation dispatch.

use crate::artifacts::{key}::diff::{{diff_set_snapshot, {prefix}Diff}};
use crate::artifacts::{key}::{prefix}Snapshot;
use protocol::Mutation;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum {prefix}Mutation {{
    SetSnapshot {{
        #[dsl(block)]
        snapshot: {prefix}Snapshot,
    }},
}}

impl Mutation<{prefix}Snapshot> for {prefix}Mutation {{
    type Diff = {prefix}Diff;

    fn diff(&self, _snapshot: &{prefix}Snapshot) -> {prefix}Diff {{
        match self {{
            {prefix}Mutation::SetSnapshot {{ snapshot }} => diff_set_snapshot(snapshot),
        }}
    }}

    fn inverse(&self, snapshot: &{prefix}Snapshot) -> Vec<Self> {{
        match self {{
            {prefix}Mutation::SetSnapshot {{ .. }} => vec![{prefix}Mutation::SetSnapshot {{ snapshot: snapshot.clone() }}],
        }}
    }}
}}
"""
    mut.write_text(content)
    snap_mut = ROOT / f"📘️{key}" / "🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"
    if snap_mut.exists():
        snap_mut.write_text(
            f"""//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::{key}::{prefix}Snapshot;
use crate::artifacts::{key}::mutations::{prefix}Mutation;

pub fn set_snapshot(snapshot: {prefix}Snapshot) -> {prefix}Mutation {{
    {prefix}Mutation::SetSnapshot {{ snapshot }}
}}

pub fn apply(base: &mut {prefix}Snapshot, replacement: &{prefix}Snapshot) {{
    *base = replacement.clone();
}}
"""
        )
    inv = ROOT / f"📘️{key}" / "🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"
    if inv.exists():
        inv.write_text(
            f"""//! ↩️ Inverse for SetSnapshot on {prefix}.
use crate::artifacts::{key}::mutations::{prefix}Mutation;
use crate::artifacts::{key}::{prefix}Snapshot;

pub fn inverse(base: &{prefix}Snapshot) -> Vec<{prefix}Mutation> {{
    vec![{prefix}Mutation::SetSnapshot {{ snapshot: base.clone() }}]
}}
"""
        )
    diff_frag = ROOT / f"📘️{key}" / "🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"
    if diff_frag.exists():
        diff_frag.write_text(
            f"""//! 🔺️ Diff fragment for SetSnapshot on {prefix}.
pub type Diff = crate::artifacts::{key}::diff::{prefix}Diff;
"""
        )


def main() -> None:
    for key, prefix, emoji, label in ARTIFACTS:
        wire_snapshot(key, prefix)
        wire_root(key, prefix, emoji, label)
        wire_mutations(key, prefix)
        mapping = {
            f"crate::artifacts::{key}::Document": f"crate::artifacts::{key}::{prefix}Snapshot",
            f"artifacts::{key}::Document": f"artifacts::{key}::{prefix}Snapshot",
            f"::Document": f"::{prefix}Snapshot",
            "SetDocumentMutation": f"{prefix}Mutation",
            "SetDocument { document": "SetSnapshot { snapshot",
            "SetDocument { document:": "SetSnapshot { snapshot:",
            "set_document": "set_snapshot",
            "SetDocument": "SetSnapshot",
            "type Document =": f"type {prefix}Snapshot =",
            "DocumentDiff": f"{prefix}Diff",
            "type Diff = crate::document::DocumentDiff": f"pub use crate::artifacts::{key}::diff::{prefix}Diff as Diff;\n//",
            "type Projection =": "type Snapshot =",
            "fn projection(&self)": "fn snapshot(&self)",
            "&self.projection": "&self.snapshot",
            "self.projection": "self.snapshot",
            "projection: Document": f"snapshot: {prefix}Snapshot",
            "into_projection": "into_snapshot",
            "new(projection:": f"new(snapshot:",
            "evaluate(document: &Document)": f"evaluate(document: &{prefix}Snapshot)",
            "type Document = Document": f"type Document = {prefix}Snapshot",
            "NormFamily for": "NormFamily for",
        }
        replace_in_tree(ROOT / f"📘️{key}", mapping)
        app_dir = APPS / f"📘️{key}"
        if app_dir.exists():
            replace_in_tree(app_dir, {
                **mapping,
                f"use crate::artifacts::{key}::Document": f"use crate::artifacts::{key}::{prefix}Snapshot",
                "DocumentView<'_, Document>": f"DocumentView<'_, {prefix}Snapshot>",
                "type Projection = Document": f"type Snapshot = {prefix}Snapshot",
                "initial_projection": "initial_snapshot",
                "doc.projection": "doc.snapshot",
                '"setDocument"': '"setSnapshot"',
                '"set-document"': '"set-snapshot"',
                "setDocument": "setSnapshot",
                "DOCUMENT_SCHEMA": "DOCUMENT_SCHEMA",
            })
        # pack/dsl explicit
        for rel in ("📸️snapshot/🎒️pack/🦀️component.rs", "🗣️dsl/🦀️component.rs", "📡️spr/🦀️component.rs", "🔧️op/🦀️component.rs"):
            p = ROOT / f"📘️{key}" / rel
            if p.exists():
                t = p.read_text().replace("Document", f"{prefix}Snapshot")
                p.write_text(t)
        print("wired", key)


if __name__ == "__main__":
    main()
