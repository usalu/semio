#!/usr/bin/env python3
"""🕸️ Package graph of the energy plugin's crate family from its manifests (lane P1): normal-edge cycles, dev-edge back
edges (a dev-dependency on a crate that already depends on the package), layer depths and, with `--dot`, the edge list.
usage: 🕸️crate-graph.py [--dot] [root]
"""
import os, re, sys
ROOT = next((a for a in sys.argv[1:] if not a.startswith("--")), "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy")
pk = {}
for r, d, f in os.walk(ROOT):
    d[:] = [x for x in d if x not in ("target", "node_modules")]
    if "Cargo.toml" not in f or "📦️packages" not in r: continue
    sec, name, rows = None, None, {"dependencies": set(), "dev-dependencies": set()}
    for line in open(os.path.join(r, "Cargo.toml"), encoding="utf-8"):
        h = re.match(r"^\[\[?([^\]]+)\]\]?\s*$", line)
        if h: sec = h.group(1); continue
        kv = re.match(r'^([\w-]+)\s*=\s*(.*)$', line)
        if not kv: continue
        if sec == "package" and kv.group(1) == "name": name = kv.group(2).strip().strip('"')
        elif sec in rows:
            p = re.search(r'package\s*=\s*"([^"]+)"', kv.group(2)); rows[sec].add(p.group(1) if p else kv.group(1))
    if name: pk[name] = (rows["dependencies"], rows["dev-dependencies"])
local = set(pk)
def reach(src):
    seen, st = set(), [src]
    while st:
        for y in pk[st.pop()][0] & local:
            if y not in seen: seen.add(y); st.append(y)
    return seen
R = {n: reach(n) for n in pk}
print(len(pk), "packages;", sum(len(pk[n][0] & local) for n in pk), "local normal edges")
print("normal cycles:", sorted(n for n in pk if n in R[n]) or "none")
print("dev back-edges:", sorted((n, y) for n in pk for y in pk[n][1] & local if n in R[y]) or "none")
depth = {}
def dep(n):
    if n not in depth: depth[n] = 0; depth[n] = 1 + max((dep(y) for y in pk[n][0] & local), default=-1)
    return depth[n]
for n in pk: dep(n)
print("max depth:", max(depth.values()))
if "--dot" in sys.argv:
    for n in sorted(pk): print(f"{depth[n]:2d} {n} -> {', '.join(sorted(pk[n][0] & local))}")
