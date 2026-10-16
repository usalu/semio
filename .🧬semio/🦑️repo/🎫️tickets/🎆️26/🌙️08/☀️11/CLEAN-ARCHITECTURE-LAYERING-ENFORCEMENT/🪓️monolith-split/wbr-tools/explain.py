"""Explain the multiset difference of a split: removed lines paired with their pub(super) widened twins; the rest is listed."""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *

for name in sys.argv[1:]:
    t = TARGETS[name]
    orig = lines_of(read(snap_dir(name) + "/" + t["file"]))
    files = json.loads(read(f"{W}/{name}/installed.json"))["files"]
    new = []
    for rel in files:
        new.extend(lines_of(read(ROOT + "/" + rel)))
    gone, added = diff_multiset(orig, new)
    gone, added = collections.Counter(gone), collections.Counter(added)
    paired = 0
    for l in list(gone):
        w = "pub(super) " + l
        while gone[l] > 0 and added[w] > 0:
            gone[l] -= 1
            added[w] -= 1
            paired += 1
    gone, added = +gone, +added
    print(f"[{name}] original={len(orig)} new={len(new)} widened pairs={paired}")
    print("  removed, unpaired:", sum(gone.values()))
    for l, n in sorted(gone.items()):
        print(f"    - {n}x {l[:170]}")
    print("  added, unpaired:", sum(added.values()))
    for l, n in sorted(added.items()):
        print(f"    + {n}x {l[:170]}")
