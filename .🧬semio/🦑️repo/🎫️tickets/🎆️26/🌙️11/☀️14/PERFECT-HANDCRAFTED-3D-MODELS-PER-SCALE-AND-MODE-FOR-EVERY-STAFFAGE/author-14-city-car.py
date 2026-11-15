import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_14_kit import *
from author_14_lib import *

SLUG = "city-car"
base.frames(SLUG)
D = Sil(SLUG)
CH = "chassis"
BODY_IDS = ("body-shell", "well-front", "well-rear", "roof-band", "side-glass-front", "side-glass-rear", "engine-bay-top", "headlamp", "bumper-rear-lamp", "greenhouse")


def esel(s):
    return s["bone"] == CH and s["id"] in BODY_IDS


def psel(s):
    return s["bone"] in (CH, "bonnet", "hatch") and "mirror" not in s["id"] and s["id"] not in ("tail-pipe", "steering-wheel", "seat-front-headrest-l", "seat-front-headrest-r", "tail-lamp-l", "tail-lamp-r", "headlamp-l", "headlamp-r")


def fsel(s):
    return s["bone"] == CH and s["id"] in ("body-shell", "roof-band", "windscreen", "bumper")


def wheel_set(sc, level):
    out = []
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            out += wheel_stack(f"{p}{n}", bone, sgn * 0.72, 0.3, 0.0925, 0.19, 0.075, 0.045, 5, 0.014, flip=sgn)
    return out


def side_window(id, stations, bottom, side, material="glass"):
    secs = []
    wb, wt = 0.712, 0.6
    dy = 0.415
    n = math.hypot(dy, wt - wb)
    ax = [0.0, dy / n, side * (wt - wb) / n]
    for x, top in stations:
        h = max(top - bottom, 0.02)
        yc = bottom + h / 2
        wc = side * (wb + (wt - wb) * (yc - 1.04) / 0.415)
        rx = h / 2 * n / dy
        y = cross3([1.0, 0.0, 0.0], ax)
        secs.append(((x, yc, wc), rx, 0.011, 8, [r(c) for c in qbasis(ax, y, [1.0, 0.0, 0.0])]))
    return L(id, CH, material, "glazing", 0.85, secs)


def hull_top(n, x, y):
    if x < 0.93:
        return min(y, 0.99)
    if 0.93 <= x < 0.95:
        return min(y, 0.93)
    if x >= 0.95:
        b = D.col(n, "elevation", x, lambda s: s["bone"] == "bonnet")
        return min(y, min(a for a, _ in b) - 0.1) if b else min(y, 0.7)
    return min(y, 0.99)


def car_shell(sc, tol, e_low, e_cab):
    n = sc.n
    low = auto_x(D, n, -1.95, 1.945, esel, psel, tol, ytop=lambda x, y: hull_top(n, x, y))
    sc.add(xloft("body-lower", CH, "paint", "body", 0.62, low, e=e_low))
    blk = auto_x(D, n, 0.96, 1.9, lambda s: s["bone"] == "bonnet", psel, tol * 3)
    sc.add(xloft("engine-block", CH, "engine-metal", "detail", 0.12, [(x, hull_top(n, x, 9) , min(y1 - 0.0012, 9) , 0.69) for x, y0, y1, hw in blk if D.col(n, "elevation", x, lambda s: s["bone"] == "bonnet")], e=20))
    cab = auto_y(D, n, 0.98, 1.497, lambda s: s["bone"] == CH and s["id"] in ("body-shell", "roof-band", "greenhouse"), fsel, tol)
    sc.add(vloft("cabin", CH, "paint", "body", 0.58, cab, e=e_cab))


