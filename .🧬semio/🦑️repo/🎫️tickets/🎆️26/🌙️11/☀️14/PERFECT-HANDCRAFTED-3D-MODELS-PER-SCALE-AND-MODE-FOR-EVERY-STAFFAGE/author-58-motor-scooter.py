import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_58_lib import *

SLUG = "motor-scooter"
FR, ST, WF, WR, SW, SE, SD = "frame", "steer", "wheel-f", "wheel-r", "swing", "seat", "stand"
PNT, RUB, ALU, VIN, IRON, EXH, ENG, GLS = "paint", "rubber", "alloy", "seat-vinyl", "cast-iron", "exhaust-steel", "engine-metal", "glass"
P = Pose(SLUG)


def interp(pts, x):
    pts = sorted(pts)
    if x <= pts[0][0]:
        return pts[0][1]
    for (a, ya), (b, yb) in zip(pts, pts[1:]):
        if x <= b:
            return ya + (yb - ya) * (x - a) / (b - a)
    return pts[-1][1]


YB = [(-0.945, 0.42), (-0.9, 0.31), (-0.8, 0.26), (-0.62, 0.27), (-0.36, 0.3), (-0.26, 0.28), (-0.2, 0.27)]
YT = [(-0.945, 0.7), (-0.93, 0.76), (-0.85, 0.81), (-0.76, 0.8), (-0.22, 0.7)]
HW = [(-0.945, 0.06), (-0.93, 0.1), (-0.88, 0.16), (-0.7, 0.19), (-0.4, 0.19), (-0.25, 0.2), (-0.2, 0.2)]
XF = [(0.31, 0.28), (0.34, 0.4), (0.37, 0.47), (0.43, 0.53), (0.54, 0.56), (0.72, 0.51), (0.86, 0.46), (0.9, 0.46), (0.995, 0.36)]
XB = [(0.34, 0.12), (0.4, 0.17), (0.56, 0.21), (0.78, 0.26), (0.93, 0.31), (0.995, 0.34)]
HZ = [(0.26, 0.21), (0.5, 0.2), (0.9, 0.16), (1.0, 0.13)]


YB5 = [(-0.945, 0.5), (-0.92, 0.36), (-0.8, 0.3), (-0.62, 0.3), (-0.31, 0.34), (-0.2, 0.27)]
XF5 = [(0.33, 0.36), (0.47, 0.5), (0.7, 0.55), (0.95, 0.56), (1.1, 0.55)]
XB5 = [(0.4, 0.12), (0.48, 0.17), (0.62, 0.21), (0.8, 0.26), (0.93, 0.31), (1.0, 0.33), (1.1, 0.33)]
HZ5 = [(0.26, 0.21), (0.5, 0.2), (0.9, 0.16), (1.0, 0.13), (1.1, 0.17)]


def rear_body(den):
    yb = YB5 if den == 500 else YB
    xs = [-0.94, -0.9, -0.8, -0.65, -0.5, -0.35, -0.2] if den == 50 else ([-0.94, -0.85, -0.65, -0.4, -0.2] if den <= 200 else [-0.94, -0.8, -0.5, -0.2])
    secs = [sec([x, (interp(yb, x) + interp(YT, x)) / 2, 0], interp(HW, x), (interp(YT, x) - interp(yb, x)) / 2, exponent=3.0, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])) for x in xs]
    return loft("body-rear", FR, PNT, 0.6, secs)


def floor_body(den):
    xs = [-0.2, -0.1, 0.0, 0.1, 0.2] if den <= 100 else [-0.2, 0.2]
    secs = [sec([x, 0.305 if den < 500 else 0.335, 0], 0.2, 0.035 if den < 500 else 0.065, exponent=3.0, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])) for x in xs]
    return loft("body-floor", FR, PNT, 0.6, secs)


