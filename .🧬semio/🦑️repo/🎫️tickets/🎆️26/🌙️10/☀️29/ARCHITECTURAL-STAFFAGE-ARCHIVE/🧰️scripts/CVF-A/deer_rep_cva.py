# [scratch CV-A] roe-deer: hand-authored representations (world coordinates of the alert default pose)
from draw import *
ASSET = load_v1("roe-deer"); DEFAULT = "alert"
W = pose_world(ASSET, DEFAULT)
HIND = [(-0.145, 0.64), (-0.12, 0.73), (-0.06, 0.78), (0.10, 0.78), (0.30, 0.775), (0.50, 0.77, 'c'), (0.50, 0.52, 'c'), (0.30, 0.53), (0.12, 0.54), (-0.02, 0.55), (-0.10, 0.57)]
CHEST = [(0.45, 0.77, 'c'), (0.60, 0.775), (0.75, 0.78), (0.85, 0.80), (0.90, 0.72), (0.90, 0.62), (0.86, 0.52), (0.78, 0.46), (0.66, 0.44), (0.55, 0.47), (0.45, 0.52, 'c')]
HEADP = [(1.07, 0.92), (1.09, 0.965), (1.14, 0.975), (1.19, 0.94), (1.25, 0.88), (1.30, 0.83), (1.31, 0.815), (1.29, 0.80), (1.24, 0.805), (1.18, 0.83), (1.12, 0.84), (1.08, 0.86)]
JAWP = [(1.13, 0.86), (1.2, 0.835), (1.25, 0.81), (1.245, 0.795), (1.2, 0.79), (1.14, 0.82)]
EAR = lambda d: [(1.13+d, 0.955), (1.16+d, 1.03), (1.24+d, 1.075), (1.255+d, 1.06), (1.21+d, 0.99), (1.17+d, 0.945)]
HN = {"n": [(0, 0.68), (0.106, 0.499), (-0.048, 0.302), (-0.028, 0.073)], "f": [(0, 0.68), (0.074, 0.483), (-0.099, 0.302), (-0.079, 0.073)]}
FN = {"n": [(0.778, 0.591), (0.670, 0.472), (0.685, 0.263), (0.692, 0.073)], "f": [(0.778, 0.591), (0.676, 0.467), (0.725, 0.263), (0.732, 0.073)]}
S0 = (0.68, 0.73)
def hoof(x): return [(x-0.02, 0.078), (x+0.02, 0.078), (x+0.055, 0.02), (x+0.06, 0.0), (x-0.015, 0.0), (x-0.025, 0.04)]
LR, FR = "coat-hindlimbs", "coat-forelimbs"

def legs(r, lvl):
    for s, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        n = "far" if s == "f" else "near"; P = HN[s]; F = FN[s]
        if lvl == 500:
            r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(P[0], P[1], 0.22, 0.10, 0.01, 0.5), region=LR)
            r.add(f"{n}-hind-leg", f"shank-h{s}", "body", z, False, tone, poly=[P[1], P[2], P[3], (P[3][0]+0.05, 0.004)], closed=False, regions=[LR, "hooves"], penMm=0.2)
            r.add(f"{n}-fore-upper", f"humerus-f{s}", "body", z, True, tone, d=limb(F[0], F[1], 0.17, 0.09, 0, 0.5), region=FR)
            r.add(f"{n}-fore-leg", f"forearm-f{s}", "body", z, False, tone, poly=[F[1], F[2], F[3], (F[3][0]+0.05, 0.004)], closed=False, regions=[FR, "hooves"], penMm=0.2)
            continue
        r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(P[0], P[1], 0.22, 0.10, 0.015, 0.5), region=LR)
        r.add(f"{n}-shank", f"shank-h{s}", "body", z, True, tone, d=limb(P[1], P[2], 0.11, 0.055, 0.0, 0.35), region=LR)
        r.add(f"{n}-hind-cannon", f"cannon-h{s}", "body", z, True, tone, d=limb(P[2], P[3], 0.05*(1.2 if lvl == 200 else 1), 0.04*(1.2 if lvl == 200 else 1), 0, 0.5), region=LR)
        r.add(f"{n}-hind-hoof", f"hoof-h{s}", "body", z, True, tone+0.08, pts=hoof(P[3][0]), region="hooves")
        r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=limb(S0, F[0], 0.18, 0.16, 0, 0.5), region=FR)
        r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=limb(F[0], F[1], 0.16, 0.085, 0.01, 0.5), region=FR)
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=limb(F[1], F[2], 0.085, 0.045, 0, 0.5), region=FR)
        r.add(f"{n}-fore-cannon", f"cannon-f{s}", "body", z, True, tone, d=limb(F[2], F[3], 0.045*(1.2 if lvl == 200 else 1), 0.038*(1.2 if lvl == 200 else 1), 0, 0.5), region=FR)
        r.add(f"{n}-fore-hoof", f"hoof-f{s}", "body", z, True, tone+0.08, pts=hoof(F[3][0]), region="hooves")

