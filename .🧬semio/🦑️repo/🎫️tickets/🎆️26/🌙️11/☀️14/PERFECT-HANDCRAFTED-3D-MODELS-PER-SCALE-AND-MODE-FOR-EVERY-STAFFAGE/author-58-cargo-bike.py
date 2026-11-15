import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_58_lib import *

SLUG = "cargo-bike"
FR, FK, BR, BX, BT, MT, CK, SD, WR, WF, CX, CY, CT = "frame", "fork", "bar", "box", "battery", "motor", "crank", "stand", "wheel-r", "wheel-f", "crate-x", "crate-y", "crate"
AL, PLY, RUB, RIM, DISC, MOT, BAT, PLA = "aluminium", "plywood", "rubber", "alloy-rim", "steel-disc", "motor-case", "battery-case", "plastic"
P = Pose(SLUG)
FANG = math.degrees(frame_angle(P, FK))
BOXWALL = [[0, 0], [0.83, 0], [0.84, 0.045], [0.865, 0.095], [0.9, 0.135], [0.95, 0.158], [1.0, 0.17], [1.0, 0.65], [0, 0.65]]
BRK = "brakes"


def _bez(p0, c1, c2, p3, n=8):
    out = []
    for k in range(1, n + 1):
        u = k / n
        out.append([round((1 - u) ** 3 * p0[i] + 3 * (1 - u) ** 2 * u * c1[i] + 3 * (1 - u) * u * u * c2[i] + u ** 3 * p3[i], 4) for i in range(2)])
    return out


def thick_line(pts, t):
    inner = []
    n = len(pts)
    for i in range(n):
        a, c = pts[max(0, i - 1)], pts[min(n - 1, i + 1)]
        dx, dy = c[0] - a[0], c[1] - a[1]
        L = math.hypot(dx, dy)
        inner.append([round(pts[i][0] - dy / L * t, 5), round(pts[i][1] + dx / L * t, 5)])
    return [list(p) for p in pts] + inner[::-1]


BOXWALL2 = [[0, 0], [0.72, 0], [0.72, 0.1]] + _bez([0.72, 0.1], [0.74, 0.25], [0.85, 0.32], [1.0, 0.34]) + [[1.0, 0.65], [0, 0.65]]
BOXNOSE2 = [[0.72, 0], [0.72, 0.1]] + _bez([0.72, 0.1], [0.74, 0.25], [0.85, 0.32], [1.0, 0.34])


def world_box(id, bone, mat, tone, size, c_world, role="body", anchor=None):
    c = P.loc(bone, [c_world[0], c_world[1], 0])
    d = box(id, bone, mat, tone, size, role=role)
    d["transform"] = {"translation": vec([c[0], c[1], c_world[2] if len(c_world) > 2 else 0]), "rotation": qaxis([0, 0, 1], -math.degrees(frame_angle(P, bone)))}
    if anchor:
        d["anchor"] = anchor
    return d


def saddle(B, fine):
    xs = [0.18, 0.2, 0.24, 0.29, 0.34, 0.38, 0.41, 0.43] if fine else [0.18, 0.3, 0.43]
    hw = {0.18: 0.016, 0.2: 0.021, 0.24: 0.03, 0.29: 0.06, 0.34: 0.086, 0.38: 0.077, 0.41: 0.04, 0.43: 0.022, 0.3: 0.09}
    top = {x: 1.075 + 0.014 * min(1, (x - 0.18) / 0.12) - (0.012 if x > 0.4 else 0) for x in xs}
    bot = {x: top[x] - (0.034 if x < 0.4 else 0.026) for x in xs}
    secs = [sec([x, (bot[x] + top[x]) / 2, 0], hw[x], (top[x] - bot[x]) / 2, exponent=2.6, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])) for x in xs]
    return loft("saddle", FR, PLA, 0.15, secs)


def wheels50(B):
    b = B.b
    for tag, bone, tout, tin, rin, hr, ax, dz in (("r", WR, 0.33, 0.2761, 0.2761, 0.03, 0.06, 0.04), ("f", WF, 0.255, 0.201, 0.201, 0.03, 0.065, 0.045)):
        w = wheel_solids(B, bone, tag, hr, tout, tin, rin, 0.03, 0.015, 0.03, flange_z=0.028, flange_r=0.036, axle_hw=ax, axle_r=0.0095, nut_r=0.014, mat_tyre=RUB, mat_rim=RIM, mat_hub=RIM)
        w["rim"] = revolve(f"rim-{tag}", bone, RIM, 0.78, "z", [0, 0, 0], superpoly(0, tin + 0.0038, 0.015, 0.0038, 4.0, 28), role="body")
        b(w["tyre"]); b(w["rim"]); b(w["hub"]); b(w["axle"])
        for f in w["flanges"]:
            b(f)
        for s in w["spokes"]:
            b(s)
        b(annulus(f"rotor-{tag}", bone, DISC, 0.45, 0.052, 0.08, dz + 0.0, dz + 0.0076, role="body"))
        b(annulus(f"rotor-carrier-{tag}", bone, DISC, 0.45, 0.03, 0.052, dz - 0.0124, dz - 0.0048, role="detail"))
        for k in range(6):
            pass