def front_body(den):
    if den == 500:
        secs = []
        for y in [0.4, 0.5, 0.75, 0.97, 1.1]:
            xf, xb = interp(XF5, y), interp(XB5, y)
            secs.append(sec([(xf + xb) / 2, y, 0], (xf - xb) / 2, interp(HZ5, y), exponent=3.0, rotation=basis_quat([1, 0, 0], [0, 0, 1], [0, -1, 0])))
        return loft("body-front", FR, PNT, 0.6, secs)
    ys = [0.33, 0.42, 0.52, 0.62, 0.72, 0.82, 0.9, 0.97] if den == 50 else ([0.33, 0.5, 0.7, 0.85, 0.97] if den <= 200 else [0.33, 0.5, 0.75, 0.97])
    secs = []
    for y in ys:
        xf, xb = interp(XF, y), interp(XB, y)
        secs.append(sec([(xf + xb) / 2, y, 0], (xf - xb) / 2, interp(HZ, y), exponent=3.0, rotation=basis_quat([1, 0, 0], [0, 0, 1], [0, -1, 0])))
    return loft("body-front", FR, PNT, 0.6, secs)


SEAT_BOT = [(0.02, -0.03), (0.18, -0.065), (0.38, -0.105), (0.58, -0.12), (0.6, -0.085)]
SEAT_UP = [(0.0, 0.075), (0.54, -0.025), (0.6, -0.085)]


def seat_loft(den):
    us = [0.02, 0.1, 0.2, 0.3, 0.4, 0.5, 0.58] if den == 50 else ([0.02, 0.2, 0.4, 0.58] if den == 100 else [0.02, 0.3, 0.58])
    secs = []
    for u in us:
        top, bot = interp(SEAT_UP, u), interp(SEAT_BOT, u)
        secs.append(sec([u, (top + bot) / 2, 0], 0.155, (top - bot) / 2, exponent=3.0, rotation=basis_quat([0, 0, 1], [0, 1, 0], [-1, 0, 0])))
    return loft("seat", SE, VIN, 0.15, secs)


COWL = [[0.0215, -0.1007], [-0.119, -0.0473], [-0.1684, 0.0306], [-0.145, 0.1071], [-0.0576, 0.1536], [0.0571, 0.1185], [0.094, 0.034]]


def wheel_body(B, tag, bone, hw, tyre_in=0.153, tyre_out=0.23, fine=True, disc=True):
    b = B.b
    b(revolve(f"tyre-{tag}", bone, RUB, 0.1, "z", [0, 0, 0], superpoly(0, (tyre_out + tyre_in) / 2, hw, (tyre_out - tyre_in) / 2, 3.0, 28)))
    if fine:
        rim = [[-hw * 0.7, 0], [-hw * 0.7, tyre_in + 0.003], [hw * 0.7, tyre_in + 0.003], [hw * 0.7, 0], [-hw * 0.7, 0]]
        rim = [[-0.03, 0.0], [-0.03, tyre_in - 0.0], [0.03, tyre_in - 0.0], [0.03, 0.0], [-0.03, 0.0]]
        b(revolve(f"rim-{tag}", bone, ALU, 0.75, "z", [0, 0, 0], rim))
    else:
        b(revolve(f"rim-{tag}", bone, ALU, 0.75, "z", [0, 0, 0], [[-0.04, 0], [-0.04, tyre_in], [0.04, tyre_in], [0.04, 0], [-0.04, 0]]))
    if disc:
        b(revolve(f"disc-{tag}", bone, IRON, 0.4, "z", [0, 0, 0], [[0.026, 0.06], [0.034, 0.06], [0.034, 0.095], [0.026, 0.095], [0.026, 0.06]], role="detail"))
    b(revolve(f"hub-{tag}", bone, IRON, 0.5, "z", [0, 0, 0], [[-hw - 0.012, 0], [-hw - 0.012, 0.045], [hw + 0.012, 0.045], [hw + 0.012, 0], [-hw - 0.012, 0]], role="detail"))


