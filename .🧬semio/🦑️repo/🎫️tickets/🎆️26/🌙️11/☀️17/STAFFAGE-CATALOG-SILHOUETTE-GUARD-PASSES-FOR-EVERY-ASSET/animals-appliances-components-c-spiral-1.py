from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/spiral-stair")
d.setv(d.at("asset", "dimensions", "heightM"), 3.75)
d.commit()
