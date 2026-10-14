#!/usr/bin/env python3
"""📐️ Buckets one crate unit's closure (from `📏️unit-files.ts --files`) by taxonomy subtree below a prefix.

usage: 📐️bucket-census.py <files.tsv> <strip-prefix> <depth> [--exclude-tests]
"""
import sys, collections
path, prefix, depth = sys.argv[1], sys.argv[2], int(sys.argv[3])
roll = collections.OrderedDict()
for line in open(path, encoding="utf-8"):
    p = line.rstrip("\n").split("\t")
    if p[0] != "F": continue
    lines, size, kind, f = int(p[1]), int(p[2]), p[3], p[4]
    rel = f[len(prefix):] if f.startswith(prefix) else "~" + f
    if "--exclude-tests" in sys.argv and "🧪️tests" in rel: continue
    parts = rel.split("/")[:-1]
    key = "/".join(parts[:depth]) or "."
    r = roll.setdefault(key, [0, 0, 0, 0]); r[0] += lines; r[1] += size; r[2] += 1; r[3] += size if kind == "embedded" else 0
for k, r in sorted(roll.items()): print(f"{r[0]:6}L {r[1]:8}B {r[2]:4}F emb={r[3]:7}  {k}")