def build50():
    B = Bike(50, SLUG)
    b = B.b
    b(rear_body(50)); b(floor_body(50)); b(front_body(50))
    b(box("tail-lamp", FR, GLS, 0.85, [0.03, 0.08, 0.14], at=[-0.94, 0.6, 0], r=0.008, role="detail"))
    # steering assembly (steer bone: u down the fork, v forward-up)
    for sg, nm in ((1, "l"), (-1, "r")):
        b(path_sweep(f"fork-{nm}", ST, PNT, 0.35, [[0.0, 0.0, sg * 0.078], [0.79, 0.0198, sg * 0.078]], 0.016, lambda t: 0.026 - 0.006 * t, step=0, caps=True, lat=(0, 0, 1)))
    b(cl("fork-crown", ST, PNT, 0.35, [0.02, 0.0, -0.085], [0.02, 0.0, 0.085], 0.024))
    b(dict(ext("cowl", ST, PNT, 0.4, "xy", COWL, -0.17, 0.17), transform={"translation": [0, 0, 0]}))
    b(ellipsoid("headlamp", ST, GLS, 0.85, [0.03, 0.045, 0.065], at=[-0.0457, 0.0558, 0]))
    b(cl("handlebar", ST, PNT, 0.3, [-0.09, 0.0, -0.29], [-0.09, 0.0, 0.29], 0.0175))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"grip-{nm}", ST, RUB, 0.1, [-0.09, 0.0, sg * 0.29], [-0.09, 0.0, sg * 0.339], 0.031))
    b(B.fender("fender-f", ST, PNT, 0.45, (0.67, 0.23), 0.268, 0.012, 0.065, 0.008, 33.0, 162.0))
    # wheels
    wheel_body(B, "f", WF, 0.05)
    wheel_body(B, "r", WR, 0.06)
    # swing: engine cover, muffler, tail pipe
    b(ellipsoid("engine-cover", SW, ENG, 0.35, [0.11, 0.09, 0.03], at=[-0.23, 0.22, -0.185], role="detail"))
    b(cl("muffler", SW, EXH, 0.4, [-0.35, 0.16, -0.15], [-0.64, 0.17, -0.15], 0.05, role="detail"))
    b(cl("tail-pipe", SW, EXH, 0.45, [-0.64, 0.17, -0.15], [-0.76, 0.17, -0.15], 0.022, 0.02, role="detail"))
    b(seat_loft(50))
    for sg, nm in ((1, "l"), (-1, "r")):
        b(cl(f"stand-leg-{nm}", SD, PNT, 0.3, [0, 0, sg * 0.11], [0.335, 0, sg * 0.11], 0.015, role="detail"))
    b(cl("stand-foot", SD, PNT, 0.3, [0.335, 0, -0.11], [0.335, 0, 0.11], 0.012, role="detail"))
    b(cl("stand-pivot", SD, PNT, 0.3, [0, 0, -0.13], [0, 0, 0.13], 0.012, role="detail"))
    return B


def regions(i):
    if i.startswith(("body", "stand", "fender")):
        return "body-panels"
    if i.startswith(("fork", "cowl", "handlebar", "grip")):
        return "steer-assembly"
    if i in ("headlamp", "tail-lamp"):
        return "headlamp"
    if i == "seat":
        return "seat"
    if i == "engine-cover":
        return "engine"
    if i in ("muffler", "tail-pipe"):
        return "exhaust-system"
    if i.startswith(("tyre", "rim")):
        return "tyres"
    if i.startswith(("disc", "hub")):
        return "brake-disc"
    return None


LAYERS = {"body-panels": [("wall", 0.0008), ("paint", 8e-05)], "steer-assembly": [("wall", 0.0008), ("paint", 8e-05)]}


def thermal50(B):
    by = {s["id"]: s for s in B.body}
    for s in B.body:
        reg = regions(s["id"])
        if reg is None:
            continue
        if reg in LAYERS:
            B.thof(f"th-{s['id']}", s["id"], reg, "paint")
            sh = inset(s, 8e-05, f"th-{s['id']}-wall")
            if sh is not None:
                sh["region"] = reg
                sh["layer"] = "wall"
                B.thermal.append(sh)
        else:
            B.thof(f"th-{s['id']}", s["id"], reg)
    hidden_engine(B, 50)


