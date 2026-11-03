import json, copy
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
reps = d["asset"]["representations"]
src = {e["id"]: e for e in reps["100"]["geometry"]["elevation"] if e["id"] in ("teat-front", "teat-rear")}
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
if not any(e["id"] == "teat-front" for e in reps["50"]["geometry"]["elevation"]):
    for k in ("teat-front", "teat-rear"):
        reps["50"]["geometry"]["elevation"].append(copy.deepcopy(src[k]))
for sc in ("50", "100"):
    g = reps[sc]["geometry"]["plan"]
    if any(s["id"].startswith("teat") for s in g): continue
    for k, tag in (("teat-front", "front"), ("teat-rear", "rear")):
        e = src[k]
        for side, (z0, z1) in (("near", (-0.076, -0.044)), ("far", (0.044, 0.076))):
            t = {x: e[x] for x in ("role", "z", "filled", "tone", "region", "regions", "material") if x in e}
            t.update({"id": f"teat-{tag}-{side}", "bone": "udder"}); t.update({x: e[x] for x in GEOM if x in e})
            t["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}; g.append(t)
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
