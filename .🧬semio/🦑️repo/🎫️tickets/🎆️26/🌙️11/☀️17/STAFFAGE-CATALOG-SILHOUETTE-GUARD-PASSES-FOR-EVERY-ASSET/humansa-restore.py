"""Restores shapes of a humans asset to their HEAD version in place. python3 humansa-restore.py <slug> <scale:view:id,...>"""
import importlib.util, json, subprocess, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('ho', O + 'humans-ops.py'); ho = importlib.util.module_from_spec(spec); spec.loader.exec_module(ho)
slug = sys.argv[1]
F = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/" + slug + "/🔣️.json"
head = json.loads(subprocess.run(["git", "-C", "/Users/ueli/Documents/semio", "show", "HEAD:" + F], capture_output=True, text=True, encoding="utf8").stdout)["asset"]
ops = []
for item in sys.argv[2].split(","):
    sc, view, sid = item.split(":")
    shape = next(x for x in head["representations"][sc]["geometry"][view] if x["id"] == sid)
    ops.append({"op": "shape", "scale": sc, "view": view, "id": sid, "shape": shape})
ho.apply("👤️humans/" + slug, ops)
print(slug, "restored", len(ops))
