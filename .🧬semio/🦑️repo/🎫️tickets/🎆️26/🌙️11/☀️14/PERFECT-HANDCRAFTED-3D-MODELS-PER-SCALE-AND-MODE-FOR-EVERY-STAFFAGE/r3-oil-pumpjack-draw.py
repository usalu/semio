"""✏️ Round 3 drawings of `machinery/oil-pumpjack`: the side elevation, front elevation and plan of every scale, drawn by hand from the dimension table of `r3-oil-pumpjack-kit.py` as the true orthographic views of that scale's model (ruling E1); moving parts are drawn once in their bone frames and extruded across their lateral span (`solid`), so the stroke moves them in all three views."""

import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("pjkit", os.path.join(T, "r3-oil-pumpjack-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

r4 = K.r4


def shape(id, bone, role, z, tone, kind, region, material, filled=True, **geo):
    d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    d.update(geo)
    d["region"] = region
    d["material"] = material
    return d


def rect(id, bone, role, z, tone, region, material, x0, y0, x1, y1, **extra):
    d = shape(id, bone, role, z, tone, "rect", region, material, x=r4(min(x0, x1)), y=r4(min(y0, y1)), w=r4(abs(x1 - x0)), h=r4(abs(y1 - y0)))
    d.update(extra)
    return d


def ellipse(id, bone, role, z, tone, region, material, cx, cy, rx, ry=None, **extra):
    d = shape(id, bone, role, z, tone, "ellipse", region, material, cx=r4(cx), cy=r4(cy), rx=r4(rx), ry=r4(rx if ry is None else ry))
    d.update(extra)
    return d


def poly(id, bone, role, z, tone, region, material, pts, **extra):
    p = [[r4(a), r4(b)] for a, b in pts]
    if K.signed_area(p) < 0:
        p.reverse()
    d = shape(id, bone, role, z, tone, "poly", region, material, closed=True, points=p)
    d.update(extra)
    return d


def path(id, bone, role, z, tone, region, material, rings, **extra):
    parts = []
    for i, ring in enumerate(rings):
        pts = [[r4(a), r4(b)] for a, b in ring]
        ccw = K.signed_area(pts) > 0
        if (i == 0) != ccw:
            pts.reverse()
        parts.append("M " + " L ".join("%s %s" % (a, b) for a, b in pts) + " Z")
    d = shape(id, bone, role, z, tone, "path", region, material, d=" ".join(parts))
    d.update(extra)
    return d


def local(bone, pts):
    return [K.L2(bone, p) for p in pts]


def lrect(bone, x0, y0, x1, y1):
    a, b = K.L2(bone, (x0, y0)), K.L2(bone, (x1, y1))
    return min(a[0], b[0]), min(a[1], b[1]), max(a[0], b[0]), max(a[1], b[1])


def solid(z0, z1):
    return {"solid": {"kind": "extruded", "sweep": "w", "z0": r4(min(z0, z1)), "z1": r4(max(z0, z1))}}


LEVEL = {
    50: dict(arc=16, circle=32, head=12, gb_top=10, gb_in=6, guard=(12, 8), fins=36, crank=(10, 6), cw=(10, 6), hub=True, eye=True, web=True, saddle_role="detail", wellhead=5, terminal=True, shafts=True, clamp=True, brace=True, rod_w=0.05, skid_notch=True),
    100: dict(arc=12, circle=24, head=8, gb_top=8, gb_in=5, guard=(10, 6), fins=18, crank=(8, 5), cw=(8, 4), hub=True, eye=True, web=False, saddle_role="body", wellhead=2, terminal=True, shafts=True, clamp=True, brace=True, rod_w=0.05, skid_notch=True),
    200: dict(arc=8, circle=16, head=6, gb_top=6, gb_in=4, guard=(8, 5), fins=0, crank=(6, 4), cw=(6, 3), hub=True, eye=False, web=False, saddle_role="body", wellhead=1, terminal=False, shafts=True, clamp=False, brace=True, rod_w=0.05, skid_notch=False),
    500: dict(arc=5, circle=10, head=4, gb_top=0, gb_in=0, guard=(6, 4), fins=0, crank=(4, 3), cw=(4, 2), hub=False, eye=False, web=False, saddle_role="body", wellhead=1, terminal=False, shafts=True, clamp=False, brace=False, rod_w=0.1, skid_notch=False),
}

STEEL = ("steel", "painted-steel")


def head_pts(s):
    return [K.L2("horsehead", K.W2("beam", p)) for p in K.head_outline(s["head"])]


def beam_pts():
    return [(K.BEAM_X0, K.BEAM_V0), (K.BEAM_X1, K.BEAM_V0), (K.BEAM_X1, K.BEAM_V1), (K.BEAM_X0, K.BEAM_V1)]


def centre_housing(s):
    n = max(3, s["arc"] // 2)
    if s is LEVEL[500]:
        return [(-0.22, K.BEAM_V0), (0.22, K.BEAM_V0), (0.22, 0.0)] + K.arc(0, 0, 0.22, 0, -180, 4)[1:-1] + [(-0.22, 0.0)]
    return K.housing(0.0, K.HOUSING_R, K.HOUSING_R, K.HOUSING_HW, K.BEAM_V0, K.HOUSING_PLATE, n)


def tail_hanger(s):
    n = max(3, s["arc"] // 2)
    if s is LEVEL[500]:
        return [(K.C_TAIL - 0.18, K.BEAM_V0), (K.C_TAIL + 0.18, K.BEAM_V0), (K.C_TAIL + 0.18, 0.0)] + K.arc(K.C_TAIL, 0, 0.18, 0, -180, 4)[1:-1] + [(K.C_TAIL - 0.18, 0.0)]
    return K.housing(K.C_TAIL, K.HANGER_R, K.HANGER_R, K.HANGER_HW, K.BEAM_V0, K.HOUSING_PLATE, n)


def pitman_local(s):
    """Pitman arm in its own frame (u down the arm, v towards +x at rest): upper fitting, arm, and at coarse scales the eye."""
    hv = K.PIT_DEPTH / 2
    if s["eye"]:
        return [(-0.09, -0.13), (0.26, -0.13), (0.26, -hv), (2.55, -hv), (2.55, hv), (0.26, hv), (0.26, 0.13), (-0.09, 0.13)]
    a = K.math.degrees(K.math.asin(hv / K.EYE_R))
    pts = [(-0.09, -0.13), (0.26, -0.13), (0.26, -hv)]
    pts += K.arc(K.P_PITMAN, 0, K.EYE_R, 180 + a, 540 - a, 8)
    pts += [(0.26, hv), (0.26, 0.13), (-0.09, 0.13)]
    return pts


def crank_arm(s):
    a, b = s["crank"]
    return K.hull2((K.TAIL_C, 0), K.TAIL_R, (K.TIP_C, 0), K.TIP_R, a, b)


def counterweight(s):
    a, b = s["cw"]
    return K.sector(0, 0, K.CW_R0, K.CW_R1, K.CW_A, a, b, chamfer=0.05 if s is LEVEL[50] else 0.0)


def guard_pts(s):
    a, b = s["guard"]
    mb = K.L2("gearbox", K.MOTOR)
    return K.hull2(K.INPUT, K.GUARD_RA, (mb[0], mb[1]), K.GUARD_RB, a, b)


def gearbox_pts(s):
    if s["gb_top"]:
        return K.gearbox_outline(s["gb_top"], s["gb_in"])
    return K.gearbox_simple()


def motor_pts(s):
    if s["fins"]:
        return K.motor_fins(s["fins"], K.MOTOR_CORE, K.MOTOR_R, 0.005 if s["fins"] > 20 else 0.0075)
    return K.circle(0, 0, K.MOTOR_R - 0.005, s["circle"])


def wellhead_elev(s, view):
    """Wellhead column: casing head, flange, pumping tee, stuffing box, flowline (x is world x in elevation, z in front)."""
    out = []
    W = ("wellhead", "painted-steel")
    c = K.WELL_X if view == "elevation" else 0.0
    if s["wellhead"] == 5:
        out.append(rect("wellhead-base", "base", "body", 3.1, 0.55, *W, c - 0.17, 0, c + 0.17, 0.5))
        out.append(rect("wellhead-valve", "base", "body", 3.1 if view == "elevation" else 3.15, 0.7, *W, c - 0.1, 0.5, c + 0.1, 1.0))
        out.append(rect("wellhead-flange", "base", "body", 3.1 if view == "elevation" else 3.15, 0.7, *W, c - 0.2, 0.5, c + 0.2, 0.53))
        out.append(rect("stuffing-box", "base", "body", 3.1 if view == "elevation" else 3.2, 0.45, *W, c - 0.08, 1.0, c + 0.08, 1.28))
        if view == "elevation":
            out.append(rect("flowline", "base", "body", 3.1, 0.55, *W, -2.83, 0.62, -2.0, 0.74))
        else:
            out.append(rect("flowline", "base", "body", 3.25, 0.55, *W, -0.06, 0.62, 0.06, 0.74))
    elif s["wellhead"] == 2:
        out.append(poly("wellhead-base", "base", "body", 3.1, 0.55, *W, [(c - 0.17, 0), (c + 0.17, 0), (c + 0.17, 0.5), (c + 0.2, 0.5), (c + 0.2, 0.53), (c - 0.2, 0.53), (c - 0.2, 0.5), (c - 0.17, 0.5)]))
        out.append(rect("wellhead-top", "base", "body", 3.15, 0.6, *W, c - 0.1, 0.53, c + 0.1, 1.28))
    else:
        out.append(poly("wellhead", "base", "body", 3.1, 0.6, *W, [(c - 0.2, 0), (c + 0.2, 0), (c + 0.2, 0.53), (c + 0.1, 0.53), (c + 0.1, 1.28), (c - 0.1, 1.28), (c - 0.1, 0.53), (c - 0.2, 0.53)]))
    return out


def elevation(scale):
    s = LEVEL[scale]
    E = []
    E.append(rect("slab", "base", "body", 3, 0.82, "base", "concrete", K.SLAB[0], 0, K.SLAB[1], K.SLAB[3]))
    E.append(rect("skid", "skid", "body", 3.05, 0.5, "base", "painted-steel", 0, 0, K.SKID_X1 - K.SKID_X0, K.SKID_Y1 - K.SKID_Y0))
    E += wellhead_elev(s, "elevation")
    E.append(rect("polished-rod", "base", "body", 3.2, 0.9, *STEEL, K.WELL_X - s["rod_w"] / 2, 1.28, K.WELL_X + s["rod_w"] / 2, K.PIVOT[1] + 0.025))
    for id, sx in (("leg-left", -1), ("leg-right", 1)):
        p0, p1 = K.leg_axis(sx, 1)
        E.append(poly(id, "post", "body", 3.3, 0.62, *STEEL, local("post", K.strip(p0, p1, K.LEG / 2, K.SKID_Y1, K.CAP_Y0))))
    if s["brace"]:
        x0, y0, x1, y1 = lrect("post", -1.06, K.BRACE_Y - K.BRACE_T / 2, 1.06, K.BRACE_Y + K.BRACE_T / 2)
        E.append(rect("brace", "post", "body", 3.35, 0.55, *STEEL, x0, y0, x1, y1))
    sad = [(-K.CAP_HW, K.cap_y0(scale)), (K.CAP_HW, K.cap_y0(scale)), (K.CAP_HW, K.CAP_Y1), (K.BLOCK_HX, K.CAP_Y1), (K.BLOCK_HX, K.BLOCK_Y1), (-K.BLOCK_HX, K.BLOCK_Y1), (-K.BLOCK_HX, K.CAP_Y1), (-K.CAP_HW, K.CAP_Y1)]
    E.append(poly("saddle", "post", s["saddle_role"], 3.45, 0.3, *STEEL, local("post", sad)))
    E.append(poly("beam", "beam", "body", 3.5, 0.7, *STEEL, beam_pts()))
    E.append(poly("centre-bearing", "beam", "detail" if scale <= 100 else "body", 3.52, 0.3, *STEEL, centre_housing(s)))
    if scale < 500:
        E.append(poly("tail-pin", "beam", "detail" if scale <= 100 else "body", 3.52, 0.3, *STEEL, tail_hanger(s)))
    E.append(poly("horsehead", "horsehead", "body", 3.55, 0.78, *STEEL, head_pts(s)))
    if s["web"]:
        E.append(poly("horsehead-web", "horsehead", "detail", 3.6, 0.3, *STEEL, [K.L2("horsehead", K.W2("beam", p)) for p in K.face_band(s["head"])]))
    E.append(rect("pedestal", "gearbox", "body", 3.15, 0.5, "gearbox", "cast-iron", -0.62, -1.20, 1.60, -1.05))
    E.append(poly("gearbox", "gearbox", "body", 3.2, 0.62, "gearbox", "cast-iron", gearbox_pts(s)))
    E.append(poly("counterweight", "crank", "body", 3.7, 0.4, "crank", "cast-iron", counterweight(s)))
    E.append(poly("crank-arm", "crank", "body", 3.75, 0.58, "crank", "cast-iron", crank_arm(s)))
    if s["hub"] and scale <= 100:
        E.append(ellipse("crank-hub", "crank", "detail", 3.85, 0.3, "crank", "cast-iron", 0, 0, K.HUB_R))
    E.append(rect("equalizer", "pitman", "body", 3.88, 0.45, *STEEL, K.eq_u0(scale), -K.EQ_V, K.EQ_U1, K.EQ_V))
    E.append(poly("pitman", "pitman", "body", 3.9, 0.6, *STEEL, pitman_local(s)))
    if s["eye"]:
        E.append(ellipse("pitman-eye", "pitman", "detail", 3.95, 0.3, *STEEL, K.P_PITMAN, 0, K.EYE_R))
    hu, hv = K.carrier_h(scale)
    E.append(rect("carrier", "rod", "body", 3.3, 0.3, *STEEL, -hu, -hv, hu, hv))
    if s["clamp"]:
        E.append(rect("rod-clamp", "rod", "detail", 3.32, 0.25, *STEEL, K.CARRIER_HU, -0.06, K.CARRIER_HU + 0.10, 0.06))
    E.append(poly("motor-feet", "motor", "body", 3.15, 0.5, "motor", "motor-steel", [(-0.32, -0.35), (0.32, -0.35), (0.32, -0.31), (0.22, -0.31), (0.22, -0.22), (-0.22, -0.22), (-0.22, -0.31), (-0.32, -0.31)] if scale == 50 else [(-0.32, -0.35), (0.32, -0.35), (0.32, -0.22), (-0.32, -0.22)]))
    if scale == 50:
        mb = K.L2("gearbox", K.MOTOR)
        for i, (gx, top) in enumerate(((K.INPUT[0], K.INPUT[1] - K.GUARD_RA + 0.03), (mb[0], mb[1] - K.GUARD_RB + 0.03))):
            E.append(rect("guard-bracket-%d" % (i + 1), "gearbox", "detail", 3.98, 0.4, *STEEL, gx - 0.04, -1.2, gx + 0.04, top))
    E.append(poly("motor", "motor", "body", 3.2, 0.68, "motor", "motor-steel", motor_pts(s)))
    if s["terminal"]:
        E.append(rect("terminal-box", "motor", "body", 3.25, 0.5, "motor", "motor-steel", -0.11, 0.24, 0.11, 0.38))
    E.append(poly("belt-guard", "gearbox", "body", 4.0, 0.45, *STEEL, guard_pts(s)))
    return E


def moving(scale, view):
    """The parts that move with the stroke, in front and plan: the elevation profile of their bone extruded across their lateral span."""
    s = LEVEL[scale]
    F = []
    zf = 1 if view == "front" else 0
    F.append(poly("beam", "beam", "body", 3.5, 0.7, *STEEL, beam_pts(), **solid(-K.BEAM_BF / 2, K.BEAM_BF / 2)))
    F.append(poly("horsehead", "horsehead", "body", 3.45 if zf else 3.55, 0.78, *STEEL, head_pts(s), **solid(-(K.HEAD_HW if s["web"] else K.FACE_HW), K.HEAD_HW if s["web"] else K.FACE_HW)))
    if s["web"]:
        F.append(poly("horsehead-web", "horsehead", "detail", 3.46 if zf else 3.6, 0.3, *STEEL, [K.L2("horsehead", K.W2("beam", p)) for p in K.face_band(s["head"])], **solid(-K.FACE_HW, K.FACE_HW)))
    F.append(poly("centre-bearing", "beam", "detail" if scale <= 100 else "body", 3.52 if zf else 3.65, 0.3, *STEEL, centre_housing(s), **solid(-K.HOUSING_R, K.HOUSING_R)))
    if scale < 500:
        F.append(poly("tail-pin", "beam", "detail" if scale <= 100 else "body", 3.55 if zf else 3.65, 0.3, *STEEL, tail_hanger(s), **solid(-0.15, 0.15)))
    F.append(rect("equalizer", "pitman", "body", 5.3 if zf else 3.95, 0.45, *STEEL, K.eq_u0(scale), -K.EQ_V, K.EQ_U1, K.EQ_V, **solid(-K.EQ_HW, K.EQ_HW)))
    for side, sg in (("l", 1), ("r", -1)):
        F.append(poly("pitman-" + side, "pitman", "body", 5.4 if zf else 3.97, 0.6, *STEEL, pitman_local(s), **solid(sg * K.PIT_Z0, sg * K.PIT_Z1)))
        if s["eye"]:
            F.append(ellipse("pitman-eye-" + side, "pitman", "detail", 5.45 if zf else 3.98, 0.3, *STEEL, K.P_PITMAN, 0, K.EYE_R, **solid(sg * K.EYE_Z0, sg * K.EYE_Z1)))
    hu, hv = K.carrier_h(scale)
    F.append(rect("carrier", "rod", "body", 3.3, 0.3, *STEEL, -hu, -hv, hu, hv, **solid(-K.CARRIER_HZ, K.CARRIER_HZ)))
    if s["clamp"]:
        F.append(rect("rod-clamp", "rod", "detail", 3.32, 0.25, *STEEL, K.CARRIER_HU, -0.06, K.CARRIER_HU + 0.10, 0.06, **solid(-0.06, 0.06)))
    for side, sg in (("l", 1), ("r", -1)):
        F.append(poly("counterweight-" + side, "crank", "body", (5.76 if zf else 3.76) if scale == 500 else (5.7 if zf else 3.7), 0.4, "crank", "cast-iron", counterweight(s), **solid(sg * K.CW_Z0, sg * (K.ARM_Z1 if scale == 500 else K.CW_Z1))))
        F.append(poly("crank-arm-" + side, "crank", "body", 5.75 if zf else 3.75, 0.58, "crank", "cast-iron", crank_arm(s), **solid(sg * K.ARM_Z0, sg * K.ARM_Z1)))
    if s["hub"]:
        F.append(ellipse("crank-hub", "crank", "detail" if scale <= 100 else "body", 5.65 if zf else 3.85, 0.3, "crank", "cast-iron", 0, 0, K.HUB_R, **solid(-K.HUB_Z1, K.HUB_Z1)))
    return F


def front(scale):
    s = LEVEL[scale]
    F = []
    F.append(rect("slab", "base", "body", 3, 0.82, "base", "concrete", -K.SLAB[4], 0, K.SLAB[4], K.SLAB[3]))
    if s["skid_notch"]:
        tw = (K.IB_TW if scale == 50 else 0.015) / 2
        sk = [(-0.8, 0.30), (-1.0, 0.30), (-1.0, 0.315), (-0.9 - tw, 0.315), (-0.9 - tw, 0.525), (-1.0, 0.525), (-1.0, 0.55), (1.0, 0.55), (1.0, 0.525), (0.9 + tw, 0.525), (0.9 + tw, 0.315), (1.0, 0.315), (1.0, 0.30), (0.8, 0.30)]
        F.append(poly("skid", "skid", "body", 3.05, 0.5, "base", "painted-steel", sk))
    else:
        F.append(rect("skid", "skid", "body", 3.05, 0.5, "base", "painted-steel", -K.SKID_Z, K.SKID_Y0, K.SKID_Z, K.SKID_Y1))
    F += wellhead_elev(s, "front")
    F.append(rect("polished-rod", "base", "body", 3.2, 0.9, *STEEL, -s["rod_w"] / 2, 1.28, s["rod_w"] / 2, K.PIVOT[1] + 0.025))
    for id, sz in (("leg-left", 1), ("leg-right", -1)):
        p0, p1 = K.leg_axis(-1, sz)
        F.append(poly(id, "post", "body", 3.3, 0.62, *STEEL, K.strip(p0, p1, K.LEG / 2, K.SKID_Y1, K.CAP_Y0, a=2)))
    if s["brace"]:
        zb = K.FOOT_Z - (K.FOOT_Z - K.TOP_Z) * (K.BRACE_Y - 0.55) / (K.TOP_Y - 0.55)
        F.append(rect("brace", "post", "body", 3.35, 0.55, *STEEL, -zb - K.BRACE_T / 2, K.BRACE_Y - K.BRACE_T / 2, zb + K.BRACE_T / 2, K.BRACE_Y + K.BRACE_T / 2))
    sad = [(-K.CAP_HW, K.cap_y0(scale)), (K.CAP_HW, K.cap_y0(scale)), (K.CAP_HW, K.CAP_Y1), (K.BLOCK_Z1, K.CAP_Y1), (K.BLOCK_Z1, K.BLOCK_Y1), (K.BLOCK_Z0, K.BLOCK_Y1), (K.BLOCK_Z0, K.CAP_Y1), (-K.BLOCK_Z0, K.CAP_Y1), (-K.BLOCK_Z0, K.BLOCK_Y1), (-K.BLOCK_Z1, K.BLOCK_Y1), (-K.BLOCK_Z1, K.CAP_Y1), (-K.CAP_HW, K.CAP_Y1)]
    F.append(poly("saddle", "post", s["saddle_role"], 3.45, 0.3, *STEEL, sad))
    F += moving(scale, "front")
    gy0 = K.CRANK[1] - 1.05
    iy = K.CRANK[1] + K.INPUT[1]
    cap = [(K.GB_HW, iy - 0.16), (K.GB_HW + 0.05, iy - 0.16), (K.GB_HW + 0.05, iy + 0.16), (K.GB_HW, iy + 0.16)] if scale <= 100 else []
    F.append(poly("gearbox", "gearbox", "body", 5.9, 0.62, "gearbox", "cast-iron", [(-K.GB_HW, gy0), (K.GB_HW, gy0)] + cap + [(K.GB_HW, K.CRANK[1] + 0.60), (-K.GB_HW, K.CRANK[1] + 0.60)]))
    F.append(rect("pedestal", "gearbox", "body", 5.85, 0.5, "gearbox", "cast-iron", -0.40, 0.55, 0.40, gy0))
    if s["shafts"]:
        iy = K.CRANK[1] + K.INPUT[1]
        F.append(rect("input-shaft", "gearbox", "detail" if scale <= 100 else "body", 5.95, 0.35, "gearbox", "cast-iron", K.GUARD_Z1, iy - 0.07, -K.GB_HW, iy + 0.07))
    gy = [K.CRANK[1] + p[1] for p in guard_pts(s)]
    g0, g1 = min(gy), max(gy)
    if scale == 50:
        F.append(poly("belt-guard", "gearbox", "body", 7.5, 0.45, *STEEL, [(K.GUARD_Z0, g0), (-1.11, g0), (-1.11, 0.55), (-0.95, 0.55), (-0.95, g0), (K.GUARD_Z1, g0), (K.GUARD_Z1, g1), (K.GUARD_Z0, g1)]))
    else:
        F.append(rect("belt-guard", "gearbox", "body", 7.5, 0.45, *STEEL, K.GUARD_Z0, g0, K.GUARD_Z1, g1))
    my = K.MOTOR[1]
    F.append(rect("motor-feet", "motor", "body", 7.9, 0.5, "motor", "motor-steel", -0.55, 0.55, 0.13, my - 0.22))
    if scale <= 100:
        F.append(poly("motor", "motor", "body", 8, 0.68, "motor", "motor-steel", [(K.MOTOR_Z0 - 0.085, my - 0.07), (K.MOTOR_Z0 - 0.03, my - 0.225), (K.MOTOR_Z0, my - K.MOTOR_R), (K.MOTOR_Z1, my - K.MOTOR_R), (K.MOTOR_Z1, my + K.MOTOR_R), (K.MOTOR_Z0, my + K.MOTOR_R), (K.MOTOR_Z0 - 0.03, my + 0.225), (K.MOTOR_Z0 - 0.085, my + 0.07)]))
    else:
        F.append(rect("motor", "motor", "body", 8, 0.68, "motor", "motor-steel", K.MOTOR_Z0 - 0.02, my - K.MOTOR_R + 0.005, K.COWL_Z1, my + K.MOTOR_R - 0.005))
    if scale <= 100:
        F.append(poly("motor-cowl", "motor", "body", 8.1, 0.5, "motor", "motor-steel", [(K.MOTOR_Z1, my - K.COWL_R), (K.COWL_Z1 - 0.03, my - K.COWL_R), (K.COWL_Z1, my - K.COWL_R + 0.03), (K.COWL_Z1, my + K.COWL_R - 0.03), (K.COWL_Z1 - 0.03, my + K.COWL_R), (K.MOTOR_Z1, my + K.COWL_R)]))
    if s["terminal"]:
        F.append(rect("terminal-box", "motor", "body", 8.2, 0.5, "motor", "motor-steel", -0.31, my + 0.24, -0.09, my + 0.38))
    if s["shafts"]:
        F.append(rect("motor-shaft", "motor", "detail" if scale <= 100 else "body", 8.3, 0.3, "motor", "motor-steel", K.GUARD_Z1, my - (0.04 if scale == 500 else 0.035), K.MOTOR_Z0 - 0.08, my + (0.04 if scale == 500 else 0.035)))
    return F


def plan(scale):
    s = LEVEL[scale]
    P = []
    P.append(rect("slab", "base", "body", 3, 0.82, "base", "concrete", K.SLAB[0], -K.SLAB[4], K.SLAB[1], K.SLAB[4]))
    P.append(rect("skid", "skid", "body", 3.05, 0.5, "base", "painted-steel", K.SKID_X0, -K.SKID_Z, K.SKID_X1, K.SKID_Z))
    W = ("wellhead", "painted-steel")
    if s["wellhead"] == 5:
        P.append(ellipse("wellhead-base", "base", "body", 3.1, 0.55, *W, K.WELL_X, 0, 0.17))
        P.append(ellipse("wellhead-flange", "base", "body", 3.15, 0.7, *W, K.WELL_X, 0, 0.2))
        P.append(ellipse("wellhead-valve", "base", "body", 3.16, 0.7, *W, K.WELL_X, 0, 0.1))
        P.append(ellipse("stuffing-box", "base", "body", 3.2, 0.45, *W, K.WELL_X, 0, 0.08))
        P.append(rect("flowline", "base", "body", 3.1, 0.55, *W, -2.83, -0.06, -2.0, 0.06))
    elif s["wellhead"] == 2:
        P.append(ellipse("wellhead-base", "base", "body", 3.1, 0.55, *W, K.WELL_X, 0, 0.2))
        P.append(ellipse("wellhead-top", "base", "body", 3.15, 0.6, *W, K.WELL_X, 0, 0.1))
    else:
        P.append(ellipse("wellhead", "base", "body", 3.1, 0.6, *W, K.WELL_X, 0, 0.2))
    P.append(ellipse("polished-rod", "base", "body", 3.2, 0.9, *STEEL, K.WELL_X, 0, max(0.04, s["rod_w"] / 2)))
    P.append(shape("polished-rod-axis", "base", "body", 3.3, 0.9, "path", *STEEL, filled=False, d="M -3 -0.001 L -3 0.001", penMm=1.0 if scale == 50 else (0.5 if scale == 100 else (0.25 if scale == 200 else 0.1))))
    for id, sx, sz in (("leg-fl", -1, 1), ("leg-fr", -1, -1), ("leg-rl", 1, 1), ("leg-rr", 1, -1)):
        p0, p1 = K.leg_axis(sx, sz)
        P.append(shape(id, "post", "body", 3.3, 0.62, "capsule", *STEEL, x0=r4(p0[0]), y0=r4(p0[2]), x1=r4(p1[0]), y1=r4(p1[2]), r0=r4(K.LEG / 2), r1=r4(K.LEG / 2)))
    if s["brace"]:
        f = (K.BRACE_Y - 0.55) / (K.TOP_Y - 0.55)
        xb = K.FOOT_X - (K.FOOT_X - K.TOP_X) * f
        zb = K.FOOT_Z - (K.FOOT_Z - K.TOP_Z) * f
        h = K.BRACE_T / 2
        P.append(path("brace", "post", "body", 3.35, 0.55, *STEEL, [[(-xb - h, -zb - h), (xb + h, -zb - h), (xb + h, zb + h), (-xb - h, zb + h)], [(-xb + h, -zb + h), (xb - h, -zb + h), (xb - h, zb - h), (-xb + h, zb - h)]]))
    P.append(poly("saddle", "post", s["saddle_role"], 3.45, 0.3, *STEEL, [(-K.CAP_HW, -K.BLOCK_Z1), (K.CAP_HW, -K.BLOCK_Z1), (K.CAP_HW, K.BLOCK_Z1), (-K.CAP_HW, K.BLOCK_Z1)]))
    P.append(rect("gearbox", "gearbox", "body", 3.2, 0.62, "gearbox", "cast-iron", K.CRANK[0] - 0.70, -K.GB_HW, K.CRANK[0] + 1.72, K.GB_HW))
    if s["shafts"]:
        ix = K.CRANK[0] + K.INPUT[0]
        P.append(rect("input-shaft", "gearbox", "detail" if scale <= 100 else "body", 3.22, 0.35, "gearbox", "cast-iron", ix - 0.07, K.GUARD_Z1, ix + 0.07, -K.GB_HW))
    gx = [K.CRANK[0] + p[0] for p in guard_pts(s)]
    P.append(rect("belt-guard", "gearbox", "body", 3.25, 0.45, *STEEL, min(gx), K.GUARD_Z0, max(gx), K.GUARD_Z1))
    mx = K.MOTOR[0]
    P.append(rect("motor-feet", "motor", "body", 3.15, 0.5, "motor", "motor-steel", mx - 0.32, -0.55, mx + 0.32, 0.13))
    if scale <= 100:
        P.append(poly("motor", "motor", "body", 3.2, 0.68, "motor", "motor-steel", [(mx - 0.07, K.MOTOR_Z0 - 0.085), (mx + 0.07, K.MOTOR_Z0 - 0.085), (mx + 0.225, K.MOTOR_Z0 - 0.03), (mx + K.MOTOR_R, K.MOTOR_Z0), (mx + K.MOTOR_R, K.MOTOR_Z1), (mx - K.MOTOR_R, K.MOTOR_Z1), (mx - K.MOTOR_R, K.MOTOR_Z0), (mx - 0.225, K.MOTOR_Z0 - 0.03)]))
    else:
        P.append(rect("motor", "motor", "body", 3.2, 0.68, "motor", "motor-steel", mx - K.MOTOR_R + 0.005, K.MOTOR_Z0 - 0.02, mx + K.MOTOR_R - 0.005, K.COWL_Z1))
    if scale <= 100:
        P.append(poly("motor-cowl", "motor", "body", 3.2, 0.5, "motor", "motor-steel", [(mx - K.COWL_R, K.MOTOR_Z1), (mx + K.COWL_R, K.MOTOR_Z1), (mx + K.COWL_R, K.COWL_Z1 - 0.03), (mx + K.COWL_R - 0.03, K.COWL_Z1), (mx - K.COWL_R + 0.03, K.COWL_Z1), (mx - K.COWL_R, K.COWL_Z1 - 0.03)]))
    if s["terminal"]:
        P.append(rect("terminal-box", "motor", "body", 3.25, 0.5, "motor", "motor-steel", mx - 0.11, -0.31, mx + 0.11, -0.09))
    if s["shafts"]:
        P.append(rect("motor-shaft", "motor", "detail" if scale <= 100 else "body", 3.3, 0.3, "motor", "motor-steel", mx - (0.04 if scale == 500 else 0.035), K.GUARD_Z1, mx + (0.04 if scale == 500 else 0.035), K.MOTOR_Z0 - 0.08))
    P += moving(scale, "plan")
    return P


def geometry(scale):
    return {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
