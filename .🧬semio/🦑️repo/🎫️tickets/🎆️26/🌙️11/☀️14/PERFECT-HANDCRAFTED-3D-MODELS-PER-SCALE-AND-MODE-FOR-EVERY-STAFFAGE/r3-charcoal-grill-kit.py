"""🔥 Round 3 charcoal kettle grill: the real geometry of a 57 cm kettle (Weber Original Kettle class) and the solid writers every scale states its parts with.

World = bowl bone frame: x ahead (lid handle side), y up, z left. Every number is metres.
"""
import math

C_B = 0.7704
RHO_B = 0.2904
T_SH = 0.0008
RIM_R = 0.2885
BEAD_Y = 0.738
BEAD_R = 0.0042
BOWL_BOTTOM = C_B - RHO_B

LID_X0 = -0.2925
LID_Y0 = 0.74
C_L = 0.68754
RHO_L = 0.29446
LID_TOP = 0.982
LBEAD_Y = 0.746

VENT_V = LID_TOP - LID_Y0
DIAL_R = 0.04
DIAL_U0 = -0.004
DIAL_U1 = 0.008
FIN_U1 = 0.018
HOLE_R = 0.006
HOLES_VW = [(0.0, 0.022), (-0.019053, -0.011), (0.019053, -0.011)]

GRATE_R = 0.273
GRATE_Y = 0.70
GRATE_PITCH = 0.0195
FUEL_R = 0.2085
FUEL_Y = 0.575
BED_A = 0.195
BED_H = 0.04
BED_Y = 0.58

LEG_R = 0.0125
LEG_TOP_Y = 0.535
FOOT_X = 0.30
LEG_SLOPE = 0.105 / 0.553
AXLE_Y = 0.076
WHEEL_RAD = 0.076
WHEEL_Z = 0.282
TRI_Y = 0.24
WIRE_R = 0.004
AZ = {"front": 0.0, "left": 120.0, "right": -120.0}

PAN_R = 0.15
PAN_BOTTOM = 0.385
PAN_TOP = 0.415

HANDLE_Y = 0.705
HANDLE_Z = 0.322
LID_GRIP_U = 0.625
LID_GRIP_V = 0.06


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


def bowl_y(r, rho=RHO_B):
    return C_B - math.sqrt(rho * rho - r * r)


def bowl_r(y, rho=RHO_B):
    return math.sqrt(rho * rho - (C_B - y) ** 2)


def lid_y(r, rho=RHO_L):
    return C_L + math.sqrt(rho * rho - r * r)


def leg_r(y):
    return FOOT_X - LEG_SLOPE * y


def leg_point(side, y):
    a = math.radians(AZ[side])
    r = leg_r(y)
    return (r * math.cos(a), y, r * math.sin(a))


def axle_x():
    return leg_point("left", AXLE_Y)[0]


def socket_top_y():
    y = 0.55
    for _ in range(60):
        y = bowl_y(leg_r(y))
    return y


def to_lid(p):
    return (p[0] - LID_X0, p[1] - LID_Y0, p[2])


def to_grate(p):
    return (p[0] + GRATE_R, p[1] - GRATE_Y, p[2])


def to_charcoal(p):
    return (p[0] + BED_A, p[1] - BED_Y, p[2])


