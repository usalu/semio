from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/spiral-stair")
for v in ("elevation", "front", "plan"): L.setf(d, 50, v, "handrail", penMm=0.85)
L.rm_key(d, d.solid(50, "baluster"), "slender")
d.commit()
