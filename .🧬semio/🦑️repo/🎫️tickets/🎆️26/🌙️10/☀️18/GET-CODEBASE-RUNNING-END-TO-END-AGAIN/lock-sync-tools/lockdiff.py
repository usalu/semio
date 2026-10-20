#!/usr/bin/env python3
"""🧮️ Compares the six bun.lock files before and after `dependencies lock --all` against the predicted batch effect.
Usage: lockdiff.py --before DIR --after DIR [--root DIR]  (DIR holds root.bun.lock and one <root>.bun.lock per product root; written by run.py)
Expected: workspace rows exactly as predicted (+@semio-tech/build-parsers, -4 Nx devDependencies in the root, -framework-filesystem dependency of the tools recipe), packages: only +@semio-tech/build-parsers, no third-party version change."""
import argparse, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lockio import ROOTS, FIELDS, load_lock, lock_name

ap = argparse.ArgumentParser(); ap.add_argument("--before", required=True); ap.add_argument("--after", required=True)
a = ap.parse_args()
ADDED = {"@semio-tech/build-parsers"}
bad, summary = [], {}
for r in ROOTS:
    n = lock_name(r) + ".bun.lock"
    b, f = load_lock(os.path.join(a.before, n)), load_lock(os.path.join(a.after, n))
    rows = {}
    for m in sorted(set(b["workspaces"]) | set(f["workspaces"])):
        x, y = b["workspaces"].get(m), f["workspaces"].get(m)
        if x is None: rows[m] = "added " + str(y.get("name")); continue
        if y is None: rows[m] = "removed"; continue
        for fld in FIELDS:
            for k in sorted(set(x.get(fld) or {}) | set(y.get(fld) or {})):
                u, v = (x.get(fld) or {}).get(k), (y.get(fld) or {}).get(k)
                if u != v: rows[m + "|" + fld + "|" + k] = f"{u} -> {v}"
    pk = {k: (b["packages"].get(k), f["packages"].get(k)) for k in set(b["packages"]) | set(f["packages"]) if b["packages"].get(k) != f["packages"].get(k)}
    pa = sorted(k for k, (u, v) in pk.items() if u is None); pr = sorted(k for k, (u, v) in pk.items() if v is None); pc = sorted(k for k, (u, v) in pk.items() if u is not None and v is not None)
    other = {k: (b.get(k), f.get(k)) for k in set(b) | set(f) if k not in ("workspaces", "packages") and b.get(k) != f.get(k)}
    summary[r] = {"workspaceRows": len(rows), "packagesAdded": pa, "packagesRemoved": pr, "packagesChanged": pc[:10], "otherKeys": sorted(other)}
    if set(pa) != ADDED: bad.append(f"{r}: unexpected packages added {pa}")
    if pr: bad.append(f"{r}: packages removed {pr[:6]}")
    if pc: bad.append(f"{r}: package entries changed {pc[:6]}")
    if other: bad.append(f"{r}: top-level keys changed {sorted(other)}")
    for k, v in rows.items():
        if "|" not in k:
            if v != "added @semio-tech/build-parsers": bad.append(f"{r}: unexpected member row {k[-60:]} {v}")
            continue
        m, fld, dep = k.split("|")
        recipe = dep == "@semio-tech/framework-filesystem" and fld == "dependencies" and v == "workspace:* -> None" and m.endswith("🛠️tools")
        nx = r == "." and m == "" and fld == "devDependencies" and dep in ("nx", "@nx/js", "@nx/devkit", "@nxlv/python") and v.endswith("-> None")
        if not (recipe or nx): bad.append(f"{r}: unexpected workspace row {m[-50:]} {fld} {dep} {v}")
print(json.dumps(summary, ensure_ascii=False, indent=1))
if bad: print("UNEXPECTED:"); [print("  " + x) for x in bad]; sys.exit(1)
print("lock delta matches the prediction")
