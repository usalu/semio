import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_43_lib import *

SLUG = "city-bicycle"
FR, ST, WR, WF, CR, PN, PF, SD = "frame", "steer", "wheel-r", "wheel-f", "crank", "pedal-n", "pedal-f", "stand"
PAINT, RUBBER, ALLOY, SADDLE, CHROME, PLASTIC = "paint", "rubber", "alloy", "saddle-skin", "chrome", "plastic"
BB = (-0.1, 0.3)
HUBR = (-0.58, 0.345)
HUBF = (0.560017, 0.345007)


def cap_local(id, bone, mat, tone, a, b, r, r1=None, role="body", **kw):
    s = capsule(id, bone, mat, tone, a, b, r, r1, role=role)
    s.update(kw)
    return s


def saddle50(B):
    xs = [-0.19, -0.22, -0.25, -0.30, -0.35, -0.40, -0.45, -0.48, -0.49]
    bot = {-0.19: 0.975, -0.22: 0.982, -0.25: 0.99, -0.30: 0.9875, -0.35: 0.986, -0.40: 0.985, -0.45: 0.992, -0.48: 1.0, -0.49: 1.008}
    top = {-0.19: 1.0, -0.22: 1.016, -0.25: 1.03, -0.30: 1.035, -0.35: 1.038, -0.40: 1.04, -0.45: 1.041, -0.48: 1.04, -0.49: 1.035}
    hw = {-0.19: 0.02, -0.22: 0.03, -0.25: 0.04, -0.30: 0.065, -0.35: 0.085, -0.40: 0.081, -0.45: 0.078, -0.48: 0.076, -0.49: 0.075}
    secs = []
    for x in xs:
        secs.append(sec([x, (bot[x] + top[x]) / 2, 0], hw[x], (top[x] - bot[x]) / 2, exponent=2.6, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])))
    secs = [dict(s, at=B.P.loc(FR, s["at"])) for s in secs]
    return loft("saddle", FR, SADDLE, 0.2, secs)


def build50():
    B = Bike(50, SLUG)
    P = B.P
    b = B.b
    for tag, bone, hub_r, tyre_out in (("r", WR, 0.05, 0.345), ("f", WF, 0.055, 0.345)):
        w = wheel_solids(B, bone, tag, hub_r, 0.345, 0.309, 0.283, 0.02, 0.0125, 0.02)
        b(w["tyre"]); b(w["rim"]); b(w["hub"])
        for f in w["flanges"]:
            b(f)
        b(w["axle"])
        for s in w["spokes"]:
            b(s)
    b(B.cap("seat-tube", FR, PAINT, 0.3, [BB[0], BB[1], 0], [-0.285, 0.8, 0], 0.0155))
    b(B.cap("seat-post", FR, CHROME, 0.5, [-0.285, 0.8, 0], [-0.348, 0.972, 0], 0.0135))
    b(B.tube("seat-lug", FR, PAINT, 0.3, [-0.2815, 0.79, 0], [-0.2865, 0.805, 0], 0.0185, role="detail"))
    pts, _ = fillet([[0.347, 0.8, 0], [0.24, 0.52, 0], [0.1, 0.38, 0], [BB[0], BB[1], 0]], 0.12)
    b(B.sweep("main-tube", FR, PAINT, 0.3, pts, 0.019, step=0.0))
    b(B.tube("head-tube", FR, PAINT, 0.3, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.0225))
    b(B.tube("head-cup-top", FR, CHROME, 0.5, [0.2975, 0.9285, 0], [0.2945, 0.9368, 0], 0.0255, role="detail"))
    b(B.tube("head-cup-bottom", FR, CHROME, 0.5, [0.3504, 0.7915, 0], [0.3469, 0.8007, 0], 0.0255, role="detail"))
    b(B.tube("bb-shell", FR, PAINT, 0.3, [BB[0], BB[1], -0.035], [BB[0], BB[1], 0.035], 0.0185))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(B.cap(f"chainstay-{nm}", FR, PAINT, 0.3, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.01))
        b(B.cap(f"seatstay-{nm}", FR, PAINT, 0.3, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.008))
    b(ext("rack-deck", FR, PAINT, 0.3, "zx", [[-0.09, -0.84], [-0.09, -0.3], [0.09, -0.3], [0.09, -0.84]], 0.792, 0.8))
    b(ext("rack-rim", FR, PAINT, 0.3, "zx", [[-0.09, -0.84], [-0.09, -0.3], [0.09, -0.3], [0.09, -0.84]], 0.8, 0.81, holes=[[[-0.076, -0.826], [-0.076, -0.314], [0.076, -0.314], [0.076, -0.826]]]))
    b(saddle50(B))
    chain_outline = [[-0.1, 0.375], [-0.3, 0.385], [-0.55, 0.385], [-0.58, 0.345], [-0.55, 0.305], [-0.3, 0.31], [-0.1, 0.225], [-0.025, 0.3]]
    b(ext("chaincase", FR, PAINT, 0.6, "xy", offset_poly(chain_outline, 0.004), -0.1, -0.04))
    b(ext("chaincase-rim", FR, PAINT, 0.6, "xy", chain_outline, -0.096, -0.044, role="detail"))
    ang = lambda a: a
    b(B.fender("fender-r", FR, PAINT, 0.55, HUBR, 0.347, 0.016, 0.032, 0.008, 25.0, 205.0))
    b(B.fender("fender-f", ST, PAINT, 0.55, HUBF, 0.347, 0.016, 0.032, 0.008, -25.0, 150.0))
    b(box("tail-lamp", FR, PLASTIC, 0.3, [0.045, 0.06, 0.06], at=[-0.9375, 0.2, 0], r=0.008))
    lamp = ellipsoid("head-lamp", ST, PLASTIC, 0.85, [0.045, 0.035, 0.045], at=P.loc(ST, [0.445, 0.9398, 0]))
    lamp["transform"]["rotation"] = qaxis([0, 0, 1], 70)
    b(lamp)
    # steer
    for sg, nm in ((1, "l"), (-1, "r")):
        zz = sg * 0.045
        b(B.sweep(f"fork-{nm}", ST, PAINT, 0.3, [[0.14 + (0.641 - 0.14) * k / 10, 0.0445 * k / 10, zz] for k in range(11)], 0.007, rx=0.007, ry=lambda t: 0.014 - 0.0005 * t, step=0, local=True))
    b(cap_local("fork-crown", ST, PAINT, 0.3, [0.15, 0.0, -0.052], [0.15, 0.0, 0.052], 0.014))
    neck, _ = fillet([[0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0]], 0.05)
    b(loft("stem-neck", ST, PAINT, 0.3, [sec(p, 0.0115, 0.0115) for p in resample(neck, 0.012)], capStart=False))
    b(ellipsoid("stem-clamp", ST, CHROME, 0.5, [0.0125, 0.0125, 0.02], at=[-0.2135, -0.198, 0.0]))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"handlebar-{nm}", ST, PAINT, 0.3, [-0.2145, -0.19755, 0], [-0.216, -0.263, sg * 0.26], 0.014))
        g = cap_local(f"grip-{nm}", ST, "rubber", 0.1, [-0.216, -0.263, sg * 0.26], [-0.214, -0.2722, sg * 0.294], 0.016)
        b(g)
    # drivetrain
    b(B.tube("crank-axle", CR, CHROME, 0.5, [BB[0], BB[1], -0.085], [BB[0], BB[1], 0.085], 0.0085, role="detail"))
    b(revolve("chainring", CR, ALLOY, 0.6, "z", [0, 0, -0.062], [[0, 0], [0, 0.09], [0.008, 0.09], [0.008, 0], [0, 0]], role="detail"))
    for sg, nm, z in ((1, "n", -0.07), (-1, "f", 0.07)):
        b(B.sweep(f"crank-arm-{nm}", CR, PAINT, 0.3, [[0.17 * k / 8 * sg, 0, z] for k in range(9)], 0.01, rx=0.01, ry=lambda t: 0.0125 - 0.0035 * t, step=0, local=True))
    b(box("pedal-n", PN, CHROME, 0.2, [0.1, 0.025, 0.09], at=[0, 0, -0.12], r=0.006))
    b(box("pedal-f", PF, CHROME, 0.2, [0.1, 0.025, 0.09], at=[0, 0, 0.12], r=0.006))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"stand-leg-{nm}", SD, PAINT, 0.3, [0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.007))
    b(cap_local("stand-foot", SD, PAINT, 0.3, [0.32, 0, -0.075], [0.32, 0, 0.075], 0.007))
    b(cap_local("stand-pivot", SD, PAINT, 0.3, [0, 0, -0.082], [0, 0, 0.082], 0.007))
    return B


