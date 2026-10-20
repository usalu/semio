#!/usr/bin/env python3
"""🔬️ Lock delta oracle: compares every root package.json and member manifest with the workspaces section of the root's bun.lock (pure python3, read only, no bun).
Usage: lockdelta.py [--root DIR] [--overlay DIR] [--json OUT] [--verbose] [--expect FILE]
--overlay DIR: files of that directory shadow the live tree (the staged batch prediction). --expect FILE: exit 1 unless the delta equals the JSON written by --json earlier."""
import argparse, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lockio import ROOTS, FIELDS, load_lock

C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ap = argparse.ArgumentParser()
ap.add_argument("--root", default=os.path.abspath(os.path.join(C, *[".."] * 7)))
ap.add_argument("--overlay"); ap.add_argument("--json"); ap.add_argument("--expect"); ap.add_argument("--verbose", action="store_true")
a = ap.parse_args()

def pick(rel):
    if a.overlay and os.path.exists(os.path.join(a.overlay, rel)): return os.path.join(a.overlay, rel)
    return os.path.join(a.root, rel)

report = {}
for r in ROOTS:
    root = json.load(open(pick(os.path.join(r, "package.json")), encoding="utf-8"))
    ws = root.get("workspaces", [])
    ws = ws.get("packages", []) if isinstance(ws, dict) else ws
    lock = load_lock(pick(os.path.join(r, "bun.lock")))
    lw = lock["workspaces"]
    rep = {"members": len(ws) + 1, "added": [], "removed": [], "changed": [], "missing": []}
    for m in [""] + list(ws):
        mp = pick(os.path.join(r, m, "package.json"))
        if not os.path.isfile(mp): rep["missing"].append(m); continue
        man = json.load(open(mp, encoding="utf-8"))
        if m not in lw: rep["added"].append({"member": m, "name": man.get("name")}); continue
        le = lw[m]
        for f in FIELDS:
            new, old = man.get(f) or {}, le.get(f) or {}
            for k in sorted(set(new) | set(old)):
                if new.get(k) != old.get(k): rep["changed"].append({"member": m, "name": man.get("name"), "field": f, "dep": k, "lock": old.get(k), "manifest": new.get(k)})
        if man.get("name") != le.get("name"): rep["changed"].append({"member": m, "field": "name", "lock": le.get("name"), "manifest": man.get("name")})
    rep["removed"] = [m for m in lw if m not in [""] + list(ws)]
    for key in ("overrides", "patchedDependencies"):
        if (root.get(key) or {}) != (lock.get(key) or {}): rep["changed"].append({"member": "<root>", "field": key, "lock": lock.get(key), "manifest": root.get(key)})
    report[r] = rep
    print(f"{r:20} members={rep['members']:4} added={len(rep['added'])} removed={len(rep['removed'])} changed={len(rep['changed'])} missing={len(rep['missing'])}")
    if a.verbose:
        for k in ("added", "removed", "missing"): [print("   ", k, x) for x in rep[k]]
        [print("    changed", json.dumps(x, ensure_ascii=False)[:300]) for x in rep["changed"]]
if a.json: json.dump(report, open(a.json, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
if a.expect:
    want = json.load(open(a.expect, encoding="utf-8"))
    if want != json.loads(json.dumps(report)): print("LOCK DELTA DIFFERS from the predicted delta " + a.expect); sys.exit(1)
    print("lock delta equals the prediction")
