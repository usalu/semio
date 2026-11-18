import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sc in (100,):
    d.set(d.field(d.shape(sc, "front", "fork-l"), "solid"), "z0", 0.058)
    d.set(d.field(d.shape(sc, "front", "fork-r"), "solid"), "z1", -0.058)
d.commit()
