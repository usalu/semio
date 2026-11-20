"""🧰 Hand-stated master lines and solid writers of the round 3 city-car (vehicles/city-car): one set of design lines (side profile, plan half-widths, arch circles, glasshouse planes) that the solids of every scale and the 2D drawings of every view are written from.

Frame of the chassis bone: x ahead, y up, z to the left of the car, metres, origin on the ground under the middle of the length.
Reference: B-segment hatchbacks of 3.85 to 3.97 m (Suzuki Swift AZG 3850 x 1695 x 1510, wheelbase 2430; VW Polo 6R 3970 x 1682 x 1462, wheelbase 2470; Honda Jazz GE 3900 x 1695 x 1525, wheelbase 2500), tyre 185/60 R15 (ETRTO: 603 mm).
"""
import math

XF, XR = 1.15, -1.30
ZW = 0.72
RT = 0.30
RA = 0.365
YA = 0.30
BONNET = (0.95, 0.97, -10.6, 0.93)
HATCH = (-1.5, 1.44, -125.6, 0.74)

QX = [0.70711, 0.0, 0.70711, 0.0]
QH = [0.70711, 0.0, 0.0, 0.70711]
TUMBLE = 0.375
BETA = math.atan(TUMBLE)
RHO = math.atan2(0.455, 0.68)


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


def bonnet_y(x):
    """🚗 Height of the bonnet hinge plane (the bonnet bone axis) at x."""
    return BONNET[1] + math.tan(math.radians(BONNET[2])) * (x - BONNET[0])


def hatch_y(x):
    """🚪 Height of the outer tailgate line (the hatch bone axis) at x."""
    return HATCH[1] + math.tan(math.radians(HATCH[2] + 180)) * (x - HATCH[0])


TOP = [(-1.95, 0.58), (-1.912, 0.60), (-1.905, 0.825), (-1.78, 1.0), (0.93, 1.0), (0.94, bonnet_y(0.94)), (1.864, bonnet_y(1.864)), (1.90, 0.765), (1.93, 0.72), (1.945, 0.66), (1.95, 0.60)]
BOTTOM = [(-1.95, 0.40), (-1.93, 0.34), (-1.90, 0.32), (-0.935, 0.32), (-0.927, 0.235), (0.777, 0.235), (0.785, 0.26), (1.85, 0.26), (1.90, 0.275), (1.93, 0.31), (1.945, 0.34), (1.95, 0.36)]
HALF = [(-1.95, 0.62), (-1.93, 0.70), (-1.905, 0.745), (-1.85, 0.785), (-1.78, 0.81), (-1.665, 0.83), (-1.30, 0.84), (-0.935, 0.84), (-0.6, 0.83), (0.0, 0.825), (0.5, 0.83), (0.785, 0.84), (1.15, 0.84), (1.515, 0.825), (1.70, 0.79), (1.80, 0.735), (1.864, 0.675), (1.90, 0.62), (1.93, 0.54), (1.945, 0.46), (1.95, 0.40)]
EXPONENT = [(-1.95, 6), (-1.85, 7), (-1.70, 10), (-1.665, 12), (-0.935, 12), (-0.927, 7), (0.777, 7), (0.785, 12), (1.515, 12), (1.6, 9), (1.80, 6), (1.95, 5)]


def arch(x):
    """⭕ Height of the wheel-arch circle above x, or None outside both arches."""
    for xc in (XR, XF):
        if abs(x - xc) < RA:
            return YA + math.sqrt(RA * RA - (x - xc) ** 2)
    return None


def hull_bottom(x):
    a = arch(x)
    b = interp(BOTTOM, x)
    return b if a is None else max(a, b)


def hull_top(x):
    return interp(TOP, x)


def hull_half(x):
    return interp(HALF, x)


def hull_exponent(x):
    return interp(EXPONENT, x)


def arch_xs(xc, steps):
    return [xc + RA * math.cos(math.pi * k / steps) for k in range(steps, -1, -1)]


def hull_xs(steps):
    xs = {x for x, _ in TOP} | {x for x, _ in BOTTOM} | {x for x, _ in HALF if abs(x) > 1.6 or abs(x) < 0.7} | set(arch_xs(XR, steps)) | set(arch_xs(XF, steps))
    out = sorted(xs)
    kept = [out[0]]
    for x in out[1:]:
        if x - kept[-1] >= 0.006:
            kept.append(x)
    return kept


