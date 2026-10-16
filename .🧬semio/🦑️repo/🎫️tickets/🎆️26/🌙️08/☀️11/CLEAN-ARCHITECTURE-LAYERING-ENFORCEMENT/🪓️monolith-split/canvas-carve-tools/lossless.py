#!/usr/bin/env python3
# 🧾️ Line-multiset losslessness of the carve: original canvas root versus the union of the new roots and module files (blank lines ignored, indentation stripped).
import collections
import os
import sys

SCRATCH = os.environ["SCRATCH"]
MODULE = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️canvas"
ORIGINAL = SCRATCH + "/baseline/🧰️framework/🔨️modules/🖼️canvas/🦀️.rs"
NEW = ["🦀️.rs", "🎬️scene/🦀️.rs", "🎬️scene/🖌️renderer/🦀️.rs", "🎬️scene/🖼️svg-icon/🦀️.rs", "🎬️scene/🏷️text/🦀️.rs", "🎬️scene/🖥️gpu-session/🦀️.rs", "📷️camera/🦀️.rs", "🪪️icon-codec/🦀️.rs"]


def lines(path):
    return collections.Counter(line.strip() for line in open(path, encoding="utf8").read().split("\n") if line.strip())


old = lines(ORIGINAL)
new = collections.Counter()
for relative in NEW:
    new += lines(os.path.join(MODULE, relative))
print("original distinct", sum(old.values()), "new", sum(new.values()))
print("--- only in original (removed or edited)")
for line, count in sorted((old - new).items()):
    print(f"{count:3d}  {line[:200]}")
print("--- only in new (added or edited)")
for line, count in sorted((new - old).items()):
    print(f"{count:3d}  {line[:200]}")
