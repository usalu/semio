import json, sys
sys.path.insert(0, "x")
C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
def rng(s):
    k = s["kind"]
    if k == "poly": v = [p[0] for p in s["points"]]
    elif k == "ellipse": v = [s["cx"] - s["rx"], s["cx"] + s["rx"]]
    elif k == "rect": v = [s["x"], s["x"] + s["w"]]
    return min(v), max(v)
def run(slug, ids, alias):
    p = C + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8")); n = 0
    for sc, r in d["asset"]["representations"].items():
        if r["status"] != "authored": continue
        for i, s in enumerate(r["geometry"]["front"]):
            if s["id"] in ids and s["role"] == "body" and not s.get("solid"):
                tid = alias.get(s["id"], s["id"])
                e = [x for x in r["geometry"]["elevation"] if x["bone"] == s["bone"] and x["id"] == tid]
                if not e: print("no twin", slug, sc, s["id"]); continue
                lo, hi = rng(s)
                new = {k: s[k] for k in KEEP if k in s}
                new.update({k: e[0][k] for k in GEOM if k in e[0]})
                new["solid"] = {"kind": "extruded", "z0": round(lo, 4), "z1": round(hi, 4)}
                r["geometry"]["front"][i] = new; n += 1
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print(slug, n)
run("horse", {"neck-1", "neck-2", "neck-3", "head", "muzzle", "neck-a", "neck-b"}, {})
run("labrador-dog", {"head", "nose"}, {"nose": "nose-leather"})