def build50(K):
    sc = Scale(SLUG, 50, K)
    car_shell(sc, 0.003, 40, 40)
    sc.add(L("roof-skin", CH, "paint", "body", 0.5, [((0.22, 1.468, 0), 0.5, 0.02, 6), ((-0.2, 1.485, 0), 0.52, 0.015, 6), ((-0.9, 1.486, 0), 0.52, 0.015, 6), ((-1.3, 1.46, 0), 0.5, 0.017, 6), ((-1.46, 1.405, 0), 0.46, 0.017, 6)]))
    sc.add(L("windscreen", CH, "glass", "glazing", 0.85, [((0.9, 1.012, 0), 0.7, 0.012, 8), ((0.26, 1.432, 0), 0.6, 0.012, 8)]))
    front_win = [(0.7, 1.05), (0.5, 1.2), (0.3, 1.405), (0.16, 1.448), (0.0, 1.455), (-0.02, 1.455)]
    rear_win = [(-0.14, 1.452), (-0.5, 1.456), (-0.9, 1.452), (-1.22, 1.435), (-1.3, 1.325), (-1.34, 1.166), (-1.347, 1.05)]
    for s_, n in ((1, "l"), (-1, "r")):
        sc.add(side_window("side-glass-front-" + n, front_win, 1.04, s_), side_window("side-glass-rear-" + n, rear_win, 1.04, s_))
        sc.add(X("quarter-" + n, CH, "paint", "body", 0.58, [(-1.49, 0.8), (-1.49, 1.43), (-1.62, 1.31), (-1.78, 1.09), (-1.94, 0.84), (-1.94, 0.8)], "xy", 0.5 if s_ > 0 else -0.58, 0.58 if s_ > 0 else -0.5))
    sc.add(B("boot-floor", CH, "interior-trim", "detail", 0.3, (0.45, 0.04, 1.24), at=(-1.72, 0.82, 0)))
    sc.add(B("grille", CH, "engine-metal", "detail", 0.3, (0.03, 0.08, 0.8), at=(1.94, 0.57, 0), corner=0.01))
    sc.add(L("bumper-front", CH, "paint", "detail", 0.45, [((1.56, 0.38, 0), 0.74, 0.08, 3.4), ((1.9, 0.38, 0), 0.6, 0.08, 3.4), ((1.945, 0.38, 0), 0.4, 0.078, 3.4)]))
    sc.add(L("bumper-rear", CH, "paint", "detail", 0.45, [((-1.58, 0.38, 0), 0.74, 0.08, 3.4), ((-1.9, 0.38, 0), 0.62, 0.08, 3.4), ((-1.945, 0.38, 0), 0.45, 0.078, 3.4)]))
    for s_, n in ((1, "l"), (-1, "r")):
        sc.add(E("headlamp-" + n, CH, "glass", "glazing", 0.9, (0.12, 0.05, 0.1), at=(1.82, 0.725, s_ * 0.6)))
        sc.add(E("mirror-" + n, CH, "plastic-trim", "detail", 0.35, (0.07, 0.045, 0.04), at=(0.86, 1.07, s_ * 0.8)))
        sc.add(C("mirror-arm-" + n, CH, "plastic-trim", "detail", 0.35, (0.82, 1.04, s_ * 0.7), (0.84, 1.06, s_ * 0.78), 0.012))
        sc.add(C("door-handle-front-" + n, CH, "plastic-trim", "detail", 0.3, (0.28, 0.94, s_ * 0.755), (0.36, 0.94, s_ * 0.755), 0.012))
        sc.add(C("door-handle-rear-" + n, CH, "plastic-trim", "detail", 0.3, (-0.63, 0.94, s_ * 0.755), (-0.55, 0.94, s_ * 0.755), 0.012))
        sc.add(B("seat-front-back-" + n, CH, "interior-trim", "detail", 0.3, (0.12, 0.2, 0.26), at=(0.02, 1.08, s_ * 0.29), corner=0.03))
        sc.add(B("seat-front-headrest-" + n, CH, "interior-trim", "detail", 0.3, (0.1, 0.16, 0.18), at=(-0.02, 1.26, s_ * 0.29), corner=0.03))
        sc.add(B("seat-front-cushion-" + n, CH, "interior-trim", "detail", 0.3, (0.5, 0.1, 0.4), at=(0.28, 0.99, s_ * 0.29), corner=0.03))
    sc.add(B("seat-rear-back", CH, "interior-trim", "detail", 0.3, (0.14, 0.17, 1.1), at=(-0.6, 1.07, 0), corner=0.03))
    sc.add(B("seat-rear-headrest", CH, "interior-trim", "detail", 0.3, (0.1, 0.15, 0.2), at=(-0.63, 1.2, 0), corner=0.03))
    sc.add(V("steering-wheel", CH, "interior-trim", "detail", 0.2, [(-0.012, 0.09), (0.012, 0.09)], axis=(0, 0, 1), origin=(0.56, 1.09, 0.29)))
    sc.add(C("exhaust-pipe", CH, "exhaust-steel", "detail", 0.35, (-1.1, 0.12, -0.35), (0.35, 0.12, -0.35), 0.03))
    sc.add(C("silencer", CH, "exhaust-steel", "detail", 0.4, (-1.7, 0.13, -0.35), (-1.1, 0.13, -0.35), 0.075, lateral=1.33))
    sc.add(C("tail-pipe", CH, "exhaust-steel", "detail", 0.45, (-1.7, 0.15, -0.3), (-1.925, 0.25, -0.3), 0.03, 0.025))
    for ax in ("axle-f", "axle-r"):
        sc.add(C(ax + "-beam", ax, "cast-iron", "detail", 0.35, (0, 0, -0.64), (0, 0, 0.64), 0.04))
    sc.add(*wheel_set(sc, 50))
    base.frames(SLUG)
    Bn = "bonnet"
    sc.add(L("bonnet-slab", Bn, "paint", "body", 0.62, [(loc(Bn, (0.95, 0.96, 0)), 0.68, 0.03, 8), (loc(Bn, (1.529, 0.8755, 0)), 0.67, 0.027, 8), (loc(Bn, (1.88, 0.8, 0)), 0.64, 0.027, 8)]))
    H = "hatch"
    sc.add(L("hatch-slab", H, "paint", "body", 0.62, [(loc(H, (-1.49, 1.43, 0)), 0.58, 0.03, 8), (loc(H, (-1.915, 0.82, 0)), 0.7, 0.03, 8)]))
    sc.add(L("hatch-glass", H, "glass", "glazing", 0.85, [(loc(H, (-1.51 - 0.012, 1.41 + 0.01, 0)), 0.48, 0.014, 8), (loc(H, (-1.8, 1.05, 0)), 0.48, 0.014, 8)]))
    for s_, n in ((1, "l"), (-1, "r")):
        sc.add(B("tail-lamp-" + n, H, "plastic-trim", "detail", 0.7, (0.12, 0.03, 0.22), at=(0.64, -0.035, s_ * 0.61)))
    return sc



