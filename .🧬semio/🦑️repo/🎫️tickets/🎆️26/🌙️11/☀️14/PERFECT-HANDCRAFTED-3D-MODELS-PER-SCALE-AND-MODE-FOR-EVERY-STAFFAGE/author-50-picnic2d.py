import importlib, copy, json
L = importlib.import_module("author-50-lib")
s, d = L.load("picnic-table")
R = d["asset"]["representations"]
def shp(sc, v, id): return next(x for x in R[sc]["geometry"][v] if x["id"] == id)
def filled(sc, v, id, **kw):
    x = copy.deepcopy(shp(sc, v, id))
    for k in ("points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1"): x.pop(k, None)
    x["filled"] = True
    x.update(kw)
    return (sc, v, id, [x])
rect = lambda x, y, w, h: dict(kind="rect", x=x, y=y, w=w, h=h)
poly = lambda pts: dict(kind="poly", points=pts, closed=True)
ops = [
    filled("500", "elevation", "leg-l", **poly([[-0.285, 0.7], [-0.175, 0.7], [-0.62, 0.0], [-0.73, 0.0]])),
    filled("500", "elevation", "leg-r", **poly([[0.175, 0.7], [0.285, 0.7], [0.73, 0.0], [0.62, 0.0]])),
    filled("500", "elevation", "bench-r", **rect(-0.11, -0.0375, 0.41, 0.075)),
    filled("500", "elevation", "bench-l", **rect(-0.11, -0.0375, 0.41, 0.075)),
    filled("500", "front", "leg-0", **rect(0.76, 0.0, 0.08, 0.7)),
    filled("500", "front", "leg-1", **rect(-0.84, 0.0, 0.08, 0.7)),
    filled("500", "plan", "leg-l-0", **rect(-0.73, 0.76, 0.555, 0.08)),
    filled("500", "plan", "leg-r-0", **rect(0.175, 0.76, 0.555, 0.08)),
    filled("500", "plan", "leg-l-1", **rect(-0.73, -0.84, 0.555, 0.08)),
    filled("500", "plan", "leg-r-1", **rect(0.175, -0.84, 0.555, 0.08)),
]
for v in ("front", "plan"):
    for b in ("bench-r", "bench-l"):
        x = copy.deepcopy(shp("500", v, b)); x["filled"] = True
        ops.append(("500", v, b, [x]))
L.edit2d("picnic-table", ops)