def saddle_n(B, k, tall=False):
    xs = [-0.19, -0.25, -0.35, -0.45, -0.49] if k == 5 else [-0.19, -0.30, -0.49]
    bot = {-0.19: 0.975, -0.22: 0.982, -0.25: 0.99, -0.30: 0.9875, -0.35: 0.986, -0.40: 0.985, -0.45: 0.992, -0.48: 1.0, -0.49: 1.008}
    top = {-0.19: 1.0, -0.22: 1.016, -0.25: 1.03, -0.30: 1.035, -0.35: 1.038, -0.40: 1.04, -0.45: 1.041, -0.48: 1.04, -0.49: 1.035}
    hw = {-0.19: 0.02, -0.22: 0.03, -0.25: 0.04, -0.30: 0.065, -0.35: 0.085, -0.40: 0.081, -0.45: 0.078, -0.48: 0.076, -0.49: 0.075}
    if tall:
        bot = {x: min(v, 0.99) for x, v in bot.items()}
        bot[-0.19] = 0.975
        topd = {-0.19: 1.015, -0.30: 1.05, -0.49: 1.055}
        top = {x: topd[x] for x in xs}
    secs = []
    for x in xs:
        secs.append(sec(B.P.loc(FR, [x, (bot[x] + top[x]) / 2, 0]), hw[x], (top[x] - bot[x]) / 2, exponent=(4.0 if tall else 2.6), rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])))
    return loft("saddle", FR, SADDLE, 0.2, secs)