def build50():
    B = Bike(50, SLUG)
    b = B.b
    wheels50(B)
    # frame
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"chainstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.55, 0.32, sg * 0.03], 0.012, 0.014))
        b(cl(f"seatstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.34, 0.86, sg * 0.02], 0.011, 0.01))
    b(oval("seat-tube", FR, AL, 0.35, [0.55, 0.32, 0], [0.30, 0.98, 0], 0.02, 0.016, e=2.4))
    b(cl("seat-post", FR, AL, 0.5, [0.3, 0.98, 0], [0.285, 1.04, 0], 0.01, role="detail"))
    b(tubel("seat-collar", FR, AL, 0.5, [0.2995, 0.97, 0], [0.2985, 0.99, 0], 0.017, role="detail"))
    b(saddle(B, True))
    b(oval("main-beam", FR, AL, 0.35, [0.55, 0.32, 0], [1.78, 0.40, 0], 0.02, 0.03, e=3.0))
    b(tubel("head-riser", FR, AL, 0.35, [1.78, 0.40, 0], [1.85, 0.62, 0], 0.025))
    b(tubel("head-cup", FR, RIM, 0.5, [1.8445, 0.598, 0], [1.8525, 0.628, 0], 0.028, role="detail"))
    b(cl("steer-rod", FR, AL, 0.3, [0.74, 0.62, 0], [1.85, 0.62, 0], 0.0135, role="detail"))
    for x in (0.74, 1.85):
        b(ellipsoid(f"steer-ball-{int(x * 100)}", FR, AL, 0.5, [0.02, 0.02, 0.02], at=[x, 0.62, 0], role="detail"))
    # rack
    ang = math.degrees(math.atan2(0.08, 0.57))
    rim_o = [[-0.15, -0.2875], [-0.15, 0.2875], [0.15, 0.2875], [0.15, -0.2875]]
    rim_i = [[-0.136, -0.2735], [-0.136, 0.2735], [0.136, 0.2735], [0.136, -0.2735]]
    deck = ext("rack-deck", FR, AL, 0.3, "zx", rim_o, 0.0, 0.008, role="detail")
    rot_place(deck, [0.015, 0.8135, 0], ang)
    b(deck)
    rimp = ext("rack-rim", FR, AL, 0.3, "zx", rim_o, 0.008, 0.02, holes=[rim_i], role="detail")
    rot_place(rimp, [0.015, 0.8135, 0], ang)
    b(rimp)
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"rack-strut-{nm}", FR, AL, 0.3, [-0.02, 0.33, sg * 0.045], [-0.22, 0.79, sg * 0.14], 0.008, role="detail", slender=True))
    b(box("rear-light", FR, PLA, 0.3, [0.04, 0.06, 0.06], at=[-0.28, 0.81, 0], r=0.008, role="detail"))
    b(B.fender("fender-r", FR, PLA, 0.4, (0.006, 0.37), 0.343, 0.014, 0.04, 0.008, 25.0, 150.5))
    # motor, drivetrain, battery
    b(box("motor-housing", MT, MOT, 0.3, [0.14, 0.14, 0.14], r=0.03, role="detail"))
    b(tubel("crank-axle", CK, AL, 0.5, [0, 0, -0.11], [0, 0, 0.11], 0.0095, role="detail"))
    b(annulus("chainring", CK, RIM, 0.6, 0.0, 0.095, -0.0765, -0.068, role="detail"))
    for sg, nm, z in ((1, "n", -0.1), (-1, "f", 0.1)):
        b(cl(f"crank-arm-{nm}", CK, AL, 0.4, [0, 0, z], [0.17 * sg, 0, z], 0.014, role="detail"))
        b(box(f"pedal-{nm}", CK, PLA, 0.2, [0.1, 0.024, 0.09], at=[0.17 * sg, 0, z * 1.45], r=0.006, role="detail"))
        b(cl(f"pedal-spindle-{nm}", CK, AL, 0.5, [0.17 * sg, 0, z], [0.17 * sg, 0, z * 1.45], 0.007, role="detail"))
    bat = ext("battery-case", BT, BAT, 0.2, "xy", [[0.107, -0.19], [0.167, -0.19], [0.023, 0.19], [-0.037, 0.19]], -0.06, 0.06, role="detail")
    b(bat)
    # stand
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"stand-leg-{nm}", SD, AL, 0.3, [0, 0, sg * 0.12], [0.3433, 0, sg * 0.12], 0.012, 0.01, role="detail"))
    b(box("stand-foot", SD, PLA, 0.25, [0.06, 0.114, 0.3], at=[0.3303, 0, 0], r=0.012, role="detail"))
    b(tubel("stand-hinge", SD, AL, 0.5, [0, 0, -0.14], [0, 0, 0.14], 0.012, role="detail"))
    # steering column and handlebar
    for sg, nm in ((1, "l"), (-1, "r")):
        b(oval(f"fork-leg-{nm}", FK, AL, 0.35, [0.0, 0.0, sg * 0.06], [0.4424, 0, sg * 0.06], 0.012, 0.018, e=2.4, lat=(0, 0, 1)))
    b(cl("fork-crown", FK, AL, 0.35, [0.0, 0.0, -0.07], [0.0, 0.0, 0.07], 0.02))
    b(cl("steerer", FK, AL, 0.35, [0.0, 0.0, 0.0], [-0.07, 0.0, 0.0], 0.016))
    lamp = ellipsoid("head-lamp", FK, PLA, 0.85, [0.04, 0.03, 0.04], at=[0.05653, 0.08249, 0], role="detail")
    b(lamp)
    b(cl("head-lamp-bracket", FK, PLA, 0.6, [0.01696, 0.02475, 0], [0.04522, 0.06599, 0], 0.008, role="detail"))
    b(B.fender("fender-f", FK, PLA, 0.4, (2.104, 0.255), 0.268, 0.014, 0.04, 0.008, 28.3, 149.3))
    b(cl("stem", BR, AL, 0.35, [-0.013, 0, 0], [0.44, 0, 0], 0.0125))
    cross = [[0.44, 0.0, 0.0]]
    for sg, nm in ((1, "l"), (-1, "r")):
        pts, _ = fillet([[0.44, 0.0, 0.0], [0.44, 0.0, sg * 0.19], [0.5, 0.137, sg * 0.255], [0.5, 0.137, sg * 0.255 + 0.0001]], 0.03)
        pts = [[0.44, 0.0, 0.0], [0.44, 0.0, sg * 0.195], [0.45, 0.03, sg * 0.225], [0.47, 0.09, sg * 0.245], [0.488, 0.137, sg * 0.255]]
        b(path_sweep(f"handlebar-{nm}", BR, PLA, 0.3, pts, 0.0125, lat=(1, 0, 0), step=0.02, caps=True))
        b(cl(f"grip-{nm}", BR, PLA, 0.12, [0.488, 0.137, sg * 0.25], [0.488, 0.137, sg * 0.305], 0.02))
        b(cl(f"brake-lever-{nm}", BR, AL, 0.5, [0.43, 0.028, sg * 0.235], [0.47, 0.1, sg * 0.255], 0.005, role="detail"))
    b(oval("stem-cross", BR, AL, 0.35, [0.44, 0, -0.195], [0.44, 0, 0.195], 0.0125, 0.0125, lat=(1, 0, 0)))
    b(box("cockpit-plate", BR, PLA, 0.3, [0.092, 0.02, 0.6], at=[0.4575, 0.08, 0.0], r=0.006))
    # cargo box
    t = 0.012
    side = lambda sg: ext(f"box-wall-{'l' if sg > 0 else 'r'}", BX, PLY, 0.7, "xy", BOXWALL, 0.325 - t if sg > 0 else -0.325, 0.325 if sg > 0 else -0.325 + t)
    b(side(1)); b(side(-1))
    b(box("box-floor", BX, PLY, 0.7, [0.83, t, 0.626], at=[0.415, t / 2, 0]))
    b(box("box-rear-wall", BX, PLY, 0.7, [t, 0.65, 0.626], at=[t / 2, 0.325, 0]))
    b(box("box-front-wall", BX, PLY, 0.7, [t, 0.48, 0.626], at=[1.0 - t / 2, 0.41, 0]))
    nose = ext("box-nose", BX, PLY, 0.7, "xy", [[0.83, 0], [0.84, 0.045], [0.865, 0.095], [0.9, 0.135], [0.95, 0.158], [1.0, 0.17], [1.0, 0.17 - t], [0.95, 0.158 - t], [0.9, 0.135 - t], [0.865, 0.095 - t], [0.84, 0.045 - t]], -0.313, 0.313)
    b(nose)
    for z in (0.2, 0.4):
        b(dict(box(f"box-batten-{int(z * 10)}", BX, PLY, 0.6, [0.99, 0.02, 0.005], at=[0.5, z, 0.3275], role="texture", repeat={"count": 2, "translation": [0, 0, -0.655]}), texture=True))
    # crates
    b(box("crate-lower", CT, PLA, 0.4, [0.5, 0.28, 0.52], at=[0.25, 0.14, 0], r=0.02))
    b(box("crate-upper", CT, PLA, 0.4, [0.46, 0.28, 0.5], at=[0.25, 0.42, 0], r=0.02))
    return B



