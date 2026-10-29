import sys, math; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "playground-swing")
FG, CS, SR = ("frame-galv", "steel-galvanised"), ("chain-steel", "steel-galvanised"), ("seat-rubber", "rubber")
B = {b["id"]: b for b in a["rig"]["bones"]}
B["chain-left"].update(x=0, y=0); B["chain-right"].update(x=0, y=0)
B["seat"]["mass"].update(comX=0, comY=-0.065); B["seat"]["lengthM"] = 0.42
for an in a["anchors"]:
    if an["id"] == "seat-centre": an.update(x=0, y=-0.04)
for p in a["poses"]:
    for l in p["loads"]: l.update(x=0, y=-0.04)
g = G()
u = (math.cos(math.radians(60.5)), math.sin(math.radians(60.5)))
def leg(side, bone, sg):
    k = 0.0170
    if sg > 0: pts = [(-k, 0.03), (k, -0.03), (2.64, -0.03), (2.64, 0.03)]
    else: pts = [(k, 0.03), (-k, -0.03), (2.64, -0.03), (2.64, 0.03)]
    g.add(f"leg-{side}-tube", bone, "body", 0, 2, poly(pts), *FG, tone=0.4)
    g.add(f"leg-{side}-weld", bone, "detail", 2, 2.1, poly([(0.28, -0.03), (0.28, 0.03)], False), *FG, filled=False, tone=0.3)
    g.add(f"leg-{side}-clamp", bone, "detail", 1, 2.2, rect(2.30, -0.04, 0.10, 0.08, 0.006), *FG, tone=0.3)
leg("left", "leg-left", 1); leg("right", "leg-right", -1)
for side, x in (("left", 0.0), ("right", 2.6)):
    g.add(f"foot-{side}-plate", "ground", "body", 1, 2.3, rect(x - 0.09, 0, 0.18, 0.012, 0.002), *FG, tone=0.25)
    for k, dx in enumerate((-0.065, 0.065)):
        g.add(f"foot-{side}-bolt-{k}", "ground", "detail", 3, 2.4, ell(x + dx, 0.015, 0.008, 0.005), *FG, tone=0.15)
g.add("brace-tube", "brace", "body", 1, 2.5, cap(0, 0, 1.92, 0, 0.02, 0.02), *FG, tone=0.45)
g.add("brace-clamp-l", "brace", "detail", 2, 2.6, rect(-0.02, -0.032, 0.05, 0.064, 0.005), *FG, tone=0.3)
g.add("brace-clamp-r", "brace", "detail", 2, 2.6, rect(1.89, -0.032, 0.05, 0.064, 0.005), *FG, tone=0.3)
g.add("beam-end", "beam", "body", 0, 3, ell(0, 0, 0.05, 0.05), *FG, tone=0.38)
g.add("beam-cap", "beam", "detail", 2, 3.1, ell(0, 0, 0.034, 0.034), *FG, filled=False, tone=0.3)
g.add("beam-hanger", "beam", "detail", 1, 3.2, path("M -0.025 -0.045 C -0.03 -0.09 0.03 -0.09 0.025 -0.045"), *CS, filled=False, tone=0.2)
for ch, z in (("chain-left", 2.5), ("chain-right", 2.4)):
    g.add(f"{ch}-link", ch, "body", 0, z, cap(0.07, 0, 1.9, 0, 0.0055), *CS, tone=0.3)
rings = "".join(f"M {0.075 + 0.1 * i - 0.017:.4f} 0 A 0.017 0.0075 0 1 0 {0.075 + 0.1 * i + 0.017:.4f} 0 A 0.017 0.0075 0 1 0 {0.075 + 0.1 * i - 0.017:.4f} 0 Z " for i in range(19))
bars = "".join(f"M {0.125 + 0.1 * i - 0.019:.4f} -0.0035 L {0.125 + 0.1 * i + 0.019:.4f} -0.0035 L {0.125 + 0.1 * i + 0.019:.4f} 0.0035 L {0.125 + 0.1 * i - 0.019:.4f} 0.0035 Z " for i in range(18))
g.add("chain-left-rings", "chain-left", "detail", 3, 2.6, path(rings.strip()), *CS, filled=False, tone=0.2)
g.add("chain-left-bars", "chain-left", "detail", 3, 2.6, path(bars.strip()), *CS, tone=0.25)
g.add("chain-left-texture", "chain-left", "texture", 2, 2.6, poly([(0.3, 0), (1.8, 0)], False), *CS, filled=False, tone=0.3)
g.add("seat-shackle", "seat", "detail", 1, 4.5, ell(0, -0.025, 0.011, 0.027), *CS, tone=0.2)
g.add("seat-plank", "seat", "body", 0, 4, path("M -0.21 -0.06 C -0.21 -0.045 -0.20 -0.038 -0.18 -0.04 L 0.18 -0.04 C 0.20 -0.038 0.21 -0.045 0.21 -0.06 C 0.21 -0.092 0.19 -0.098 0.17 -0.098 L -0.17 -0.098 C -0.19 -0.098 -0.21 -0.092 -0.21 -0.06 Z"), *SR, tone=0.25)
g.add("seat-edge", "seat", "detail", 1, 4.1, poly([(-0.19, -0.068), (0.19, -0.068)], False), *SR, filled=False, tone=0.3)
g.add("seat-tread", "seat", "texture", 2, 4.2, path("M -0.15 -0.05 L -0.12 -0.05 M -0.07 -0.05 L -0.04 -0.05 M 0.02 -0.05 L 0.05 -0.05 M 0.10 -0.05 L 0.13 -0.05"), *SR, filled=False, tone=0.3)
g.add("seat-bolt-l", "seat", "detail", 3, 4.3, ell(-0.17, -0.07, 0.005, 0.005), *CS, tone=0.1)
g.add("seat-bolt-r", "seat", "detail", 3, 4.3, ell(0.17, -0.07, 0.005, 0.005), *CS, tone=0.1)
a["geometry"]["elevation"] = g.items
note = a["provenance"]["notes"]
for k in ("en", "de"):
    note[k] = note[k]
import re
note["en"] = note["en"].replace(" The seat stays horizontal; the double-pendulum wording of the brief is simplified to this one-DOF pendulum with a rigid occupant.", " The seat stays horizontal; the swing is modelled as a one-degree-of-freedom pendulum with a rigid occupant.")
note["de"] = re.sub(r"Doppelpendel[^.]*\.", "Die Schaukel ist als Pendel mit einem Freiheitsgrad und starrem Insassen modelliert.", note["de"])
a["structure"]["assumptions"][1] = T("The occupant is rigid and seated; the two chains coincide in this side elevation, the second is kinematically coupled to the first; no member bending is checked.", "Der Insasse ist starr und sitzend; die beiden Ketten fallen in dieser Seitenansicht zusammen, die zweite ist kinematisch an die erste gekoppelt; keine Biegenachweise der Glieder.")
save("objects", doc)
