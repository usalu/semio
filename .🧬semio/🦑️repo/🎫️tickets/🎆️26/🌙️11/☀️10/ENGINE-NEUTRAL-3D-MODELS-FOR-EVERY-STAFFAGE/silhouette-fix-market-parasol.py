import sys; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("market-parasol")
raw, doc, idx = load(p)
a = doc["asset"]
ops = []
for sc, r in a["representations"].items():
    g = r.get("geometry")
    if g is None: continue
    el = g["elevation"]
    for k, s in enumerate(el):
        if s["id"] in ("panel-right", "panel-left"):
            right = s["id"] == "panel-right"
            n = {"id": s["id"] + "-core", "kind": "poly", "bone": s["bone"], "role": "body", "z": 1.9, "filled": True, "tone": s["tone"], "region": s["region"], "material": s["material"], "closed": True,
                 "points": [[0, 0], [1.2374, 0], [0.0859, -0.3145 if right else 0.3145]]}
            ops.append(("insert_after", ("asset", "representations", sc, "geometry", "elevation", k), [n]))
print(len(ops), apply(p, ops, expect=raw))
