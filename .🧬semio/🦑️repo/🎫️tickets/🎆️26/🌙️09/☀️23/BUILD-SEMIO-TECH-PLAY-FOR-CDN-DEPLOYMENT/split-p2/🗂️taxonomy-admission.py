#!/usr/bin/env python3
"""🗂️ Admits the directories the remodeling cut introduces into the taxonomy member registries, by a surgical text insert
into `🔣️taxonomy.json` (the file is shared and edited concurrently, so it is never reformatted): the command groups
(`members-of-commands`), the mutation group package directories (`members-of-schema`) and the engine sub-topics
(`members-of-members-of-engine`) (lane P2)."""
import json, os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
PATH = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")
ADD = {"members-of-commands": ["📥️ingest", "🎛️workflow"], "members-of-schema": ["🎞️media", "🎯️calibration", "⚙️params", "🧵️results"], "members-of-members-of-engine": ["🔺️surface", "🩹️repair", "🗺️atlas", "🎞️h264"]}
text = open(PATH, encoding="utf-8").read()
data = json.loads(text)
for kind, names in ADD.items():
    present = set(data["semanticDirectoryMemberKinds"][kind]["memberNames"])
    missing = [name for name in names if name not in present]
    if not missing:
        continue
    match = re.search(r'\n(\s*)"' + re.escape(kind) + r'": \{\s*\n\s*"ownerKindIds": \[[^\]]*\],\s*\n(\s*)"memberNames": \[\n', text)
    assert match, kind
    indent = match.group(2) + "  "
    insert = "".join(f'{indent}{json.dumps(name, ensure_ascii=False)},\n' for name in missing)
    text = text[:match.end()] + insert + text[match.end():]
json.loads(text)
open(PATH, "w", encoding="utf-8").write(text)
check = json.loads(open(PATH, encoding="utf-8").read())["semanticDirectoryMemberKinds"]
print({kind: [name for name in names if name in check[kind]["memberNames"]] for kind, names in ADD.items()})
