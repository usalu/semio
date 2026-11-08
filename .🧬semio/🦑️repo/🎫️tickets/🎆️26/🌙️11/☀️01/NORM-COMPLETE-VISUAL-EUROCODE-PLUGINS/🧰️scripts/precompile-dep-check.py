#!/usr/bin/env python3
"""🩺 Static check: every external crate path used by a norm artifact crate's reachable sources is declared in its Cargo.toml (usage in the lib graph + [[test]] graphs)."""
import os, re, sys, tomllib
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
arts = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm", "🗿️artifacts")
sel = sys.argv[1:]
EXT = re.compile(r'\b(semio_[a-z0-9_]+|serde_json|serde|sqlite|rusqlite|futures|tokio)\b(?=\s*::|\s+as\b|\s*;|\s*\{|,|\s*$)')
def reach(entry):
    seen, order = set(), []
    def walk(f):
        if f in seen or not os.path.isfile(f): return
        seen.add(f); order.append(f)
        src = open(f, encoding="utf-8").read()
        for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*([;{])', src):
            attrs, name, end = m.groups()
            if end == "{": continue
            pm = re.search(r'#\[path\s*=\s*"([^"]+)"\]', attrs)
            if pm: walk(os.path.normpath(os.path.join(os.path.dirname(f), pm.group(1))))
            else:
                b = os.path.dirname(f)
                for c in (os.path.join(b, name + ".rs"), os.path.join(b, name, "mod.rs")):
                    if os.path.isfile(c): walk(c); break
    walk(entry); return order
out = []
for a in sorted(os.listdir(arts)):
    if sel and not any(s in a for s in sel): continue
    root = os.path.join(arts, a)
    if not os.path.isfile(os.path.join(root, "Cargo.toml")): continue
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in ("target", "node_modules", "🗑️generated", "🏭️bridge", "👽️guest", "🔮️oracles")]
        if "Cargo.toml" not in fs or d == root: continue
        doc = tomllib.load(open(os.path.join(d, "Cargo.toml"), "rb"))
        if "package" not in doc: continue
        pkg = doc["package"]["name"]
        declared = {k.replace("-", "_") for kind in ("dependencies", "dev-dependencies", "build-dependencies") for k in doc.get(kind, {})}
        for k, v in list(doc.get("dependencies", {}).items()) + list(doc.get("dev-dependencies", {}).items()):
            if isinstance(v, dict) and "package" in v: declared.add(k.replace("-", "_"))
        entries = []
        if doc.get("lib", {}).get("path"): entries.append(os.path.normpath(os.path.join(d, doc["lib"]["path"])))
        for t in doc.get("test", []): entries.append(os.path.normpath(os.path.join(d, t["path"])))
        own = pkg.replace("-", "_")
        used = {}
        for e in entries:
            for f in reach(e):
                for ln, line in enumerate(open(f, encoding="utf-8").read().split("\n"), 1):
                    s = line.strip()
                    if s.startswith("//"): continue
                    s = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
                    for m in EXT.finditer(s):
                        n = m.group(1)
                        if n != own and n not in used: used[n] = (f, ln)
        for n, (f, ln) in sorted(used.items()):
            if n not in declared: out.append(f"{pkg}: uses `{n}` but Cargo.toml does not declare it ({os.path.relpath(f, root)}:{ln})")
print("\n".join(out)); print(len(out), "issues")