def wheel_ring(B, tag, bone, tout, tin, hw, rim=None):
    b = B.b
    b(revolve(f"tyre-{tag}", bone, RUB, 0.1, "z", [0, 0, 0], superpoly(0, (tout + tin) / 2, hw, (tout - tin) / 2, 3.0, 24)))
    if rim:
        b(revolve(f"rim-{tag}", bone, RIM, 0.78, "z", [0, 0, 0], superpoly(0, (rim[0] + rim[1]) / 2, 0.0175, (rim[1] - rim[0]) / 2, 4.0, 20)))


def frame_common(B, den):
    b = B.b
    k = 1 if den == 100 else 2
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"chainstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.55, 0.32, sg * 0.03], 0.0065 * k, slender=True))
        b(cl(f"seatstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.34, 0.86, sg * 0.02], 0.0065 * k, slender=True))
    b(oval("seat-tube", FR, AL, 0.35, [0.55, 0.32, 0], [0.285, 1.04, 0], 0.0175, 0.0075 * k, e=2.4, slender=(k == 1)))
    b(oval("main-beam", FR, AL, 0.35, [0.55, 0.32, 0], [1.66, 0.39, 0], 0.02, 0.015 * k, e=3.0, slender=True if k == 1 else False))
    b(cl("steer-rod", FR, AL, 0.3, [0.74, 0.62, 0], [1.85, 0.62, 0], 0.0095 if k == 1 else 0.0135, slender=True))


def build100():
    B = Bike(100, SLUG)
    b = B.b
    for tag, bone, tout, tin, dz in (("r", WR, 0.33, 0.2761, 0.04), ("f", WF, 0.255, 0.201, 0.045)):
        wheel_ring(B, tag, bone, tout, tin, 0.03, rim=(0.2575, 0.2725) if tag == "r" else (0.1775, 0.1925))
        b(revolve(f"hub-{tag}", bone, RIM, 0.7, "z", [0, 0, 0], [[-0.03, 0], [-0.03, 0.03], [0.03, 0.03], [0.03, 0], [-0.03, 0]]))
        b(annulus(f"rotor-{tag}", bone, DISC, 0.45, 0.0, 0.08, dz - 0.0075, dz + 0.0075, role="detail"))
    frame_common(B, 100)
    b(saddle(B, False))
    b(tubel("head-riser", FR, AL, 0.35, [1.66, 0.39, 0], [1.85, 0.62, 0], 0.032))
    ang = math.degrees(math.atan2(0.08, 0.57))
    b(rot_place(ext("rack-deck", FR, AL, 0.3, "zx", [[-0.15, -0.2875], [-0.15, 0.2875], [0.15, 0.2875], [0.15, -0.2875]], 0.0, 0.02, role="detail"), [0.015, 0.8035, 0], ang))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"rack-strut-{nm}", FR, AL, 0.3, [-0.02, 0.33, sg * 0.045], [-0.22, 0.79, sg * 0.14], 0.008, role="detail", slender=True))
    b(box("rear-light", FR, PLA, 0.3, [0.04, 0.06, 0.06], at=[-0.28, 0.81, 0], r=0.008, role="detail"))
    b(B.fender("fender-r", FR, PLA, 0.4, (0.006, 0.37), 0.3915, 0.014, 0.04, 0.015, 25.3, 147.0))
    b(B.fender("fender-f", FK, PLA, 0.4, (2.104, 0.255), 0.3235, 0.014, 0.04, 0.015, 33.0, 148.9))
    b(box("motor-housing", MT, MOT, 0.3, [0.14, 0.14, 0.14], r=0.03, role="detail"))
    b(tubel("crank-axle", CK, AL, 0.5, [0, 0, -0.11], [0, 0, 0.11], 0.0095 + 0.003, role="detail"))
    for sg, nm, z in ((1, "n", -0.1), (-1, "f", 0.1)):
        b(cl(f"crank-arm-{nm}", CK, AL, 0.4, [0, 0, z], [0.17 * sg, 0, z], 0.015, role="detail"))
        b(box(f"pedal-{nm}", CK, PLA, 0.2, [0.1, 0.024, 0.09], at=[0.17 * sg, 0, z * 1.45], r=0.006, role="detail"))
    b(box("battery-case", BT, BAT, 0.2, [0.06, 0.32, 0.12], at=[0.03, 0.03, 0], r=0.005, role="detail"))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"stand-leg-{nm}", SD, AL, 0.3, [0, 0, sg * 0.12], [0.3433, 0, sg * 0.12], 0.012, 0.01, role="detail"))
    b(cl("stand-hinge", SD, AL, 0.5, [0, 0, -0.14], [0, 0, 0.14], 0.012, role="detail"))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(oval(f"fork-leg-{nm}", FK, AL, 0.35, [0.0, 0.0, sg * 0.06], [0.4424, 0, sg * 0.06], 0.01, 0.0075, e=2.4, slender=True))
    b(cl("fork-crown", FK, AL, 0.35, [0.0, 0.0, -0.07], [0.0, 0.0, 0.07], 0.02))
    b(ellipsoid("head-lamp", FK, PLA, 0.85, [0.04, 0.03, 0.04], at=[0.05653, 0.08249, 0], role="detail"))
    b(cl("stem", BR, AL, 0.35, [-0.013, 0, 0], [0.44, 0, 0], 0.0065, slender=True))
    b(oval("stem-cross", BR, AL, 0.35, [0.44, 0, -0.195], [0.44, 0, 0.195], 0.0125, 0.0125, lat=(1, 0, 0)))
    b(box("cockpit-plate", BR, PLA, 0.3, [0.092, 0.02, 0.6], at=[0.4575, 0.08, 0.0], r=0.006))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(path_sweep(f"handlebar-{nm}", BR, PLA, 0.3, [[0.44, 0.0, sg * 0.195], [0.48, 0.09, sg * 0.245], [0.5, 0.137, sg * 0.255]], 0.0125, lat=(1, 0, 0), step=0, caps=True))
        b(cl(f"grip-{nm}", BR, PLA, 0.12, [0.488, 0.137, sg * 0.25], [0.488, 0.137, sg * 0.305], 0.02))
    t = 0.015
    box_common(B, t, True)
    b(box("crate-lower", CT, PLA, 0.4, [0.45, 0.22, 0.52], at=[0.225, 0.17, 0], r=0.02))
    b(box("crate-upper", CT, PLA, 0.4, [0.42, 0.2, 0.52], at=[0.23, 0.44, 0], r=0.02))
    return B


def box_common(B, t, battens):
    b = B.b
    b(ext("box-wall-l", BX, PLY, 0.7, "xy", BOXWALL2, 0.325 - t, 0.325))
    b(ext("box-wall-r", BX, PLY, 0.7, "xy", BOXWALL2, -0.325, -0.325 + t))
    b(box("box-floor", BX, PLY, 0.7, [0.72, t, 0.65 - 2 * t], at=[0.36, t / 2, 0]))
    b(box("box-rear-wall", BX, PLY, 0.7, [t, 0.65, 0.65 - 2 * t], at=[t / 2, 0.325, 0]))
    b(box("box-front-wall", BX, PLY, 0.7, [t, 0.31, 0.65 - 2 * t], at=[1.0 - t / 2, 0.495, 0]))
    b(ext("box-nose", BX, PLY, 0.7, "xy", thick_line(BOXNOSE2, t), -0.325 + t, 0.325 - t))
    if battens:
        for z in (0.2, 0.4):
            b(dict(box(f"box-batten-{int(z * 10)}", BX, PLY, 0.6, [0.99, 0.02, 0.005 * (2 if B.den > 50 else 1)], at=[0.5, z, 0.3275], role="texture", repeat={"count": 2, "translation": [0, 0, -0.655]}), texture=True))


