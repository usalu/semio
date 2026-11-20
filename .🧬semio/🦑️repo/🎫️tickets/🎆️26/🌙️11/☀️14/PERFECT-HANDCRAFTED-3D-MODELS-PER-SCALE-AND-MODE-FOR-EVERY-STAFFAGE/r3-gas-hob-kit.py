"""🔥 Round 3 domino gas hob: the real measures of a 0.30 m table-top domino hob with one 2.5 kW rapid burner and the solid writers every scale states its parts with.

Frames: hob bone = world (x across the hob to the right as seen from the front, y up, z depth: the front with the knob at -z, the back at +z). burner bone at world (0, 0.04), v = y - 0.04; flame and flame-body bones at world (0, 0.047); flame-tip at world (0, 0.062); trivet bone at world (0, 0.04); knob bone at world (0.105, 0.019). The burner axis stands at z = BURNER_Z. Every number is metres.
"""
import math

BURNER_Z = 0.03

PLATE_X = 0.15
PLATE_Z0 = -0.236
PLATE_Z1 = 0.26
PLATE_R = 0.01
PLATE_Y0 = 0.03
PLATE_Y1 = 0.04

HOUSING_X = 0.14
HOUSING_Z0 = -0.234
HOUSING_Z1 = 0.25
HOUSING_R = 0.008
FOOT_H = 0.0075
FOOT_R = 0.01
FEET_X = 0.12
FEET_Z = (-0.21, 0.225)
BEAD_Y = 0.0235

WELL_R = 0.075
WELL = [(0.0303, 0.075), (0.04, 0.075), (0.0393, 0.0715), (0.0372, 0.0655), (0.0362, 0.06), (0.036, 0.055), (0.036, 0.036), (0.0303, 0.036)]

BURNER_Y = 0.04
CUP = [(-0.004, 0.0), (-0.004, 0.041), (0.002, 0.043), (0.002, 0.045), (0.0125, 0.045), (0.0125, 0.0)]
CAP = [(0.0122, 0.0), (0.0122, 0.0465), (0.0135, 0.048), (0.0172, 0.048), (0.019, 0.0462), (0.0196, 0.03), (0.02, 0.0)]
PORT_N = 36

FLAME_Y = 0.047
TIP_Y = 0.062
FLAME_RING = [(-0.0005, 0.0442), (-0.0005, 0.0505), (0.007, 0.0505), (0.007, 0.0442)]
TONGUE = [(0.0445, 0.0015, 0.0036, 0.006), (0.0495, 0.0032, 0.0038, 0.0064), (0.054, 0.0072, 0.0038, 0.0058), (0.0568, 0.0125, 0.0033, 0.0046), (0.058, 0.0178, 0.0022, 0.0028), (0.0582, 0.0215, 0.0007, 0.0008)]
TONGUE_STEP = 15.0

KNOB_X = 0.105
KNOB_Y = 0.019
KNOB_FACE = HOUSING_Z0
KNOB = [(0.0, 0.0), (0.0, 0.013), (0.0035, 0.013), (0.0042, 0.0115), (0.0148, 0.0115), (0.016, 0.0105), (0.016, 0.0)]
RIDGE = (0.0075, 0.022, 0.0075)

TRIVET_Y = 0.04
TRIVET_HALF = 0.135
TRIVET_BAR = 0.008
TRIVET_TOP = 0.048
TRIVET_LEG = 0.008
FINGER_TIP = 0.062
FINGER = [(0.127, 0.0482), (0.064, 0.0482), (0.0625, 0.0475), (0.062, 0.0455), (0.0622, 0.038), (0.0635, 0.0362), (0.066, 0.036), (0.12, 0.0398), (0.127, 0.0398)]


def r5(x):
    return round(x, 5) + 0.0


def v3(p):
    return [r5(p[0]), r5(p[1]), r5(p[2])]


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


def solid(sid, kind, bone, material, role, tone, **extra):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update({k: v for k, v in extra.items() if v is not None})
    return s


