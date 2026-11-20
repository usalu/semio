"""🚐 Hand-stated master lines and solid writers of the round 3 delivery van (vehicles/delivery-van): one set of design lines (side profile of the nose, plan half-widths, arch circles, the leaning side plane, the swept windscreen, the door apertures) that the solids of every scale and the 2D drawings of every view are written from.

Frame of the chassis bone: x ahead, y up, z to the left of the van, metres, origin on the ground under the middle of the length.
Reference: medium panel vans of 5.3 m (Ford Transit Custom L2H1 5340 x 1986 x 1972..2000, wheelbase 3300; VW Transporter T6 long 5304 x 1904 x 1990, wheelbase 3400; Mercedes Vito long 5140 x 1928 x 1910, wheelbase 3200; Renault Trafic L2 5399 x 1956 x 1971, wheelbase 3498), tyre 235/65 R16 (ETRTO: 712 mm).
"""
import math

XF, XR = 1.70, -1.60
ZW = 0.84
RT = 0.356
RA = 0.42
YA = 0.365
W = 0.985
T = 0.045
Y_SILL = 0.30
Y_FLOOR = 0.56
Y_BELT = 1.17
Y_CANT = 1.90
H = 1.99
LEAN = 0.08 / 0.73
BETA = math.atan(LEAN)
X_SKIN = -2.61
X_TAIL = -2.67
X_NOSE = 2.67
X_QUARTER = -0.80
X_B = 0.25
X_BULK = 0.33
X_HINGE = 1.22
Z_LEAF = 0.86
Y_DOOR = 0.55
SLIDER = (-0.8, 0.356)
SCREEN_R = 3.0
SCREEN_BASE = (1.74, 1.20)
SCREEN_TOP = (1.10, 1.83)
RHO = math.atan2(SCREEN_TOP[1] - SCREEN_BASE[1], SCREEN_BASE[0] - SCREEN_TOP[0])

QX = [0.70711, 0.0, 0.70711, 0.0]
QH = [0.70711, 0.0, 0.0, 0.70711]

NOSE_TOP = [(1.22, 1.17), (1.60, 1.185), (1.74, 1.20), (1.80, 1.19), (2.20, 1.105), (2.44, 1.045), (2.55, 1.0), (2.615, 0.93), (2.65, 0.84), (2.665, 0.74), (2.67, 0.62)]
NOSE_BOTTOM = [(1.22, 0.30), (2.115, 0.30), (2.20, 0.27), (2.55, 0.26), (2.63, 0.30), (2.66, 0.36), (2.67, 0.42)]
NOSE_HALF = [(1.22, 0.985), (2.10, 0.985), (2.20, 0.98), (2.32, 0.965), (2.44, 0.935), (2.54, 0.885), (2.61, 0.82), (2.65, 0.74), (2.67, 0.66)]
NOSE_EXPONENT = [(1.22, 40), (1.285, 16), (1.70, 10), (2.115, 8), (2.30, 7), (2.60, 6), (2.67, 6)]
ROOF = [(-2.64, 1.945, 0.03, 0.87), (-2.63, 1.945, 0.042, 0.895), (-2.61, 1.945, 0.045, 0.90), (-2.55, 1.945, 0.045, 0.905), (-2.49, 1.90, 0.09, 0.905), (0.35, 1.90, 0.09, 0.905), (0.60, 1.898, 0.09, 0.90), (0.85, 1.89, 0.085, 0.89), (1.0, 1.875, 0.075, 0.875), (1.08, 1.86, 0.06, 0.86), (1.12, 1.845, 0.04, 0.845)]


def r5(v):
    return round(v, 5) if isinstance(v, float) else v


def rr(vs):
    return [r5(float(v)) for v in vs]


def interp(table, x):
    if x <= table[0][0]:
        return table[0][1]
    for (x0, y0), (x1, y1) in zip(table, table[1:]):
        if x <= x1:
            return y0 if x1 == x0 else y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return table[-1][1]


def side_z(y):
    """📐 Lateral position of the outer skin of the side wall at height y: vertical up to the belt, leaning in above it."""
    return W if y <= Y_BELT else W - LEAN * (y - Y_BELT)


