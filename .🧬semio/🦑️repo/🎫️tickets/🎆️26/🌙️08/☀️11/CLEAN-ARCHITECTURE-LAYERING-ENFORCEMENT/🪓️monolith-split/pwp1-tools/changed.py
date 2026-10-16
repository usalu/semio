#!/usr/bin/env python3
# 📃️ Files the WP1 carve changed: owned trees, hub, macro mount, lift files, foreign anchored edits (snapshot against live; peers' unrelated edits are not listed).
import filecmp, os
from lib import *
import uo

S, L = SNAP + "/" + PLUG_REL, PLUG
rows = []
def walk(rel):
    sd, ld = S + "/" + rel, L + "/" + rel
    names = set()
    for base, dirs, files in os.walk(sd if os.path.isdir(sd) else "/nonexistent"):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "📦️packages")]
        names |= {os.path.relpath(os.path.join(base, f), S) for f in files}
    for base, dirs, files in os.walk(ld):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "📦️packages")]
        names |= {os.path.relpath(os.path.join(base, f), L) for f in files}
    for n in sorted(names):
        if not n.endswith(".rs") and not n.endswith(".toml"):
            continue
        a, b = os.path.exists(S + "/" + n), os.path.exists(L + "/" + n)
        if a and b:
            if not filecmp.cmp(S + "/" + n, L + "/" + n, shallow=False):
                rows.append(("changed", n))
        elif b:
            rows.append(("added", n))
        else:
            rows.append(("removed", n))
for d in uo.OWNED_DIRS:
    walk(d)
for f in uo.OWNED_FILES + ["🦀️.rs", "⚛️reactor/🦀️.rs", "🗃️document/🦀️.rs", "🧪️tests/🔬️app-window-kits/🦀️.rs", "🧪️tests/🔬️app-panel-kit/🦀️.rs"]:
    a, b = os.path.exists(S + "/" + f), os.path.exists(L + "/" + f)
    if a and b and not filecmp.cmp(S + "/" + f, L + "/" + f, shallow=False):
        rows.append(("changed", f))
# crate folders: manifests
import json
C = json.load(open(GEN + "/pwp0-crates.json"))
from crates_wp1 import ORDER
for k in ORDER:
    rel = C[k]["root"] + "/" + PKGDIR
    for f in ("Cargo.toml", "🦀️.rs"):
        a, b = os.path.exists(S + "/" + rel + "/" + f), os.path.exists(L + "/" + rel + "/" + f)
        if a and not b:
            rows.append(("removed", rel + "/" + f))
        elif a and b and not filecmp.cmp(S + "/" + rel + "/" + f, L + "/" + rel + "/" + f, shallow=False):
            rows.append(("changed", rel + "/" + f))
open(GEN + "/pwp1-files-changed.txt", "w").write("\n".join(f"{s}\t{n}" for s, n in rows) + "\n")
import collections
print(collections.Counter(s for s, n in rows))
