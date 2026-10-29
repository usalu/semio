import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "street-lamp")
PG, PC, AA, LH, LL = ("pole-galv", "galv-steel"), ("plinth-concrete", "concrete"), ("arm-alu", "aluminium-anodised"), ("lamp-housing", "aluminium-anodised"), ("lamp-lens", "polycarbonate")
old = {s["id"]: s for s in a["geometry"]["elevation"]}
LH = (old["head-housing"]["region"], old["head-housing"]["material"]); LL = (old["lens"]["region"], old["lens"]["material"])
AA = (old["arm"]["region"], old["arm"]["material"]); PG = (old["pole"]["region"], old["pole"]["material"]); PC = (old["plinth"]["region"], old["plinth"]["material"])
B = {b["id"]: b for b in a["rig"]["bones"]}
B["head"].update(x=0.72, lengthM=0.33)
g = G()
g.add("plinth", "foundation", "body", 0, 1, rect(-0.15, 0, 0.3, 0.1), *PC, tone=0.65)
g.add("plinth-joint", "foundation", "detail", 2, 1.1, poly([(-0.15, 0.05), (0.15, 0.05)], False), *PC, filled=False, tone=0.4)
g.add("base-plate", "foundation", "body", 0, 2, rect(-0.15, 0.1, 0.3, 0.02, 0.002), *PG, tone=0.35)
for k, x in enumerate((-0.12, 0.12)):
    g.add(f"base-nut-{k}", "foundation", "detail", 2, 2.1, rect(x - 0.012, 0.12, 0.024, 0.014), *PG, tone=0.2)
g.add("base-cover", "foundation", "body", 1, 2.5, path("M -0.12 0.13 C -0.11 0.20 -0.085 0.24 -0.082 0.30 L 0.082 0.30 C 0.085 0.24 0.11 0.20 0.12 0.13 Z"), *PG, tone=0.33)
g.add("pole", "pole", "body", 0, 3, path("M 0 -0.085 C 2 -0.08 4 -0.06 5.88 -0.04 L 5.88 0.04 C 4 0.06 2 0.08 0 0.085 Z"), *PG, tone=0.4)
g.add("pole-collar", "pole", "detail", 1, 3.5, rect(5.6, -0.055, 0.08, 0.11, 0.01), *PG, tone=0.3)
g.add("pole-cap", "pole", "body", 1, 3.2, path("M 5.86 -0.042 C 5.9 -0.03 5.9 0.03 5.86 0.042 Z"), *PG, tone=0.3)
g.add("pole-highlight", "pole", "texture", 2, 3.6, poly([(0.4, 0.06), (5.6, 0.035)], False), *PG, filled=False, tone=0.3)
g.add("pole-hatch", "pole", "detail", 2, 3.7, rect(0.55, -0.045, 0.5, 0.09, 0.008), *PG, filled=False, tone=0.3)
g.add("pole-hatch-lock", "pole", "detail", 3, 3.8, ell(0.62, 0, 0.007, 0.007), *PG, tone=0.15)
for k, x in enumerate((2.1, 3.9)):
    g.add(f"pole-seam-{k}", "pole", "texture", 2, 3.6, poly([(x, -0.07 + 0.005 * x / 2), (x, 0.07 - 0.005 * x / 2)], False), *PG, filled=False, tone=0.3)
g.add("arm", "arm", "body", 0, 2, path("M 0 -0.034 C 0.35 -0.03 0.8 -0.02 0.95 0.0 C 0.8 0.024 0.35 0.034 0 0.036 Z"), *AA, tone=0.5)
g.add("arm-brace", "arm", "body", 1, 1.9, cap(0.02, -0.46, 0.62, -0.012, 0.012), *AA, tone=0.45)
g.add("arm-brace-joint", "arm", "detail", 2, 2.1, ell(0.62, -0.012, 0.02, 0.02), *AA, tone=0.3)
g.add("arm-clamp", "arm", "detail", 2, 2.2, rect(-0.05, -0.05, 0.1, 0.1, 0.008), *AA, tone=0.3)
g.add("arm-seam", "arm", "texture", 2, 2.3, poly([(0.2, 0.012), (0.8, 0.008)], False), *AA, filled=False, tone=0.3)
g.add("head-housing", "head", "body", 0, 4, path("M 0.0 0.03 C 0.01 0.045 0.03 0.05 0.05 0.055 C 0.12 0.082 0.25 0.078 0.30 0.04 C 0.325 0.025 0.335 0.0 0.33 -0.02 L 0.31 -0.05 C 0.20 -0.062 0.10 -0.062 0.02 -0.05 C 0.005 -0.04 -0.005 -0.01 0.0 0.03 Z"), *LH, tone=0.5)
g.add("head-spigot", "head", "detail", 1, 4.05, rect(-0.04, -0.02, 0.07, 0.05, 0.006), *LH, tone=0.35)
g.add("head-driver", "head", "detail", 1, 4.1, path("M 0.03 0.055 C 0.04 0.095 0.11 0.098 0.13 0.07"), *LH, filled=False, tone=0.3)
for k in range(6):
    x = 0.15 + k * 0.028
    g.add(f"head-fin-{k}", "head", "detail", 2, 4.1, poly([(x, 0.066 - 0.0006 * k), (x, 0.082 - 0.002 * k)], False), *LH, filled=False, tone=0.3)
g.add("head-rim", "head", "detail", 2, 4.2, path("M 0.02 -0.046 C 0.10 -0.056 0.22 -0.056 0.31 -0.046"), *LH, filled=False, tone=0.3)
g.add("head-latch", "head", "detail", 3, 4.2, ell(0.30, -0.03, 0.006, 0.006), *LH, tone=0.2)
g.add("lens", "head", "glazing", 0, 5, path("M 0.03 -0.050 C 0.10 -0.088 0.24 -0.088 0.30 -0.050 L 0.30 -0.046 C 0.24 -0.076 0.10 -0.076 0.03 -0.046 Z"), *LL, tone=0.85)
g.add("lens-reflector", "head", "detail", 3, 5.1, path("M 0.06 -0.044 C 0.12 -0.020 0.22 -0.020 0.28 -0.044"), *LL, filled=False, tone=0.4)
a["geometry"]["elevation"] = g.items
save("objects", doc)
