# [scratch CVF-A] dairy-cow v2: four individually drawn scales (world coordinates of the standing-chewing default pose, metres)
# Legs sit BEHIND the trunk masses (lower z) so upper limbs read as part of the body; every outline is authored point by point.
from draw import *
ASSET = load_v1("dairy-cow"); DEFAULT = "standing-chewing"
W = pose_world(ASSET, DEFAULT)
LR, FR = "coat-hindlimbs", "coat-forelimbs"
HP = {"n": [(0, 1.104), (0.261, 0.802), (-0.035, 0.424), (-0.015, 0.135)], "f": [(0, 1.104), (0.313, 0.856), (0.105, 0.424), (0.124, 0.135)]}
FP = {"n": [(1.145, 1.382), (1.354, 0.972), (1.048, 0.783), (1.077, 0.394), (1.095, 0.134)], "f": [(1.145, 1.382), (1.354, 0.972), (1.048, 0.783), (1.017, 0.394), (1.036, 0.135)]}
TAIL = [(-0.27, 1.374), (-0.322, 1.079), (-0.306, 0.629), (-0.27, 0.332)]
HIND = [(-0.34, 1.30), (-0.31, 1.40), (-0.18, 1.45), (-0.02, 1.455), (0.12, 1.47), (0.30, 1.44), (0.72, 1.43, 'c'), (0.72, 0.62, 'c'), (0.45, 0.62), (0.28, 0.66), (0.12, 0.73), (-0.02, 0.78), (-0.16, 0.81), (-0.28, 0.87), (-0.335, 1.0)]
CHEST = [(0.65, 1.43, 'c'), (1.0, 1.45), (1.2, 1.47), (1.32, 1.46), (1.41, 1.30), (1.39, 1.06), (1.30, 0.80), (1.14, 0.64), (0.88, 0.585), (0.65, 0.62, 'c')]
NECK = [(1.15, 1.46), (1.4, 1.44), (1.65, 1.40), (1.88, 1.35), (1.95, 1.25), (1.93, 1.12), (1.78, 1.04), (1.58, 0.97), (1.42, 0.92), (1.28, 0.96)]
HEADP = [(1.88, 1.33), (1.95, 1.345), (2.02, 1.28), (2.08, 1.14), (2.15, 0.95), (2.19, 0.86), (2.18, 0.82), (2.12, 0.80), (2.07, 0.86), (2.02, 0.98), (1.95, 1.08), (1.90, 1.16), (1.86, 1.25)]
JAWP = [(1.93, 1.14), (2.0, 1.05), (2.07, 0.90), (2.10, 0.84), (2.06, 0.82), (1.99, 0.92), (1.93, 1.03)]
MUZZLE = [(2.09, 0.93), (2.17, 0.95), (2.20, 0.86), (2.185, 0.81), (2.12, 0.80), (2.08, 0.85)]
EAR = lambda dy: [(1.97, 1.30+dy), (1.90, 1.345+dy), (1.80, 1.335+dy), (1.745, 1.30+dy), (1.80, 1.265+dy), (1.90, 1.26+dy)]
UDDER = [(0.03, 0.70), (0.18, 0.70), (0.30, 0.66), (0.33, 0.58), (0.28, 0.52), (0.18, 0.50), (0.08, 0.53), (0.03, 0.60)]
PATCH_R = [(-0.25, 1.33), (-0.20, 1.38), (-0.02, 1.39), (0.14, 1.38), (0.22, 1.30), (0.22, 1.14), (0.12, 1.02), (-0.02, 1.0), (-0.18, 1.04), (-0.26, 1.15)]
PATCH_B = [(0.62, 1.37), (0.88, 1.38), (0.92, 1.25), (0.85, 1.0), (0.74, 0.86), (0.64, 0.92), (0.60, 1.15)]
PATCH_C = [(1.06, 1.38), (1.28, 1.39), (1.34, 1.27), (1.27, 1.12), (1.14, 1.09), (1.07, 1.2)]
RUMEN = [(0.31, 1.18), (0.38, 1.26), (0.48, 1.22), (0.50, 1.08), (0.44, 0.97), (0.35, 0.99), (0.30, 1.08)]
def hoof(x, top=0.14): return [(x-0.035, top), (x+0.03, top), (x+0.09, 0.04), (x+0.098, 0.0), (x+0.0, 0.0), (x-0.04, 0.06)]

def lower500(K, F):
    return [(K[0]-0.065, K[1]+0.05), (K[0]+0.065, K[1]+0.05), (F[0]+0.062, F[1]), (F[0]+0.115, 0.0), (F[0]-0.01, 0.0), (F[0]-0.062, F[1]+0.0)]

