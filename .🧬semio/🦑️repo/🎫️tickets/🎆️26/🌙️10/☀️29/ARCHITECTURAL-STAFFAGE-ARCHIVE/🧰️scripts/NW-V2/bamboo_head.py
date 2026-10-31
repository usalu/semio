"""bamboo-clump: Phyllostachys aureosulcata clump (6 culms) in a rhizome barrier."""
import sys
from lib import *
from common import *
from shapes import *

ID = "bamboo-clump"
CULMS = [(1, -0.60, 3.7, -0.55), (2, -0.28, 4.5, -0.08), (3, 0.02, 4.1, 0.30), (4, 0.32, 3.5, 0.70), (5, 0.62, 2.9, 0.62), (6, -0.82, 2.4, -0.38)]
FR = (0, 0.28, 0.55, 1.0)
HW = (0.0145, 0.0135, 0.0115, 0.0075)

B = Bones()
B.add("root-plate", None, T("Rhizome plate in the root barrier", "Rhizomplatte in der Wurzelsperre"), 0, 0, 0, 1.6, FIXED, mass=(8.0, "authored-estimate", "liese-koehl-2015"))


def culm_pts(x0, H, lean):
    return [(x0 + lean * (f ** 1.7), H * f) for f in FR]


def ctl(c, z):
    """Centre-line point of culm c at height fraction z (piecewise linear through the bone joints)."""
    P = c["P"]
    for i in range(3):
        if z <= FR[i + 1] + 1e-9:
            u = (z - FR[i]) / (FR[i + 1] - FR[i])
            return lerp(P[i], P[i + 1], u), math.degrees(atan2(P[i + 1][1] - P[i][1], P[i + 1][0] - P[i][0]))
    return P[3], 90


def hw_at(z):
    return float(np_interp(z, FR, HW))


def np_interp(x, xs, ys):
    for i in range(len(xs) - 1):
        if x <= xs[i + 1]:
            u = (x - xs[i]) / (xs[i + 1] - xs[i])
            return ys[i] + (ys[i + 1] - ys[i]) * u
    return ys[-1]


culms = []
for (n, x0, H, lean) in CULMS:
    P = culm_pts(x0, H, lean)
    c = {"n": n, "x0": x0, "H": H, "lean": lean, "P": P}
    culms.append(c)
    names = ["a", "b", "c"]
    lim = [revolute(-8, 8), revolute(-14, 14), revolute(-24, 24)]
    kgm = [0.22, 0.20, 0.16]
    for i in range(3):
        p0, p1 = P[i], P[i + 1]
        L = hypot(p1[0] - p0[0], p1[1] - p0[1])
        ang = math.degrees(atan2(p1[1] - p0[1], p1[0] - p0[0]))
        D = 2 * HW[i]
        d_in = D - 2 * (0.004 if i < 2 else 0.0035)
        A = pi / 4 * (D * D - d_in * d_in)
        I = pi / 64 * (D ** 4 - d_in ** 4)
        sec = {"areaM2": float(f"{A:.3e}"), "secondMomentM4": float(f"{I:.3e}"), "extremeFibreM": r4(D / 2), "material": "bamboo-culm", "title": T(f"Hollow culm, outer diameter {D*1000:.0f} mm", f"Hohler Halm, Außendurchmesser {D*1000:.0f} mm")}
        parent = "root-plate" if i == 0 else f"culm-{n}-{names[i-1]}"
        B.add(f"culm-{n}-{names[i]}", parent, T(f"Culm {n}, {['lower','middle','upper arching'][i]} part", f"Halm {n}, {['unterer','mittlerer','oberer bogiger'][i]} Teil"), p0[0], p0[1], ang, L, lim[i], mass=(round(kgm[i] * L, 3), "authored-estimate", "liese-1985"), section=sec)
    pm, ang_c = ctl(c, 0.775)
    Lc = B.w[f"culm-{n}-c"][3]
    B.add(f"fol-{n}", f"culm-{n}-c", T(f"Foliage and branches of culm {n}", f"Laub und Äste von Halm {n}"), pm[0], pm[1], ang_c, Lc * 0.52, revolute(-14, 14), mass=(round(0.42 * H, 3), "authored-estimate", "jones-2014"))

MASS = round(sum(b["mass"]["kg"] for b in B.list if "mass" in b), 1)


def culm_side(c, z, off):
    p, a = ctl(c, z)
    return (p[0] + cos(radians(a + 90)) * off, p[1] + sin(radians(a + 90)) * off)


NODE_Z = [0.07, 0.14, 0.22, 0.30, 0.38, 0.46, 0.54, 0.61, 0.68, 0.75, 0.82, 0.89, 0.96]


def seg_bone(z):
    return "a" if z < FR[1] else "b" if z < FR[2] else "c"


