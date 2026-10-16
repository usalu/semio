#!/usr/bin/env python3
# 🧬️ Singleton hazard scan: a `static` / `thread_local!` name defined in more than one file of the renderer trees (a re-mounted file would fork state).
import collections, os, re, sys
from common import *
import items
by = collections.defaultdict(set)
mount_files = collections.defaultdict(list)
for base, dirs, fs in os.walk(E):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "🧪️tests")]
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(base, f)
            for it in items.table(read(p)):
                if it["kind"] == "static":
                    by[it["name"]].add(p.replace(E + "/", ""))
dups = {k: v for k, v in by.items() if len(v) > 1}
print("statics", len(by), "defined in more than one file:", len(dups))
for k, v in sorted(dups.items()):
    print(" ", k, sorted(v))
# files mounted by more than one crate/module via #[path]
import modtree
roots = [T + "/" + d + "/🦀️.rs" for d in CRATES.values()]
owner = collections.defaultdict(list)
for r in roots:
    if os.path.exists(r):
        for f in modtree.tree(r):
            owner[f].append(r.split("/")[-2])
multi = {f: o for f, o in owner.items() if len(o) > 1}
print("files mounted by more than one WP1 crate:", len(multi))
for f, o in multi.items():
    print(" ", f.replace(E + "/", "")[:100], o)