def hidden_engine(B, den):
    th = B.th
    f = 0.0075 * den / 50
    if den == 50:
        th(box("th-crankcase", SW, ENG, 0.3, [0.2, 0.14, 0.16], at=[-0.3, 0.2, 0.0], r=0.02), "engine")
        th(revolve("th-cylinder", SW, ENG, 0.3, [0.9, 0.44, 0], [-0.22, 0.26, 0.0], [[0, 0], [0, 0.045], [0.11, 0.045], [0.11, 0.035], [0.13, 0.035], [0.13, 0]]), "engine")
        th(cl("th-header", SW, EXH, 0.4, [-0.2, 0.32, -0.02], [-0.3, 0.16, -0.15], 0.017), "exhaust-system")
        th(cl("th-header-2", SW, EXH, 0.4, [-0.3, 0.16, -0.15], [-0.35, 0.16, -0.15], 0.017), "exhaust-system")
    elif den == 100:
        th(box("th-engine-block", SW, ENG, 0.3, [0.26, 0.16, 0.17], at=[-0.26, 0.21, 0.0], r=0.02), "engine")
        th(cl("th-header", SW, EXH, 0.4, [-0.2, 0.32, -0.02], [-0.35, 0.16, -0.15], 0.0175), "exhaust-system")
    else:
        th(box("th-engine-block", SW, ENG, 0.3, [0.3, 0.2, 0.2], at=[-0.27, 0.21, 0.0], r=0.03), "engine")
        th(cl("th-header", SW, EXH, 0.4, [-0.2, 0.32, -0.02], [-0.35, 0.16, -0.15], max(0.02, f * 0.51)), "exhaust-system")
    L = max(0.0076, f)
    emitter = revolve("tailpipe-jet", SW, EXH, 0.45, [1, 0, 0], [-0.76, 0.17, -0.15], [[0, 0], [0, max(0.0175, f * 0.51)], [L, max(0.0175, f * 0.51)], [L, 0]], tags=["tailpipe"])
    emitter["region"] = "exhaust-system"
    B.thermal.append(emitter)


def world_box(id, bone, mat, tone, size, c_world, anchor=None):
    c = P.loc(bone, [c_world[0], c_world[1], 0])
    d = box(id, bone, mat, tone, size)
    d["transform"] = {"translation": vec([c[0], c[1], 0.0]), "rotation": qaxis([0, 0, 1], -math.degrees(frame_angle(P, bone)))}
    if anchor:
        d["anchor"] = anchor
    return d


def build_coarse(den):
    B = Bike(den, SLUG)
    b = B.b
    f = 0.0075 * den / 50
    b(rear_body(den)); b(floor_body(den)); b(front_body(den))
    if den == 100:
        for sg, nm in ((1, "l"), (-1, "r")):
            b(path_sweep(f"fork-{nm}", ST, PNT, 0.35, [[0.0, 0.0, sg * 0.078], [0.79, 0.0198, sg * 0.078]], 0.016, lambda t: 0.026 - 0.006 * t, step=0, caps=True, lat=(0, 0, 1)))
        b(ext("cowl", ST, PNT, 0.4, "xy", COWL, -0.17, 0.17))
        b(ellipsoid("headlamp", ST, GLS, 0.85, [0.03, 0.045, 0.065], at=[-0.0457, 0.0558, 0]))
        b(cl("handlebar", ST, PNT, 0.3, [-0.09, 0.0, -0.3475], [-0.09, 0.0, 0.3475], 0.0225))
        b(B.fender("fender-f", ST, PNT, 0.45, (0.67, 0.23), 0.268, 0.012, 0.065, 0.015, 33.0, 162.0))
        for sg, nm in ((1, "l"), (-1, "r")):
            b(cl(f"stand-leg-{nm}", SD, PNT, 0.3, [0, 0, sg * 0.11], [0.335, 0, sg * 0.11], 0.015, role="detail"))
        b(ellipsoid("engine-cover", SW, ENG, 0.35, [0.11, 0.09, 0.03], at=[-0.23, 0.22, -0.185], role="detail"))
        b(cl("muffler", SW, EXH, 0.4, [-0.35, 0.16, -0.15], [-0.64, 0.17, -0.15], 0.05, role="detail"))
    else:
        fk = box("fork", ST, PNT, 0.35, [0.79, 0.05 if den == 200 else 0.075, 0.188], at=[0.395, 0.01, 0.0], r=0.01)
        b(fk)
        b(dict(ext("cowl", ST, PNT, 0.4, "xy", COWL, -0.17, 0.17)))
        b(world_box("bars-plate", ST, PNT, 0.3, [0.06 if den == 200 else 0.075, 0.22, 0.74], [0.4, 1.04]))
        if den == 200:
            b(cl("stand-leg-l", SD, PNT, 0.3, [0, 0, 0.11], [0.335, 0, 0.11], 0.015, role="detail"))
            b(cl("stand-leg-r", SD, PNT, 0.3, [0, 0, -0.11], [0.335, 0, -0.11], 0.015, role="detail"))
    for tag, bone, hw in (("f", WF, 0.05), ("r", WR, 0.06)):
        if den == 100:
            wheel_body(B, tag, bone, hw, fine=False, disc=False)
        else:
            hw2 = hw if den == 200 else max(hw, 0.0375)
            b(revolve(f"tyre-{tag}", bone, RUB, 0.1, "z", [0, 0, 0], [[-hw2, 0], [-hw2, 0.23], [hw2, 0.23], [hw2, 0], [-hw2, 0]]))
            if den == 200:
                b(revolve(f"hub-{tag}", bone, IRON, 0.5, "z", [0, 0, 0], [[-hw - 0.015, 0], [-hw - 0.015, 0.085], [hw + 0.015, 0.085], [hw + 0.015, 0], [-hw - 0.015, 0]], role="detail"))
    b(seat_loft(den))
    return B


