#!/usr/bin/env python3
"""Append step/ifc/las/gltf modules to glue.rs, plugin, TS index."""
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

SPECS = [
    ("step", ROSTER["step"]["dir"], "txt"),
    ("ifc", ROSTER["ifc"]["dir"], "txt"),
    ("las", ROSTER["las"]["dir"], "binary"),
    ("gltf", ROSTER["gltf"]["dir"], "json"),
]

glue_text = GLUE.read_text(encoding="utf-8")
if "pub mod step" in glue_text:
    print("glue already has step")
else:
    start = glue_text.find("    #[path = \".\"]\n    pub mod md {")
    if start < 0:
        start = glue_text.find("    pub mod md {")
    if start < 0:
        raise SystemExit("md block missing")
    depth = 0
    end = start
    for i, ch in enumerate(glue_text[start:], start):
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break
    template = glue_text[start:end] + "\n"
    blocks = []
    for mid, emoji_dir, io_parent in SPECS:
        block = template.replace("pub mod md", f"pub mod {mid}")
        if not block.lstrip().startswith("#[path"):
            block = "    #[path = \".\"]\n" + block.lstrip()
        block = block.replace("📝️md", emoji_dir)
        block = block.replace("/md/", f"/{mid}/")
        block = block.replace("artifacts::md", f"artifacts::{mid}")
        if io_parent == "binary":
            block = block.replace("deserializers::artifacts::txt", "deserializers::artifacts::binary")
            block = block.replace("serializers::artifacts::txt", "serializers::artifacts::binary")
            block = block.replace("📄txt/🦀️", "💾️binary/🦀️")
            block = block.replace("📄txt/🟦️", "💾️binary/🟦️")
        elif io_parent == "json":
            block = block.replace("deserializers::artifacts::txt", "deserializers::artifacts::json")
            block = block.replace("serializers::artifacts::txt", "serializers::artifacts::json")
            block = block.replace("📄txt/🦀️", "🔣️json/🦀️")
            block = block.replace("📄txt/🟦️", "🔣️json/🟦️")
        blocks.append(block)
    marker = "\n}\n//#endregion Artifacts"
    idx = glue_text.rfind(marker)
    if idx < 0:
        raise SystemExit("artifacts end marker missing")
    glue_text = glue_text[:idx] + "\n" + "".join(blocks) + glue_text[idx:]
    GLUE.write_text(glue_text, encoding="utf-8")
    print("glue updated")

plugin_text = PLUGIN_RS.read_text(encoding="utf-8")
for mid, _, _ in SPECS:
    reg = f"    crate::artifacts::{mid}::engine::register();"
    kind = f"        .artifact_kind(crate::artifacts::{mid}::artifact_kind())"
    if reg not in plugin_text:
        plugin_text = plugin_text.replace(
            "    crate::artifacts::zip::engine::register();",
            f"    crate::artifacts::zip::engine::register();\n{reg}",
        )
    if kind not in plugin_text:
        plugin_text = plugin_text.replace(
            "        .artifact_kind(crate::artifacts::zip::artifact_kind())",
            f"        .artifact_kind(crate::artifacts::zip::artifact_kind())\n{kind}",
        )
PLUGIN_RS.write_text(plugin_text, encoding="utf-8")

index_lines = INDEX_TS.read_text(encoding="utf-8").strip().splitlines()
for mid, emoji_dir, _ in SPECS:
    export_line = f'export * as {mid} from "../../🗿️artifacts/{emoji_dir}/🟦️component.ts";'
    if export_line not in index_lines:
        index_lines.append(export_line)
INDEX_TS.write_text("\n".join(index_lines) + "\n", encoding="utf-8")
print("plugin and index updated")
