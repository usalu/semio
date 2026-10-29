import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "waste-bin")
BH, LH, LF = ("bin-hdpe", "hdpe-plastic"), ("lid-hdpe", "hdpe-plastic"), ("liner-film", "ldpe-film")
if not any(m["id"] == "ldpe-film" for m in a["materials"]):
    a["materials"].append({"id": "ldpe-film", "title": T("LDPE liner film", "LDPE-Einlegesack"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 920, "source": "authored-estimate"})
if not any(r["id"] == "liner-film" for r in a["thermal"]["regions"]):
    a["thermal"]["regions"].append({"id": "liner-film", "title": T("Liner bag collar", "Sackkragen"), "kind": "surface", "material": "ldpe-film", "model": "passive", "areaM2": 0.08, "characteristicLengthM": 0.5, "source": "authored-estimate"})
if not any(r["id"] == "bin-ribs" for r in a["thermal"]["regions"]):
    for r in a["thermal"]["regions"]:
        if r["id"] == "bin-hdpe": r["areaM2"] = 1.64
    a["thermal"]["regions"].append({"id": "bin-ribs", "title": T("Moulded reinforcing bands", "Angeformte Verstärkungsbänder"), "kind": "surface", "material": "hdpe-plastic", "model": "passive", "areaM2": 0.16, "characteristicLengthM": 0.5, "source": "authored-estimate"})
BR = ("bin-ribs", "hdpe-plastic")
def hw(y): return 0.2 + 0.05 * (y - 0.025) / 0.935 + 0.004 * 4 * ((y - 0.025) / 0.935) * (1 - (y - 0.025) / 0.935)
g = G()
g.add("pedestal", "body", "body", 0, 1, path("M -0.2 0 L 0.2 0 L 0.2 0.012 C 0.2 0.02 0.196 0.025 0.19 0.025 L -0.19 0.025 C -0.196 0.025 -0.2 0.02 -0.2 0.012 Z"), *BH, tone=0.3)
g.add("pedestal-foot-l", "body", "detail", 2, 1.1, rect(-0.2, 0, 0.05, 0.008, 0.002), *BH, tone=0.15)
g.add("pedestal-foot-r", "body", "detail", 2, 1.1, rect(0.15, 0, 0.05, 0.008, 0.002), *BH, tone=0.15)
g.add("shell", "body", "body", 0, 3, path("M -0.2 0.025 L 0.2 0.025 C 0.215 0.3 0.235 0.7 0.248 0.955 L -0.248 0.955 C -0.235 0.7 -0.215 0.3 -0.2 0.025 Z"), *BH, tone=0.42)
for k, y in enumerate((0.15, 0.45, 0.72)):
    g.add(f"band-{k}", "body", "detail", 1, 3.5, poly([(-hw(y), y), (hw(y), y)], False), *BR, filled=False, tone=0.3)
    g.add(f"band-{k}-bead", "body", "detail", 2, 3.5, poly([(-hw(y + 0.018), y + 0.018), (hw(y + 0.018), y + 0.018)], False), *BR, filled=False, tone=0.3)
g.add("shell-texture", "body", "texture", 2, 3.6, path("M -0.15 0.30 C -0.05 0.31 0.05 0.29 0.15 0.30 M -0.14 0.58 C -0.05 0.59 0.06 0.57 0.14 0.58 M -0.12 0.85 C -0.04 0.86 0.05 0.84 0.12 0.85"), *BR, filled=False, tone=0.3)
g.add("rim", "body", "body", 0, 4, rect(-0.255, 0.94, 0.51, 0.04, 0.008), *BH, tone=0.5)
g.add("rim-inner", "body", "detail", 2, 4.1, poly([(-0.235, 0.955), (0.235, 0.955)], False), *BR, filled=False, tone=0.3)
g.add("liner-collar", "body", "detail", 1, 4.2, path("M -0.245 0.945 C -0.20 0.915 -0.10 0.930 0.0 0.918 C 0.10 0.930 0.20 0.915 0.245 0.945"), *LF, filled=False, tone=0.5)
g.add("liner-fold", "body", "texture", 3, 4.2, path("M -0.12 0.935 L -0.11 0.915 M 0.07 0.934 L 0.08 0.917"), *LF, filled=False, tone=0.3)
g.add("liner-ring", "body", "detail", 2, 3.8, rect(-hw(0.90) + 0.002, 0.895, 2 * hw(0.90) - 0.004, 0.012), *BR, filled=False, tone=0.3)
g.add("hinge-bracket", "body", "body", 1, 4.5, rect(-0.268, 0.925, 0.026, 0.065, 0.004), *BH, tone=0.3)
g.add("lid", "lid", "body", 0, 5, path("M 0.0 0.0 L 0.52 0.0 L 0.52 0.006 C 0.52 0.03 0.45 0.04 0.40 0.038 L 0.12 0.038 C 0.05 0.04 0.0 0.03 0.0 0.006 Z"), *LH, tone=0.55)
g.add("lid-skirt", "lid", "body", 1, 4.9, rect(0.0, -0.028, 0.52, 0.03, 0.006), *LH, tone=0.45)
g.add("lid-skirt-line", "lid", "detail", 2, 5.05, poly([(0.01, -0.012), (0.51, -0.012)], False), *LH, filled=False, tone=0.3)
g.add("lid-handle", "lid", "detail", 1, 5.1, path("M 0.38 0.038 C 0.39 0.058 0.47 0.058 0.48 0.038 Z"), *LH, tone=0.35)
g.add("lid-handle-recess", "lid", "detail", 3, 5.2, poly([(0.405, 0.045), (0.455, 0.045)], False), *LH, filled=False, tone=0.2)
g.add("hinge-barrel", "lid", "detail", 2, 5.3, ell(0.0, 0.004, 0.014, 0.014), *LH, tone=0.25)
g.add("hinge-pin", "lid", "detail", 3, 5.4, ell(0.0, 0.004, 0.005, 0.005), *BH, tone=0.1)
a["geometry"]["elevation"] = g.items
B = {b["id"]: b for b in a["rig"]["bones"]}
B["lid"].update(x=-0.255, y=0.98)
a["summary"] = a["summary"]
save("objects", doc)
