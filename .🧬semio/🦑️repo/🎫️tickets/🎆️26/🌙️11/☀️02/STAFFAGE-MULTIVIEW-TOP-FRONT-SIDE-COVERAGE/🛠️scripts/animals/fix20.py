import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
reps = d["asset"]["representations"]
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
for sc in ("50", "100"):
    g = reps[sc]["geometry"]
    if any(s["id"].startswith("teat-rear") for s in g["front"]): continue
    e = [x for x in g["elevation"] if x["id"] == "teat-rear"][0]
    for side, (z0, z1) in (("near", (-0.076, -0.044)), ("far", (0.044, 0.076))):
        t = {x: e[x] for x in ("role", "z", "filled", "tone", "region", "regions", "material") if x in e}
        t.update({"id": f"teat-rear-{side}", "bone": "udder", "z": t.get("z", 3) - 0.1}); t.update({x: e[x] for x in GEOM if x in e})
        t["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}; g["front"].append(t)
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