def build100():
    B = Bike(100, SLUG)
    P = B.P
    b = B.b
    for tag, bone, hub_r in (("r", WR, 0.05), ("f", WF, 0.055)):
        b(revolve(f"tyre-{tag}", bone, RUBBER, 0.1, "z", [0, 0, 0], superpoly(0, 0.327, 0.02, 0.018, 3.0, 28)))
        b(revolve(f"rim-{tag}", bone, ALLOY, 0.78, "z", [0, 0, 0], superpoly(0, 0.297, 0.0125, 0.012, 6.0, 24)))
        b(revolve(f"hub-{tag}", bone, CHROME, 0.7, "z", [0, 0, 0], [[-0.02, 0], [-0.02, hub_r], [0.02, hub_r], [0.02, 0], [-0.02, 0]]))
        b(revolve(f"axle-{tag}", bone, CHROME, 0.5, "z", [0, 0, 0], [[-0.05, 0], [-0.05, 0.016], [0.05, 0.016], [0.05, 0], [-0.05, 0]]))
    seat, _ = fillet([[BB[0], BB[1], 0], [-0.285, 0.8, 0], [-0.348, 0.972, 0]], 0.3)
    b(B.sweep("seat-tube", FR, PAINT, 0.3, seat, 0.016, rx=0.0165, ry=lambda t: 0.0165 if t < 0.74 else 0.0155, step=0, local=False, slender=True))
    pts, _ = fillet([[0.347, 0.8, 0], [0.24, 0.52, 0], [0.1, 0.38, 0], [BB[0], BB[1], 0]], 0.12)
    b(B.sweep("main-tube", FR, PAINT, 0.3, pts, 0.0175, step=0.0))
    b(B.tube("head-tube", FR, PAINT, 0.3, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.022))
    b(B.tube("bb-shell", FR, PAINT, 0.3, [BB[0], BB[1], -0.035], [BB[0], BB[1], 0.035], 0.0185))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(B.cap(f"chainstay-{nm}", FR, PAINT, 0.3, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.0105, slender=True))
        b(B.cap(f"seatstay-{nm}", FR, PAINT, 0.3, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.0095, slender=True))
    b(ext("rack", FR, PAINT, 0.3, "zx", [[-0.09, -0.84], [-0.09, -0.3], [0.09, -0.3], [0.09, -0.84]], 0.79, 0.81))
    b(saddle_n(B, 5))
    chain_outline = [[-0.1, 0.375], [-0.3, 0.385], [-0.55, 0.385], [-0.58, 0.345], [-0.55, 0.305], [-0.3, 0.31], [-0.1, 0.225], [-0.025, 0.3]]
    b(ext("chaincase", FR, PAINT, 0.6, "xy", chain_outline, -0.1, -0.04))
    b(B.fender("fender-r", FR, PAINT, 0.55, HUBR, 0.347, 0.016, 0.032, 0.015, 25.0, 205.0))
    b(B.fender("fender-f", ST, PAINT, 0.55, HUBF, 0.347, 0.016, 0.032, 0.015, -25.0, 150.0))
    b(box("tail-lamp", FR, PLASTIC, 0.3, [0.05, 0.08, 0.06], at=[-0.935, 0.2, 0], r=0.01))
    lamp = ellipsoid("head-lamp", ST, PLASTIC, 0.85, [0.045, 0.035, 0.045], at=P.loc(ST, [0.445, 0.9398, 0]))
    lamp["transform"]["rotation"] = qaxis([0, 0, 1], 70)
    b(lamp)
    for sg, nm in ((1, "l"), (-1, "r")):
        b(B.sweep(f"fork-{nm}", ST, PAINT, 0.3, [[0.14 + (0.641 - 0.14) * k / 4, 0.0445 * k / 4, sg * 0.045] for k in range(5)], 0.0078, rx=0.0078, ry=0.0145, step=0, local=True))
    b(cap_local("fork-crown", ST, PAINT, 0.3, [0.15, 0.0, -0.05], [0.15, 0.0, 0.05], 0.0145))
    neck, _ = fillet([[0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0]], 0.05)
    b(loft("stem-neck", ST, PAINT, 0.3, [sec(p, 0.0125, 0.0125) for p in resample(neck, 0.03)], capStart=False))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"handlebar-{nm}", ST, PAINT, 0.3, [-0.2145, -0.19755, 0], [-0.216, -0.263, sg * 0.26], 0.014))
        b(cap_local(f"grip-{nm}", ST, "rubber", 0.1, [-0.216, -0.263, sg * 0.26], [-0.214, -0.2722, sg * 0.294], 0.016))
    for sg, nm, z in ((1, "n", -0.07), (-1, "f", 0.07)):
        b(B.sweep(f"crank-arm-{nm}", CR, PAINT, 0.3, [[0.17 * k / 3 * sg, 0, z] for k in range(4)], 0.01, rx=0.01, ry=lambda t: 0.0135 - 0.004 * t, step=0, local=True))
    b(box("pedal-n", PN, CHROME, 0.2, [0.1, 0.025, 0.09], at=[0, 0, -0.12], r=0.006))
    b(box("pedal-f", PF, CHROME, 0.2, [0.1, 0.025, 0.09], at=[0, 0, 0.12], r=0.006))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"stand-leg-{nm}", SD, PAINT, 0.3, [0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.0075, slender=True))
    b(cap_local("stand-foot", SD, PAINT, 0.3, [0.32, 0, -0.075], [0.32, 0, 0.075], 0.0075))
    return B




def build200():
    B = Bike(200, SLUG)
    P = B.P
    b = B.b
    for tag, bone, hub_r, hw in (("r", WR, 0.05, 0.0275), ("f", WF, 0.055, 0.03)):
        b(revolve(f"tyre-{tag}", bone, RUBBER, 0.1, "z", [0, 0, 0], superpoly(0, 0.31, hw, 0.035, 4.0, 24)))
        b(revolve(f"hub-{tag}", bone, CHROME, 0.7, "z", [0, 0, 0], [[-0.05, 0], [-0.05, 0.02], [-0.02, 0.02], [-0.02, hub_r], [0.02, hub_r], [0.02, 0.02], [0.05, 0.02], [0.05, 0], [-0.05, 0]]))
    seat, _ = fillet([[BB[0], BB[1], 0], [-0.285, 0.8, 0], [-0.348, 0.972, 0]], 0.3)
    b(B.sweep("seat-tube", FR, PAINT, 0.3, seat, 0.014, rx=0.0235, ry=lambda t: 0.0155 if t < 0.74 else 0.0150, step=0))
    pts, _ = fillet([[0.347, 0.8, 0], [0.24, 0.52, 0], [0.1, 0.38, 0], [BB[0], BB[1], 0]], 0.12)
    b(B.sweep("main-tube", FR, PAINT, 0.3, pts, 0.0175, rx=lambda t: 0.02 + 0.016 * t, ry=0.0175, step=0.0))
    b(B.tube("head-tube", FR, PAINT, 0.3, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.022))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(B.cap(f"chainstay-{nm}", FR, PAINT, 0.3, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.0125, slender=True))
        b(B.cap(f"seatstay-{nm}", FR, PAINT, 0.3, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.0115, slender=True))
    b(ext("rack", FR, PAINT, 0.3, "zx", [[-0.09, -0.97], [-0.09, -0.3], [0.09, -0.3], [0.09, -0.97]], 0.79, 0.82))
    web = [sec(P.loc(FR, [x, 0.3 + (0.345 - 0.3) * (BB[0] - x) / (BB[0] - HUBR[0]), 0]), 0.035 + 0.02 * (BB[0] - x) / (BB[0] - HUBR[0]), 0.015, exponent=6, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])) for x in (-0.1, -0.2, -0.3, -0.4, -0.5, -0.58)]
    b(loft("chainstay-web", FR, PAINT, 0.3, web))
    b(saddle_n(B, 3, tall=True))
    chain_outline = [[-0.1, 0.375], [-0.55, 0.385], [-0.58, 0.345], [-0.55, 0.305], [-0.1, 0.225], [-0.025, 0.3]]
    b(ext("chaincase", FR, PAINT, 0.6, "xy", chain_outline, -0.1, -0.04))
    b(box("tail-lamp", FR, PLASTIC, 0.3, [0.08, 0.1, 0.06], at=[-0.93, 0.2, 0], r=0.01))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(B.sweep(f"fork-{nm}", ST, PAINT, 0.3, [[0.14 + (0.641 - 0.14) * k / 2, 0.0445 * k / 2, sg * 0.045] for k in range(3)], 0.0078, rx=0.0078, ry=0.0122, step=0, local=True, slender=True))
    b(cap_local("fork-crown", ST, PAINT, 0.3, [0.15, 0.0, -0.05], [0.15, 0.0, 0.05], 0.0155))
    neck, _ = fillet([[0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0]], 0.05)
    b(loft("stem-neck", ST, PAINT, 0.3, [sec(p, 0.0155, 0.0155) for p in resample(neck, 0.05)], capStart=False))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"handlebar-{nm}", ST, PAINT, 0.3, [-0.2145, -0.19755, 0], [-0.214, -0.2722, sg * 0.294], 0.0155))
    b(box("pedal-n", PN, CHROME, 0.2, [0.03, 0.03, 0.03], at=[0, 0, -0.12], r=0.005))
    b(box("pedal-f", PF, CHROME, 0.2, [0.03, 0.03, 0.03], at=[0, 0, 0.12], r=0.005))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cap_local(f"stand-leg-{nm}", SD, PAINT, 0.3, [0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.02))
    return B


