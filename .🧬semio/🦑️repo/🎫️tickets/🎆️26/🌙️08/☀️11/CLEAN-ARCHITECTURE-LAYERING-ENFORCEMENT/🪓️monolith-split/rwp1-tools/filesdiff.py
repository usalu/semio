#!/usr/bin/env python3
# 📂️ Files changed/added/removed in the WP1-owned trees against the pre-carve snapshot (writes 🗑️generated/rwp1-files-changed.txt).
import os, filecmp
from common import *
changed, added, removed = [], [], []
dirs = OWNED + OWNED_FILES + ["🧪️tests/📨️browser-frame-transport/🟦️.ts", "🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts", "🧱️elements/🌉️ProgramBridge/🧪️tests", "🧪️tests/🔬️wgpu-renderer-renderer-io-retained"]
for d in dirs:
    a, b = SNAP + "/engine/" + d, E + "/" + d
    if os.path.isfile(b) or os.path.isfile(a):
        if not os.path.exists(a): added.append(d)
        elif not os.path.exists(b): removed.append(d)
        elif not filecmp.cmp(a, b, shallow=False): changed.append(d)
        continue
    sa = {os.path.relpath(os.path.join(r, f), a) for r, ds, fs in os.walk(a) for f in fs if "node_modules" not in r} if os.path.isdir(a) else set()
    sb = {os.path.relpath(os.path.join(r, f), b) for r, ds, fs in os.walk(b) for f in fs if "node_modules" not in r} if os.path.isdir(b) else set()
    added += [d + "/" + f for f in sorted(sb - sa)]
    removed += [d + "/" + f for f in sorted(sa - sb)]
    changed += [d + "/" + f for f in sorted(sa & sb) if not filecmp.cmp(a + "/" + f, b + "/" + f, shallow=False)]
outside = ["♾️infinite RENDERER_SOURCE test (🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🧪️tests/🔬️unit/🦀️.rs)"]
open(GEN + "/rwp1-files-changed.txt", "w").write("CHANGED\n" + "\n".join(changed) + "\nADDED\n" + "\n".join(added) + "\nREMOVED\n" + "\n".join(removed) + "\nOUTSIDE THE ENGINE TREE\n" + "\n".join(outside) + "\n")
print("changed", len(changed), "added", len(added), "removed", len(removed))
