"""🎠 Round 3 playground swing: the real construction stated by hand in world metres at rest (x ahead = swing direction, y up, z left = along the beam), the bone frames, and helpers that turn parts into solids and 2D shapes.

Reference: public single-bay steel A-frame swing (KOMPAN KSW921 class, 2.5 m frame), flat rubber seat 430 x 175 mm with aluminium core and ring screws (Huck 4712 class), DIN 766 8 x 24 chains, EN 10219-2 tubes.
"""
import math

H_BEAM = 2.45
R_BEAM = 0.0508
BEAM_HALF = 0.95
BEAM_TUBE_HALF = 0.945
APEX_X = 1.0
FEET_X = (0.0, 2.0)
ZA = 0.875
R_LEG = 0.03015
R_LEG_IN = 0.02725
LEG_ANGLE = math.degrees(math.atan2(H_BEAM, APEX_X))
LEG_LEN = math.hypot(APEX_X, H_BEAM)
R_SLEEVE = 0.05715
SLEEVE_HALF = 0.06
R_SOCKET = 0.03805
SOCKET_S = (0.04, 0.22)
LEG_TOP_S = 0.06
R_BRACE = 0.0212
R_BRACE_IN = 0.0186
BRACE_Y = 0.6
BRACE_X0 = BRACE_Y / math.tan(math.radians(LEG_ANGLE))
BRACE_LEN = FEET_X[1] - 2 * BRACE_X0
PLATE = (0.18, 0.012, 0.10)
NUT_AF = 0.019
PIVOT_DROP = 0.085
CHAIN_Z = 0.20
CHAIN_LEN = 1.9
HITCH_Y = H_BEAM - PIVOT_DROP - CHAIN_LEN
WIRE_R = 0.004
LINK_STRAIGHT = 0.0128
LINK_RC = 0.0096
PITCH = 0.024
LINK0_U = 0.0775
LINKS = 76
SEAT = (0.175, 0.035, 0.43)
SEAT_TOP_V = -0.03
SEAT_EYE_RC = 0.0155
SEAT_EYE_R = 0.005
SEAT_EYE_Z = 0.20

BONES = {
    "ground": {"origin": (0.0, 0.0), "angle": 0.0},
    "leg-left": {"origin": (FEET_X[0], 0.0), "angle": LEG_ANGLE},
    "leg-right": {"origin": (FEET_X[1], 0.0), "angle": 180.0 - LEG_ANGLE},
    "brace": {"origin": (BRACE_X0, BRACE_Y), "angle": 0.0},
    "beam": {"origin": (APEX_X, H_BEAM), "angle": 0.0},
    "chain-left": {"origin": (APEX_X, H_BEAM - PIVOT_DROP), "angle": -90.0},
    "chain-right": {"origin": (APEX_X, H_BEAM - PIVOT_DROP), "angle": -90.0},
    "seat": {"origin": (APEX_X, HITCH_Y), "angle": 0.0},
}
SIDES = (("l", 1), ("r", -1))
LEGS = (("left", "leg-left", FEET_X[0]), ("right", "leg-right", FEET_X[1]))
CHAINS = (("left", "chain-left", CHAIN_Z), ("right", "chain-right", -CHAIN_Z))


def r6(p):
    return [round(c + 0.0, 6) for c in p]


def rd(v):
    return round(v + 0.0, 6)


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


def ldir(bone, d):
    """🧭 A world direction (x, y, z) in the bone-local frame."""
    a = math.radians(-BONES[bone]["angle"])
    return [d[0] * math.cos(a) - d[1] * math.sin(a), d[0] * math.sin(a) + d[1] * math.cos(a), d[2]]


def leg_dir(name):
    a = math.radians(BONES["leg-" + name]["angle"])
    return (math.cos(a), math.sin(a))


def leg_centre_x(name, y):
    """📏 World x of the centre line of a leg at height y."""
    a = math.radians(BONES["leg-" + name]["angle"])
    return BONES["leg-" + name]["origin"][0] + y * math.cos(a) / math.sin(a)


