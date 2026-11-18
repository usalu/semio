import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
r = float(sys.argv[1])
for sid in ("tyre-f", "tyre-r"):
    s = d.shape(500, "elevation", sid)
    d.set(s, "rx", r); d.set(s, "ry", r)
d.commit()
