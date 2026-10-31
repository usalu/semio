"""ivy-on-trellis: Hedera helix climbing a timber trellis in steel post shoes."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from common import *
from shapes import *
from union import union

ID = "ivy-on-trellis"
PX = 0.9
STEM_X = [-0.62, -0.20, 0.22, 0.64]
STEM_H = [2.0, 2.05, 2.05, 1.95]
LO_FROM = [0.92, 0.82, 0.80, 0.86]
SPLIT = 1.30
B = Bones()
B.add("root-plate", None, T("Footing line", "Fundamentlinie"), 0, 0, 0, 2.0, FIXED)
sec_post = {"areaM2": 0.0081, "secondMomentM4": 5.468e-06, "extremeFibreM": 0.045, "material": "timber-c24", "title": T("Timber post 90 x 90 mm", "Holzpfosten 90 x 90 mm")}
for s, x in (("L", -PX), ("R", PX)):
    B.add(f"post-{s}", "root-plate", T(f"Post {'left' if s == 'L' else 'right'} with steel shoe", f"Pfosten {'links' if s == 'L' else 'rechts'} mit Stahlschuh"), x, 0, 90, 2.1, FIXED, mass=(9.6, "authored-estimate", "en-338"), section=sec_post)
B.add("panel", "root-plate", T("Lattice panel with rails", "Gitterfeld mit Riegeln"), -PX, 0.35, 0, 2 * PX, FIXED, mass=(18.0, "authored-estimate", "en-338"))
for k, (x, h) in enumerate(zip(STEM_X, STEM_H), 1):
    B.add(f"stem-{k}", "panel", T(f"Ivy main stem {k}", f"Efeu-Hauptspross {k}"), x, 0.0, 90, h, FIXED, mass=(1.2, "authored-estimate", "metcalfe-2005"))
for k, x in enumerate(STEM_X, 1):
    B.add(f"fol-{k}-lo", f"stem-{k}", T(f"Leaf mat {k}, lower", f"Blattteppich {k}, unten"), x, LO_FROM[k - 1], 90, SPLIT - LO_FROM[k - 1], revolute(-8, 8), mass=(0.7, "authored-estimate", "metcalfe-2005"))
    B.add(f"fol-{k}-hi", f"stem-{k}", T(f"Leaf mat {k}, upper", f"Blattteppich {k}, oben"), x, SPLIT, 90, 0.78, revolute(-10, 10), mass=(0.7, "authored-estimate", "metcalfe-2005"))
B.add("crown-L", "stem-1", T("Overhanging shoots, left", "Überhängende Triebe, links"), -0.62, 1.95, 112, 0.30, revolute(-25, 25), mass=(0.4, "authored-estimate", "metcalfe-2005"))
B.add("crown-R", "stem-4", T("Overhanging shoots, right", "Überhängende Triebe, rechts"), 0.64, 1.90, 72, 0.30, revolute(-25, 25), mass=(0.4, "authored-estimate", "metcalfe-2005"))
MASS = round(sum(b["mass"]["kg"] for b in B.list if "mass" in b), 1)
FOLS = [f"fol-{k}-{z}" for k in range(1, 5) for z in ("lo", "hi")]


def zone(k, z):
    x = STEM_X[k - 1]
    if z == "lo":
        return (x - 0.30, x + 0.30, LO_FROM[k - 1], SPLIT + 0.08)
    return (x - 0.30, x + 0.30, SPLIT - 0.08, STEM_H[k - 1] - 0.12)


def hashf(i, j, s):
    return (sin(i * 12.9898 + j * 78.233 + s * 37.719) * 43758.5453) % 1.0


def leaf_field(k, z, R, sp, jitter=0.35):
    x0, x1, y0, y1 = zone(k, z)
    xl = max(x0, -PX + R * 0.9)
    xr = min(x1, PX - R * 0.9)
    loops = []
    nx = int((xr - xl) / sp) + 1
    ny = int((y1 - y0 - 0.08) / sp) + 1
    for i in range(nx + 1):
        for j in range(ny + 1):
            cx = xl + i * sp + (hashf(i, j, k + (z == "hi") * 7) - 0.5) * sp * jitter * 2
            cy = y0 + j * sp + (hashf(j, i, k + 3 + (z == "hi") * 7) - 0.5) * sp * jitter * 2
            if (i in (0, nx) or j in (0, ny)) and hashf(i, j, 99) < 0.45:
                continue
            rot = 360 * hashf(i + 5, j + 9, k)
            loops.append(ragged(cx, cy, R, R * 0.92, rot, 5, 0.5, 0, 0.0))
    return loops


def post_shapes(D, sc):
    for s, x in (("L", -PX), ("R", PX)):
        D.poly(f"post-{s}", f"post-{s}", "body", 2, True, 0.35, [(x - 0.045, 0.0), (x + 0.045, 0.0), (x + 0.045, 2.1), (x - 0.045, 2.1)], closed=True, region="trellis-timber")
        D.poly(f"shoe-{s}", f"post-{s}", "body", 4, True, 0.15, [(x - 0.055, 0.0), (x + 0.055, 0.0), (x + 0.055, 0.28), (x - 0.055, 0.28)], closed=True, region="post-shoes")


def rails(D, h, tone=0.35):
    D.poly("rail-top", "panel", "body", 3, True, tone, [(-PX, 1.93), (PX, 1.93), (PX, 2.0), (-PX, 2.0)], closed=True, region="trellis-timber")
    D.poly("rail-bottom", "panel", "body", 3, True, tone, [(-PX, 0.35), (PX, 0.35), (PX, 0.42), (-PX, 0.42)], closed=True, region="trellis-timber")


def build_50():
    D = Draw(B)
    post_shapes(D, 50)
    rails(D, 0.07)
    for i, y in enumerate([0.52, 0.66]):
        D.poly(f"batten-h-{i}", "panel", "body", 2, True, 0.4, [(-PX, y - 0.01), (PX, y - 0.01), (PX, y + 0.01), (-PX, y + 0.01)], closed=True, region="trellis-timber")
    for i, x in enumerate([-0.7, -0.5, -0.3, -0.1, 0.1, 0.3, 0.5, 0.7]):
        D.poly(f"batten-v-{i}", "panel", "body", 2.5, True, 0.4, [(x - 0.01, 0.42), (x + 0.01, 0.42), (x + 0.01, 0.74), (x - 0.01, 0.74)], closed=True, region="trellis-timber")
    for k in range(1, 5):
        x = STEM_X[k - 1]
        D.poly(f"stem-{k}", f"stem-{k}", "detail", 5, False, 0.5, [(x, 0.05), (x + 0.012, 0.5), (x - 0.01, 1.0), (x + 0.008, 1.5), (x, STEM_H[k - 1] - 0.1)], closed=False, region="stems")
        for z, tone, reg in (("lo", 0.65, "leaves-lower"), ("hi", 0.5, "leaves-upper")):
            for n, lp in enumerate(union(leaf_field(k, z, 0.065, 0.07), 0.004, 0.003, min_area=2e-3, close=3)):
                D.poly(f"mat-{k}-{z}-{n}", f"fol-{k}-{z}", "body", 6 + (z == "hi") * 0.1, True, tone, lp, closed=True, region=reg)
    return D.shapes


def build_100():
    D = Draw(B)
    post_shapes(D, 100)
    rails(D, 0.07)
    D.poly("batten-h-0", "panel", "detail", 2, False, 0.4, [(-PX, 0.60), (PX, 0.60)], closed=False, region="trellis-timber")
    for i, x in enumerate([-0.6, -0.3, 0.0, 0.3, 0.6]):
        D.poly(f"batten-v-{i}", "panel", "detail", 2.5, False, 0.4, [(x, 0.42), (x, 0.74)], closed=False, region="trellis-timber")
    for k in range(1, 5):
        for z, tone, reg in (("lo", 0.65, "leaves-lower"), ("hi", 0.5, "leaves-upper")):
            for n, lp in enumerate(union(leaf_field(k, z, 0.085, 0.10), 0.006, 0.005, min_area=4e-3, close=3)):
                D.poly(f"mat-{k}-{z}-{n}", f"fol-{k}-{z}", "body", 6 + (z == "hi") * 0.1, True, tone, lp, closed=True, region=reg)
    return D.shapes


def build_200():
    D = Draw(B)
    post_shapes(D, 200)
    rails(D, 0.07)
    for k in range(1, 5):
        for z, tone, reg in (("lo", 0.65, "leaves-lower"), ("hi", 0.5, "leaves-upper")):
            x0, x1, y0, y1 = zone(k, z)
            xl, xr = max(x0, -PX + 0.05), min(x1, PX - 0.05)
            cx, cy = (xl + xr) / 2, (y0 + y1) / 2
            D.smooth(f"mat-{k}-{z}", f"fol-{k}-{z}", "body", 6 + (z == "hi") * 0.1, True, tone, blob(cx, cy, (xr - xl) / 2 + 0.02, (y1 - y0) / 2 + 0.03, 0, 9, 0.06, 20 * k), region=reg)
    return D.shapes


def build_500():
    D = Draw(B)
    D.poly("screen", "panel", "body", 2, True, 0.5, [(-0.945, 0.0), (0.945, 0.0), (0.945, 2.05), (-0.945, 2.05)], closed=True, regions=["trellis-timber", "post-shoes", "leaves-lower", "leaves-upper"])
    D.smooth("crown-L", "crown-L", "body", 3, True, 0.5, blob(-0.68, 2.03, 0.26, 0.085, 8, 5, 0.1, 0), regions=["leaves-upper"])
    D.smooth("crown-R", "crown-R", "body", 3, True, 0.5, blob(0.68, 2.03, 0.26, 0.085, -8, 5, 0.1, 40), regions=["leaves-upper"])
    return D.shapes


def plan_symbol():
    return [{"id": "footprint", "bone": "root-plate", "role": "body", "z": 1, "filled": False, "tone": 0.5, "kind": "path", "d": "M-1 -0.125L1 -0.125L1 0.125L-1 0.125Z"},
            {"id": "post-L", "bone": "root-plate", "role": "detail", "z": 2, "filled": False, "tone": 0.3, "kind": "path", "d": "M-0.945 -0.045L-0.855 -0.045L-0.855 0.045L-0.945 0.045Z"},
            {"id": "post-R", "bone": "root-plate", "role": "detail", "z": 2, "filled": False, "tone": 0.3, "kind": "path", "d": "M0.855 -0.045L0.945 -0.045L0.945 0.045L0.855 0.045Z"}]
