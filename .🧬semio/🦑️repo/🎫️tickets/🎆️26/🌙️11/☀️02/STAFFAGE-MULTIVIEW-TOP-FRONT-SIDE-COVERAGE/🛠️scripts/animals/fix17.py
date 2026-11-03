import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
def rng(s, ax):
    if s["kind"] == "poly": v = [q[ax] for q in s["points"]]
    elif s["kind"] == "capsule":
        a, b = ("x0", "x1") if ax == 0 else ("y0", "y1"); v = [s[a] - s["r0"], s[a] + s["r0"], s[b] - s["r1"], s[b] + s["r1"]]
    else: return None
    return min(v), max(v)
want = {("neck", "plan"), ("tail-3", "front"), ("tail-3", "plan"), ("thigh-hn", "plan"), ("thigh-hf", "plan")}
n = 0
for sc in ("50", "100"):
    g = d["asset"]["representations"][sc]["geometry"]
    for view in ("front", "plan"):
        for i, s in enumerate(g[view]):
            if (s["bone"], view) in want and s["role"] == "body" and not s.get("solid"):
                el = [x for x in g["elevation"] if x["bone"] == s["bone"] and x["role"] == "body"]
                same = [x for x in el if x["id"] == s["id"]]
                e = (same or el)[0]
                lo, hi = rng(s, 0 if view == "front" else 1)
                t = {k: s[k] for k in KEEP if k in s}; t.update({k: e[k] for k in GEOM if k in e})
                t["solid"] = {"kind": "extruded", "z0": round(lo, 4), "z1": round(hi, 4)}
                g[view][i] = t; n += 1
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print(n)