def arch_y(x, xc, radius=RA):
    """⭕ Height of the wheel-arch circle about the axle at xc above x, or None outside it."""
    d = abs(x - xc)
    return None if d >= radius else YA + math.sqrt(radius * radius - d * d)


def arc(xc, yc, radius, deg0, deg1, steps):
    return [[xc + radius * math.cos(math.radians(deg0 + (deg1 - deg0) * k / steps)), yc + radius * math.sin(math.radians(deg0 + (deg1 - deg0) * k / steps))] for k in range(steps + 1)]


def arch_points(xc, steps, radius=RA, floor=Y_SILL):
    """⭕ The arch cut-out from the sill line over the wheel back to the sill line, rear to front."""
    low = math.degrees(math.asin((floor - YA) / radius))
    return arc(xc, YA, radius, 180 - low, low, steps)


def nose_bottom(x):
    a = arch_y(x, XF)
    b = interp(NOSE_BOTTOM, x)
    return b if a is None else max(a, b)


def nose_top(x):
    return interp(NOSE_TOP, x)


def nose_half(x):
    return interp(NOSE_HALF, x)


def nose_exponent(x):
    return interp(NOSE_EXPONENT, x)


def nose_xs(steps):
    xs = {x for x, _ in NOSE_TOP} | {x for x, _ in NOSE_BOTTOM} | {x for x, _ in NOSE_HALF} | {XF + RA * math.cos(math.pi * k / steps) for k in range(steps + 1)}
    out = sorted(x for x in xs if X_HINGE <= x <= X_NOSE)
    kept = [out[0]]
    for x in out[1:]:
        if x - kept[-1] >= 0.008:
            kept.append(x)
    return kept


def superellipse(a, b, n, t):
    return 0.0 if abs(t) >= 1 else a * (1 - abs(t) ** n) ** (1 / n)


def nose_z(x, y):
    """🧈 Lateral position of the skin of the front end at (x, y)."""
    bottom, top = nose_bottom(x), nose_top(x)
    return superellipse(nose_half(x), (top - bottom) / 2, nose_exponent(x), (y - (top + bottom) / 2) / ((top - bottom) / 2))


def sweep(z, radius=SCREEN_R):
    """🌬️ Plan sweep of the windscreen: how far the glass lies behind its centre line at z."""
    return radius - math.sqrt(radius * radius - z * z)


def pillar_x(y):
    """📏 Centre line of the A-pillar in the side view."""
    return 1.60 - (y - Y_BELT) * 0.64 / 0.69


def base(id, kind, bone, material, role, tone, **fields):
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def sec(at, rx, ry, n=None, q=None):
    out = {"at": rr(at), "rx": r5(float(rx)), "ry": r5(float(ry))}
    if n is not None:
        out["exponent"] = r5(float(n))
    if q is not None:
        out["rotation"] = q
    return out


def loft(id, sections, bone="chassis", material="paint", role="body", tone=0.65, caps=(True, True), **fields):
    return base(id, "loft", bone, material, role, tone, sections=sections, capStart=caps[0], capEnd=caps[1], **fields)


def capsule(id, p0, p1, r0, r1=None, bone="chassis", material="paint", role="detail", tone=0.5, **fields):
    return base(id, "capsule", bone, material, role, tone, p0=rr(p0), p1=rr(p1), r0=r5(float(r0)), r1=r5(float(r0 if r1 is None else r1)), **fields)


def box(id, size, at, bone="chassis", material="paint", role="detail", tone=0.5, radius=None, rotation=None, **fields):
    transform = {"translation": rr(at)}
    if rotation is not None:
        transform = {"rotation": rotation, "translation": rr(at)}
    out = base(id, "box", bone, material, role, tone, size=rr(size), transform=transform, **fields)
    if radius is not None:
        out["cornerRadius"] = radius
    return out


def ellipsoid(id, radii, at, bone="chassis", material="paint", role="detail", tone=0.5, rotation=None, **fields):
    transform = {"translation": rr(at)}
    if rotation is not None:
        transform = {"rotation": rotation, "translation": rr(at)}
    return base(id, "ellipsoid", bone, material, role, tone, radii=rr(radii), transform=transform, **fields)


