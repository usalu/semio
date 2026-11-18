import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
s = d.shape(50, "elevation", "rear-leaf-l")
sol = d.field(s, "solid")
d.set(sol, "z0", 0.83); d.set(sol, "z1", 0.88)
d.commit()
