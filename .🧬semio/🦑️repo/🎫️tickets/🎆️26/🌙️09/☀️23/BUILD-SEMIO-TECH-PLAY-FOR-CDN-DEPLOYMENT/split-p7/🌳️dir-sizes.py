#!/usr/bin/env python3
"""🌳️ Aggregates a `📋️unit-files.ts` dump per directory: files, active lines and bytes of the test unit (lib rows marked),
down to a depth below an artifact-relative prefix.

usage: 🌳️dir-sizes.py <files.tsv> <artifact dir (repo-relative)> <depth> [prefix]
"""
import collections, sys
tsv, artifact, depth = sys.argv[1], sys.argv[2].rstrip("/") + "/", int(sys.argv[3])
prefix = sys.argv[4].rstrip("/") + "/" if len(sys.argv) > 4 else ""
agg = collections.defaultdict(lambda: [0, 0, 0, 0, 0])
for line in open(tsv, encoding="utf-8"):
    unit, kind, lines, size, path = line.rstrip("\n").split("\t")
    rel = path[len(artifact):] if path.startswith(artifact) else "<external>/" + path
    if not rel.startswith(prefix):
        continue
    parts = rel[len(prefix):].split("/")
    key = "/".join(parts[:min(depth, len(parts) - 1)]) or "."
    row = agg[key]
    if unit == "test":
        row[0] += 1; row[1] += int(lines); row[2] += int(size)
    else:
        row[3] += 1; row[4] += int(lines)
print("testF\ttestL\ttestB\tlibF\tlibL\tdir")
for key in sorted(agg):
    r = agg[key]
    print(f"{r[0]}\t{r[1]}\t{r[2]}\t{r[3]}\t{r[4]}\t{key}")
