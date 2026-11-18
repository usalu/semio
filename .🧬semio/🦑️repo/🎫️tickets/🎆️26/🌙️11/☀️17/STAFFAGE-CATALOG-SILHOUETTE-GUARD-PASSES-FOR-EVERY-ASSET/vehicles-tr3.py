import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    s = d.shape(sc, "plan", "gangway")
    print(sc, d.obj(s))
    if "r" in d.obj(s): d.set(s, "r", 0.0)
d.commit()
