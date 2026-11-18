from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/tied-canopy")
s = d.solid(500, "strut-tube")
d.edits.append((s["pairs"][-1][1]["e"], s["pairs"][-1][1]["e"], ',\n            "slender": true'))
L.sn(d, d.shape(500, "front", "strut"), "points", "0", "1", value=2.33)
d.commit()
