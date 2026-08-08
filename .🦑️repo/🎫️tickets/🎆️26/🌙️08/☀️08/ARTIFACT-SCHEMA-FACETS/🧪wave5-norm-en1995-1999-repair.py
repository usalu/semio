#!/usr/bin/env python3
"""Repair runtime wiring using HEAD snapshot of Document structs (read-only git show)."""
from __future__ import annotations

import re
import subprocess
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
ROOT = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
APPS = REPO / "✏️s/🔌️plugins/📕️norm/🎛️apps"

ARTIFACTS = [
    ("en1995", "En1995"),
    ("en1996", "En1996"),
    ("en1997", "En1997"),
    ("en1998", "En1998"),
    ("en1999", "En1999"),
]


def git_show(path: str) -> str:
    r = subprocess.run(
        ["git", "-C", str(REPO), "show", f"HEAD:{path}"],
        capture_output=True,
        text=True,
    )
    if r.returncode != 0:
        raise RuntimeError(r.stderr)
    return r.stdout


def extract_block(src: str, start: str, end: str) -> str:
    i = src.find(start)
    j = src.find(end, i)
    if i < 0 or j < 0:
        return ""
    return src[i : j + len(end)]


def snapshot_tail(key: str, prefix: str) -> str:
    rel = f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️{key}/🦀️component.rs"
    src = git_show(rel)
    codec = extract_block(src, "//#region 🔖️HandcraftedDocumentCodecs", "//#endregion 🔖️HandcraftedDocumentCodecs")
    default = re.search(r"impl Default for Document \{.*?\n\}\n", src, re.S)
    codec = codec.replace("Document", prefix + "Snapshot")
    default_text = default.group(0).replace("Document", prefix + "Snapshot") if default else ""
    return codec + "\n\n\n" + default_text


def parse_snapshot_fields(snap_rs: str) -> list[str]:
    m = re.search(rf"pub struct En\d{{4}}Snapshot \{{(.*?)\n\}}", snap_rs, re.S)
    if not m:
        m = re.search(r"pub struct \w+Snapshot \{(.*?)\n\}", snap_rs, re.S)
    fields = []
    for line in m.group(1).splitlines():
        line = line.strip()
        if line.startswith("pub "):
            fields.append(re.match(r"pub (\w+):", line).group(1))
    return fields


def write_diff_runtime(key: str, prefix: str, fields: list[str]) -> None:
    apply_snap = "\n        ".join(
        f"if let Some(value) = self.{f} {{ next.{f} = value; }}" for f in fields
    )
    apply_art = "\n        ".join(
        f"if let Some(value) = &self.{f} {{ next.{f} = value.clone(); }}" for f in fields
    )
    takes = "\n        ".join(f"take!({f});" for f in fields)
    path = ROOT / f"📘️{key}" / "🔺️diff/🦀️component.rs"
    path.write_text(
        f"""//! 🔺️ {prefix} artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::{key}::schema::{prefix}Artifact;
use crate::artifacts::{key}::{prefix}Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl {prefix}Diff {{
    pub fn apply_to_artifact(&self, artifact: &{prefix}Artifact) -> {prefix}Artifact {{
        if let Some(replacement) = &self.artifact {{
            return (**replacement).clone();
        }}
        let mut next = artifact.clone();
{apply_art}
        if let Some(value) = &self.selected_check_index {{
            next.selected_check_index = *value;
        }}
        next
    }}
}}

impl MutationDiff<{prefix}Snapshot> for {prefix}Diff {{
    fn apply(&self, snapshot: &{prefix}Snapshot) -> {prefix}Snapshot {{
        if let Some(replacement) = &self.artifact {{
            return replacement.to_snapshot();
        }}
        let mut next = snapshot.clone();
{apply_snap}
        next
    }}

    fn absorb(&mut self, other: Self) {{
        if other.artifact.is_some() {{
            *self = other;
            return;
        }}
        macro_rules! take {{
            ($field:ident) => {{
                if other.$field.is_some() {{
                    self.$field = other.$field;
                }}
            }};
        }}
        {takes}
        take!(selected_check_index);
    }}
}}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &{prefix}Snapshot) -> {prefix}Diff {{
    {prefix}Diff {{
        artifact: Some(Box::new({prefix}Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }}
}}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;
    use crate::artifacts::{key}::mutations::{prefix}Mutation;
    use protocol::{{Mutation as _, MutationDiff}};

    #[test]
    fn set_snapshot_diff_replaces_the_whole_snapshot() {{
        let base = {prefix}Snapshot::default();
        let mutation = {prefix}Mutation::SetSnapshot {{ snapshot: {prefix}Snapshot::default() }};
        let diff = mutation.diff(&base);
        assert_eq!(diff.apply(&base), {prefix}Snapshot::default());
    }}
}}
//#endregion 🧪️Tests
"""
    )


