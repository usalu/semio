import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sc in (50, 100):
    for sid, x, y, w, h in (("roof-cap", -1.2, 2.863, 2.4, 0.118), ("battery-pack", -1.075, 2.975, 2.15, 0.145), ("hvac-unit", -1.0, 2.975, 2.0, 0.165)):
        s = d.shape(sc, "front", sid)
        for k, v in (("x", x), ("y", y), ("w", w), ("h", h), ("r", 0.005)): d.set(s, k, v)
d.commit()
