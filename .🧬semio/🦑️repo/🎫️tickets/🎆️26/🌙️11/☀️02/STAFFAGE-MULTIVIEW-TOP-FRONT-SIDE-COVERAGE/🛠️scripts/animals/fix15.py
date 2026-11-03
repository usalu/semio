import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
g = d["asset"]["representations"]["200"]["geometry"]
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
leg = {"hoof-fn": "cannon-fn", "hoof-ff": "cannon-ff", "hoof-hn": "metatarsus-hn", "hoof-hf": "metatarsus-hf"}
for e in [x for x in g["elevation"] if x["bone"] in leg]:
    ref = [s for s in g["front"] if s["bone"] == leg[e["bone"]] and s.get("solid")][0]
    n = {"id": e["id"].replace("mass-200-", ""), "bone": e["bone"]}
    n.update({k: e[k] for k in KEEP if k in e}); n.update({k: e[k] for k in GEOM if k in e})
    n["solid"] = dict(ref["solid"]); g["front"].append(n); print(n["id"], n["solid"])
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
