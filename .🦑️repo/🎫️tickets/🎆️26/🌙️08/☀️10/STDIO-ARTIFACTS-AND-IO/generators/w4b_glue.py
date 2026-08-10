#!/usr/bin/env python3
"""Append w4b artifact modules to glue.rs, plugin, TS index."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]
GLUE = PLUGIN / "📦️packages/🦀️rust/📦️glue.rs"
PLUGIN_RS = PLUGIN / "🔌️plugin/🦀️component.rs"
INDEX_TS = PLUGIN / "📦️packages/🟦️typescript/📦️index.ts"

TXT_IO_ARTIFACTS = [
    ("obj", ROSTER["obj"]["dir"]),
    ("ply", ROSTER["ply"]["dir"]),
    ("dxf", ROSTER["dxf"]["dir"]),
]
STL = ("stl", ROSTER["stl"]["dir"])
SVG = ("svg", ROSTER["svg"]["dir"])
BMP = ("bmp", ROSTER["bmp"]["dir"])

glue_text = GLUE.read_text(encoding="utf-8")


def extract_mod_block(text: str, mod_name: str) -> str:
    needle = f"    pub mod {mod_name} {{"
    start = text.index(needle)
    depth = 0
    for i in range(start, len(text)):
        ch = text[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[start : i + 1] + "\n"
    raise SystemExit(f"unbalanced mod {mod_name}")


csv_tpl = extract_mod_block(glue_text, "csv")
zip_tpl = extract_mod_block(glue_text, "zip")
footer = "\n}\n//#endregion Artifacts"
if footer not in glue_text:
    raise SystemExit("glue footer missing")


def block_from_csv(mid: str, emoji: str) -> str:
    b = csv_tpl.replace("pub mod csv", f"pub mod {mid}")
    b = b.replace("📊️csv", emoji)
    b = b.replace("/csv/", f"/{mid}/")
    return b


def block_from_zip_stl(mid: str, emoji: str) -> str:
    b = zip_tpl.replace("pub mod zip", f"pub mod {mid}")
    b = b.replace("🎒️zip", emoji)
    b = b.replace("/zip/", f"/{mid}/")
    b = b.replace("pub mod deflate", "pub mod txt")
    b = b.replace("🗜️deflate", "📄txt")
    b = b.replace("/deflate/", "/txt/")
    return b


def block_from_csv_svg(mid: str, emoji: str) -> str:
    b = csv_tpl.replace("pub mod csv", f"pub mod {mid}")
    b = b.replace("📊️csv", emoji)
    b = b.replace("/csv/", f"/{mid}/")
    b = b.replace("pub mod txt", "pub mod xml")
    b = b.replace("📄txt", "📰xml")
    b = b.replace("/txt/", "/xml/")
    return b


def block_from_deflate_bmp(mid: str, emoji: str) -> str:
    b = extract_mod_block(glue_text, "deflate")
    b = b.replace("pub mod deflate", f"pub mod {mid}")
    b = b.replace("🗜️deflate", emoji)
    b = b.replace("/deflate/", f"/{mid}/")
    return b


new_blocks = []
for mid, emoji in TXT_IO_ARTIFACTS:
    if f"pub mod {mid}" not in glue_text:
        new_blocks.append(block_from_csv(mid, emoji))

mid, emoji = STL
if f"pub mod {mid}" not in glue_text:
    new_blocks.append(block_from_zip_stl(mid, emoji))

mid, emoji = SVG
if f"pub mod {mid}" not in glue_text:
    new_blocks.append(block_from_csv_svg(mid, emoji))

mid, emoji = BMP
if f"pub mod {mid}" not in glue_text:
    new_blocks.append(block_from_deflate_bmp(mid, emoji))

if new_blocks:
    glue_text = glue_text.replace(footer, "".join(new_blocks) + footer)
    GLUE.write_text(glue_text, encoding="utf-8")
    print("glue updated", len(new_blocks), "blocks")

plugin_text = PLUGIN_RS.read_text(encoding="utf-8")
mids = ["obj", "stl", "ply", "dxf", "svg", "bmp"]
for mid in mids:
    reg = f"    crate::artifacts::{mid}::engine::register();"
    kind = f"        .artifact_kind(crate::artifacts::{mid}::artifact_kind())"
    if reg not in plugin_text:
        plugin_text = plugin_text.replace(
            "    crate::artifacts::deflate::engine::register();",
            "    crate::artifacts::deflate::engine::register();\n" + reg,
        )
    if kind not in plugin_text:
        plugin_text = plugin_text.replace(
            "        .artifact_kind(crate::artifacts::deflate::artifact_kind())",
            "        .artifact_kind(crate::artifacts::deflate::artifact_kind())\n" + kind,
        )
PLUGIN_RS.write_text(plugin_text, encoding="utf-8")

index_lines = INDEX_TS.read_text(encoding="utf-8").strip().splitlines()
for mid in mids:
    emoji = ROSTER[mid]["dir"]
    line = f'export * as {mid} from "../../🗿️artifacts/{emoji}/🟦️component.ts";'
    if line not in index_lines:
        index_lines.append(line)
INDEX_TS.write_text("\n".join(index_lines) + "\n", encoding="utf-8")
print("plugin and index updated")
