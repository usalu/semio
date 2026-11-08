#!/usr/bin/env python3
"""🩺 Static check: first path segment after a norm crate name (`semio_s_artifact_norm_*::X`, `semio_s_plugin_norm_*::X`) is defined or re-exported in that crate (reachable sources)."""
import os, re, sys, tomllib
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
norm = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")
sel = sys.argv[1:]
SKIP = ("target", "node_modules", "🗑️generated", "🏭️bridge", "👽️guest", "🔮️oracles")
crates = {}
for base in (norm,):
    for d, ds, fs in os.walk(base):
        ds[:] = [x for x in ds if x not in SKIP]
        if "Cargo.toml" in fs:
            try: doc = tomllib.load(open(os.path.join(d, "Cargo.toml"), "rb"))
            except Exception: continue
            if "package" in doc: crates[doc["package"]["name"].replace("-", "_")] = (d, doc)
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
cache = {}
def idents(name):
    if name in cache: return cache[name]
    d, doc = crates[name]; lib = doc.get("lib", {}).get("path")
    s = set(); glob = False
    if lib:
        for f in reach(os.path.normpath(os.path.join(d, lib))):
            t = open(f, encoding="utf-8").read()
            s.update(re.findall(r'\b(?:fn|struct|enum|const|static|type|trait|mod|union)\s+([A-Za-z_]\w*)', t))
            s.update(re.findall(r'macro_rules!\s+(\w+)', t))
            s.update(re.findall(r'extern\s+crate\s+\w+\s+as\s+(\w+)', t))
            for m in re.finditer(r'\buse\s+([^;]+);', t):
                body = m.group(1)
                s.update(re.findall(r'\bas\s+(\w+)', body)); s.update(re.findall(r'\b([A-Za-z_]\w*)\b', body))
    cache[name] = s; return s
issues = set()
for name, (d, doc) in crates.items():
    if sel and not any(x in d for x in sel): continue
    lib = doc.get("lib", {}).get("path"); entries = []
    if lib: entries.append(os.path.normpath(os.path.join(d, lib)))
    for t in doc.get("test", []): entries.append(os.path.normpath(os.path.join(d, t["path"])))
    for e in entries:
        for f in reach(e):
            t = re.sub(r'//[^\n]*', '', open(f, encoding="utf-8").read())
            for m in re.finditer(r'\b(semio_s_(?:artifact|plugin)_norm_[a-z0-9_]+)\s*::\s*(\{[^}]*\}|[A-Za-z_]\w*)', t):
                tgt, rest = m.group(1), m.group(2)
                if tgt not in crates or tgt == name: continue
                items = re.findall(r'(?:^|[,{]\s*)([A-Za-z_]\w*)\s*(?:::|,|\}|\s+as\b|$)', rest) if rest.startswith("{") else [rest]
                if rest.startswith("{"): items = [x.strip().split("::")[0].split(" as ")[0].strip() for x in rest[1:-1].split(",") if x.strip()]
                known = idents(tgt)
                for it in items:
                    if it in ("self", "super", "crate", "*") or not it or not re.match(r'^\w+$', it): continue
                    if it not in known: issues.add(f"{os.path.relpath(f, norm)}: `{tgt}::{it}` not found in target crate")
print("\n".join(sorted(issues))); print(len(issues), "issues")
