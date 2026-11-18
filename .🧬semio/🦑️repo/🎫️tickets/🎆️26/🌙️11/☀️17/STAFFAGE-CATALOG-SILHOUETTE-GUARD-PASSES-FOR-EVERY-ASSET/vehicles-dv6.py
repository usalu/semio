import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
for sc in (50, 100, 200, 500):
    off = 0.025 if sc != 500 else 0.04
    d.set(d.field(d.solid(sc, "rear-leaf-l"), "transform"), "translation", [0.675, 0.45, -off])
d.commit()
