#!/usr/bin/env python3
"""W4a: scaffold xml/csv/md artifacts from json reference."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
SRC_DIR = PLUGIN / "🗿️artifacts" / ROSTER["json"]["dir"]

ARTIFACTS = [
    ("xml", "📰xml", "Xml", "xml", "example.xml"),
    ("csv", "📊️csv", "Csv", "csv", "example.csv"),
    ("md", "📝️md", "Md", "md", "example.md"),
]


def replace_all(text: str, mid: str, emoji_dir: str, name: str) -> str:
    pairs = [
        ("🔣️json", emoji_dir),
        ("stdio.json", f"stdio.{mid}"),
        ("s.stdio.json", f"s.stdio.{mid}"),
        ("s.stdio.json.diff", f"s.stdio.{mid}.diff"),
        ("JsonArtifact", f"{name}Artifact"),
        ("JsonSnapshot", f"{name}Snapshot"),
        ("JsonDiff", f"{name}Diff"),
        ("JsonMutation", f"{name}Mutation"),
        ("JsonEngine", f"{name}Engine"),
        ("JsonBuilder", f"{name}Builder"),
        ("JsonDecomposer", f"{name}Decomposer"),
        ("JsonParts", f"{name}Parts"),
        ("json_artifact_schema_descriptor", f"{mid}_artifact_schema_descriptor"),
        ("apply_json_mutation", f"apply_{mid}_mutation"),
        ("empty_json_snapshot", f"empty_{mid}_snapshot"),
        ("STDIO_JSON_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        ("JSON_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        ("crate::artifacts::json", f"crate::artifacts::{mid}"),
        ("artifacts::json", f"artifacts::{mid}"),
        ("pub mod json", f"pub mod {mid}"),
        ("serde_json::Value", "PLACEHOLDER_VALUE_TYPE"),
        ("&from.value", "PLACEHOLDER_VALUE_REF"),
        ("from.value", "PLACEHOLDER_VALUE_FIELD"),
        ("self.value", "PLACEHOLDER_SELF_VALUE"),
        ("pub value:", "PLACEHOLDER_PUB_VALUE"),
        (" value:", " PLACEHOLDER_VALUE_COLON"),
        ("Json", name),
        ("json", mid),
    ]
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def scaffold_one(mid: str, emoji_dir: str, name: str, ext: str, example_file: str) -> Path:
    dst = PLUGIN / "🗿️artifacts" / emoji_dir
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(SRC_DIR, dst)
    for path in dst.rglob("*"):
        if path.is_file():
            if path.suffix in (".rs", ".ts", ".json", ".semio", ".proto", ".graphql", ".ebnf", ".g4", ".abnf", ".ksy", ".spicy"):
                raw = path.read_text(encoding="utf-8")
                path.write_text(replace_all(raw, mid, emoji_dir, name), encoding="utf-8")
    old_example = dst / "📚️examples/🎬️demo/🖼️assets/🔣️example.json"
    if old_example.exists():
        new_example = old_example.parent / example_file
        old_example.rename(new_example)
    return dst


for mid, emoji, name, ext, ex in ARTIFACTS:
    scaffold_one(mid, emoji, name, ext, ex)
    print("scaffolded", emoji)
