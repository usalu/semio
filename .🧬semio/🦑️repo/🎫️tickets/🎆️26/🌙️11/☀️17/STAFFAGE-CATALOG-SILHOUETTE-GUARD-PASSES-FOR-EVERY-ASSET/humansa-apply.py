"""Dedupes an ops file of humansa-auto (last op per scale, view and shape id wins), writes poly shapes back as compact paths where the file draws a path, applies it to the asset file by span edits. python3 humansa-apply.py <slug> <ops.json> [--dry]"""
import importlib.util, json, sys
O = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/'
spec = importlib.util.spec_from_file_location('ho', O + 'humans-ops.py'); ho = importlib.util.module_from_spec(spec); spec.loader.exec_module(ho)
slug, f = sys.argv[1], sys.argv[2]
ops = json.load(open(f))
last = {}
order = []
for o in ops:
    k = (o["op"], o.get("scale"), o.get("view"), o.get("id"))
    if k not in last: order.append(k)
    last[k] = o
d = ho.s.Doc("👤️humans/" + slug)
res = []
for k in order:
    o = last[k]
    if o["op"] == "shape":
        n = ho.find(d, ["asset", "representations", o["scale"], "geometry", o["view"]], o["id"])
        orig = json.loads(d.t[n["s"]:n["e"]])
        sh = o["shape"]
        if orig.get("kind") == "path" and sh.get("kind") == "poly":
            new = {}
            for kk, v in sh.items():
                if kk == "kind": new["kind"] = "path"; new["d"] = "M " + " L ".join(" ".join(repr(round(c, 4)) for c in p) for p in sh["points"]) + " Z"
                elif kk in ("points", "closed"): continue
                else: new[kk] = v
            sh = new
        res.append({**o, "shape": sh})
    else: res.append(o)
if "--dry" in sys.argv: print(len(res), "ops"); sys.exit(0)
ho.apply("👤️humans/" + slug, res)
print(len(res), "ops applied")
