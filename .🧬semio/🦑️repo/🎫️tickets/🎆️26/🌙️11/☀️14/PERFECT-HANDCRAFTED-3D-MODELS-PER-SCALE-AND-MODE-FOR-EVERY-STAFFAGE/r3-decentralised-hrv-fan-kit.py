"""🌬️ Round 3, appliances/decentralised-hrv-fan: measures and hand-stated part geometry (base tray, fan housing, sleeve, EPP jacket, honeycomb core, reversible blades, hub, struts) shared by the volumes and roots scripts.

Frame: bone-local u = x (right in the elevation), v = y (up), w = z (through the wall); the room is at -z. The rim of the base tray and the lip of the fan housing are at z = -0.12, the tray lies on the room face of the wall at z = -0.10 (its floor with ribs and foam gasket is 7.5 mm, the printable feature of 1:50), the sleeve ends at the outer face at z = 0.12.
"""
import math

PLATE = 0.26
PLATE_R = 0.02
SHELL = 0.0025
Z_FACE = -0.12
Z_FLOOR = -0.1075
Z_WALL = -0.10
Z_END = 0.12
Z_RIM = 0.105
SLEEVE_OUT = 0.098
SLEEVE_IN = 0.095
RIM_R = 0.104
HOUSING_OUT = 0.0945
THROAT = 0.088
Z_STAGE = -0.045
CORE_R = 0.08
Z_CORE0 = -0.037
Z_CORE1 = 0.063
Z_JACKET = 0.07
SEAT_R = 0.076
SCREW = 0.108
BOSS_R = 0.007
HUB_R = 0.028
Z_HUB0 = -0.103
Z_HUB1 = -0.073
BLADES = 5
TIP = 0.086
ROOT = 0.022
Z_BLADE = -0.088
PHI0 = math.radians(54)
STRUTS = (90.0, 210.0, 330.0)
Z_STRUT = -0.056
CELL = 0.02165
CELL_100 = 0.03


def r6(v):
    """🔢 Round to a micrometre."""
    return round(v + 0.0, 6)


def pt(p):
    """📍 A rounded point."""
    return [r6(c) for c in p]


def rounded_square(half, radius, n=6):
    """⬜ Counter-clockwise rounded square of half side `half` with corner radius `radius`, `n` segments per corner."""
    out = []
    for cx, cy, a0 in ((half - radius, -half + radius, -90), (half - radius, half - radius, 0), (-half + radius, half - radius, 90), (-half + radius, -half + radius, 180)):
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            out.append(pt((cx + radius * math.cos(a), cy + radius * math.sin(a))))
    return out


def circle(r, n=64, cx=0.0, cy=0.0):
    """⭕ Counter-clockwise circle polygon."""
    return [pt((cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n))) for i in range(n)]


def arc(cz, cr, radius, a0, a1, n):
    """🌙 Points (axial, radial) of a circular arc about (cz, cr) from angle a0 to a1 in degrees (0 = +axial, 90 = +radial)."""
    return [pt((cz + radius * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cr + radius * math.sin(math.radians(a0 + (a1 - a0) * i / n)))) for i in range(n + 1)]


def quat(x, y, z):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose columns are the orthonormal axes x, y, z."""
    m00, m10, m20 = x
    m01, m11, m21 = y
    m02, m12, m22 = z
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
    return [round(c / n, 7) for c in q]


def cross(a, b):
    """✖️ Cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def blade_sections(k, count=BLADES, phi0=PHI0, root=ROOT, tip=TIP, chord=(0.034, 0.056), pitch=(36.0, 24.0), sweep=0.0, thick=(0.0030, 0.0020), boss=0.0076, z=Z_BLADE, stations=9, hub=HUB_R):
    """🪭 Loft sections of blade k: a root boss of thickness `boss` buried in the hub at radius `root`, then planar airfoil lenses normal to the radius from the hub surface out, chord pitched by `pitch` (root, tip degrees) to the rotation plane with the leading edge upstream (-z) and ahead (+theta), swept forward by `sweep` degrees; beyond the last full chord two shortening sections round the tip along the circle of radius `tip`."""
    out = []
    tip_mid = math.sqrt(tip * tip - (0.5 * chord[1] * math.cos(math.radians(pitch[1]))) ** 2)
    first = hub + 0.0005
    radii = [root] + [first + (tip_mid - first) * i / (stations - 2) for i in range(stations - 1)]
    for i, r in enumerate(radii):
        s = min(1.0, max(0.0, (r - hub) / (tip_mid - hub)))
        c = chord[0] + (chord[1] - chord[0]) * s
        beta = math.radians(pitch[0] + (pitch[1] - pitch[0]) * s)
        psi = 2 * math.pi * k / count + phi0 + math.radians(sweep) * s
        er = (math.cos(psi), math.sin(psi), 0.0)
        et = (-math.sin(psi), math.cos(psi), 0.0)
        x = tuple(math.cos(beta) * et[j] - math.sin(beta) * (0, 0, 1)[j] for j in range(3))
        y = cross(er, x)
        t = boss if i == 0 else thick[0] + (thick[1] - thick[0]) * s
        out.append({"at": pt((r * er[0], r * er[1], z)), "rx": r6(c / 2), "ry": r6(t / 2), "exponent": 2, "rotation": quat(x, y, er)})
    for share in (0.5, 0.85):
        r = tip_mid + share * (tip - tip_mid)
        c = 2 * math.sqrt(tip * tip - r * r) / math.cos(beta)
        out.append({"at": pt((r * er[0], r * er[1], z)), "rx": r6(c / 2), "ry": r6(t / 2), "exponent": 2, "rotation": quat(x, y, er)})
    return out


