"""hosta-clump: broad-leaved shade perennial (Hosta sieboldiana type), 9 leaf groups."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from common import *
from shapes import *
from union import union

ID = "hosta-clump"
CROWN = (0.0, 0.06)
SPEC = {
    "R1": ((22, 0.16), (-14, 0.30)), "R2": ((45, 0.21), (10, 0.30)), "R3": ((65, 0.26), (28, 0.30)), "R4": ((80, 0.30), (52, 0.28)),
    "L1": ((158, 0.17), (194, 0.29)), "L2": ((133, 0.20), (168, 0.30)), "L3": ((114, 0.25), (150, 0.29)), "L4": ((101, 0.29), (130, 0.27)),
    "C1": ((90, 0.32), (98, 0.24)),
}
OUTER = ("R1", "R2", "L1", "L2")
B = Bones()
B.add("root-plate", None, T("Crown and root ball", "Wurzelstock mit Ballen"), 0, 0, 0, 0.9, FIXED, mass=(3.4, "authored-estimate", "schmid-1991"))
LEAF = {}
for n, ((pa, pl), (ba, bl)) in SPEC.items():
    p0 = CROWN
    p1 = (p0[0] + pl * cos(radians(pa)), p0[1] + pl * sin(radians(pa)))
    p2 = (p1[0] + bl * cos(radians(ba)), p1[1] + bl * sin(radians(ba)))
    B.add(f"petiole-{n}", "root-plate", T(f"Petiole {n}", f"Blattstiel {n}"), p0[0], p0[1], pa, pl, revolute(-30, 30), mass=(0.07, "authored-estimate", "schmid-1991"))
    B.add(f"blade-{n}", f"petiole-{n}", T(f"Leaf blade {n}", f"Blattspreite {n}"), p1[0], p1[1], ba, bl, revolute(-40, 40), mass=(0.07, "authored-estimate", "schmid-1991"))
    LEAF[n] = {"p0": p0, "p1": p1, "p2": p2, "ba": ba, "side": 1 if n[0] in "RC" else -1}
MASS = round(sum(b["mass"]["kg"] for b in B.list if "mass" in b), 1)


def spine(n, droop=0.12):
    l = LEAF[n]
    p1, p2 = l["p1"], l["p2"]
    m = lerp(p1, p2, 0.5)
    L = hypot(p2[0] - p1[0], p2[1] - p1[1])
    m = (m[0], m[1] + droop * L)
    q = lerp(p1, p2, 0.78)
    return [p1, lerp(p1, m, 0.7), m, (q[0], q[1] + droop * L * 0.9), p2]


def blade_hw(W):
    def f(t):
        return 0.5 * W * (sin(pi * min(1.0, t ** 0.62))) ** 0.85 * (1 - 0.25 * t)
    return f


def blade_pts(n, W, N=12, wave=0.0):
    return leaf_outline(spine(n), 0.0, 1.0, blade_hw(W), N, wave)


def petiole_shape(D, n, w, z, role="body", filled=True):
    l = LEAF[n]
    D.poly(f"petiole-{n}", f"petiole-{n}", role, z, filled, 0.3, quad(l["p0"], l["p1"], w, w * 0.8), closed=True, region="petioles")


def build_50():
    D = Draw(B)
    order = ["R1", "L1", "R2", "L2", "R3", "L3", "R4", "L4", "C1"]
    for zi, n in enumerate(order):
        reg = "blades-outer" if n in OUTER else "blades-inner"
        D.smooth(f"blade-{n}", f"blade-{n}", "body", 2 + zi * 0.01, True, 0.4 if n in OUTER else 0.55, blade_pts(n, 0.25, 14, 0.03), region=reg)
        sp = spine(n)
        D.smooth(f"midrib-{n}", f"blade-{n}", "detail", 5 + zi * 0.01, False, 0.3, [lerp(sp[0], sp[2], 0.25), sp[2], lerp(sp[2], sp[4], 0.6)], closed=False)
    return D.shapes


def build_100():
    D = Draw(B)
    order = ["R1", "L1", "R2", "L2", "R3", "L3", "R4", "L4", "C1"]
    for zi, n in enumerate(order):
        reg = "blades-outer" if n in OUTER else "blades-inner"
        D.smooth(f"blade-{n}", f"blade-{n}", "body", 2 + zi * 0.01, True, 0.4 if n in OUTER else 0.55, blade_pts(n, 0.27, 10), region=reg)
    return D.shapes


def build_200():
    D = Draw(B)
    for nm, names, bone, tone, reg in (("outer", ["R1", "R2", "L1", "L2"], "blade-R1", 0.4, "blades-outer"), ("inner", ["R3", "R4", "L3", "L4", "C1"], "blade-R3", 0.55, "blades-inner")):
        loops = [blade_pts(n, 0.30, 8) for n in names]
        loops = [[(p[0], p[1]) for p in l] for l in loops]
        for k, lp in enumerate(union(loops, 0.01, 0.008, min_area=0.002, close=2, open_=2)):
            D.poly(f"mass-{nm}-{k}", bone, "body", 2 if nm == "inner" else 1, True, tone, lp, closed=True, region=reg)
    return D.shapes


def plan_symbol():
    sh = []
    for i, (cx, cy, rx, ry, rot) in enumerate([(0.2, 0.15, 0.27, 0.15, 25), (-0.2, 0.2, 0.27, 0.15, 150), (0.25, -0.15, 0.27, 0.15, -25), (-0.25, -0.18, 0.27, 0.15, 205), (0.0, 0.3, 0.27, 0.15, 90), (0.0, -0.3, 0.27, 0.15, -90), (0.0, 0.0, 0.2, 0.12, 0)]):
        sh.append({"id": f"leaf-{i}", "bone": "root-plate", "role": "body", "z": 1 + i * 0.1, "filled": False, "tone": 0.5, "kind": "ellipse", "cx": cx * 1.0, "cy": cy * 1.0, "rx": rx, "ry": ry, "rotationDeg": rot})
    return sh