def leg_half_x():
    """↔️ Half width of a leg tube cut by a horizontal plane."""
    return R_LEG / math.sin(math.radians(LEG_ANGLE))


def apex_point(name, s):
    """🔺 World point at distance s below the apex (beam axis) along the leg `name`."""
    dx, dy = leg_dir(name)
    return (APEX_X - dx * s, H_BEAM - dy * s)


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


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


def revolve(sid, bone, material, role, tone, profile, axis, origin, **kw):
    return solid(sid, "revolve", bone, material, role, tone, profile=[r6(p) for p in profile], axis=r6(axis), origin=r6(origin), **kw)


def ellipsoid(sid, bone, material, role, tone, radii, centre, **kw):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=r6(radii), transform={"translation": r6(centre)}, **kw)


def box(sid, bone, material, role, tone, size, centre, rotation=None, radius=None, **kw):
    t = {"translation": r6(centre)}
    if rotation:
        t["rotation"] = rotation
    s = solid(sid, "box", bone, material, role, tone, size=r6(size), transform=t, **kw)
    if radius:
        s["cornerRadius"] = radius
    return s


def prism(sid, bone, material, role, tone, profile, origin, ex, ey, t0, t1, holes=None, **kw):
    """🧱 A polygon (a, b) in the plane spanned by the bone-local directions ex, ey through `origin`, pushed from t0 to t1 along ex x ey."""
    ex = unit(ex)
    ey = unit(ey)
    ez = cross(ex, ey)
    s = solid(sid, "extrude", bone, material, role, tone, profile=[r6(p) for p in profile], plane="xy", **{"from": rd(t0), "to": rd(t1)}, transform={"translation": r6(origin), "rotation": quat_from_axes(ex, ey, ez)}, **kw)
    if holes:
        s["holes"] = [[r6(p) for p in h] for h in holes]
    return s


def loft(sid, bone, material, role, tone, sections, caps=True, **kw):
    return solid(sid, "loft", bone, material, role, tone, sections=sections, capStart=caps, capEnd=caps, **kw)


def section(at, rx, ry, exponent=None, rotation=None):
    s = {"at": r6(at), "rx": rd(rx), "ry": rd(ry)}
    if exponent:
        s["exponent"] = exponent
    if rotation:
        s["rotation"] = rotation
    return s


def leg_cut_rotation(bone):
    """✂️ Rotation of a loft section into the horizontal world plane on a leg bone (x = horizontal in the leg plane, y = lateral)."""
    ex = ldir(bone, (1, 0, 0))
    ey = [0, 0, 1]
    return quat_from_axes(ex, ey, cross(ex, ey))


def leg_tube(sid, bone, w, r, s_top, material, role, tone, y_foot, **kw):
    """🦵 A leg tube on its true centre line, its foot cut by the horizontal plane y = y_foot (the mitre on the base plate), its head at s_top below the apex."""
    a = math.radians(LEG_ANGLE)
    rot = leg_cut_rotation(bone)
    u0 = y_foot / math.sin(a)
    u1 = LEG_LEN - s_top
    rx = r / math.sin(a)
    return loft(sid, bone, material, role, tone, [section([u0, 0, w], rx, r, rotation=rot), section([u1, 0, w], rx, r, rotation=rot)], **kw)


def hexagon(af, cx=0.0, cy=0.0, turn=0.0):
    rc = af / math.sqrt(3)
    return [[cx + rc * math.cos(math.radians(30 + 60 * i + turn)), cy + rc * math.sin(math.radians(30 + 60 * i + turn))] for i in range(6)]


def circle(cx, cy, r, n=24):
    return [[cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)] for i in range(n)]


def torus_profile(rc, r, n=12):
    """⭕ Profile [axial, radial] of a ring of wire radius r on the centre radius rc."""
    return [[r * math.cos(2 * math.pi * i / n), rc + r * math.sin(2 * math.pi * i / n)] for i in range(n)]


