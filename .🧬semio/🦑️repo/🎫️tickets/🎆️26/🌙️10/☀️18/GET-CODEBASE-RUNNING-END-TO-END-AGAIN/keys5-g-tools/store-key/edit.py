# keys5-g: anchored, idempotent edit of the framework project.json: target `test-source-text` declares the store Rust module files its source text reads (law laws/store-source-text.test.ts).
# usage: python3 -I edit.py <root> apply|revert|check
import sys, os
root, mode = sys.argv[1], sys.argv[2]
path = os.path.join(root, "🧰️framework/📦️packages/🟦️typescript/📋️project.json")
text = open(path, encoding="utf-8").read()
STORE = "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
anchor = '        "{workspaceRoot}/🧰️framework/🔨️modules/🧪️test/📖️source-text/**/*"\n      ],\n      "options": {\n        "command": "repo ./📜️script.ts test-source-text",'
lines = [
    f'{STORE}/🦀️.rs',
    f'{STORE}/{{⚙️engine,🏗️initialization,🧰️test-support,🧩️composition,🧾️document,🎚️config,📜️space-history/🏠️space,📦️codec/🏭️factory}}/**/🦀️.rs',
    f'{STORE}/**/📥️lifted/**/🦀️.rs',
    f'!{STORE}/**/{{🧪️tests,🧪️testing,📦️packages,node_modules}}/**',
]
added = "".join(f'        "{line}",\n' for line in lines)
new = '        "{workspaceRoot}/🧰️framework/🔨️modules/🧪️test/📖️source-text/**/*",\n' + added.rstrip(",\n") + "\n      ],\n      \"options\": {\n        \"command\": \"repo ./📜️script.ts test-source-text\","
if mode == "check":
    print("applied" if new in text else ("base" if text.count(anchor) == 1 else "unknown")); sys.exit(0)
if mode == "apply":
    if new in text: print("already applied"); sys.exit(0)
    assert text.count(anchor) == 1, "anchor not unique"
    text = text.replace(anchor, new)
elif mode == "revert":
    if new not in text: print("not applied"); sys.exit(0)
    text = text.replace(new, anchor)
tmp = path + ".k5tmp"
open(tmp, "w", encoding="utf-8").write(text); os.replace(tmp, path)
import json; json.load(open(path, encoding="utf-8"))
print(mode, "ok")
