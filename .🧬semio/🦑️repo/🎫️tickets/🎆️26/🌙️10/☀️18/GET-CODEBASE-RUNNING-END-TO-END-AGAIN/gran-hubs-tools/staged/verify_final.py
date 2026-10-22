#!/usr/bin/env python3
"""🔎️ Verifies the manifests produced by an ordered list of plans with Bun's TOML parser (third party): every final manifest parses; the only removed dependency keys are hub packages; every added key is a workspace-declared producer or a path to a crate of that name (existing or created by a plan); no other entry changes. usage: verify_final.py <tag,tag,...> [--root DIR]"""
import json, os, re, subprocess, sys
import apply as A
HERE = os.path.dirname(os.path.abspath(__file__))
tags = sys.argv[1].split(",")
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else A.DEFAULT_ROOT
plans = A.load_plans(",".join(tags))
gen = os.path.join(HERE, "../../🗑️generated/gran-hubs")
roots = json.load(open(gen + "/manifests-roots.json")); man = json.load(open(gen + "/manifests.json"))
HUBS = {"semio-framework", "semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-job"}
created = {}
for p in plans:
    for c in p.get("creates", []): created[c["path"]] = c["content"]
files, manifests = A.collect(plans, set())
pairs, rels = [], []
for rel, entries in manifests.items():
    old = open(os.path.join(root, rel), encoding="utf-8").read()
    text = old
    for e in entries:
        text = e["new_text"] if A.sha(text.encode()) == e["sha256"] else A.new_text_of_manifest(text, e)
    pairs.append([old, text]); rels.append(rel)
out = json.loads(subprocess.run(["bun", os.path.join(HERE, "keys.ts")], input=json.dumps(pairs), capture_output=True, text=True).stdout)
def pkgname(d):
    p = d + "/Cargo.toml"
    t = created.get(p)
    if t is None:
        fp = os.path.join(root, p)
        if not os.path.isfile(fp): return None
        t = open(fp, encoding="utf-8").read()
    m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
    return m.group(1) if m else None
bad = []
for rel, r in zip(rels, out):
    d = os.path.dirname(rel)
    if r is None: bad.append((rel, "final text does not parse")); continue
    before, after = r
    for tb in set(before) | set(after):
        b, a = before.get(tb, {}), after.get(tb, {})
        for k in set(b) - set(a):
            pk = b[k].get("package", k) if isinstance(b[k], dict) else k
            if pk not in HUBS:
                # a key may move between tables: must exist in some table afterwards
                if not any(k in after.get(t2, {}) for t2 in after): bad.append((rel, f"removed {k} from {tb}"))
        for k in set(a) - set(b):
            v = a[k]
            if not isinstance(v, dict): bad.append((rel, f"added {k} not a table")); continue
            if v.get("workspace") is True:
                ws = man.get(d, {}).get("wsroot")
                if k not in roots.get(ws, {}): bad.append((rel, f"{k}: workspace root {ws} does not declare it"))
            elif "path" in v:
                tp = os.path.normpath(os.path.join(d, v["path"]))
                if pkgname(tp) != k: bad.append((rel, f"{k}: path {v['path'][-50:]} is not that crate"))
            else: bad.append((rel, f"{k}: neither workspace nor path"))
        for k in set(a) & set(b):
            if a[k] != b[k] and not (isinstance(b[k], dict) and b[k].get("package", k) in HUBS): bad.append((rel, f"changed {k} in {tb}"))
for rel, content in created.items():
    if rel.endswith("Cargo.toml"):
        o = json.loads(subprocess.run(["bun", os.path.join(HERE, "keys.ts")], input=json.dumps([[content, content]]), capture_output=True, text=True).stdout)
        if o[0] is None: bad.append((rel, "created manifest does not parse"))
print(f"{','.join(tags)}: {len(rels)} manifests + {sum(1 for r in created if r.endswith('Cargo.toml'))} created, problems {len(bad)}")
for b in bad[:20]: print("  ", b)
sys.exit(1 if bad else 0)
