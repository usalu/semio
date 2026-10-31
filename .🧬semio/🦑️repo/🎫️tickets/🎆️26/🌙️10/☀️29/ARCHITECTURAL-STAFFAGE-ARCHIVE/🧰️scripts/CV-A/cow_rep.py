# [scratch CV-A] dairy-cow: hand-authored representations (world coordinates of the standing-chewing default pose)
from draw import *
ASSET = load_v1("dairy-cow"); DEFAULT = "standing-chewing"
W = pose_world(ASSET, DEFAULT)
HIND = [(-0.34, 1.30), (-0.30, 1.42), (-0.10, 1.46), (0.30, 1.43), (0.72, 1.43, 'c'), (0.72, 0.62, 'c'), (0.45, 0.62), (0.20, 0.68), (0.0, 0.80), (-0.25, 0.86), (-0.34, 1.0)]
CHEST = [(0.65, 1.43, 'c'), (1.0, 1.45), (1.2, 1.47), (1.32, 1.46), (1.40, 1.28), (1.38, 1.0), (1.28, 0.74), (1.10, 0.62), (0.85, 0.58), (0.65, 0.62, 'c')]
HEADP = [(1.88, 1.33), (1.95, 1.345), (2.02, 1.28), (2.08, 1.14), (2.15, 0.95), (2.19, 0.86), (2.18, 0.82), (2.12, 0.80), (2.07, 0.86), (2.02, 0.98), (1.95, 1.08), (1.90, 1.16), (1.86, 1.25)]
JAWP = [(1.93, 1.14), (2.0, 1.05), (2.07, 0.90), (2.10, 0.84), (2.06, 0.82), (1.99, 0.92), (1.93, 1.03)]
EAR = lambda dy: [(1.97, 1.30+dy), (1.90, 1.345+dy), (1.80, 1.335+dy), (1.745, 1.30+dy), (1.80, 1.265+dy), (1.90, 1.26+dy)]
HN = {"n": [(0, 1.104), (0.261, 0.802), (-0.035, 0.424), (-0.015, 0.135)], "f": [(0, 1.104), (0.313, 0.856), (0.105, 0.424), (0.124, 0.135)]}
FN = {"n": [(1.354, 0.972), (1.048, 0.783), (1.077, 0.394), (1.095, 0.134)], "f": [(1.354, 0.972), (1.048, 0.783), (1.017, 0.394), (1.036, 0.135)]}
S0 = (1.145, 1.382)
NECK = [(1.15, 1.46), (1.4, 1.44), (1.65, 1.40), (1.88, 1.35), (1.95, 1.25), (1.93, 1.12), (1.78, 1.06), (1.58, 1.0), (1.40, 0.94), (1.25, 0.98)]
TAIL = [(-0.27, 1.374), (-0.322, 1.079), (-0.306, 0.629), (-0.27, 0.332)]
UDDER = [(0.03, 0.70), (0.18, 0.70), (0.30, 0.66), (0.33, 0.58), (0.28, 0.52), (0.18, 0.50), (0.08, 0.53), (0.03, 0.60)]
def hoof(x): return [(x-0.035, 0.14), (x+0.03, 0.14), (x+0.09, 0.04), (x+0.098, 0.0), (x+0.0, 0.0), (x-0.04, 0.06)]
LR, FR = "coat-hindlimbs", "coat-forelimbs"
PATCH_R = [(-0.34, 1.34), (-0.28, 1.43), (0.0, 1.46), (0.28, 1.42), (0.30, 1.16), (0.2, 0.98), (0.0, 0.95), (-0.2, 0.98), (-0.33, 1.1)]
PATCH_B = [(0.5, 1.43), (0.9, 1.44), (0.93, 1.3), (0.85, 1.02), (0.7, 0.86), (0.56, 0.95), (0.5, 1.2)]
PATCH_C = [(1.0, 1.45), (1.3, 1.47), (1.38, 1.3), (1.3, 1.12), (1.15, 1.08), (1.02, 1.2)]
RUMEN = [(0.30, 1.2), (0.42, 1.28), (0.58, 1.22), (0.60, 1.05), (0.50, 0.94), (0.36, 0.98), (0.28, 1.08)]

