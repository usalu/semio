"""🧲 Replaces drawn shapes of a view by the convex hull of the same-named volume solids. python3 humansb-C2-hullall.py <scale> <view> <out.json> <patches|-> <id[=solid,solid]> ..."""
import json, subprocess, sys
N = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
F = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/cyclist/🔣️.json"
scale, view, out, patches, *specs = sys.argv[1:]
d = json.load(open(F))["asset"]
shapes = {s["id"]: s for s in d["representations"][scale]["geometry"][view]}
ops = []
for spec in specs:
    sid, _, solids = spec.partition("=")
    solids = solids or sid
    bone = shapes[sid]["bone"]
    r = subprocess.run([N + "/humansb-C-hullbatch.sh", "cyclist", scale, view, bone, "/tmp/humansb-C2-one.json", patches, "replace", f"{sid} {sid} {solids}"], capture_output=True, text=True)
    if r.returncode != 0: print("FAIL", sid, r.stderr[-200:]); continue
    ops += json.load(open("/tmp/humansb-C2-one.json"))
json.dump(ops, open(out, "w")); print(len(ops), "ops")