def superellipse(a, b, n, t):
    """📐 Half-width of the section |z/a|^n + |y/b|^n = 1 at the height share t = |y|/b."""
    return 0.0 if abs(t) >= 1 else a * (1 - abs(t) ** n) ** (1 / n)


def hull_z(x, y):
    """🧈 Lateral position of the hull skin at (x, y)."""
    bottom, top = hull_bottom(x), hull_top(x)
    return superellipse(hull_half(x), (top - bottom) / 2, hull_exponent(x), (y - (top + bottom) / 2) / ((top - bottom) / 2))


def side_z(y):
    """🪟 Lateral position of the flat tumblehome plane of the side glass at height y."""
    return 0.775 - TUMBLE * (y - 1.0)


def q_side(sign):
    theta = sign * (math.pi / 2 - BETA)
    return rr([math.sin(theta / 2), 0.0, 0.0, math.cos(theta / 2)])


def q_screen():
    phi = math.pi - RHO
    return rr([0.0, 0.0, math.sin(phi / 2), math.cos(phi / 2)])


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


def loft(id, sections, bone="chassis", material="paint", role="body", tone=0.62, caps=(True, True), **fields):
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


def extrude(id, profile, plane, lo, hi, bone="chassis", material="paint", role="detail", tone=0.5, **fields):
    return base(id, "extrude", bone, material, role, tone, profile=[rr(p) for p in profile], plane=plane, **{"from": r5(float(lo)), "to": r5(float(hi))}, **fields)


def q_axis(axis, deg):
    half = math.radians(deg) / 2
    s = math.sin(half)
    return rr([axis[0] * s, axis[1] * s, axis[2] * s, math.cos(half)])


def hull(id, steps, **fields):
    """🛻 The lower body as one loft along the length: every section spans from the floor, sill or arch line to the belt, bonnet plane or tail line of the master tables."""
    sections = []
    for x in hull_xs(steps):
        bottom, top = hull_bottom(x), hull_top(x)
        sections.append(sec([x, (top + bottom) / 2, 0], hull_half(x), (top - bottom) / 2, hull_exponent(x), QX))
    return loft(id, sections, **fields)


def hull_stations(id, xs, **fields):
    """🛻 The lower body of a coarser scale: one section of the master lines at each hand-chosen station."""
    sections = []
    for x in xs:
        bottom, top = hull_bottom(x), hull_top(x)
        sections.append(sec([x, (top + bottom) / 2, 0], hull_half(x), (top - bottom) / 2, hull_exponent(x), QX))
    return loft(id, sections, **fields)


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


def plan_loft(id, levels, n, **fields):
    """🏠 A vertical loft of plan sections: levels are (y, x rear, x front, half width)."""
    return loft(id, [sec([(xr + xf) / 2, y, 0], (xf - xr) / 2, half, n, QH) for y, xr, xf, half in levels], **fields)


def roof_loft(id, rows, **fields):
    """🏠 The roof as a loft along the length: rows are (x, underside, top, half width, exponent), so the plan sweep of the header and the crown across the car are both stated."""
    return loft(id, [sec([x, (b + t) / 2, 0], h, (t - b) / 2, n, QX) for x, b, t, h, n in rows], **fields)


def screen_point(s, z, lift=0.0):
    """🧽 A point on the windscreen: s metres up the rake from the belt plane at the lateral station z, lifted along the outward normal."""
    sin_r, cos_r = math.sin(RHO), math.cos(RHO)
    return [0.95 - screen_x(z) - cos_r * s + sin_r * lift, 1.0 + sin_r * s + cos_r * lift, z]


def side_plate(id, sign, rows, half_thickness, offset, n=8, **fields):
    """🪟 A flat plate lying in the tumblehome plane of one side: rows are (y, x rear, x front); offset moves it along the outward normal."""
    sections = []
    for y, xr, xf in rows:
        z = side_z(y) + offset * math.cos(BETA)
        sections.append(sec([(xr + xf) / 2, y + offset * math.sin(BETA), sign * z], (xf - xr) / 2, half_thickness, n, q_side(sign)))
    return loft(id, sections, **fields)


def screen_x(z, radius=1.95):
    """🌬️ Plan sweep of the windscreen base: how far the glass lies behind its centre line at z."""
    return radius - math.sqrt(radius * radius - z * z)


