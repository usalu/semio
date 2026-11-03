import json, sys

sys.path.insert(0, "🛠️scripts_placeholder")
C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
REVOLVED = {"red-fox": {"hindbody", "chest", "neck", "head"}}


def rng(s, view):
    k = s["kind"]
    if s.get("solid"):
        return None
    ax = 0 if view == "front" else 1
    if k == "poly":
        v = [p[ax] for p in s["points"]]
    elif k == "rect":
        v = [s["x"], s["x"] + s["w"]] if ax == 0 else [s["y"], s["y"] + s["h"]]
    elif k == "ellipse":
        c, r = (s["cx"], s["rx"]) if ax == 0 else (s["cy"], s["ry"])
        v = [c - r, c + r]
    elif k == "capsule":
        a, b = ("x0", "x1") if ax == 0 else ("y0", "y1")
        v = [s[a] - s["r0"], s[a] + s["r0"], s[b] - s["r1"], s[b] + s["r1"]]
    else:
        return None
    return min(v), max(v)


def bone_range(rep, bone, ids=None):
    lo, hi = [], []
    for view in ("front", "plan"):
        for s in rep["geometry"][view]:
            if s["bone"] == bone and s["role"] != "ground" and (ids is None or s["id"] in ids):
                r = rng(s, view)
                if r:
                    lo.append(r[0])
                    hi.append(r[1])
    return (min(lo), max(hi)) if lo else None


def run(slug, items):
    p = C + "🐕️animals/" + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8-sig"))
    a = d["asset"]
    L = {b["id"]: b["lengthM"] for b in a["rig"]["bones"]}
    n = 0
    plan = []
    for it in items:
        for scale, view in it["combos"]:
            plan.append((0 if view == "elevation" else 1, scale, view, it))
    plan.sort(key=lambda t: t[0])
    ranges = {}
    for _, scale, view, it in plan:
        rep = a["representations"][str(scale)]
        key = (scale, it["bone"])
        if key not in ranges:
            ranges[key] = bone_range(rep, it["bone"])
        for idx, s in enumerate(rep["geometry"][view]):
            if s["bone"] != it["bone"] or s["id"] not in it["ids"] or s.get("solid"):
                continue
            if view == "elevation":
                r = ranges[key] or (-0.03, 0.03)
                s["solid"] = {"kind": "extruded", "z0": round(r[0], 4), "z1": round(r[1], 4)}
                s.pop("followTip", None)
                n += 1
                continue
            own = rng(s, view) or ranges[key] or (-0.03, 0.03)
            elev = [e for e in rep["geometry"]["elevation"] if e["bone"] == s["bone"] and e["role"] != "ground"]
            same = [e for e in elev if e["id"] == s["id"]]
            pick = same[0] if same else (elev[0] if elev else None)
            if pick is not None:
                prof = {k: pick[k] for k in GEOM if k in pick}
            else:
                rr = max(abs(own[0]), abs(own[1]))
                prof = {"kind": "capsule", "x0": 0, "y0": 0, "x1": L[s["bone"]], "y1": 0, "r0": rr, "r1": rr}
            new = {k: s[k] for k in KEEP if k in s}
            new.update(prof)
            if s["id"] in REVOLVED.get(slug, ()) and pick is not None and view in ("front", "plan"):
                new["solid"] = {"kind": "revolved", "axisBone": s["bone"], "axisX": 0, "axisZ": 0, "mirror": True}
            else:
                new["solid"] = {"kind": "extruded", "z0": round(own[0], 4), "z1": round(own[1], 4)}
            rep["geometry"][view][idx] = new
            n += 1
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print(slug, "solidified", n)


if __name__ == "__main__":
    data = json.load(open(sys.argv[1], encoding="utf-8"))
    for slug in sys.argv[2:]:
        run(slug, data[slug])