def revolve(id, profile, axis, origin, bone="chassis", material="paint", role="detail", tone=0.5, **fields):
    return base(id, "revolve", bone, material, role, tone, profile=[rr(p) for p in profile], axis=rr(axis), origin=rr(origin), **fields)


def extrude(id, profile, plane, lo, hi, bone="chassis", material="paint", role="detail", tone=0.5, holes=None, **fields):
    out = base(id, "extrude", bone, material, role, tone, profile=[rr(p) for p in profile], plane=plane, **{"from": r5(float(min(lo, hi))), "to": r5(float(max(lo, hi)))}, **fields)
    if holes:
        out["holes"] = [[rr(p) for p in hole] for hole in holes]
    return out


def q_axis(axis, deg):
    half = math.radians(deg) / 2
    s = math.sin(half)
    return rr([axis[0] * s, axis[1] * s, axis[2] * s, math.cos(half)])


def q_side(sign, beta=BETA):
    theta = sign * (math.pi / 2 - beta)
    return rr([math.sin(theta / 2), 0.0, 0.0, math.cos(theta / 2)])


def q_screen(rho=RHO):
    phi = math.pi - rho
    return rr([0.0, 0.0, math.sin(phi / 2), math.cos(phi / 2)])


def shifted(points, origin):
    return [[x - origin[0], y - origin[1]] for x, y in points]


def wall(id, sign, outline, thickness=T, proud=0.0, origin=(0.0, 0.0), holes=None, **fields):
    """🧱 A vertical side panel below the belt: its side-view outline pushed through the wall thickness; proud moves the outer face outwards."""
    z0, z1 = sign * (W - thickness), sign * (W + proud)
    return extrude(id, shifted(outline, origin), "xy", z0, z1, holes=[shifted(h, origin) for h in holes] if holes else None, **fields)


def plate(id, sign, rows, thickness=T, proud=0.0, sunk=0.0, origin=(0.0, 0.0), n=24, **fields):
    """🪟 A flat panel in the leaning plane of one side above the belt: rows are (y, x rear, x front); proud moves the outer face outwards along the normal and thickens the panel, sunk moves the whole panel inwards."""
    half = (thickness + proud) / 2
    inward = half - proud + sunk
    sections = []
    for y, xr, xf in rows:
        z = W - LEAN * (y - Y_BELT) - inward * math.cos(BETA)
        sections.append(sec([(xr + xf) / 2 - origin[0], y - inward * math.sin(BETA) - origin[1], sign * z], (xf - xr) / 2, half, n, q_side(sign)))
    return loft(id, sections, **fields)


def nose(id, steps, x0=X_HINGE, x1=X_NOSE, grow=0.0, top=None, bottom=None, **fields):
    """🛻 The front end as one loft along the length: every section spans from the sill, arch or bumper line to the fender, cowl or bonnet line of the master tables."""
    sections = []
    for x in nose_xs(steps):
        if x < x0 - 1e-9 or x > x1 + 1e-9:
            continue
        lo, hi = nose_bottom(x) if bottom is None else max(bottom, nose_bottom(x)), nose_top(x) if top is None else min(top, nose_top(x))
        sections.append(sec([x, (hi + lo) / 2, 0], nose_half(x) + grow, (hi - lo) / 2 + grow / 2, nose_exponent(x), QX))
    return loft(id, sections, **fields)


def roof(id, rows=ROOF, n=5, **fields):
    """🏠 The roof as one loft along the length: rows are (x, height of the widest line, half thickness, half width)."""
    return loft(id, [sec([x, yc, 0], rx, ry, n, QX) for x, yc, ry, rx in rows], **fields)


def windscreen(id, zs, half_base=0.90, half_top=0.845, thickness=0.006, lift=0.0, **fields):
    """🌬️ The windscreen as a sheet bent about a vertical axis: one raked strip per lateral station, the corner strips shortened along the A-pillar line."""
    sin_r, cos_r = math.sin(RHO), math.cos(RHO)
    length = (SCREEN_TOP[1] - SCREEN_BASE[1]) / sin_r
    sections = []
    for z in zs:
        share = 1.0 if abs(z) <= half_top else max(0.04, (half_base - abs(z)) / (half_base - half_top))
        mid = (share * length - 0.012) / 2
        sections.append(sec([SCREEN_BASE[0] - sweep(z) - cos_r * mid + sin_r * lift, SCREEN_BASE[1] + sin_r * mid + cos_r * lift, z], (share * length + 0.012) / 2, thickness / 2, 10, q_screen()))
    return loft(id, sections, material="glass", role="glazing", tone=0.88, **fields)


