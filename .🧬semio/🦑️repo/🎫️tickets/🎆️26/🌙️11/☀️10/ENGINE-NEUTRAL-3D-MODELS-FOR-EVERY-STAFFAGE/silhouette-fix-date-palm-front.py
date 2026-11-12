import sys; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("date-palm")
raw, doc, idx = load(p)
H = {"bole-stem-1": 0.287, "bole-stem-2": 0.261, "bole-stem-3": 0.244, "bole-crown": 0.337}
ops = []
for k, s in enumerate(doc["asset"]["representations"]["50"]["geometry"]["front"]):
    if s["id"] in H:
        n = {x: y for x, y in s.items() if x not in ("x", "w")}
        top = max(abs(q[0]) for q in n["points"])
        n["points"] = [[round(q[0] * H[s["id"]] / top, 4), q[1]] for q in n["points"]]
        ops.append(("replace", ("asset", "representations", "50", "geometry", "front", k), n))
print(apply(p, ops, expect=raw))