def hind_leg(r, s, z, tone, lvl):
    n = "far" if s == "f" else "near"; P = HP[s]
    if lvl == 500:
        r.add(f"{n}-gaskin", f"shank-h{s}", "body", z+0.5, True, tone, d=prof(P[1], P[2], [(0, 0.17, 0.30), (0.35, 0.13, 0.17), (0.7, 0.11, 0.12), (1.0, 0.11, 0.13)]), region=LR)
        r.add(f"{n}-hind-lower", f"metatarsus-h{s}", "body", z, True, tone, pts=lower500(P[2], P[3]), regions=["lower-legs", "hooves"])
        return
    r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=prof(P[0], P[1], [(0, 0.15, 0.15), (0.5, 0.17, 0.15), (1.0, 0.14, 0.12)]), region=LR)
    if lvl == 200:
        r.add(f"{n}-gaskin", f"shank-h{s}", "body", z, True, tone, d=prof(P[1], P[2], [(0, 0.17, 0.30), (0.35, 0.12, 0.17), (0.7, 0.075, 0.085), (1.0, 0.06, 0.09)]), region=LR)
        r.add(f"{n}-hind-cannon", f"metatarsus-h{s}", "body", z, True, tone, d=prof(P[2], P[3], [(0, 0.06, 0.06), (0.5, 0.05, 0.05), (1.0, 0.055, 0.055)]), region="lower-legs")
        r.add(f"{n}-hind-hoof", f"hoof-h{s}", "body", z, True, 0.7, pts=hoof(P[3][0]), region="hooves")
        return
    hb = 0.08 if lvl == 50 else 0.06
    r.add(f"{n}-gaskin", f"shank-h{s}", "body", z, True, tone, d=prof(P[1], P[2], [(0, 0.17, 0.30), (0.3, 0.125, 0.17), (0.6, 0.085, 0.09), (0.88, 0.065, 0.075), (1.0, 0.06, hb+0.03)]), region=LR)
    r.add(f"{n}-hind-cannon", f"metatarsus-h{s}", "body", z, True, tone, d=prof(P[2], P[3], [(0, 0.062, 0.06), (0.45, 0.05, 0.047), (0.9, 0.052, 0.05), (1.0, 0.06, 0.058)]), region="lower-legs")
    r.add(f"{n}-hind-hoof", f"hoof-h{s}", "body", z, True, 0.7, pts=hoof(P[3][0]), region="hooves")

def fore_leg(r, s, z, tone, lvl):
    n = "far" if s == "f" else "near"; P = FP[s]
    if lvl == 500:
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z+0.5, True, tone, d=prof(P[2], P[3], [(0, 0.12, 0.12), (0.4, 0.10, 0.10), (1.0, 0.10, 0.10)]), region=FR)
        r.add(f"{n}-fore-lower", f"cannon-f{s}", "body", z, True, tone, pts=lower500(P[3], P[4]), regions=["lower-legs", "hooves"])
        return
    r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=prof(P[0], P[1], [(0, 0.12, 0.12), (0.5, 0.17, 0.15), (1.0, 0.13, 0.13)]), region=FR)
    r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=prof(P[1], P[2], [(0, 0.15, 0.16), (0.5, 0.13, 0.12), (1.0, 0.10, 0.10)]), region=FR)
    if lvl == 200:
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=prof(P[2], P[3], [(0, 0.11, 0.11), (0.4, 0.075, 0.07), (1.0, 0.055, 0.055)]), region=FR)
        r.add(f"{n}-fore-cannon", f"cannon-f{s}", "body", z, True, tone, d=prof(P[3], P[4], [(0, 0.055, 0.055), (0.5, 0.045, 0.045), (1.0, 0.05, 0.05)]), region="lower-legs")
        r.add(f"{n}-fore-hoof", f"hoof-f{s}", "body", z, True, 0.7, pts=hoof(P[4][0]), region="hooves")
        return
    r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=prof(P[2], P[3], [(0, 0.115, 0.11), (0.35, 0.08, 0.075), (0.75, 0.062, 0.06), (0.96, 0.068, 0.066), (1.0, 0.062, 0.06)]), region=FR)
    r.add(f"{n}-fore-cannon", f"cannon-f{s}", "body", z, True, tone, d=prof(P[3], P[4], [(0, 0.058, 0.056), (0.45, 0.046, 0.044), (0.9, 0.05, 0.048), (1.0, 0.056, 0.054)]), region="lower-legs")
    r.add(f"{n}-fore-hoof", f"hoof-f{s}", "body", z, True, 0.7, pts=hoof(P[4][0]), region="hooves")