def panels(sc):
    """Bonnet and tailgate panels on their hinged bones: lofts through the outline that scale draws (thick panel of the coarser drawings)."""
    n = sc.n
    shape = {100: ("bonnet-panel", "tailgate-panel"), 200: ("bonnet-panel", "tailgate-panel"), 500: ("bonnet-panel", "tailgate-panel")}[n]
    sc.add(panel_loft("bonnet-panel", D, n, shape[0], "bonnet", [-0.04, 0.2, 0.5, 0.8, 0.9, 0.94], lambda u: 0.68, "paint", "body", 0.62))
    sc.add(panel_loft("hatch-panel", D, n, shape[1], "hatch", [-0.04, 0.2, 0.49, 0.72, 0.78], lambda u: 0.58 + 0.12 * min(1, (u + 0.05) / 0.8), "paint", "body", 0.62))


def build100(K):
    sc = Scale(SLUG, 100, K)
    car_shell(sc, 0.006, 18, 14)
    sc.add(L("roof-skin", CH, "paint", "body", 0.5, [((0.22, 1.468, 0), 0.5, 0.02, 6), ((-0.9, 1.486, 0), 0.52, 0.02, 6), ((-1.46, 1.405, 0), 0.46, 0.02, 6)]))
    sc.add(L("windscreen", CH, "glass", "glazing", 0.85, [((0.9, 1.012, 0), 0.7, 0.015, 8), ((0.26, 1.432, 0), 0.6, 0.015, 8)]))
    for s_, n in ((1, "l"), (-1, "r")):
        sc.add(side_window("side-glass-front-" + n, [(0.7, 1.05), (0.5, 1.2), (0.3, 1.405), (0.0, 1.455), (-0.02, 1.455)], 1.04, s_), side_window("side-glass-rear-" + n, [(-0.14, 1.452), (-0.9, 1.452), (-1.22, 1.435), (-1.3, 1.325), (-1.347, 1.05)], 1.04, s_))
        sc.add(E("mirror-" + n, CH, "plastic-trim", "detail", 0.35, (0.07, 0.045, 0.04), at=(0.86, 1.07, s_ * 0.8)))
        sc.add(E("headlamp-" + n, CH, "glass", "glazing", 0.9, (0.1, 0.05, 0.1), at=(1.74, 0.725, s_ * 0.6)))
    sc.add(B("seat-front", CH, "interior-trim", "detail", 0.3, (0.5, 0.4, 0.9), at=(0.2, 1.1, 0), corner=0.05))
    sc.add(B("seat-rear", CH, "interior-trim", "detail", 0.3, (0.3, 0.4, 1.1), at=(-0.62, 1.1, 0), corner=0.05))
    sc.add(C("exhaust-pipe", CH, "exhaust-steel", "detail", 0.35, (-0.9, 0.16, -0.35), (0.35, 0.16, -0.35), 0.03))
    sc.add(C("silencer", CH, "exhaust-steel", "detail", 0.4, (-1.7, 0.14, -0.35), (-1.0, 0.14, -0.35), 0.07, lateral=1.43))
    sc.add(C("tail-pipe", CH, "exhaust-steel", "detail", 0.45, (-1.7, 0.16, -0.3), (-1.925, 0.25, -0.3), 0.03, 0.025))
    for ax in ("axle-f", "axle-r"):
        sc.add(C(ax + "-beam", ax, "cast-iron", "detail", 0.35, (0, 0, -0.64), (0, 0, 0.64), 0.04))
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            sc.add(*wheel_stack(f"{p}{n}", bone, sgn * 0.72, 0.3, 0.0925, 0.19, 0.075, 0.045, 5, 0.02, flip=sgn, parts=("tyre", "rim", "hub")))
    base.frames(SLUG)
    Bn, H = "bonnet", "hatch"
    panels(sc)
    sc.add(L("hatch-glass", H, "glass", "glazing", 0.85, [(loc(H, (-1.51 - 0.012, 1.41 + 0.01, 0)), 0.48, 0.016, 8), (loc(H, (-1.8, 1.05, 0)), 0.48, 0.016, 8)]))
    return sc


