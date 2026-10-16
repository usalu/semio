#!/usr/bin/env python3
# 🧾️ Line-multiset losslessness: every Rust file of the baseline registry tree versus every Rust file of the carved tree (blank lines ignored, indentation stripped).
import collections, os, sys
SCRATCH = os.environ["SCRATCH"]
OLD = SCRATCH + "/baseline/✏️s/🔌️plugins/🗄️stdio/📇️registry"
NEW = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry"
def lines(tree):
    out = collections.Counter()
    files = 0
    for directory, _, names in os.walk(tree):
        if "node_modules" in directory or "/dist" in directory:
            continue
        for name in names:
            if name.endswith(".rs"):
                files += 1
                for line in open(os.path.join(directory, name), encoding="utf8").read().split("\n"):
                    if line.strip():
                        out[line.strip()] += 1
    return out, files
old, of = lines(OLD)
new, nf = lines(NEW)
print("old files", of, "lines", sum(old.values()), "| new files", nf, "lines", sum(new.values()))
print("--- only in old (removed or edited):", sum((old - new).values()))
for line, count in sorted((old - new).items()):
    print(f"{count:3d}  {line[:230]}")
print("--- only in new (added or edited):", sum((new - old).values()))
for line, count in sorted((new - old).items()):
    print(f"{count:3d}  {line[:230]}")
