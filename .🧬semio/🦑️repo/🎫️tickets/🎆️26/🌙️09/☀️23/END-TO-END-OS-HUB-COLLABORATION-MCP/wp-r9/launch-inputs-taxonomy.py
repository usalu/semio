#!/usr/bin/env python3
"""📋️ R9 item 1 follow-up (kernel-derive input — apply only after W3 announces 7800 on B3): the plugin-registry generator
renders one launch row per declared `📋️project.json` target, so every project manifest is an input of its contract.
Adds `**/📋️project.json` to `generatorContracts["plugin-registry"].inputPatterns` in the taxonomy (text edit, key order kept).
Usage: `python3 launch-inputs-taxonomy.py [--apply]` (default dry run)."""
import json
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
PATTERN = "**/📋️project.json"
text = open(PATH, encoding="utf-8").read()
taxonomy = json.loads(text)
patterns = taxonomy["generatorContracts"]["plugin-registry"]["inputPatterns"]
if PATTERN in patterns:
    print("already present")
    sys.exit(0)
anchor = json.dumps(".vscode/🧩️launch.seed.jsonc", ensure_ascii=False)
start = text.index('"plugin-registry": {', text.index('"generatorContracts"'))
at = text.index(anchor, start)
line_start = text.rfind("\n", 0, at) + 1
indent = text[line_start:at]
new = text[:at] + json.dumps(PATTERN, ensure_ascii=False) + ",\n" + indent + text[at:]
parsed = json.loads(new)
assert parsed["generatorContracts"]["plugin-registry"]["inputPatterns"][:2] == [PATTERN, ".vscode/🧩️launch.seed.jsonc"]
print("dry run clean" if "--apply" not in sys.argv else "applied")
if "--apply" in sys.argv:
    open(PATH, "w", encoding="utf-8").write(new)
