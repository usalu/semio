#!/usr/bin/env python3
"""🧪️ W2-W norm-1: regenerates the `🔖️Apply` region of `En1991Diff` from the diff struct itself, so `apply_to_artifact`,
`MutationDiff::apply` and `absorb` carry every field (22 fields — thermal, fire, LM3/LM4, load group, storey count — were
silently dropped, found by the committed wire witnesses).

    python3 🧪️w2-w-norm-1-diff-apply.py
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SCHEMA = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"
STRUCT = SCHEMA / "🦀️.rs"
APPLY = SCHEMA / "📝️text/🦀️.rs"


def fields():
    """📋️ `(name, type)` of every `En1991Diff` field except the whole-artifact replacement, in declaration order."""
    body = STRUCT.read_text(encoding="utf-8").split("pub struct En1991Diff {", 1)[1].split("\n}", 1)[0]
    return [(name, kind) for name, kind in re.findall(r"pub (\w+): Option<(.+?)>,", body) if name != "artifact"]


def assign(name, kind):
    """✍️ One field's write into `next`."""
    if kind.startswith("En1991") and kind.endswith("List"):
        return f"        if let Some(list) = &self.{name} {{ next.{name} = list.values.clone(); }}\n"
    if kind == "String":
        return f"        if let Some(value) = &self.{name} {{ next.{name} = value.clone(); }}\n"
    return f"        if let Some(value) = &self.{name} {{ next.{name} = *value; }}\n"


def region():
    """🔖️ The whole `🔖️Apply` region."""
    writes = "".join(assign(name, kind) for name, kind in fields())
    absorbs = "".join(f"        if other.{name}.is_some() {{ self.{name} = other.{name}; }}\n" for name, _ in [("artifact", None)] + fields())
    return (
        "//#region 🔖️Apply\n"
        "impl En1991Diff {\n"
        "    pub fn apply_to_artifact(&self, artifact: &En1991Artifact) -> protocol::MutationApplyResult<En1991Artifact> {\n"
        "        if let Some(replacement) = &self.artifact {\n"
        "            return Ok((**replacement).clone());\n"
        "        }\n"
        "        let mut next = artifact.clone();\n"
        f"{writes}"
        "        Ok(next)\n"
        "    }\n"
        "}\n"
        "\n"
        "impl MutationDiff<En1991Snapshot> for En1991Diff {\n"
        "    fn apply(&self, snapshot: &En1991Snapshot) -> protocol::MutationApplyResult<En1991Snapshot> {\n"
        "        if let Some(replacement) = &self.artifact {\n"
        "            return Ok(replacement.to_snapshot());\n"
        "        }\n"
        "        let mut next = snapshot.clone();\n"
        f"{writes}"
        "        Ok(next)\n"
        "    }\n"
        "\n"
        "    fn absorb(&mut self, other: Self) {\n"
        f"{absorbs}"
        "    }\n"
        "}\n"
        "//#endregion 🔖️Apply"
    )


if __name__ == "__main__":
    source = APPLY.read_text(encoding="utf-8")
    start, end = source.index("//#region 🔖️Apply"), source.index("//#endregion 🔖️Apply") + len("//#endregion 🔖️Apply")
    APPLY.write_text(source[:start] + region() + source[end:], encoding="utf-8")
    print(len(fields()), "fields")
