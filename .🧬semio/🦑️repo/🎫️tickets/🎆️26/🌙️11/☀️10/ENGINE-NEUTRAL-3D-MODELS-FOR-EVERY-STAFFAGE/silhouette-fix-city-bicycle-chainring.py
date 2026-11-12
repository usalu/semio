import sys; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("city-bicycle")
raw, doc, idx = load(p)
g = doc["asset"]["representations"]["50"]["geometry"]
ring = {"id": "chainring", "kind": "ellipse", "bone": "crank", "role": "detail", "z": 5.5, "filled": True, "tone": 0.6, "region": "drivetrain", "material": "alloy", "cx": 0, "cy": 0, "rx": 0.09, "ry": 0.09}
ops = []
for v, after in (("elevation", "crank-f"), ("front", "arm-f"), ("plan", "arm-f")):
    k = [s["id"] for s in g[v]].index(after)
    s = dict(ring)
    if v != "elevation": s["solid"] = {"kind": "extruded", "sweep": "w", "z0": -0.066, "z1": -0.058}
    ops.append(("insert_after", ("asset", "representations", "50", "geometry", v, k), [s]))
print(apply(p, ops, expect=raw))
