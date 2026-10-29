import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "height-adjustable-desk")
SD, TB, EQ = ("steel-desk", "steel-desk"), ("top-board", "particleboard-laminate"), ("equipment", "equipment-housing")
if not any(m["id"] == "equipment-housing" for m in a["materials"]):
    a["materials"].append({"id": "equipment-housing", "title": T("Plastic and aluminium housing", "Kunststoff- und Aluminiumgehäuse"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1100, "source": "authored-estimate"})
if not any(r["id"] == "equipment" for r in a["thermal"]["regions"]):
    a["thermal"]["regions"].append({"id": "equipment", "title": T("Monitor and laptop housings", "Monitor- und Laptopgehäuse"), "kind": "surface", "material": "equipment-housing", "model": "passive", "areaM2": 0.55, "characteristicLengthM": 0.45, "source": "authored-estimate"})
# rig: two identical telescope columns, each foot -> stage (prismatic up) -> column (prismatic up)
bones = [b for b in a["rig"]["bones"] if b["id"] in ("foot-left", "foot-right")]
colmass = lambda: {"kg": 3.0, "comX": 0.17, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-1993-1-1"}
topmass = next(b for b in a["rig"]["bones"] if b["id"] == "top")["mass"]
for s, ft in (("left", "foot-left"), ("right", "foot-right")):
    ts = "Left" if s == "left" else "Right"; tsd = "Linke" if s == "left" else "Rechte"
    bones.append({"id": f"stage-{s}", "parent": ft, "title": T(f"{ts} lower telescope stage", f"{tsd} untere Teleskopstufe"), "x": 0, "y": 0.05, "angleDeg": 90, "lengthM": 0.5, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.215}})
    bones.append({"id": f"column-{s}", "parent": f"stage-{s}", "title": T(f"{ts} telescoping column", f"{tsd} Teleskopsäule"), "x": 0.20, "y": 0, "angleDeg": 0, "lengthM": 0.445, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.215}, "mass": colmass()})
bones.append({"id": "top", "parent": "column-left", "title": T("Desk top", "Tischplatte"), "x": 0.445, "y": 0, "angleDeg": -90, "lengthM": 1.6, "joint": {"type": "fixed"}, "mass": topmass})
a["rig"]["bones"] = bones
g = G()
for s in ("left", "right"):
    ft = f"foot-{s}"; st = f"stage-{s}"; co = f"column-{s}"
    g.add(f"{ft}-plate", ft, "body", 0, 1, path("M -0.25 0 L 0.25 0 L 0.25 0.008 C 0.25 0.016 0.2 0.016 0.12 0.016 L -0.12 0.016 C -0.2 0.016 -0.25 0.016 -0.25 0.008 Z"), *SD, tone=0.3)
    for k, x in enumerate((-0.22, 0.22)):
        g.add(f"{ft}-glide-{k}", ft, "detail", 2, 1.1, rect(x - 0.02, 0, 0.04, 0.005, 0.002), *SD, tone=0.1)
    g.add(f"{ft}-sleeve", ft, "body", 0, 3.5, rect(-0.032, 0.012, 0.064, 0.388, 0.004), *SD, tone=0.28)
    g.add(f"{ft}-sleeve-collar", ft, "detail", 1, 3.6, rect(-0.036, 0.385, 0.072, 0.02, 0.003), *SD, tone=0.18)
    g.add(f"{ft}-sleeve-seam", ft, "detail", 2, 3.6, poly([(0.0, 0.03), (0.0, 0.375)], False), *SD, filled=False, tone=0.3)
    g.add(f"{ft}-boss", ft, "detail", 2, 3.7, rect(-0.05, 0.012, 0.1, 0.04, 0.006), *SD, tone=0.22)
    g.add(f"{st}-tube", st, "body", 0, 3.0, rect(0, -0.027, 0.5, 0.054), *SD, tone=0.42)
    g.add(f"{st}-groove", st, "detail", 2, 3.1, poly([(0.41, 0.0), (0.5, 0.0)], False), *SD, filled=False, tone=0.3)
    g.add(f"{st}-cap", st, "detail", 1, 3.2, rect(0.488, -0.029, 0.012, 0.058, 0.002), *SD, tone=0.2)
    g.add(f"{co}-tube", co, "body", 0, 2.5, rect(0, -0.022, 0.445, 0.044), *SD, tone=0.55)
    g.add(f"{co}-groove", co, "detail", 1, 2.6, poly([(0.02, 0.0), (0.43, 0.0)], False), *SD, filled=False, tone=0.3)
    g.add(f"{co}-bracket", co, "body", 1, 2.7, rect(0.41, -0.045, 0.035, 0.09, 0.004), *SD, tone=0.22)
    g.add(f"{co}-bolt", co, "detail", 3, 2.8, ell(0.4275, 0.03, 0.004, 0.004), *SD, tone=0.1)
    g.add(f"{co}-bolt-b", co, "detail", 3, 2.8, ell(0.4275, -0.03, 0.004, 0.004), *SD, tone=0.1)
