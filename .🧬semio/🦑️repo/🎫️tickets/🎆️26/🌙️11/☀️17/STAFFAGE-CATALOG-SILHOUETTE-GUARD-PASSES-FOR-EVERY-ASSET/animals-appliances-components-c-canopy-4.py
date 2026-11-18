from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
d.set(d.solid(50, "cheek-beam-l"), "from", 0.041)
d.set(d.solid(50, "cheek-beam-r"), "to", -0.041)
d.commit()
