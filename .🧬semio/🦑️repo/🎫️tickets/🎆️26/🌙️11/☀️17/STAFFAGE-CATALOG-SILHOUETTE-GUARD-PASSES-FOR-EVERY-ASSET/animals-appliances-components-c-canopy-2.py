from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
L.rm_key(d, d.solid(500, "strut-tube"), "slender")
d.commit()
