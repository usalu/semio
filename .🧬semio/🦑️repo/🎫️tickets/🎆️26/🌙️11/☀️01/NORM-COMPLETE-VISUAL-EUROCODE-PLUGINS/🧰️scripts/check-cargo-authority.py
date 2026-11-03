#!/usr/bin/env python3
"""🩺 Validates cargo member authority of every norm crate: workspace key, members list, glob, deps, lib path."""
import os, re, sys, tomllib, fnmatch
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 0)) if False else None
here = os.getcwd()
while not os.path.isdir(os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")): here = os.path.dirname(here)
norm = os.path.join(here, "✏️s", "🔌️plugins", "📕️norm")
ws = tomllib.load(open(os.path.join(norm, "Cargo.toml"), "rb"))["workspace"]
members = {os.path.normpath(os.path.join(norm, m)) for m in ws["members"]}
globs = ws["metadata"]["semio"]["repository"]["member-manifests"]
errors, names, crates = [], {}, []
errors += [f"duplicate workspace member: {m}" for m in sorted({m for m in ws["members"] if ws["members"].count(m) > 1})]
for d, ds, fs in os.walk(norm):
    ds[:] = [x for x in ds if x not in ("target", "🗿️artifacts", "node_modules", "🗑️generated", "🏭️bridge")]
    if "Cargo.toml" in fs and d != norm: crates.append(d)
def glob_ok(rel):
    rx = "^" + re.escape(rel).replace(r"\*\*/", "(?:.*/)?").replace(r"\*", "[^/]*") + "$"
    return any(re.match("^" + re.escape(g).replace(r"\*\*/", "(?:.*/)?").replace(r"\*", "[^/]*") + "$", rel) for g in globs)
for d in sorted(crates):
    rel = os.path.relpath(d, norm); doc = tomllib.load(open(os.path.join(d, "Cargo.toml"), "rb"))
    if "package" not in doc: continue
    p = doc["package"]
    if os.path.normpath(os.path.join(d, p.get("workspace", ""))) != norm: errors.append(f"{rel}: package.workspace does not resolve to norm root")
    if d not in members: errors.append(f"{rel}: not in workspace.members")
    if not glob_ok(rel + "/Cargo.toml"): errors.append(f"{rel}: not covered by member-manifests glob")
    if p["name"] in names: errors.append(f"{rel}: duplicate name {p['name']}")
    names[p["name"]] = rel
    for k in ("version", "edition", "rust-version"):
        if not (isinstance(p.get(k), dict) and p[k].get("workspace")): errors.append(f"{rel}: {k}.workspace missing")
    if not doc.get("lints", {}).get("workspace"): errors.append(f"{rel}: lints.workspace missing")
    lib = doc.get("lib", {}).get("path")
    if lib and not os.path.isfile(os.path.normpath(os.path.join(d, lib))): errors.append(f"{rel}: lib path missing {lib}")
    for kind in ("dependencies", "dev-dependencies", "build-dependencies"):
        for a, v in doc.get(kind, {}).items():
            if isinstance(v, dict) and "path" in v:
                t = os.path.normpath(os.path.join(d, v["path"]))
                if not os.path.isfile(os.path.join(t, "Cargo.toml")): errors.append(f"{rel}: dep {a} path missing")
                elif t.startswith(norm+os.sep) and t not in members: errors.append(f"{rel}: dep {a} target not a member")
            elif isinstance(v, dict) and v.get("workspace") and a not in ws.get("dependencies", {}): errors.append(f"{rel}: workspace dep {a} undeclared")
for m in members:
    if not os.path.isfile(os.path.join(m, "Cargo.toml")): errors.append(f"member missing on disk: {os.path.relpath(m, norm)}")
print(f"checked {len(crates)} crates, {len(members)} members, {len(errors)} errors")
for e in errors: print("ERROR", e)
sys.exit(1 if errors else 0)
