import importlib, copy
L = importlib.import_module("author-50-lib")
s, d = L.load("park-bench")
R = d["asset"]["representations"]
def shp(sc, v, id): return next(x for x in R[sc]["geometry"][v] if x["id"] == id)
ops = []
base = shp("50", "plan", "seat-front")
slats = []
for k in range(4):
    x = copy.deepcopy(base); x["id"] = "seat-slat-%d" % (k + 1); x["y"] = round(-0.3 + k * 0.12375, 5); x["h"] = 0.11875; slats.append(x)
ops.append(("50", "plan", "seat-front", slats))
base = shp("50", "front", "seat-front")
slats = []
for k in range(4):
    x = copy.deepcopy(base); x["id"] = "seat-slat-%d" % (k + 1); x["x"] = round(-0.3 + k * 0.12375, 5); x["w"] = 0.11875; slats.append(x)
ops.append(("50", "front", "seat-front", slats))
for k in (1, 2, 3):
    x = copy.deepcopy(shp("50", "plan", "back-slat-%d" % k)); x["h"] = 0.0498
    ops.append(("50", "plan", "back-slat-%d" % k, [x]))
for sc in ("100", "200"):
    x = copy.deepcopy(shp(sc, "plan", "back-band")); x["y"] = 0.1701; x["h"] = 0.0958
    ops.append((sc, "plan", "back-band", [x]))
L.edit2d("park-bench", ops)
