"""agave: Agave americana rosette (CAM), 28 leaves represented by 20 leaf bones."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from common import *
from shapes import *
from union import union

ID = "agave"
HEART = (0.0, 0.30)
B = Bones()
B.add("root-plate", None, T("Root plate in the planting bed", "Wurzelplatte im Pflanzbeet"), 0, 0, 0, 1.0, FIXED, mass=(10.0, "authored-estimate", "nobel-1988"))
B.add("caudex", "root-plate", T("Caudex (short trunk) with leaf bases", "Caudex (kurzer Stamm) mit Blattbasen"), 0, 0, 90, 0.30, FIXED, mass=(14.0, "authored-estimate", "nobel-1988"))

LEAVES = {}
SPEC = {
    "R1": [(10, 0.85), (-34, 0.55)], "R2": [(31, 0.80), (4, 0.52)], "R3": [(50, 0.80), (24, 0.50)], "R4": [(66, 0.78), (46, 0.45)], "R5": [(78, 1.0)],
    "L1": [(170, 0.82), (214, 0.53)], "L2": [(149, 0.78), (178, 0.50)], "L3": [(130, 0.82), (156, 0.52)], "L4": [(114, 0.74), (134, 0.45)], "L5": [(102, 0.95)],
    "S": [(90, 1.15)],
}
LEAF_KG = {"R1": 4.4, "R2": 4.0, "R3": 3.6, "R4": 3.2, "R5": 2.6, "L1": 4.3, "L2": 4.0, "L3": 3.7, "L4": 3.1, "L5": 2.5, "S": 2.6}
REGION = {"R1": "leaves-spreading", "R2": "leaves-spreading", "L1": "leaves-spreading", "L2": "leaves-spreading"}
for k in SPEC:
    REGION.setdefault(k, "leaves-erect")
for name, parts in SPEC.items():
    p = HEART
    spine = [p]
    ids = []
    for i, (ang, L) in enumerate(parts):
        nm = f"leaf-{name}" + (f"-{'ab'[i]}" if len(parts) > 1 else "")
        ids.append(nm)
        par = "caudex" if i == 0 else ids[i - 1]
        lim = revolute(-10, 10) if i == 0 else revolute(-8, 8)
        kg = LEAF_KG[name] * (0.6 if i == 0 and len(parts) > 1 else 0.4 if len(parts) > 1 else 1)
        B.add(nm, par, T(f"Leaf {name}" + ("" if len(parts) == 1 else (", base" if i == 0 else ", arching tip")), f"Blatt {name}" + ("" if len(parts) == 1 else (", Basis" if i == 0 else ", bogige Spitze"))), p[0], p[1], ang, L, lim, mass=(round(kg, 3), "authored-estimate", "nobel-1988"))
        p = (p[0] + L * cos(radians(ang)), p[1] + L * sin(radians(ang)))
        spine.append(p)
    LEAVES[name] = {"spine": spine, "bones": ids, "parts": parts}
MASS = round(sum(b["mass"]["kg"] for b in B.list if "mass" in b), 1)
LEN = {n: sum(hypot(l["spine"][i + 1][0] - l["spine"][i][0], l["spine"][i + 1][1] - l["spine"][i][1]) for i in range(len(l["spine"]) - 1)) for n, l in LEAVES.items()}


def hw_fn(W, thick_base=0.5):
    def f(t):
        base = thick_base + (1 - thick_base) * sin(pi / 2 * min(1.0, t / 0.35))
        return 0.5 * W * base * max(0.0, (1 - t ** 4)) ** 0.6
    return f


def leaf_shapes(D, name, W, n, teeth, tone, z, prefix="leaf", spine_fn=None, overlap=0.05):
    l = LEAVES[name]
    sp = l["spine"]
    segs = [hypot(sp[i + 1][0] - sp[i][0], sp[i + 1][1] - sp[i][1]) for i in range(len(sp) - 1)]
    tot = sum(segs)
    cuts = [0.0]
    for s in segs:
        cuts.append(cuts[-1] + s / tot)
    for i, b in enumerate(l["bones"]):
        t0 = max(0, cuts[i] - (overlap if i > 0 else 0))
        t1 = min(1, cuts[i + 1] + (overlap if i < len(l["bones"]) - 1 else 0))
        pts = leaf_outline(sp, t0, t1, hw_fn(W), n, teeth)
        D.poly(f"{prefix}-{name}-{'ab'[i]}" if len(l["bones"]) > 1 else f"{prefix}-{name}", b, "body", z, True, tone, pts, closed=True, region=REGION[name])


def caudex_shape(D, z=1, hw=0.11, top=0.42):
    D.poly("caudex", "caudex", "body", z, True, 0.3, [(-hw, 0.0), (hw, 0.0), (hw * 0.9, top), (-hw * 0.9, top)], closed=True, region="caudex")


def build_50():
    D = Draw(B)
    caudex_shape(D, 1)
    order = ["R1", "L1", "R2", "L2", "R3", "L3", "R4", "L4", "R5", "L5", "S"]
    for zi, nm in enumerate(order):
        W = 0.21 if nm != "S" else 0.17
        tone = 0.45 if REGION[nm] == "leaves-spreading" else 0.35
        leaf_shapes(D, nm, W, 14, 0.07, tone, 2 + zi * 0.01)
    return D.shapes


def build_100():
    D = Draw(B)
    caudex_shape(D, 1)
    order = ["R1", "L1", "R2", "L2", "R3", "L3", "R4", "L4", "R5", "L5", "S"]
    for zi, nm in enumerate(order):
        W = 0.22 if nm != "S" else 0.18
        tone = 0.45 if REGION[nm] == "leaves-spreading" else 0.35
        leaf_shapes(D, nm, W, 8, 0.0, tone, 2 + zi * 0.01)
    return D.shapes


def build_200():
    D = Draw(B)
    caudex_shape(D, 1, hw=0.15, top=0.42)
    order = ["R1", "L1", "R2", "L2", "R3", "L3", "R4", "L4", "R5", "L5", "S"]
    for zi, nm in enumerate(order):
        W = 0.25 if nm != "S" else 0.2
        tone = 0.45 if REGION[nm] == "leaves-spreading" else 0.35
        leaf_shapes(D, nm, W, 6, 0.0, tone, 2 + zi * 0.01)
    return D.shapes


def build_500():
    D = Draw(B)
    for side, names, bone in (("left", ["L1", "L2", "L3", "L4", "L5"], "leaf-L2-a"), ("right", ["R1", "R2", "R3", "R4", "R5"], "leaf-R2-a")):
        loops = []
        for nm in names:
            l = LEAVES[nm]
            sp = l["spine"]
            loops.append(leaf_outline(sp, 0, 1, hw_fn(0.40), 8, 0.0))
        for k, lp in enumerate(union(loops, 0.012, 0.012, min_area=0.02, close=2, open_=3)):
            D.poly(f"rosette-{side}-{k}", bone, "body", 2, True, 0.4, lp, closed=True, regions=["leaves-spreading", "leaves-erect"])
    sp = LEAVES["S"]["spine"]
    for k, lp in enumerate(union([leaf_outline(sp, 0, 1, hw_fn(0.46), 8, 0.0)], 0.012, 0.012, min_area=0.02, open_=3)):
        D.poly(f"heart-{k}", "leaf-S", "body", 3, True, 0.35, lp, closed=True, regions=["leaves-erect"])
    D.poly("caudex", "caudex", "body", 1, True, 0.3, [(-0.14, 0.0), (0.14, 0.0), (0.12, 0.42), (-0.12, 0.42)], closed=True, region="caudex")
    return D.shapes


def plan_symbol():
    sh = []
    cmd = []
    for k in range(22):
        a = 2 * pi * k / 22
        r = 1.28 if k % 2 == 0 else 1.0
        cmd.append((r * cos(a), r * sin(a) * 0.95, "s"))
    sh.append({"id": "rosette-extent", "bone": "root-plate", "role": "body", "z": 1, "filled": False, "tone": 0.5, "kind": "path", "d": catmull(cmd, True)})
    sh.append({"id": "heart", "bone": "root-plate", "role": "detail", "z": 2, "filled": False, "tone": 0.3, "kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.12, "ry": 0.12})
    return sh
