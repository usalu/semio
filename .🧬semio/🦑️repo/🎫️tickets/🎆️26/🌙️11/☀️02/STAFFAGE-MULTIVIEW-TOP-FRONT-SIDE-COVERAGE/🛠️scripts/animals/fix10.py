import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
n = 0
for sc, r in d["asset"]["representations"].items():
    if r["status"] != "authored": continue
    elev = [e for e in r["geometry"]["elevation"] if e["bone"] == "neck" and e["role"] == "body"][0]
    for i, s in enumerate(r["geometry"]["front"]):
        if s["bone"] == "neck" and s["role"] == "body" and not s.get("solid"):
            zr = 0.19 if s["kind"] == "poly" else s["rx"]
            new = {k: s[k] for k in KEEP if k in s}
            new.update({k: elev[k] for k in GEOM if k in elev})
            new["solid"] = {"kind": "extruded", "z0": -zr, "z1": zr}
            r["geometry"]["front"][i] = new; n += 1
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print(n)
