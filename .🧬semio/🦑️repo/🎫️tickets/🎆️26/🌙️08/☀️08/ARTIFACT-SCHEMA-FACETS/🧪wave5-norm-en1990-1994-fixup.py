#!/usr/bin/env python3
"""Targeted fixups: codecs, mutations, set-snapshot rename, pack paths."""

from __future__ import annotations

import re
import shutil
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm")
KEYS = ["en1990", "en1991", "en1992", "en1993", "en1994"]
PREFIX = {k: f"En{k[2:]}" for k in KEYS}
FOLDER = {k: f"📘️{k}" for k in KEYS}


def fix_snapshot_codecs(path: Path, key: str) -> None:
    text = path.read_text(encoding="utf-8")
    p = PREFIX[key]
    text = re.sub(rf"store::{p}SnapshotDsl", "store::DocumentDsl", text)
    text = re.sub(rf"store::{p}SnapshotPack", "store::DocumentPack", text)
    text = re.sub(rf"SourceMode::{p}Snapshot", "SourceMode::Document", text)
    text = re.sub(rf"JoinMode::{p}Snapshot", "JoinMode::Document", text)
    text = re.sub(rf"//#region 🔖️Handcrafted{p}SnapshotCodecs", "//#region 🔖️HandcraftedDocumentCodecs", text)
    text = re.sub(rf"//#endregion 🔖️Handcrafted{p}SnapshotCodecs", "//#endregion 🔖️HandcraftedDocumentCodecs", text)
    if "AnnexChoice" in text and "use crate::document" not in text:
        imp = "use crate::document::AnnexChoice;"
        if "ImposedCategory" in text:
            imp = "use crate::document::{AnnexChoice, ImposedCategory};"
        text = text.replace("use serde::{Deserialize, Serialize};", f"{imp}\nuse serde::{{Deserialize, Serialize}};")
    path.write_text(text, encoding="utf-8")


def fix_schema_use(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    text = text.replace("use serde::{{Deserialize, Serialize}};", "use serde::{Deserialize, Serialize};")
    path.write_text(text, encoding="utf-8")


def rename_set_snapshot(art: Path) -> None:
    old, new = art / "🧬️mutations/📤️set-document", art / "🧬️mutations/📄set-snapshot"
    if old.exists() and not new.exists():
        shutil.move(str(old), str(new))


def write_mutations(key: str) -> None:
    snap, pre = f"{PREFIX[key]}Snapshot", PREFIX[key]
    art = ROOT / "🗿️artifacts" / FOLDER[key]
    (art / "🧬️mutations/🦀️component.rs").write_text(
        f"""//! 🧬️ {pre} artifact — snapshot mutation dispatch.

use crate::artifacts::{key}::diff::{{diff_set_snapshot, {pre}Diff}};
use crate::artifacts::{key}::{snap};
use protocol::Mutation;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum {pre}Mutation {{
    SetSnapshot {{ snapshot: {snap} }},
}}

impl Mutation<{snap}> for {pre}Mutation {{
    type Diff = {pre}Diff;

    fn diff(&self, _snapshot: &{snap}) -> {pre}Diff {{
        match self {{
            Self::SetSnapshot {{ snapshot }} => diff_set_snapshot(snapshot),
        }}
    }}

    fn inverse(&self, snapshot: &{snap}) -> Vec<Self> {{
        vec![Self::SetSnapshot {{ snapshot: snapshot.clone() }}]
    }}
}}
//#endregion 🔖️Mutation
""",
        encoding="utf-8",
    )


def patch_engine_pack_paths(path: Path, key: str) -> None:
    text = path.read_text(encoding="utf-8")
    text = text.replace(f"artifacts::{key}::pack::", f"artifacts::{key}::snapshot::pack::")
    text = text.replace(f"artifacts::{key}::{{Document", f"artifacts::{key}::{{{PREFIX[key]}Snapshot")
    text = text.replace(f"use crate::artifacts::{key}::{PREFIX[key]}Snapshot", f"use crate::artifacts::{key}::{{{PREFIX[key]}Snapshot")
    if f"::{PREFIX[key]}QkEntry" in text or key == "en1990":
        text = re.sub(rf"\bQkEntry\b", f"{PREFIX[key]}QkEntry", text)
    text = re.sub(rf"\bDocument\b", f"{PREFIX[key]}Snapshot", text)
    text = text.replace("NormFamily for En", "NormFamily for En")  # no-op guard
    text = text.replace(f"type {PREFIX[key]}Snapshot = {PREFIX[key]}Snapshot", f"type Document = {PREFIX[key]}Snapshot")
    path.write_text(text, encoding="utf-8")


def patch_app_imports(app_dir: Path, key: str) -> None:
    snap = f"{PREFIX[key]}Snapshot"
    for p in app_dir.rglob("*.rs"):
        text = p.read_text(encoding="utf-8")
        new = text.replace(f"::artifacts::{key}::Document", f"::artifacts::{key}::{snap}")
        new = new.replace(f"type Projection = Document;", f"type Projection = {snap};")
        new = new.replace("fn initial_projection() -> Document", f"fn initial_projection() -> {snap}")
        new = new.replace("Document::default()", f"{snap}::default()")
        new = new.replace("&DocumentView<'_, Document>", f"&DocumentView<'_, {snap}>")
        new = new.replace("DocumentView<'_, Document>", f"DocumentView<'_, {snap}>")
        new = new.replace(f"import_media::<Document>", f"import_media::<{snap}>")
        if new != text:
            p.write_text(new, encoding="utf-8")


def patch_artifact_rs_files(art: Path, key: str) -> None:
    snap, pre, diff = f"{PREFIX[key]}Snapshot", PREFIX[key], f"{PREFIX[key]}Diff"
    for p in art.rglob("*.rs"):
        if "🧬️schema" in str(p):
            continue
        if "📸️snapshot/🧬️schema" in str(p):
            continue
        text = p.read_text(encoding="utf-8")
        new = text.replace(f"::artifacts::{key}::Document", f"::artifacts::{key}::{snap}")
        new = new.replace("DocumentDiff<Document>", f"{diff}").replace(f"type Diff = crate::document::DocumentDiff<{snap}>", f"type Diff = crate::artifacts::{key}::diff::{diff}")
        new = new.replace("SetDocumentMutation<Document>", f"{pre}Mutation")
        new = new.replace("SetDocument { document", "SetSnapshot { snapshot")
        new = new.replace(f"artifacts::{key}::pack::", f"artifacts::{key}::snapshot::pack::")
        if new != text:
            p.write_text(new, encoding="utf-8")


def main() -> None:
    for key in KEYS:
        art = ROOT / "🗿️artifacts" / FOLDER[key]
        fix_snapshot_codecs(art / "📸️snapshot/🧬️schema/🦀️component.rs", key)
        fix_schema_use(art / "🧬️schema/🦀️component.rs")
        fix_schema_use(art / "🔺️diff/🧬️schema/🦀️component.rs")
        rename_set_snapshot(art)
        write_mutations(key)
        patch_engine_pack_paths(art / "⚙️engine/🦀️component.rs", key)
        patch_artifact_rs_files(art, key)
        patch_app_imports(ROOT / "🎛️apps" / FOLDER[key], key)
        print("ok", key)


if __name__ == "__main__":
    main()
