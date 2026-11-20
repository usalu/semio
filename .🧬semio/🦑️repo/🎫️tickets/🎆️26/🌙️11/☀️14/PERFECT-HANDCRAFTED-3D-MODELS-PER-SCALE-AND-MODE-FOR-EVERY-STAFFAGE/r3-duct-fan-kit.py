"""🌀 Round 3, appliances/duct-fan: measures and hand-stated part geometry (plate, nozzle, spigot, blades, hub, spider) shared by the volumes and roots scripts.

Frame: bone-local u = x (right in the elevation), v = y (up), w = z (into the wall); the room is at -z, the plate face at z = -0.045, its back on the wall at z = -0.033.
"""
import math

PLATE = 0.22
PLATE_R = 0.014
Z_FACE = -0.045
Z_WALL = -0.033
BORE = 0.100
THROAT = 0.088
SPIGOT_IN = 0.0975
LIP_R = 0.104
Z_END = 0.045
Z_LIP = 0.0375
Z_SEAM = -0.015
SCREW = 0.092
HUB_R = 0.034
BLADES = 5
TIP = 0.087
ROOT = 0.028
Z_BLADE = -0.0185
PHI0 = math.radians(18)


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


def blade_sections(k, count=BLADES, phi0=PHI0, root=ROOT, tip=TIP, chord=(0.040, 0.062), pitch=(38.0, 24.0), sweep=22.0, thick=(0.0032, 0.0020), boss=0.0076, z=Z_BLADE, stations=9, hub=HUB_R):
    """🪭 Loft sections of blade k: a root boss of thickness `boss` buried in the hub at radius `root`, then planar airfoil lenses normal to the radius from the hub surface out, chord pitched by `pitch` (root, tip degrees) to the rotation plane with the leading edge upstream (-z) and ahead (+theta), swept forward by `sweep` degrees, the tip cut so that both blade corners reach `tip`."""
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


if __name__ == "__main__":
    for k in range(1):
        sec = blade_sections(k)
        pts = loft_points(sec)
        print("blade", k, "extents mm", [round(e * 1000, 2) for e in principal_extents(pts)])
        print("radii", [round(math.hypot(p[0], p[1]) * 1000, 1) for p in pts][-24:])
        print("z", round(min(p[2] for p in pts) * 1000, 1), round(max(p[2] for p in pts) * 1000, 1))