def build200():
    B = Bike(200, SLUG)
    b = B.b
    for tag, bone, tout, tin, dz in (("r", WR, 0.33, 0.2761, 0.04), ("f", WF, 0.255, 0.201, 0.045)):
        wheel_ring(B, tag, bone, tout, tin, 0.035 if tag == "r" else 0.04, rim=(0.2461, 0.2761) if tag == "r" else (0.171, 0.201))
        b(annulus(f"rotor-{tag}", bone, DISC, 0.45, 0.0, 0.08, dz - 0.015, dz + 0.015, role="detail"))
    frame_common(B, 200)
    b(saddle(B, False))
    ang = math.degrees(math.atan2(0.08, 0.57))
    b(rot_place(ext("rack-deck", FR, AL, 0.3, "zx", [[-0.15, -0.2875], [-0.15, 0.2875], [0.15, 0.2875], [0.15, -0.2875]], 0.0, 0.03, role="detail"), [0.015, 0.7985, 0], ang))
    b(B.fender("fender-r", FR, PLA, 0.4, (0.006, 0.37), 0.399, 0.014, 0.04, 0.03, 25.3, 147.0))
    b(box("motor-housing", MT, MOT, 0.3, [0.14, 0.14, 0.14], r=0.03, role="detail"))
    b(box("battery-case", BT, BAT, 0.2, [0.06, 0.32, 0.12], at=[0.03, 0.03, 0], r=0.005, role="detail"))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"stand-leg-{nm}", SD, AL, 0.3, [0, 0, sg * 0.12], [0.3433, 0, sg * 0.12], 0.015, role="detail"))
        b(oval(f"fork-leg-{nm}", FK, AL, 0.35, [0.0, 0.0, sg * 0.06], [0.4424, 0, sg * 0.06], 0.015, 0.018, e=2.4))
    b(cl("fork-crown", FK, AL, 0.35, [0.0, 0.0, -0.07], [0.0, 0.0, 0.07], 0.02))
    b(cl("stem", BR, AL, 0.35, [-0.013, 0, 0], [0.44, 0, 0], 0.011, slender=True))
    b(dict(ext("bars-envelope", BR, PLA, 0.3, "xy", [[0.42, -0.04], [0.455, -0.04], [0.5, 0.16], [0.465, 0.16]], -0.3, 0.3), envelope=True))
    b(box("stand-web", SD, AL, 0.3, [0.3433, 0.03, 0.26], at=[0.17, 0, 0], r=0.01, role="detail"))
    box_common(B, 0.03, True)
    b(box("crate-lower", CT, PLA, 0.4, [0.45, 0.22, 0.52], at=[0.225, 0.17, 0], r=0.03))
    b(box("crate-upper", CT, PLA, 0.4, [0.42, 0.2, 0.52], at=[0.23, 0.44, 0], r=0.03))
    return B


def build500():
    B = Bike(500, SLUG)
    b = B.b
    for tag, bone, tout, hw in (("r", WR, 0.33, 0.04), ("f", WF, 0.255, 0.04)):
        b(revolve(f"tyre-{tag}", bone, RUB, 0.1, "z", [0, 0, 0], [[-hw, 0], [-hw, tout], [hw, tout], [hw, 0], [-hw, 0]]))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"chainstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.55, 0.32, sg * 0.03], 0.0375))
        b(cl(f"seatstay-{nm}", FR, AL, 0.35, [0, 0.33, sg * 0.06], [0.34, 0.86, sg * 0.02], 0.0375))
    b(oval("seat-tube", FR, AL, 0.35, [0.55, 0.32, 0], [0.285, 1.07, 0], 0.0375, 0.0375, e=2.4))
    b(oval("main-beam", FR, AL, 0.35, [0.55, 0.32, 0], [1.66, 0.39, 0], 0.0375, 0.0375, e=3.0))
    ang = math.degrees(math.atan2(0.08, 0.57))
    b(rot_place(ext("rack-deck", FR, AL, 0.3, "zx", [[-0.12, -0.27], [0.12, -0.27], [0.12, 0.3], [0.03, 0.45], [-0.03, 0.45], [-0.12, 0.3]], 0.0, 0.075, role="detail"), [0.015, 0.7925, 0], ang))
    b(cl("steer-rod", FR, AL, 0.3, [0.74, 0.62, 0], [1.85, 0.62, 0], 0.0275, slender=True))
    b(cl("head-riser", FR, AL, 0.35, [1.78, 0.40, 0], [1.85, 0.62, 0], 0.0375))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"fork-leg-{nm}", FK, AL, 0.35, [0.0, 0.0, sg * 0.06], [0.4424, 0, sg * 0.06], 0.0375))
    b(dict(ext("bars-envelope", BR, PLA, 0.3, "xy", [[0, 0.033], [0.421, 0.031], [0.471, 0.155], [0.529, 0.125], [0.439, -0.031], [0, -0.032]], -0.3, 0.3), envelope=True))
    b(box("stand-web", SD, AL, 0.3, [0.3433, 0.075, 0.26], at=[0.17, 0, 0], r=0.02, role="detail"))
    t = 0.075
    b(ext("box-wall-l", BX, PLY, 0.7, "xy", BOXWALL2, 0.325 - t, 0.325))
    b(ext("box-wall-r", BX, PLY, 0.7, "xy", BOXWALL2, -0.325, -0.325 + t))
    b(box("box-floor", BX, PLY, 0.7, [0.72, t, 0.65 - 2 * t], at=[0.36, t / 2, 0]))
    b(box("box-rear-wall", BX, PLY, 0.7, [t, 0.65, 0.65 - 2 * t], at=[t / 2, 0.325, 0]))
    b(box("box-front-wall", BX, PLY, 0.7, [t, 0.31, 0.65 - 2 * t], at=[1.0 - t / 2, 0.495, 0]))
    b(ext("box-nose", BX, PLY, 0.7, "xy", thick_line(BOXNOSE2, t), -0.325 + t, 0.325 - t))
    b(box("crate", CT, PLA, 0.4, [0.5, 0.56, 0.52], at=[0.25, 0.28, 0], r=0.05))
    return B


