import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    s = d.shape(sc, "elevation", "gangway")
    pts = d.obj(s)["points"]
    d.drawn_array(s, "points", [[round(x + (0.012 if x > 0 else -0.012), 4), y] for x, y in pts])
    s = d.shape(sc, "plan", "gangway")
    o = d.obj(s)
    d.set(s, "x", round(o["x"] - 0.012, 4)); d.set(s, "w", round(o["w"] + 0.024, 4))
d.commit()
