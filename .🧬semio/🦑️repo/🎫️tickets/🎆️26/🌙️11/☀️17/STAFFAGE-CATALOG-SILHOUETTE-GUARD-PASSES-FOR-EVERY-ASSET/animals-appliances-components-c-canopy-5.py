from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
n = d.shape(100, "elevation", "strut")
for i, (x, y) in enumerate([[3.8282, 0.0865], [0.0312, 2.8205], [0.0664, 2.8695], [3.8634, 0.1355]]):
    L.sn(d, n, "points", str(i), "0", value=x); L.sn(d, n, "points", str(i), "1", value=y)
d.commit()
