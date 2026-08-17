#!/usr/bin/env python3
"""Append xml/csv/md modules to glue.rs and update plugin + TS index."""
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
    ("xml", ROSTER["xml"]["dir"]),
    ("csv", ROSTER["csv"]["dir"]),
    ("md", ROSTER["md"]["dir"]),
]

glue_text = GLUE.read_text(encoding="utf-8")
json_block_match = re.search(
    r'(    \[path = "\."\]\n    pub mod json \{.*?\n    \}\n)(}\n//#endregion Artifacts)',
    glue_text,
    re.DOTALL,
)
if not json_block_match:
    raise SystemExit("json block not found in glue.rs")
json_template = json_block_match.group(1)

new_blocks = []
for mid, emoji_dir in SPECS:
    block = json_template.replace("pub mod json", f"pub mod {mid}")
    block = block.replace("🔣️json", emoji_dir)
    block = block.replace("/json/", f"/{mid}/")
    block = block.replace("artifacts::json", f"artifacts::{mid}")
    new_blocks.append(block)

if "pub mod xml" in glue_text:
    print("glue already has xml")
else:
    insertion = "".join(new_blocks) + json_block_match.group(2)
    glue_text = glue_text[:json_block_match.start()] + json_block_match.group(1) + "".join(new_blocks) + json_block_match.group(2)
    GLUE.write_text(glue_text, encoding="utf-8")
    print("glue updated")

plugin_text = PLUGIN_RS.read_text(encoding="utf-8")
registers = []
for mid in ["xml", "csv", "md"]:
    line = f"    crate::artifacts::{mid}::engine::register();"
    if line not in plugin_text:
        registers.append(line)
if registers:
    plugin_text = plugin_text.replace(
        "    crate::artifacts::json::engine::register();",
        "    crate::artifacts::json::engine::register();\n" + "\n".join(registers),
    )
PLUGIN_RS.write_text(plugin_text, encoding="utf-8")

index_lines = INDEX_TS.read_text(encoding="utf-8").strip().splitlines()
for mid, emoji_dir in SPECS:
    export_line = f"export * as {mid} from \"../../🗿️artifacts/{emoji_dir}/🟦️component.ts\";"
    if export_line not in index_lines:
        index_lines.append(export_line)
INDEX_TS.write_text("\n".join(index_lines) + "\n", encoding="utf-8")
print("plugin and index updated")