def build200(K):
    sc = Scale(SLUG, 200, K)
    car_shell(sc, 0.012, 18, 14)
    sc.add(L("windscreen", CH, "glass", "glazing", 0.85, [((0.9, 1.012, 0), 0.7, 0.03, 8), ((0.26, 1.432, 0), 0.6, 0.03, 8)]))
    for s_, n in ((1, "l"), (-1, "r")):
        sc.add(side_window("side-glass-" + n, [(0.7, 1.05), (0.3, 1.405), (0.0, 1.455), (-0.9, 1.452), (-1.22, 1.435), (-1.3, 1.325), (-1.347, 1.05)], 1.04, s_))
        sc.add(E("mirror-" + n, CH, "plastic-trim", "detail", 0.35, (0.07, 0.045, 0.04), at=(0.86, 1.07, s_ * 0.8)))
    sc.add(B("seats", CH, "interior-trim", "detail", 0.3, (1.1, 0.4, 0.9), at=(-0.2, 1.1, 0), corner=0.05))
    sc.add(C("exhaust-pipe", CH, "exhaust-steel", "detail", 0.35, (-0.9, 0.16, -0.35), (0.35, 0.16, -0.35), 0.035))
    sc.add(C("silencer", CH, "exhaust-steel", "detail", 0.4, (-1.75, 0.2, -0.35), (-1.0, 0.2, -0.35), 0.08, lateral=1.25))
    for ax in ("axle-f", "axle-r"):
        sc.add(C(ax + "-beam", ax, "cast-iron", "detail", 0.35, (0, 0, -0.64), (0, 0, 0.64), 0.05))
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            sc.add(*wheel_stack(f"{p}{n}", bone, sgn * 0.72, 0.3, 0.0925, 0.19, 0.075, 0.045, 0, 0.02, flip=sgn, parts=("tyre", "rim")))
    base.frames(SLUG)
    Bn, H = "bonnet", "hatch"
    panels(sc)
    return sc


