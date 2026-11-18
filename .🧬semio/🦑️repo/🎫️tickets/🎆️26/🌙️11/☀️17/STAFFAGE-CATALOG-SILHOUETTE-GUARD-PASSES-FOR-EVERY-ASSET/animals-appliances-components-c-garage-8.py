from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
for s in ("hinge-1", "hinge-2", "hinge-3"): L.setf(d, 50, "elevation", s, penMm=0.1)
d.commit()