def patch_engine(key: str, prefix: str) -> None:
    path = ROOT / f"📘️{key}" / "⚙️engine/🦀️component.rs"
    text = path.read_text()
    text = re.sub(
        r"impl crate::document::NormFamily for \w+ \{.*?\n\}",
        f"""impl crate::document::NormFamily for {prefix}Family {{
    type Document = crate::artifacts::{key}::{prefix}Snapshot;
    type Mutation = crate::artifacts::{key}::mutations::{prefix}Mutation;

    fn family_id() -> crate::document::NormFamilyId {{
        crate::document::NormFamilyId::{prefix}
    }}

    fn evaluate(document: &Self::Document) -> crate::document::CheckReport {{
        super::evaluate(document)
    }}
}}""",
        text,
        count=1,
        flags=re.S,
    )
    engine_block = f"""//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent {prefix} artifact engine — owns the full artifact; `snapshot()` is persisted only.
pub struct {prefix}Engine {{
    artifact: crate::artifacts::{key}::schema::{prefix}Artifact,
    snapshot: crate::artifacts::{key}::{prefix}Snapshot,
}}

impl {prefix}Engine {{
    pub fn new(snapshot: crate::artifacts::{key}::{prefix}Snapshot) -> Self {{
        let artifact = crate::artifacts::{key}::schema::{prefix}Artifact::from_snapshot(snapshot.clone());
        Self {{ artifact, snapshot }}
    }}

    pub fn into_snapshot(self) -> crate::artifacts::{key}::{prefix}Snapshot {{
        self.snapshot
    }}
}}

impl protocol::ArtifactEngine for {prefix}Engine {{
    type Artifact = crate::artifacts::{key}::schema::{prefix}Artifact;
    type Snapshot = crate::artifacts::{key}::{prefix}Snapshot;
    type Mutation = crate::artifacts::{key}::mutations::{prefix}Mutation;
    type Diff = crate::artifacts::{key}::diff::{prefix}Diff;

    fn artifact(&self) -> &Self::Artifact {{
        &self.artifact
    }}

    fn snapshot(&self) -> &Self::Snapshot {{
        &self.snapshot
    }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }}
}}
//#endregion 🔖️ArtifactEngine"""
    text = re.sub(
        r"//#region 🔖️ArtifactEngine.*?//#endregion 🔖️ArtifactEngine",
        engine_block,
        text,
        count=1,
        flags=re.S,
    )
    text = text.replace("Document", f"{prefix}Snapshot")
    text = text.replace(f"{prefix}SnapshotDiff", f"{prefix}Diff")
    path.write_text(text)


def restore_en1996_types() -> None:
    src = git_show("✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🦀️component.rs")
    types = src.split("pub struct Document")[0]
    types = types.replace("// #region 🔖️Types", "//#region 🔖️Types").rstrip()
    root = ROOT / "📘️en1996/🦀️component.rs"
    tail = root.read_text().split("pub use crate::artifacts::en1996")[1]
    root.write_text(types + "\n\npub use crate::artifacts::en1996" + tail)


def fix_artifact_default(key: str, prefix: str) -> None:
    p = ROOT / f"📘️{key}" / "🧬️schema/🦀️component.rs"
    sch = p.read_text()
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
    p.write_text(sch)


def fix_apps(key: str, prefix: str) -> None:
    app = APPS / f"📘️{key}"
    if not app.exists():
        return
    for p in app.rglob("*.rs"):
        t = p.read_text()
        t = t.replace("Document", f"{prefix}Snapshot")
        t = t.replace("type Projection", "type Snapshot")
        t = t.replace("initial_projection", "initial_snapshot")
        t = t.replace("doc.projection", "doc.snapshot")
        t = t.replace("cfg.projection", "cfg.snapshot")
        t = t.replace("set_document::", "set_snapshot::")
        t = t.replace("set-document", "set-snapshot")
        t = t.replace("setDocument", "setSnapshot")
        t = t.replace(f"{prefix}SnapshotView", "DocumentView")
        p.write_text(t)
    for p in app.rglob("*.ts"):
        t = p.read_text().replace("Document", f"{prefix}Snapshot")
        p.write_text(t)


def main() -> None:
    restore_en1996_types()
    for key, prefix in ARTIFACTS:
        snap_path = ROOT / f"📘️{key}" / "📸️snapshot/🧬️schema/🦀️component.rs"
        snap = snap_path.read_text().split("//#region 🔖️Handcrafted")[0].rstrip()
        snap_path.write_text(snap + "\n\n" + snapshot_tail(key, prefix))
        fields = parse_snapshot_fields(snap_path.read_text())
        write_diff_runtime(key, prefix, fields)
        patch_engine(key, prefix)
        fix_artifact_default(key, prefix)
        fix_apps(key, prefix)
        for rel in (
            "📸️snapshot/🎒️pack/🦀️component.rs",
            "🗣️dsl/🦀️component.rs",
            "📡️spr/🦀️component.rs",
            "🔧️op/🦀️component.rs",
        ):
            p = ROOT / f"📘️{key}" / rel
            if p.exists():
                p.write_text(p.read_text().replace("Document", f"{prefix}Snapshot"))
        print("repaired", key)


if __name__ == "__main__":
    main()