def build500(K):
    sc = Scale(SLUG, 500, K)
    car_shell(sc, 0.03, 18, 12)
    for s_, n in ((1, "l"), (-1, "r")):
        pass
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            sc.add(V(f"{p}{n}-wheel", bone, "rubber", "body", 0.1, [(-0.0925, 0.3), (0.0925, 0.3)], axis=(0, 0, 1), origin=[0, 0, sgn * 0.72]))
    base.frames(SLUG)
    Bn, H = "bonnet", "hatch"
    panels(sc)
    return sc


SIDE = [(-1.88, 0.45), (-1.58, 0.45), (-1.5, 0.6), (-1.2, 0.69), (-0.9, 0.58), (-0.84, 0.4), (-0.8, 0.3), (0.85, 0.3), (0.87, 0.45), (0.93, 0.6), (1.25, 0.69), (1.6, 0.58), (1.66, 0.45), (1.88, 0.45), (1.88, 0.78), (1.0, 0.93), (0.4, 0.98), (-1.0, 0.98), (-1.5, 0.97), (-1.88, 0.84)]
LAYERS = [("sheet", 0.0008, "paint"), ("paint", 0.00008, "paint")]


def thermal_skins(sc, layered):
    """Painted sheet metal regions: outer face plates on the real panel positions (side, fascia, roof, bonnet, tailgate), nested layers at 1:50."""
    lay = LAYERS if layered else [("paint", 0.0008, "paint")]
    one = lambda t: [(lay[-1][0], max(t, sc.feature), "paint")]
    L_ = lay if layered else one(0.0008)
    skin(sc, "body-panels", L_, [("side-l", CH, "xy", SIDE, 0.7615, 1), ("side-r", CH, "xy", SIDE, -0.7615, -1), ("front", CH, "yz", [(0.45, -0.5), (0.45, 0.5), (0.78, 0.5), (0.78, -0.5)], 1.9515, 1), ("rear", CH, "yz", [(0.45, -0.55), (0.45, 0.55), (0.78, 0.55), (0.78, -0.55)], -1.9515, -1)])
    skin(sc, "roof", L_, [("roof", CH, "xz", [(-0.5, 0.2), (0.5, 0.2), (0.5, -1.4), (-0.5, -1.4)], 1.4985, 1)])
    skin(sc, "bonnet", L_, [("bonnet", "bonnet", "xz", [(-0.6, 0.05), (0.6, 0.05), (0.6, 0.9), (-0.6, 0.9)], 0.0575, 1)])
    skin(sc, "tailgate", L_, [("tailgate", "hatch", "xz", [(-0.5, 0.1), (0.5, 0.1), (0.6, 0.7), (-0.6, 0.7)], -0.0575, -1)])