def build200_thermal_extras(B):
    th = B.thermal
    P = B.P
    for tag, bone, hub_hw in (("r", FR, 0), ("f", ST, 0)):
        pass
    f_r = B.fender("th-fender-r", FR, PAINT, 0.55, HUBR, 0.347, 0.016, 0.032, 0.03, 25.0, 205.0)
    f_f = B.fender("th-fender-f", ST, PAINT, 0.55, HUBF, 0.347, 0.016, 0.032, 0.03, -25.0, 150.0)
    for f in (f_r, f_f):
        f["region"] = "mudguards"
        th.append(f)
    for tag, bone in (("r", WR), ("f", WF)):
        r = revolve(f"th-rim-{tag}", bone, ALLOY, 0.78, "z", [0, 0, 0], superpoly(0, 0.297, 0.0155, 0.0155, 6.0, 20))
        r["region"] = "rims-spokes"
        th.append(r)
    for sg, nm in ((1, "l"), (-1, "r")):
        pass
    pad = None


STEEL = "paint"


def struct50(B):
    P = B.P
    st = B.st
    W = 0.0015
    # frame
    st(B.tube("seat-tube", FR, STEEL, 0.4, [BB[0], BB[1], 0], [-0.285, 0.8, 0], 0.0145, 0.0145 - W), FR, "member")
    st(B.tube("seat-post", FR, STEEL, 0.4, [-0.285, 0.8, 0], [-0.348, 0.972, 0], 0.0125, 0.0125 - W), FR, "member")
    for d in B.tube_path("main-tube", FR, STEEL, 0.4, [[0.347, 0.8], [0.24, 0.52], [0.1, 0.38], [BB[0], BB[1]]], 0.12, 0.018, 0.018 - W):
        st(d, FR, "member")
    st(B.tube("head-tube", FR, STEEL, 0.4, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.0205, 0.0205 - W), FR, "member")
    st(B.tube("bb-shell", FR, STEEL, 0.4, [BB[0], BB[1], -0.033], [BB[0], BB[1], 0.033], 0.0175, 0.0175 - W), FR, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.tube(f"chainstay-{nm}", FR, STEEL, 0.4, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.0095, 0.0095 - W), FR, "member")
        st(B.tube(f"seatstay-{nm}", FR, STEEL, 0.4, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.0075, 0.0075 - W), FR, "member")
        st(B.tube(f"rack-rail-{nm}", FR, STEEL, 0.4, [-0.835, 0.8, sg * 0.078], [-0.305, 0.8, sg * 0.078], 0.0075, 0.0075 - W), FR, "member")
    # steer
    st(B.tube("steerer", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, 0]), P.wld(ST, [-0.0, 0, 0]), 0.011, 0.011 - W), ST, "member")
    neck_w = [P.wld(ST, p) for p in ([0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0])]
    for d in B.tube_path("stem-neck", ST, STEEL, 0.4, [[p[0], p[1]] for p in neck_w], 0.05, 0.0105, 0.0105 - W):
        st(d, ST, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.tube(f"fork-{nm}", ST, STEEL, 0.4, P.wld(ST, [0.14, 0, sg * 0.045]), P.wld(ST, [0.641, 0.0445, sg * 0.045]), 0.0066, 0.0066 - W), ST, "member")
        st(B.tube(f"handlebar-{nm}", ST, STEEL, 0.4, P.wld(ST, [-0.2145, -0.19755, 0]), P.wld(ST, [-0.216, -0.263, sg * 0.26]), 0.0135, 0.0135 - W), ST, "member")
    st(B.tube("fork-crown", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, -0.05]), P.wld(ST, [0.15, 0, 0.05]), 0.0125, 0.0125 - W), ST, "member")
    # wheels
    for tag, bone in (("r", WR), ("f", WF)):
        rr, hw, ro_, ri_ = 0.296, 0.0115, 0.013, 0.0115
        for nm, sgn in (("a", 1), ("b", -1)):
            n = 10
            pts = [[sgn * ro_ * math.sin(math.pi * i / n), rr + ro_ * math.cos(math.pi * i / n)] for i in range(n + 1)]
            pts += [[sgn * ri_ * math.sin(math.pi * (n - i) / n), rr + ri_ * math.cos(math.pi * (n - i) / n)] for i in range(n + 1)]
            pts.append(pts[0])
            st(revolve(f"rim-{tag}-{nm}", bone, ALLOY, 0.7, "z", [0, 0, 0], pts), bone, "member")
        R = 0.286
        for gi, (zf, lead) in enumerate(((0.03, 1), (0.03, -1), (-0.03, 1), (-0.03, -1))):
            a_h = math.radians(10 * gi)
            a_r = a_h + lead * math.radians(62)
            s = capsule(f"spoke-{tag}-{gi + 1}", bone, ALLOY, 0.6, [0.032 * math.cos(a_h), 0.032 * math.sin(a_h), zf], [R * math.cos(a_r), R * math.sin(a_r), 0.0], 0.0015, 0.0015, repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / 9), "origin": [0, 0, 0]})
            s["slender"] = True
            st(s, bone, "member")
        st(revolve(f"axle-{tag}", bone, CHROME, 0.5, "z", [0, 0, 0], [[-0.05, 0], [-0.05, 0.0095], [0.05, 0.0095], [0.05, 0], [-0.05, 0]]), bone, "joint")
    # drivetrain
    st(B.tube("crank-axle", CR, STEEL, 0.4, [BB[0], BB[1], -0.085], [BB[0], BB[1], 0.085], 0.0085, None), CR, "member")
    for sg, nm, z in ((1, "n", -0.07), (-1, "f", 0.07)):
        st(B.sweep(f"crank-arm-{nm}", CR, STEEL, 0.4, [[0.165 * k / 6 * sg, 0, z] for k in range(7)], 0.008, rx=0.0075, ry=lambda t: 0.0095 - 0.003 * t, step=0, local=True), CR, "member")
    for nm, bone, z in (("n", PN, -0.12), ("f", PF, 0.12)):
        st(cyl(f"pedal-spindle-{nm}", bone, STEEL, 0.4, "z", [0, 0, z], 0.09, 0.006), bone, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(capsule(f"stand-leg-{nm}", SD, STEEL, 0.4, [0.0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.005, 0.005), SD, "member")
    st(capsule("stand-bar", SD, STEEL, 0.4, [0.32, 0, -0.075], [0.32, 0, 0.075], 0.005, 0.005), SD, "member")
    # joints
    st(ellipsoid("head-bearing", ST, CHROME, 0.5, [0.0185, 0.0185, 0.0185], at=[0.01, 0.0, 0.0]), ST, "joint")
    st(ellipsoid("bb-bearing", CR, CHROME, 0.5, [0.0165, 0.0165, 0.025], at=[0, 0, 0]), CR, "joint")
    st(ellipsoid("pedal-bearing-n", PN, CHROME, 0.5, [0.008, 0.008, 0.008], at=[0, 0, -0.085]), PN, "joint")
    st(ellipsoid("pedal-bearing-f", PF, CHROME, 0.5, [0.008, 0.008, 0.008], at=[0, 0, 0.085]), PF, "joint")
    st(ellipsoid("stand-hinge", SD, CHROME, 0.5, [0.009, 0.009, 0.012], at=[0, 0, 0.0]), SD, "joint")
    # support
    hp = P.loc(ST, [0.56, 0.0, 0.0])
    st(dict(box("contact-pad-f", ST, "rubber", 0.1, [0.04, 0.0075, 0.037], at=[hp[0], hp[1] + 0.00375, 0.0]), anchor="contact-f"), ST, "support")
    # masses
    st(ellipsoid("mass-steer", ST, STEEL, 0.4, [0.02, 0.02, 0.02], at=[0.3161, -0.0086, 0.0]), ST, "mass")
    st(ellipsoid("mass-wheel-r", WR, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WR, "mass")
    st(ellipsoid("mass-wheel-f", WF, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WF, "mass")


def temp_lists(B):
    B.thermal = [{"id": "th-tmp", "of": B.body[0]["id"], "region": "tyres"}]
    B.structure = [dict(B.body[0], id="st-tmp", member="wheel-f", part="member")]
    B.dec("tmp", "tmp")


def struct_coarse(B, den):
    P = B.P
    st = B.st
    W = 0.002 if den == 100 else 0.003
    nspoke = 5 if den == 100 else 3
    st(B.tube("seat-tube", FR, STEEL, 0.4, [BB[0], BB[1], 0], [-0.285, 0.8, 0], 0.0145, 0.0145 - W), FR, "member")
    st(B.tube("seat-post", FR, STEEL, 0.4, [-0.285, 0.8, 0], [-0.348, 0.972, 0], 0.0125, 0.0125 - W), FR, "member")
    for d in B.tube_path("main-tube", FR, STEEL, 0.4, [[0.347, 0.8], [0.24, 0.52], [0.1, 0.38], [BB[0], BB[1]]], 0.12, 0.0165, 0.0165 - W):
        st(d, FR, "member")
    st(B.tube("head-tube", FR, STEEL, 0.4, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.0195, 0.0195 - W), FR, "member")
    st(B.tube("bb-shell", FR, STEEL, 0.4, [BB[0], BB[1], -0.033], [BB[0], BB[1], 0.033], 0.0175, 0.0175 - W), FR, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.tube(f"chainstay-{nm}", FR, STEEL, 0.4, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.0095, 0.0095 - W), FR, "member")
        st(B.tube(f"seatstay-{nm}", FR, STEEL, 0.4, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.0075, 0.0075 - W), FR, "member")
        st(B.tube(f"rack-rail-{nm}", FR, STEEL, 0.4, [-0.835, 0.8, sg * 0.078], [-0.305, 0.8, sg * 0.078], 0.0075, 0.0075 - W), FR, "member")
    st(B.tube("steerer", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, 0]), P.wld(ST, [-0.0, 0, 0]), 0.011, 0.011 - W), ST, "member")
    neck_w = [P.wld(ST, p) for p in ([0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0])]
    nk, _ = fillet([[p[0], p[1], 0.0] for p in neck_w], 0.05)
    st(B.sweep("stem-neck", ST, STEEL, 0.4, nk, 0.0105, step=0.03, capStart=False), ST, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.tube(f"fork-{nm}", ST, STEEL, 0.4, P.wld(ST, [0.14, 0, sg * 0.045]), P.wld(ST, [0.641, 0.0445, sg * 0.045]), 0.0078, 0.0078 - W), ST, "member")
        st(B.tube(f"handlebar-{nm}", ST, STEEL, 0.4, P.wld(ST, [-0.2145, -0.19755, 0]), P.wld(ST, [-0.216, -0.263, sg * 0.26]), 0.0135, 0.0135 - W), ST, "member")
    st(B.tube("fork-crown", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, -0.05]), P.wld(ST, [0.15, 0, 0.05]), 0.0125, 0.0125 - W), ST, "member")
    for tag, bone in (("r", WR), ("f", WF)):
        rr, ro_, ri_ = 0.297, 0.0125, 0.0105
        st(revolve(f"rim-{tag}", bone, ALLOY, 0.7, "z", [0, 0, 0], superpoly(0, rr, 0.0115, 0.0115, 4.0, 24)), bone, "member")
        for gi, (zf, lead) in enumerate(((0.03, 1), (0.03, -1), (-0.03, 1), (-0.03, -1))):
            a_h = math.radians(10 * gi)
            a_r = a_h + lead * math.radians(62)
            s = capsule(f"spoke-{tag}-{gi + 1}", bone, ALLOY, 0.6, [0.032 * math.cos(a_h), 0.032 * math.sin(a_h), zf], [0.292 * math.cos(a_r), 0.292 * math.sin(a_r), 0.0], 0.002, 0.002, repeat={"count": nspoke, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / nspoke), "origin": [0, 0, 0]})
            s["slender"] = True
            st(s, bone, "member")
    st(B.tube("crank-axle", CR, STEEL, 0.4, [BB[0], BB[1], -0.085], [BB[0], BB[1], 0.085], 0.0085, None), CR, "member")
    for sg, nm, z in ((1, "n", -0.07), (-1, "f", 0.07)):
        st(B.sweep(f"crank-arm-{nm}", CR, STEEL, 0.4, [[0.165 * k / 3 * sg, 0, z] for k in range(4)], 0.008, rx=0.0085, ry=lambda t: 0.011 - 0.003 * t, step=0, local=True), CR, "member")
    for nm, bone, z in (("n", PN, -0.12), ("f", PF, 0.12)):
        st(cyl(f"pedal-spindle-{nm}", bone, STEEL, 0.4, "z", [0, 0, z], 0.09, 0.0075), bone, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(capsule(f"stand-leg-{nm}", SD, STEEL, 0.4, [0.0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.0075, 0.0075), SD, "member")
    st(capsule("stand-bar", SD, STEEL, 0.4, [0.32, 0, -0.075], [0.32, 0, 0.075], 0.0075, 0.0075), SD, "member")
    st(ellipsoid("head-bearing", ST, CHROME, 0.5, [0.019, 0.019, 0.019], at=[0.01, 0.0, 0.0]), ST, "joint")
    st(ellipsoid("bb-bearing", CR, CHROME, 0.5, [0.017, 0.017, 0.03], at=[0, 0, 0]), CR, "joint")
    for tag, bone in (("r", WR), ("f", WF)):
        st(revolve(f"hub-joint-{tag}", bone, CHROME, 0.5, "z", [0, 0, 0], [[-0.045, 0], [-0.045, 0.011], [0.045, 0.011], [0.045, 0], [-0.045, 0]]), bone, "joint")
    hp = P.loc(ST, [0.56, 0.0, 0.0])
    h = 0.0075 if den == 50 else (0.015 if den == 100 else 0.03)
    st(dict(box("contact-pad-f", ST, "rubber", 0.1, [0.04, h, 0.037], at=[hp[0], hp[1] + h / 2, 0.0]), anchor="contact-f"), ST, "support")
    st(ellipsoid("mass-steer", ST, STEEL, 0.4, [0.02, 0.02, 0.02], at=[0.3161, -0.0086, 0.0]), ST, "mass")
    st(ellipsoid("mass-wheel-r", WR, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WR, "mass")
    st(ellipsoid("mass-wheel-f", WF, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WF, "mass")


def struct200(B):
    P = B.P
    st = B.st
    seat, _ = fillet([[BB[0], BB[1], 0], [-0.285, 0.8, 0], [-0.348, 0.972, 0]], 0.3)
    st(B.sweep("seat-tube", FR, STEEL, 0.4, seat, 0.0135, rx=0.0135, ry=0.0135, step=0, slender=True), FR, "member")
    pts, _ = fillet([[0.347, 0.8, 0], [0.24, 0.52, 0], [0.1, 0.38, 0], [BB[0], BB[1], 0]], 0.12)
    st(B.sweep("main-tube", FR, STEEL, 0.4, pts, 0.0165, step=0.0), FR, "member")
    st(B.tube("head-tube", FR, STEEL, 0.4, [0.347, 0.8, 0], [0.299, 0.932, 0], 0.02, 0.017), FR, "member")
    st(B.tube("bb-shell", FR, STEEL, 0.4, [BB[0], BB[1], -0.033], [BB[0], BB[1], 0.033], 0.0185, 0.0155), FR, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.cap(f"chainstay-{nm}", FR, STEEL, 0.4, [BB[0], BB[1], sg * 0.035], [HUBR[0], HUBR[1], sg * 0.055], 0.011, slender=True), FR, "member")
        st(B.cap(f"seatstay-{nm}", FR, STEEL, 0.4, [-0.285, 0.8, sg * 0.02], [HUBR[0], HUBR[1], sg * 0.055], 0.01, slender=True), FR, "member")
        st(B.cap(f"rack-rail-{nm}", FR, STEEL, 0.4, [-0.835, 0.8, sg * 0.078], [-0.305, 0.8, sg * 0.078], 0.0075, slender=True), FR, "member")
    st(B.tube("steerer", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, 0]), P.wld(ST, [-0.0, 0, 0]), 0.0155, 0.0125), ST, "member")
    nk, _ = fillet([[p[0], p[1], 0.0] for p in [P.wld(ST, q) for q in ([0.0, 0.0, 0.0], [-0.1095, 0.0145, 0.0], [-0.161, -0.025, 0.0], [-0.208, -0.115, 0.0], [-0.2135, -0.198, 0.0])]], 0.05)
    st(B.sweep("stem-neck", ST, STEEL, 0.4, nk, 0.0155, step=0.05, capStart=False), ST, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(B.cap(f"fork-{nm}", ST, STEEL, 0.4, P.wld(ST, [0.14, 0, sg * 0.045]), P.wld(ST, [0.641, 0.0445, sg * 0.045]), 0.0155), ST, "member")
        st(B.cap(f"handlebar-{nm}", ST, STEEL, 0.4, P.wld(ST, [-0.2145, -0.19755, 0]), P.wld(ST, [-0.216, -0.263, sg * 0.26]), 0.0155), ST, "member")
    st(B.cap("fork-crown", ST, STEEL, 0.4, P.wld(ST, [0.15, 0, -0.05]), P.wld(ST, [0.15, 0, 0.05]), 0.0155), ST, "member")
    for tag, bone in (("r", WR), ("f", WF)):
        st(revolve(f"rim-{tag}", bone, ALLOY, 0.7, "z", [0, 0, 0], superpoly(0, 0.297, 0.015, 0.015, 4.0, 20)), bone, "member")
        for gi, (zf, lead) in enumerate(((0.03, 1), (0.03, -1), (-0.03, 1), (-0.03, -1))):
            a_h = math.radians(10 * gi)
            a_r = a_h + lead * math.radians(62)
            s = capsule(f"spoke-{tag}-{gi + 1}", bone, ALLOY, 0.6, [0.032 * math.cos(a_h), 0.032 * math.sin(a_h), zf], [0.292 * math.cos(a_r), 0.292 * math.sin(a_r), 0.0], 0.002, 0.002, repeat={"count": 3, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / 3), "origin": [0, 0, 0]})
            s["slender"] = True
            st(s, bone, "member")
        st(revolve(f"hub-joint-{tag}", bone, CHROME, 0.5, "z", [0, 0, 0], [[-0.05, 0], [-0.05, 0.016], [0.05, 0.016], [0.05, 0], [-0.05, 0]]), bone, "joint")
    st(B.tube("crank-axle", CR, STEEL, 0.4, [BB[0], BB[1], -0.085], [BB[0], BB[1], 0.085], 0.015, None), CR, "member")
    st(cyl("pedal-spindle-n", PN, STEEL, 0.4, "z", [0, 0, -0.12], 0.03, 0.015), PN, "member")
    st(cyl("pedal-spindle-f", PF, STEEL, 0.4, "z", [0, 0, 0.12], 0.03, 0.015), PF, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(capsule(f"stand-leg-{nm}", SD, STEEL, 0.4, [0.0, 0, sg * 0.075], [0.32, 0, sg * 0.075], 0.015, 0.015), SD, "member")
    st(ellipsoid("head-bearing", ST, CHROME, 0.5, [0.02, 0.02, 0.02], at=[0.01, 0.0, 0.0]), ST, "joint")
    hp = P.loc(ST, [0.56, 0.0, 0.0])
    st(dict(box("contact-pad-f", ST, "rubber", 0.1, [0.04, 0.03, 0.037], at=[hp[0], hp[1] + 0.015, 0.0]), anchor="contact-f"), ST, "support")
    st(ellipsoid("mass-steer", ST, STEEL, 0.4, [0.02, 0.02, 0.02], at=[0.3161, -0.0086, 0.0]), ST, "mass")
    st(ellipsoid("mass-wheel-r", WR, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WR, "mass")
    st(ellipsoid("mass-wheel-f", WF, ALLOY, 0.4, [0.02, 0.02, 0.02], at=[0, 0, 0]), WF, "mass")


def regions_of(B):
    reg = {k: [] for k in ("frame-tubes", "steer-assembly", "mudguards", "tyres", "rims-spokes", "saddle", "drivetrain", "hub-brakes", "lamps")}
    for s in B.body:
        i = s["id"]
        if i.startswith(("seat", "main-tube", "head-tube", "bb-shell", "chainstay", "rack", "stand")):
            reg["frame-tubes"].append(i)
        elif i.startswith(("fork", "stem", "handlebar", "grip", "head-cup")):
            reg["steer-assembly"].append(i)
        elif i.startswith("fender"):
            reg["mudguards"].append(i)
        elif i.startswith("tyre"):
            reg["tyres"].append(i)
        elif i.startswith(("rim", "spoke")):
            reg["rims-spokes"].append(i)
        elif i == "saddle":
            reg["saddle"].append(i)
        elif i.startswith(("chain", "crank", "pedal")):
            reg["drivetrain"].append(i)
        elif i.startswith(("hub", "axle", "flange")):
            reg["hub-brakes"].append(i)
        elif i.endswith("lamp"):
            reg["lamps"].append(i)
        else:
            raise SystemExit("unbound " + i)
    return reg


def layers_of():
    a = asset_of(SLUG)
    return {r["id"]: [(l["id"], l["thicknessM"]) for l in r.get("layers", [])] for r in a["thermal"]["regions"]}


DEC = {
    200: (
        "1:200 models the bicycle as the drawing does: each wheel is one ring (tyre and rim merged, 70 mm radial) with a hub, the frame is a swept seat tube with post, one swept main tube, the head tube and two slender stay pairs, the saddle has three sections, the rack is one 30 mm slab, the chain case one extrude, handlebar bar and grip are one capsule per side, the stand legs are drawn 40 mm thick; fenders, head lamp, cranks, pedals and spokes are below the drawing's resolution and stay out of the body. A 30 mm rear-light box stands for the lamp.",
        "1:200 modelliert das Fahrrad wie die Zeichnung: jedes Rad ist ein Ring (Reifen und Felge verschmolzen, 70 mm radial) mit Nabe, der Rahmen besteht aus geschwungenem Sattelrohr mit Stütze, einem geschwungenen Hauptrohr, dem Steuerrohr und zwei schlanken Strebenpaaren, der Sattel hat drei Schnitte, der Gepäckträger ist eine 30-mm-Platte, der Kettenkasten ein Extrusionskörper, Lenkerstange und Griff sind je Seite eine Kapsel, die Ständerbeine sind wie gezeichnet 40 mm dick; Schutzbleche, Frontlampe, Kurbeln, Pedale und Speichen liegen unter der Auflösung der Zeichnung und bleiben aus dem Körper heraus. Ein 30-mm-Rücklichtkasten steht für die Lampe.",
        "Thermal: references for the body solids; the mudguards and the rims, which the drawing no longer shows, are real thermal solids (arched 30 mm channels over the wheel arcs and rim rings inside the wheel rings); only the sum of the exposed areas counts at this scale.",
        "Thermik: Verweise auf die Körper; die Schutzbleche und die Felgen, die die Zeichnung nicht mehr zeigt, sind echte Thermikkörper (30-mm-Bogenrinnen über den Radbögen und Felgenringe in den Radringen); in diesem Maßstab zählt nur die Summe der freien Flächen.",
        "Structure: simplified sections that respect the 30 mm feature: swept solid seat tube and main tube, hollow head tube and bottom-bracket shell, slender stays, rack rails and fork blades, a hollow steerer, solid stem neck and handlebar bars, solid rim rings with twelve slender spokes per wheel, merged crank axle and pedal spindles of 30 mm, stand legs, head bearing and hub joints, the front contact patch as 30 mm support and masses as at 1:50.",
        "Tragwerk: vereinfachte Querschnitte im Rahmen des 30-mm-Merkmals: geschwungenes massives Sattel- und Hauptrohr, hohles Steuerrohr und Tretlagerhülse, schlanke Streben, Gepäckträgerschienen und Gabelholme, hohler Gabelschaft, massiver Vorbauhals und Lenkerstangen, massive Felgenringe mit zwölf schlanken Speichen je Rad, zusammengefasste Kurbelachse und Pedalachsen von 30 mm, Ständerbeine, Steuerlager und Nabengelenke, die vordere Aufstandsfläche als 30-mm-Auflager und Massen wie bei 1:50.",
    ),
    50: (
        "1:50 is the detailed model of the Dutch step-through bicycle, rebuilt from the drawings of this scale: hollow-tube frame on its real centre lines (seat tube with post, one swept main tube from the head tube to the bottom bracket, head tube with cups, chain and seat stays splayed to the rear axle), a lofted saddle with its drawn plan and elevation, oval fork blades with a crown, the swan-neck stem and swept handlebar with grips, tyre and rim as revolves about the wheel axis with 36 laced spokes, drum hubs with flanges and axle nuts, the chain case, mudguards as shallow arched channels, rack tray, crank arms with pedals and the folded rear stand.",
        "1:50 ist das ausführliche Modell des holländischen Tiefeinsteigers, neu aus den Zeichnungen dieses Maßstabs gebaut: Rahmen aus Hohlrohren auf ihren wirklichen Mittellinien (Sattelrohr mit Stütze, ein geschwungenes Hauptrohr vom Steuerrohr zum Tretlager, Steuerrohr mit Schalen, zum Hinterrad gespreizte Kettenstreben und Sitzstreben), ein gelofteter Sattel nach gezeichnetem Grund- und Aufriss, ovale Gabelholme mit Krone, Schwanenhalsvorbau und geschwungener Lenker mit Griffen, Reifen und Felge als Rotationskörper um die Radachse mit 36 gekreuzten Speichen, Trommelnaben mit Flanschen und Achsmuttern, Kettenkasten, Schutzbleche als flache Bogenrinnen, Gepäckträgerplatte, Kurbelarme mit Pedalen und der hochgeklappte Hinterbauständer.",
        "Thermal: every region has its own solids; the outer layer of each solid is a reference to the body solid, regions with a steel wall and a paint film (frame tubes, steer assembly, mudguards, drivetrain) additionally carry the wall as a nested shell inset by the paint film. Hub, flanges and axle form the hub-brake region, tyres and rims keep their single layer.",
        "Thermik: jede Region hat eigene Körper; die äußere Schicht jedes Körpers verweist auf den Körper selbst, Regionen mit Stahlwand und Lackfilm (Rahmenrohre, Lenkeinheit, Schutzbleche, Antrieb) tragen zusätzlich die Wand als verschachtelte, um den Lackfilm eingerückte Schale. Nabe, Flansche und Achse bilden die Nabenbremsen-Region, Reifen und Felgen behalten ihre einzige Schicht.",
        "Structure: every eligible bone has a hollow 1.5 mm steel member on its real centre line (seat tube and post, main tube as straight pieces and half-ring bends, head tube, bottom-bracket shell, both stay pairs, rack rails, steerer, stem neck, fork blades and crown, both handlebar bars, wheel rims as hollow torus halves with 36 slender spokes, crank axle and arms, pedal spindles, stand legs and foot bar); joints at head bearing, bottom bracket, hubs, pedals and stand hinge; the front tyre contact patch is the support; compact masses sit at the centres of mass of the steer assembly and the two wheels.",
        "Tragwerk: jeder tragende Knochen hat ein hohles 1,5-mm-Stahlglied auf seiner wirklichen Mittellinie (Sattelrohr und Stütze, Hauptrohr als gerade Stücke und Halbringbögen, Steuerrohr, Tretlagerhülse, beide Streben, Gepäckträgerschienen, Gabelschaft, Vorbauhals, Gabelholme und Krone, beide Lenkerhälften, Felgen als hohle Torushälften mit 36 schlanken Speichen, Kurbelachse und Kurbelarme, Pedalachsen, Ständerbeine und Fußbügel); Gelenke an Steuersatz, Tretlager, Naben, Pedalen und Ständerscharnier; Auflager ist die Aufstandsfläche des Vorderreifens; kompakte Massen sitzen in den Schwerpunkten von Lenkeinheit und beiden Rädern.",
    ),
    100: (
        "1:100 merges the fine model against the drawings of this scale: tyre and rim stay two revolves per wheel without spokes (the 2D draws one ring), seat tube and post are one swept tube, head cups, lug, flanges and rack rim are gone, the rack is one 20 mm tray, the saddle has five sections, fenders are 15 mm arched channels, crank arms and fork blades use four and five sections; frame tubes follow the 30 mm strokes of the drawing and are slender.",
        "1:100 fasst das feine Modell gegen die Zeichnungen dieses Maßstabs zusammen: Reifen und Felge bleiben zwei Rotationskörper je Rad ohne Speichen (die 2D-Zeichnung zeigt einen Ring), Sattelrohr und Stütze sind ein geschwungenes Rohr, Steuersatzschalen, Muffe, Flansche und Gepäckträgerrand entfallen, der Träger ist eine 20-mm-Platte, der Sattel hat fünf Schnitte, Schutzbleche sind 15-mm-Bogenrinnen, Kurbelarme und Gabelholme nutzen vier und fünf Schnitte; die Rahmenrohre folgen den 30-mm-Strichen der Zeichnung und sind schlank.",
        "Thermal: one reference per body solid in its region; the layer shells of 1:50 are not modelled at this scale (a 1.3 mm wall is far below the 15 mm feature), the region coverage is complete.",
        "Thermik: ein Verweis je Körper in seiner Region; die Schichtschalen von 1:50 entfallen in diesem Maßstab (eine 1,3-mm-Wand liegt weit unter dem 15-mm-Merkmal), die Regionsabdeckung ist vollständig.",
        "Structure: the same load path as 1:50 with 2 mm hollow walls, joints merged to head bearing, bottom bracket and two hub joints, 20 slender spokes per wheel instead of 36, four-section crank arms; the front contact patch is the support (15 mm pad); masses as at 1:50.",
        "Tragwerk: derselbe Lastpfad wie bei 1:50 mit 2-mm-Hohlwänden, Gelenke zu Steuersatz, Tretlager und zwei Nabengelenken zusammengefasst, 20 schlanke Speichen je Rad statt 36, vierschnittige Kurbelarme; Auflager ist die vordere Aufstandsfläche (15-mm-Platte); Massen wie bei 1:50.",
    ),
}


def decisions(B, den):
    d = DEC[den]
    for k in range(0, len(d), 2):
        B.dec(d[k], d[k + 1])


def main(den):
    B = {50: build50, 100: build100, 200: build200}[den]()
    {50: struct50, 100: lambda x: struct_coarse(x, 100), 200: struct200}[den](B)
    thermal_from(B, regions_of(B), layers_of(), layers_of() if den == 50 else {}, skip=())
    if den == 200:
        build200_thermal_extras(B)
    decisions(B, den)
    print(write_scale(SLUG, den, B))
    print(run_check(SLUG, den))


if __name__ == "__main__":
    main(int(sys.argv[1]) if len(sys.argv) > 1 and sys.argv[1].isdigit() else 50)
