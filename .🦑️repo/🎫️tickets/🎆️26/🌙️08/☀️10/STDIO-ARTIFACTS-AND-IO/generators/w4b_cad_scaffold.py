#!/usr/bin/env python3
"""W4b cad/cloud: scaffold step/ifc/las/gltf from txt/json/binary trees."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]

SRC = {
    "txt": PLUGIN / "🗿️artifacts" / ROSTER["txt"]["dir"],
    "binary": PLUGIN / "🗿️artifacts" / ROSTER["binary"]["dir"],
    "json": PLUGIN / "🗿️artifacts" / ROSTER["json"]["dir"],
}

SRC_META = {
    "txt": ("Txt", "txt", "📄txt", "📄example.txt"),
    "binary": ("Binary", "binary", "💾️binary", "📄example.bin"),
    "json": ("Json", "json", "🔣️json", "🔣️example.json"),
}

ARTIFACTS = [
    ("step", ROSTER["step"]["dir"], "Step", "step", "example.step", "txt", "txt_io"),
    ("ifc", ROSTER["ifc"]["dir"], "Ifc", "ifc", "example.ifc", "txt", "txt_io"),
    ("las", ROSTER["las"]["dir"], "Las", "las", "example.las", "binary", "binary_io"),
    ("gltf", ROSTER["gltf"]["dir"], "Gltf", "gltf", "example.gltf", "json", "json_io"),
]


def replace_from_src(text: str, mid: str, emoji_dir: str, name: str, src_key: str) -> str:
    sname, smid, semoji, _ = SRC_META[src_key]
    pairs = [
        (semoji, emoji_dir),
        (f"stdio.{smid}", f"stdio.{mid}"),
        (f"s.stdio.{smid}", f"s.stdio.{mid}"),
        (f"s.stdio.{smid}.diff", f"s.stdio.{mid}.diff"),
        (f"{sname}Artifact", f"{name}Artifact"),
        (f"{sname}Snapshot", f"{name}Snapshot"),
        (f"{sname}Diff", f"{name}Diff"),
        (f"{sname}Mutation", f"{name}Mutation"),
        (f"{sname}Engine", f"{name}Engine"),
        (f"{sname}Builder", f"{name}Builder"),
        (f"{sname}Decomposer", f"{name}Decomposer"),
        (f"{sname}Parts", f"{name}Parts"),
        (f"{smid}_artifact_schema_descriptor", f"{mid}_artifact_schema_descriptor"),
        (f"apply_{smid}_mutation", f"apply_{mid}_mutation"),
        (f"empty_{smid}_snapshot", f"empty_{smid}_snapshot".replace(smid, mid)),
        (f"STDIO_{smid.upper()}_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        (f"{smid.upper()}_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        (f"crate::artifacts::{smid}", f"crate::artifacts::{mid}"),
        (f"artifacts::{smid}", f"artifacts::{mid}"),
        (f"pub mod {smid}", f"pub mod {mid}"),
        (sname, name),
        (smid, mid),
    ]
    if src_key == "json":
        pairs.extend([
            ("serde_json::Value", "PLACEHOLDER_VALUE_TYPE"),
            ("pub value:", "PLACEHOLDER_PUB_VALUE"),
            (" value:", " PLACEHOLDER_VALUE_COLON"),
            ("from.value", "PLACEHOLDER_VALUE_REF"),
            ("self.value", "PLACEHOLDER_SELF_VALUE"),
        ])
    if src_key == "txt":
        pairs.extend([
            ("pub text:", "PLACEHOLDER_PUB_TEXT"),
            (" text:", " PLACEHOLDER_TEXT_COLON"),
            ("from.text", "PLACEHOLDER_TEXT_REF"),
            ("self.text", "PLACEHOLDER_SELF_TEXT"),
        ])
    if src_key == "binary":
        pairs.extend([
            ("pub bytes:", "PLACEHOLDER_PUB_BYTES"),
            (" bytes:", " PLACEHOLDER_BYTES_COLON"),
        ])
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def add_json_txt_io(dst: Path, mid: str, name: str) -> None:
    j_imp = SRC["json"] / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt"
    j_exp = SRC["json"] / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt"
    for src_io, rel in [
        (j_imp, "📥️import/🧩️deserializers/🗿️artifacts/📄txt"),
        (j_exp, "📤️export/🧵️serializers/🗿️artifacts/📄txt"),
    ]:
        dst_io = dst / "🚪️io" / rel
        dst_io.mkdir(parents=True, exist_ok=True)
        for leaf in ("🦀️component.rs", "🟦️component.ts"):
            raw = (src_io / leaf).read_text(encoding="utf-8")
            raw = raw.replace("json", mid).replace("Json", name)
            (dst_io / leaf).write_text(raw, encoding="utf-8")


def scaffold_one(
    mid: str, emoji_dir: str, name: str, ext: str, example_file: str, src_key: str, io_kind: str
) -> None:
    src_dir = SRC[src_key]
    dst = PLUGIN / "🗿️artifacts" / emoji_dir
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src_dir, dst)
    _, _, _, old_example = SRC_META[src_key]
    for path in dst.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix in (
            ".rs", ".ts", ".json", ".semio", ".proto", ".graphql",
            ".ebnf", ".g4", ".abnf", ".ksy", ".spicy",
        ):
            path.write_text(
                replace_from_src(path.read_text(encoding="utf-8"), mid, emoji_dir, name, src_key),
                encoding="utf-8",
            )
    assets = dst / "📚️examples/🎬️demo/🖼️assets"
    old_asset = assets / old_example.split("/")[-1]
    if old_asset.exists():
        old_asset.rename(assets / example_file)

    if io_kind == "txt_io":
        add_json_txt_io(dst, mid, name)
    elif io_kind == "json_io":
        pass
    elif io_kind == "binary_io":
        pass


for row in ARTIFACTS:
    scaffold_one(*row)
    print("scaffolded", row[1])