def rep(lvl):
    r = Rep(W); legs(r, lvl)
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=HIND, regions=["coat-trunk", "rump-patch"] if lvl == 500 else None, region=None if lvl == 500 else "coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=CHEST, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, d=limb((0.80, 0.74), (1.098, 0.91), 0.30, 0.16, 0, 0.5), region="coat-trunk")
    if lvl == 500:
        r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, regions=["coat-head", "nose-skin", "eyes"])
        r.add("ears", "ear-n", "body", 4.5, True, 0.6, pts=EAR(0), region="ears")
        return r.shapes
    r.add("rump-patch", "lumbar", "body", 3.1, True, 0.12, pts=[(-0.13, 0.64), (-0.115, 0.70), (-0.08, 0.72), (-0.06, 0.66), (-0.08, 0.60), (-0.115, 0.60)], region="rump-patch")
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAWP, region="coat-head")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, **({"regions": ["coat-head", "nose-skin", "eyes"]} if lvl == 200 else {"region": "coat-head"}))
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=EAR(0.012), region="ears")
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR(0), region="ears")
    if lvl != 200:
        r.ellipse("nose-leather", "head", "body", 4.2, True, 0.08, (1.303, 0.815), 0.012, 0.011, region="nose-skin")
        r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (1.211, 0.904), 0.012 if lvl == 100 else 0.008, 0.012 if lvl == 100 else 0.007, region="eyes")
    if lvl >= 100:
        r.add("shoulder-line", "thorax", "detail", 5.5, False, 0.25, poly=[(0.72, 0.74), (0.78, 0.62), (0.76, 0.52)], closed=False)
        r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.08, 0.76), (0.04, 0.66), (0.06, 0.56)], closed=False)
    if lvl == 50:
        r.add("brow", "head", "detail", 5.0, False, 0.2, poly=[(1.225, 0.915), (1.21, 0.915), (1.195, 0.908)], closed=False)
        r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(1.305, 0.808), (1.25, 0.80), (1.2, 0.815)], closed=False)
        r.add("inner-ear", "ear-n", "detail", 5.0, False, 0.2, poly=[(1.17, 0.975), (1.21, 1.03), (1.24, 1.055)], closed=False)
        r.add("hoof-cleft-h", "hoof-hn", "detail", 5.5, False, 0.25, poly=[(-0.01, 0.07), (0.02, 0.01)], closed=False)
        r.add("hoof-cleft-f", "hoof-fn", "detail", 5.5, False, 0.25, poly=[(0.71, 0.07), (0.74, 0.01)], closed=False)
        r.add("coat-1", "thorax", "texture", 5.5, False, 0.35, poly=[(0.55, 0.70), (0.62, 0.69), (0.70, 0.70)], closed=False)
        r.add("coat-2", "lumbar", "texture", 5.5, False, 0.35, poly=[(0.15, 0.70), (0.25, 0.69), (0.35, 0.70)], closed=False)
        r.add("neck-fur", "neck", "texture", 5.5, False, 0.35, poly=[(0.90, 0.82), (0.98, 0.86), (1.04, 0.90)], closed=False)
    return r.shapes

PENS = {500: dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
if __name__ == "__main__":
    for den in (50, 100, 200, 500):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/deer-{den}-zoom.png", 1000/den, zoom={50: 2.5, 100: 5, 200: 10, 500: 20}[den], pad_m=0.03)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