def thermal_common(sc, glass_ids, seat_ids, tyre_ids):
    f = sc.f("glazing")
    for g in glass_ids:
        src = next(x for x in sc.body if x["id"] == g)
        t = proud(src, "th-" + g, 0.006)
        for sec in t["sections"]:
            sec["rx"] = r(sec["rx"] * f)
        sc.th(t, "glazing")
    for g in seat_ids:
        sc.th_of(g, "interior")
    f = sc.f("interior")
    sc.th(B("th-dash-liner", CH, "interior-trim", "detail", 0.3, (0.3 * f, 0.15 * f, 1.0 * f), at=(0.55, 1.0, 0)), "interior")
    sc.th(B("th-floor-liner", CH, "interior-trim", "detail", 0.3, (1.6 * f, 0.03, 1.1 * f), at=(-0.5, 0.45, 0)), "interior")
    sc.th_of("engine-block", "engine-bay")
    f = sc.f("exhaust-system")
    sc.th(C("th-exhaust-line", CH, "exhaust-steel", "detail", 0.35, (-1.0, 0.14, -0.35), (0.35, 0.14, -0.35), 0.03 * f), "exhaust-system")
    sc.th(C("th-silencer-can", CH, "exhaust-steel", "detail", 0.4, (-1.7, 0.14, -0.35), (-1.0, 0.14, -0.35), 0.07 * f, lateral=1.4), "exhaust-system")
    sc.th(C("th-tailpipe-nozzle", CH, "exhaust-steel", "detail", 0.45, (-1.7, 0.16, -0.3), (-1.925, 0.25, -0.3), 0.03, 0.025), "exhaust-system", tags=["tailpipe"])
    f = sc.f("tyres")
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            sc.th(V(f"th-tread-{p}{n}", bone, "rubber", "body", 0.1, tyre_profile(0.3 * f, 0.0925), axis=(0, 0, 1), origin=[0, 0, sgn * 0.72]), "tyres")
    f = sc.f("brakes")
    for sgn, n in ((1, "l"), (-1, "r")):
        for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
            sc.th(V(f"th-brake-{p}{n}", bone, "cast-iron", "detail", 0.35, [(-0.01, 0.15 * f), (0.01, 0.15 * f)], axis=(0, 0, 1), origin=[0, 0, sgn * 0.62]), "brakes")


