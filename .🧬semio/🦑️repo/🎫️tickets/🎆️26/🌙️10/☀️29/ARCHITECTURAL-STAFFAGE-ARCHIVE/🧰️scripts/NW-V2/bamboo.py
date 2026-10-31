"""bamboo-clump: Phyllostachys aureosulcata clump (6 culms) in a rhizome barrier."""
import sys
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from common import *
from shapes import *
from union import union

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


def emit_union(D, id, bone, zz, tone, region, loops, cell, eps, close=0, open_=0):
    for k, lp in enumerate(union(loops, cell, eps, min_area=3e-4, close=close, open_=open_)):
        D.poly(f"{id}-{k}", bone, "body", zz, True, tone, lp, closed=True, region=region)


GROUPS = (("A", (6, 1, 2), "fol-1"), ("B", (3, 4, 5), "fol-4"))


def build_50():
    D = Draw(B)
    loops = {g[0]: {"sun": [], "shade": []} for g in GROUPS}
    grp = {n: g[0] for g in GROUPS for n in g[1]}
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 1, True, 0.25, quad(P[i], P[i + 1], HW[i], HW[i + 1]), region="culms", material="bamboo-culm")
        for k, z in enumerate([0.13, 0.22, 0.35, 0.46]):
            h = hw_at(z) * 1.3
            D.poly(f"node-{n}-{k}", f"culm-{n}-{seg_bone(z)}", "texture", 5, False, 0.2, [culm_side(c, z, h), culm_side(c, z, -h)], closed=False)
        for k, z in enumerate([0.58, 0.64, 0.70, 0.76, 0.82, 0.88, 0.94]):
            side = 1 if (k + n) % 2 == 0 else -1
            p, a = ctl(c, z)
            Lb = (0.62 - 0.30 * (z - 0.58) / 0.4) * sc
            thb = 90 - side * (58 + 5 * (k % 3))
            tgt = loops[grp[n]]["sun" if z >= 0.76 else "shade"]
            tgt.append(plume(p, pol(p, thb - side * 30, Lb), 0.17 * sc, 10, 0.45, side * 0.10, 0.25, k))
            tgt.append(plume(p, pol(p, thb + side * 38, 0.7 * Lb), 0.12 * sc, 8, 0.45, -side * 0.06, 0.25, k + 1))
        topp, topa = ctl(c, 1.0)
        loops[grp[n]]["sun"].append(plume(ctl(c, 0.93)[0], (topp[0], topp[1] + 0.04), 0.2 * sc, 8, 0.45, 0.0, 0.25, 1))
    for g, members, bone in GROUPS:
        emit_union(D, f"foliage-{g}-shade", bone, 3, 0.72, "leaves-shade", loops[g]["shade"], 0.003, 0.0022, 3, 2)
        emit_union(D, f"foliage-{g}-sun", bone, 3.1, 0.55, "leaves-sun", loops[g]["sun"], 0.003, 0.0022, 3, 2)
    return D.shapes


def build_100():
    D = Draw(B)
    loops = {g[0]: {"sun": [], "shade": []} for g in GROUPS}
    grp = {n: g[0] for g in GROUPS for n in g[1]}
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        hw = (0.0150, 0.0140, 0.0130, 0.0122)
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 1, True, 0.25, quad(P[i], P[i + 1], hw[i], hw[i + 1]), region="culms")
        for k, z in enumerate([0.13, 0.22, 0.35, 0.46]):
            D.poly(f"node-{n}-{k}", f"culm-{n}-{seg_bone(z)}", "texture", 5, False, 0.2, [culm_side(c, z, 0.021), culm_side(c, z, -0.021)], closed=False)
        for k, z in enumerate([0.60, 0.69, 0.78, 0.87]):
            side = 1 if (k + n) % 2 == 0 else -1
            p, a = ctl(c, z)
            thb = 90 - side * 60
            tip = pol(p, thb - side * 38, (0.66 - 0.08 * k) * sc)
            loops[grp[n]]["sun" if z >= 0.78 else "shade"].append(plume(p, tip, 0.36 * sc, 8, 0.42, side * 0.12, 0.22, k))
        topp, topa = ctl(c, 1.0)
        loops[grp[n]]["sun"].append(plume(ctl(c, 0.93)[0], (topp[0], topp[1] + 0.05), 0.34 * sc, 8, 0.45, 0.0, 0.22, 1))
    for g, members, bone in GROUPS:
        emit_union(D, f"spray-{g}-shade", bone, 3, 0.7, "leaves-shade", loops[g]["shade"], 0.005, 0.0035, 3, 3)
        emit_union(D, f"spray-{g}-sun", bone, 3.1, 0.5, "leaves-sun", loops[g]["sun"], 0.005, 0.0035, 3, 3)
    return D.shapes


