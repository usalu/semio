import json
p = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
reps = d["asset"]["representations"]
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
def xr(s, ax):
    if s["kind"] == "poly": v = [q[ax] for q in s["points"]]
    elif s["kind"] == "capsule":
        a, b = ("x0", "x1") if ax == 0 else ("y0", "y1"); v = [s[a] - s["r0"], s[a] + s["r0"], s[b] - s["r1"], s[b] + s["r1"]]
    return min(v), max(v)
def solid_from(s, prof, view):
    lo, hi = xr(s, 0 if view == "front" else 1)
    n = {k: s[k] for k in KEEP if k in s}
    n.update({k: prof[k] for k in GEOM if k in prof})
    n["solid"] = {"kind": "extruded", "z0": round(lo, 4), "z1": round(hi, 4)}
    return n
count = 0
for sc, r in reps.items():
    if r["status"] != "authored": continue
    g = r["geometry"]
    def elev_body(bone):
        e = [x for x in g["elevation"] if x["bone"] == bone and x["role"] == "body"]
        return e
    for view in ("front", "plan"):
        for i, s in enumerate(g[view]):
            if s.get("solid"): continue
            if s["bone"] == "udder" and s["role"] == "body":
                if s["id"].startswith("teat"):
                    src = [x for x in reps["100"]["geometry"]["elevation"] if x["id"] == "teat-front"][0]
                    g[view][i] = solid_from(s, src, view); count += 1
                else:
                    e = elev_body("udder")
                    e = [x for x in e if not x["id"].startswith("teat")]
                    g[view][i] = solid_from(s, e[0], view); count += 1
            elif sc == "200" and view == "front" and s["bone"] in ("cannon-ff", "cannon-fn", "humerus-ff", "humerus-fn", "radius-ff", "radius-fn", "shank-hf", "shank-hn", "metatarsus-hf", "metatarsus-hn", "thigh-hf", "thigh-hn") and s["role"] == "body" and s["kind"] == "poly":
                e = elev_body(s["bone"])
                if len(e) == 1:
                    g[view][i] = solid_from(s, e[0], view); count += 1
open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
print("converted", count)
