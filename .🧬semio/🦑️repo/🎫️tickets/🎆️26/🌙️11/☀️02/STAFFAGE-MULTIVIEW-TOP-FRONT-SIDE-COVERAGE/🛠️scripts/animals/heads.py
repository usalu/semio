import json, sys

C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
KEEP = ("role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")


def pts_of(s):
    k = s["kind"]
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "ellipse":
        return [(s["cx"] - s["rx"], s["cy"] - s["ry"]), (s["cx"] + s["rx"], s["cy"] + s["ry"])]
    if k == "rect":
        return [(s["x"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"])]
    if k == "capsule":
        return [(s["x0"] - s["r0"], s["y0"] - s["r0"]), (s["x1"] + s["r1"], s["y1"] + s["r1"])]
    return []


def lateral(s, view):
    k = s["kind"]
    sol = s.get("solid")
    if sol and sol["kind"] == "extruded":
        return abs(sol["z0"]), abs(sol["z1"])
    ax = 0 if view == "front" else 1
    if k == "poly":
        v = [p[ax] for p in s["points"]]
    elif k == "ellipse":
        c, r = (s["cx"], s["rx"]) if ax == 0 else (s["cy"], s["ry"])
        v = [c - r, c + r]
    elif k == "capsule":
        a, b = ("x0", "x1") if ax == 0 else ("y0", "y1")
        v = [s[a] - s["r0"], s[a] + s["r0"], s[b] - s["r1"], s[b] + s["r1"]]
    elif k == "rect":
        v = [s["x"], s["x"] + s["w"]] if ax == 0 else [s["y"], s["y"] + s["h"]]
    else:
        return 0, 0
    return abs(min(v)), abs(max(v))


def head_profile(a0, a1, wc, wm):
    length = a1 - a0
    return [
        [round(a0, 4), 0.0],
        [round(a0 + 0.03 * length, 4), round(0.55 * wc, 4)],
        [round(a0 + 0.22 * length, 4), round(wc, 4)],
        [round(a0 + 0.5 * length, 4), round(0.9 * wc, 4)],
        [round(a0 + 0.72 * length, 4), round(max(wm, 0.55 * wc), 4)],
        [round(a1 - 0.03 * length, 4), round(0.85 * max(wm, 0.55 * wc), 4)],
        [round(a1, 4), round(0.6 * wm, 4)],
        [round(a1, 4), 0.0],
    ]


def run(slug, scales, bone="head", head_ids=("head", "muzzle", "nose"), wm_ids=("muzzle", "nose")):
    p = C + slug + "/🔣️.json"
    d = json.load(open(p, encoding="utf-8"))
    for sc in scales:
        r = d["asset"]["representations"][str(sc)]
        g = r["geometry"]
        elev = [e for e in g["elevation"] if e["bone"] == bone and e["role"] == "body"]
        allp = [q for e in elev for q in pts_of(e)]
        a0, a1 = min(q[0] for q in allp), max(q[0] for q in allp)
        for view in ("front", "plan"):
            mine = [s for s in g[view] if s["bone"] == bone and s["role"] == "body" and s["id"] in head_ids]
            if not mine:
                continue
            wc = max(max(lateral(s, view)) for s in mine if s["id"] == "head") if any(s["id"] == "head" for s in mine) else max(max(lateral(s, view)) for s in mine)
            wms = [max(lateral(s, view)) for s in mine if s["id"] in wm_ids]
            wm = max(wms) if wms else 0.55 * wc
            base = mine[0]
            new = {"id": "head", "bone": bone}
            new.update({k: base[k] for k in KEEP if k in base})
            new.update({"kind": "poly", "points": head_profile(a0, a1, wc, wm), "closed": True})
            new["solid"] = {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
            out, placed = [], False
            for s in g[view]:
                if s in mine:
                    if not placed:
                        out.append(new)
                        placed = True
                else:
                    out.append(s)
            g[view] = out
            print(slug, sc, view, "wc", round(wc, 3), "wm", round(wm, 3), "a", round(a0, 3), round(a1, 3))
    if "--dry" not in sys.argv:
        open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    return d


if __name__ == "__main__":
    slug = sys.argv[1]
    scales = [int(x) for x in sys.argv[2].split(",")]
    run(slug, scales)