def screen_point(s, z, lift=0.0):
    """🧽 A point on the windscreen: s metres up the rake from its base at the lateral station z, lifted along the outward normal."""
    sin_r, cos_r = math.sin(RHO), math.cos(RHO)
    return [SCREEN_BASE[0] - sweep(z) - cos_r * s + sin_r * lift, SCREEN_BASE[1] + sin_r * s + cos_r * lift, z]


def torus(id, axis, origin, ring, tube, corners=8, **fields):
    profile = [[tube * math.cos(2 * math.pi * k / corners), ring + tube * math.sin(2 * math.pi * k / corners)] for k in range(corners)]
    return revolve(id, profile + [profile[0]], axis, origin, **fields)


def circle(xc, yc, radius, corners):
    return [[xc + radius * math.cos(2 * math.pi * k / corners), yc + radius * math.sin(2 * math.pi * k / corners)] for k in range(corners)]


def inset(points, t):
    """🧅 The polygon moved inwards by t (mitred corners), whatever its winding."""
    area = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1]))
    s = 1.0 if area > 0 else -1.0
    out = []
    count = len(points)
    for k in range(count):
        (xa, ya), (xb, yb), (xc, yc) = points[k - 1], points[k], points[(k + 1) % count]
        d0, d1 = (xb - xa, yb - ya), (xc - xb, yc - yb)
        l0, l1 = math.hypot(*d0), math.hypot(*d1)
        n0, n1 = (-d0[1] / l0 * s, d0[0] / l0 * s), (-d1[1] / l1 * s, d1[0] / l1 * s)
        bx, by = n0[0] + n1[0], n0[1] + n1[1]
        scale = t / max(0.2, 1 + n0[0] * n1[0] + n0[1] * n1[1])
        out.append([xb + bx * scale, yb + by * scale])
    return out


def shrunk(solid, t):
    """🧅 The same solid with its skin moved inwards by t: the inner layer shell of a nested thermal pair."""
    out = dict(solid)
    kind = solid["kind"]
    if kind == "loft":
        out["sections"] = [dict(s, rx=r5(s["rx"] - t), ry=r5(s["ry"] - t)) for s in solid["sections"]]
    elif kind == "capsule":
        out["r0"], out["r1"] = r5(solid["r0"] - t), r5(solid["r1"] - t)
    elif kind == "box":
        out["size"] = rr([v - 2 * t for v in solid["size"]])
    elif kind == "ellipsoid":
        out["radii"] = rr([v - t for v in solid["radii"]])
    elif kind == "extrude":
        out["profile"] = [rr(p) for p in inset(solid["profile"], t)]
        if "holes" in solid:
            out["holes"] = [[rr(p) for p in inset(h, -t)] for h in solid["holes"]]
        out["from"], out["to"] = r5(solid["from"] + t), r5(solid["to"] - t)
    else:
        raise ValueError(kind)
    return out


def ref(id, of, region, layer=None, faces=None):
    out = {"id": id, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    if faces:
        out["sectionFaces"] = faces
    return out


def member(solid, bone_member, part="member"):
    solid.update(member=bone_member, part=part)
    return solid


def quaternion(c0, c1, c2):
    """🧭 The unit quaternion [x, y, z, w] of the rotation whose matrix has the columns c0, c1, c2."""
    m00, m10, m20 = c0
    m01, m11, m21 = c1
    m02, m12, m22 = c2
    trace = m00 + m11 + m22
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1 + m00 - m11 - m22) * 2
        q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    elif m11 > m22:
        s = math.sqrt(1 + m11 - m00 - m22) * 2
        q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]
    else:
        s = math.sqrt(1 + m22 - m00 - m11) * 2
        q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]
    return rr(q)