def structure_common(sc, level):
    f = 1 if level <= 100 else 1.2
    for sgn, n in ((1, "l"), (-1, "r")):
        sc.st(B(f"sill-{n}", CH, "cast-iron", "detail", 0.5, (3.4, 0.12 * f, 0.1 * f), at=(0.0, 0.34, sgn * 0.6)), "chassis", "member")
    for x, nm in ((1.25, "front-crossmember"), (-1.2, "rear-crossmember")):
        sc.st(B(nm, CH, "cast-iron", "detail", 0.5, (0.1 * f, 0.1 * f, 1.2), at=(x, 0.34, 0)), "chassis", "member")
    if level <= 100:
        for sgn, n in ((1, "l"), (-1, "r")):
            sc.st(C(f"roof-rail-{n}", CH, "cast-iron", "detail", 0.5, (-1.3, 1.44, sgn * 0.52), (0.25, 1.44, sgn * 0.52), 0.04), "chassis", "member")
            sc.st(C(f"a-pillar-{n}", CH, "cast-iron", "detail", 0.5, (0.78, 1.05, sgn * 0.6), (0.25, 1.42, sgn * 0.52), 0.035), "chassis", "member")
    sc.st(E("com-chassis", CH, "cast-iron", "detail", 0.5, (0.3, 0.12, 0.3), at=(0.38, 0.55, 0)), "chassis", "mass")
    sc.st(B("bonnet-rib", "bonnet", "cast-iron", "detail", 0.5, (0.85, 0.03, 0.6), at=(0.5, -0.02, 0)), "bonnet", "member")
    sc.st(B("hatch-rib", "hatch", "cast-iron", "detail", 0.5, (0.7, 0.03, 0.8), at=(0.4, 0.02, 0)), "hatch", "member")
    if level <= 100:
        for z, n in ((0.5, "l"), (-0.5, "r")):
            sc.st(E("bonnet-hinge-" + n, "bonnet", "cast-iron", "detail", 0.5, (0.05, 0.05, 0.05), at=(0, 0, z)), "bonnet", "joint")
            sc.st(E("hatch-hinge-" + n, "hatch", "cast-iron", "detail", 0.5, (0.05, 0.05, 0.05), at=(0, 0, z)), "hatch", "joint")
    for ax in ("axle-f", "axle-r"):
        sc.st(C(ax + "-beam-member", ax, "cast-iron", "detail", 0.35, (0, 0, -0.64), (0, 0, 0.64), 0.04 if level <= 100 else 0.05), ax, "member")
        sc.st(E(ax + "-mass", ax, "cast-iron", "detail", 0.35, (0.09, 0.09, 0.09), at=(0, 0, 0)), ax, "mass")
        for z, n in ((0.5, "l"), (-0.5, "r")):
            sc.st(E(f"{ax}-spring-{n}", ax, "cast-iron", "detail", 0.5, (0.06, 0.1, 0.06), at=(0, 0.1, z)), ax, "joint")
    for bone in ("wheel-f", "wheel-r"):
        for sgn, n in ((1, "l"), (-1, "r")):
            sc.st(V(f"{bone}-rim-{n}", bone, "alloy", "detail", 0.55, [(-0.075, 0.19), (0.075, 0.19)], axis=(0, 0, 1), origin=[0, 0, sgn * 0.72]), bone, "member")
            sc.st(V(f"{bone}-hub-{n}", bone, "alloy", "detail", 0.7, [(-0.09, 0.045), (0.09, 0.045)], axis=(0, 0, 1), origin=[0, 0, sgn * 0.72]), bone, "joint")
        sc.st(E(bone + "-mass", bone, "rubber", "detail", 0.1, (0.1, 0.1, 0.1), at=(0, 0, 0.72)), bone, "mass")