def link_path(centre_u, plane_axis, w):
    """⛓️ Centre line of one DIN 766 link: two straights of LINK_STRAIGHT and two half circles of LINK_RC, in the plane of u and `plane_axis` (1 = v, 2 = w), starting mid side."""
    pts = [(0.0, LINK_RC), (LINK_STRAIGHT / 2, LINK_RC)]
    pts += [(LINK_STRAIGHT / 2 + LINK_RC * math.cos(math.radians(t)), LINK_RC * math.sin(math.radians(t))) for t in (60, 30, 0, -30, -60)]
    pts += [(LINK_STRAIGHT / 2, -LINK_RC), (-LINK_STRAIGHT / 2, -LINK_RC)]
    pts += [(-LINK_STRAIGHT / 2 + LINK_RC * math.cos(math.radians(t)), LINK_RC * math.sin(math.radians(t))) for t in (-120, -150, 180, 150, 120)]
    pts += [(-LINK_STRAIGHT / 2, LINK_RC), (0.0, LINK_RC)]
    out = []
    for a, b in pts:
        p = [centre_u + a, 0.0, w]
        p[plane_axis] += b
        out.append(p)
    return out


def link_half(centre_u, plane_axis, w, end):
    """⛓️ Centre line of one half of a DIN 766 link (end -1 = the half towards the pivot, +1 = towards the seat): from the middle of one straight round the half circle to the middle of the other straight, so two halves meet flush mid side."""
    s = LINK_STRAIGHT / 2
    pts = [(0.0, LINK_RC), (end * s, LINK_RC)]
    pts += [(end * (s + LINK_RC * math.cos(math.radians(t))), LINK_RC * math.sin(math.radians(t))) for t in (60, 30, 0, -30, -60)]
    pts += [(end * s, -LINK_RC), (0.0, -LINK_RC)]
    out = []
    for a, b in pts:
        p = [centre_u + a, 0.0, w]
        p[plane_axis] += b
        out.append(p)
    return out


def link_u(k):
    return LINK0_U + k * PITCH


def link_outline(cu, cv, half_len, half_w):
    """🔗 Elevation outline (u, v) of a stadium link of half length half_len and half width half_w, as SVG path rings."""
    rr = half_w
    s = half_len - rr
    return "M %.5f %.5f L %.5f %.5f A %.5f %.5f 0 0 1 %.5f %.5f L %.5f %.5f A %.5f %.5f 0 0 1 %.5f %.5f Z" % (cu - s, cv - rr, cu + s, cv - rr, rr, rr, cu + s, cv + rr, cu - s, cv + rr, rr, rr, cu - s, cv - rr)


def shape(sid, bone, role, z, filled, tone, kind, **kw):
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(kw)
    out["kind"] = kind
    return out


def poly2(sid, bone, role, z, filled, tone, pts, closed=True, **kw):
    out = shape(sid, bone, role, z, filled, tone, "poly", **kw)
    out["points"] = [[round(p[0] + 0.0, 5), round(p[1] + 0.0, 5)] for p in pts]
    out["closed"] = closed
    return out


def rect2(sid, bone, role, z, filled, tone, x, y, w, h, r=None, **kw):
    out = shape(sid, bone, role, z, filled, tone, "rect", **kw)
    out.update({"x": round(x, 5), "y": round(y, 5), "w": round(w, 5), "h": round(h, 5)})
    if r:
        out["r"] = r
    return out


def ellipse2(sid, bone, role, z, filled, tone, cx, cy, rx, ry, **kw):
    out = shape(sid, bone, role, z, filled, tone, "ellipse", **kw)
    out.update({"cx": round(cx, 5), "cy": round(cy, 5), "rx": round(rx, 5), "ry": round(ry, 5)})
    return out


def capsule2(sid, bone, role, z, filled, tone, x0, y0, x1, y1, r0, r1=None, **kw):
    out = shape(sid, bone, role, z, filled, tone, "capsule", **kw)
    out.update({"x0": round(x0, 5), "y0": round(y0, 5), "x1": round(x1, 5), "y1": round(y1, 5), "r0": r0, "r1": r1 if r1 is not None else r0})
    return out