def nose_patch(x, y, sign, sunk=0.0):
    """🩹 A point of the front-end skin on one side with the rotation that lays a flat part on it: local x along the skin horizontally, local y up the skin, local z outward; sunk moves the point inwards along the normal."""
    e = 0.004
    z = nose_z(x, y)
    nx, ny, nz = -(nose_z(x + e, y) - nose_z(x - e, y)) / (2 * e), -(nose_z(x, y + e) - nose_z(x, y - e)) / (2 * e), 1.0
    length = math.sqrt(nx * nx + ny * ny + nz * nz)
    n = (nx / length, ny / length, sign * nz / length)
    t1 = (n[2], 0.0, -n[0])
    l1 = math.hypot(t1[0], t1[2])
    t1 = (t1[0] / l1, 0.0, t1[2] / l1)
    t2 = (n[1] * t1[2] - n[2] * t1[1], n[2] * t1[0] - n[0] * t1[2], n[0] * t1[1] - n[1] * t1[0])
    return [x - sunk * n[0], y - sunk * n[1], sign * z - sunk * n[2]], quaternion(t1, t2, n)


TYRE = [[-0.085, 0.3545], [-0.05, 0.356], [0.05, 0.356], [0.085, 0.3545], [0.104, 0.345], [0.114, 0.325], [0.1175, 0.295], [0.114, 0.26], [0.104, 0.232], [0.096, 0.215], [0.092, 0.206], [-0.092, 0.206], [-0.096, 0.215], [-0.104, 0.232], [-0.114, 0.26], [-0.1175, 0.295], [-0.114, 0.325], [-0.104, 0.345], [-0.085, 0.3545]]
RIM = [[0.085, 0.221], [0.094, 0.221], [0.094, 0.2], [0.03, 0.2], [0.02, 0.17], [-0.05, 0.17], [-0.06, 0.2], [-0.094, 0.2], [-0.094, 0.221], [-0.085, 0.221], [-0.085, 0.206], [-0.055, 0.206], [-0.045, 0.176], [0.015, 0.176], [0.025, 0.206], [0.085, 0.206], [0.085, 0.221]]
DISC_VENTED = [[-0.014, 0.15], [-0.005, 0.15], [-0.005, 0.095], [0.005, 0.095], [0.005, 0.15], [0.014, 0.15], [0.014, 0.085], [-0.014, 0.085], [-0.014, 0.15]]
DISC_SOLID = [[-0.006, 0.14], [0.006, 0.14], [0.006, 0.08], [-0.006, 0.08], [-0.006, 0.14]]


def nose_stations(id, xs, grow=0.0, top=None, bottom=None, exponent=None, **fields):
    """🛻 The front end of a coarser scale: one section of the master lines at each hand-chosen station."""
    sections = []
    for x in xs:
        lo, hi = nose_bottom(x) if bottom is None else max(bottom, nose_bottom(x)), nose_top(x) if top is None else min(top, nose_top(x))
        sections.append(sec([x, (hi + lo) / 2, 0], nose_half(x) + grow, (hi - lo) / 2 + grow / 2, exponent or nose_exponent(x), QX))
    return loft(id, sections, **fields)


def arch_stations(steps):
    """⭕ The stations of the front arch for a loft: its two feet and the chord points between them."""
    return [XF + RA * math.cos(math.pi * k / steps) * 0.9999 for k in range(steps, -1, -1)]


def post(id, sign, levels, half_x, half_z, x=-2.5725, n=6, **fields):
    """🧍 A rear corner post as one loft of horizontal sections that follows the side plane: levels are heights."""
    return loft(id, [sec([x, y, sign * (side_z(y) - half_z)], half_x, half_z, n, q_side(sign, 0.0)) for y in levels], **fields)


def sill_step(id, sign, x0=-1.19, x1=1.285, **fields):
    """🪜 Sill cover and door step of one side as one L-shaped section along the length."""
    profile = [[0.30, 0.922], [0.30, 0.988], [0.40, 0.988], [0.40, 0.94], [0.47, 0.94], [0.47, 0.88], [0.39, 0.88], [0.39, 0.922]]
    return extrude(id, [[y, sign * z] for y, z in profile], "yz", x0, x1, **fields)
