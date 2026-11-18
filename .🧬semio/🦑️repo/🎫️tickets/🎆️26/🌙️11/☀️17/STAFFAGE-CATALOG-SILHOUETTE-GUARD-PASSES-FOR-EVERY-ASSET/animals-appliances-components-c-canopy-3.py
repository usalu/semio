from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
g = d.solid(50, "glass-2")
for i, v in ((3, 0.05), (4, 0.05), (5, -0.05), (6, -0.05)): L.sn(d, g, "profile", str(i), "0", value=v)
n = d.shape(50, "plan", "glass-2")
for i, v in ((3, 0.05), (4, 0.05), (5, -0.05), (6, -0.05)): L.sn(d, n, "points", str(i), "1", value=v)
d.commit()
