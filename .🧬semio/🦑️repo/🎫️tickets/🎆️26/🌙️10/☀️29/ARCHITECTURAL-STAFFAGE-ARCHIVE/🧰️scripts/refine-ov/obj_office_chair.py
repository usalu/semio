import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "office-swivel-chair")
PL, FB, SL, RB = ("chair-plastic", "pa-plastic"), ("chair-fabric", "fabric-upholstery"), ("chair-steel", "steel-painted"), ("castor-rubber", "rubber")
bones = a["rig"]["bones"]
if not any(b["id"] == "column-sleeve" for b in bones):
    i = next(k for k, b in enumerate(bones) if b["id"] == "column")
    bones.insert(i, {"id": "column-sleeve", "parent": "base", "title": T("Gas-lift cover", "Gaslift-Hülse"), "x": 0, "y": 0, "angleDeg": 90, "lengthM": 0.3, "joint": {"type": "fixed"}})
g = G()
# hub boss and five-star arms (elevation projections of the 72 degree plan arms)
g.add("hub", "base", "body", 0, 3, path("M -0.055 0.004 C -0.055 -0.012 -0.03 -0.02 0 -0.02 C 0.03 -0.02 0.055 -0.012 0.055 0.004 L 0.04 0.042 L -0.04 0.042 Z"), *PL, tone=0.3)
g.add("hub-ring", "base", "detail", 2, 3.1, poly([(-0.047, 0.014), (0.047, 0.014)], False), *PL, filled=False, tone=0.3)
for bone, L, tag in (("arm-front", 0.30, "front"), ("arm-side-a", 0.093, "side-a"), ("arm-side-b", 0.093, "side-b"), ("arm-back-a", 0.243, "back-a"), ("arm-back-b", 0.243, "back-b")):
    near = tag in ("front", "side-a", "back-a"); f = -1 if tag.startswith("back") else 1
    def yy(v): return v * f
    g.add(bone, bone, "body" if near else "detail", 0 if near else 1, 2.5 if near else 2.2, path(f"M 0.03 {yy(0.034):.4f} L {L-0.012:.4f} {yy(0.022):.4f} L {L:.4f} {yy(0.022):.4f} L {L:.4f} {yy(-0.003):.4f} L {L-0.02:.4f} {yy(-0.006):.4f} L 0.03 {yy(0.002):.4f} Z"), *PL, tone=0.3 if near else 0.38)
    g.add(f"{bone}-rib", bone, "detail", 2, 2.6, poly([(0.05, yy(0.018)), (L - 0.03, yy(0.012))], False), *PL, filled=False, tone=0.3)
    g.add(f"{bone}-yoke", bone, "body" if near else "detail", 1, 2.7 if near else 2.3, path(f"M {L-0.022:.4f} 0.0 C {L-0.022:.4f} {yy(0.034):.4f} {L+0.022:.4f} {yy(0.034):.4f} {L+0.022:.4f} 0.0 Z"), *PL, tone=0.28)
    g.add(f"{bone}-stem", bone, "detail", 2, 2.8, rect(L - 0.006, 0.02 if f > 0 else -0.032, 0.012, 0.012), *SL, tone=0.2)
    c = bone.replace("arm", "castor")
    g.add(f"{c}-wheel", c, "body", 0, 1, ell(0, 0, 0.025, 0.025), *RB, tone=0.2)
    g.add(f"{c}-hub", c, "detail", 1, 1.1, ell(0, 0, 0.011, 0.011), *RB, tone=0.55)
    g.add(f"{c}-tread", c, "texture", 2, 1.2, ell(0, 0, 0.019, 0.019), *RB, filled=False, tone=0.3)
    g.add(f"{c}-axle", c, "detail", 3, 1.3, ell(0, 0, 0.004, 0.004), *SL, tone=0.1)
# gas lift: fixed sleeve with bellows rings, chrome piston, release lever
g.add("column-sleeve", "column-sleeve", "body", 0, 3.4, path("M 0.02 -0.034 L 0.30 -0.026 L 0.30 0.026 L 0.02 0.034 Z"), *SL, tone=0.25)
for k, x in enumerate((0.10, 0.16, 0.22, 0.28)):
    g.add(f"column-ring-{k}", "column-sleeve", "detail", 2, 3.5, poly([(x, -0.028 + 0.0003 * x), (x, 0.028 - 0.0003 * x)], False), *SL, filled=False, tone=0.3)
