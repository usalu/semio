"""🛒 Round 3 shopping trolley: the real construction stated by hand in world metres at rest (x ahead, y up, z left), the bone frames, and small helpers that turn parts into solids and 2D shapes.

Reference: Wanzl EL/ELX 101 class (catalogue 1193 EN, 2025): 100 l wire basket, o125 castors, L 1.0-1.04 m, W 0.59-0.60 m, H 0.98 m.
"""
import math

R = 4


def rd(v):
    return round(v + 0.0, R + 2) if isinstance(v, float) else v


def r6(p):
    return [round(c + 0.0, 6) for c in p]


BONES = {
    "chassis": {"origin": (-0.295, 0.1675), "angle": 0.0},
    "basket": {"origin": (-0.36, 0.54), "angle": 0.0},
    "handle": {"origin": (-0.5075, 0.9825), "angle": 0.0},
    "gate": {"origin": (-0.395, 0.875), "angle": -84.0},
}
GATE_LEN = 0.328

TUBE_R = 0.0125
STRUT_R = 0.01
RAIL_Y = 0.1675
RAIL_Z_REAR = 0.2725
RAIL_Z_FRONT = 0.195
RAIL_X_REAR = -0.295
RAIL_X_FRONT = 0.33
BEND = [(-0.322, 0.18), (-0.336, 0.21)]
UPRIGHT_TOP = (-0.455, 0.935)
ARM = [(-0.468, 0.962), (-0.4875, 0.978)]
GRIP_C = (-0.5075, 0.9825)
GRIP_R = 0.0175
SIGN_CORNER = (-0.33, 0.1675)
SIGN_TOP = (-0.52, 0.9825)
GRIP_HALF = 0.24
CAP_R = 0.0185

WHEEL_R = 0.0625
TYRE_IN = 0.0475
WHEEL_W = 0.032
TRAIL = 0.035
PLATE_Y = (0.147, 0.155)
HEAD_Y = (0.134, 0.147)
CASTORS = {
    "rear": {"xs": -0.26, "zs": 0.25},
    "front": {"xs": 0.30, "zs": 0.198},
}

FLOOR_REAR = (0.0, 0.0, 0.21)
FLOOR_FRONT = (0.62, 0.025, 0.186)
RIM_REAR = (-0.035, 0.335, 0.255)
RIM_FRONT = (0.88, 0.335, 0.215)
RIM_WR = 0.004
FRAME_WR = 0.003
MESH_WR = 0.00225
GATE_FRAME_WR = 0.003
GATE_WIRE_WR = 0.002

TRAY_X = (-0.23, 0.25)
TRAY_Y = 0.183
TRAY_LIP = 0.04


def rail_z(x):
    return RAIL_Z_REAR - (x - RAIL_X_REAR) / (RAIL_X_FRONT - RAIL_X_REAR) * (RAIL_Z_REAR - RAIL_Z_FRONT)


def upright_x(y):
    (x0, y0), (x1, y1) = BEND[-1], UPRIGHT_TOP
    return x0 + (y - y0) / (y1 - y0) * (x1 - x0)


def local(bone, p):
    """📍 World rest point (x, y[, z]) to the bone-local frame (u, v[, w])."""
    b = BONES[bone]
    dx, dy = p[0] - b["origin"][0], p[1] - b["origin"][1]
    a = math.radians(-b["angle"])
    u, v = dx * math.cos(a) - dy * math.sin(a), dx * math.sin(a) + dy * math.cos(a)
    return r6([u, v] + ([p[2]] if len(p) > 2 else []))


def world(bone, p):
    """🌍 Bone-local (u, v[, w]) to world rest coordinates."""
    b = BONES[bone]
    a = math.radians(b["angle"])
    x = b["origin"][0] + p[0] * math.cos(a) - p[1] * math.sin(a)
    y = b["origin"][1] + p[0] * math.sin(a) + p[1] * math.cos(a)
    return r6([x, y] + ([p[2]] if len(p) > 2 else []))


def basket_world(p):
    return world("basket", p)


def lerp(a, b, t):
    return [a[i] + (b[i] - a[i]) * t for i in range(len(a))]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def mul(a, s):
    return [c * s for c in a]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [c / n for c in a]


