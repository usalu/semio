#!/usr/bin/env python3
"""W4b: scaffold obj/stl/ply/dxf/svg/bmp from txt/xml/binary reference trees."""
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
    "xml": PLUGIN / "🗿️artifacts" / ROSTER["xml"]["dir"],
    "binary": PLUGIN / "🗿️artifacts" / ROSTER["binary"]["dir"],
    "json": PLUGIN / "🗿️artifacts" / ROSTER["json"]["dir"],
}

ARTIFACTS = [
    ("obj", ROSTER["obj"]["dir"], "Obj", "obj", "example.obj", "txt", None),
    ("ply", ROSTER["ply"]["dir"], "Ply", "ply", "example.ply", "txt", None),
    ("dxf", ROSTER["dxf"]["dir"], "Dxf", "dxf", "example.dxf", "txt", None),
    ("stl", ROSTER["stl"]["dir"], "Stl", "stl", "example.stl", "txt", "add_txt_io"),
    ("svg", ROSTER["svg"]["dir"], "Svg", "svg", "example.svg", "xml", "xml_io"),
    ("bmp", ROSTER["bmp"]["dir"], "Bmp", "bmp", "example.bmp", "binary", None),
]

SRC_META = {
    "txt": ("Txt", "txt", "📄txt", "📄example.txt"),
    "xml": ("Xml", "xml", "📰xml", "example.xml"),
    "binary": ("Binary", "binary", "💾️binary", "📄example.bin"),
}


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
        (f"empty_{smid}_snapshot", f"empty_{mid}_snapshot"),
        (f"STDIO_{smid.upper()}_DOCUMENT_SCHEMA", f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"),
        (f"{smid.upper()}_ARTIFACT_SCHEMA_ID", f"{mid.upper()}_ARTIFACT_SCHEMA_ID"),
        (f"crate::artifacts::{smid}", f"crate::artifacts::{mid}"),
        (f"artifacts::{smid}", f"artifacts::{mid}"),
        (f"pub mod {smid}", f"pub mod {mid}"),
        (sname, name),
        (smid, mid),
    ]
    for old, new in pairs:
        text = text.replace(old, new)
    return text


def scaffold_one(
    mid: str, emoji_dir: str, name: str, ext: str, example_file: str, src_key: str, io_fixup: str | None
) -> Path:
    src_dir = SRC[src_key]
    dst = PLUGIN / "🗿️artifacts" / emoji_dir
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src_dir, dst)
    _, smid, _, old_example = SRC_META[src_key]
    for path in dst.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix in (
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
        ):
            raw = path.read_text(encoding="utf-8")
            path.write_text(replace_from_src(raw, mid, emoji_dir, name, src_key), encoding="utf-8")
    old_asset = dst / "📚️examples/🎬️demo/🖼️assets" / old_example.split("/")[-1]
    if not old_asset.exists():
        for p in (dst / "📚️examples/🎬️demo/🖼️assets").iterdir():
            if p.is_file() and not p.name.endswith(".semio"):
                old_asset = p
                break
    if old_asset.exists():
        old_asset.rename(old_asset.parent / example_file)

    if io_fixup == "add_txt_io":
        j_imp = SRC["json"] / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt"
        j_exp = SRC["json"] / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt"
        for src_io, rel in [(j_imp, "📥️import/🧩️deserializers/🗿️artifacts/📄txt"), (j_exp, "📤️export/🧵️serializers/🗿️artifacts/📄txt")]:
            dst_io = dst / "🚪️io" / rel
            dst_io.mkdir(parents=True, exist_ok=True)
            for leaf in ("🦀️component.rs", "🟦️component.ts"):
                raw = (src_io / leaf).read_text(encoding="utf-8")
                raw = raw.replace("json", mid).replace("Json", name)
                (dst_io / leaf).write_text(raw, encoding="utf-8")

    if io_fixup == "xml_io":
        txt_io = dst / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt"
        xml_io = dst / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml"
        if txt_io.exists():
            shutil.move(str(txt_io), str(xml_io))
        txt_io_s = dst / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt"
        xml_io_s = dst / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml"
        if txt_io_s.exists():
            shutil.move(str(txt_io_s), str(xml_io_s))
        for p in dst.rglob("*"):
            if p.suffix in (".rs", ".ts") and "🚪️io" in str(p):
                raw = p.read_text(encoding="utf-8")
                raw = raw.replace("artifacts::txt::", "artifacts::xml::")
                raw = raw.replace("TxtSnapshot", "XmlSnapshot")
                raw = raw.replace("STDIO_TXT_DOCUMENT_SCHEMA", "STDIO_XML_DOCUMENT_SCHEMA")
                raw = raw.replace("from.text", "from.doc")
                raw = raw.replace("mesh_placeholder", "doc")
                p.write_text(raw, encoding="utf-8")

    return dst


for mid, emoji, name, ext, ex, src_key, io_fixup in ARTIFACTS:
    scaffold_one(mid, emoji, name, ext, ex, src_key, io_fixup)
    print("scaffolded", emoji)