g.add("column-collar", "column-sleeve", "detail", 1, 3.6, rect(0.285, -0.032, 0.02, 0.064, 0.004), *SL, tone=0.15)
g.add("column", "column", "body", 0, 3.0, cap(0.20, 0, 0.43, 0, 0.0125), *SL, tone=0.7)
g.add("column-groove", "column", "detail", 2, 3.1, poly([(0.31, 0.005), (0.40, 0.005)], False), *SL, filled=False, tone=0.4)
g.add("column-cone", "column", "body", 1, 3.2, path("M 0.37 -0.014 L 0.435 -0.032 L 0.435 0.032 L 0.37 0.014 Z"), *SL, tone=0.22)
# seat pan with waterfall edge, mechanism, lever, armrest
g.add("seat-cushion", "seat", "body", 0, 4, path("M -0.20 -0.016 C -0.23 -0.014 -0.24 0.004 -0.23 0.02 C -0.22 0.036 -0.21 0.04 -0.19 0.04 L 0.17 0.04 C 0.21 0.04 0.232 0.03 0.232 0.01 C 0.232 -0.008 0.22 -0.022 0.19 -0.022 L -0.18 -0.022 Z"), *FB, tone=0.62)
g.add("seat-shell", "seat", "detail", 1, 4.1, path("M -0.20 -0.014 C -0.1 -0.03 0.1 -0.03 0.19 -0.02"), *PL, filled=False, tone=0.3)
g.add("seat-piping", "seat", "detail", 2, 4.1, path("M -0.17 0.03 L 0.15 0.03"), *FB, filled=False, tone=0.3)
g.add("seat-fabric-fold", "seat", "texture", 2, 4.2, path("M -0.10 0.012 C -0.05 0.016 0.02 0.008 0.09 0.014"), *FB, filled=False, tone=0.3)
g.add("seat-mechanism", "seat", "body", 1, 3.9, rect(-0.10, -0.062, 0.2, 0.042, 0.006), *SL, tone=0.22)
g.add("seat-lever", "seat", "detail", 2, 3.95, path("M 0.10 -0.045 L 0.17 -0.05 L 0.17 -0.044 L 0.10 -0.04"), *SL, tone=0.2)
g.add("seat-lever-knob", "seat", "detail", 3, 3.96, ell(0.175, -0.047, 0.007, 0.007), *PL, tone=0.2)
g.add("armrest-post", "seat", "body", 2, 3.0, path("M -0.05 0.04 L -0.02 0.04 L -0.025 0.20 L -0.045 0.20 Z"), *PL, tone=0.35)
g.add("armrest-pad", "seat", "body", 2, 3.1, cap(-0.19, 0.215, 0.115, 0.215, 0.014, 0.018), *PL, tone=0.4)
g.add("armrest-line", "seat", "detail", 3, 3.2, poly([(-0.18, 0.222), (0.10, 0.224)], False), *PL, filled=False, tone=0.3)
# backrest: lumbar-contoured shell, back strut, upholstery panel
g.add("backrest-strut", "backrest", "body", 1, 3.8, cap(0.0, 0.012, 0.24, 0.026, 0.011, 0.009), *SL, tone=0.22)
g.add("backrest-shell", "backrest", "body", 0, 4, path("M 0.07 -0.012 C 0.20 -0.046 0.36 -0.030 0.60 -0.014 C 0.635 -0.010 0.65 0.010 0.62 0.028 C 0.40 0.038 0.22 0.044 0.08 0.040 C 0.06 0.036 0.06 0.0 0.07 -0.012 Z"), *PL, tone=0.5)
g.add("backrest-pad", "backrest", "detail", 1, 4.1, path("M 0.12 -0.020 C 0.24 -0.040 0.36 -0.028 0.56 -0.012"), *FB, filled=False, tone=0.3)
g.add("backrest-rim", "backrest", "detail", 2, 4.1, path("M 0.10 0.028 C 0.30 0.032 0.45 0.030 0.60 0.020"), *PL, filled=False, tone=0.3)
g.add("backrest-texture", "backrest", "texture", 2, 4.2, path("M 0.2 0.0 C 0.3 0.004 0.4 -0.004 0.52 0.002"), *FB, filled=False, tone=0.3)
a["geometry"]["elevation"] = g.items
B = {b["id"]: b for b in bones}
B["column"].update(lengthM=0.42)
a["summary"] = a["summary"]
save("objects", doc)