def thermal_extra(B, den):
    th = B.th
    f = 0.0075 * den / 50
    if den == 100:
        th(box("th-tail-lamp", FR, GLS, 0.85, [0.03, 0.08, 0.14], at=[-0.94, 0.6, 0], r=0.008), "headlamp")
        for tag, bone, dz in (("f", WF, 0.03), ("r", WR, 0.035)):
            th(revolve(f"th-disc-{tag}", bone, IRON, 0.4, "z", [0, 0, 0], [[dz, 0.06], [dz + 0.0151, 0.06], [dz + 0.0151, 0.095], [dz, 0.095], [dz, 0.06]]), "brake-disc")
    if den >= 200:
        th(ellipsoid("th-headlamp", ST, GLS, 0.85, [max(0.03, f * 0.51)] * 2 + [0.065], at=[-0.0457, 0.0558, 0]), "headlamp")
        th(cl("th-fender-f", ST, PNT, 0.45, [0.3, 0.0, 0.0], [0.5, 0.0, 0.0], max(0.03, f * 0.51)), "body-panels") if False else None
    if den == 500:
        for tag, bone in (("f", WF), ("r", WR)):
            th(revolve(f"th-disc-{tag}", bone, IRON, 0.4, "z", [0, 0, 0], [[0.02, 0.06], [0.02 + 0.0751, 0.06], [0.02 + 0.0751, 0.095], [0.02, 0.095], [0.02, 0.06]]), "brake-disc")


