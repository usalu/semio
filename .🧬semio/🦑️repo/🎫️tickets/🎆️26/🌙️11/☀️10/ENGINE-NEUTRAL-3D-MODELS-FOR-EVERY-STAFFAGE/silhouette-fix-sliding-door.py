import sys; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("sliding-door")
raw, doc, idx = load(p)
g = doc["asset"]["representations"]["50"]["geometry"]["plan"]
ops = []
for k, s in enumerate(g):
    if s["id"] in ("hanger-above-a", "hanger-above-b"):
        n = {x: y for x, y in s.items() if x not in ("cx", "cy", "rx", "ry", "kind")}
        n.update({"kind": "rect", "x": 1.32 if s["id"].endswith("a") else 1.64, "y": -0.025, "w": 0.04, "h": 0.05})
        ops.append(("replace", ("asset", "representations", "50", "geometry", "plan", k), n))
print(apply(p, ops, expect=raw))