def thermal500(B):
    th = B.th
    for tag, bone, tout, tin in (("r", WR, 0.33, 0.2761), ("f", WF, 0.255, 0.201)):
        th(revolve(f"th-tyre-{tag}", bone, RUB, 0.1, "z", [0, 0, 0], superpoly(0, (tout + tin) / 2, 0.04, (tout - tin) / 2, 3.0, 24)), "tyres")
    th(box("th-saddle", FR, PLA, 0.15, [0.25, 0.075, 0.17], at=[0.3, 1.09, 0], r=0.02), "touch-parts")
    th(box("th-battery-case", BT, BAT, 0.2, [0.075, 0.32, 0.12], at=[0.03, 0.03, 0], r=0.005), "battery-pack")
    th(box("th-motor-housing", MT, MOT, 0.3, [0.14, 0.14, 0.14], r=0.03), "drive-motor")
    th(annulus("th-rotor-f", WF, DISC, 0.45, 0.0, 0.08, 0.045 - 0.0375, 0.045 + 0.0375, role="detail"), "brakes")
    th(B.fender("th-fender-r", FR, PLA, 0.4, (0.006, 0.37), 0.34, 0.075, 0.04, 0.075, 25.3, 147.0), "fenders")
    th(B.fender("th-fender-f", FK, PLA, 0.4, (2.104, 0.255), 0.27, 0.075, 0.04, 0.075, 33.0, 148.9), "fenders")


def struct500(B, den=500):
    st = B.st
    stof = B.stof
    for bone, lst in {FR: ["seat-tube", "main-beam", "chainstay-l", "chainstay-r", "seatstay-l", "seatstay-r", "rack-deck"], BR: ["bars-envelope"], BX: ["box-floor", "box-rear-wall", "box-front-wall", "box-wall-l", "box-wall-r", "box-nose"], SD: ["stand-web"], WR: ["tyre-r"], WF: ["tyre-f"], CT: ["crate"]}.items():
        for i in lst:
            stof(f"st-{i}", i, bone, "member")
    for i, bone in (("steer-rod", FR), ("head-riser", FR), ("fork-leg-l", FK), ("fork-leg-r", FK)):
        stof(f"st-{i}", i, bone, "member")
    st(cl("fork-crown", FK, AL, 0.4, [0.0, 0.0, -0.07], [0.0, 0.0, 0.07], 0.0375), FK, "member")
    st(box("battery-case", BT, BAT, 0.4, [0.075, 0.32, 0.12], at=[0.03, 0.03, 0]), BT, "member")
    st(box("motor-housing", MT, MOT, 0.4, [0.14, 0.14, 0.14], r=0.03), MT, "member")
    st(box("crank-core", CK, AL, 0.4, [0.15, 0.15, 0.12]), CK, "member")
    st(box("crate-x-rail", CX, PLA, 0.4, [0.5, 0.075, 0.44], at=[0.25, 0.0375, 0]), CX, "member")
    st(box("crate-y-post", CY, PLA, 0.4, [0.075, 0.56, 0.44], at=[0.0375, 0.28, 0]), CY, "member")
    st(ellipsoid("head-bearing", FK, RIM, 0.5, [0.04, 0.04, 0.05], at=[0, 0, 0]), FK, "joint")
    for tag, bone in (("r", WR), ("f", WF)):
        st(ellipsoid(f"hub-{tag}", bone, RIM, 0.5, [0.04, 0.04, 0.04], at=[0, 0, 0]), bone, "joint")
    st(world_box("contact-pad-f", FK, "rubber", 0.1, [0.08, 0.075, 0.08], [2.099, 0.0001 + 0.0376], role="body", anchor="contact-f"), FK, "support")
    st(ellipsoid("mass-crates", CT, PLA, 0.4, [0.12, 0.1, 0.12], at=[0.25, 0.28, 0]), CT, "mass")
    st(ellipsoid("mass-battery", BT, BAT, 0.4, [0.04, 0.1, 0.06], at=[0.03, 0.03, 0]), BT, "mass")
    st(ellipsoid("mass-motor", MT, MOT, 0.4, [0.05, 0.05, 0.05], at=[0, 0, 0]), MT, "mass")


def regions(i):
    if i.startswith(("chainstay", "seatstay", "seat-", "main-beam", "head-", "steer-", "rack", "fork", "steerer", "stand", "crank-axle", "crank-arm", "pedal", "stem")):
        return "frame"
    if i.startswith(("saddle", "handlebar", "grip", "bars")):
        return "touch-parts"
    if i.startswith(("fender", "rear-light")):
        return "fenders"
    if i.startswith("box") and "batten" not in i:
        return "box-wood"
    if i.startswith("crate"):
        return "cargo"
    if i == "battery-case":
        return "battery-pack"
    if i == "motor-housing":
        return "drive-motor"
    if i.startswith("tyre"):
        return "tyres"
    if i.startswith("rotor"):
        return "brakes"
    return None


def struct_coarse(B, den):
    st = B.st
    stof = B.stof
    mem = {FR: ["seat-tube", "main-beam", "head-riser", "chainstay-l", "chainstay-r", "seatstay-l", "seatstay-r", "steer-rod", "rack-deck"], FK: ["fork-leg-l", "fork-leg-r", "fork-crown"], BR: (["stem", "stem-cross", "handlebar-l", "handlebar-r"] if den == 100 else (["stem", "bars-envelope"] if den == 200 else ["stem", "stem-cross"])), BX: ["box-floor", "box-rear-wall", "box-front-wall", "box-wall-l", "box-wall-r"], BT: ["battery-case"], MT: ["motor-housing"], CK: (["crank-arm-n", "crank-arm-f"] if den == 100 else []), SD: ["stand-leg-l", "stand-leg-r"] + (["stand-foot"] if den <= 100 else []), WR: ["tyre-r"], WF: ["tyre-f"], CT: ["crate-lower", "crate-upper"]}
    ids = {s["id"] for s in B.body}
    for bone, lst in mem.items():
        for i in lst:
            if i in ids:
                stof(f"st-{i}", i, bone, "member")
    ft = 0.0075 * den / 50
    if den == 100:
        st(tubel("crank-axle", CK, AL, 0.4, [0, 0, -0.105], [0, 0, 0.105], 0.012, 0.009), CK, "member")
    else:
        st(box("crank-core", CK, AL, 0.4, [ft * 2, ft * 2, 0.12]), CK, "member")
    st(box("crate-x-rail", CX, PLA, 0.4, [0.5, ft * 2, 0.44], at=[0.25, ft, 0]), CX, "member")
    st(box("crate-y-post", CY, PLA, 0.4, [ft * 2, 0.56, 0.44], at=[ft, 0.28, 0]), CY, "member")
    st(ellipsoid("head-bearing", FK, RIM, 0.5, [0.017, 0.017, 0.05], at=[0, 0, 0]), FK, "joint")
    st(ellipsoid("bar-joint", BR, RIM, 0.5, [max(0.0105, ft * 0.52)] * 3, at=[0.0, 0, 0]), BR, "joint")
    if den <= 100:
        st(ellipsoid("bb-bearing", CK, RIM, 0.5, [0.025, 0.025, 0.04], at=[0, 0, 0]), CK, "joint")
        st(ellipsoid("stand-pivot", SD, RIM, 0.5, [0.0105, 0.0105, 0.04], at=[0, 0, 0]), SD, "joint")
    for tag, bone in (("r", WR), ("f", WF)):
        st(ellipsoid(f"hub-{tag}", bone, RIM, 0.5, [0.025, 0.025, 0.04], at=[0, 0, 0]), bone, "joint")
    st(world_box("contact-pad-f", FK, "rubber", 0.1, [0.05, ft * 1.01, 0.05], [2.099, 0.0001 + ft * 0.56], role="body", anchor="contact-f"), FK, "support")
    st(ellipsoid("mass-crates", CT, PLA, 0.4, [0.12, 0.1, 0.12], at=[0.25, 0.28, 0]), CT, "mass")
    st(ellipsoid("mass-battery", BT, BAT, 0.4, [0.02, 0.1, 0.04], at=[0.03, 0.03, 0]), BT, "mass")
    st(ellipsoid("mass-motor", MT, MOT, 0.4, [0.05, 0.05, 0.05], at=[0, 0, 0]), MT, "mass")


