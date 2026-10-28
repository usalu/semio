#!/usr/bin/env python3
"""🧹️ Adds the comment-in-body removals of files that peers wrote today to the batch specs (comments inside definitions move into the docstring of the function): python3 -I cleanup_spec.py"""
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and os.path.dirname(ROOT) != ROOT: ROOT = os.path.dirname(ROOT)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(batch, rel, old, new):
    path = os.path.join(HERE, "batches", batch + ".json")
    spec = json.load(open(path, encoding="utf8"))
    if rel not in spec["files"]: spec["files"].append(rel)
    spec["manual"] = [m for m in spec["manual"] if not (m["rel"] == rel and m.get("old") == old)]
    spec["manual"].append({"rel": rel, "old": old, "new": new, "count": "any", "optional": True})
    json.dump(spec, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
pg = L + "🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs"
text = open(os.path.join(ROOT, pg), encoding="utf8").read()
groups = []
rows = text.split("\n")
i = 0
while i < len(rows):
    if re.match(r"^\s+// ", rows[i]) and i > 0:
        j = i
        while j + 1 < len(rows) and re.match(r"^\s+// ", rows[j + 1]): j += 1
        groups.append((i, j)); i = j + 1
    else: i += 1
notes = []
for a, b in groups:
    block = rows[a:b + 1]
    edit("libgraph", pg, "\n".join(block) + "\n", "")
    notes.append(" ".join(re.sub(r"^\s+// ?", "", r) for r in block))
anchor = " */\nexport async function playgroundPreparationTargets"
if anchor in text:
    edit("libgraph", pg, anchor, "".join(" * %s\n" % n for n in notes) + anchor)
print(len(groups), "comment groups of the playground targets")
proc = L + "🏃️process/🟦️.ts"
edit("libother", proc, "    /* optional */\n", "")
py = L + "🏗️build/🚢️fleet-status/🧪️tests/🐍️.py"
t = open(os.path.join(ROOT, py), encoding="utf8").read().split("\n")
for r in t:
    if r.strip().startswith("# keep per resource at most one exclusive holder"): edit("libbuild", py, r + "\n", "")
cut = L + "🏗️build/🕸️graph/🕰️generations/✂️cutoff/🧪️tests/🐍️oracle/🐍️.py"
tx = open(os.path.join(ROOT, cut), encoding="utf8").read()
m = re.match(r'(#![^\n]*\n)?"""(?=[A-Za-z])', tx)
if m:
    head = tx[:m.end() + 24]
    edit("libgraph", cut, head, m.group(0) + "🎯 " + head[m.end():])
