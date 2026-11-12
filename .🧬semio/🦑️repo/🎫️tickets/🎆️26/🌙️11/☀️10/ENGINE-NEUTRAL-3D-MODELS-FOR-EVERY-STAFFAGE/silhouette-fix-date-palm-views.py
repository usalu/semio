import sys, json; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("date-palm")
raw, doc, idx = load(p)
a = doc["asset"]
baked = json.load(open("🗑️generated/silhouette/dp-plan-baked.json"))
g = a["representations"]["50"]["geometry"]
el = {s["id"]: s for s in g["elevation"]}
SRC = {"stem-1": "stem-1", "stem-2": "stem-2", "stem-3": "stem-3", "crown-shaft": "crown-shaft", "skirt": "skirt-w1", "bunch-l": "bunch-l", "bunch-r": "bunch-r"}
BONE = {"stem-1": "stem-1", "stem-2": "stem-2", "stem-3": "stem-3", "crown-shaft": "crown", "skirt": "stem-3", "bunch-l": "bunch-l", "bunch-r": "bunch-r"}
shapes = []
for gid, rings in baked.items():
    t = el[SRC[gid]]
    for n, ring in enumerate(rings):
        s = {"id": gid if len(rings) == 1 else f"{gid}-{n + 1}", "kind": "poly", "bone": BONE[gid], "role": "body", "z": 1.0, "filled": True, "tone": t["tone"], "closed": True, "points": ring}
        for k in ("region", "regions", "material"):
            if k in t: s[k] = t[k]
        shapes.append(s)
ops = [("insert_after", ("asset", "representations", "50", "geometry", "plan", 0), shapes)]
for k, s in enumerate(g["front"]):
    h = {"bole-stem-1": 0.287, "bole-stem-2": 0.261, "bole-stem-3": 0.244}.get(s["id"])
    if h is not None:
        n = dict(s); n["x"] = -h; n["w"] = round(2 * h, 4)
        ops.append(("replace", ("asset", "representations", "50", "geometry", "front", k), n))
print(len(shapes), apply(p, ops, expect=raw))
