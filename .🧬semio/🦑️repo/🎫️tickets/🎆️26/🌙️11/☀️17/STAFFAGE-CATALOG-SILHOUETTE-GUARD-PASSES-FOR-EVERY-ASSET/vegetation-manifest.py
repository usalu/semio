"""Writes the declared dimensions of the vegetation assets into their entries of the catalog manifest (the asset file is the single source of truth; value edits only): python3 vegetation-manifest.py"""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
sp = importlib.import_module("sil-span")
CAT = sp.CAT
path = CAT + "🔣️.json"
t = open(path, encoding="utf8").read()
root = sp.parse(t)
m = json.loads(t)
edits = []
for i, e in enumerate(m["entries"]):
    f = CAT + "🌳️vegetation/" + e["id"] + "/🔣️.json" if "id" in e else None
    import os
    slug = e.get("id") or e.get("slug")
    f = CAT + "🌳️vegetation/" + str(slug) + "/🔣️.json"
    if not os.path.exists(f): continue
    dims = json.load(open(f, encoding="utf8"))["asset"]["dimensions"]
    for k in ("widthM", "heightM", "depthM"):
        if k in e and abs(e[k] - dims[k]) > 1e-9:
            node = sp.walk(root, ["entries", str(i), k]); edits.append((node["s"], node["e"], sp.flat(dims[k]))); print(slug, k, e[k], "->", dims[k])
for a, b, x in sorted(edits, reverse=True): t = t[:a] + x + t[b:]
json.loads(t)
open(path, "w", encoding="utf8").write(t); print("manifest values", len(edits))
