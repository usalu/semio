#!/usr/bin/env python3
"""🔎️ Verifies every manifest of a plan with Bun's TOML parser (third party): after-text parses, the only removed dependency keys are the hub, every added key is a workspace-declared producer or a path to a crate of that name, nothing else changed (values of untouched keys equal). usage: verify_manifests.py <tag> [--root DIR]"""
import json, os, subprocess, sys, re
HERE = os.path.dirname(os.path.abspath(__file__))
tag = sys.argv[1]
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else os.path.abspath(HERE + "/" + "/".join([".."] * 9))
plan = json.load(open(os.path.join(HERE, "plans", tag + ".json"), encoding="utf-8"))
roots = json.load(open(os.path.join(HERE, "../../🗑️generated/gran-hubs/manifests-roots.json")))
man = json.load(open(os.path.join(HERE, "../../🗑️generated/gran-hubs/manifests.json")))
pairs = [[open(os.path.join(root, m["path"]), encoding="utf-8").read(), m["new_text"]] for m in plan["manifests"]]
out = json.loads(subprocess.run(["bun", os.path.join(HERE, "keys.ts")], input=json.dumps(pairs), capture_output=True, text=True).stdout)
bad = []
hub = plan["hub_package"]
def pkgname(d):
    t = open(os.path.join(root, d, "Cargo.toml"), encoding="utf-8").read()
    m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
    return m.group(1) if m else None
for m, r in zip(plan["manifests"], out):
    d = os.path.dirname(m["path"])
    if r is None: bad.append((m["path"], "after-text does not parse")); continue
    before, after = r
    for tb in set(before) | set(after):
        b, a = before.get(tb, {}), after.get(tb, {})
        for k in set(b) - set(a):
            if k != hub and (b[k].get("package") if isinstance(b[k], dict) else None) != hub: bad.append((m["path"], f"removed {k} from {tb}"))
        for k in set(a) - set(b):
            v = a[k]
            if not isinstance(v, dict): bad.append((m["path"], f"added {k} not a table")); continue
            if v.get("workspace") is True:
                ws = man[d]["wsroot"]
                if k not in roots.get(ws, {}): bad.append((m["path"], f"{k}: workspace root {ws} does not declare it"))
            elif "path" in v:
                p = os.path.normpath(os.path.join(root, d, v["path"]))
                if not os.path.isfile(os.path.join(p, "Cargo.toml")) or pkgname(os.path.relpath(p, root)) != k: bad.append((m["path"], f"{k}: path {v['path'][:60]} is not that crate"))
            else: bad.append((m["path"], f"{k}: neither workspace nor path"))
        for k in set(a) & set(b):
            if a[k] != b[k]: bad.append((m["path"], f"changed {k} in {tb}"))
print(f"{tag}: {len(plan['manifests'])} manifests, problems {len(bad)}")
for b in bad[:15]: print("  ", b)
sys.exit(1 if bad else 0)