DEC = {
    50: [("Detailed model: the lower body is a loft along the car with superelliptic sections read from this scale's own elevation and plan outline every 3 mm of tolerance (wheel arches, sill step, nose), the cabin a loft of horizontal slices so that elevation and the tumblehome of the front view are exact; windscreen, side, tailgate glass, bumpers, lamps, mirrors, seats, steering wheel, exhaust line, axle beams and five-spoke wheels are separate solids on their bones.",
          "Detailliertes Modell: Der Unterbau ist ein Loft entlang des Fahrzeugs mit superelliptischen Querschnitten, die aus dem Seitenriss und Grundriss dieses Massstabs mit 3 mm Toleranz abgelesen sind (Radläufe, Schwellerstufe, Front); die Kabine ist ein Loft aus horizontalen Schnitten, damit Seitenriss und Einzug der Vorderansicht stimmen; Front-, Seiten- und Heckscheibe, Stossfänger, Leuchten, Spiegel, Sitze, Lenkrad, Abgasstrang, Achsbalken und fünfspeichige Räder sind eigene Körper an ihren Knochen."),
         ("Thermal: every sheet-metal region carries nested layer plates (steel sheet 0.8 mm, paint film 0.08 mm, outermost last) on the real panel positions; plate size is calibrated to the exposed area of the region; glazing, seats and tyres reference body solids, the engine block, exhaust line, silencer, tailpipe nozzle and brake discs are own solids.",
          "Thermik: Jede Blechregion trägt verschachtelte Schichtplatten (Stahlblech 0,8 mm, Lackfilm 0,08 mm, aussen zuletzt) an den echten Blechpositionen; die Plattengrösse ist auf die exponierte Fläche der Region kalibriert; Verglasung, Sitze und Reifen verweisen auf Körper, Motorblock, Abgasstrang, Schalldämpfer, Auspuffdüse und Bremsscheiben sind eigene Körper."),
         ("Structure: sills, front and rear cross members and roof rails with A-pillars carry the unibody, rib frames the bonnet and tailgate, an axle beam with spring joints each axle, rim and hub the wheels; masses sit at the centre of mass of the body and at each axle and wheel.",
          "Tragwerk: Schweller, vordere und hintere Querträger sowie Dachholme mit A-Säulen tragen die Karosserie, Rippenrahmen die Motorhaube und Heckklappe, ein Achsbalken mit Federgelenken jede Achse, Felge und Nabe die Räder; Massen sitzen im Schwerpunkt der Karosserie sowie an jeder Achse und jedem Rad.")],
    100: [("Remodelled against the 1:100 drawing: hull and cabin lofts use fewer stations (6 mm tolerance), side glass is one pane per door window, seats are two blocks, wheels are tyre, rim disc and hub without spokes, the bonnet and tailgate panels are the thicker outline the 1:100 drawing shows.",
           "Neu modelliert gegen die 1:100-Zeichnung: Unterbau- und Kabinen-Loft mit weniger Stationen (6 mm Toleranz), eine Seitenscheibe je Türfenster, Sitze als zwei Blöcke, Räder aus Reifen, Felgenscheibe und Nabe ohne Speichen, Haube und Heckklappe als dickere Kontur, wie sie die 1:100-Zeichnung zeigt."),
          ("Thermal and structure follow the 1:50 logic with single-layer plates and fewer joints.", "Thermik und Tragwerk folgen der 1:50-Logik mit einschichtigen Platten und weniger Gelenken.")],
    200: [("Remodelled against the 1:200 drawing: merged side glass per side, one seat block, exhaust as pipe and silencer, wheels as tyre and rim; mirrors kept because the 1:200 drawing shows them.",
           "Neu modelliert gegen die 1:200-Zeichnung: eine Seitenscheibe je Seite, ein Sitzblock, Abgas als Rohr und Schalldämpfer, Räder aus Reifen und Felge; Spiegel bleiben, weil die 1:200-Zeichnung sie zeigt."),
          ("Thermal plates are single-layer; structure keeps the sills, cross members, rib frames, axle beams and wheel rims at heavier sections.", "Thermik-Platten sind einschichtig; das Tragwerk behält Schweller, Querträger, Rippenrahmen, Achsbalken und Felgen mit kräftigeren Querschnitten.")],
    500: [("Minimal model: hull and greenhouse lofts, bonnet and tailgate panels and four wheel cylinders; glass is part of the greenhouse, no interior or exhaust.",
           "Minimales Modell: Unterbau- und Glashaus-Loft, Haube und Heckklappe sowie vier Radzylinder; das Glas gehört zum Glashaus, kein Innenraum und kein Abgasstrang."),
          ("Thermal regions without a drawn body part (interior, exhaust) use own compact solids inside the hull; structure is the sills, cross members, rib frames, axle beams and wheel rims.", "Thermikregionen ohne gezeichneten Körper (Innenraum, Abgas) nutzen eigene kompakte Körper im Unterbau; das Tragwerk besteht aus Schwellern, Querträgern, Rippenrahmen, Achsbalken und Felgen.")],
}


def finish(sc, level):
    for en, de in DEC[level]:
        sc.dec(en, de)
    ids = {s["id"] for s in sc.body}
    glass = [g for g in ("windscreen", "side-glass-front-l", "side-glass-front-r", "side-glass-rear-l", "side-glass-rear-r", "side-glass-l", "side-glass-r", "hatch-glass") if g in ids]
    seats = [g for g in ids if g.startswith("seat-")]
    tyres = [g for g in ids if g.endswith("-tyre") or g.endswith("-wheel")]
    thermal_skins(sc, level == 50)
    thermal_common(sc, glass, sorted(seats), sorted(tyres))
    structure_common(sc, level)
    return sc


def build(K):
    return {n: finish(f(K), n) for n, f in ((50, build50), (100, build100), (200, build200), (500, build500))}


if __name__ == "__main__":
    K = calibrate(SLUG, build)
    print(K)
    print(check(SLUG, ("--scale", "50")))