def tail(r, lvl):
    if lvl == 500:
        r.add("tail", "tail-2", "body", 2.8, False, 0.5, poly=[TAIL[0], TAIL[1], TAIL[2], (-0.27, 0.30)], closed=False, region="coat-tail", penMm=0.2)
        return
    if lvl == 200:
        r.add("tail", "tail-2", "body", 2.8, True, 0.3, d=prof(TAIL[0], TAIL[2], [(0, 0.045, 0.045), (0.5, 0.032, 0.032), (1.0, 0.03, 0.03)]), region="coat-tail")
        r.add("tail-switch", "tail-3", "body", 2.85, True, 0.75, pts=[(-0.27, 0.64), (-0.23, 0.56), (-0.235, 0.42), (-0.27, 0.30), (-0.31, 0.42), (-0.33, 0.56)], region="coat-tail")
        return
    ws = [(0.04, 0.04), (0.032, 0.03), (0.028, 0.026)]
    for i, b in enumerate(("tail-1", "tail-2", "tail-3")):
        A, B = TAIL[i], TAIL[i+1]
        r.add(f"tail-{i+1}", b, "body", 2.8, True, 0.3, d=prof(A, B, [(0, ws[i][0], ws[i][0]), (1.0, ws[i][1], ws[i][1])]), region="coat-tail")
    sw = [(-0.27, 0.40), (-0.225, 0.33), (-0.235, 0.22), (-0.27, 0.15), (-0.305, 0.22), (-0.315, 0.33)]
    r.add("tail-switch", "tail-3", "body", 2.85, True, 0.78, pts=sw, region="coat-tail", hatch="hair" if lvl == 50 else None)

TRUNK500 = [(-0.34, 1.30), (-0.31, 1.40), (-0.18, 1.45), (-0.02, 1.455), (0.12, 1.47), (0.30, 1.44), (0.72, 1.43), (1.0, 1.45), (1.2, 1.47), (1.32, 1.46), (1.41, 1.30), (1.39, 1.06), (1.30, 0.80), (1.14, 0.64), (0.88, 0.585), (0.65, 0.62), (0.45, 0.625), (0.37, 0.62), (0.34, 0.57), (0.30, 0.52), (0.20, 0.50), (0.10, 0.52), (0.04, 0.60), (0.02, 0.72), (-0.04, 0.78), (-0.16, 0.81), (-0.28, 0.87), (-0.335, 1.0)]

def rep(lvl):
    r = Rep(W)
    for s, z, tone in (("f", 1.0, 0.25), ("n", 2.0, 0.3)):
        hind_leg(r, s, z, tone, lvl); fore_leg(r, s, z, tone, lvl)
    tail(r, lvl)
    if lvl != 500: r.add("udder", "udder", "body", 1.5, True, 0.5, pts=UDDER, region="udder")
    if lvl in (100, 50):
        for nm, x in (("teat-front", 0.26), ("teat-rear", 0.10)):
            r.add(nm, "udder", "body", 1.55, True, 0.5, d=capsule_world((x, 0.52), (x, 0.41), 0.017, 0.013), region="udder")
    if lvl == 500:
        r.add("trunk", "lumbar", "body", 3.0, True, 0.2, pts=TRUNK500, regions=["coat-trunk", "udder"])
        r.add("patches", "lumbar", "detail", 3.4, True, 0.8, pts=PATCH_B, regions=["coat-patches", "flank-rumen"])
        r.add("neck", "neck", "body", 3.3, True, 0.65, pts=NECK, region="coat-head-neck")
        r.add("head", "head", "body", 4.0, True, 0.7, pts=HEADP, regions=["coat-head-neck", "ears", "muzzle", "eyes"])
        return r.shapes
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.2, pts=HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.2, pts=CHEST, region="coat-trunk")
    r.add("patch-rump", "lumbar", "detail", 3.4, True, 0.8, pts=PATCH_R, region="coat-patches")
    r.add("patch-barrel", "thorax", "detail", 3.4, True, 0.8, pts=PATCH_B, region="coat-patches")
    r.add("patch-chest", "thorax", "detail", 3.4, True, 0.8, pts=PATCH_C, region="coat-patches")
    r.add("rumen-flank", "lumbar", "detail", 3.5, True, 0.5, pts=RUMEN, region="flank-rumen")
    r.add("neck", "neck", "body", 3.3, True, 0.65, pts=NECK, region="coat-head-neck")
    r.add("jaw", "jaw", "body", 3.9, True, 0.7, pts=JAWP, region="coat-head-neck")
    r.add("head", "head", "body", 4.0, True, 0.75, pts=HEADP, region="coat-head-neck")
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.7, pts=EAR(0.01), region="ears")
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.8, pts=EAR(0), region="ears")
    r.add("muzzle", "head", "body", 4.2, True, 0.15, pts=MUZZLE, region="muzzle")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (2.0, 1.17), 0.012 if lvl in (100, 50) else 0.02, 0.01 if lvl in (100, 50) else 0.02, region="eyes")
    return r.shapes

PENS = {500: dict(body=0.13, detail=0.1, texture=None, glazing=None, ground=0.18), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
if __name__ == "__main__":
    for den in (50, 100, 200, 500):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/cow-{den}-zoom.png", 1000/den, zoom={50: 1.5, 100: 3, 200: 6, 500: 12}[den], pad_m=0.05)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
