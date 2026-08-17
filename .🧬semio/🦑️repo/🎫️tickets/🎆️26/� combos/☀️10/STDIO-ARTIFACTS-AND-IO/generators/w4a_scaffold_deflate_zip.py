#!/usr/bin/env python3
"""W4a: scaffold deflate/zip facet trees from txt reference, then leave codecs for handcraft."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
SRC_DIR = PLUGIN / "🗿️artifacts" / ROSTER["txt"]["dir"]
BINARY_DIR = ROSTER["binary"]["dir"]
DEFLATE_DIR = ROSTER["deflate"]["dir"]
ZIP_DIR = ROSTER["zip"]["dir"]


def replace_pairs(text: str, pairs: list[tuple[str, str]]) -> str:
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def scaffold(mid: str, emoji_dir: str, name: str, ext: str, example_name: str) -> Path:
    dst = PLUGIN / "🗿️artifacts" / emoji_dir
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(SRC_DIR, dst)
    pairs = [
        ("📄txt", emoji_dir),
        ("stdio.txt", f"stdio.{mid}"),
        ("s.stdio.txt", f"s.stdio.{mid}"),
        ("s.stdio.txt.diff", f"s.stdio.{mid}.diff"),
        ("TxtArtifact", f"{name}Artifact"),
        ("TxtSnapshot", f"{name}Snapshot"),
        ("TxtDiff", f"{name}Diff"),
        ("TxtMutation", f"{name}Mutation"),
        ("TxtEngine", f"{name}Engine"),
        ("TxtBuilder", f"{name}Builder"),
        ("TxtDecomposer", f"{name}Decomposer"),
        ("TxtParts", f"{name}Parts"),
        ("txt_artifact_schema_descriptor", f"{mid}_artifact_schema_descriptor"),
        ("apply_txt_mutation", f"apply_{mid}_mutation"),
        ("empty_txt_snapshot", f"empty_{mid}_snapshot"),
        ("STDIO_TXT_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        ("TXT_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        ("crate::artifacts::txt", f"crate::artifacts::{mid}"),
        ("artifacts::txt", f"artifacts::{mid}"),
        ("pub mod txt", f"pub mod {mid}"),
        ("MediaClass::Text", "MediaClass::Data"),
        ("MediaForm::Document", "MediaForm::Value"),
        ("\"Txt\"", f"\"{name}\""),
        ("Txt", name),
        # keep binary dep dir name intact — do mid rename last carefully
        ("txt", mid),
    ]
    for path in dst.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix in {
            ".rs",
            ".ts",
            ".json",
            ".semio",
            ".proto",
            ".graphql",
            ".ebnf",
            ".g4",
            ".abnf",
            ".ksy",
            ".spicy",
            ".dsl.semio",
        } or path.name.endswith(".dsl.semio"):
            raw = path.read_text(encoding="utf-8")
            # protect binary dependency path token during mid rename
            raw = raw.replace(BINARY_DIR, "__BINARY_DIR__")
            raw = replace_pairs(raw, pairs)
            raw = raw.replace("__BINARY_DIR__", BINARY_DIR)
            # fix accidental binary mid collisions from "txt"→mid inside words already handled
            path.write_text(raw, encoding="utf-8")
    old_example = dst / "📚️examples/🎬️demo/🖼️assets/📄example.txt"
    if old_example.exists():
        new_example = old_example.parent / example_name
        old_example.rename(new_example)
        # placeholder bytes written later by handcraft step
        new_example.write_bytes(b"")
    return dst


def add_zip_deflate_io(zip_dst: Path) -> None:
    """Zip depends on binary + deflate — add deflate deser/ser leaves beside binary."""
    for side, leaf in [
        ("📥️import/� contenders", None),
    ]:
        pass
    # import deserializer from deflate
    for direction, folder, leaf_name in [
        ("📥️import", "� contenders", "deserializers"),
    ]:
        pass
    deser_src = zip_dst / "🚪️io/📥️import/� contenders"
    # Use exact emoji dirs from tokens
    deser_bin = zip_dst / f"🚪️io/📥️import/{TOKENS['deserializers']}/🗿️artifacts/{BINARY_DIR}"
    ser_bin = zip_dst / f"🚪️io/📤️export/{TOKENS['serializers']}/🗿️artifacts/{BINARY_DIR}"
    deser_def = zip_dst / f"🚪️io/📥️import/{TOKENS['deserializers']}/🗿️artifacts/{DEFLATE_DIR}"
    ser_def = zip_dst / f"🚪️io/📤️export/{TOKENS['serializers']}/🗿️artifacts/{DEFLATE_DIR}"
    assert deser_bin.is_dir(), deser_bin
    assert ser_bin.is_dir(), ser_bin
    if deser_def.exists():
        shutil.rmtree(deser_def)
    if ser_def.exists():
        shutil.rmtree(ser_def)
    shutil.copytree(deser_bin, deser_def)
    shutil.copytree(ser_bin, ser_def)
    for path in list(deser_def.rglob("*")) + list(ser_def.rglob("*")):
        if path.is_file() and path.suffix in {".rs", ".ts"}:
            text = path.read_text(encoding="utf-8")
            text = text.replace(BINARY_DIR, DEFLATE_DIR)
            text = text.replace("binary", "deflate")
            text = text.replace("Binary", "Deflate")
            text = text.replace("STDIO_DEFLATE_DOCUMENT_SCHEMA", "STDIO_DEFLATE_DOCUMENT_SCHEMA")
            path.write_text(text, encoding="utf-8")


def main() -> None:
    d = scaffold("deflate", DEFLATE_DIR, "Deflate", "zz", "🗜️example.zz")
    print("scaffolded", d)
    z = scaffold("zip", ZIP_DIR, "Zip", "zip", "🎒️example.zip")
    print("scaffolded", z)
    add_zip_deflate_io(z)
    print("added zip↔deflate io leaves")


if __name__ == "__main__":
    main()
