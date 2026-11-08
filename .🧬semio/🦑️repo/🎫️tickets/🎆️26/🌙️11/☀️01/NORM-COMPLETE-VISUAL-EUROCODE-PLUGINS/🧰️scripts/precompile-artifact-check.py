#!/usr/bin/env python3
"""🩺 Static pre-compile structure check of norm artifact cargo workspaces: members, deps, lib paths, mod/#[path] targets, sibling project.json/script.ts."""
import os, re, sys, tomllib
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
arts = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm", "🗿️artifacts")
sel = sys.argv[1:]
out = []
for a in sorted(os.listdir(arts)):
    if sel and not any(s in a for s in sel): continue
    root = os.path.join(arts, a); ct = os.path.join(root, "Cargo.toml")
    if not os.path.isfile(ct): continue
    ws = tomllib.load(open(ct, "rb"))["workspace"]
    members = {os.path.normpath(os.path.join(root, m)) for m in ws["members"]}
    wdeps = ws.get("dependencies", {})
    disk = []
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in ("target", "node_modules", "🗑️generated", "🏭️bridge", "👽️guest", "🔮️oracles")]
        if "Cargo.toml" in fs and d != root: disk.append(d)
    names = {}
    for d in disk:
        doc = tomllib.load(open(os.path.join(d, "Cargo.toml"), "rb"))
        if "package" in doc: names[doc["package"]["name"]] = d
    for d in sorted(disk):
        rel = os.path.relpath(d, root)
        if d not in members: out.append(f"{a}: {rel}: has Cargo.toml but not workspace member")
        doc = tomllib.load(open(os.path.join(d, "Cargo.toml"), "rb"))
        if "package" not in doc: continue
        pkg = doc["package"]["name"]
        lib = doc.get("lib", {}).get("path")
        libp = os.path.normpath(os.path.join(d, lib)) if lib else None
        if lib and not os.path.isfile(libp): out.append(f"{a}: {pkg}: lib path missing {lib}")
        for sec in ("test", "bench", "bin", "example"):
            for t in doc.get(sec, []):
                if "path" in t and not os.path.isfile(os.path.normpath(os.path.join(d, t["path"]))): out.append(f"{a}: {pkg}: [[{sec}]] path missing {t['path']}")
        for kind in ("dependencies", "dev-dependencies", "build-dependencies"):
            for dn, v in doc.get(kind, {}).items():
                if not isinstance(v, dict): continue
                if v.get("workspace") and dn not in wdeps: out.append(f"{a}: {pkg}: workspace dep {dn} undeclared in {a}/Cargo.toml")
                if "path" in v and not os.path.isfile(os.path.join(os.path.normpath(os.path.join(d, v["path"])), "Cargo.toml")): out.append(f"{a}: {pkg}: dep {dn} path missing")
        # mod resolution
        if libp and os.path.isfile(libp):
            seen = set()
            def walk(f, modroot):
                if f in seen: return
                seen.add(f)
                src = open(f, encoding="utf-8").read()
                for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*([;{])', src):
                    attrs, name, end = m.groups()
                    pm = re.search(r'#\[path\s*=\s*"([^"]+)"\]', attrs)
                    if end == "{": continue
                    if pm: t = os.path.normpath(os.path.join(os.path.dirname(f), pm.group(1)))
                    else:
                        base = os.path.dirname(f)
                        c = [os.path.join(base, name + ".rs"), os.path.join(base, name, "mod.rs")]
                        t = next((x for x in c if os.path.isfile(x)), c[0])
                    if not os.path.isfile(t): out.append(f"{a}: {pkg}: {os.path.relpath(f, root)}: mod {name} target missing -> {os.path.relpath(t, root)}")
                    else: walk(t, None)
            walk(libp, None)
print("\n".join(out)); print(len(out), "issues")