def path2(sid, bone, role, z, filled, tone, d, **kw):
    out = shape(sid, bone, role, z, filled, tone, "path", **kw)
    out["d"] = d
    return out


def ring_d(cx, cy, ro, ri, n=24):
    """⭕ SVG path of a ring (outer and inner circle as polygons, even-odd)."""
    def ring(r):
        pts = circle(cx, cy, r, n)
        return "M " + " L ".join("%.5f %.5f" % (p[0], p[1]) for p in pts) + " Z"
    return ring(ro) + " " + ring(ri)


def poly_d(pts):
    return "M " + " L ".join("%.5f %.5f" % (p[0], p[1]) for p in pts) + " Z"


def leg_outline_world(name, y_foot, s_top):
    """🦵 World (x, y) outline of a leg tube from its horizontal foot cut to its head s_top below the apex (perpendicular end)."""
    dx, dy = leg_dir(name)
    nx, ny = -dy, dx
    hx = leg_half_x()
    xf = leg_centre_x(name, y_foot)
    top = apex_point(name, s_top)
    a, b = (top[0] + nx * R_LEG, top[1] + ny * R_LEG), (top[0] - nx * R_LEG, top[1] - ny * R_LEG)
    return [(xf - hx, y_foot), (xf + hx, y_foot)] + sorted([a, b], key=lambda p: -p[0])


def ribs_world(name):
    """📐 The two triangular foot gussets (world x, y) of a leg: outer and inner, pushed 3 mm into the tube."""
    hx = leg_half_x()
    sgn = 1 if name == "left" else -1
    xf = BONES["leg-" + name]["origin"][0]
    y0 = PLATE[1]
    def surf(y, side):
        return leg_centre_x(name, y) + side * sgn * (hx - 0.003)
    outer = [(xf - sgn * 0.085, y0), (surf(y0, -1), y0), (surf(0.09, -1), 0.09)]
    inner = [(xf + sgn * 0.085, y0), (surf(0.075, 1), 0.075), (surf(y0, 1), y0)]
    return outer, inner


def nut_spots(name):
    xf = BONES["leg-" + name]["origin"][0]
    return [(xf - 0.06, 0.03), (xf + 0.06, -0.03)]


def gusset_world():
    """🔺 The apex gusset between the two sockets (beam-local x, y)."""
    out = []
    for name, sgn in (("left", -1), ("right", 1)):
        dx, dy = leg_dir(name)
        ax, ay = -dx, -dy
        nx, ny = -sgn * ay, sgn * ax
        if nx * sgn > 0:
            nx, ny = -nx, -ny
        s = SOCKET_S[1] - 0.02
        out.append((ax * s + nx * 0.026, ay * s + ny * 0.026))
    return [out[0], out[1], (0.0, -0.045)]


def tube_mass(do, t, length):
    return 7850 * math.pi / 4 * (do ** 2 - (do - 2 * t) ** 2) * length


