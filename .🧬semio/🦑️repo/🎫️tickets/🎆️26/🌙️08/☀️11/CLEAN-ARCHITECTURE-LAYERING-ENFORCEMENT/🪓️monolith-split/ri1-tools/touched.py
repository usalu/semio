#!/usr/bin/env python3
# 🧾️ Files of the engine tree that differ from the pre-edit snapshot (ri1/snap0): new, changed or removed; writes touched.txt (relative to the engine folder) unless --print.
import sys, os, filecmp
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
live = E
rows = []
for base, dirs, files in os.walk(live):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target")]
    for f in files:
        p = os.path.join(base, f)
        rel = os.path.relpath(p, live)
        s = os.path.join(SNAP, rel)
        if not os.path.exists(s):
            rows.append(("new", rel))
        elif not filecmp.cmp(p, s, shallow=False):
            rows.append(("changed", rel))
for base, dirs, files in os.walk(SNAP):
    for f in files:
        rel = os.path.relpath(os.path.join(base, f), SNAP)
        if not os.path.exists(os.path.join(live, rel)):
            rows.append(("removed", rel))
rows.sort(key=lambda r: r[1])
if "--print" in sys.argv:
    for k, r in rows:
        print(k, r.replace("🎯️targets/🧊️wgpu/", "T/").replace("🧱️elements/", "el/"))
else:
    write(os.path.dirname(os.path.abspath(__file__)) + "/touched.txt", "# paths relative to the engine folder E: restored from ri1/snap0 (or deleted when absent there) in the baseline sandbox\n" + "\n".join(r for _, r in rows) + "\n")
print(len(rows))
