"""🚜 The round-3 wheel loader stated by hand, one authored model per scale: a 20 t articulated loader of the Cat 962M class (wheelbase 3.35 m, 23.5R25 tyres, Z-bar linkage, 3.5 m3 bucket). `build(scale)` gives body, thermal and structure solids and the three 2D shape lists that are their true orthographic outlines."""
import math
import r3_wheel_loader_kin as K
from r3_wheel_loader_lib import *

AX, TR, TW, TZ = K.AX, K.TYRE_R, 0.60, 1.07
WHEEL.update({"wheel-r": (-AX, TR), "wheel-f": (AX, TR)})
REAR = -AX - 2.147
CARRY = K.solve(*K.POSES["carry"])
ARM_DEG, BKT_DEG = K.POSES["carry"]
LIFT_BARREL, TILT_BARREL = 1.30, 1.62
LIFT_REST, TILT_REST = CARRY["lift"], CARRY["tilt"]
H_LEV = K.rot((K.R1, 0), K.BEND)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

PS, CI, GL, RU, EX, CH, WS = "paint-steel", "cast-iron", "glass", "rubber", "exhaust-steel", "chrome", "wear-steel"
RF = Ctx("rear-frame", PS, "body", 0.45, "front-frame", "rear-frame")
FF = Ctx("front-frame", PS, "body", 0.45, "front-frame", "front-frame")
LA = Ctx("lift-arm", PS, "body", 0.35, "lift-arm", "lift-arm")
BK = Ctx("bucket", WS, "body", 0.25, "bucket", "bucket")
LV = Ctx("tilt-lever", PS, "body", 0.35, "lift-arm", "tilt-lever")
LK = Ctx("tilt-link", PS, "body", 0.35, "lift-arm", "tilt-link")

RAIL = [(-3.30, 1.02), (-0.95, 1.02), (-0.80, 0.62), (-0.30, 0.62), (-0.30, 1.42), (-3.30, 1.42)]
HOOD = [(-3.72, 1.42, 2.36), (-3.62, 1.42, 2.44), (-2.60, 1.42, 2.52), (-1.42, 1.42, 2.60)]
CWT_PLAN = [(1.22, -3.30), (1.22, -3.62), (1.02, -3.82), (-1.02, -3.82), (-1.22, -3.62), (-1.22, -3.30)]
CWT_SKIRT = [(-3.30, 0.95), (-3.30, 1.14), (-3.82, 1.14), (-3.62, 0.95)]
FENDER_R = [(-2.70, 1.30), (-2.62, 1.70), (-2.45, 1.85), (-0.30, 1.85)]
CAB_PLAN = [(0.76, -1.40), (0.76, -0.25), (0.50, 0.24), (-0.50, 0.24), (-0.76, -0.25), (-0.76, -1.40)]
CAB_SIDE = [(-1.40, 1.85), (0.24, 1.85), (0.24, 3.30), (0.36, 3.30), (0.36, 3.40), (0.30, 3.45), (-1.42, 3.45), (-1.48, 3.40), (-1.48, 3.30), (-1.40, 3.30)]
TOWER = [(0.10, 0.62), (2.10, 0.58), (2.30, 0.80), (2.30, 1.08), (1.70, 1.34), (1.30, 2.42), (1.06, 2.50), (0.86, 2.42), (0.50, 1.52), (0.10, 1.46)]
ARM = [(-0.08, -0.15), (0.45, -0.08), (1.00, 0.03), (1.50, 0.13), (2.05, 0.02), (2.62, -0.13), (2.80, -0.11), (2.87, 0.0), (2.80, 0.12), (2.30, 0.26), (1.65, 0.53), (1.35, 0.52), (0.70, 0.34), (0.10, 0.15), (-0.08, 0.15), (-0.16, 0.0)]
ARM_500 = [(-0.10, -0.15), (1.50, 0.13), (2.80, -0.12), (2.86, 0.0), (2.80, 0.12), (1.65, 0.53), (1.35, 0.52), (-0.10, 0.15)]
BK_OUT = [(0.58, 0.93), (0.33, 0.80), (0.20, 0.50), (0.15, 0.05), (0.186, -0.13), (0.288, -0.282), (0.44, -0.384), (0.62, -0.42), (1.47, -0.42)]
BK_OUT_COARSE = [(0.58, 0.93), (0.33, 0.80), (0.15, 0.05), (0.288, -0.282), (0.62, -0.42), (1.47, -0.42)]
BK_EDGE = [(1.10, -0.42), (1.47, -0.42), (1.47, -0.405), (1.36, -0.38), (1.10, -0.38)]
BK_HINGE = [(-0.13, -0.06), (-0.07, -0.13), (0.05, -0.16), (0.30, -0.30), (0.17, 0.0), (0.19, 0.30), (0.0, 0.14), (-0.10, 0.09)]
BK_LUG = [(-0.05, 0.36), (0.0, 0.29), (0.10, 0.27), (0.22, 0.30), (0.24, 0.62), (0.10, 0.52), (0.0, 0.50)]
BK_BEAM = [(0.31, 0.70), (0.20, 0.74), (0.25, 0.86), (0.36, 0.82)]
LEVER = [(0.80, -0.11), (0.91, 0.0), (0.80, 0.11), (0.10, 0.16), (-0.02, 0.17), (-0.52, -0.16), (-0.68, -0.19), (-0.74, -0.30), (-0.66, -0.40), (-0.56, -0.40), (-0.02, -0.16)]
LINK = [(-0.10, 0.0), (-0.06, -0.09), (0.06, -0.09), (0.20, -0.055), (0.70, -0.055), (0.84, -0.09), (0.96, -0.09), (1.0, 0.0), (0.96, 0.09), (0.84, 0.09), (0.70, 0.055), (0.20, 0.055), (0.06, 0.09), (-0.06, 0.09)]
TYRE = [(-0.25, 0.32), (-0.30, 0.44), (-0.30, 0.62), (-0.285, 0.71), (-0.25, 0.765), (-0.20, 0.7775), (0.20, 0.7775), (0.25, 0.765), (0.285, 0.71), (0.30, 0.62), (0.30, 0.44), (0.25, 0.32), (-0.25, 0.32)]
TYRE_PLAIN = [(-0.25, 0.0), (-0.25, 0.32), (-0.30, 0.44), (-0.30, 0.66), (-0.27, 0.775), (-0.20, TR), (0.20, TR), (0.27, 0.775), (0.30, 0.66), (0.30, 0.44), (0.25, 0.32), (0.25, 0.0)]
LUG = [(0.7700, 0.010), (0.8075, 0.010), (0.8075, 0.200), (0.7950, 0.255), (0.7400, 0.292), (0.6600, 0.315), (0.6600, 0.282), (0.7100, 0.272), (0.7550, 0.240), (0.7700, 0.200)]
AXLE = [(-0.77, 0.0), (-0.77, 0.14), (-0.50, 0.12), (-0.22, 0.12), (-0.14, 0.27), (0.14, 0.27), (0.22, 0.12), (0.50, 0.12), (0.77, 0.14), (0.77, 0.0)]