def masses():
    """⚖️ Bone masses and centres (bone-local) from EN 10219-2 tube masses, plate and link volumes."""
    a = math.radians(LEG_ANGLE)
    u0, u1 = PLATE[1] / math.sin(a), LEG_LEN - LEG_TOP_S
    leg = tube_mass(0.0603, 0.0029, u1 - u0)
    plate = 7850 * PLATE[0] * PLATE[1] * PLATE[2]
    ribs = 7850 * 0.008 * (0.5 * 0.06 * 0.078 + 0.5 * 0.05 * 0.063)
    hardware = 2 * 0.06
    leg_bone = 2 * (leg + plate + ribs + hardware)
    leg_com = 2 * leg * (u0 + u1) / 2 / leg_bone
    brace_tube = tube_mass(0.0424, 0.0026, BRACE_LEN - 2 * R_LEG)
    clamp = 7850 * (math.pi / 4 * (0.076 ** 2 - 0.0603 ** 2) * 0.07 + math.pi / 4 * (0.052 ** 2 - 0.0424 ** 2) * 0.045)
    brace = 2 * brace_tube + 4 * clamp
    beam_tube = tube_mass(0.1016, 0.0036, 2 * BEAM_TUBE_HALF) + 2 * 7850 * math.pi * R_BEAM ** 2 * 0.003
    sleeve = tube_mass(0.1143, 0.004, 2 * SLEEVE_HALF)
    socket = tube_mass(0.0761, 0.0032, SOCKET_S[1] - SOCKET_S[0])
    gusset = 7850 * 0.01 * 0.5 * 0.103 * 0.15
    hanger = 7850 * (math.pi * (0.0585 ** 2 - R_BEAM ** 2) * 0.05 + 2 * 0.008 * 0.05 * 0.065 + math.pi * 0.008 ** 2 * 0.06)
    beam = beam_tube + 2 * (sleeve + 2 * socket + gusset) + 2 * hanger + 4 * 0.06
    beam_com_v = (2 * (2 * socket * -0.13 + gusset * -0.11) + 2 * hanger * -0.07) / beam
    link = 7850 * math.pi * WIRE_R ** 2 * (2 * LINK_STRAIGHT + 2 * math.pi * LINK_RC)
    hook = 0.15
    chain = LINKS * link + hook
    chain_com = (LINKS * link * (link_u(0) + link_u(LINKS - 1)) / 2 + hook * 0.04) / chain
    core = 2700 * 0.40 * 0.008 * 0.16
    rubber = 1100 * (SEAT[0] * SEAT[1] * SEAT[2] * 0.93 - 0.40 * 0.008 * 0.16)
    eyes = 2 * 0.11
    seat = core + rubber + eyes
    seat_com_v = ((core + rubber) * (SEAT_TOP_V - SEAT[1] / 2) + eyes * -0.02) / seat
    out = {
        "leg-left": (leg_bone, leg_com, 0.0),
        "leg-right": (leg_bone, leg_com, 0.0),
        "brace": (brace, BRACE_LEN / 2, 0.0),
        "beam": (beam, 0.0, beam_com_v),
        "chain-left": (chain, chain_com, 0.0),
        "chain-right": (chain, chain_com, 0.0),
        "seat": (seat, 0.0, seat_com_v),
    }
    return {k: (round(v[0], 2), round(v[1], 3), round(v[2], 3)) for k, v in out.items()}


def sections():
    """📐 EN 10219-2 section properties of the members (area, second moment, extreme fibre)."""
    def tube(do, t):
        di = do - 2 * t
        return {"areaM2": round(math.pi / 4 * (do ** 2 - di ** 2), 7), "secondMomentM4": float("%.4g" % (math.pi / 64 * (do ** 4 - di ** 4))), "extremeFibreM": round(do / 2, 5)}
    return {
        "leg": tube(0.0603, 0.0029),
        "brace": tube(0.0424, 0.0026),
        "beam": tube(0.1016, 0.0036),
        "chain": {"areaM2": round(2 * math.pi * WIRE_R ** 2, 7), "secondMomentM4": float("%.4g" % (2 * math.pi * (2 * WIRE_R) ** 4 / 64)), "extremeFibreM": WIRE_R},
        "seat": {"areaM2": round(0.16 * 0.008, 6), "secondMomentM4": float("%.4g" % (0.16 * 0.008 ** 3 / 12)), "extremeFibreM": 0.004},
    }


if __name__ == "__main__":
    print("leg angle", LEG_ANGLE, "length", LEG_LEN, "brace", BRACE_X0, BRACE_LEN, "hitch y", HITCH_Y)
    m = masses()
    print(m, "total", round(sum(v[0] for v in m.values()), 2))
    print(sections())
    print("last link u", link_u(LINKS - 1), "bottom wire", link_u(LINKS - 1) + LINK_STRAIGHT / 2 + LINK_RC, "eye top wire", CHAIN_LEN - SEAT_EYE_RC, "gap", CHAIN_LEN - SEAT_EYE_RC - (link_u(LINKS - 1) + LINK_STRAIGHT / 2 + LINK_RC) - (WIRE_R + SEAT_EYE_R))
    print("gusset", gusset_world())