def legs(r, lvl):
    for s, z, tone in (("f", 1.0, 0.2), ("n", 5.0, 0.28)):
        n = "far" if s == "f" else "near"; P = HN[s]; F = FN[s]
        if lvl == 500:
            r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(P[0], P[1], 0.46, 0.20, 0.02, 0.5), region=LR)
            r.add(f"{n}-hind-leg", f"shank-h{s}", "body", z, False, tone, poly=[P[1], P[2], P[3], (P[3][0]+0.07, 0.004)], closed=False, regions=[LR, "lower-legs", "hooves"], penMm=0.25)
            r.add(f"{n}-fore-upper", f"humerus-f{s}", "body", z, True, tone, d=limb(F[0], F[1], 0.30, 0.17, 0, 0.5), region=FR)
            r.add(f"{n}-fore-leg", f"forearm-f{s}", "body", z, False, tone, poly=[F[1], F[2], F[3], (F[3][0]+0.07, 0.004)], closed=False, regions=[FR, "lower-legs", "hooves"], penMm=0.25)
            continue
        r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(P[0], P[1], 0.46, 0.20, 0.03, 0.5), region=LR)
        r.add(f"{n}-gaskin", f"shank-h{s}", "body", z, True, tone, d=limb(P[1], P[2], 0.20, 0.10, 0.0, 0.4), region=LR)
        r.add(f"{n}-hind-cannon", f"metatarsus-h{s}", "body", z, True, tone, d=limb(P[2], P[3], 0.10, 0.08, 0, 0.5), region="lower-legs")
        r.add(f"{n}-hind-hoof", f"hoof-h{s}", "body", z, True, 0.7, pts=hoof(P[3][0]), region="hooves")
        r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=limb(S0, F[0], 0.32, 0.28, 0, 0.5), region=FR)
        r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=limb(F[0], F[1], 0.28, 0.17, 0.01, 0.5), region=FR)
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=limb(F[1], F[2], 0.17, 0.11, 0, 0.5), region=FR)
        r.add(f"{n}-fore-cannon", f"cannon-f{s}", "body", z, True, tone, d=limb(F[2], F[3], 0.10, 0.08, 0, 0.5), region="lower-legs")
        r.add(f"{n}-fore-hoof", f"hoof-f{s}", "body", z, True, 0.7, pts=hoof(F[3][0]), region="hooves")