def norm(v):
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v)


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def quat_from_axes(x, y, z):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose columns are the given orthonormal axes."""
    m00, m01, m02 = x[0], y[0], z[0]
    m10, m11, m12 = x[1], y[1], z[1]
    m20, m21, m22 = x[2], y[2], z[2]
    tr = m00 + m11 + m22
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s)
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2
        q = (0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s)
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2
        q = ((m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s)
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2
        q = ((m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s)
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) + 0.0 for c in q]


def quat_y(angle_deg):
    h = math.radians(angle_deg) / 2
    return [0.0, round(math.sin(h), 6) + 0.0, 0.0, round(math.cos(h), 6) + 0.0]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def base(sid, kind, bone, material, role, tone):
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def revolve(sid, bone, material, role, tone, profile, axis, origin, **extra):
    s = base(sid, "revolve", bone, material, role, tone)
    s.update({"profile": [[r5(a), r5(r)] for a, r in profile], "axis": axis, "origin": v3(origin)})
    s.update(extra)
    return s


def capsule(sid, bone, material, role, tone, p0, p1, r0, r1=None, **extra):
    s = base(sid, "capsule", bone, material, role, tone)
    s.update({"p0": v3(p0), "p1": v3(p1), "r0": r5(r0), "r1": r5(r0 if r1 is None else r1)})
    s.update(extra)
    return s


def box(sid, bone, material, role, tone, size, at, corner=0.0, rotation=None, **extra):
    s = base(sid, "box", bone, material, role, tone)
    s["size"] = [r5(c) for c in size]
    if corner:
        s["cornerRadius"] = r5(corner)
    t = {"translation": v3(at)}
    if rotation is not None:
        t["rotation"] = rotation
    s["transform"] = t
    s.update(extra)
    return s


def ellipsoid(sid, bone, material, role, tone, radii, at, rotation=None, **extra):
    s = base(sid, "ellipsoid", bone, material, role, tone)
    s["radii"] = [r5(c) for c in radii]
    t = {"translation": v3(at)}
    if rotation is not None:
        t["rotation"] = rotation
    s["transform"] = t
    s.update(extra)
    return s


def extrude(sid, bone, material, role, tone, profile, plane, z0, z1, holes=None, **extra):
    s = base(sid, "extrude", bone, material, role, tone)
    s.update({"profile": [[r5(a), r5(b)] for a, b in profile], "plane": plane, "from": r5(z0), "to": r5(z1)})
    if holes:
        s["holes"] = [[[r5(a), r5(b)] for a, b in h] for h in holes]
    s.update(extra)
    return s


def loft(sid, bone, material, role, tone, sections, **extra):
    s = base(sid, "loft", bone, material, role, tone)
    out = []
    for sec in sections:
        o = {"at": v3(sec["at"]), "rx": r5(sec["rx"]), "ry": r5(sec["ry"])}
        if "n" in sec:
            o["exponent"] = sec["n"]
        if "rot" in sec:
            o["rotation"] = sec["rot"]
        out.append(o)
    s.update({"sections": out, "capStart": True, "capEnd": True})
    s.update(extra)
    return s


def circle(r, n=24, cx=0.0, cy=0.0, start=0.0):
    return [(cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)) for k in range(n)]


def shell_profile(rho_out, y_top, bottom_flat=0.0, n=22):
    """🥣 Open revolve profile of the bowl shell: outer sphere zone from the bottom pole to the rim, inner surface back to the pole."""
    a_top = math.asin(bowl_r(y_top, rho_out) / rho_out)
    out = [(C_B - rho_out * math.cos(a_top * k / n), rho_out * math.sin(a_top * k / n)) for k in range(n + 1)]
    rho_in = rho_out - T_SH
    a_in = math.asin(bowl_r(y_top, rho_in) / rho_in)
    inn = [(C_B - rho_in * math.cos(a_in * k / n), rho_in * math.sin(a_in * k / n)) for k in range(n, -1, -1)]
    return out + inn


def bowl_outline(n=22):
    """🥣 Outer outline of bowl and bead (the elevation and front silhouette), from the left bead top over the bottom to the right bead top."""
    top_y = bowl_y(RIM_R)
    a_top = math.asin(RIM_R / RHO_B)
    right = [(RHO_B * math.sin(a_top * k / n), C_B - RHO_B * math.cos(a_top * k / n)) for k in range(n + 1)]
    bead = [(RIM_R + BEAD_R * math.cos(math.radians(a)), BEAD_Y + BEAD_R * math.sin(math.radians(a))) for a in (-60, -30, 0, 30, 60, 90)]
    half = right[:-1] + bead
    return [(-x, y) for x, y in reversed(half)] + half[1:], top_y


def lid_profile_outer(n=22):
    """⛑️ Lid dome outer surface as (radial, world y) from the top pole down to the bead centre."""
    a_end = math.asin(RIM_R / RHO_L)
    return [(RHO_L * math.sin(a_end * k / n), C_L + RHO_L * math.cos(a_end * k / n)) for k in range(n + 1)]


def lid_shell_profile(n=22):
    """⛑️ Open revolve profile of the lid shell in the lid frame about the vertical axis through u = 0.2925: [axial v, radial]."""
    a_end = math.asin(RIM_R / RHO_L)
    out = [(C_L + RHO_L * math.cos(a_end * k / n) - LID_Y0, RHO_L * math.sin(a_end * k / n)) for k in range(n + 1)]
    rho_in = RHO_L - T_SH
    y_end = lid_y(RIM_R)
    a_in = math.asin(math.sqrt(rho_in ** 2 - (y_end - C_L) ** 2) / rho_in)
    inn = [(C_L + rho_in * math.cos(a_in * k / n) - LID_Y0, rho_in * math.sin(a_in * k / n)) for k in range(n, -1, -1)]
    return out + inn


def torus(center_axial, center_radial, r, n=14):
    pts = [(center_axial + r * math.sin(2 * math.pi * k / n), center_radial + r * math.cos(2 * math.pi * k / n)) for k in range(n)]
    return pts + [pts[0]]


def pan_profile(top=PAN_TOP, bottom=PAN_BOTTOM):
    """🍳 Open revolve profile of the 2 mm cast aluminium ash pan with a rolled lip: outer bottom, flared wall, lip, inner wall back to the axis."""
    t = 0.002
    return [
        (bottom, 0.0), (bottom, PAN_R - 0.014), (bottom + 0.0015, PAN_R - 0.0055), (bottom + 0.005, PAN_R - 0.002),
        (top - 0.002, PAN_R), (top, PAN_R + 0.0045), (top + 0.003, PAN_R + 0.006), (top + 0.003, PAN_R + 0.001),
        (top - 0.002, PAN_R - t), (bottom + 0.005 + t * 0.4, PAN_R - 0.002 - t), (bottom + 0.0015 + t, PAN_R - 0.0055 - t * 0.7), (bottom + t, PAN_R - 0.014 - t * 0.3), (bottom + t, 0.0),
    ]


def wheel_profile(hub=True):
    """🛞 Open revolve profile of a 152 mm (6 in) plastic wheel about its axle, outward = +axial: dished outer face with hub cap, rounded tread shoulders."""
    if hub:
        return [(0.024, 0.0), (0.024, 0.018), (0.022, 0.024), (0.013, 0.026), (0.013, 0.052), (0.019, 0.064), (0.019, 0.07), (0.0165, 0.0745), (0.012, 0.076),
                (-0.012, 0.076), (-0.0165, 0.0745), (-0.019, 0.07), (-0.019, 0.064), (-0.016, 0.05), (-0.019, 0.022), (-0.019, 0.0)]
    return [(0.019, 0.0), (0.019, 0.066), (0.0155, 0.074), (0.01, 0.076), (-0.01, 0.076), (-0.0155, 0.074), (-0.019, 0.066), (-0.019, 0.0)]


def bed_profile(a=BED_A, h=BED_H, n=10):
    rc = (a * a + h * h) / (2 * h)
    cap = [(math.sqrt(rc * rc - (a * (n - k) / n) ** 2) - (rc - h), a * (n - k) / n) for k in range(n + 1)]
    return [(-0.0015, 0.0)] + [(v - 0.0015, r) for v, r in cap]


def bed_surface(r, a=BED_A, h=BED_H):
    rc = (a * a + h * h) / (2 * h)
    return math.sqrt(rc * rc - r * r) - (rc - h) - 0.0015


BRIQUETTES = [(0.0, 0.0, 0), (0.055, 0.02, 40), (-0.05, 0.035, 110), (0.02, -0.06, 75), (-0.035, -0.05, 15), (0.1, -0.025, 130), (0.085, 0.07, 60), (-0.095, 0.0, 95),
              (-0.06, 0.095, 150), (0.03, 0.11, 20), (-0.01, -0.115, 55), (0.12, 0.045, 170), (-0.125, -0.06, 35), (0.065, -0.1, 140)]


def section_frame(d, n=None):
    """🧭 Section axes of a straight sleeve along d: perpendicular to d, or cut by the plane of normal n with x along the tilt, so that every section of the sleeve shares one x direction (no twist)."""
    if n is None:
        return None
    n = n if dot(d, n) > 0 else tuple(-c for c in n)
    m = tuple(dc - dot(d, n) * nc for dc, nc in zip(d, n))
    if math.sqrt(dot(m, m)) < 1e-9:
        m = cross(d, (0.0, 0.0, 1.0)) if abs(d[2]) < 0.9 else cross(d, (1.0, 0.0, 0.0))
    m = norm(m)
    return m, n


def flush_loft(sid, bone, material, role, tone, p0, p1, r, n0=None, n1=None, r_mid=None, inset=0.0004, **extra):
    """🔩 A round sleeve or rod from p0 to p1 whose end faces lie in the planes of normal n0 and n1 (a shell, the ground) instead of across the rod, so it sits flush without entering the surface; all sections share one x axis."""
    d = norm(tuple(b - a for a, b in zip(p0, p1)))
    ref_n = n1 if n1 is not None else n0
    if ref_n is not None:
        m, _ = section_frame(d, ref_n)
    else:
        m = norm(cross(d, (0.0, 0.0, 1.0)) if abs(d[2]) < 0.9 else cross(d, (1.0, 0.0, 0.0)))
    xp = norm(tuple(mc - dot(m, d) * dc for mc, dc in zip(m, d)))
    perp = quat_from_axes(xp, cross(d, xp), d)

    def cut(p, n, sign):
        if n is None:
            return {"at": p, "rx": r, "ry": r, "n": 2, "rot": perp}
        mm, nn = section_frame(d, n)
        if dot(mm, xp) < 0:
            mm = tuple(-c for c in mm)
        q = quat_from_axes(mm, cross(nn, mm), nn)
        shift = tuple(pc + sign * inset * dc for pc, dc in zip(p, d))
        return {"at": shift, "rx": r / abs(dot(d, nn)), "ry": r, "n": 2, "rot": q}

    mid = tuple((a + b) / 2 for a, b in zip(p0, p1))
    rm = r if r_mid is None else r_mid
    return loft(sid, bone, material, role, tone, [cut(p0, n0, -1), {"at": mid, "rx": rm, "ry": rm, "n": 2, "rot": perp}, cut(p1, n1, 1)], **extra)


def bowl_normal(p):
    return norm((p[0], p[1] - C_B, p[2]))


def lid_normal(p_lid):
    return norm((p_lid[0] - 0.2925, p_lid[1] - (C_L - LID_Y0), p_lid[2]))


def socket_loft(sid, side, r, material="steel-black", role="body", tone=0.25, y_low=0.49, **extra):
    """🔩 Leg socket: a sleeve along the leg axis from below up to the bowl, its top cut in the tangent plane of the shell."""
    y_top = socket_top_y()
    p_top = leg_point(side, y_top)
    return flush_loft(sid, "bowl", material, role, tone, leg_point(side, y_low), p_top, r, n1=bowl_normal(p_top), r_mid=r, **extra)


def foot_loft(sid, r=0.0165, h=0.03, **extra):
    """🦶 Plastic foot cap on the front leg: a sleeve along the leg whose sole is cut level with the ground."""
    p_low = leg_point("front", 0.0)
    return flush_loft(sid, "bowl", "steel-black", "body", 0.15, leg_point("front", h), p_low, r, n1=(0.0, -1.0, 0.0), inset=0.0, **extra)


def damper_u_surface(r):
    return BOWL_BOTTOM - bowl_y(r)


def damper_profile(r_plate=0.06, t=0.0045, n=6):
    """🌀 Open revolve profile of the dished bottom damper plate in the ash-damper frame (u down from the bowl pole), hugging the bowl."""
    rs = [r_plate * k / n for k in range(n + 1)]
    low = [(damper_u_surface(r) - 0.0004 + t, r) for r in rs]
    up = [(damper_u_surface(r) - 0.0004, r) for r in reversed(rs)]
    return low + [(damper_u_surface(r_plate) - 0.0004 + t * 0.5, r_plate + 0.002)] + up


LEVER_AZ = 0.0
LEVER_END = 0.135


def lever_point(s, below=0.0065):
    """🌀 A point of the damper lever centre line in the ash-damper frame (u down, v ahead, w left) at radius s, 6.5 mm under the bowl, pointing ahead under the lid handle and ending well inside the front leg."""
    a = math.radians(LEVER_AZ)
    return (damper_u_surface(s) + below, s * math.cos(a), s * math.sin(a))


def lever_sections():
    return [{"at": lever_point(s), "rx": 0.004, "ry": 0.0065, "n": 4} for s in (0.045, 0.075, 0.105, LEVER_END)]


def lid_shell_to(y_end, n=20, top=None):
    """⛑️ Open revolve profile of a lid shell without bead (coarser scales) in the lid frame, from the top pole down to world height y_end and back inside."""
    rho = RHO_L
    a_end = math.acos((y_end - C_L) / rho)
    out = [(C_L + rho * math.cos(a_end * k / n) - LID_Y0, rho * math.sin(a_end * k / n)) for k in range(n + 1)]
    rho_in = rho - T_SH
    a_in = math.acos((y_end - C_L) / rho_in)
    inn = [(C_L + rho_in * math.cos(a_in * k / n) - LID_Y0, rho_in * math.sin(a_in * k / n)) for k in range(n, -1, -1)]
    return out + inn


def bowl_envelope_to(y_end, n=20):
    """🥣 Closed outer envelope of the bowl up to y_end (rim face is the section)."""
    a_end = math.asin(bowl_r(y_end) / RHO_B)
    return [(C_B - RHO_B * math.cos(a_end * k / n), RHO_B * math.sin(a_end * k / n)) for k in range(n + 1)] + [(y_end, 0.0)]


def lid_envelope_from(y_end, n=20):
    """⛑️ Closed outer envelope of the lid dome down to y_end in the lid frame (bottom face is the section)."""
    a_end = math.acos((y_end - C_L) / RHO_L)
    return [(C_L + RHO_L * math.cos(a_end * k / n) - LID_Y0, RHO_L * math.sin(a_end * k / n)) for k in range(n + 1)] + [(y_end - LID_Y0, 0.0)]


def dome_u(r, w=0.0):
    """⛑️ Height of the dome surface in the lid-vent frame (u up from the dome top) at radius r from the vent axis."""
    return C_L + math.sqrt(RHO_L ** 2 - r * r - w * w) - LID_TOP


def dial_profile(top, r=DIAL_R, skirt=0.0005):
    """🌀 Open revolve profile of a vent dial seated on the dome: dished underside following the dome, flat top at u = top."""
    return [(dome_u(0.0) - 0.0004, 0.0), (dome_u(r * 0.5) - 0.0004, r * 0.5), (dome_u(r + skirt) - 0.0004, r + skirt), (top - 0.004, r + skirt), (top, r - 0.004), (top, 0.0)]


def wheel_profile_wide(width):
    h = width / 2
    return [(h, 0.0), (h, 0.064), (h - 0.006, 0.074), (h - 0.012, 0.076), (-h + 0.012, 0.076), (-h + 0.006, 0.074), (-h, 0.064), (-h, 0.0)]
