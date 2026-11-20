"""✏️ Round 3 shopping trolley 2D: the side elevation, front elevation and plan of every scale redrawn as the true orthographic views of the real construction (ruling E1), per bone, from the same hand-stated geometry as the solids."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

BW, GW, FT, HG, WH = "basket-wire", "gate-wire", "frame-tube", "handle-grip", "wheels"
ZT, ZW, PP, PU = "zinc-tube", "zinc-wire", "pp-grip", "pu-tyre"
MESH = 0.55
SIDES = (("l", 1), ("r", -1))
BA = k.BONES["basket"]["origin"]


def C(x, y):
    return k.local("chassis", (x, y))


def B(x, y):
    return k.local("basket", (x, y))


def Hh(x, y):
    return k.local("handle", (x, y))


def bw(p):
    return [BA[0] + p[0], BA[1] + p[1], p[2]]


def frame_line():
    return [(k.RAIL_X_FRONT, k.RAIL_Y), (k.RAIL_X_REAR, k.RAIL_Y)] + list(k.BEND) + [k.UPRIGHT_TOP]


def pen(scale, true_m, floor=0.13):
    return round(max(floor, true_m * 1000.0 / scale), 3)


GATE_T = {50: 0.003, 100: 0.0045, 200: 0.0075, 500: 0.0075}


def gate_solid():
    return {"kind": "extruded", "sweep": "w", "z0": -0.252, "z1": 0.252, "zTip": [-0.208, 0.208]}


def gate_band(t):
    return [[0.012, -t], [0.325, -t], [0.325, t], [0.012, t]]


def gate_shape(scale, t):
    return k.poly2("gate", "gate", "glazing", 3.4, True, MESH, gate_band(t), True, region=GW, material=ZW, solid=gate_solid())


def basket_side_outline(e):
    fr, ff, rf, rr = k.side_corners(1)
    return [[fr[0] - e, fr[1] - e], [ff[0] + e * 0.5, ff[1] - e], [rf[0] + e, rf[1] + e], [rr[0] - e, rr[1] + e]]


def front_outline(e):
    return [[-k.RIM_REAR[2] - e, BA[1] + k.RIM_REAR[1] + e], [k.RIM_REAR[2] + e, BA[1] + k.RIM_REAR[1] + e], [k.FLOOR_REAR[2] + e, BA[1] - e], [-k.FLOOR_REAR[2] - e, BA[1] - e]]


def plan_rim():
    return [[BA[0] + k.RIM_REAR[0], k.RIM_REAR[2]], [BA[0] + k.RIM_FRONT[0], k.RIM_FRONT[2]], [BA[0] + k.RIM_FRONT[0], -k.RIM_FRONT[2]], [BA[0] + k.RIM_REAR[0], -k.RIM_REAR[2]]]


def plan_floor():
    return [[BA[0] + k.FLOOR_REAR[0], k.FLOOR_REAR[2]], [BA[0] + k.FLOOR_FRONT[0], k.FLOOR_FRONT[2]], [BA[0] + k.FLOOR_FRONT[0], -k.FLOOR_FRONT[2]], [BA[0] + k.FLOOR_REAR[0], -k.FLOOR_REAR[2]]]


def grow(poly, e):
    cx = sum(p[0] for p in poly) / len(poly)
    cy = sum(p[1] for p in poly) / len(poly)
    out = []
    for x, y in poly:
        dx, dy = x - cx, y - cy
        out.append([x + (e if dx > 0 else -e), y + (e if dy > 0 else -e)])
    return out


def castor_elev(pos, scale, detail):
    c = k.CASTORS[pos]
    xs = c["xs"]
    xw = xs - k.TRAIL
    out = []
    if scale <= 100:
        out.append(k.ellipse2("wheel-%s" % pos, "chassis", "body", 2.5, True, 0.18, *C(xw, k.WHEEL_R), k.WHEEL_R, k.WHEEL_R, region=WH, material=PU))
        out.append(k.ellipse2("hub-%s" % pos, "chassis", "detail", 2.55, True, 0.78, *C(xw, k.WHEEL_R), k.TYRE_IN, k.TYRE_IN, region=WH))
        if detail:
            out.append(k.ellipse2("tread-%s" % pos, "chassis", "texture", 2.56, False, 0.3, *C(xw, k.WHEEL_R), 0.056, 0.056, region=WH))
        fork = [C(x, y) for x, y in k.fork_profile(xs)]
        out.append(k.poly2("fork-%s" % pos, "chassis", "body", 2.6, True, 0.6, fork, True, region=FT, material=ZT))
        if detail:
            out.append(k.ellipse2("axle-%s" % pos, "chassis", "detail", 2.65, True, 0.85, *C(xw, k.WHEEL_R), 0.006, 0.006, region=FT))
            out.append(k.rect2("castor-head-%s" % pos, "chassis", "body", 2.62, True, 0.5, *C(xs - 0.0285, k.HEAD_Y[0]), 0.057, k.HEAD_Y[1] - k.HEAD_Y[0], region=FT, material=ZT))
            out.append(k.rect2("castor-plate-%s" % pos, "chassis", "body", 2.63, True, 0.7, *C(xs - 0.035, k.PLATE_Y[0]), 0.07, k.PLATE_Y[1] - k.PLATE_Y[0], region=FT, material=ZT))
        else:
            out.append(k.rect2("castor-head-%s" % pos, "chassis", "body", 2.62, True, 0.5, *C(xs - 0.035, k.HEAD_Y[0]), 0.07, k.PLATE_Y[1] - k.HEAD_Y[0], region=FT, material=ZT))
    elif scale == 200:
        out.append(k.ellipse2("wheel-%s" % pos, "chassis", "body", 2.5, True, 0.18, *C(xw, k.WHEEL_R), k.WHEEL_R, k.WHEEL_R, region=WH, material=PU))
        fork = [C(x, y) for x, y in [(xs + 0.035, k.PLATE_Y[1]), (xs - 0.035, k.PLATE_Y[1]), (xs - 0.035, k.HEAD_Y[0]), (xw - 0.015, k.WHEEL_R), (xw + 0.015, k.WHEEL_R), (xs + 0.02, k.HEAD_Y[0]), (xs + 0.035, k.HEAD_Y[0])]]
        out.append(k.poly2("fork-%s" % pos, "chassis", "body", 2.6, True, 0.6, fork, True, region=FT, material=ZT))
    else:
        pts = [C(x, y) for x, y in [(xs + 0.04, k.PLATE_Y[1]), (xs - 0.04, k.PLATE_Y[1]), (xs - 0.04, 0.11)]]
        arc = [C(xw + k.WHEEL_R * k.math.cos(a), k.WHEEL_R + k.WHEEL_R * k.math.sin(a)) for a in [k.math.radians(d) for d in range(150, 400, 20)]]
        out.append(k.poly2("castor-%s" % pos, "chassis", "body", 2.5, True, 0.18, pts + arc + [C(xs + 0.04, 0.11)], True, regions=[WH, FT], material=PU))
    return out


def elevation(scale):
    detail = scale == 50
    out = []
    p_tube = pen(scale, 0.025)
    line = frame_line() if scale <= 200 else [(k.RAIL_X_FRONT, k.RAIL_Y), k.SIGN_CORNER, k.SIGN_TOP]
    out.append(k.poly2("chassis-line", "chassis", "body", 2.7, False, 0.3, [C(*p) for p in line], False, region=FT, material=ZT, penMm=p_tube))
    if scale <= 200:
        out.append(k.poly2("front-strut", "chassis", "body", 2.7, False, 0.3, [C(0.30, 0.18), C(0.262, 0.562)], False, region=FT, material=ZT, penMm=pen(scale, 0.02)))
    if scale <= 100:
        out.append(k.poly2("bracket-rim", "chassis", "detail", 2.75, False, 0.3, [C(k.upright_x(0.875), 0.875), C(-0.391, 0.875)], False, region=FT, penMm=pen(scale, 0.011)))
        out.append(k.poly2("bracket-floor", "chassis", "detail", 2.75, False, 0.3, [C(k.upright_x(0.54), 0.54), C(-0.357, 0.54)], False, region=FT, penMm=pen(scale, 0.01)))
    if scale <= 200:
        xr, xf = k.TRAY_X
        top = k.TRAY_Y + k.TRAY_LIP
        out.append(k.poly2("tray", "chassis", "detail" if scale <= 200 else "body", 2.65, False, 0.3, [C(xr, top), C(xr, k.TRAY_Y), C(xf, k.TRAY_Y), C(xf, top)], False, region=BW, penMm=pen(scale, 0.006)))
    for pos in ("rear", "front"):
        out += castor_elev(pos, scale, detail)
    e = 0.004 if scale <= 100 else (0.0 if scale == 200 else 0.004)
    out.append(k.poly2("basket", "basket", "glazing", 3.0, True, MESH, basket_side_outline(e), True, region=BW, material=ZW))
    fr, ff, rf, rr = k.side_corners(1)
    if scale <= 200:
        out.append(k.poly2("rim", "basket", "detail", 3.3, False, 0.3, [rr[:2], rf[:2]], False, region=BW, penMm=pen(scale, 0.008)))
        out.append(k.poly2("floor-wire", "basket", "detail", 3.3, False, 0.3, [fr[:2], ff[:2]], False, region=BW, penMm=pen(scale, 0.006)))
        out.append(k.poly2("corner-rear", "basket", "detail", 3.3, False, 0.3, [fr[:2], rr[:2]], False, region=BW, penMm=pen(scale, 0.006)))
        out.append(k.poly2("corner-front", "basket", "detail", 3.3, False, 0.3, [ff[:2], rf[:2]], False, region=BW, penMm=pen(scale, 0.006)))
    else:
        out.append(k.poly2("rim", "basket", "body", 3.3, False, 0.3, [rr[:2], rf[:2]], False, region=BW, penMm=0.13))
    role = "texture" if scale == 50 else "detail"
    if scale <= 100:
        for i, x in enumerate(k.picket_xs(1) if scale == 50 else k.picket_xs(2, 0.185)):
            lo, hi = k.side_vertical(1, x)
            out.append(k.poly2("picket-%d" % (i + 1), "basket", role, 3.2, False, 0.3, [lo[:2], hi[:2]], False, region=BW))
    hoops = {50: (0.07, 0.14, 0.21, 0.28), 100: (0.105, 0.21), 200: (0.105, 0.21)}.get(scale, ())
    for j, y in enumerate(hoops):
        a, b = k.side_horizontal(1, y)
        out.append(k.poly2("wire-h-%d" % (j + 1), "basket", role if scale <= 100 else "detail", 3.2, False, 0.3, [a[:2], b[:2]], False, region=BW))
    out.append(gate_shape(scale, GATE_T[scale]))
    out.append(k.poly2("gate-frame", "gate", "detail" if scale <= 200 else "body", 3.45, False, 0.3, [[0.015, 0], [0.322, 0]], False, region=GW, penMm=pen(scale, 0.006)))
    if scale <= 100:
        out.append(k.capsule2("gate-eye", "gate", "detail", 3.6, True, 0.75, -0.002, 0, 0.015, 0, 0.0065, region=GW))
    arm = [Hh(*k.UPRIGHT_TOP)] + [Hh(*p) for p in k.ARM] + [[0, 0]]
    if scale <= 200:
        out.append(k.poly2("handle-arm", "handle", "body", 3.5, False, 0.3, arm, False, region=FT, material=ZT, penMm=p_tube))
        out.append(k.ellipse2("grip", "handle", "body", 3.6, True, 0.4, 0, 0, k.CAP_R, k.CAP_R, region=HG, material=PP))
        out.append(k.rect2("coin-lock", "handle", "detail" if scale <= 100 else "body", 3.55, True, 0.35, -0.002, -0.037, 0.06, 0.05, r=0.008 if scale <= 100 else None, region=HG, material=PP))
    else:
        out.append(k.poly2("handle", "handle", "body", 3.5, False, 0.3, [Hh(*k.SIGN_TOP), [Hh(*k.SIGN_TOP)[0] - 0.0005, 0.0]], False, region=HG, material=PP, penMm=0.13))
    return out


def castor_front(pos, n, s, scale, detail):
    c = k.CASTORS[pos]
    zs = s * c["zs"]
    tag = "%s-%s" % (pos, n)
    zl = 2.4 if pos == "rear" else 2.5
    out = []
    if scale <= 200:
        out.append(k.rect2("wheel-%s" % tag, "chassis", "body", zl, True, 0.18, zs - k.WHEEL_W / 2, 0.0, k.WHEEL_W, 2 * k.WHEEL_R, r=0.008 if scale <= 100 else None, region=WH, material=PU))
        if scale <= 100:
            pts = [(zs - 0.026, 0.0465), (zs - 0.0185, 0.0465), (zs - 0.0185, 0.134), (zs + 0.0185, 0.134), (zs + 0.0185, 0.0465), (zs + 0.026, 0.0465), (zs + 0.026, 0.142), (zs - 0.026, 0.142)]
            out.append(k.poly2("fork-%s" % tag, "chassis", "body", zl + 0.05, True, 0.6, pts, True, region=FT, material=ZT))
            out.append(k.rect2("castor-head-%s" % tag, "chassis", "body", zl + 0.06, True, 0.5, zs - 0.0285, k.HEAD_Y[0], 0.057, k.HEAD_Y[1] - k.HEAD_Y[0], region=FT, material=ZT))
            out.append(k.rect2("castor-plate-%s" % tag, "chassis", "body", zl + 0.07, True, 0.7, zs - 0.03, k.PLATE_Y[0], 0.06, k.PLATE_Y[1] - k.PLATE_Y[0], region=FT, material=ZT))
        else:
            out.append(k.rect2("fork-%s" % tag, "chassis", "body", zl + 0.05, True, 0.6, zs - 0.03, k.HEAD_Y[0] - 0.02, 0.06, k.PLATE_Y[1] - k.HEAD_Y[0] + 0.02, region=FT, material=ZT))
    else:
        zb = s * min(c["zs"], 0.2475)
        out.append(k.rect2("castor-%s" % tag, "chassis", "body", zl, True, 0.18, zb - 0.0375, 0.0, 0.075, k.PLATE_Y[1], regions=[WH, FT], material=PU))
    return out


def front(scale):
    detail = scale == 50
    out = []
    p_tube = pen(scale, 0.025)
    zr = k.RAIL_Z_REAR
    top = k.UPRIGHT_TOP[1] if scale <= 200 else k.SIGN_TOP[1]
    out.append(k.poly2("chassis-line", "chassis", "body", 2.7, False, 0.3, [[zr, top], [zr, k.RAIL_Y], [-zr, k.RAIL_Y], [-zr, top]], False, region=FT, material=ZT, penMm=p_tube))
    if scale <= 200:
        for n, s in SIDES:
            out.append(k.poly2("front-strut-%s" % n, "chassis", "body", 2.7, False, 0.3, [[s * 0.1987, 0.18], [s * 0.186, 0.562]], False, region=FT, material=ZT, penMm=pen(scale, 0.02)))
        zt = k.rail_z(k.TRAY_X[0])
        zf = k.rail_z(k.TRAY_X[1]) - 0.01
        top = k.TRAY_Y + k.TRAY_LIP
        out.append(k.poly2("tray", "chassis", "detail", 2.65, False, 0.3, [[-zt, k.TRAY_Y], [-zf, k.TRAY_Y], [-zf, top], [zf, top], [zf, k.TRAY_Y], [zt, k.TRAY_Y]], False, region=BW, penMm=pen(scale, 0.006)))
    if scale <= 200:
        for pos in ("rear", "front"):
            for n, s in SIDES:
                out += castor_front(pos, n, s, scale, detail)
    else:
        lo, hi = k.CASTORS["front"]["zs"] - 0.0375, 0.285
        for n, s in SIDES:
            out.append(k.rect2("castors-%s" % n, "chassis", "body", 2.5, True, 0.18, lo if s > 0 else -hi, 0.0, hi - lo, k.PLATE_Y[1], regions=[WH, FT], material=PU))
    e = 0.004 if scale <= 100 else 0.0
    out.append(k.poly2("basket", "basket", "glazing", 3.0, True, MESH, front_outline(e), True, region=BW, material=ZW))
    yb = BA[1]
    rim_y = yb + k.RIM_REAR[1]
    if scale <= 200:
        out.append(k.poly2("rim", "basket", "detail", 3.3, False, 0.3, [[-k.RIM_REAR[2], rim_y], [k.RIM_REAR[2], rim_y]], False, region=BW, penMm=pen(scale, 0.008)))
        out.append(k.poly2("floor-wire", "basket", "detail", 3.3, False, 0.3, [[-k.FLOOR_REAR[2], yb], [k.FLOOR_REAR[2], yb]], False, region=BW, penMm=pen(scale, 0.006)))
        for n, s in SIDES:
            out.append(k.poly2("corner-rear-%s" % n, "basket", "detail", 3.3, False, 0.3, [[s * k.FLOOR_REAR[2], yb], [s * k.RIM_REAR[2], rim_y]], False, region=BW, penMm=pen(scale, 0.006)))
            out.append(k.poly2("corner-front-%s" % n, "basket", "detail", 3.3, False, 0.3, [[s * k.FLOOR_FRONT[2], yb + k.FLOOR_FRONT[1]], [s * k.RIM_FRONT[2], rim_y]], False, region=BW, penMm=pen(scale, 0.006)))
    else:
        out.append(k.poly2("rim", "basket", "body", 3.3, False, 0.3, [[-k.RIM_REAR[2], rim_y], [k.RIM_REAR[2], rim_y]], False, region=BW, penMm=0.13))
    role = "texture" if scale == 50 else "detail"
    if scale <= 100:
        step = 1 if scale == 50 else 2
        for i in range(0, 13, step):
            z = -0.18 + 0.03 * i
            lo, hi = k.front_vertical(z)
            out.append(k.poly2("picket-f-%d" % (i // step + 1), "basket", role, 3.2, False, 0.3, [[z, yb + lo[1]], [z, yb + hi[1]]], False, region=BW))
    hoops = {50: (0.07, 0.14, 0.21, 0.28), 100: (0.105, 0.21), 200: (0.105, 0.21)}.get(scale, ())
    for j, y in enumerate(hoops):
        a, _ = k.side_horizontal(1, y)
        out.append(k.poly2("wire-h-%d" % (j + 1), "basket", role, 3.2, False, 0.3, [[-a[2], yb + y], [a[2], yb + y]], False, region=BW))
    out.append(gate_shape(scale, GATE_T[scale]))
    gy = k.GRIP_C[1]
    if scale <= 200:
        for n, s in SIDES:
            out.append(k.poly2("handle-post-%s" % n, "handle", "body", 3.5, False, 0.3, [[s * zr, k.UPRIGHT_TOP[1]], [s * zr, gy]], False, region=FT, material=ZT, penMm=p_tube))
            out.append(k.capsule2("cap-%s" % n, "handle", "detail" if scale <= 100 else "body", 3.65, True, 0.4, s * 0.25, gy, s * 0.267, gy, k.CAP_R, region=HG, material=PP))
        out.append(k.rect2("grip", "handle", "body", 3.6, True, 0.4, -k.GRIP_HALF, gy - k.GRIP_R, 2 * k.GRIP_HALF, 2 * k.GRIP_R, r=k.GRIP_R if scale <= 100 else None, region=HG, material=PP, **({"hatch": "grip-ribs"} if scale == 50 else {})))
        out.append(k.rect2("coin-lock", "handle", "detail" if scale <= 100 else "body", 3.7, True, 0.35, -0.05, gy - 0.037, 0.10, 0.05, r=0.008 if scale <= 100 else None, region=HG, material=PP))
    else:
        out.append(k.poly2("handle", "handle", "body", 3.5, False, 0.3, [[-zr, gy], [zr, gy]], False, region=HG, material=PP, penMm=0.13))
    return out


def plan(scale):
    detail = scale == 50
    out = []
    p_tube = pen(scale, 0.025)
    zr, zf = k.RAIL_Z_REAR, k.RAIL_Z_FRONT
    ux = k.UPRIGHT_TOP[0]
    if scale <= 200:
        line = [[ux, zr], [k.RAIL_X_REAR, zr], [k.RAIL_X_FRONT, zf], [k.RAIL_X_FRONT, -zf], [k.RAIL_X_REAR, -zr], [ux, -zr]]
    else:
        cx, tx_ = k.SIGN_CORNER[0], k.SIGN_TOP[0]
        line = [[tx_, zr], [cx, zr], [k.RAIL_X_FRONT, zf], [k.RAIL_X_FRONT, -zf], [cx, -zr], [tx_, -zr]]
    out.append(k.poly2("chassis-line", "chassis", "body", 2.6, False, 0.3, line, False, region=FT, material=ZT, penMm=p_tube))
    for pos in ("rear", "front"):
        c = k.CASTORS[pos]
        xs = c["xs"]
        xw = xs - k.TRAIL
        for n, s in SIDES:
            zs = s * c["zs"]
            tag = "%s-%s" % (pos, n)
            if scale <= 200:
                out.append(k.rect2("wheel-%s" % tag, "chassis", "body", 2.45, True, 0.18, xw - k.WHEEL_R, zs - k.WHEEL_W / 2, 2 * k.WHEEL_R, k.WHEEL_W, r=0.008 if scale <= 100 else None, region=WH, material=PU))
                if scale <= 100:
                    out.append(k.rect2("castor-plate-%s" % tag, "chassis", "body", 2.5, True, 0.7, xs - 0.035, zs - 0.03, 0.07, 0.06, region=FT, material=ZT))
            else:
                zb = s * min(c["zs"], 0.2475)
                out.append(k.rect2("castor-%s" % tag, "chassis", "body", 2.45, True, 0.18, xw - k.WHEEL_R, zb - 0.0375, xs + 0.04 - xw + k.WHEEL_R, 0.075, regions=[WH, FT], material=PU))
    if scale <= 200:
        xr, xf = k.TRAY_X
        ring = [[xr, k.rail_z(xr)], [xf, k.rail_z(xf)], [xf, -k.rail_z(xf)], [xr, -k.rail_z(xr)]]
        out.append(k.poly2("tray", "chassis", "detail", 2.55, False, 0.3, ring, True, region=BW, penMm=pen(scale, 0.006)))
    e = 0.004 if scale <= 100 else 0.005
    out.append(k.poly2("basket", "basket", "glazing", 3.0, True, MESH, grow(plan_rim(), e), True, region=BW, material=ZW))
    if scale <= 200:
        out.append(k.poly2("floor", "basket", "glazing", 3.05, True, MESH, plan_floor(), True, region=BW, material=ZW))
        out.append(k.poly2("rim", "basket", "detail", 3.3, False, 0.3, plan_rim(), True, region=BW, penMm=pen(scale, 0.008)))
        out.append(k.poly2("floor-wire", "basket", "detail", 3.3, False, 0.3, plan_floor(), True, region=BW, penMm=pen(scale, 0.006)))
        for n, s in SIDES:
            fr, ff, rf, rr = [bw(p) for p in k.side_corners(s)]
            out.append(k.poly2("corner-rear-%s" % n, "basket", "detail", 3.3, False, 0.3, [[fr[0], fr[2]], [rr[0], rr[2]]], False, region=BW, penMm=pen(scale, 0.006)))
            out.append(k.poly2("corner-front-%s" % n, "basket", "detail", 3.3, False, 0.3, [[ff[0], ff[2]], [rf[0], rf[2]]], False, region=BW, penMm=pen(scale, 0.006)))
    else:
        out.append(k.poly2("rim", "basket", "body", 3.3, False, 0.3, plan_rim(), True, region=BW, penMm=0.13))
    role = "texture" if scale == 50 else "detail"
    if scale <= 100:
        n_long, n_cross, fstep, sstep = (13, (0.1, 0.2, 0.3, 0.4, 0.5), 1, 1) if scale == 50 else (7, (0.155, 0.31, 0.465), 2, 2)
        for i in range(n_long):
            a, b = k.floor_long(i, n_long, 0.025)
            a, b = bw(a), bw(b)
            out.append(k.poly2("floor-long-%d" % (i + 1), "basket", role, 3.2, False, 0.3, [[a[0], a[2]], [b[0], b[2]]], False, region=BW))
        for i, x in enumerate(n_cross):
            h = k.floor_half(x)
            out.append(k.poly2("floor-cross-%d" % (i + 1), "basket", role, 3.15, False, 0.3, [[BA[0] + x, -h], [BA[0] + x, h]], False, region=BW))
        for i in range(0, 13, fstep):
            z = -0.18 + 0.03 * i
            lo, hi = k.front_vertical(z)
            out.append(k.poly2("picket-f-%d" % (i // fstep + 1), "basket", role, 3.2, False, 0.3, [[BA[0] + lo[0], z], [BA[0] + hi[0], z]], False, region=BW))
        for n, s in SIDES:
            for i, x in enumerate(k.picket_xs(1) if scale == 50 else k.picket_xs(2, 0.185)):
                lo, hi = k.side_vertical(s, x)
                out.append(k.poly2("picket-%s-%d" % (n, i + 1), "basket", role, 3.2, False, 0.3, [[BA[0] + lo[0], lo[2]], [BA[0] + hi[0], hi[2]]], False, region=BW))
    out.append(gate_shape(scale, GATE_T[scale]))
    gx = k.GRIP_C[0]
    if scale <= 200:
        for n, s in SIDES:
            out.append(k.poly2("handle-arm-%s" % n, "handle", "body", 3.5, False, 0.3, [[ux, s * zr], [gx, s * zr]], False, region=FT, material=ZT, penMm=p_tube))
            out.append(k.capsule2("cap-%s" % n, "handle", "detail" if scale <= 100 else "body", 3.65, True, 0.4, gx, s * 0.25, gx, s * 0.267, k.CAP_R, region=HG, material=PP))
        out.append(k.capsule2("grip", "handle", "body", 3.6, True, 0.4, gx, -0.2325, gx, 0.2325, k.GRIP_R, region=HG, material=PP, **({"hatch": "grip-ribs"} if scale == 50 else {})))
        out.append(k.rect2("coin-lock", "handle", "detail" if scale <= 100 else "body", 3.7, True, 0.35, gx - 0.002, -0.05, 0.06, 0.10, r=0.008 if scale <= 100 else None, region=HG, material=PP))
    else:
        out.append(k.poly2("handle", "handle", "body", 3.5, False, 0.3, [[k.SIGN_TOP[0], zr], [k.SIGN_TOP[0], -zr]], False, region=HG, material=PP, penMm=0.13))
    return out


def tx(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    50: {
        "silhouette": tx("The trolley 21 mm long and 20 mm high as built: the translucent wire basket with every picket at 30 mm, four horizontal hoops, rim, floor and corner wires, the rear gate hanging from the rim bar with its hinge eye, the o25 chassis tube bent up into the push-frame upright, the front strut and the two basket brackets, the crate tray with its lips on the rails, two swivel castors trailing 35 mm with fork, head, plate, tyre, hub and axle, and the handle arm with the grip end, cap and coin lock. Front view: the tapering basket with its 13 front pickets and four hoops, the U of rails and uprights, struts, tray lip, all four castors (the rear pair outside the front pair), grip with ribs, caps and coin lock. Plan: rim and floor outlines, floor wires, front and side pickets, the rail trapezoid converging to the front, castor plates and wheels, the tray, the grip with caps and coin lock. The gate is an extruded solid on its hinge bone: swung up, it rises inside the basket in the front view and swings forward over the floor in the plan.", "Der Wagen 21 mm lang und 20 mm hoch wie gebaut: der durchscheinende Drahtkorb mit jedem Stab im Abstand 30 mm, vier waagrechten Bügeln, Rand-, Boden- und Eckdrähten, die hintere Klappe am Randstab mit ihrer Scharnieröse, das Fahrgestellrohr o25 in den Schiebebügel hochgebogen, die vordere Stütze und die zwei Korblaschen, die Kistenablage mit ihren Kanten auf den Holmen, zwei Lenkrollen 35 mm nachlaufend mit Gabel, Kopf, Platte, Reifen, Nabe und Achse sowie der Griffarm mit Griffende, Kappe und Pfandschloss. Vorderansicht: der sich verjüngende Korb mit 13 Stirnstäben und vier Bügeln, das U aus Holmen und Bügeln, Stützen, Ablagekante, alle vier Rollen (das hintere Paar außerhalb des vorderen), Griff mit Rippen, Kappen und Pfandschloss. Draufsicht: Rand- und Bodenumriss, Bodendrähte, Stirn- und Seitenstäbe, das nach vorn zusammenlaufende Holmtrapez, Rollenplatten und Räder, die Ablage, der Griff mit Kappen und Pfandschloss. Die Klappe ist ein extrudierter Körper am Scharnierknochen: hochgeschwenkt steigt sie in der Vorderansicht im Korb auf und schwenkt in der Draufsicht nach vorn über den Boden."),
        "retained": tx("Every wire at its real position (pickets 30 mm, hoops 70 mm, floor wires 30 mm), tubes at their real o25 and o20 width, castor parts at real size, grip, caps and coin lock.", "Jeder Draht in seiner realen Lage (Stäbe 30 mm, Bügel 70 mm, Bodendrähte 30 mm), Rohre in realer Breite o25 und o20, Rollenteile in realer Größe, Griff, Kappen und Pfandschloss."),
        "merged": tx("The far-side frame, castors and wires coincide with the near side in the elevation; the gate is one translucent panel with its frame line.", "Rahmen, Rollen und Drähte der fernen Seite fallen in der Ansicht mit der nahen zusammen; die Klappe ist eine durchscheinende Tafel mit ihrer Rahmenlinie."),
        "omitted": tx("Wire welds, brake pedal, deflector rings, coin-lock chain, the individual gate wires. Front view: the side pickets, seen almost edge-on in a 0.9 mm band. Plan: the crate tray wires below the basket floor mesh.", "Drahtschweißpunkte, Bremspedal, Abweisringe, Pfandschlosskette, die einzelnen Klappendrähte. Vorderansicht: die Seitenstäbe, fast kantenparallel in einem 0,9-mm-Streifen. Draufsicht: die Drähte der Kistenablage unter dem Bodengitter."),
        "thinElements": tx("Tubes are drawn at their true width (o25 = 0.5 mm, o20 = 0.4 mm); rim 0.16 mm (o8), frame and corner wires 0.13 mm (o6, at the printable minimum), mesh wires 0.13 mm texture (real o4.5).", "Rohre in wahrer Breite (o25 = 0,5 mm, o20 = 0,4 mm); Rand 0,16 mm (o8), Rahmen- und Eckdrähte 0,13 mm (o6, am druckbaren Minimum), Gitterdrähte 0,13 mm Textur (real o4,5)."),
        "hatching": tx("Grip ribs as 0.5 mm spaced lines at 90 degrees.", "Griffrippen als Linien im Abstand 0,5 mm unter 90 Grad."),
        "styles": tx("Mesh planes translucent glazing 0.55, tyres 0.18, hubs 0.78, castor steel 0.5-0.7, grip and caps 0.4, coin lock 0.35; tube strokes at true width, detail 0.25 mm, texture 0.13 mm.", "Gitterebenen durchscheinende Verglasung 0,55, Reifen 0,18, Naben 0,78, Rollenstahl 0,5-0,7, Griff und Kappen 0,4, Pfandschloss 0,35; Rohrstriche in wahrer Breite, Detail 0,25 mm, Textur 0,13 mm."),
        "overlays": tx("Basket, mesh, rim, floor and tray wires carry basket-wire; gate panel and eye gate-wire; tubes, struts, brackets and castor steel frame-tube; tyres and hubs wheels; grip, caps and coin lock handle-grip.", "Korb, Gitter, Rand-, Boden- und Ablagedrähte tragen basket-wire; Klappentafel und Öse gate-wire; Rohre, Stützen, Laschen und Rollenstahl frame-tube; Reifen und Naben wheels; Griff, Kappen und Pfandschloss handle-grip."),
    },
    100: {
        "silhouette": tx("The trolley 10.5 mm long and 10 mm high: the translucent basket with every second picket (60 mm), two hoops, rim, floor and corner wires, the gate panel with its eye, the bent chassis tube with strut and brackets, the crate tray, two castors with fork, head, wheel and hub, the handle arm with grip end and coin lock. Front view: basket with seven front pickets and two hoops, rails and uprights, struts, tray lip, four castors, grip, caps, coin lock. Plan: rim, floor, seven floor wires, three cross wires, front and side pickets at 60 mm, rails, castor plates and wheels, tray, grip with caps and coin lock.", "Der Wagen 10,5 mm lang und 10 mm hoch: der durchscheinende Korb mit jedem zweiten Stab (60 mm), zwei Bügeln, Rand-, Boden- und Eckdrähten, die Klappentafel mit Öse, das gebogene Fahrgestellrohr mit Stütze und Laschen, die Kistenablage, zwei Rollen mit Gabel, Kopf, Rad und Nabe, der Griffarm mit Griffende und Pfandschloss. Vorderansicht: Korb mit sieben Stirnstäben und zwei Bügeln, Holme und Bügel, Stützen, Ablagekante, vier Rollen, Griff, Kappen, Pfandschloss. Draufsicht: Rand, Boden, sieben Bodendrähte, drei Querdrähte, Stirn- und Seitenstäbe im Abstand 60 mm, Holme, Rollenplatten und Räder, Ablage, Griff mit Kappen und Pfandschloss."),
        "retained": tx("Basket form, rim, floor and corner wires, every second picket, tubes at true width, castor fork, head, plate, wheel and hub, grip, caps, coin lock.", "Korbform, Rand-, Boden- und Eckdrähte, jeder zweite Stab, Rohre in wahrer Breite, Rollengabel, Kopf, Platte, Rad und Nabe, Griff, Kappen, Pfandschloss."),
        "merged": tx("Two pickets and two hoops of 1:50 are drawn as one wire; castor head and plate are one block in the elevation; the gate is one translucent panel.", "Je zwei Stäbe und zwei Bügel von 1:50 sind als ein Draht gezeichnet; Rollenkopf und Platte sind in der Ansicht ein Block; die Klappe ist eine durchscheinende Tafel."),
        "omitted": tx("Tread line, axle nut, grip ribs, tray wires, wire welds, brake pedal.", "Laufflächenlinie, Achsmutter, Griffrippen, Ablagedrähte, Drahtschweißpunkte, Bremspedal."),
        "thinElements": tx("Tubes at true width (o25 = 0.25 mm), wires as 0.13 mm detail strokes (real o4.5 to o8).", "Rohre in wahrer Breite (o25 = 0,25 mm), Drähte als Detailstriche 0,13 mm (real o4,5 bis o8)."),
        "hatching": tx("No hatching.", "Keine Schraffur."),
        "styles": tx("Mesh planes translucent glazing 0.55, tyres 0.18, hubs 0.78, castor steel 0.5-0.7, grip 0.4; body pen 0.25 mm, detail 0.13 mm.", "Gitterebenen durchscheinende Verglasung 0,55, Reifen 0,18, Naben 0,78, Rollenstahl 0,5-0,7, Griff 0,4; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": tx("As at 1:50: basket-wire, gate-wire, frame-tube, wheels and handle-grip on their parts.", "Wie in 1:50: basket-wire, gate-wire, frame-tube, wheels und handle-grip auf ihren Teilen."),
    },
    200: {
        "silhouette": tx("The trolley 5.3 mm long and 5 mm high: the translucent basket outlined by rim, floor and corner wires with two hoops, the gate panel, the bent chassis tube with strut, the crate tray line, two wheels on simple castor blocks, the handle arm with grip end and coin lock. Front view: basket with two hoops, rails and uprights, struts, tray lip, four wheels with castor blocks, grip, caps, coin lock. Plan: rim and floor outlines, rails, four wheels, tray outline, grip with caps and coin lock.", "Der Wagen 5,3 mm lang und 5 mm hoch: der durchscheinende Korb, umrissen von Rand-, Boden- und Eckdrähten mit zwei Bügeln, die Klappentafel, das gebogene Fahrgestellrohr mit Stütze, die Ablagelinie, zwei Räder auf einfachen Rollenblöcken, der Griffarm mit Griffende und Pfandschloss. Vorderansicht: Korb mit zwei Bügeln, Holme und Bügel, Stützen, Ablagekante, vier Räder mit Rollenblöcken, Griff, Kappen, Pfandschloss. Draufsicht: Rand- und Bodenumriss, Holme, vier Räder, Ablageumriss, Griff mit Kappen und Pfandschloss."),
        "retained": tx("Basket form and its edge wires, two hoops, chassis and strut, castors, grip, coin lock.", "Korbform mit Kantendrähten, zwei Bügel, Fahrgestell und Stütze, Rollen, Griff, Pfandschloss."),
        "merged": tx("Castor fork, head and plate are one block; the mesh is the translucent plane; the gate is one panel.", "Rollengabel, Kopf und Platte sind ein Block; das Gitter ist die durchscheinende Ebene; die Klappe ist eine Tafel."),
        "omitted": tx("Pickets, floor wires, hubs, gate eye, brackets, tray wires.", "Stäbe, Bodendrähte, Naben, Klappenöse, Laschen, Ablagedrähte."),
        "thinElements": tx("Tubes and wires at the 0.13 mm minimum pen (o25 tube would be 0.125 mm).", "Rohre und Drähte mit dem Mindeststift 0,13 mm (das Rohr o25 wäre 0,125 mm)."),
        "hatching": tx("No hatching.", "Keine Schraffur."),
        "styles": tx("Mesh planes translucent glazing 0.55, tyres 0.18, castor blocks 0.6, grip 0.4; body pen 0.18 mm, detail 0.13 mm.", "Gitterebenen durchscheinende Verglasung 0,55, Reifen 0,18, Rollenblöcke 0,6, Griff 0,4; Körperstift 0,18 mm, Detail 0,13 mm."),
        "overlays": tx("basket-wire, gate-wire, frame-tube, wheels and handle-grip on their parts.", "basket-wire, gate-wire, frame-tube, wheels und handle-grip auf ihren Teilen."),
    },
    500: {
        "silhouette": tx("A trolley sign 2.1 mm long and 2 mm high: the translucent basket trapezoid with its rim line, the chassis line running straight up from the rail end to the handle, the gate edge and two castor blocks with round wheels. Front view: the basket trapezoid, the U of rails and uprights with the handle bar, the gate panel and four castor blocks. Plan: the rim trapezoid, the rail trapezoid running back to the handle bar and four castor blocks.", "Ein Wagenzeichen, 2,1 mm lang und 2 mm hoch: das durchscheinende Korbtrapez mit Randlinie, die vom Holmende gerade zum Griff hochlaufende Fahrgestelllinie, die Klappenkante und zwei Rollenblöcke mit runden Rädern. Vorderansicht: das Korbtrapez, das U aus Holmen und Bügeln mit der Griffstange, die Klappentafel und vier Rollenblöcke. Draufsicht: das Randtrapez, das bis zur Griffstange zurücklaufende Holmtrapez und vier Rollenblöcke."),
        "retained": tx("Basket outline at real size and taper, rim, chassis line, handle height 1.0 m, castor positions.", "Korbumriss in realer Größe und Verjüngung, Rand, Fahrgestelllinie, Griffhöhe 1,0 m, Rollenlagen."),
        "merged": tx("Each castor (wheel, fork, head, plate) is one block 75 mm wide, the two castors of a side one block in the front view; rail bend, upright and handle arm are one straight line, grip and coin lock one handle bar 12 mm behind the grip axis so the drawing keeps the real 1.05 m length.", "Jede Rolle (Rad, Gabel, Kopf, Platte) ist ein Block von 75 mm Breite, die zwei Rollen einer Seite in der Vorderansicht ein Block; Holmbogen, Bügel und Griffarm sind eine gerade Linie, Griff und Pfandschloss eine Griffstange 12 mm hinter der Griffachse, damit die Zeichnung die reale Länge von 1,05 m behält."),
        "omitted": tx("All wires except the rim, struts, tray, brackets, hubs, caps.", "Alle Drähte außer dem Rand, Stützen, Ablage, Laschen, Naben, Kappen."),
        "thinElements": tx("Tubes and rim at the 0.13 mm minimum pen; castor blocks widened to 75 mm (0.15 mm).", "Rohre und Rand mit dem Mindeststift 0,13 mm; Rollenblöcke auf 75 mm (0,15 mm) verbreitert."),
        "hatching": tx("No hatching.", "Keine Schraffur."),
        "styles": tx("Mesh plane translucent glazing 0.55, castor blocks 0.18; body pen 0.13 mm.", "Gitterebene durchscheinende Verglasung 0,55, Rollenblöcke 0,18; Körperstift 0,13 mm."),
        "overlays": tx("The castor blocks carry wheels and frame-tube; the handle line handle-grip; the basket basket-wire; the gate gate-wire; the chassis line frame-tube.", "Die Rollenblöcke tragen wheels und frame-tube; die Grifflinie handle-grip; der Korb basket-wire; die Klappe gate-wire; die Fahrgestelllinie frame-tube."),
    },
}


def apply(asset):
    for scale in (50, 100, 200, 500):
        rep = asset["representations"][str(scale)]
        rep["pens"]["glazing"] = 0.13
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[scale]
