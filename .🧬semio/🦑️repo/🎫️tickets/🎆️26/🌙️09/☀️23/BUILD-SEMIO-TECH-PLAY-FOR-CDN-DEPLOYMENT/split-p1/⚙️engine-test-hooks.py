#!/usr/bin/env python3
"""⚙️ Promotes `#[cfg(test)]` engine items that another energy crate's laws name into regular `pub` API (lane P1, S1b).

A test-only item compiled only under its own crate's `cfg(test)` is invisible to a dependent crate's tests, and the
split forbids `testing` features. Each `#[cfg(test)]` item (fn, const, static, struct, enum, method) whose name occurs in
another engine module's or the artifact's files loses the attribute and is made `pub`; `mod tests` mounts stay gated.
usage: ⚙️engine-test-hooks.py plan|apply <module>:<name>...   (only the listed items the compiler reported missing)
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "🕸️engine-graph.py"), encoding="utf-8").read().split("mods = {")[0])
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
files = {}
for root, dirs, fs in os.walk(PLUGIN):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if f.endswith(".rs"): files[os.path.join(root, f)] = strip(open(os.path.join(root, f), encoding="utf-8").read())
def owner(p): return p[len(ENGINE) + 1:].split("/")[0] if p.startswith(ENGINE + "/") else "artifact"
ITEM = re.compile(r"#\[cfg\(test\)\]\s*\n(\s*)((?:#\[[^\]]*\]\s*\n\s*)*)((?:pub(?:\([^)]*\))?\s+)?)((?:const\s+fn|async\s+fn|fn|const|static|struct|enum|type)\s+)([A-Za-z_][A-Za-z0-9_]*)")
WANT = set(sys.argv[2:])
total = 0
for p, text in files.items():
    if not p.startswith(ENGINE + "/") or "🧪️tests" in p: continue
    mine = owner(p); raw = open(p, encoding="utf-8").read(); edits = []
    for m in ITEM.finditer(text):
        name = m.group(5); pat = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")
        if f"{mine}:{name}" in WANT and any(owner(q) != mine and pat.search(t) for q, t in files.items()):
            edits.append((m.start(), m.start(3), m.end(3), name))
    for start, vs, ve, name in reversed(edits):
        block = raw[start:vs]; assert block.startswith("#[cfg(test)]")
        raw = raw[:start] + block[len("#[cfg(test)]"):].lstrip("\n").lstrip(" \t") if False else raw[:start] + raw[start + len("#[cfg(test)]"):vs].lstrip("\n") + "pub " + raw[ve:]
        print(" ", os.path.relpath(p, ENGINE), name)
    if edits and sys.argv[1] == "apply": open(p, "w", encoding="utf-8").write(raw)
    total += len(edits)
print("promoted", total)