# desk top with edging, cross rail, cable tray, control panel
g.add("top-board", "top", "body", 0, 4, rect(-0.25, 0, 1.6, 0.025, 0.004), *TB, tone=0.58)
g.add("top-edge", "top", "detail", 1, 4.1, poly([(-0.246, 0.0125), (1.346, 0.0125)], False), *TB, filled=False, tone=0.3)
g.add("top-grain", "top", "texture", 2, 4.2, path("M -0.1 0.018 C 0.3 0.020 0.6 0.016 1.2 0.019"), *TB, filled=False, tone=0.3)
g.add("top-rail", "top", "body", 1, 3.5, rect(0.0, -0.075, 1.1, 0.05, 0.003), *SD, tone=0.35)
g.add("top-rail-line", "top", "detail", 2, 3.6, poly([(0.03, -0.05), (1.07, -0.05)], False), *SD, filled=False, tone=0.3)
g.add("top-cable-tray", "top", "detail", 2, 3.4, path("M 0.30 -0.075 L 0.30 -0.115 L 0.80 -0.115 L 0.80 -0.075"), *SD, filled=False, tone=0.3)
g.add("top-keypad", "top", "detail", 3, 4.3, rect(1.17, -0.012, 0.09, 0.012, 0.003), *TB, tone=0.15)
# equipment carried by the loads: monitor on stand (x 0.35 on top), laptop at x 0.82
g.add("monitor-base", "top", "body", 1, 5, rect(0.26, 0.025, 0.18, 0.012, 0.005), *EQ, tone=0.25)
g.add("monitor-neck", "top", "body", 1, 5, path("M 0.335 0.037 L 0.365 0.037 L 0.362 0.15 L 0.338 0.15 Z"), *EQ, tone=0.3)
g.add("monitor-panel", "top", "body", 1, 5.1, rect(0.05, 0.12, 0.60, 0.34, 0.012), *EQ, tone=0.2)
g.add("monitor-screen", "top", "glazing", 1, 5.2, rect(0.065, 0.135, 0.57, 0.31, 0.004), *EQ, filled=False, tone=0.1)
g.add("monitor-bezel-logo", "top", "detail", 3, 5.3, poly([(0.32, 0.127), (0.38, 0.127)], False), *EQ, filled=False, tone=0.4)
g.add("laptop", "top", "body", 0, 5, rect(0.7, 0.025, 0.25, 0.02, 0.003), *EQ, tone=0.4)
g.add("laptop-seam", "top", "detail", 2, 5.1, poly([(0.705, 0.034), (0.945, 0.034)], False), *EQ, filled=False, tone=0.3)
a["geometry"]["elevation"] = g.items
def off(d): return {"stage-left": {"dy": d / 2}, "column-left": {"dx": d / 2}, "stage-right": {"dy": d / 2}, "column-right": {"dx": d / 2}}
for p in a["poses"]:
    d = {"seated-height": 0, "mid-height": 0.215, "standing-height": 0.43}[p["id"]]
    p["bones"] = off(d)
    for l in p["loads"]:
        if l["id"] == "monitor": l["x"] = 0.35
for p in a["poses"]:
    for l in p["loads"]:
        if l["id"] == "monitor": l["title"] = T("Monitor (8 kg) on its stand", "Monitor (8 kg) auf Standfuß")
        l["source"] = "authored-estimate"
a["presentation"]["defaultPose"] = "seated-height"
a["structure"]["assumptions"] = a["structure"]["assumptions"] if False else a["structure"]["assumptions"]
a["summary"] = T("A two-column sit-stand desk: each T-foot carries a two-stage telescope (identical prismatic chains left and right) under a laminated top with a cross rail; a monitor and a laptop stand on it. Front elevation, length along x.", "Ein Sitz-Steh-Schreibtisch mit zwei Säulen: Jeder T-Fuß trägt eine zweistufige Teleskopsäule (links und rechts identische Schubgelenkketten) unter einer laminierten Platte mit Querträger; Monitor und Laptop stehen darauf. Vorderansicht, Länge entlang x.")
save("objects", doc)
