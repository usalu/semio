import sys, copy; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("person-with-umbrella")
raw, doc, idx = load(p)
a = doc["asset"]
ops = []
for sc, r in a["representations"].items():
    el = r["geometry"]["elevation"]
    for k, s in enumerate(el):
        if s["id"] in ("canopy-front", "canopy-back"):
            front = s["id"] == "canopy-front"
            new = {"id": "canopy-core-" + ("front" if front else "back"), "kind": "poly", "bone": s["bone"], "role": "body", "z": 5.9, "filled": True, "tone": s["tone"], "region": "canopy", "hatch": s.get("hatch"), "closed": True,
                   "points": [[0, 0], [0.56, 0], [0.0926, -0.2081 if front else 0.2081]]}
            if new["hatch"] is None: del new["hatch"]
            ops.append(("insert_after", ("asset", "representations", sc, "geometry", "elevation", k), [new]))
print(apply(p, ops, expect=raw))