def struct50(B, den=50):
    st = B.st
    W = 0.002
    stof = B.stof
    st(tubel("seat-tube", FR, AL, 0.4, [0.55, 0.32, 0], [0.30, 0.98, 0], 0.0155, 0.0135), FR, "member")
    st(cl("seat-post", FR, AL, 0.4, [0.3, 0.98, 0], [0.285, 1.04, 0], 0.0095), FR, "member")
    st(rbar_hollow("main-beam", FR, AL, 0.4, [0.55, 0.32], [1.78, 0.40], 0.019, 0.029, W), FR, "member")
    st(tubel("head-riser", FR, AL, 0.4, [1.78, 0.40, 0], [1.85, 0.62, 0], 0.0245, 0.0225), FR, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tubel(f"chainstay-{nm}", FR, AL, 0.4, [0, 0.33, sg * 0.06], [0.55, 0.32, sg * 0.03], 0.0105, 0.0085), FR, "member")
        st(tubel(f"seatstay-{nm}", FR, AL, 0.4, [0, 0.33, sg * 0.06], [0.34, 0.86, sg * 0.02], 0.009, 0.007), FR, "member")
        st(tubel(f"rack-rail-{nm}", FR, AL, 0.4, [-0.2575, 0.7915 + 0.0, sg * 0.1425], [0.2875, 0.8605, sg * 0.1425], 0.0055, 0.0035), FR, "member")
    st(tubel("steer-rod", FR, AL, 0.4, [0.74, 0.62, 0], [1.85, 0.62, 0], 0.0125, 0.0105), FR, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tubel(f"fork-leg-{nm}", FK, AL, 0.4, [0.0, 0.0, sg * 0.06], [0.4424, 0, sg * 0.06], 0.0105, 0.0085), FK, "member")
    st(tubel("fork-crown", FK, AL, 0.4, [0.0, 0.0, -0.068], [0.0, 0.0, 0.068], 0.0185, 0.0165), FK, "member")
    st(tubel("steerer", FK, AL, 0.4, [0.0, 0.0, 0.0], [-0.07, 0.0, 0.0], 0.0145, 0.0125), FK, "member")
    st(tubel("stem", BR, AL, 0.4, [0.0, 0, 0], [0.44, 0, 0], 0.0115, 0.0095), BR, "member")
    st(tubel("stem-cross", BR, AL, 0.4, [0.44, 0, -0.195], [0.44, 0, 0.195], 0.0115, 0.0095), BR, "member")
    hb = [[0.44, 0.0, 0.195], [0.45, 0.03, 0.225], [0.47, 0.09, 0.245], [0.488, 0.137, 0.255]]
    for sg, nm in ((1, "l"), (-1, "r")):
        for k in range(3):
            st(tubel(f"handlebar-{nm}-{k + 1}", BR, AL, 0.4, [hb[k][0], hb[k][1], sg * hb[k][2]], [hb[k + 1][0], hb[k + 1][1], sg * hb[k + 1][2]], 0.0105, 0.0085), BR, "member")
    for nm in ("floor", "rear-wall", "front-wall", "wall-l", "wall-r"):
        stof(f"st-box-{nm}", f"box-{nm}", BX, "member")
    stof("st-battery", "battery-case", BT, "member")
    stof("st-motor", "motor-housing", MT, "member")
    st(tubel("crank-axle", CK, AL, 0.4, [0, 0, -0.105], [0, 0, 0.105], 0.0085, 0.0065), CK, "member")
    for nm in ("n", "f"):
        stof(f"st-crank-arm-{nm}", f"crank-arm-{nm}", CK, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tubel(f"stand-leg-{nm}", SD, AL, 0.4, [0, 0, sg * 0.12], [0.3433, 0, sg * 0.12], 0.0095, 0.0075), SD, "member")
    stof("st-stand-foot", "stand-foot", SD, "member")
    for tag, bone, rr, hr in (("r", WR, 0.283, 0.0145), ("f", WF, 0.208, 0.0145)):
        stof(f"st-rim-{tag}", f"rim-{tag}", bone, "member")
        for gi, (zf, lead) in enumerate(((0.028, 1), (0.028, -1), (-0.028, 1), (-0.028, -1))):
            a_h = math.radians(10 * gi)
            a_r = a_h + lead * math.radians(62)
            R = rr - 0.0039
            s = capsule(f"spoke-{tag}-{gi + 1}", bone, RIM, 0.6, [0.036 * math.cos(a_h), 0.036 * math.sin(a_h), zf], [R * math.cos(a_r), R * math.sin(a_r), 0.0], 0.0015, 0.0015, repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": r6(2 * math.pi / 9), "origin": [0, 0, 0]})
            s["slender"] = True
            st(s, bone, "member")
        st(revolve(f"axle-{tag}", bone, AL, 0.5, "z", [0, 0, 0], [[-0.06, 0], [-0.06, 0.0095], [0.06, 0.0095], [0.06, 0], [-0.06, 0]]), bone, "joint")
    st(box("crate-x-rail", CX, PLA, 0.4, [0.5, 0.012, 0.44], at=[0.25, 0.006, 0]), CX, "member")
    st(box("crate-y-post", CY, PLA, 0.4, [0.012, 0.56, 0.44], at=[0.006, 0.28, 0]), CY, "member")
    stof("st-crate-lower", "crate-lower", CT, "member")
    stof("st-crate-upper", "crate-upper", CT, "member")
    # joints
    st(ellipsoid("head-bearing", FK, RIM, 0.5, [0.03, 0.03, 0.03], at=[0.0, 0.0, 0.0]), FK, "joint")
    st(ellipsoid("bar-joint", BR, RIM, 0.5, [0.02, 0.02, 0.02], at=[0.0, 0.0, 0.0]), BR, "joint")
    st(ellipsoid("bb-bearing", CK, RIM, 0.5, [0.02, 0.02, 0.03], at=[0, 0, 0]), CK, "joint")
    st(ellipsoid("stand-pivot", SD, RIM, 0.5, [0.011, 0.011, 0.03], at=[0, 0, 0.0]), SD, "joint")
    st(ellipsoid("battery-mount", BT, RIM, 0.5, [0.02, 0.02, 0.03], at=[0.133, -0.175, 0.0]), BT, "joint")
    st(ellipsoid("box-mount", BX, RIM, 0.5, [0.025, 0.02, 0.05], at=[0.1, 0.012, 0.0]), BX, "joint")
    # support
    fp = P.loc(FK, [0.6528, -0.1442, 0])
    gw = [2.099, 0.0001]
    st(world_box("contact-pad-f", FK, "rubber", 0.1, [0.05, 0.0076, 0.05], [gw[0], gw[1] + 0.0042], role="body", anchor="contact-f"), FK, "support")
    # masses
    st(ellipsoid("mass-crates", CT, PLA, 0.4, [0.12, 0.1, 0.12], at=[0.25, 0.28, 0]), CT, "mass")
    st(dict(ellipsoid("mass-battery", BT, BAT, 0.4, [0.022, 0.12, 0.045], at=[0.065, 0.0, 0]), transform={"translation": [0.065, 0.0, 0.0], "rotation": qaxis([0, 0, 1], 20.8)}), BT, "mass")
    st(ellipsoid("mass-motor", MT, MOT, 0.4, [0.05, 0.05, 0.05], at=[0, 0, 0]), MT, "mass")



DEC = {
    50: (
        "1:50 is the detailed model of the electric cargo bike with its open plywood box: aluminium frame as oval tubes on their real centre lines (seat tube with post and collar, 60 mm main beam from the bottom bracket to the head riser, splayed chain and seat stays), a lofted saddle from the drawn plan and elevation, rear rack as a tray with a rim, fork legs with crown and steerer, handlebar bars swept through the drawn cross bar to the grips with brake levers and a 20 mm cockpit plate between the grips (the drawn front view fills the handlebar as a 600 x 92 mm block), tyre and rim as revolves with 36 slender laced spokes and a brake disc with carrier on each wheel, motor housing with crank axle, chainring, arms and pedals, battery case as the drawn slanted prism, folded rear stand, mudguards as arched channels, and the cargo box as 12 mm plywood walls, floor, nose panel and two crates (battens as texture).",
        "1:50 ist das ausführliche Modell des elektrischen Lastenrads mit offener Sperrholzbox: Aluminiumrahmen aus ovalen Rohren auf ihren wirklichen Mittellinien (Sattelrohr mit Stütze und Klemmring, 60-mm-Hauptträger vom Tretlager zum Steuerrohr, gespreizte Kettenstreben und Sitzstreben), ein gelofteter Sattel nach gezeichnetem Grund- und Aufriss, Gepäckträger als Wanne mit Rand, Gabelholme mit Krone und Gabelschaft, Lenkerstangen über die gezeichnete Quertraverse zu den Griffen geschwungen mit Bremshebeln und einer 20-mm-Cockpitplatte zwischen den Griffen (die gezeichnete Vorderansicht füllt den Lenker als Block von 600 x 92 mm), Reifen und Felge als Rotationskörper mit 36 schlanken gekreuzten Speichen und je Rad einer Bremsscheibe mit Träger, Motorgehäuse mit Kurbelachse, Kettenblatt, Armen und Pedalen, Akkugehäuse als gezeichnetes schräges Prisma, hochgeklappter Hinterbauständer, Schutzbleche als Bogenrinnen und die Transportbox aus 12-mm-Sperrholzwänden, Boden, Bugblech und zwei Kisten (Leisten als Textur).",
        "Thermal: references to the body solids in the region the 2D binds them to (frame tubes, handle bars and grips, mudguards, box, crates, battery, motor, tyres, brake discs); every layer is a single skin, so there are no nested shells; rims, spokes and hubs have no region. The areas of the crates, the motor and the tyres are corrected from this model (old 1.2, 0.12 and 0.9 m2, new 1.61, 0.095 and 0.635 m2); frame, box (open, both faces exposed), mudguards, saddle and grips and brake discs deviate by more than a factor 1.5 and are listed in the area review.",
        "Thermik: Verweise auf die Körper in der Region, an die die 2D-Zeichnung sie bindet (Rahmenrohre, Lenker und Griffe, Schutzbleche, Box, Kisten, Akku, Motor, Reifen, Bremsscheiben); jede Schicht ist eine einzelne Haut, es gibt keine verschachtelten Schalen; Felgen, Speichen und Naben haben keine Region. Die Flächen von Kisten, Motor und Reifen sind aus diesem Modell korrigiert (alt 1,2, 0,12 und 0,9 m2, neu 1,61, 0,095 und 0,635 m2); Rahmen, Box (offen, beide Seiten frei), Schutzbleche, Sattel und Griffe sowie Bremsscheiben weichen um mehr als den Faktor 1,5 ab und stehen im Flächen-Review.",
        "Structure: every bone has a member on its real line: hollow 2 mm aluminium for seat tube, post, main beam, head riser, stays, steer rod, fork legs, crown, steerer, stem and the three straight handlebar pieces, the box floor and walls, battery case, motor and crank arms as the body solids themselves, crank axle, stand legs and foot, rim rings with 36 slender spokes, the crate tray and post as the prismatic crate bones; joints at head bearing, bar base, bottom bracket, stand pivot, battery mount, box mount and both hubs; the front tyre patch is the support (the rear tyre and the stand foot are pose-dependent contacts); masses sit in the crates, the battery and the motor.",
        "Tragwerk: jeder Knochen hat ein Glied auf seiner wirklichen Linie: hohles 2-mm-Aluminium für Sattelrohr, Stütze, Hauptträger, Steuerrohr, Streben, Lenkstange, Gabelholme, Krone, Gabelschaft, Vorbau und die drei geraden Lenkerstücke, Boxboden und -wände, Akkugehäuse, Motor und Kurbelarme als die Körper selbst, Kurbelachse, Ständerbeine und -fuß, Felgenringe mit 36 schlanken Speichen, Kistenwanne und -pfosten als die verschiebbaren Kistenknochen; Gelenke an Steuersatz, Lenkerfuß, Tretlager, Ständerdrehpunkt, Akkuhalter, Boxhalter und beiden Naben; Auflager ist die Aufstandsfläche des Vorderreifens (Hinterreifen und Ständerfuß sind lageabhängige Kontakte); Massen sitzen in den Kisten, im Akku und im Motor.",
    ),
    100: (
        "1:100 is modelled anew against that scale's drawings: frame tubes follow the drawn strokes and are slender at the drawn pen widths (13 to 30 mm), the head riser and spokes are gone (not drawn), tyre and rim are two revolves per wheel with the rim ring at the drawn radius, the brake disc is one 15 mm disc, the rack is one 20 mm slab, the saddle three sections, mudguards 15 mm channels at the radii of this scale, the handlebar two three-point sweeps, the stand two legs with a hinge tube, the box has the lower front cut away as drawn with 15 mm walls and two crates of the drawn sizes.",
        "1:100 ist neu gegen die Zeichnungen dieses Maßstabs modelliert: Die Rahmenrohre folgen den gezeichneten Strichen und sind schlank mit den gezeichneten Stiftbreiten (13 bis 30 mm), Steuerrohr und Speichen entfallen (nicht gezeichnet), Reifen und Felge sind zwei Rotationskörper je Rad mit dem Felgenring auf dem gezeichneten Radius, die Bremsscheibe ist eine 15-mm-Scheibe, der Träger eine 20-mm-Platte, der Sattel hat drei Schnitte, Schutzbleche sind 15-mm-Rinnen auf den Radien dieses Maßstabs, der Lenker zwei Schwünge aus drei Punkten, der Ständer zwei Beine mit Scharnierrohr, die Box hat die untere Front wie gezeichnet ausgeschnitten mit 15-mm-Wänden und zwei Kisten in den gezeichneten Größen.",
        "Thermal: one reference per body solid in its region; the 2 mm walls and 12 mm plywood layers are far below the 15 mm feature and stay single skins; every region has solids.",
        "Thermik: ein Verweis je Körper in seiner Region; die 2-mm-Wände und 12-mm-Sperrholzschichten liegen weit unter dem 15-mm-Merkmal und bleiben einfache Häute; jede Region hat Körper.",
        "Structure: the same load path as 1:50 on solid members that are the body solids (the 2 mm tube walls cannot be shown at 15 mm), a hollow crank axle, crate tray and post, joints at head bearing, bar base, bottom bracket, stand pivot and hubs, the front patch as 15 mm support, masses as at 1:50.",
        "Tragwerk: derselbe Lastpfad wie bei 1:50 auf massiven Gliedern, die die Körper selbst sind (die 2-mm-Rohrwände sind bei 15 mm nicht darstellbar), hohle Kurbelachse, Kistenwanne und -pfosten, Gelenke an Steuersatz, Lenkerfuß, Tretlager, Ständerdrehpunkt und Naben, die vordere Aufstandsfläche als 15-mm-Auflager, Massen wie bei 1:50.",
    ),
    200: (
        "1:200 merges: each wheel is one tyre ring with the rim as a 30 mm ring at the drawn radius and a solid brake disc, the frame is slender strokes (seat tube, main beam, stays, steer rod) at the drawn pen widths, the rear mudguard is one 30 mm channel, rack one 30 mm slab, the saddle three sections, the handlebar with stem, levers and grips one envelope prism (the drawings fill it 600 x 80 mm in front and 600 x 200 mm in plan), the stand two slender legs and a web plate that the front view draws filled, the box 30 mm walls and floor with the drawn cut-away nose and the two crates; head riser, fork crown detail, head lamp, pedals and chainring are below the drawing and stay out.",
        "1:200 fasst zusammen: jedes Rad ist ein Reifenring mit der Felge als 30-mm-Ring auf dem gezeichneten Radius und einer massiven Bremsscheibe, der Rahmen besteht aus schlanken Strichen (Sattelrohr, Hauptträger, Streben, Lenkstange) in den gezeichneten Stiftbreiten, das hintere Schutzblech ist eine 30-mm-Rinne, der Träger eine 30-mm-Platte, der Sattel hat drei Schnitte, der Lenker mit Vorbau, Hebeln und Griffen ist ein Hüllprisma (die Zeichnungen füllen es 600 x 80 mm von vorn und 600 x 200 mm im Grundriss), der Ständer zwei schlanke Beine und eine Stegplatte, die die Vorderansicht gefüllt zeichnet, die Box 30-mm-Wände und -Boden mit der gezeichneten ausgeschnittenen Nase und die zwei Kisten; Steuerrohr, Gabelkrone, Frontlampe, Pedale und Kettenblatt liegen unter der Zeichnung und bleiben draußen.",
        "Thermal: references to the body solids; only the sum of the exposed areas counts at this scale and every region has a solid (the front mudguard is not drawn and is part of the rear one's region total).",
        "Thermik: Verweise auf die Körper; in diesem Maßstab zählt nur die Summe der freien Flächen und jede Region hat einen Körper (das vordere Schutzblech ist nicht gezeichnet und steckt im Regionswert des hinteren).",
        "Structure: members are the body solids at their 30 mm sections (box walls and floor, crates, tyres, frame strokes, fork legs, stand legs and web, envelope prism for the handlebar), a 60 mm crank core, the crate tray and post, joints at the head bearing and both hubs, the front patch as 30 mm support, masses as at 1:50.",
        "Tragwerk: die Glieder sind die Körper in ihren 30-mm-Querschnitten (Boxwände und -boden, Kisten, Reifen, Rahmenstriche, Gabelholme, Ständerbeine und -steg, Hüllprisma für den Lenker), ein 60-mm-Kurbelkern, Kistenwanne und -pfosten, Gelenke am Steuersatz und an beiden Naben, die vordere Aufstandsfläche als 30-mm-Auflager, Massen wie bei 1:50.",
    ),
    500: (
        "1:500 is the pure mass model: two wheel discs 80 mm wide, the frame as 65 mm strokes and a 75 mm main beam, rack and gusset as one 75 mm plate following the drawn plan, the handlebar and stem as the extruded envelope of the drawn outline (600 mm wide), the stand as a 75 mm web, the box as a tray with 75 mm walls, floor and cut-away nose, the crates as one 500 x 560 mm block; steer rod, head riser and fork legs are added to the 2D as strokes and modelled at 65 to 75 mm.",
        "1:500 ist das reine Massenmodell: zwei 80 mm breite Radscheiben, der Rahmen als 65-mm-Striche und ein 75-mm-Hauptträger, Träger und Knotenblech als eine 75-mm-Platte nach dem gezeichneten Grundriss, Lenker und Vorbau als extrudierte Hülle des gezeichneten Umrisses (600 mm breit), der Ständer als 75-mm-Steg, die Box als Wanne mit 75-mm-Wänden, -Boden und ausgeschnittener Nase, die Kisten als ein Block von 500 x 560 mm; Lenkstange, Steuerrohr und Gabelholme sind der 2D-Zeichnung als Striche hinzugefügt und mit 65 bis 75 mm modelliert.",
        "Thermal: the body solids as references where they carry a region (the 75 mm stand web, fork legs and head riser are left out: they stand for 20 mm tubes and would overstate the frame surface); tyres are real 80 mm rings instead of the drawn discs; saddle, battery, motor, brake discs and both mudguards are real thermal solids at the 75 mm floor (not drawn here); only the total of the exposed areas counts at this scale.",
        "Thermik: die Körper als Verweise, wo sie eine Region tragen (Ständersteg, Gabelholme und Steuerrohr von 75 mm bleiben draußen: sie stehen für 20-mm-Rohre und würden die Rahmenfläche überzeichnen); Reifen sind echte 80-mm-Ringe statt der gezeichneten Scheiben; Sattel, Akku, Motor, Bremsscheiben und beide Schutzbleche sind echte Thermikkörper auf dem 75-mm-Mindestmaß (hier nicht gezeichnet); in diesem Maßstab zählt nur die Summe der freien Flächen.",
        "Structure: the body solids as members, real members for steer rod aside, crown, battery, motor, crank core and the crate tray and post, joints at the head bearing and hubs, the front patch as 75 mm support, masses as at 1:50.",
        "Tragwerk: die Körper als Glieder, eigene Glieder für Gabelkrone, Akku, Motor, Kurbelkern sowie Kistenwanne und -pfosten, Gelenke am Steuersatz und an den Naben, die vordere Aufstandsfläche als 75-mm-Auflager, Massen wie bei 1:50.",
    ),
}


def main(den):
    B = {50: build50, 100: build100, 200: build200, 500: build500}[den]()
    (struct50 if den == 50 else struct500 if den == 500 else struct_coarse)(B, den) if den != 50 else struct50(B)
    thermal_by(B, (lambda i: None if (den == 500 and i.startswith(("tyre", "stand", "fork", "head-riser"))) else regions(i)))
    if den == 500:
        thermal500(B)
    dec(B, DEC[den])
    print(write_scale(SLUG, den, B))
    print(check(SLUG, den))


if __name__ == "__main__":
    main(int(sys.argv[1]))
