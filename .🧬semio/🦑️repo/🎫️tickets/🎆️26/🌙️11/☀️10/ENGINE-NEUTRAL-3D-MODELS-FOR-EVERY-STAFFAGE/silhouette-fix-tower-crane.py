import sys, math; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("tower-crane")
raw, doc, idx = load(p)
a = doc["asset"]
FAMILY = {"mast": ("mast-chord", "mast-tie", "mast-diag"), "jib": ("jib-chord", "jib-rail", "jib-diag", "jib-brace"), "counter-jib": ("counter-chord", "counter-diag", "counter-brace")}
def qrot(q, v):
    x, y, z, w = q
    t = [2 * (y * v[2] - z * v[1]), 2 * (z * v[0] - x * v[2]), 2 * (x * v[1] - y * v[0])]
    return [v[0] + w * t[0] + (y * t[2] - z * t[1]), v[1] + w * t[1] + (z * t[0] - x * t[2]), v[2] + w * t[2] + (x * t[1] - y * t[0])]
def members(bone):
    out = []
    for s in a["volume"]["solids"]:
        if s["bone"] != bone or not s["id"].startswith(FAMILY[bone]): continue
        if s["kind"] == "capsule":
            ends = [list(s["p0"]), list(s["p1"])]
            section = {"kind": "circle", "d": round(2 * s["r0"], 4)}
        elif s["kind"] == "box":
            size = s["size"]; t = s["transform"]
            half = size[1] / 2
            ends = [[0, -half, 0], [0, half, 0]]
            ends = [[c + d for c, d in zip(qrot(t["rotation"], e), t["translation"])] for e in ends]
            section = {"kind": "rect", "w": size[0], "h": size[2]}
        else:
            raise SystemExit("unexpected " + s["id"])
        rep = s.get("repeat") or {"count": 1, "translation": [0, 0, 0]}
        for k in range(rep["count"]):
            step = [c * k for c in rep["translation"]]
            out.append((s["id"], k, [[round(e[i] + step[i], 5) for i in range(3)] for e in ends], section, s["role"]))
    return out
def shapes(bone, z):
    res = []
    for sid, k, ends, section, role in members(bone):
        res.append({"kind": "member", "from": {"x": ends[0][0], "y": ends[0][1], "z": ends[0][2]}, "to": {"x": ends[1][0], "y": ends[1][1], "z": ends[1][2]}, "section": section,
                    "id": f"{sid}-{k + 1}", "bone": bone, "role": "body" if sid.split("-")[1] in ("chord",) else "detail", "z": z, "filled": False, "tone": 0.5, "region": "lattice", "material": "steel-s355"})
    return res
REPLACE = {
    "elevation": {"mast": ["mast-chords", "mast-diagonals", "mast-horizontals"], "jib": ["jib-outline", "jib-diagonals", "jib-rail"], "counter-jib": ["counter-outline", "counter-diagonals"]},
    "front": {"mast": ["mast-chord-l", "mast-chord-r", "mast-diagonals", "mast-ties"] + [f"mast-tie-{i}" for i in range(3, 38, 2)], "jib": [], "counter-jib": []},
    "plan": {"jib": ["jib-chord-l", "jib-chord-r", "jib-diagonals", "jib-rail-l", "jib-rail-r"], "counter-jib": ["counter-chord-l", "counter-chord-r", "counter-diagonals"], "mast": []},
}
ops = []
g = a["representations"]["50"]["geometry"]
for view, per in REPLACE.items():
    ids = [s["id"] for s in g[view]]
    for bone, old in per.items():
        new = shapes(bone, 3)
        if view == "front" and bone != "mast": continue
        if view == "plan" and bone == "mast": continue
        slots = [ids.index(i) for i in old if i in ids]
        if not slots:
            continue
        for n, k in enumerate(slots):
            ops.append(("replace", ("asset", "representations", "50", "geometry", view, k), new[n]))
        rest = new[len(slots):]
        if rest: ops.append(("insert_after", ("asset", "representations", "50", "geometry", view, slots[-1]), rest))
        print(view, bone, len(old), "slots", len(slots), "new", len(new))
print(apply(p, ops, expect=raw))
