import sys, copy; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("city-bicycle")
raw, doc, idx = load(p)
a = doc["asset"]
ops = []
def member(old, sgn):
    keep = {k: v for k, v in old.items() if k in ("id", "bone", "role", "z", "tone", "region", "material")}
    n = {"kind": "member", "from": {"x": -0.2145, "y": -0.19755, "z": 0.0}, "to": {"x": -0.21597, "y": -0.26299, "z": sgn * 0.26}, "section": {"kind": "circle", "d": 0.028}}
    n.update(keep); n["filled"] = False
    return n
g50 = a["representations"]["50"]["geometry"]["front"]
grips = [s for s in g50 if s["id"] in ("grip-l", "grip-r")]
for sc, r in a["representations"].items():
    g = r.get("geometry")
    if g is None: continue
    for v in ("front", "plan"):
        for k, s in enumerate(g[v]):
            if s["id"] in ("handlebar-l", "handlebar-r") and s["kind"] == "capsule":
                ops.append(("replace", ("asset", "representations", sc, "geometry", v, k), member(s, 1 if s["id"].endswith("l") else -1)))
    if sc in ("100", "200") and not any(s["id"] == "grip-l" for s in g["front"]):
        k = [s["id"] for s in g["front"]].index("handlebar-r")
        ops.append(("insert_after", ("asset", "representations", sc, "geometry", "front", k), copy.deepcopy(grips)))
print(len(ops), apply(p, ops, expect=raw))
