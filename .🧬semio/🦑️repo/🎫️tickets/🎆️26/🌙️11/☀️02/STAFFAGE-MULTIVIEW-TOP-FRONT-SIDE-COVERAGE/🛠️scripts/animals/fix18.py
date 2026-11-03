import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
def rng(s):
    if s["kind"] == "poly": v = [q[0] for q in s["points"]]
    elif s["kind"] == "ellipse": v = [s["cx"] - s["rx"], s["cx"] + s["rx"]]
    elif s["kind"] == "capsule": v = [s["x0"] - s["r0"], s["x0"] + s["r0"], s["x1"] - s["r1"], s["x1"] + s["r1"]]
    return min(v), max(v)
bones = {"lumbar", "thigh-hf", "thigh-hn", "shank-hf", "shank-hn"}
n = 0
for sc in ("50", "100"):
    g = d["asset"]["representations"][sc]["geometry"]
    for i, s in enumerate(g["front"]):
        if s["bone"] in bones and s["role"] == "body" and not s.get("solid"):
            el = [x for x in g["elevation"] if x["bone"] == s["bone"] and x["role"] == "body"]
            if not el: continue
            e = ([x for x in el if x["id"] == s["id"]] or el)[0]
            lo, hi = rng(s)
            t = {k: s[k] for k in KEEP if k in s}; t.update({k: e[k] for k in GEOM if k in e})
            t["solid"] = {"kind": "extruded", "z0": round(lo, 4), "z1": round(hi, 4)}
            g["front"][i] = t; n += 1
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print(n)
