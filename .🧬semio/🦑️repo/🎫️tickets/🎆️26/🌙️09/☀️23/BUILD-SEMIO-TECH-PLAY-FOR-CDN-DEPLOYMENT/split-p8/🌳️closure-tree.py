#!/usr/bin/env python3
"""🌳️ Per-directory census of one crate's canonical closure (`📂️closure-files.ts` JSON): lib files/lines and the
test-only files/lines (sources and embedded inputs) beneath each directory, to choose taxonomy cut points.

usage: 🌳️closure-tree.py <closure.json> <crate dir (repo-relative)> [depth]
"""
import collections, json, sys
data = json.load(open(sys.argv[1])); base = sys.argv[2].rstrip("/") + "/"; depth = int(sys.argv[3]) if len(sys.argv) > 3 else 3
agg = collections.defaultdict(lambda: [0, 0, 0, 0, 0])
for path, lines in data["test"].items():
    rel = path[len(base):] if path.startswith(base) else "@" + path
    lib = path in data["lib"]
    parts = rel.split("/")[:-1]
    for k in range(0, min(len(parts), depth) + 1):
        a = agg["/".join(parts[:k]) or "."]
        if lib: a[0] += 1; a[1] += data["lib"][path]
        else: a[2] += 1; a[3] += lines
        a[4] += data["bytes"][path]
for key in sorted(agg):
    a = agg[key]
    print(f"lib {a[0]:4d}F {a[1]:6d}L | +test {a[2]:4d}F {a[3]:6d}L | {a[4] // 1024:6d}KB  {'  ' * (key.count('/') + (key != '.'))}{key.split('/')[-1]}")