def quat_from_axes(ex, ey, ez):
    m00, m01, m02 = ex[0], ey[0], ez[0]
    m10, m11, m12 = ex[1], ey[1], ez[1]
    m20, m21, m22 = ex[2], ey[2], ez[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w, x, y, z = 0.25 * s, (m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        w, x, y, z = (m21 - m12) / s, 0.25 * s, (m01 + m10) / s, (m02 + m20) / s
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        w, x, y, z = (m02 - m20) / s, (m01 + m10) / s, 0.25 * s, (m12 + m21) / s
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        w, x, y, z = (m10 - m01) / s, (m02 + m20) / s, (m12 + m21) / s, 0.25 * s
    return [round(x, 6), round(y, 6), round(z, 6), round(w, 6)]


def solid(sid, kind, bone, material, role, tone, **kw):
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def capsule(sid, bone, material, role, tone, p0, p1, r0, r1=None, slender=False, **kw):
    s = solid(sid, "capsule", bone, material, role, tone, p0=r6(p0), p1=r6(p1), r0=rd(r0), r1=rd(r1 if r1 is not None else r0), **kw)
    if slender:
        s["slender"] = True
    return s


def polyline(prefix, bone, material, role, tone, pts, r, slender=False, **kw):
    return [capsule("%s-%d" % (prefix, i + 1), bone, material, role, tone, pts[i], pts[i + 1], r, slender=slender, **kw) for i in range(len(pts) - 1)]


def sheet(sid, bone, material, role, tone, corners, thickness, **kw):
    """🪟 A planar polygon of bone-local 3D corners as a thin extrude placed by a rotation and translation."""
    o = corners[0]
    ex = unit(sub(corners[1], o))
    n = unit(cross(ex, sub(corners[2], o)))
    ey = cross(n, ex)
    prof = [[round(dot(sub(c, o), ex), 6), round(dot(sub(c, o), ey), 6)] for c in corners]
    for c in corners:
        assert abs(dot(sub(c, o), n)) < 6e-4, (sid, dot(sub(c, o), n))
    return solid(sid, "extrude", bone, material, role, tone, profile=prof, plane="xy", **{"from": -thickness / 2, "to": thickness / 2}, transform={"translation": r6(o), "rotation": quat_from_axes(ex, ey, n)}, **kw)


def revolve(sid, bone, material, role, tone, profile, axis, origin, **kw):
    return solid(sid, "revolve", bone, material, role, tone, profile=[r6(p) for p in profile], axis=axis, origin=r6(origin), **kw)


def box(sid, bone, material, role, tone, size, centre, radius=None, **kw):
    s = solid(sid, "box", bone, material, role, tone, size=r6(size), transform={"translation": r6(centre)}, **kw)
    if radius:
        s["cornerRadius"] = radius
    return s


def ellipsoid(sid, bone, material, role, tone, radii, centre, **kw):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=r6(radii), transform={"translation": r6(centre)}, **kw)


def extrude(sid, bone, material, role, tone, profile, plane, z0, z1, translation=None, **kw):
    s = solid(sid, "extrude", bone, material, role, tone, profile=[r6(p) for p in profile], plane=plane, **{"from": rd(z0), "to": rd(z1)}, **kw)
    if translation:
        s["transform"] = {"translation": r6(translation)}
    return s


def wall_plane(a, b, c):
    n = unit(cross(sub(b, a), sub(c, a)))
    return n, dot(n, a)


def on_plane_at(n, d, fixed_axis, fixed_value, other_axis, other_value):
    """📐 The point of the plane n.p = d where coordinate fixed_axis = fixed_value and other_axis = other_value (solves the third)."""
    p = [0.0, 0.0, 0.0]
    p[fixed_axis] = fixed_value
    p[other_axis] = other_value
    k = 3 - fixed_axis - other_axis
    p[k] = (d - n[fixed_axis] * fixed_value - n[other_axis] * other_value) / n[k]
    return p


def side_corners(s):
    fr = [FLOOR_REAR[0], FLOOR_REAR[1], s * FLOOR_REAR[2]]
    ff = [FLOOR_FRONT[0], FLOOR_FRONT[1], s * FLOOR_FRONT[2]]
    rf = [RIM_FRONT[0], RIM_FRONT[1], s * RIM_FRONT[2]]
    rr = [RIM_REAR[0], RIM_REAR[1], s * RIM_REAR[2]]
    return fr, ff, rf, rr


def seg_at_x(a, b, x):
    if (a[0] - x) * (b[0] - x) > 0 or a[0] == b[0]:
        return None
    return lerp(a, b, (x - a[0]) / (b[0] - a[0]))


def seg_at_y(a, b, y):
    if (a[1] - y) * (b[1] - y) > 0 or a[1] == b[1]:
        return None
    return lerp(a, b, (y - a[1]) / (b[1] - a[1]))


def seg_at_z(a, b, z):
    if (a[2] - z) * (b[2] - z) > 0 or a[2] == b[2]:
        return None
    return lerp(a, b, (z - a[2]) / (b[2] - a[2]))


def side_vertical(s, x):
    """🧷 A side-wall picket at basket-local u = x: from the floor edge or the front corner wire up to the rim, in the wall plane."""
    fr, ff, rf, rr = side_corners(s)
    lo = seg_at_x(fr, ff, x) or seg_at_x(ff, rf, x)
    hi = seg_at_x(rr, rf, x)
    if x < fr[0]:
        lo = seg_at_x(fr, rr, x)
    return lo, hi


def picket_xs(step, shortest=0.095):
    """🧷 The u positions of the side pickets at a spacing of step x 30 mm, without the stubs near the front corner that are shorter than `shortest` (a slender wire is at least 20 times as long as thick)."""
    out = []
    for i in range(step - 1, 28, step):
        x = 0.03 * (i + 1)
        lo, hi = side_vertical(1, x)
        if math.dist(lo, hi) >= shortest:
            out.append(x)
    return out


def bundles(prefix, make, ys, front_ys, floor_n, r_side, r_front, r_floor):
    """🌡️ Thermal-only wire bundles in the mesh planes: horizontals along both side walls at heights ys, across the front wall at front_ys, and floor_n longitudinal floor bundles; long enough to stay slender."""
    out = []
    for n, s in (("l", 1), ("r", -1)):
        for j, y in enumerate(ys):
            a, b = side_horizontal(s, y)
            out.append(make("%s-side-%s-%d" % (prefix, n, j + 1), a, b, r_side))
    for j, y in enumerate(front_ys):
        a, b = front_horizontal(y)
        out.append(make("%s-front-%d" % (prefix, j + 1), a, b, r_front))
    for i in range(floor_n):
        a, b = floor_long(i, floor_n, 0.05)
        out.append(make("%s-floor-%d" % (prefix, i + 1), a, b, r_floor))
    return out


def side_horizontal(s, y):
    fr, ff, rf, rr = side_corners(s)
    return seg_at_y(fr, rr, y), seg_at_y(ff, rf, y)


def front_vertical(z):
    ffl, ffr = [FLOOR_FRONT[0], FLOOR_FRONT[1], FLOOR_FRONT[2]], [FLOOR_FRONT[0], FLOOR_FRONT[1], -FLOOR_FRONT[2]]
    rfl, rfr = [RIM_FRONT[0], RIM_FRONT[1], RIM_FRONT[2]], [RIM_FRONT[0], RIM_FRONT[1], -RIM_FRONT[2]]
    lo = seg_at_z(ffr, ffl, z)
    hi = seg_at_z(rfr, rfl, z)
    return lo, hi


def front_horizontal(y):
    _, ffl, rfl, _ = side_corners(1)
    a = seg_at_y(ffl, rfl, y)
    return [a[0], a[1], -a[2]], a


def floor_long(k, n, margin):
    zr = -FLOOR_REAR[2] + margin + (2 * (FLOOR_REAR[2] - margin)) * k / (n - 1)
    zf = zr * (FLOOR_FRONT[2] - margin * FLOOR_FRONT[2] / FLOOR_REAR[2]) / (FLOOR_REAR[2] - margin)
    return [FLOOR_REAR[0], FLOOR_REAR[1], zr], [FLOOR_FRONT[0], FLOOR_FRONT[1], zf]


def floor_y(x):
    return FLOOR_REAR[1] + (x - FLOOR_REAR[0]) / (FLOOR_FRONT[0] - FLOOR_REAR[0]) * (FLOOR_FRONT[1] - FLOOR_REAR[1])


def floor_half(x):
    return FLOOR_REAR[2] + (x - FLOOR_REAR[0]) / (FLOOR_FRONT[0] - FLOOR_REAR[0]) * (FLOOR_FRONT[2] - FLOOR_REAR[2])


def gate_half(u):
    return 0.249 - u / GATE_LEN * 0.044


def tyre_profile():
    return [[-0.016, TYRE_IN], [-0.016, 0.056], [-0.0148, 0.0598], [-0.0118, 0.0618], [-0.007, WHEEL_R], [0.007, WHEEL_R], [0.0118, 0.0618], [0.0148, 0.0598], [0.016, 0.056], [0.016, TYRE_IN]]


def hub_profile():
    return [[-0.018, 0.0], [-0.018, 0.0135], [-0.0125, 0.017], [-0.0125, 0.04], [-0.0145, 0.0435], [-0.0145, 0.0485], [0.0145, 0.0485], [0.0145, 0.0435], [0.0125, 0.04], [0.0125, 0.017], [0.018, 0.0135], [0.018, 0.0]]


def circle(cx, cy, r, n=24):
    return [[round(cx + r * math.cos(2 * math.pi * i / n), 5), round(cy + r * math.sin(2 * math.pi * i / n), 5)] for i in range(n)]


def fork_profile(xs):
    """🍴 Side profile (world x, y) of one pressed-steel castor fork leg: from the crown under the swivel head down and back to the axle boss."""
    xa, ya = xs - TRAIL, WHEEL_R
    return [[xs + 0.02, HEAD_Y[0]], [xs - 0.024, HEAD_Y[0]], [xa - 0.017, ya + 0.004], [xa - 0.012, ya - 0.012], [xa, ya - 0.016], [xa + 0.012, ya - 0.012], [xa + 0.016, ya + 0.002]]


def castor_masses(make, r):
    """⚖️ The chassis mass is a distributed tube frame whose centre lies in the air between the rails; its concentrated masses are the four castors (0.55 kg each), drawn at their centroids."""
    out = []
    for pos in ("rear", "front"):
        c = CASTORS[pos]
        for n, s in (("l", 1), ("r", -1)):
            out.append(make("mass-castor-%s-%s" % (pos, n), local("chassis", (c["xs"] - 0.017, 0.10, s * c["zs"])), r))
    return out


def shape(sid, bone, role, z, filled, tone, kind, **kw):
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(kw)
    out["kind"] = kind
    return out


def poly2(sid, bone, role, z, filled, tone, pts, closed, **kw):
    out = shape(sid, bone, role, z, filled, tone, "poly", **kw)
    out["points"] = [[round(p[0] + 0.0, 5), round(p[1] + 0.0, 5)] for p in pts]
    out["closed"] = closed
    return out


def ellipse2(sid, bone, role, z, filled, tone, cx, cy, rx, ry, **kw):
    out = shape(sid, bone, role, z, filled, tone, "ellipse", **kw)
    out.update({"cx": round(cx, 5), "cy": round(cy, 5), "rx": round(rx, 5), "ry": round(ry, 5)})
    return out


def rect2(sid, bone, role, z, filled, tone, x, y, w, h, r=None, **kw):
    out = shape(sid, bone, role, z, filled, tone, "rect", **kw)
    out.update({"x": round(x, 5), "y": round(y, 5), "w": round(w, 5), "h": round(h, 5)})
    if r:
        out["r"] = r
    return out


def capsule2(sid, bone, role, z, filled, tone, x0, y0, x1, y1, r0, r1=None, **kw):
    out = shape(sid, bone, role, z, filled, tone, "capsule", **kw)
    out.update({"x0": round(x0, 5), "y0": round(y0, 5), "x1": round(x1, 5), "y1": round(y1, 5), "r0": r0, "r1": r1 if r1 is not None else r0})
    return out