def hood_sections(n):
    rows = HOOD if n >= 4 else [HOOD[0], HOOD[1], HOOD[3]]
    return [((x, (lo + hi) / 2, 0), 0.70 if i == 0 else 0.72, (hi - lo) / 2) for i, (x, lo, hi) in enumerate(rows)]


def fender_f(t):
    return strip(arc(0, 0, 0.93, 190, 95, 7), -t)


def build(sc):
    f = FEATURE[sc]
    fine, mid, coarse, tiny = sc == 50, sc == 100, sc == 200, sc == 500
    plate = max(0.008, f)
    B, sh, extra_th, st = [], Sheet(), [], []

    def add(s, z=None, **kw):
        B.append(s)
        if z is not None:
            draw(sh, s, z, **kw)
        return s

    hood_c, cwt_c, cab_c, glass_c = RF(region="engine-bay"), RF(material=CI, tone=0.3, region="counterweight"), RF(tone=0.55, region="cab"), RF(material=GL, role="glazing", tone=0.85, region="cab-glazing")
    det = lambda c, tone=0.3: c(role="detail", tone=tone)

    if tiny:
        add(ext("rear-body", RF, [(-3.30, 1.02), (-0.95, 1.02), (-0.80, 0.62), (0.10, 0.62), (0.10, 1.46), (0.02, 1.46), (0.02, 1.85), (-2.45, 1.85), (-2.70, 1.46), (-3.30, 1.42)], -0.46, 0.46), (1.9, 1.9, 1.9), sid="rear-frame")
        add(ext("deck", RF, [(-2.62, 1.775), (-0.30, 1.775), (-0.30, 1.85), (-2.62, 1.85)], 0.46, 1.37, mir=True), (2.6, 1.95, 1.95), views="fp")
        add(ext("hood", hood_c, [(-3.72, 1.42), (-3.72, 2.36), (-3.62, 2.44), (-1.42, 2.60), (-1.42, 1.42)], -0.72, 0.72), (2, 2, 2))
        add(ext("counterweight", cwt_c, [(-3.30, 0.95), (-3.30, 1.52), (-3.82, 1.52), (-3.82, 1.14), (-3.62, 0.95)], -1.22, 1.22), (2.05, 2.05, 2.05))
        add(cyl("exhaust-stack", RF(material=EX, tone=0.2, region="exhaust-stack"), "v", (-2.20, 2.50, -0.40), 0.80, 0.065), (2.1, 2.1, 2.1))
        add(ext("cab", cab_c, CAB_PLAN, 1.85, 2.12, "zx"), (2.2, 2.2, 2.2))
        add(ext("cab-glass", glass_c, [(0.74, -1.38), (0.74, -0.25), (0.48, 0.22), (-0.48, 0.22), (-0.74, -0.25), (-0.74, -1.38)], 2.12, 3.26, "zx"), (2.4, 2.4, 2.21))
        add(ext("cab-roof", cab_c, [(0.84, -1.48), (0.84, -0.22), (0.56, 0.36), (-0.56, 0.36), (-0.84, -0.22), (-0.84, -1.48)], 3.26, 3.45, "zx"), (2.25, 2.25, 2.45))
        add(ext("front-body", FF, TOWER, -0.44, 0.44), (2, 2, 2), sid="front-frame")
    else:
        if coarse:
            add(ext("rear-frame", RF, RAIL, 0.34, 0.46, mir=True), (1.9, 1.9, 1.9), views="e")
            add(box("hitch", RF, (-0.36, 0.62, -0.34), (0.16, 1.46, 0.34)), (1.95, 1.9, 1.9), views="e")
            add(cyl("axle-r", RF(tone=0.3), "w", (-AX, TR, -0.77), 1.54, 0.13), (1.8, 1.8, 1.4), views="fp")
            add(ext("hood", hood_c, [(-3.72, 1.42), (-3.72, 2.36), (-3.62, 2.44), (-1.42, 2.60), (-1.42, 1.42)], -0.72, 0.72), (2, 2, 2))
            add(ext("counterweight", cwt_c, CWT_PLAN, 0.95, 1.52, "zx"), (2.05, 2.05, 2.05))
        else:
            add(ext("rear-frame", RF, RAIL, 0.34, 0.46, mir=True), (1.9, 1.9, 1.9), views="e")
            add(box("rear-crossmember", RF, (-3.30, 1.04, -0.34), (-3.24, 1.42, 0.34)))
            add(box("hitch-bulkhead", RF, (-0.36, 0.66, -0.34), (-0.30, 1.42, 0.34)))
            add(box("hitch-upper-r", RF, (-0.36, 1.40, -0.28), (0.13, 1.46, 0.28)), (1.95, 1.9, 1.9), views="e")
            add(box("hitch-lower-r", RF, (-0.36, 0.62, -0.28), (0.13, 0.68, 0.28)), (1.95, 1.9, 1.9), views="e")
            add(rev("axle-r", RF(tone=0.3), AXLE, "w", (-AX, TR, 0)), (1.8, 1.8, 1.4), views="fp")
            if fine:
                add(box("axle-cradle-r", RF, (-AX - 0.25, 0.93, -0.46), (-AX + 0.25, 1.02, 0.46)))
                add(loft("fuel-tank", RF, [((-3.22, 1.20, 0), 0.60, 0.20), ((-2.90, 1.20, 0), 0.68, 0.22), ((-2.58, 1.20, 0), 0.60, 0.20)], 6.0), (1.85, 1.85, 1.85), views="e")
            add(loft("hood", hood_c, hood_sections(4 if fine else 3), 6.0, True), (2, 2, 2))
            add(ext("counterweight", cwt_c, CWT_PLAN, 1.14, 1.52, "zx"), (2.05, 2.05, 2.05))
            add(ext("counterweight-skirt", cwt_c, CWT_SKIRT, -1.02, 1.02), (2.05, 2.05, 2.05), views="ef")
            add(box("grille", hood_c(role="detail", tone=0.2), (-3.74, 1.55, -0.58), (-3.715, 2.28, 0.58)), (2.3, 2.3, 2.3), views="ep")
            if fine:
                for i, y in enumerate((2.10, 1.95)):
                    add(mirror(cap("hood-louvres" + ("" if i == 0 else "-2"), hood_c(role="detail", tone=0.3), (-3.2, y, 0.722), (-2.0, y, 0.722), 0.006, False), 0.722, 0.722), (2.3, 2.3, 2.3), views="e")
                for side, z in (("l", 0.86), ("r", -0.86)):
                    add(box("tail-lamp-" + side, RF(material=GL, role="detail", tone=0.9), (-3.76, 1.58, z - 0.09), (-3.70, 1.72, z + 0.09)))
            add(rev("air-cleaner", hood_c(role="detail", tone=0.35), [(2.50, 0), (2.50, 0.05), (2.80, 0.05), (2.80, 0.12), (2.86, 0.14), (2.98, 0.14), (3.02, 0.10), (3.02, 0)], "v", (-1.80, 0, 0.40)), (2.1, 2.1, 2.1))
        add(rev("exhaust-stack", RF(material=EX, tone=0.2, region="exhaust-stack"), [(2.50, 0.057), (3.30, 0.057), (3.30, 0.065), (2.50, 0.065), (2.50, 0.057)] if not coarse else [(2.50, 0), (2.50, 0.065), (3.30, 0.065), (3.30, 0)], "v", (-2.20, 0, -0.40)), (2.1, 2.1, 2.1))
        add(ext("fender-r", RF, strip(FENDER_R, -plate), 0.46 if coarse else 0.74, 1.37, mir=True), (4.1, 1.95, 1.95))
        add(box("cab-riser", RF, (-1.42, 1.42, -0.46), (0.02, 1.85, 0.46)), (1.95, 1.95, 1.95), views="e")
        if not coarse:
            add(ext("deck-inner", RF, [(-1.42, 1.85 - plate), (-0.30, 1.85 - plate), (-0.30, 1.85), (-1.42, 1.85)], 0.46, 0.74, mir=True), (1.95, 1.95, 1.95), views="f")
        if coarse:
            add(ext("ladder", RF, [(-0.34, 1.85), (-0.46, 1.85), (-0.60, 0.50), (-0.48, 0.50)], 0.92, 1.36, mir=True), (4.2, 1.96, 1.96), sid="access-step")
        else:
            for side, sg in (("l", 1), ("r", -1)):
                for j, z in enumerate((0.92, 1.33)):
                    add(ext("ladder-%s-%d" % (side, j), RF(tone=0.3, role="detail"), [(-0.36, 1.85), (-0.42, 1.85), (-0.56, 0.50), (-0.50, 0.50)], sg * z if sg > 0 else -z - 0.03, sg * z + 0.03 if sg > 0 else -z), (4.2, 1.96, 1.96), views="efp" if side == "r" and j == 1 else "fp")
                for j, y in enumerate((0.52, 0.96, 1.40)):
                    xs = -0.36 - (1.85 - y) * 0.14 / 1.35
                    add(box("step-%s-%d" % (side, j), RF(tone=0.3, role="detail"), (xs - 0.26, y, sg * 1.14 - 0.19), (xs, y + 0.03, sg * 1.14 + 0.19)), (4.25, 1.97, 1.97), views="efp" if side == "r" else "fp")
        for side, sg in (("l", 1), ("r", -1)):
            rail = RF(tone=0.3, role="detail")
            z = sg * 1.34
            pts = [(-2.45, 1.85), (-2.45, 2.80), (-1.50, 2.80), (-1.50, 1.85)]
            for j in range(3):
                add(cap("handrail-%s-%d" % (side, j), rail, (pts[j][0], pts[j][1], z), (pts[j + 1][0], pts[j + 1][1], z), 0.017, False), (4.3, 1.98, 2.3), views="efp" if side == "r" else ("fp" if j == 1 else "f" if j == 0 else ""))
            add(cap("grab-rail-%s" % side, rail, (-0.36, 1.85, z), (-0.36, 2.80, z), 0.017, False), (4.3, 1.98, 2.3), views="ef" if side == "r" else "f")
        sill = 1.97 if not coarse else 2.05
        add(ext("cab-sill", cab_c, CAB_PLAN, 1.85, sill, "zx"), (2.2, 2.2, 2.2), views="")
        g = 0.03 if coarse else 0.006
        if coarse:
            add(ext("cab-roof", cab_c, [(0.84, -1.48), (0.84, -0.22), (0.56, 0.36), (-0.56, 0.36), (-0.84, -0.22), (-0.84, -1.48)], 3.30, 3.45, "zx"), (2.25, 2.25, 2.45), views="p")
        else:
            add(loft("cab-roof", cab_c, [((-0.56, 3.30, 0), 0.92, 0.84), ((-0.56, 3.40, 0), 0.92, 0.84), ((-0.56, 3.45, 0), 0.86, 0.78)] if fine else [((-0.56, 3.30, 0), 0.92, 0.84), ((-0.56, 3.45, 0), 0.90, 0.82)], 8.0), (2.25, 2.25, 2.45), views="p")
        w = {50: 0.07, 100: 0.09, 200: 0.10}[sc]
        for name, x0, x1, zz in (("c", -1.40, -1.40, 0.76 - w), ("b", -0.32, -0.32, 0.76 - w), ("a", 0.24 - w, 0.24 - w, 0.50 - w)):
            if coarse and name == "b":
                continue
            add(ext("cab-post-" + name, cab_c, [(x0, sill), (x0 + w, sill), (x1 + w, 3.30), (x1, 3.30)], zz, zz + w, mir=True), (2.5, 2.5, 2.5), views="e", role="detail")
        add(box("cab-rear-panel", cab_c, (-1.40, sill - 0.01, -0.70), (-1.40 + max(g, 0.015), 2.64, 0.70)))
        add(ext("cab-windscreen", glass_c, [(0.24 - g, sill - 0.01), (0.24, sill - 0.01), (0.24, 3.31), (0.24 - g, 3.31)], -0.44, 0.44), (2.4, 2.4, 2.4), views="f", sid="cab-glass-front")
        for side, sg in (("l", 1), ("r", -1)):
            a_, b_ = (0.76 - g, 0.76) if sg > 0 else (-0.76, -0.76 + g)
            add(box("cab-glass-side-" + side, glass_c, (-1.34, sill - 0.01, a_), (-0.31 if not coarse else -0.24, 3.31, b_)), (2.4, 2.4, 2.4) if side == "r" else None, views="e", sid="cab-glass-side")
            add(ext("cab-glass-corner-" + side, glass_c, [(sg * 0.76, -0.26), (sg * 0.76, -0.25), (sg * 0.49, 0.22), (sg * (0.49 - g), 0.22 - g / 2), (sg * (0.76 - g), -0.25 - g / 2), (sg * (0.76 - g), -0.26)], sill - 0.01, 3.31, "zx"), (2.4, 2.4, 2.4) if side == "r" else None, views="e", sid="cab-glass-front")
        add(box("cab-rear-glass", glass_c, (-1.40, 2.63, -0.70), (-1.40 + g, 3.31, 0.70)))
        if fine or mid:
            metal = det(cab_c, 0.16)
            rubber = metal(material=RU, tone=0.10)
            outside = [(-1.333, 2.004), (-1.31, 1.977), (-0.349, 1.977), (-0.32, 2.016), (-0.32, 3.258), (-0.36, 3.296), (-1.304, 3.296), (-1.333, 3.264)]
            inside = [(-1.317, 2.010), (-1.302, 1.993), (-0.357, 1.993), (-0.336, 2.022), (-0.336, 3.252), (-0.366, 3.280), (-1.297, 3.280), (-1.317, 3.258)]
            add(ext("cab-door-gasket", rubber, outside, -0.766, -0.758 if fine else -0.751, holes=[inside]))
            sh.add("elevation", "cab-door-gasket", "rear-frame", "detail", 2.47, 0.1, poly(outside), filled=False, region="cab", material=RU, pen=0.18)
            blade = [(2.163, -0.378), (2.174, -0.385), (2.309, 0.203), (2.308, 0.216), (2.295, 0.211)]
            add(ext("cab-wiper-blade", rubber, blade, 0.239, 0.249 if fine else 0.254, plane="yz"))
            sh.add("front", "cab-wiper-blade", "rear-frame", "detail", 2.52, 0.1, poly([(z,y) for y,z in blade]), region="cab", material=RU, pen=0.18)
            add(cap("cab-wiper-arm", metal, (0.264,1.942,0.276), (0.255,2.238,-0.076), 0.008, False), (2.52,2.52,2.52), views="f", pen=0.18)
            if fine:
                add(rev("cab-wiper-spindle", metal, [(0,0),(0,0.020),(0.012,0.020),(0.017,0.015),(0.028,0.015),(0.028,0)], "u", (0.237,1.942,0.276)), (2.53,2.53,2.53), views="f", pen=0.18)
                add(box("cab-wiper-clevis", metal, (0.246,2.229,-0.092),(0.262,2.250,-0.061)), (2.53,2.53,2.53), views="f", pen=0.18)
        if fine:
            add(box("door-handle", det(cab_c, 0.2), (-0.60, 2.40, -0.79), (-0.36, 2.44, -0.76)), (2.5, 2.5, 2.5), views="e")
            add(loft("seat", RF(material=RU, role="detail", tone=0.2, region=None), [((-0.80, 2.25, 0), 0.26, 0.25), ((-0.80, 2.42, 0), 0.26, 0.25)], 4.0))
            add(loft("seat-back", RF(material=RU, role="detail", tone=0.2, region=None), [((-1.04, 2.42, 0), 0.08, 0.24), ((-1.10, 3.05, 0), 0.07, 0.22)], 4.0))
            add(cap("steering-column", RF(role="detail", tone=0.2, region=None), (-0.10, 1.97, 0), (-0.38, 2.62, 0), 0.03, False))
            for side, sg in (("l", 1), ("r", -1)):
                for j, zz in enumerate((0.30, 0.60)):
                    add(box("work-lamp-%s-%d" % (side, j), RF(material=GL, role="detail", tone=0.9, region="cab"), (0.36, 3.31, sg * zz - 0.07), (0.42, 3.40, sg * zz + 0.07)), (2.5, 2.5, 2.5), views="efp" if side == "r" and j == 1 else "fp")
        if fine or mid:
            for side, sg in (("l", 1), ("r", -1)):
                add(cap("mirror-arm-" + side, det(cab_c, 0.2), (0.20, 3.20, sg * 0.50), (0.32, 3.20, sg * 0.96), 0.015, False), (2.5, 2.5, 2.5), views="efp" if side == "r" else "fp")
                add(box("mirror-" + side, det(cab_c, 0.2), (0.30, 2.88, sg * 0.98 - 0.11), (0.34, 3.24, sg * 0.98 + 0.11)), (2.5, 2.5, 2.5), views="efp" if side == "r" else "fp")
        sh.add("elevation", "cab", "rear-frame", "body", 2.2, 0.55, poly(CAB_SIDE), region="cab", material=PS)
        sh.add("front", "cab", "rear-frame", "body", 2.2, 0.55, poly([(-0.76, 1.85), (0.76, 1.85), (0.76, 3.30), (0.84, 3.30), (0.84, 3.40), (0.78, 3.45), (-0.78, 3.45), (-0.84, 3.40), (-0.84, 3.30), (-0.76, 3.30)]), region="cab", material=PS)
        sh.add("plan", "cab", "rear-frame", "body", 2.2, 0.55, poly([(p[1], p[0]) for p in CAB_PLAN]), region="cab", material=PS)
        if coarse:
            add(ext("front-frame", FF, TOWER, 0.38, 0.44, mir=True), (2, 2, 2), views="e")
            sh.add("front", "tower-l", "front-frame", "body", 2, 0.45, rect(0.38, 0.58, 0.44, 2.50), region="front-frame", material=PS)
            sh.add("front", "tower-r", "front-frame", "body", 2, 0.45, rect(-0.44, 0.58, -0.38, 2.50), region="front-frame", material=PS)
            add(ext("belly-plate", FF, [(0.16, 0.619), (2.10, 0.58), (2.30, 0.80), (2.24, 0.84), (2.06, 0.66), (0.16, 0.70)], -0.38, 0.38), (2, 2, 2), views="f", sid="front-frame")
            sh.add("plan", "front-frame", "front-frame", "body", 2, 0.45, rect(0.10, -0.44, 2.30, 0.44), region="front-frame", material=PS)
            add(cyl("axle-f", FF(tone=0.3), "w", (AX, TR, -0.77), 1.54, 0.13), (1.8, 1.8, 1.4), views="fp")
            add(cyl("lift-pivot", FF(tone=0.3), "w", (K.A[0], K.A[1], -0.66), 1.32, 0.07), (2.1, 2.1, 2.1), views="fp")
        else:
            add(ext("front-frame", FF, TOWER, 0.40, 0.44, mir=True), (2, 2, 2), views="e")
            sh.add("front", "tower-l", "front-frame", "body", 2, 0.45, rect(0.40, 0.58, 0.44, 2.50), region="front-frame", material=PS)
            sh.add("front", "tower-r", "front-frame", "body", 2, 0.45, rect(-0.44, 0.58, -0.40, 2.50), region="front-frame", material=PS)
            add(ext("belly-plate", FF, [(0.16, 0.619), (2.10, 0.58), (2.30, 0.80), (2.26, 0.83), (2.08, 0.63), (0.16, 0.669)], -0.40, 0.40), (2, 2, 2), views="f", sid="front-frame")
            sh.add("plan", "front-frame", "front-frame", "body", 2, 0.45, rect(0.10, -0.44, 2.30, 0.44), region="front-frame", material=PS)
            add(box("nose-plate", FF, (2.24, 0.80, -0.40), (2.30, 1.08, 0.40)))
            add(box("front-bulkhead", FF, (0.10, 0.64, -0.40), (0.16, 1.46, 0.40)))
            add(box("hitch-upper-f", FF, (-0.13, 1.34, -0.25), (0.30, 1.40, 0.25)), (1.96, 1.9, 1.9), views="e")
            add(box("hitch-lower-f", FF, (-0.13, 0.68, -0.25), (0.30, 0.74, 0.25)), (1.96, 1.9, 1.9), views="e")
            for nm, y in (("upper", 1.30), ("lower", 0.59)):
                add(cyl("hitch-pin-" + nm, FF(tone=0.3, role="detail"), "v", (0, y, 0), 0.19, 0.055), (2.6, 2.6, 2.6), views="e")
            add(rev("axle-f", FF(tone=0.3), AXLE, "w", (AX, TR, 0)), (1.8, 1.8, 1.4), views="fp")
            add(cyl("lift-pivot", FF(tone=0.3), "w", (K.A[0], K.A[1], -0.66), 1.32, 0.06), (2.1, 2.1, 2.1), views="fp")
            add(cyl("tilt-crossmember", FF(tone=0.3), "w", (K.C0[0], K.C0[1], -0.44), 0.88, 0.07), (2.1, 2.1, 2.1), views="e")
            add(cyl("lift-cyl-crossmember", FF(tone=0.3), "w", (K.E[0], K.E[1], -0.70), 1.40, 0.06), (2.1, 2.1, 2.1), views="efp")
            hy = Ctx("rear-frame", PS, "detail", 0.5, "hydraulics", None)
            for side, sg in (("l", 1), ("r", -1)):
                add(cyl("steer-cyl-" + side, hy, "u", (-0.82, 1.0, sg * 0.50), 0.72, 0.055), (2.7, 1.5, 1.5), views="efp" if side == "r" else "fp")
                add(cyl("steer-rod-" + side, hy(bone="front-frame", material=CH, tone=0.8), "u", (-0.14, 1.0, sg * 0.50), 0.56, 0.03), (2.65, 1.5, 1.5), views="efp" if side == "r" else "fp")
                add(box("head-lamp-" + side, FF(material=GL, role="detail", tone=0.9), (1.52, 1.755, sg * 1.05 - 0.13), (1.64, 1.915, sg * 1.05 + 0.13)), (4.5, 2.5, 2.5), views="efp" if side == "r" else "fp")
        add(ext("fender-f", FF, [(p[0] + AX, p[1] + TR) for p in fender_f(plate)], 0.74, 1.37, mir=True), (4.1, 1.95, 1.95))

    aw = (0.555, 0.635) if tiny else (0.565, 0.625)
    add(ext("lift-arm", LA, ARM_500 if tiny or coarse else ARM, aw[0], aw[1], mir=True), (3, 5, 3))
    if not tiny:
        add(cyl("torque-tube", LA, "w", (1.72, 0.36, -aw[0]), 2 * aw[0], 0.11), (2.95, 4.95, 2.95), views="fp")
        add(ext("lever-bracket", LA, [(1.62, 0.30), (1.86, 0.30), (2.0, 0.50), (1.9, 0.60), (1.78, 0.52)], 0.06, 0.12, mir=True), (2.96, 4.96, 2.96), views="e" if coarse else "ep")
    if fine or mid:
        add(cyl("bucket-pin", LA(tone=0.3, role="detail"), "w", (K.L, 0, -0.70), 1.40, 0.05), (3.5, 5.1, 3.1), views="e")
        add(ext("lift-lug", LA, [(0.84, 0.0), (0.92, -0.13), (1.02, -0.16), (1.10, -0.10), (1.16, 0.06)], aw[0], aw[1], mir=True), (3, 5, 3), views="e")
    if fine:
        for nm, (u, v), r in (("boss-a", (0, 0), 0.165), ("boss-b", (K.L, 0), 0.14)):
            add(mirror(cyl(nm, LA(role="detail", tone=0.3), "w", (u, v, 0.55), 0.09, r), 0.55, 0.64), (3.4, 5.05, 3.05), views="e")

    lv = add(ext("tilt-lever", LV, LEVER, -0.05, 0.05), (3.1, 5.2, 3.2)) if not tiny else None
    if tiny:
        add(ext("tilt-lever", LV, [(0.80, -0.11), (0.90, 0.0), (0.80, 0.11), (-0.02, 0.17), (-0.70, -0.19), (-0.66, -0.40), (-0.02, -0.16)], -0.05, 0.05), (3.1, 5.2, 3.2))
        add(ext("tilt-link", LK, [(-0.08, -0.06), (0.98, -0.06), (0.98, 0.06), (-0.08, 0.06)], -0.08, 0.08), (3.15, 5.25, 3.25))
    elif coarse:
        add(ext("tilt-link", LK, [(-0.10, -0.08), (1.0, -0.08), (1.0, 0.08), (-0.10, 0.08)], -0.10, 0.10), (3.15, 5.25, 3.25))
    else:
        add(ext("tilt-link", LK, LINK, 0.06, 0.10, mir=True), (3.15, 5.25, 3.25))
    if fine:
        add(cyl("lever-pin", LV(role="detail", tone=0.3), "w", (0, 0, -0.13), 0.26, 0.06), (3.6, 5.3, 3.3), views="e")
        add(cyl("link-pin", LK(role="detail", tone=0.3), "w", (0, 0, -0.11), 0.22, 0.045), (3.6, 5.3, 3.3), views="e")

    hyd = lambda bone, mat, tone: Ctx(bone, mat, "detail", tone, "hydraulics", bone)
    lr, tr_ = LIFT_REST - LIFT_BARREL, TILT_REST - TILT_BARREL
    detail = fine or mid

    def cylinder(name, bone, rb, rr, barrel, rod, zz, zs, near):
        hb, hr = hyd(bone + "-cyl", PS, 0.5), hyd(bone + "-rod", CH, 0.8)
        v = "efp" if near else "fp"
        sfx = name[len(bone) + 4:]
        bprof = [(0, 0), (0, rb), (barrel - 0.06, rb), (barrel - 0.06, rb + 0.01), (barrel, rb + 0.01), (barrel, 0)] if detail else [(-0.06, 0), (-0.06, rb), (barrel, rb), (barrel, 0)]
        add(rev(name, hb, bprof, "u", (0, 0, zz)), zs, views=v, sid=bone + "-cyl-barrel" + sfx if near and not sfx else None)
        add(cyl(bone + "-rod" + sfx, hr, "u", (-rod[0], 0, zz), rod[0] + rod[1] - (0.05 if detail else -0.04), rr), tuple(x - 0.05 for x in zs), views=(("e" if near else "") if coarse and bone == "lift" else v))
        if detail:
            add(cyl(bone + "-cyl-eye" + sfx, hb, "w", (0, 0, zz - 0.06), 0.12, rb - 0.015), zs, views=v)
            add(cyl(bone + "-rod-eye" + sfx, hr(material=PS, tone=0.5), "w", (rod[1], 0, zz - 0.06), 0.12, rr + 0.03), zs, views=v)

    cylinder("lift-cyl-l", "lift", 0.095, 0.045 if detail else 0.05, LIFT_BARREL, (1.0, lr), 0.595, (2.9, 4.9, 2.9), False)
    cylinder("lift-cyl-r", "lift", 0.095, 0.045 if detail else 0.05, LIFT_BARREL, (1.0, lr), -0.595, (2.9, 4.9, 2.9), True)
    cylinder("tilt-cyl", "tilt", 0.11, 0.05 if detail else 0.055, TILT_BARREL, (0.75, tr_), 0, (3.3, 5.3, 3.3), True)

    t = {50: 0.02, 100: 0.025, 200: 0.035, 500: 0.075}[sc]
    out = BK_OUT if fine or mid else BK_OUT_COARSE
    add(ext("bucket-shell", BK, strip(out, t), -1.475 + t, 1.475 - t), (3.2, 6.2, 3.2), views="")
    side = out + ([(1.20, -0.21), (0.86, 0.42)] if fine or mid else [])
    add(ext("bucket-side", BK, side, 1.475 - t, 1.475, mir=True), (3.2, 6.2, 3.2), views="")
    sh.add("elevation", "bucket", "bucket", "body", 3.2, 0.25, poly(side), region="bucket", material=WS)
    for v, zl in (("front", 6.2), ("plan", 3.2)):
        sh.add(v, "bucket", "bucket", "body", zl, 0.25, poly(side), region="bucket", material=WS, solid=dict(kind="extruded", sweep="w", z0=-1.475, z1=1.475))
    if not tiny:
        add(ext("bucket-edge", BK(role="detail", tone=0.1), BK_EDGE, -1.475, 1.475), (3.4, 6.4, 3.4))
        add(ext("bucket-hinge", BK(tone=0.2), BK_HINGE, 0.50, 0.69, mir=True) if not fine else ext("bucket-hinge-o", BK(tone=0.2), BK_HINGE, 0.65, 0.69, mir=True), (3.45, 6.1, 3.1), sid="bucket-ear", views="ep")
        add(ext("tilt-lug", BK(tone=0.2), BK_LUG, -0.05, 0.05), (3.45, 6.1, 3.1), views="ep")
    if fine:
        add(ext("bucket-hinge-i", BK(tone=0.2), BK_HINGE, 0.50, 0.54, mir=True))
    if fine or mid:
        add(ext("bucket-beam", BK, BK_BEAM, -1.475 + t, 1.475 - t), (3.3, 6.1, 3.3), views="p", sid="bucket-rib-1", filled=False)
    if fine:
        for i, zz in enumerate((1.0, -1.04)):
            add(ext("bucket-rib-%d" % (i + 2), BK, [(0.10, 0.05), (0.15, 0.05), (0.20, 0.50), (0.14, 0.50)], zz, zz + 0.04), (3.3, 6.1, 3.3), views="p", filled=False)

    for bone in ("wheel-r", "wheel-f"):
        k = bone[-1]
        ox = WHEEL[bone][0]
        tc = Ctx(bone, RU, "body", 0.12, "tyres", bone)
        rc = Ctx(bone, CI, "detail", 0.6, "front-frame", bone)
        for side, sg in (("l", 1), ("r", -1)):
            zc = sg * TZ
            near = side == "r"
            if fine:
                add(rev("tyre-%s-%s" % (k, side), tc, TYRE, "w", (0, 0, zc)), None)
                for row, (a0, sw) in enumerate(((0.0, 1), (7.5, -1))):
                    lug = base("lugs-%s-%s-%d" % (k, side, row), "extrude", tc(role="detail", tone=0.05), profile=P(ccw([(r, sw * w) for r, w in LUG])), plane="yz", transform={"translation": [0, 0, r4(zc)], "rotation": [0, 0, r4(math.sin(math.radians(a0) / 2)), r4(math.cos(math.radians(a0) / 2))]}, repeat={"count": 24, "rotationRad": r4(math.pi / 12), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}, **{"from": -0.0575, "to": 0.0575})
                    B.append(lug)
            else:
                add(rev("tyre-%s-%s" % (k, side), tc, TYRE_PLAIN, "w", (0, 0, zc)), None)
            o = 1 if sg > 0 else -1
            rim = [(-0.26, 0.30), (-0.26, 0.365), (-0.235, 0.365), (-0.235, 0.325), (0.235, 0.325), (0.235, 0.365), (0.26, 0.365), (0.26, 0.30), (0.13, 0.30), (0.13, 0.17), (0.09, 0.17), (0.09, 0.30), (-0.26, 0.30)] if fine or mid else [(-0.26, 0), (-0.26, 0.365), (0.26, 0.365), (0.26, 0)]
            add(rev("rim-%s-%s" % (k, side), rc, [(o * a, r) for a, r in rim] if fine or mid else rim, "w", (0, 0, zc)), None)
            if fine or mid:
                hub = [(0.02, 0), (0.02, 0.19), (0.20, 0.19), (0.25, 0.15), (0.27, 0.08), (0.27, 0)]
                add(rev("hub-%s-%s" % (k, side), rc(tone=0.3), [(o * a, r) for a, r in hub][:: 1 if o > 0 else -1], "w", (0, 0, zc)), None)
            if fine:
                add(rev("bolt-%s-%s" % (k, side), rc(tone=0.0), [(0, 0), (0, 0.02), (0.035, 0.02), (0.035, 0)], "w", (0.245, 0, zc + (0.13 if o > 0 else -0.165)), repeat={"count": 12, "rotationRad": r4(math.pi / 6), "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}), None)
            up = [(0.30, 0.62), (0.285, 0.71), (0.285, TR)] if fine else [(0.30, 0.66), (0.27, 0.775), (0.20, TR)]
            q = up + [(-a, r) for a, r in up[::-1]]
            prof = q + [(-a, -r) for a, r in q]
            sh.add("front", "tyre-%s-%s" % (k, side), bone, "body", 4 if k == "f" else 1.5, 0.12, poly([(zc + a, TR + r) for a, r in prof]), region="tyres", material=RU)
            sh.add("plan", "tyre-%s-%s" % (k, side), bone, "body", 1.5, 0.12, poly([(ox + r, zc + a) for a, r in prof]), region="tyres", material=RU)
        sh.add("elevation", "tyre-" + k if not tiny else "wheel-" + k, bone, "body", 4, 0.12, circle(0, 0, TR), region="tyres", material=RU)
        sh.add("elevation", "rim-" + k, bone, "detail", 4.2, 0.6, circle(0, 0, 0.365), region="front-frame", material=CI)
        if fine or mid:
            sh.add("elevation", "hub-" + k, bone, "detail", 4.3, 0.3, circle(0, 0, 0.19), region="front-frame", material=CI)
            for i in range(12 if fine else 6):
                a = math.radians(i * (30 if fine else 60))
                sh.add("elevation", "bolt-%s-%d" % (k, i + 1), bone, "detail", 4.3, 0.0, circle(0.245 * math.cos(a), 0.245 * math.sin(a), 0.02 if fine else 0.03), filled=False, region="front-frame", material=CI)
        if fine:
            lug = []
            for i in range(48):
                a0, a1 = math.radians(i * 7.5 - 2), math.radians(i * 7.5 + 2)
                lug += [(0.7775 * math.cos(a0), 0.7775 * math.sin(a0)), (0.7775 * math.cos(a1), 0.7775 * math.sin(a1))] if i % 2 == 0 else [(0.74 * math.cos(a0), 0.74 * math.sin(a0)), (0.74 * math.cos(a1), 0.74 * math.sin(a1))]
            sh.add("elevation", "lugs-" + k, bone, "texture", 4.1, 0.05, poly(lug), filled=False, region="tyres", material=RU)

    return B, sh