def rep(lvl):
    r = Rep(W); legs(r, lvl)
    if lvl == 500:
        r.add("tail", "tail-2", "body", 2.8, False, 0.5, poly=[TAIL[0], TAIL[1], TAIL[2], (-0.27, 0.30)], closed=False, region="coat-tail", penMm=0.2)
        r.add("udder", "udder", "body", 2.9, True, 0.5, pts=UDDER, regions=["udder"])
        r.add("hindbody", "lumbar", "body", 3.0, True, 0.4, pts=HIND, regions=["coat-trunk", "coat-patches", "flank-rumen"])
        r.add("chest", "thorax", "body", 3.2, True, 0.4, pts=CHEST, regions=["coat-trunk", "coat-patches", "flank-rumen"])
        r.add("neck", "neck", "body", 3.3, True, 0.4, pts=NECK, region="coat-head-neck")
        r.add("head", "head", "body", 4.0, True, 0.6, pts=HEADP, regions=["coat-head-neck", "ears", "muzzle", "eyes"])
        return r.shapes
    ts = [(-0.27, 1.374), (-0.322, 1.079), (-0.306, 0.629), (-0.27, 0.332)]
    for i, (b, wd) in enumerate((("tail-1", (0.07, 0.055)), ("tail-2", (0.055, 0.05)), ("tail-3", (0.05, 0.04)))):
        r.add(f"tail-{i+1}", b, "body", 2.8, True, 0.25, d=limb(ts[i], ts[i+1], wd[0], wd[1], 0, 0.5), region="coat-tail")
    r.add("tail-switch", "tail-3", "body", 2.85, True, 0.8, pts=[(-0.27, 0.40), (-0.225, 0.33), (-0.235, 0.22), (-0.27, 0.17), (-0.305, 0.22), (-0.31, 0.33)], region="coat-tail")
    r.add("udder", "udder", "body", 2.9, True, 0.45, pts=UDDER, region="udder")
    if lvl >= 100:
        r.add("teat-front", "udder", "body", 2.95, True, 0.45, d=capsule_world((0.26, 0.52), (0.26, 0.41), 0.017, 0.013), region="udder")
        r.add("teat-rear", "udder", "body", 2.95, True, 0.45, d=capsule_world((0.10, 0.52), (0.10, 0.41), 0.017, 0.013), region="udder")
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.2, pts=HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.2, pts=CHEST, region="coat-trunk")
    r.add("patch-rump", "lumbar", "body", 3.4, True, 0.8, pts=PATCH_R, region="coat-patches")
    r.add("patch-barrel", "thorax", "body", 3.4, True, 0.8, pts=PATCH_B, region="coat-patches")
    r.add("patch-chest", "thorax", "body", 3.4, True, 0.8, pts=PATCH_C, region="coat-patches")
    if lvl >= 100: r.add("rumen-flank", "lumbar", "body", 3.5, True, 0.5, pts=RUMEN, region="flank-rumen")
    else: r.add("rumen-flank", "lumbar", "body", 3.5, True, 0.5, pts=RUMEN, region="flank-rumen")
    r.add("neck", "neck", "body", 3.3, True, 0.7, pts=NECK, region="coat-head-neck")
    r.add("jaw", "jaw", "body", 3.9, True, 0.7, pts=JAWP, region="coat-head-neck")
    r.add("head", "head", "body", 4.0, True, 0.8, pts=HEADP, region="coat-head-neck")
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.7, pts=EAR(0.01), region="ears")
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.8, pts=EAR(0), region="ears")
    r.add("muzzle", "head", "body", 4.2, True, 0.15, pts=[(2.09, 0.93), (2.17, 0.95), (2.20, 0.86), (2.185, 0.81), (2.12, 0.80), (2.08, 0.85)], region="muzzle")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (2.0, 1.17), 0.012 if lvl >= 100 else 0.02, 0.01 if lvl >= 100 else 0.02, region="eyes")
    if lvl >= 100:
        r.add("blaze", "head", "body", 4.1, True, 0.1, pts=[(1.98, 1.30), (2.03, 1.27), (2.12, 1.0), (2.14, 0.93), (2.09, 0.95), (2.0, 1.15)], region="coat-head-neck")
        r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.08, 1.4), (0.02, 1.2), (0.06, 0.95)], closed=False)
    if lvl == 50:
        r.add("nostril", "head", "detail", 5.0, False, 0.2, poly=[(2.17, 0.87), (2.18, 0.85)], closed=False)
        r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(2.18, 0.82), (2.12, 0.82)], closed=False)
        r.add("hoof-cleft-h", "hoof-hn", "detail", 5.5, False, 0.25, poly=[(0.04, 0.12), (0.06, 0.02)], closed=False)
        r.add("hoof-cleft-f", "hoof-fn", "detail", 5.5, False, 0.25, poly=[(1.14, 0.12), (1.16, 0.02)], closed=False)
        r.add("knee-line", "forearm-fn", "detail", 5.5, False, 0.25, poly=[(1.04, 0.43), (1.11, 0.43)], closed=False)
        r.add("hair-1", "thorax", "texture", 5.6, False, 0.35, poly=[(0.9, 1.3), (1.0, 1.28), (1.1, 1.3)], closed=False)
        r.add("hair-2", "lumbar", "texture", 5.6, False, 0.35, poly=[(0.1, 1.3), (0.2, 1.28), (0.3, 1.3)], closed=False)
        r.add("neck-fold", "neck", "texture", 5.6, False, 0.35, poly=[(1.5, 1.1), (1.62, 1.14), (1.74, 1.16)], closed=False)
    return r.shapes

PENS = {500: dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
if __name__ == "__main__":
    for den in (50, 100, 200, 500):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/cow-{den}-zoom.png", 1000/den, zoom={50: 1.5, 100: 3, 200: 6, 500: 12}[den], pad_m=0.05)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
