#!/usr/bin/env python3
"""Builds /private/tmp/agents-fix/tsc/<group> : overlay copies of staged TS files + tsconfig with rootDirs over the live tree.
usage: mk-tsc.py <group> <substring>...   (files of the manifest whose path contains any substring; 'all' = every .ts/.tsx)"""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
group = sys.argv[1]
mirrors = [a[9:] for a in sys.argv[2:] if a.startswith("--mirror=")]
needles = [a for a in sys.argv[2:] if not a.startswith("--mirror=")]
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
base = "/private/tmp/agents-fix/tsc/" + group
shutil.rmtree(base, ignore_errors=True)
ovl = os.path.join(base, "ovl")
files = []
for m in mirrors:
    shutil.copytree(os.path.join(ROOT, m), os.path.join(ovl, m), symlinks=True, ignore=shutil.ignore_patterns("node_modules", "target", "🗑️generated"))
for row in manifest["files"]:
    p = row["path"]
    if not p.endswith((".ts", ".tsx", ".json")): continue
    is_ts = p.endswith((".ts", ".tsx"))
    if needles != ["all"] and not any(n in p for n in needles): continue
    dest = os.path.join(ovl, p)
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copyfile(os.path.join(HERE, "staged", row["after"]), dest)
    parts = p.split("/")
    for i in range(0, len(parts)):
        real = os.path.join(ROOT, *parts[:i]) if i else ROOT
        nm = os.path.join(real, "node_modules")
        link = os.path.join(ovl, *parts[:i], "node_modules") if i else os.path.join(ovl, "node_modules")
        if os.path.isdir(nm) and not os.path.lexists(link):
            os.makedirs(os.path.dirname(link), exist_ok=True)
            os.symlink(nm, link)
    if is_ts: files.append(dest)
cfg = {"extends": os.path.join(ROOT, "tsconfig.json"), "compilerOptions": {"rootDirs": [ovl, ROOT], "noEmit": True, "noUnusedLocals": False, "typeRoots": [os.path.join(ROOT, "node_modules", "@types")], "types": ["bun", "node"]}, "include": [], "files": files, "exclude": []}
json.dump(cfg, open(os.path.join(base, "tsconfig.json"), "w"), indent=1)
print(base, len(files))