def revolve(sid, bone, material, role, tone, profile, origin=(0.0, 0.0, BURNER_Z), axis=(0, 1, 0), **extra):
    """🌀 A turned solid; a profile that starts and ends off the axis is closed explicitly, so the kernel never fills it through the axis."""
    profile = list(profile)
    if profile[0][1] > 1e-9 and profile[-1][1] > 1e-9 and tuple(profile[0]) != tuple(profile[-1]):
        profile.append(profile[0])
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(r)] for a, r in profile], axis=list(axis), origin=v3(origin), **extra)


def ellipsoid(sid, bone, material, role, tone, radii, at, **extra):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": v3(at)}, **extra)


def box(sid, bone, material, role, tone, size, at, corner=0.0, **extra):
    s = solid(sid, "box", bone, material, role, tone, size=[r5(c) for c in size], transform={"translation": v3(at)}, **extra)
    if corner:
        s["cornerRadius"] = r5(corner)
    return s


def capsule(sid, bone, material, role, tone, p0, p1, r, **extra):
    return solid(sid, "capsule", bone, material, role, tone, p0=v3(p0), p1=v3(p1), r0=r5(r), r1=r5(r), **extra)


def extrude(sid, bone, material, role, tone, profile, plane, lo, hi, holes=None, **extra):
    return solid(sid, "extrude", bone, material, role, tone, profile=[[r5(a), r5(b)] for a, b in profile], plane=plane, **{"from": r5(lo), "to": r5(hi)}, holes=[[[r5(a), r5(b)] for a, b in h] for h in holes] if holes else None, **extra)


def loft(sid, bone, material, role, tone, sections, **extra):
    return solid(sid, "loft", bone, material, role, tone, sections=sections, capStart=True, capEnd=True, **extra)


def ring_repeat(count, step_deg, centre=(0.0, 0.0, BURNER_Z)):
    """🔁 Copies rotated about the vertical axis through centre (positive steps turn +x towards -z)."""
    return {"count": count, "rotationAxis": [0, 1, 0], "rotationRad": r5(math.radians(step_deg)), "origin": v3(centre)}


def rounded_rect(x0, x1, z0, z1, r, steps=4):
    """⬜ Rounded rectangle as extrude profile points [z, x] for plane zx, counter-clockwise."""
    pts = []
    corners = [(x1 - r, z1 - r, 0), (x0 + r, z1 - r, 90), (x0 + r, z0 + r, 180), (x1 - r, z0 + r, 270)]
    for cx, cz, a0 in corners:
        for k in range(steps + 1):
            a = math.radians(a0 + 90.0 * k / steps)
            pts.append((cz + r * math.sin(a), cx + r * math.cos(a)))
    return pts


def circle_zx(cz, cx, r, n=48):
    return [(cz + r * math.sin(2 * math.pi * k / n), cx + r * math.cos(2 * math.pi * k / n)) for k in range(n)]


def tongue_sections(theta_deg, path=TONGUE, centre_z=BURNER_Z, v0=0.0, scale=1.0):
    """🔥 Loft sections of one flame tongue at azimuth theta (x = r cos, z = centre + r sin): every section square to the tongue's centre line in its radial plane; rx = radial thickness, ry = tangential width."""
    t = math.radians(theta_deg)
    er = (math.cos(t), 0.0, math.sin(t))
    et = (-math.sin(t), 0.0, math.cos(t))
    secs = []
    for k, (r, v, th, wd) in enumerate(path):
        a = path[max(k - 1, 0)]
        b = path[min(k + 1, len(path) - 1)]
        dr, dv = b[0] - a[0], b[1] - a[1]
        n = math.hypot(dr, dv)
        dr, dv = dr / n, dv / n
        z_ax = (dr * er[0], dv, dr * er[2])
        x_ax = (-dv * er[0], dr, -dv * er[2])
        secs.append({"at": v3((r * er[0], v0 + v, centre_z + r * er[2])), "rx": r5(th * scale), "ry": r5(wd * scale), "exponent": 2, "rotation": quat_from_axes(x_ax, et, z_ax)})
    return secs
