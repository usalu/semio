import sys, math; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("cargo-bike")
raw, doc, idx = load(p)
a = doc["asset"]
def ring(R, r, n=72):
    out = [(R * math.cos(2 * math.pi * i / n), R * math.sin(2 * math.pi * i / n)) for i in range(n)]
    inn = [(r * math.cos(-2 * math.pi * i / n), r * math.sin(-2 * math.pi * i / n)) for i in range(n)]
    f = lambda pts: "M " + " L ".join(f"{x:.4f},{y:.4f}" for x, y in pts) + " Z"
    return f(out) + " " + f(inn)
ops = []
for sc in ("50", "100", "200"):
    g = a["representations"][sc]["geometry"]["elevation"]
    for k, s in enumerate(g):
        if s["id"] in ("tyre-r", "tyre-f") and s["kind"] == "ellipse":
            R = s["rx"]
            n = {x: y for x, y in s.items() if x not in ("cx", "cy", "rx", "ry", "kind")}
            n["kind"] = "path"
            n["d"] = ring(R, R - 0.054)
            ops.append(("replace", ("asset", "representations", sc, "geometry", "elevation", k), n))
            print(sc, s["id"], R)
print(apply(p, ops, expect=raw))