def build_200():
    D = Draw(B)
    loops = {g[0]: {"sun": [], "shade": []} for g in GROUPS}
    grp = {n: g[0] for g in GROUPS for n in g[1]}
    for c in culms:
        n, H, P = c["n"], c["H"], c["P"]
        sc = 0.8 + 0.2 * H / 4.5
        for i, nm in enumerate("abc"):
            D.poly(f"culm-{n}-{nm}", f"culm-{n}-{nm}", "body", 1, False, 0.25, [P[i], P[i + 1]], closed=False, region="culms")
        b0, _ = ctl(c, 0.54)
        t0, _ = ctl(c, 0.86)
        loops[grp[n]]["shade"].append(plume(b0, t0, 0.95 * sc, 8, 0.5, 0.0, 0.20, n))
        b1, _ = ctl(c, 0.72)
        t1, _ = ctl(c, 1.0)
        loops[grp[n]]["sun"].append(plume(b1, (t1[0], t1[1] + 0.05), 0.7 * sc, 8, 0.5, 0.0, 0.2, n + 1))
    for g, members, bone in GROUPS:
        emit_union(D, f"mass-{g}-shade", bone, 3, 0.7, "leaves-shade", loops[g]["shade"], 0.008, 0.006, 3)
        emit_union(D, f"mass-{g}-sun", bone, 3.1, 0.5, "leaves-sun", loops[g]["sun"], 0.008, 0.006, 3)
    return D.shapes


def build_500():
    D = Draw(B)
    D.poly("stems", "culm-3-a", "body", 2, True, 0.3, [(-0.92, 0), (0.74, 0), (0.95, 2.0), (0.0, 2.5), (-1.0, 2.0)], closed=True, region="culms")
    for g, members, bone in GROUPS:
        loops = []
        for c in culms:
            if c["n"] not in members:
                continue
            H = c["H"]
            sc = 0.8 + 0.2 * H / 4.5
            base, _ = ctl(c, 0.50)
            top, _ = ctl(c, 1.0)
            loops.append(lancet(base, (top[0], top[1] + 0.06), 1.04 * sc, 7, 0.55, 0.02 * (1 if c["lean"] > 0 else -1)))
        for k, lp in enumerate(union(loops, 0.02, 0.016, min_area=0.01, close=3)):
            D.poly(f"crown-{g}-{k}", bone, "body", 3, True, 0.55, lp, closed=True, regions=["leaves-sun", "leaves-shade"])
    return D.shapes


def plan_symbol():
    shapes = [{"id": "crown-extent", "bone": "root-plate", "role": "body", "z": 1, "filled": False, "tone": 0.5, "kind": "path", "d": catmull(blob(-0.05, 0, 1.55, 0.85, 0, 7, 0.07, 0, 28), True)}]
    for c in culms:
        shapes.append({"id": f"culm-{c['n']}", "bone": "root-plate", "role": "detail", "z": 2, "filled": False, "tone": 0.3, "kind": "ellipse", "cx": r4(c["x0"]), "cy": r4(0.22 * ((c["n"] * 37) % 7 - 3) / 3), "rx": 0.0145, "ry": 0.0145})
    return shapes