def windscreen(id, zs, y0=1.03, y1=1.44, half=0.745, flat=0.585, thickness=0.004, **fields):
    """🌬️ The windscreen as a sheet bent about a vertical axis: one raked strip per lateral station, the corner strips shortened along the A-pillar line."""
    sin_r, cos_r = math.sin(RHO), math.cos(RHO)
    sections = []
    for z in zs:
        top = y1 if abs(z) <= flat else y0 + (y1 - y0) * (half - abs(z)) / (half - flat)
        s0, s1 = (y0 - 1.0) / sin_r, (top - 1.0) / sin_r
        mid = (s0 + s1) / 2
        sections.append(sec([0.95 - screen_x(z) - cos_r * mid, 1.0 + sin_r * mid, z], (s1 - s0) / 2, thickness / 2, 8, q_screen()))
    return loft(id, sections, material="glass", role="glazing", tone=0.85, **fields)


def torus(id, axis, origin, ring, tube, corners=8, **fields):
    profile = [[tube * math.cos(2 * math.pi * k / corners), ring + tube * math.sin(2 * math.pi * k / corners)] for k in range(corners)]
    return revolve(id, profile + [profile[0]], axis, origin, **fields)


TYRE = [[-0.066, 0.2985], [-0.04, 0.3], [0.04, 0.3], [0.066, 0.2985], [0.082, 0.29], [0.0905, 0.272], [0.0925, 0.245], [0.088, 0.215], [0.078, 0.196], [0.072, 0.189], [-0.072, 0.189], [-0.078, 0.196], [-0.088, 0.215], [-0.0925, 0.245], [-0.0905, 0.272], [-0.082, 0.29], [-0.066, 0.2985]]
RIM = [[0.0775, 0.208], [0.0835, 0.208], [0.0835, 0.184], [0.033, 0.184], [0.023, 0.166], [-0.033, 0.166], [-0.043, 0.184], [-0.0835, 0.184], [-0.0835, 0.208], [-0.0775, 0.208], [-0.0775, 0.19], [-0.04, 0.19], [-0.03, 0.172], [0.02, 0.172], [0.03, 0.19], [0.0775, 0.19], [0.0775, 0.208]]
DISC_VENTED = [[-0.011, 0.15], [-0.004, 0.15], [-0.004, 0.09], [0.004, 0.09], [0.004, 0.15], [0.011, 0.15], [0.011, 0.08], [-0.011, 0.08], [-0.011, 0.15]]
DISC_SOLID = [[-0.006, 0.15], [0.006, 0.15], [0.006, 0.075], [-0.006, 0.075], [-0.006, 0.15]]


def shrunk(solid, t):
    """🧅 The same solid with its skin moved inwards by t: the inner layer shell of a nested thermal pair."""
    out = {k: (v if k not in ("sections",) else [dict(s) for s in v]) for k, v in solid.items()}
    kind = solid["kind"]
    if kind == "loft":
        for s in out["sections"]:
            s["rx"], s["ry"] = r5(s["rx"] - t), r5(s["ry"] - t)
    elif kind == "capsule":
        out["r0"], out["r1"] = r5(solid["r0"] - t), r5(solid["r1"] - t)
    elif kind == "box":
        out["size"] = rr([v - 2 * t for v in solid["size"]])
    elif kind == "ellipsoid":
        out["radii"] = rr([v - t for v in solid["radii"]])
    else:
        raise ValueError(kind)
    return out


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


def hull_patch(x, y, sign, sunk=0.0):
    """🩹 A point of the hull skin on one side with the rotation that lays a flat part on it: local x along the skin horizontally, local y up the skin, local z outward; sunk moves the point inwards along the normal."""
    e = 0.004
    z = hull_z(x, y)
    nx, ny, nz = -(hull_z(x + e, y) - hull_z(x - e, y)) / (2 * e), -(hull_z(x, y + e) - hull_z(x, y - e)) / (2 * e), 1.0
    length = math.sqrt(nx * nx + ny * ny + nz * nz)
    n = (nx / length, ny / length, sign * nz / length)
    t1 = (n[2], 0.0, -n[0])
    l1 = math.hypot(t1[0], t1[2])
    t1 = (t1[0] / l1, 0.0, t1[2] / l1)
    t2 = (n[1] * t1[2] - n[2] * t1[1], n[2] * t1[0] - n[0] * t1[2], n[0] * t1[1] - n[1] * t1[0])
    return [x - sunk * n[0], y - sunk * n[1], sign * z - sunk * n[2]], quaternion(t1, t2, n)
