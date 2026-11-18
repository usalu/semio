from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
n = d.shape(50, "plan", "glass-2"); o = d.obj(n)
for k in ("x", "y", "w", "h"): o.pop(k)
o["kind"] = "poly"; o["closed"] = True
o["points"] = [[2.78, -0.5], [2.78, 0.5], [4.06, 0.5], [4.06, 0.055], [3.77, 0.055], [3.77, -0.055], [4.06, -0.055], [4.06, -0.5]]
d.replace_obj(n, o)
d.commit()
