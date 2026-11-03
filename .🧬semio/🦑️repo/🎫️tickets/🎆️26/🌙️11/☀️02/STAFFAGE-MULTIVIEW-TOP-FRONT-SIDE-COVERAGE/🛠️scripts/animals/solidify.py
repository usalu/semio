import json, re, sys, subprocess

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
KEEP = ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")
GEOM = ("kind", "points", "closed", "followTip", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")


def rng(s, view):
    k = s["kind"]
    if s.get("solid"):
        return None
    if view == "front":
        if k == "poly":
            v = [p[0] for p in s["points"]]
        elif k == "rect":
            v = [s["x"], s["x"] + s["w"]]
        elif k == "ellipse":
            v = [s["cx"] - s["rx"], s["cx"] + s["rx"]]
        elif k == "capsule":
            v = [s["x0"] - s["r0"], s["x0"] + s["r0"], s["x1"] - s["r1"], s["x1"] + s["r1"]]
        else:
            return None
    else:
        if k == "poly":
            v = [p[1] for p in s["points"]]
        elif k == "rect":
            v = [s["y"], s["y"] + s["h"]]
        elif k == "ellipse":
            v = [s["cy"] - s["ry"], s["cy"] + s["ry"]]
        elif k == "capsule":
            v = [s["y0"] - s["r0"], s["y0"] + s["r0"], s["y1"] - s["r1"], s["y1"] + s["r1"]]
        else:
            return None
    return min(v), max(v)


def bone_range(rep, bone):
    lo, hi = [], []
    for view in ("front", "plan"):
        for s in rep["geometry"][view]:
            if s["bone"] == bone and s["role"] != "ground":
                r = rng(s, view)
                if r:
                    lo.append(r[0])
                    hi.append(r[1])
    return (min(lo), max(hi)) if lo else None


def profile_of(e):
    return {k: e[k] for k in GEOM if k in e and k != "followTip"}


def solidify(asset, scale, view, index, length):
    rep = asset["representations"][str(scale)]
    s = rep["geometry"][view][index]
    if s.get("solid"):
        return False
    bone = s["bone"]
    if view == "elevation":
        r = bone_range(rep, bone) or (-0.03, 0.03)
        s["solid"] = {"kind": "extruded", "z0": round(r[0], 4), "z1": round(r[1], 4)}
        return True
    own = rng(s, view)
    if own is None:
        own = bone_range(rep, bone) or (-0.03, 0.03)
    elev = [e for e in rep["geometry"]["elevation"] if e["bone"] == bone and e["role"] != "ground"]
    same = [e for e in elev if e["id"] == s["id"]]
    pick = same[0] if same else (elev[0] if elev else None)
    if pick is not None:
        prof = profile_of(pick)
    else:
        r0, r1 = (s["r0"], s["r1"]) if s["kind"] == "capsule" else (max(abs(own[0]), abs(own[1])), max(abs(own[0]), abs(own[1])))
        prof = {"kind": "capsule", "x0": 0, "y0": 0, "x1": length, "y1": 0, "r0": r1, "r1": r1}
    new = {k: s[k] for k in KEEP if k in s}
    new.update(prof)
    new["solid"] = {"kind": "extruded", "z0": round(own[0], 4), "z1": round(own[1], 4)}
    rep["geometry"][view][index] = new
    return True


if __name__ == "__main__":
    slug = sys.argv[1]
    flagged = json.load(open(sys.argv[2], encoding="utf-8"))
    p = C + "🐕️animals/" + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8-sig"))
    a = d["asset"]
    L = {b["id"]: b["lengthM"] for b in a["rig"]["bones"]}
    n = 0
    for scale, view, index in flagged[slug]:
        s = a["representations"][str(scale)]["geometry"][view][index]
        if solidify(a, scale, view, index, L[s["bone"]]):
            n += 1
    open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print(slug, "solidified", n)