def loft_points(sections, n=24):
    """☁️ Sample points of loft rings (for extents and checks)."""
    pts = []
    for s in sections:
        qx, qy, qz, qw = s["rotation"]
        def rot(v):
            ux, uy, uz = v
            tx = 2 * (qy * uz - qz * uy)
            ty = 2 * (qz * ux - qx * uz)
            tz = 2 * (qx * uy - qy * ux)
            return (ux + qw * tx + qy * tz - qz * ty, uy + qw * ty + qz * tx - qx * tz, uz + qw * tz + qx * ty - qy * tx)
        for j in range(n):
            a = 2 * math.pi * j / n
            v = rot((s["rx"] * math.cos(a), s["ry"] * math.sin(a), 0))
            pts.append(tuple(s["at"][i] + v[i] for i in range(3)))
    return pts


def principal_extents(pts):
    """📐 Extents of a point cloud over its covariance eigenvectors (Jacobi), smallest first."""
    n = len(pts)
    m = [sum(p[i] for p in pts) / n for i in range(3)]
    c = [[sum((p[i] - m[i]) * (p[j] - m[j]) for p in pts) / n for j in range(3)] for i in range(3)]
    v = [[1.0 if i == j else 0.0 for j in range(3)] for i in range(3)]
    for _ in range(60):
        p, q = max(((0, 1), (0, 2), (1, 2)), key=lambda ij: abs(c[ij[0]][ij[1]]))
        if abs(c[p][q]) < 1e-18:
            break
        th = 0.5 * math.atan2(2 * c[p][q], c[q][q] - c[p][p])
        cs, sn = math.cos(th), math.sin(th)
        for k in range(3):
            cpk, cqk = c[p][k], c[q][k]
            c[p][k], c[q][k] = cs * cpk - sn * cqk, sn * cpk + cs * cqk
        for k in range(3):
            ckp, ckq = c[k][p], c[k][q]
            c[k][p], c[k][q] = cs * ckp - sn * ckq, sn * ckp + cs * ckq
        for k in range(3):
            vkp, vkq = v[k][p], v[k][q]
            v[k][p], v[k][q] = cs * vkp - sn * vkq, sn * vkp + cs * vkq
    ext = []
    for j in range(3):
        axis = [v[k][j] for k in range(3)]
        proj = [sum(p[k] * axis[k] for k in range(3)) for p in pts]
        ext.append(max(proj) - min(proj))
    return sorted(ext)


def hexagon(cx, cy, radius):
    """⬡ Counter-clockwise pointed-top hexagon of circumradius `radius`."""
    return [pt((cx + radius * math.cos(math.radians(30 + 60 * i)), cy + radius * math.sin(math.radians(30 + 60 * i)))) for i in range(6)]


def cell_centres(pitch, rings):
    """🍯 Centres of a hexagonal packing with `rings` rings round the middle cell, row by row."""
    out = []
    for r in range(-rings, rings + 1):
        for q in range(-rings, rings + 1):
            if abs(q + r) <= rings:
                out.append((pitch * (q + r / 2), pitch * math.sqrt(3) / 2 * r))
    return out


def strut_outline(angle, r0, r1, width):
    """➖ The rectangle of a radial strut at `angle` degrees from radius r0 to r1 seen along the axis."""
    c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
    return [pt((r * c - h * s, r * s + h * c)) for r, h in ((r0, -width / 2), (r1, -width / 2), (r1, width / 2), (r0, width / 2))]


def planform(i, count, root, tip, chord, phi0=PHI0, n=6):
    """🪭 Blade i (1-based) of a coarse scale seen along the axis: a radial, symmetric plan-form whose chord grows from `chord[0]` at `root` to `chord[1]` at `tip`, counter-clockwise."""
    psi = 2 * math.pi * (i - 1) / count + phi0
    half = lambda r: (chord[0] + (chord[1] - chord[0]) * (r - root) / (tip - root)) / (2 * r)
    radii = [root + (tip - root) * j / n for j in range(n + 1)]
    lead = [(r, psi + half(r)) for r in radii]
    trail = [(r, psi - half(r)) for r in radii]
    tip_arc = [(tip, lead[-1][1] + (trail[-1][1] - lead[-1][1]) * j / 4) for j in range(1, 4)]
    ring = list(reversed(lead)) + trail + list(reversed(tip_arc))
    poly = [pt((r * math.cos(a), r * math.sin(a))) for r, a in ring]
    if sum(poly[j][0] * poly[(j + 1) % len(poly)][1] - poly[(j + 1) % len(poly)][0] * poly[j][1] for j in range(len(poly))) < 0:
        poly.reverse()
    return poly


if __name__ == "__main__":
    sec = blade_sections(0)
    pts = loft_points(sec)
    print("blade extents mm", [round(e * 1000, 2) for e in principal_extents(pts)])
    print("max radius mm", round(max(math.hypot(p[0], p[1]) for p in pts) * 1000, 2))
    print("z mm", round(min(p[2] for p in pts) * 1000, 1), round(max(p[2] for p in pts) * 1000, 1))
    print("cells", len(cell_centres(CELL, 3)), len(cell_centres(CELL_100, 2)))