def structure(B, den):
    st = B.st
    stof = B.stof
    ids = {s["id"] for s in B.body}
    fine = den == 50
    f = 0.0075 * den / 50
    W = 0.0015 if fine else 0
    def tube(id, bone, a, b_, ro):
        return tubel(id, bone, PNT, 0.4, a, b_, ro, ro - W) if fine else (tubel(id, bone, PNT, 0.4, a, b_, ro) if den <= 200 else tubel(id, bone, PNT, 0.4, a, b_, max(ro, 0.039)))
    pts = [[0.36, 0.92], [0.3, 0.62], [0.24, 0.44], [0.17, 0.33], [0.1, 0.31], [-0.15, 0.31], [-0.3, 0.32]]
    if den == 500:
        pts = [pts[0], pts[1], pts[2], pts[4], pts[5], pts[6]]
    for k in range(len(pts) - 1):
        st(tube(f"spine-{k + 1}", FR, pts[k] + [0.0], pts[k + 1] + [0.0], 0.014 if den <= 100 else (0.02 if den == 200 else 0.039)), FR, "member")
    rail = [[-0.3, 0.32], [-0.42, 0.6], [-0.85, 0.64]]
    for sg, nm in ((1, "l"), (-1, "r")):
        for k in range(2):
            st(tube(f"rail-{nm}-{k + 1}", FR, rail[k] + [sg * 0.12], rail[k + 1] + [sg * 0.12], 0.014 if den <= 100 else (0.02 if den == 200 else 0.039)), FR, "member")
    st(tube("steer-column", ST, [0.0, 0.0, 0.0], [0.3, 0.0, 0.0], 0.016 if den <= 100 else 0.02), ST, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tube(f"fork-leg-{nm}", ST, [0.0, 0.0, sg * 0.078], [0.79, 0.0198, sg * 0.078], 0.015 if den <= 100 else 0.0225), ST, "member")
    st(tube("handlebar-bar", ST, [-0.09, 0.0, -0.29], [-0.09, 0.0, 0.29], 0.0165 if den <= 100 else 0.03), ST, "member")
    for tag, bone in (("f", WF), ("r", WR)):
        stof(f"st-rim-{tag}", f"rim-{tag}" if f"rim-{tag}" in ids else f"tyre-{tag}", bone, "member")
        st(revolve(f"axle-{tag}", bone, ALU, 0.5, "z", [0, 0, 0], [[-0.07, 0], [-0.07, max(0.016, f * 0.51)], [0.07, max(0.016, f * 0.51)], [0.07, 0], [-0.07, 0]]), bone, "joint")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tube(f"swingarm-{nm}", SW, [0.0, 0.02, sg * 0.1], [-0.47, 0.0, sg * 0.1], 0.014 if den <= 100 else 0.02), SW, "member")
    st(box("crankcase", SW, ENG, 0.4, {50: [0.2, 0.14, 0.16], 100: [0.22, 0.15, 0.17], 200: [0.24, 0.17, 0.19], 500: [0.26, 0.19, 0.2]}[den], at=[-0.3, 0.2, 0.0], r=0.02), SW, "member")
    stof("st-seat", "seat", SE, "member")
    for sg, nm in ((1, "l"), (-1, "r")):
        st(tube(f"stand-leg-{nm}", SD, [0, 0, sg * 0.11], [0.335, 0, sg * 0.11], 0.0125 if den <= 100 else 0.0225), SD, "member")
    st(cl("stand-foot", SD, PNT, 0.4, [0.335, 0, -0.11], [0.335, 0, 0.11], max(0.012, f * 1.01)), SD, "member")
    st(ellipsoid("head-bearing", ST, ALU, 0.5, [max(0.03, f * 1.01)] * 3, at=[0.06, 0.0, 0.0]), ST, "joint")
    if den <= 200:
        st(ellipsoid("swing-pivot", SW, ALU, 0.5, [max(0.02, f * 1.01)] * 3, at=[0.0, 0.02, 0.0]), SW, "joint")
    if den <= 200:
        st(ellipsoid("seat-hinge", SE, ALU, 0.5, [max(0.015, f * 1.01)] * 3, at=[0.04, 0.02, 0.0]), SE, "joint")
    st(world_box("contact-pad-f", ST, "rubber", 0.1, [0.08 if den == 500 else 0.07, max(0.0076, f * 1.01), 0.08 if den == 500 else 0.06], [0.6697, max(0.0076, f * 1.01) / 2 + 0.0002], anchor="contact-f"), ST, "support")
    st(ellipsoid("mass-engine", SW, ENG, 0.4, [max(0.07, f), 0.06, 0.07], at=[-0.3, 0.2, 0.0]), SW, "mass")
    st(ellipsoid("mass-frame", FR, PNT, 0.4, [max(0.06, f), 0.06, 0.06], at=[-0.4, 0.5, 0.0]), FR, "mass")
    for tag, bone in (("f", WF), ("r", WR)):
        st(ellipsoid(f"mass-wheel-{tag}", bone, ALU, 0.4, [max(0.05, f)] * 3, at=[0, 0, 0]), bone, "mass")



