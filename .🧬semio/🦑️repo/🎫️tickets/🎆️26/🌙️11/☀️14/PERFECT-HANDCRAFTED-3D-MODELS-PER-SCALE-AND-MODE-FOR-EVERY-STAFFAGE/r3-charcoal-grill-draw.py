"""🔥 Round 3 charcoal kettle grill: the 2D drawings of every scale (side elevation, front, plan) drawn as the true views of that scale's body, from the same real geometry the solids are stated with.

Frames: bowl shapes in world coordinates (elevation x, y; front z, y; plan x, z); lid shapes in the lid frame and lid-vent shapes in the vent frame (u up), carried to front and plan by `solid` hints; ash-damper shapes in its frame (u down, v ahead) in the elevation and in world rest coordinates in the front view.
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-charcoal-grill-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

r5 = K.r5
REVOLVED_LID = {"kind": "revolved", "axisBone": "lid", "axis": "v", "axisX": 0.2925}
REVOLVED_VENT = {"kind": "revolved", "axisBone": "lid-vent", "axis": "u", "axisX": 0.0}
ENAMEL = {"region": "lid-shell", "material": "enamel-steel"}
BOWL = {"region": "bowl-shell", "material": "enamel-steel"}
WOOD = {"region": "handle-wood", "material": "wood-handle"}
LEGS = {"region": "legs-steel", "material": "steel-black"}
VENTS = {"region": "vents-alu", "material": "alu-cast"}
PAN = {"region": "ash-catcher", "material": "alu-cast"}


def text(en, de):
    return {"en": en, "de": de}


def pts(seq):
    return [[r5(x), r5(y)] for x, y in seq]


def shape(sid, bone, role, z, filled, tone, kind, extra, meta=None, **more):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(extra)
    if meta:
        s.update({k: v for k, v in meta.items() if filled or k != "material"})
    s.update(more)
    return s


def poly(sid, bone, role, z, tone, points, meta=None, closed=True, filled=None, **more):
    return shape(sid, bone, role, z, closed if filled is None else filled, tone, "poly", {"points": pts(points), "closed": closed}, meta, **more)


def stroke(sid, bone, role, z, tone, points, meta=None, **more):
    return poly(sid, bone, role, z, tone, points, meta, closed=False, filled=False, **more)


def ellipse(sid, bone, role, z, tone, cx, cy, rx, ry, meta=None, filled=True, **more):
    return shape(sid, bone, role, z, filled, tone, "ellipse", {"cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}, meta, **more)


def capsule(sid, bone, role, z, tone, a, b, r, meta=None, **more):
    return shape(sid, bone, role, z, True, tone, "capsule", {"x0": r5(a[0]), "y0": r5(a[1]), "x1": r5(b[0]), "y1": r5(b[1]), "r0": r5(r), "r1": r5(r)}, meta, **more)


def superellipse(cx, cy, a, b, n=2.6, k=16):
    out = []
    for i in range(k):
        t = 2 * math.pi * i / k
        c, s = math.cos(t), math.sin(t)
        out.append((cx + a * math.copysign(abs(c) ** (2 / n), c), cy + b * math.copysign(abs(s) ** (2 / n), s)))
    return out


def rrect(x0, y0, x1, y1, r, k=3):
    out = []
    for cx, cy, a0 in ((x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180), (x1 - r, y0 + r, 270)):
        out += K.arc(cx, cy, r, a0, a0 + 90, k)
    return out


def mirror_x(half, axis=0.0):
    return half + [(2 * axis - x, y) for x, y in reversed(half[1:-1])]


def lid_outline(n, bead):
    """⛑️ Lid dome outline in the lid frame, with the rolled bead at 1:50 or down to the 742 mm rim line at coarser scales."""
    if bead:
        dome = [(0.2925 + r, y - K.LID_Y0) for r, y in K.lid_profile_outer(n) if r <= 0.2866]
        bv = K.LBEAD_Y - K.LID_Y0
        right = dome + [(0.2925 + K.RIM_R + K.BEAD_R * math.cos(math.radians(a)), bv + K.BEAD_R * math.sin(math.radians(a))) for a in (60, 25, 0, -40, -90)]
    else:
        a_end = math.acos((0.742 - K.C_L) / K.RHO_L)
        right = [(0.2925 + K.RHO_L * math.sin(a_end * k / n), K.C_L + K.RHO_L * math.cos(a_end * k / n) - K.LID_Y0) for k in range(n + 1)]
    right = list(reversed(right))
    return right + [(0.585 - u, v) for u, v in reversed(right[:-1])]


def bowl_outline(n, bead):
    if bead:
        out, _ = K.bowl_outline(n)
        return out
    a_end = math.asin(K.bowl_r(0.742) / K.RHO_B)
    right = [(K.RHO_B * math.sin(a_end * k / n), K.C_B - K.RHO_B * math.cos(a_end * k / n)) for k in range(n + 1)]
    return [(-x, y) for x, y in reversed(right)] + right[1:]


def pan_outline(lip=True):
    b, t, r = K.PAN_BOTTOM, K.PAN_TOP, K.PAN_R
    right = [(0.136, b), (0.1445, b + 0.0015), (0.148, b + 0.005), (r, t - 0.002)] + ([(r + 0.0045, t), (r + 0.006, t + 0.003)] if lip else [(r + 0.004, t)])
    return [(-x, y) for x, y in reversed(right)] + right


def pan_against_bowl(low, r=K.PAN_R, chamfer=0.01):
    """🍳 The pan mass of the coarser scales: from its bottom up to the bowl underside, outline in elevation and front."""
    top = [(x, K.bowl_y(abs(x)) - 0.0004) for x in (r, 0.1, 0.05, 0.0, -0.05, -0.1, -r)]
    return [(-r + chamfer, low), (r - chamfer, low), (r, low + chamfer)] + top + [(-r, low + chamfer)]


def leg_x(side, y):
    return K.leg_point(side, y)


def grip_profile():
    return superellipse(K.LID_GRIP_U, K.LID_GRIP_V, 0.0152, 0.0132)


def lid_bracket_u(w):
    cv = K.C_L - K.LID_Y0
    return 0.2925 + math.sqrt(K.RHO_L ** 2 - (K.LID_GRIP_V - cv) ** 2 - w * w)


def side_handle_elevation(ry_mid=0.0158):
    xs = [(-0.065, 0.008), (-0.061, 0.012), (-0.053, 0.0145), (-0.03, ry_mid - 0.0003), (0.0, ry_mid), (0.03, ry_mid - 0.0003), (0.053, 0.0145), (0.061, 0.012), (0.065, 0.008)]
    return [(x, K.HANDLE_Y + ry) for x, ry in xs] + [(x, K.HANDLE_Y - ry) for x, ry in reversed(xs)]


def side_handle_plan(s):
    xs = [(-0.065, 0.007), (-0.061, 0.0105), (-0.053, 0.0125), (0.0, 0.0132), (0.053, 0.0125), (0.061, 0.0105), (0.065, 0.007)]
    return [(x, s * K.HANDLE_Z + rx) for x, rx in xs] + [(x, s * K.HANDLE_Z - rx) for x, rx in reversed(xs)]


def wheel_front(sign, hub=True, half=0.019):
    """🛞 A wheel seen along its axle direction turned 90 degrees: tread band with rounded shoulders and, at 1:50, the hub cap."""
    zc = sign * K.WHEEL_Z
    top = [(-half, 0.064), (-half, 0.07), (-half + 0.0025, 0.0745), (-half + 0.007, 0.076), (half - 0.007, 0.076), (half - 0.0025, 0.0745), (half, 0.07)]
    if hub:
        top += [(half, 0.024), (0.024, 0.018)]
    upper = [(zc + sign * a, K.AXLE_Y + r) for a, r in top]
    lower = [(zc + sign * a, K.AXLE_Y - r) for a, r in reversed(top)]
    out = upper + lower
    return out if sign > 0 else list(reversed(out))


def wheel_plan(sign, hub=True, half=0.019):
    ax = K.axle_x()
    zc = sign * K.WHEEL_Z
    z_in, z_out = zc - sign * half, zc + sign * half
    lo, hi = min(z_in, z_out), max(z_in, z_out)
    ring = rrect(ax - K.WHEEL_RAD, lo, ax + K.WHEEL_RAD, hi, 0.007)
    if not hub:
        return ring
    cap = (hi, hi + 0.005) if sign > 0 else (lo - 0.005, lo)
    return ring, rrect(ax - 0.024, cap[0] - (0.002 if sign > 0 else 0), ax + 0.024, cap[1] + (0.002 if sign < 0 else 0), 0.002, 2)


def damper_elevation():
    """🌀 The bottom damper in its own frame (x = u down, y = v ahead): the dished plate and, separately, the lever with its knob."""
    us = K.damper_u_surface
    plate = [(us(abs(v)) - 0.0004, v) for v in (-0.062, -0.04, -0.02, 0.0, 0.02, 0.04, 0.062)]
    plate += [(us(abs(v)) + 0.0041, v) for v in (0.062, 0.04, 0.02, 0.0, -0.02, -0.04, -0.062)]
    centre = [K.lever_point(s) for s in (0.05, 0.09, K.LEVER_END)]
    upper = [(u - 0.0035, v) for u, v, _ in centre]
    lower = [(u + 0.0035, v) for u, v, _ in reversed(centre)]
    ku, kv, _ = K.lever_point(K.LEVER_END + 0.007, 0.0105)
    knob = [(ku + 0.009 * math.cos(t), kv + 0.0055 * math.sin(t)) for t in [math.radians(a) for a in range(-90, 271, 45)]]
    return plate, upper + knob[1:-1] + lower


def highlight_lid():
    out = []
    for a in (128, 118, 106, 96):
        t = math.radians(a)
        out.append((0.2925 + (K.RHO_L - 0.03) * math.cos(t), K.C_L + (K.RHO_L - 0.03) * math.sin(t) - K.LID_Y0))
    return out


def highlight_bowl():
    out = []
    for a in (205, 222, 240, 255):
        t = math.radians(a)
        out.append(((K.RHO_B - 0.03) * math.cos(t), K.C_B + (K.RHO_B - 0.03) * math.sin(t)))
    return out


def lid_set(scale, view):
    """⛑️ The lid, its handle and the vent dial: the same lid-frame profiles in every view, front and plan through their solid hints."""
    fine = scale == 50
    n = {50: 14, 100: 10, 200: 8, 500: 6}[scale]
    hint = None if view == "elevation" else dict(REVOLVED_LID, mirror=view == "front")
    meta = ENAMEL if scale != 500 else {"regions": ["lid-shell", "vents-alu"], "material": "enamel-steel"}
    out = [poly("lid", "lid", "body", 3.2, 0.3, lid_outline(n, fine), meta, **({"solid": hint} if hint else {}))]
    if fine:
        line = [(0.004, 0.0102), (0.581, 0.0102)]
        if view == "elevation":
            out.append(stroke("lid-rim", "lid", "detail", 3.6, 0.3, line, {"region": "lid-shell"}, penMm=0.246))
            out.append(stroke("lid-bead", "lid", "texture", 3.6, 0.3, highlight_lid(), {"region": "lid-shell"}))
        elif view == "plan":
            out.append(stroke("lid-rim-ring", "lid", "detail", 3.6, 0.3, line, {"region": "lid-shell"}, penMm=0.246, solid=dict(REVOLVED_LID, mirror=False)))
    if scale in (50, 100):
        grip = grip_profile()
        ext = lambda z0, z1: {} if view == "elevation" else {"solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}
        out.append(poly("handle", "lid", "body", 3.5, 0.55, grip, WOOD, **({"hatch": "grain"} if fine else {}), **ext(-0.08, 0.08)))
        rb = 0.005 if fine else 0.0075
        u0 = lid_bracket_u(0.062) - 0.0005
        bracket = rrect(u0, K.LID_GRIP_V - rb, K.LID_GRIP_U - 0.004, K.LID_GRIP_V + rb, rb * 0.6, 2)
        if view == "elevation":
            out.append(poly("handle-bracket", "lid", "body", 3.45, 0.25, bracket, {"region": "lid-shell", "material": "steel-black"}))
        else:
            for side, w in (("l", 0.062), ("r", -0.062)):
                out.append(poly("handle-bracket-%s" % side, "lid", "body", 3.45, 0.25, bracket, {"region": "lid-shell", "material": "steel-black"}, **ext(w - rb, w + rb)))
    else:
        u_in = lid_bracket_u(0.0) - 0.0006
        h = 0.015 if scale == 200 else 0.0375
        vc = K.LID_GRIP_V if scale == 200 else 0.055
        block = rrect(u_in, vc - h, 0.64, vc + h, 0.008 if scale == 200 else 0.012, 2)
        out.append(poly("handle", "lid", "body", 3.5, 0.55, block, WOOD, **({} if view == "elevation" else {"solid": {"kind": "extruded", "sweep": "w", "z0": -0.08, "z1": 0.08}})))
    if scale == 500:
        return out
    vhint = dict(REVOLVED_VENT, mirror=view != "plan")
    if fine:
        dial = [(K.dome_u(0.0405) - 0.0004, -0.0405), (0.002, -0.0405), (0.002, -0.04), (0.008, -0.04), (0.008, 0.04), (0.002, 0.04), (0.002, 0.0405), (K.dome_u(0.0405) - 0.0004, 0.0405), (-0.0004, 0.0)]
    else:
        top = K.FIN_U1
        dial = [(K.dome_u(0.0405) - 0.0004, -0.0405), (top - 0.004, -0.0405), (top, -0.0365), (top, 0.0365), (top - 0.004, 0.0405), (K.dome_u(0.0405) - 0.0004, 0.0405), (-0.0004, 0.0)]
    out.append(poly("vent-dial", "lid-vent", "body", 3.4, 0.7, dial, VENTS, solid=vhint))
    if fine:
        fin = rrect(0.007, -0.035, K.FIN_U1, 0.035, 0.003, 2)
        out.append(poly("vent-fin", "lid-vent", "body", 3.45, 0.7, fin, VENTS, solid={"kind": "extruded", "sweep": "w", "z0": -0.004, "z1": 0.004}))
        if view == "plan":
            for i, (v, w) in enumerate(K.HOLES_VW):
                hole = [(0.002, v - K.HOLE_R), (K.DIAL_U1, v - K.HOLE_R), (K.DIAL_U1, v + K.HOLE_R), (0.002, v + K.HOLE_R)]
                out.append(poly("vent-hole-%d" % i, "lid-vent", "detail", 3.5, 0.1, hole, {"region": "vents-alu"}, solid={"kind": "revolved", "axisBone": "lid-vent", "axis": "u", "axisX": r5(-v), "axisZ": r5(w), "mirror": False}))
    return out


def elevation(scale):
    fine = scale == 50
    n = {50: 16, 100: 12, 200: 10, 500: 7}[scale]
    g = lid_set(scale, "elevation")
    g.append(poly("bowl", "bowl", "body", 3.0, 0.3, bowl_outline(n, fine), BOWL))
    if fine:
        g.append(stroke("rim-line", "bowl", "detail", 3.6, 0.3, [(-0.2885, K.BEAD_Y - K.BEAD_R), (0.2885, K.BEAD_Y - K.BEAD_R)], {"region": "bowl-shell"}, penMm=0.246))
        g.append(stroke("bowl-highlight", "bowl", "texture", 3.6, 0.3, highlight_bowl(), {"region": "bowl-shell"}))
    if scale in (50, 100):
        g.append(poly("side-handle", "bowl", "body", 3.4, 0.55, side_handle_elevation(), WOOD, **({"hatch": "grain"} if fine else {})))
    elif scale == 200:
        g.append(poly("side-handle", "bowl", "body", 3.4, 0.55, rrect(-0.065, K.HANDLE_Y - 0.015, 0.065, K.HANDLE_Y + 0.015, 0.008, 2), WOOD))
    if scale in (50, 100):
        g.append(poly("ash-catcher", "bowl", "body", 2.9, 0.65, pan_outline(fine), PAN))
        y = K.PAN_TOP + (0.0015 if fine else -0.003)
        g.append(stroke("pan-hanger-front", "bowl", "detail", 2.85, 0.25, [(0.15, y), (K.leg_r(y), y)], {"region": "legs-steel"}))
        g.append(stroke("pan-hanger-rear", "bowl", "detail", 3.06, 0.25, [(-0.5 * 0.146, y), (-0.5 * K.leg_r(y), y)], {"region": "legs-steel"}))
    else:
        g.append(poly("ash-catcher", "bowl", "body", 2.9, 0.65, pan_against_bowl(K.BOWL_BOTTOM - 0.03 if scale == 200 else 0.405), PAN))
    top_y = K.socket_top_y()
    fx = lambda side, y: K.leg_point(side, y)[0]
    if fine:
        g.append(capsule("leg-front", "bowl", "body", 2.5, 0.25, (fx("front", 0.022), 0.022), (fx("front", 0.535), 0.535), K.LEG_R, LEGS))
        g.append(capsule("leg-socket-front", "bowl", "body", 2.55, 0.25, (fx("front", 0.49), 0.49), (fx("front", top_y - 0.004), top_y - 0.004), 0.0165, LEGS))
        g.append(poly("foot-front", "bowl", "body", 2.56, 0.15, [(fx("front", 0.0) - 0.0168, 0.0), (fx("front", 0.0) + 0.0168, 0.0), (fx("front", 0.03) + 0.0165, 0.03), (fx("front", 0.03) - 0.0165, 0.03)], LEGS))
        g.append(capsule("leg-wheel", "bowl", "body", 3.05, 0.25, (fx("right", 0.062), 0.062), (fx("right", 0.535), 0.535), K.LEG_R, LEGS))
        g.append(capsule("leg-socket-wheel", "bowl", "body", 3.06, 0.25, (fx("right", 0.49), 0.49), (fx("right", top_y - 0.004), top_y - 0.004), 0.0165, LEGS))
    elif scale == 100:
        g.append(capsule("leg-front", "bowl", "body", 2.5, 0.25, (fx("front", 0.012), 0.012), (fx("front", top_y - 0.01), top_y - 0.01), 0.0135, LEGS))
        g.append(capsule("leg-wheel", "bowl", "body", 3.05, 0.25, (fx("right", 0.062), 0.062), (fx("right", top_y - 0.01), top_y - 0.01), 0.0125, LEGS))
    else:
        bottom_w = K.AXLE_Y - (0.012 if scale == 200 else 0.0)
        g.append(stroke("leg-front", "bowl", "body", 2.5, 0.25, [(fx("front", 0.0), 0.0), (fx("front", top_y), top_y)], LEGS))
        g.append(stroke("leg-wheel", "bowl", "body", 3.05, 0.25, [(fx("right", bottom_w), bottom_w), (fx("right", top_y), top_y)], LEGS))
    g.append(ellipse("wheel", "bowl", "body", 3.1, 0.15, K.axle_x(), K.AXLE_Y, K.WHEEL_RAD, K.WHEEL_RAD, LEGS))
    if fine:
        g.append(ellipse("wheel-hub", "bowl", "detail", 3.15, 0.3, K.axle_x(), K.AXLE_Y, 0.024, 0.024, {"region": "legs-steel"}))
    if scale != 500:
        tri = [(K.leg_point("left", K.TRI_Y)[0], K.TRI_Y), (K.leg_point("front", K.TRI_Y)[0], K.TRI_Y)]
        g.append(stroke("leg-triangle", "bowl", "detail", 3.07, 0.25, tri, {"region": "legs-steel"}))
    if fine:
        g += damper_shapes(True)
    return g


def damper_shapes(mirror):
    """🌀 Plate (revolved about the damper's vertical axis) and lever (extruded across it), so side and front views follow the damper's twist."""
    plate, lever = damper_elevation()
    return [poly("damper", "ash-damper", "body", 3.02, 0.7, plate, VENTS, solid={"kind": "revolved", "axisBone": "ash-damper", "axis": "u", "axisX": 0.0, "mirror": mirror}),
            poly("damper-lever", "ash-damper", "detail", 3.03, 0.7, lever, {"region": "vents-alu"}, solid={"kind": "extruded", "sweep": "w", "z0": -0.0065, "z1": 0.0065})]


def front(scale):
    fine = scale == 50
    n = {50: 16, 100: 12, 200: 10, 500: 7}[scale]
    g = lid_set(scale, "front")
    g.append(poly("bowl", "bowl", "body", 3.0, 0.3, bowl_outline(n, fine), BOWL))
    if fine:
        g.append(stroke("rim-line", "bowl", "detail", 3.6, 0.3, [(-0.2885, K.BEAD_Y - K.BEAD_R), (0.2885, K.BEAD_Y - K.BEAD_R)], {"region": "bowl-shell"}, penMm=0.246))
    for side, s in (("l", 1), ("r", -1)):
        if scale in (50, 100):
            g.append(poly("side-handle-%s" % side, "bowl", "body", 3.4, 0.55, superellipse(s * K.HANDLE_Z, K.HANDLE_Y, 0.0132, 0.0158), WOOD, **({"hatch": "grain"} if fine else {})))
            rb = 0.005 if fine else 0.0075
            z0, z1 = sorted((s * 0.2825, s * (K.HANDLE_Z - 0.008)))
            g.append(poly("side-bracket-%s" % side, "bowl", "body", 3.35, 0.25, [(z0, K.HANDLE_Y - rb), (z1, K.HANDLE_Y - rb), (z1, K.HANDLE_Y + rb), (z0, K.HANDLE_Y + rb)], {"region": "bowl-shell", "material": "steel-black"}))
        elif scale == 200:
            z0, z1 = sorted((s * 0.2826, s * 0.335))
            g.append(poly("side-handle-%s" % side, "bowl", "body", 3.4, 0.55, rrect(z0, K.HANDLE_Y - 0.015, z1, K.HANDLE_Y + 0.015, 0.008, 2), WOOD))
    if scale in (50, 100):
        g.append(poly("ash-catcher", "bowl", "body", 2.9, 0.65, pan_outline(fine), PAN))
        y = K.PAN_TOP + (0.0015 if fine else -0.003)
        for side, s in (("l", 1), ("r", -1)):
            g.append(stroke("pan-hanger-%s" % side, "bowl", "detail", 2.85, 0.25, [(s * 0.866 * (0.151 if fine else 0.146), y), (s * 0.866 * K.leg_r(y), y)], {"region": "legs-steel"}))
    else:
        g.append(poly("ash-catcher", "bowl", "body", 2.9, 0.65, pan_against_bowl(K.BOWL_BOTTOM - 0.03 if scale == 200 else 0.405), PAN))
    top_y = K.socket_top_y()
    fz = lambda side, y: K.leg_point(side, y)[2]
    if fine:
        g.append(capsule("leg-front", "bowl", "body", 3.25, 0.25, (0.0, 0.022), (0.0, 0.535), K.LEG_R, LEGS))
        g.append(capsule("leg-socket-front", "bowl", "body", 3.26, 0.25, (0.0, 0.49), (0.0, top_y - 0.004), 0.0165, LEGS))
        g.append(poly("foot-front", "bowl", "body", 3.27, 0.15, [(-0.0165, 0.0), (0.0165, 0.0), (0.0165, 0.03), (-0.0165, 0.03)], LEGS))
        for side in ("left", "right"):
            g.append(capsule("leg-%s" % side, "bowl", "body", 2.62, 0.25, (fz(side, 0.062), 0.062), (fz(side, 0.535), 0.535), K.LEG_R, LEGS))
            g.append(capsule("leg-socket-%s" % side, "bowl", "body", 2.63, 0.25, (fz(side, 0.49), 0.49), (fz(side, top_y - 0.004), top_y - 0.004), 0.0165, LEGS))
    elif scale == 100:
        g.append(capsule("leg-front", "bowl", "body", 3.25, 0.25, (0.0, 0.012), (0.0, top_y - 0.01), 0.0135, LEGS))
        for side in ("left", "right"):
            g.append(capsule("leg-%s" % side, "bowl", "body", 2.62, 0.25, (fz(side, 0.062), 0.062), (fz(side, top_y - 0.01), top_y - 0.01), 0.0125, LEGS))
    else:
        bottom_w = K.AXLE_Y - (0.012 if scale == 200 else 0.0)
        g.append(stroke("leg-front", "bowl", "body", 3.25, 0.25, [(0.0, 0.0), (0.0, top_y)], LEGS))
        for side in ("left", "right"):
            g.append(stroke("leg-%s" % side, "bowl", "body", 2.62, 0.25, [(fz(side, bottom_w), bottom_w), (fz(side, top_y), top_y)], LEGS))
    for side, s in (("l", 1), ("r", -1)):
        if scale == 500:
            zc = s * 0.2955
            g.append(poly("wheel-%s" % side, "bowl", "body", 2.64, 0.15, rrect(zc - 0.0375, 0.0, zc + 0.0375, 0.152, 0.012, 2), LEGS))
        else:
            g.append(poly("wheel-%s" % side, "bowl", "body", 2.64, 0.15, wheel_front(s, hub=fine), LEGS))
    if scale in (50, 100):
        g.append(capsule("axle", "bowl", "body", 2.6, 0.3, (-0.3, K.AXLE_Y), (0.3, K.AXLE_Y), 0.005, LEGS))
    elif scale == 200:
        g.append(stroke("axle", "bowl", "detail", 2.6, 0.3, [(-0.3, K.AXLE_Y), (0.3, K.AXLE_Y)], {"region": "legs-steel"}))
    if scale != 500:
        tz = K.leg_point("left", K.TRI_Y)[2]
        g.append(stroke("leg-triangle", "bowl", "detail", 2.61, 0.25, [(-tz, K.TRI_Y), (tz, K.TRI_Y)], {"region": "legs-steel"}))
    if fine:
        g += damper_shapes(True)
    return g


def plan(scale):
    fine = scale == 50
    g = lid_set(scale, "plan")
    g.append(ellipse("bowl", "bowl", "body", 3.0, 0.3, 0.0, 0.0, K.RIM_R + K.BEAD_R if fine else K.bowl_r(0.742), K.RIM_R + K.BEAD_R if fine else K.bowl_r(0.742), BOWL))
    if fine:
        g.append(ellipse("rim-ring", "bowl", "detail", 3.6, 0.3, 0.0, 0.0, K.RIM_R - K.BEAD_R, K.RIM_R - K.BEAD_R, {"region": "bowl-shell"}, filled=False, penMm=0.246))
    g.append(ellipse("charcoal-bed", "charcoal", "body", 3.05, 0.15, 0.0, 0.0, K.BED_A, K.BED_A, {"region": "charcoal-bed", "material": "charcoal"}))
    ring_r = K.GRATE_R if scale in (50, 100) else 0.2773
    g.append(ellipse("cooking-grate", "grate", "detail" if scale != 500 else "body", 3.1, 0.75, 0.0, 0.0, ring_r, ring_r, {"region": "cooking-grate"}, filled=False))
    bars = {50: (13, K.GRATE_PITCH), 100: (6, 0.039), 200: (3, 0.068), 500: (0, 0.0)}[scale]
    for k in range(-bars[0], bars[0] + 1) if bars[0] else []:
        z = k * bars[1]
        half = math.sqrt(ring_r ** 2 - z * z)
        g.append(stroke("cooking-grate-bar-%d" % (k + bars[0] + 1), "grate", "detail", 3.1, 0.75, [(-half, z), (half, z)], {"region": "cooking-grate"}))
    for side, s in (("l", 1), ("r", -1)):
        if scale in (50, 100):
            g.append(poly("side-handle-%s" % side, "bowl", "body", 3.4, 0.55, side_handle_plan(s), WOOD, **({"hatch": "grain"} if fine else {})))
            rb = 0.005 if fine else 0.0075
            for tag, x in (("a", -0.045), ("b", 0.045)):
                z0, z1 = sorted((s * 0.2825, s * (K.HANDLE_Z - 0.008)))
                g.append(poly("side-bracket-%s-%s" % (side, tag), "bowl", "body", 3.35, 0.25, [(x - rb, z0), (x + rb, z0), (x + rb, z1), (x - rb, z1)], {"region": "bowl-shell", "material": "steel-black"}))
        elif scale == 200:
            z0, z1 = sorted((s * 0.2826, s * 0.335))
            g.append(poly("side-handle-%s" % side, "bowl", "body", 3.4, 0.55, rrect(-0.065, z0, 0.065, z1, 0.008, 2), WOOD))
    for side, s in (("l", 1), ("r", -1)):
        if scale == 500:
            zc = s * 0.2955
            g.append(poly("wheel-%s" % side, "bowl", "body", 2.3, 0.15, rrect(K.axle_x() - K.WHEEL_RAD, zc - 0.0375, K.axle_x() + K.WHEEL_RAD, zc + 0.0375, 0.012, 2), LEGS))
        elif fine:
            ring, cap = wheel_plan(s)
            g.append(poly("wheel-%s" % side, "bowl", "body", 2.3, 0.15, ring, LEGS))
            g.append(poly("wheel-hub-%s" % side, "bowl", "detail", 2.31, 0.3, cap, {"region": "legs-steel"}))
        else:
            g.append(poly("wheel-%s" % side, "bowl", "body", 2.3, 0.15, wheel_plan(s, hub=False), LEGS))
    if scale in (50, 100):
        g.append(capsule("axle", "bowl", "body", 2.32, 0.3, (K.axle_x(), -0.3), (K.axle_x(), 0.3), 0.005, LEGS))
    elif scale == 200:
        g.append(stroke("axle", "bowl", "detail", 2.32, 0.3, [(K.axle_x(), -0.3), (K.axle_x(), 0.3)], {"region": "legs-steel"}))
    top_y = K.socket_top_y()
    for side in ("front", "left", "right"):
        y0 = 0.0 if side == "front" else K.AXLE_Y
        a, b = K.leg_point(side, top_y), K.leg_point(side, y0)
        if scale in (50, 100):
            g.append(capsule("leg-%s" % side, "bowl", "body", 2.35, 0.25, (a[0], a[2]), (b[0], b[2]), K.LEG_R if fine else 0.0135, LEGS))
        else:
            g.append(stroke("leg-%s" % side, "bowl", "body", 2.35, 0.25, [(a[0], a[2]), (b[0], b[2])], LEGS))
    if fine:
        g.append(ellipse("foot-front", "bowl", "body", 2.4, 0.15, K.FOOT_X, 0.0, 0.0168, 0.0165, LEGS))
    return g


TEXTS = {
    "50": {
        "silhouette": text("A 57 cm kettle grill 13.4 mm wide and 20 mm high as the true views of its body: lid dome with the rolled rim bead, the 80 mm vent dial with grip fin, the wooden lid handle on two rod brackets, bowl with bead and highlight, the wooden side handle, the bottom damper with its lever, the aluminium ash pan on its hanger, the tripod (front leg on its foot and the rear legs to the wheel axle, each in its socket), the 152 mm wheel with hub cap and the wire triangle. Front: the same lid and bowl, both side handles on their brackets, the front leg in front of the pan, the rear legs, wheels with hub caps on the axle, the triangle and the damper; plan: bowl with rim ring, lid with rim ring, vent dial with fin and three holes, handles and brackets, the grate with its 27 bars and the charcoal (seen with the lid raised), wheels, axle, legs and the front foot.",
                          "Ein 57-cm-Kugelgrill, 13,4 mm breit und 20 mm hoch, als wahre Ansichten seines Körpers: Deckelkuppel mit gerolltem Randwulst, die Lüftungsscheibe von 80 mm mit Griffrippe, der hölzerne Deckelgriff auf zwei Rundstabhaltern, Kessel mit Wulst und Lichtlinie, der hölzerne Seitengriff, der Bodenschieber mit Hebel, die Aluminium-Aschewanne an ihrem Halter, das Dreibein (vorderes Bein auf seinem Fuß, hintere Beine bis zur Radachse, jedes in seinem Halter), das Rad von 152 mm mit Nabenkappe und das Drahtdreieck. Vorderansicht: derselbe Deckel und Kessel, beide Seitengriffe auf ihren Haltern, das vordere Bein vor der Wanne, die hinteren Beine, Räder mit Nabenkappen auf der Achse, das Dreieck und der Schieber; Draufsicht: Kessel mit Randring, Deckel mit Randring, Lüftungsscheibe mit Rippe und drei Löchern, Griffe und Halter, der Rost mit seinen 27 Stäben und die Kohle (bei angehobenem Deckel sichtbar), Räder, Achse, Beine und der vordere Fuß."),
        "retained": text("Every visible part at real size; the dial and the damper lever turn with the twist of their bones, the lid, its handle and the dial follow every lid pose through the lid bone.",
                         "Jedes sichtbare Teil in realer Größe; Scheibe und Schieberhebel drehen mit der Verdrehung ihrer Knochen, Deckel, Griff und Scheibe folgen jeder Deckelpose über den Deckelknochen."),
        "merged": text("In the side elevation the left rear leg, wheel and socket lie behind the right ones and are one shape each; grate, fuel grate and charcoal are inside the bowl.",
                       "In der Seitenansicht liegen linkes Hinterbein, Rad und Halter hinter den rechten und sind je eine Form; Rost, Kohlerost und Kohle liegen im Kessel."),
        "omitted": text("Handle screws, the lid hook, the vent holes in the lid under the dial, the One-Touch blades inside the bowl.",
                        "Griffschrauben, Deckelhaken, die Lüftungslöcher im Deckel unter der Scheibe, die One-Touch-Flügel im Kessel."),
        "thinElements": text("Vent holes are 12 mm (0.24 mm) circles, brackets 10 mm rods, the damper plate 4.5 mm; grate bars (6 mm), pan hangers (8 mm) and the wire triangle (8 mm) are detail lines of 0.18 mm.",
                             "Lüftungslöcher sind Kreise von 12 mm (0,24 mm), Halter Rundstäbe von 10 mm, die Schieberplatte 4,5 mm; Roststäbe (6 mm), Wannenhalter (8 mm) und das Drahtdreieck (8 mm) sind Detaillinien von 0,18 mm."),
        "hatching": text("Wood grain as parallel lines at 0.5 mm spacing on the three handles.", "Holzmaserung als parallele Linien im Abstand 0,5 mm auf den drei Griffen."),
        "styles": text("Enamel 0.3, wood 0.55, aluminium 0.65 to 0.7, wheels and feet 0.15, steel 0.25; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.",
                       "Email 0,3, Holz 0,55, Aluminium 0,65 bis 0,7, Räder und Füße 0,15, Stahl 0,25; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm."),
        "overlays": text("Lines carry the region of the part they mark; the thermal view paints the hot lid and bowl shells.", "Linien tragen die Region des Teils, das sie kennzeichnen; die Thermalansicht malt die heißen Deckel- und Kesselschalen."),
    },
    "100": {
        "silhouette": text("A kettle grill 6.7 mm wide and 10 mm high: lid dome and bowl meeting at the rim, the vent dial as a cap, the lid handle on its brackets, the side handle, the ash pan with its hangers, the front leg with foot and socket merged, the near rear leg, the wheel and the triangle. Front: lid, bowl, both side handles with brackets, pan, three legs, both wheels on the axle, triangle; plan: bowl, lid, dial, handles, the grate as ring and 13 bars and the charcoal, wheels, axle and legs.",
                          "Ein Kugelgrill, 6,7 mm breit und 10 mm hoch: Deckelkuppel und Kessel, die sich am Rand treffen, die Lüftungsscheibe als Kappe, der Deckelgriff auf seinen Haltern, der Seitengriff, die Aschewanne mit Haltern, das vordere Bein mit eingeschlossenem Fuß und Halter, das nahe Hinterbein, das Rad und das Dreieck. Vorderansicht: Deckel, Kessel, beide Seitengriffe mit Haltern, Wanne, drei Beine, beide Räder auf der Achse, Dreieck; Draufsicht: Kessel, Deckel, Scheibe, Griffe, der Rost als Ring und 13 Stäbe und die Kohle, Räder, Achse und Beine."),
        "retained": text("Lid, bowl, handles, pan, legs, wheels, axle and triangle at real size; brackets at the 15 mm feature.", "Deckel, Kessel, Griffe, Wanne, Beine, Räder, Achse und Dreieck in realer Größe; Halter in der Einzelheit von 15 mm."),
        "merged": text("Rim beads into the shells, feet and sockets into the legs, the dial fin into the dial, the grate bars to every second bar.", "Randwülste in die Schalen, Füße und Halter in die Beine, die Scheibenrippe in die Scheibe, die Roststäbe auf jeden zweiten Stab."),
        "omitted": text("Vent holes, hub caps, the bottom damper and its lever (below 15 mm), highlights.", "Lüftungslöcher, Nabenkappen, der Bodenschieber mit Hebel (unter 15 mm), Lichtlinien."),
        "thinElements": text("Hangers, triangle and grate bars as 0.13 mm detail lines; the axle as a 10 mm bar.", "Wannenhalter, Dreieck und Roststäbe als Detaillinien von 0,13 mm; die Achse als Stab von 10 mm."),
        "hatching": text("No hatching at this scale.", "Keine Schraffur in diesem Maßstab."),
        "styles": text("Enamel 0.3, wood 0.55, aluminium 0.65 to 0.7, wheels 0.15, steel 0.25; body pen 0.25 mm, detail 0.13 mm.", "Email 0,3, Holz 0,55, Aluminium 0,65 bis 0,7, Räder 0,15, Stahl 0,25; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": text("Lines carry the region of the part they mark.", "Linien tragen die Region des Teils, das sie kennzeichnen."),
    },
    "200": {
        "silhouette": text("A kettle grill 3.4 mm wide and 5 mm high: lid dome with the dial cap and a block handle, the bowl with the side handle and the ash pan against it, two leg strokes, the wheel and the triangle line. Front: lid, bowl, both side handles, pan, three leg strokes, both wheels, axle and triangle lines; plan: bowl, lid, dial, handles, grate ring with seven bars, charcoal, wheels, axle and legs.",
                          "Ein Kugelgrill, 3,4 mm breit und 5 mm hoch: Deckelkuppel mit Scheibenkappe und Blockgriff, der Kessel mit Seitengriff und anliegender Aschewanne, zwei Beinstriche, das Rad und die Dreieckslinie. Vorderansicht: Deckel, Kessel, beide Seitengriffe, Wanne, drei Beinstriche, beide Räder, Achs- und Dreieckslinie; Draufsicht: Kessel, Deckel, Scheibe, Griffe, Rostring mit sieben Stäben, Kohle, Räder, Achse und Beine."),
        "retained": text("Lid, bowl, wheels at real size; legs as body strokes at their 25 mm.", "Deckel, Kessel, Räder in realer Größe; Beine als Körperstriche in ihren 25 mm."),
        "merged": text("Handles with their brackets into 30 mm blocks against the shells, the ash pan against the bowl (its 65 mm gap would print below the 0.15 mm gap), the grate to seven bars.", "Griffe mit Haltern zu Blöcken von 30 mm an den Schalen, die Aschewanne an den Kessel (ihr Abstand von 65 mm würde unter der Lücke von 0,15 mm gedruckt), der Rost auf sieben Stäbe."),
        "omitted": text("Beads, vent holes and fin, damper, hangers, feet, sockets, hub caps.", "Wülste, Lüftungslöcher und Rippe, Schieber, Wannenhalter, Füße, Beinhalter, Nabenkappen."),
        "thinElements": text("Legs as 0.18 mm body strokes, triangle, axle and grate bars as 0.13 mm detail lines.", "Beine als Körperstriche von 0,18 mm, Dreieck, Achse und Roststäbe als Detaillinien von 0,13 mm."),
        "hatching": text("No hatching at this scale.", "Keine Schraffur in diesem Maßstab."),
        "styles": text("Enamel 0.3, wood 0.55, aluminium 0.65 to 0.7, wheels 0.15; body pen 0.18 mm, detail 0.13 mm.", "Email 0,3, Holz 0,55, Aluminium 0,65 bis 0,7, Räder 0,15; Körperstift 0,18 mm, Detail 0,13 mm."),
        "overlays": text("Lines carry the region of the part they mark.", "Linien tragen die Region des Teils, das sie kennzeichnen."),
    },
    "500": {
        "silhouette": text("A kettle sign 1.3 mm wide and 2 mm high: the dark lid dome with its handle block over the bowl, the pan mass under it, two leg strokes and the wheel. Front: lid, bowl, pan, three leg strokes and both wheels; plan: bowl, lid, handle, wheels, legs, charcoal and the grate ring.",
                          "Ein Grillzeichen, 1,3 mm breit und 2 mm hoch: die dunkle Deckelkuppel mit Griffblock über dem Kessel, die Wannenmasse darunter, zwei Beinstriche und das Rad. Vorderansicht: Deckel, Kessel, Wanne, drei Beinstriche und beide Räder; Draufsicht: Kessel, Deckel, Griff, Räder, Beine, Kohle und der Rostring."),
        "retained": text("Lid, bowl and wheel diameter at real size; legs as body strokes.", "Deckel, Kessel und Raddurchmesser in realer Größe; Beine als Körperstriche."),
        "merged": text("The lid handle into a 75 mm block, the pan and its gap into one 75 mm mass, the wheel width to 75 mm.", "Der Deckelgriff zu einem Block von 75 mm, Wanne und Abstand zu einer Masse von 75 mm, die Radbreite auf 75 mm."),
        "omitted": text("Side handles, vent dial, damper, triangle, axle and every bead and hole (below 75 mm).", "Seitengriffe, Lüftungsscheibe, Schieber, Dreieck, Achse und alle Wülste und Löcher (unter 75 mm)."),
        "thinElements": text("Legs and the grate ring as 0.13 mm body strokes.", "Beine und Rostring als Körperstriche von 0,13 mm."),
        "hatching": text("No hatching at this scale.", "Keine Schraffur in diesem Maßstab."),
        "styles": text("Enamel 0.3, wood 0.55, aluminium 0.65, wheels 0.15; body pen 0.13 mm.", "Email 0,3, Holz 0,55, Aluminium 0,65, Räder 0,15; Körperstift 0,13 mm."),
        "overlays": text("Lines carry the region of the part they mark.", "Linien tragen die Region des Teils, das sie kennzeichnen."),
    },
}


def apply(asset):
    for scale in ("50", "100", "200", "500"):
        rep = asset["representations"][scale]
        d = int(scale)
        rep["geometry"] = {"elevation": elevation(d), "front": front(d), "plan": plan(d)}
        rep["decisions"].update(TEXTS[scale])
    return asset
