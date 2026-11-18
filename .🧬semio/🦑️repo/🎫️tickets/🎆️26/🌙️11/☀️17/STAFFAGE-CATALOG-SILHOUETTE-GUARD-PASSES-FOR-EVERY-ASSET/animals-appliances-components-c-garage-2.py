from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
for s in ("panel-1", "panel-2", "panel-3", "panel-4", "track"): L.setf(d, 200, "elevation", s, penMm=0.2)
d.commit()
