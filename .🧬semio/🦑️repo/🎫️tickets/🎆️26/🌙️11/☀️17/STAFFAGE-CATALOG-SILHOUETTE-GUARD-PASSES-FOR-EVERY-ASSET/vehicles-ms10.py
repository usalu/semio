import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sc in (50, 100):
    s = d.solid(sc, "fender-f")
    for k in range(13, 26):
        d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), 0.245)
d.commit()
