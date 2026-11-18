from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
for sid in ("clevis-wall", "clevis-beam"):
    o = d.obj(d.shape(50, "elevation", sid))
    d.insert_after(d.shape(100, "elevation", "strut"), o)
d.commit()