DEC = {
    50: (
        "1:50 models the vintage motor scooter from the drawn outlines of all three views: the bodywork as three lofted shells (rear body with seat deck and engine bay, floor tunnel, front leg shield with nose) whose sections follow the elevation outline, the plan half widths and the front half widths, the cowl as an extrusion of the drawn outline, headlamp lens, fork legs with crown, handlebar with grips, front mudguard as an arched channel, solid cast wheels as tyre torus with cast rim web, brake disc ring and hub, lofted seat, engine cover disc, muffler and tail pipe, and the folded side stand with pivot and foot bar. The drawn engine and exhaust below the bodywork are modelled as hidden thermal solids, not as body.",
        "1:50 modelliert den klassischen Motorroller aus den gezeichneten Umrissen aller drei Ansichten: die Karosserie als drei geloftete Schalen (Heck mit Sitzbank und Motorraum, Bodentunnel, vorderer Beinschild mit Nase), deren Schnitte dem Aufrissumriss, den Halbbreiten im Grundriss und den Halbbreiten der Vorderansicht folgen, die Lenkerverkleidung als Extrusion des gezeichneten Umrisses, Scheinwerferlinse, Gabelholme mit Krone, Lenker mit Griffen, Vorderkotflügel als Bogenrinne, massive Gussräder als Reifentorus mit gegossenem Felgenstern, Bremsscheibenring und Nabe, gelofteter Sitz, Motorabdeckung, Schalldämpfer und Auspuffendrohr sowie der eingeklappte Seitenständer mit Drehpunkt und Fußbügel. Der gezeichnete Motor und die Abgasanlage unter der Karosserie sind verdeckte Thermikkörper, kein Körper.",
        "Thermal: body panels and the steer assembly carry their two layers as nested shells (paint film as the reference to the body solid, the 0.8 mm steel wall inset by the 0.08 mm paint); the other regions are single references. The engine region has a real crankcase and a cylinder with head inside the bodywork besides the visible cover, the exhaust region a header pipe, the muffler and the tail pipe, and the tail pipe jet is the emitter solid at the pipe mouth. Corrected areas: body panels 2.58, seat 0.453, engine 0.255, tyres 1.12 m2; headlamp, exhaust system and brake discs are listed in the area review.",
        "Thermik: Karosserie und Lenkeinheit tragen ihre zwei Schichten als verschachtelte Schalen (Lackfilm als Verweis auf den Körper, die 0,8-mm-Stahlwand um den 0,08-mm-Lack eingerückt); die übrigen Regionen sind einfache Verweise. Die Motorregion hat neben der sichtbaren Abdeckung ein echtes Kurbelgehäuse und einen Zylinder mit Kopf in der Karosserie, die Abgasregion einen Krümmer, den Schalldämpfer und das Endrohr, und der Auspuffstrahl ist der Emitterkörper an der Rohrmündung. Korrigierte Flächen: Karosserie 2,58, Sitz 0,453, Motor 0,255, Reifen 1,12 m2; Scheinwerfer, Abgasanlage und Bremsscheiben stehen im Flächen-Review.",
        "Structure: tubular steel backbone from the steering head along the leg shield and floor to the rear rails (hollow 1.5 mm), steering column, fork legs and handlebar tube, the cast wheels as members with axle joints, two swing arm tubes with the crankcase, the seat, stand legs with foot bar, joints at head bearing, swing pivot and seat hinge, the front tyre patch as support (rear tyre and stand foot are pose-dependent contacts), masses for engine, frame and both wheels.",
        "Tragwerk: Stahlrohr-Rückgrat vom Lenkkopf über Beinschild und Boden zu den hinteren Holmen (hohl, 1,5 mm), Lenksäule, Gabelholme und Lenkerrohr, die Gussräder als Glieder mit Achsgelenken, zwei Schwingenrohre mit Kurbelgehäuse, der Sitz, Ständerbeine mit Fußbügel, Gelenke an Steuersatz, Schwingendrehpunkt und Sitzscharnier, die vordere Aufstandsfläche als Auflager (Hinterreifen und Ständerfuß sind lageabhängige Kontakte), Massen für Motor, Rahmen und beide Räder.",
    ),
    100: (
        "1:100 merges against the drawings of this scale: the bodywork lofts use five stations, brake discs, tail lamp, tail pipe and grips are gone (not drawn), each wheel is a tyre torus with a plain rim cylinder, the fork legs and handlebar are single swept parts, the mudguard a 15 mm channel, the stand two legs.",
        "1:100 fasst gegen die Zeichnungen dieses Maßstabs zusammen: Die Karosserielofts nutzen fünf Stationen, Bremsscheiben, Rücklicht, Endrohr und Griffe entfallen (nicht gezeichnet), jedes Rad ist ein Reifentorus mit einfachem Felgenzylinder, Gabelholme und Lenker sind einzelne Schwünge, das Schutzblech eine 15-mm-Rinne, der Ständer zwei Beine.",
        "Thermal: single references per body solid (the 15 mm feature is far above the 0.8 mm wall); the hidden engine block and the header are one real solid each, the tail pipe jet is the emitter.",
        "Thermik: einfache Verweise je Körper (das 15-mm-Merkmal liegt weit über der 0,8-mm-Wand); Motorblock und Krümmer sind je ein echter verdeckter Körper, der Auspuffstrahl ist der Emitter.",
        "Structure: the 1:50 load path on solid members at 15 mm (tubes no longer hollow), joints at head bearing, swing pivot and seat hinge, the front patch as support, masses as at 1:50.",
        "Tragwerk: der Lastpfad von 1:50 auf massiven Gliedern bei 15 mm (Rohre nicht mehr hohl), Gelenke an Steuersatz, Schwingendrehpunkt und Sitzscharnier, die vordere Aufstandsfläche als Auflager, Massen wie bei 1:50.",
    ),
    200: (
        "1:200 models what the drawings still show: bodywork as five-station shells, each wheel one solid 120 mm tyre disc with a hub drum, the fork as a 50 mm plate (the plan and front draw it as a block), handlebar with cowl cover as one 60 mm plate across the 740 mm width and the cowl extrusion, the seat with three sections and two stand legs; mudguard, lamps, exhaust and engine cover are below the drawing.",
        "1:200 modelliert, was die Zeichnungen noch zeigen: Karosserie als Schalen mit fünf Stationen, jedes Rad eine massive 120-mm-Reifenscheibe mit Nabentrommel, die Gabel als 50-mm-Platte (Grundriss und Vorderansicht zeichnen sie als Block), Lenker mit Verkleidung als eine 60-mm-Platte über die Breite von 740 mm samt Verkleidungsextrusion, der Sitz mit drei Schnitten und zwei Ständerbeine; Schutzblech, Lampen, Auspuff und Motorabdeckung liegen unter der Zeichnung.",
        "Thermal: references plus real solids for what the drawing no longer shows (headlamp lens, hidden engine block, header and the emitter); only the sum of the exposed areas counts at this scale.",
        "Thermik: Verweise plus echte Körper für das, was die Zeichnung nicht mehr zeigt (Scheinwerferlinse, verdeckter Motorblock, Krümmer und der Emitter); in diesem Maßstab zählt nur die Summe der freien Flächen.",
        "Structure: 40 mm tube members and refs to the body solids, joints at head bearing and both hubs, the front patch as 30 mm support, masses as at 1:50.",
        "Tragwerk: 40-mm-Rohrglieder und Verweise auf die Körper, Gelenke am Steuersatz und an beiden Naben, die vordere Aufstandsfläche als 30-mm-Auflager, Massen wie bei 1:50.",
    ),
    500: (
        "1:500 is the mass model: bodywork shells with the 75 mm floor and the cowl merged into the front shell as drawn, two 80 mm wheel discs, the fork as a 75 mm plate, the handlebar plate, the seat; no stand or hubs.",
        "1:500 ist das Massenmodell: Karosserieschalen mit dem 75-mm-Boden und der Verkleidung in der vorderen Schale wie gezeichnet, zwei 80 mm breite Radscheiben, die Gabel als 75-mm-Platte, die Lenkerplatte, der Sitz; kein Ständer, keine Naben.",
        "Thermal: references plus real solids for headlamp, brake discs, hidden engine block, header and the emitter at the 75 mm floor; only the total counts.",
        "Thermik: Verweise plus echte Körper für Scheinwerfer, Bremsscheiben, verdeckten Motorblock, Krümmer und den Emitter auf dem 75-mm-Mindestmaß; es zählt nur die Summe.",
        "Structure: 78 mm tube members and refs to the body solids, head bearing and hub joints, the front patch as 75 mm support, masses as at 1:50.",
        "Tragwerk: 78-mm-Rohrglieder und Verweise auf die Körper, Gelenke an Steuersatz und Naben, die vordere Aufstandsfläche als 75-mm-Auflager, Massen wie bei 1:50.",
    ),
}


def main(den):
    B = build50() if den == 50 else build_coarse(den)
    if den == 50:
        thermal50(B)
    else:
        thermal_by(B, regions)
        hidden_engine(B, den)
        thermal_extra(B, den)
    structure(B, den)
    dec(B, DEC[den])
    print(write_scale(SLUG, den, B))
    print(check(SLUG, den))


if __name__ == "__main__":
    main(int(sys.argv[1]))
