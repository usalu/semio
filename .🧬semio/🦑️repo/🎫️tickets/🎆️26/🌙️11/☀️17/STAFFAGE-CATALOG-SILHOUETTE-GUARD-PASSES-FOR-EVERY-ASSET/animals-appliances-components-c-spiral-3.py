from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/spiral-stair")
L.rm_key(d, d.solid(50, "handrail"), "slender")
d.commit()
