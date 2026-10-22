#!/usr/bin/env python3
"""Proves the declared classification equals the old rule on every probed project: per target the classification tuple, the test scope and the preparation gate must be identical. usage: compare.py <old-dir> <new-dir>"""
import json, glob, re, sys, collections
old_dir, new_dir = sys.argv[1], sys.argv[2]
PREP = re.compile(r"^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)"); TESTS = re.compile(r"^(?:test|bench)(?:-|$)")
def load(d):
    rows = {}
    for f in sorted(glob.glob(d + "/*-*.json")):
        data = json.load(open(f))
        for fail in data["failures"]: rows.setdefault("__failures__", []).append(fail)
        for r in data["rows"]: rows[r["path"]] = r
    return rows
old, new = load(old_dir), load(new_dir)
fails = new.pop("__failures__", []); old.pop("__failures__", None)
diffs = collections.Counter(); samples = collections.defaultdict(list); targets = 0
def old_prep(c):
    (proj, name, npt, nt, e, orch, cmd, auth, np_, exe, runs, pkg, gen) = c[:13]
    if not pkg: return "skip"
    if e == "source" or (e is None and not PREP.match(name)) or gen: return None
    return runs if e == "native" else False if e == "orchestration" else bool(TESTS.match(name))
def new_prep(c):
    (proj, name, npt, nt, e, orch, cmd, auth, np_, exe, runs, pkg, gen) = c[:13]; declared = c[13]
    if not pkg: return "skip"
    if e not in ("native", "orchestration") or gen or (declared or {}).get("preparation") == "none": return None
    return ((declared.get("preparation") or declared.get("scope")) == "test") if e == "native" else False
for path, o in old.items():
    n = new.get(path)
    if n is None: diffs["manifest missing in new run"] += 1; samples["manifest missing in new run"].append(path[-60:]); continue
    om = {(c[0], c[1]): c for c in o["classes"]}; nm = {(c[0], c[1]): c for c in n["classes"]}
    if set(om) != set(nm): diffs["target set differs"] += 1; samples["target set differs"].append(path[-60:])
    for key, oc in om.items():
        nc = nm.get(key)
        if nc is None: continue
        targets += 1
        tuple_old = (oc[2], oc[3], oc[5], oc[6], oc[7], oc[10]); tuple_new = (nc[2], nc[3], nc[5], nc[6], nc[7], nc[10])
        if tuple_old != tuple_new: diffs["classification"] += 1; samples["classification"].append((path[-50:], key[1], tuple_old, tuple_new))
        po, pn = old_prep(oc), new_prep(nc)
        if po != pn: diffs["preparation"] += 1; samples["preparation"].append((path[-50:], key[1], po, pn))
print(json.dumps({"manifests_old": len(old), "manifests_new": len(new), "targets_compared": targets, "new_run_failures": len(fails), "diffs": dict(diffs)}))
for k, v in samples.items(): print(k, v[:8])
for f in fails[:10]: print("failure", f["path"][-60:], f["error"].split("\n")[0][:160])
sys.exit(1 if diffs or fails or len(old) != len(new) else 0)
