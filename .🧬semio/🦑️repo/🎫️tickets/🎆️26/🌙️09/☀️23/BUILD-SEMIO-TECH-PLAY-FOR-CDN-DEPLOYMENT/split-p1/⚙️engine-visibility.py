#!/usr/bin/env python3
"""⚙️ Widens `pub(crate)` engine items that another energy crate names to `pub` (lane P1, slice S1b).

A `pub(crate)` declaration (item, field or method) becomes `pub` when its name occurs, comment-stripped and word-bounded,
in a file of another engine module or of the artifact; everything else keeps its crate-private visibility.
"""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "🕸️engine-graph.py"), encoding="utf-8").read().split("mods = {")[0])
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
files = {}
for root, dirs, fs in os.walk(PLUGIN):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if f.endswith(".rs"): files[os.path.join(root, f)] = strip(open(os.path.join(root, f), encoding="utf-8").read())
def owner(p):
    if p.startswith(ENGINE + "/"): return p[len(ENGINE) + 1:].split("/")[0]
    return "artifact"
DECL = re.compile(r"pub\(crate\)\s+(?:(?:async|const|unsafe)\s+)*(?:(?:fn|struct|enum|const|static|type|trait|mod)\s+)?([A-Za-z_][A-Za-z0-9_]*)")
changed = 0
for p, text in files.items():
    if not p.startswith(ENGINE) or "🧪️tests" in p: continue
    mine = owner(p); raw = open(p, encoding="utf-8").read(); edits = []
    for m in DECL.finditer(text):
        name = m.group(1); pat = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")
        if any(owner(q) != mine and pat.search(t) for q, t in files.items()):
            edits.append(m.start())
    for i in reversed(edits):
        assert raw[i:i + 10] == "pub(crate)"; raw = raw[:i] + "pub" + raw[i + 10:]
    if edits:
        open(p, "w", encoding="utf-8").write(raw); changed += len(edits); print(len(edits), os.path.relpath(p, ENGINE))
print("widened", changed)
