#!/usr/bin/env python3
"""🔣️ Admits the P5 fem split's new directory names into the closed member registries of `🔣️taxonomy.json`, by text
insertion at the end of each registry's `memberNames` (the file's formatting stays untouched; present names are skipped).

usage: 🔣️taxonomy-admit.py plan|apply
"""
import json, re, sys
P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
ADMIT = {
    "members-of-engine": ["⚖️static"],
    "members-of-members-of-engine": ["🔺️ldlt", "🔁️pcg", "🎵️subspace", "🧬️derived", "🗂️model", "🧩️assembly"],
    "members-of-schema": ["🏗️structure", "🛡️guards", "🌉️apply"],
    "members-of-io": ["📋️reports"],
    "members-of-members-of-editor": ["🎞️visual-job"],
    "members-of-tests": ["🧰️support", "🔬️elements2d", "🔬️continuum", "🔬️plate", "🔬️elements3d", "🔬️shell", "🔬️solid",
                         "🔬️topology-kinds", "🔬️section-kinds", "🔬️region-kinds", "🔬️solid-kinds", "🔬️selection-kinds",
                         "🔬️case-kinds", "🔬️load-kinds", "🔬️combination-kinds", "🔬️support-kinds", "🔬️material-kinds",
                         "🔬️settings-kinds", "🔬️analysis-kinds", "⚖️laws", "🔬️app", "🔬️emphasis"],
}
s = open(P, encoding="utf-8").read()
reg = json.loads(s)["semanticDirectoryMemberKinds"]
for key, names in ADMIT.items():
    have = set(reg[key]["memberNames"])
    add = [n for n in names if n not in have]
    print(key, "add", add)
    if not add or sys.argv[1] != "apply": continue
    i = s.index('"' + key + '": {')
    j = s.index('"memberNames": [', i)
    k = s.index("]", j)
    body = s[j:k].rstrip()
    indent = re.search(r"\n(\s*)\"", s[j:k]).group(1)
    s = s[:j] + body + "".join(f',\n{indent}"{n}"' for n in add) + "\n" + indent[:-2] + s[k:]
if sys.argv[1] == "apply":
    json.loads(s)
    open(P, "w", encoding="utf-8").write(s)
