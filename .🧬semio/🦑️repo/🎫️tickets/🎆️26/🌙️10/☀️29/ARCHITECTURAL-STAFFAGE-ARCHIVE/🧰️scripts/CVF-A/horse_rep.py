# [scratch CV-A] horse: hand-authored representations (world coordinates of the standing-alert default pose)
from draw import *
ASSET = load_v1("horse"); DEFAULT = "standing-alert"
W = pose_world(ASSET, DEFAULT)
H = (0, 1.312); K = (0.210, 0.948); HK = (-0.089, 0.523); HF = (-0.031, 0.188); HP = (0.043, 0.069); HT = (0.101, 0.0)
S0 = (0.92, 1.612); E0 = (1.164, 1.153); EL = (0.967, 0.876); CA = (0.983, 0.476); FF = (0.983, 0.181); FP = (1.057, 0.075); FT = (1.116, 0.0)
N1 = [(1.16, 1.442), (1.396, 1.687)]; N2 = [(1.396, 1.687), (1.528, 1.934)]; N3 = [(1.528, 1.934), (1.638, 2.124)]
TAIL = [(-0.26, 1.452), (-0.383, 1.27), (-0.486, 1.075), (-0.56, 0.847), (-0.602, 0.631)]
HIND = [(-0.30, 1.40), (-0.27, 1.50), (-0.15, 1.575), (0.05, 1.585), (0.30, 1.55), (0.58, 1.54, 'c'), (0.58, 0.83, 'c'), (0.30, 0.93), (0.10, 0.98), (-0.12, 1.08), (-0.26, 1.22)]
CHEST = [(0.45, 1.535, 'c'), (0.70, 1.55), (0.95, 1.58), (1.10, 1.59), (1.20, 1.56), (1.27, 1.40), (1.28, 1.22), (1.22, 1.0), (1.12, 0.82), (0.95, 0.76), (0.73, 0.745), (0.55, 0.80), (0.45, 0.85, 'c')]
HEADP = [(1.62, 2.16), (1.66, 2.21), (1.74, 2.20), (1.85, 2.15), (1.98, 2.09), (2.10, 2.05), (2.19, 2.04), (2.235, 2.01), (2.23, 1.97), (2.18, 1.95), (2.10, 1.96), (2.0, 1.94), (1.93, 1.90), (1.85, 1.86), (1.76, 1.87), (1.68, 1.92), (1.63, 2.00)]
JAWP = [(1.75, 2.01), (1.85, 1.98), (2.0, 1.965), (2.12, 1.955), (2.18, 1.95), (2.18, 1.925), (2.1, 1.915), (1.95, 1.89), (1.85, 1.85), (1.78, 1.87), (1.74, 1.93)]
EAR = lambda dx: [(1.655+dx, 2.20), (1.662+dx, 2.365), (1.700+dx, 2.28), (1.715+dx, 2.20)]
HHOOF = [(-0.01, 0.095), (0.05, 0.095), (0.115, 0.03), (0.125, 0.0), (0.04, 0.0), (0.02, 0.05)]
FHOOF = [(1.0, 0.095), (1.065, 0.095), (1.13, 0.03), (1.14, 0.0), (1.06, 0.0), (1.04, 0.05)]
CREST = [(1.64, 2.22), (1.52, 2.04), (1.38, 1.87), (1.22, 1.73), (1.06, 1.63)]
LR, FR = "coat-hindlimbs", "coat-forelimbs"

def legs(r, lvl):
    for s, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        n = "far" if s == "f" else "near"
        if lvl == 500:
            r.add(f"{n}-hind-upper", f"thigh-h{s}", "body", z, True, tone, d=limb(H, K, 0.50, 0.22, 0.02, 0.5), region=LR)
            r.add(f"{n}-gaskin", f"shank-h{s}", "body", z, True, tone, d=limb(K, HK, 0.24, 0.17, 0, 0.5), region=LR)
            r.add(f"{n}-hind-lower", f"cannon-h{s}", "body", z, False, tone, poly=[HK, HF, HP, (0.1, 0.004)], closed=False, regions=[LR, "hooves"], penMm=0.3)
            r.add(f"{n}-fore-upper", f"humerus-f{s}", "body", z, True, tone, d=limb(E0, EL, 0.26, 0.17, 0, 0.5), region=FR)
            r.add(f"{n}-forearm", f"radius-f{s}", "body", z, True, tone, d=limb(EL, CA, 0.16, 0.14, 0, 0.5), region=FR)
            r.add(f"{n}-fore-lower", f"cannon-f{s}", "body", z, False, tone, poly=[CA, FF, FP, (1.115, 0.004)], closed=False, regions=[FR, "hooves"], penMm=0.3)
        else:
            wid = 1.0 if lvl != 200 else 1.15
            r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(H, K, 0.52, 0.22, 0.03, 0.5), region=LR)
            r.add(f"{n}-gaskin", f"shank-h{s}", "body", z, True, tone, d=limb(K, HK, 0.24, 0.16, 0.0, 0.5), region=LR)
            r.add(f"{n}-hind-cannon", f"cannon-h{s}", "body", z, True, tone, d=limb(HK, HF, 0.13*wid, 0.09*wid, 0, 0.5), region=LR)
            r.add(f"{n}-hind-pastern", f"pastern-h{s}", "body", z, True, tone, d=limb(HF, HP, 0.085*wid, 0.075*wid, 0, 0.5), region=LR)
            r.add(f"{n}-hind-hoof", f"hoof-h{s}", "body", z, True, tone+0.05, pts=HHOOF, region="hooves")
            r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=limb(S0, E0, 0.30, 0.26, 0, 0.5), region=FR)
            r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=limb(E0, EL, 0.26, 0.17, 0.01, 0.5), region=FR)
            r.add(f"{n}-radius", f"radius-f{s}", "body", z, True, tone, d=limb(EL, CA, 0.16, 0.12, 0, 0.5), region=FR)
            r.add(f"{n}-fore-cannon", f"cannon-f{s}", "body", z, True, tone, d=limb(CA, FF, 0.105*wid, 0.085*wid, 0, 0.5), region=FR)
            r.add(f"{n}-fore-pastern", f"pastern-f{s}", "body", z, True, tone, d=limb(FF, FP, 0.085*wid, 0.075*wid, 0, 0.5), region=FR)
            r.add(f"{n}-fore-hoof", f"hoof-f{s}", "body", z, True, tone+0.05, pts=FHOOF, region="hooves")

def tail(r, lvl):
    ws = [(0.09, 0.12), (0.12, 0.16), (0.16, 0.15), (0.15, 0.11), (0.11, 0.04)]
    if lvl == 500:
        r.add("tail-upper", "tail-2", "body", 2.8, True, 0.6, d=limb(TAIL[0], TAIL[2], 0.11, 0.16, 0, 0.5), region="tail")
        r.add("tail-lower", "tail-4", "body", 2.8, True, 0.6, d=limb(TAIL[2], TAIL[4], 0.16, 0.05, 0, 0.5), region="tail")
    else:
        for i, b in enumerate(("tail-1", "tail-2", "tail-3", "tail-4")):
            r.add(f"tail-{i+1}", b, "body", 2.8, True, 0.6, d=limb(TAIL[i], TAIL[i+1], ws[i][0], ws[i][1], 0, 0.5), region="tail")

def trunk(r, lvl):
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=CHEST, region="coat-trunk")
    if lvl == 500:
        r.add("neck-a", "neck-1", "body", 3.3, True, 0.72, d=limb(N1[0], N1[1], 0.46, 0.38, 0, 0.45), region="coat-trunk")
        r.add("neck-b", "neck-2", "body", 3.3, True, 0.72, d=limb(N2[0], N3[1], 0.36, 0.22, 0, 0.45), regions=["coat-trunk", "mane"])
    else:
        r.add("neck-1", "neck-1", "body", 3.3, True, 0.72, d=limb(N1[0], N1[1], 0.46, 0.38, 0, 0.45), region="coat-trunk")
        r.add("neck-2", "neck-2", "body", 3.3, True, 0.72, d=limb(N2[0], N2[1], 0.38, 0.32, 0, 0.45), region="coat-trunk")
        r.add("neck-3", "neck-3", "body", 3.3, True, 0.72, d=limb(N3[0], N3[1], 0.32, 0.24, 0, 0.45), region="coat-trunk")

def head(r, lvl):
    if lvl == 500:
        r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, regions=["coat-head", "muzzle-skin", "eyes"])
        r.add("ears", "ear-n", "body", 4.5, True, 0.6, pts=[(1.645, 2.20), (1.662, 2.365), (1.745, 2.20)], region="ears")
        return
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAWP, region="coat-head")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, region="coat-head")
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=EAR(0.012), region="ears")
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR(0), region="ears")
    r.ellipse("muzzle", "head", "body", 4.2, True, 0.1, (2.205, 2.0), 0.035, 0.03, region="muzzle-skin")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (1.93, 2.05), 0.014 if lvl >= 100 else 0.02, 0.012 if lvl >= 100 else 0.018, region="eyes")

def mane(r, lvl):
    if lvl == 500: return
    out = [(x-0.03, y+0.06) for x, y in CREST]
    if lvl == 200:
        r.add("mane", "neck-2", "body", 3.4, True, 0.5, pts=CREST[:4] + [(x-0.0, y-0.08) for x, y in reversed(CREST[:4])], region="mane")
        return
    for i, b in enumerate(("neck-3", "neck-2", "neck-1")):
        a, c = CREST[i], CREST[i+1]; a2, c2 = out[i], out[i+1]
        r.add(f"mane-{i+1}", b, "body", 3.4, True, 0.5, pts=[a, c, (c[0]-0.02, c[1]-0.07), (a[0]-0.04, a[1]-0.07)] if False else [a2, c2, c, a], region="mane")
    r.add("forelock", "head", "body", 4.1, True, 0.5, pts=[(1.70, 2.20), (1.76, 2.19), (1.80, 2.12), (1.74, 2.10), (1.70, 2.13)], region="mane")

def rep(lvl):
    r = Rep(W); legs(r, lvl); tail(r, lvl); trunk(r, lvl); head(r, lvl); mane(r, lvl)
    if lvl != 500:
        r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.10, 1.55), (0.02, 1.40), (0.05, 1.20)], closed=False)
        r.add("shoulder-line", "thorax", "detail", 5.5, False, 0.25, poly=[(1.0, 1.55), (1.15, 1.35), (1.15, 1.15)], closed=False)
    if lvl == 50:
        r.add("nostril", "head", "detail", 5.0, False, 0.2, poly=[(2.20, 2.025), (2.21, 2.005), (2.225, 2.0)], closed=False)
        r.add("brow", "head", "detail", 5.0, False, 0.2, poly=[(1.95, 2.085), (1.93, 2.075), (1.90, 2.07)], closed=False)
        r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(2.22, 1.975), (2.12, 1.96), (2.0, 1.945), (1.93, 1.915)], closed=False)
        r.add("stifle-line", "thigh-hn", "detail", 5.5, False, 0.25, poly=[(0.25, 1.15), (0.2, 1.0), (0.16, 0.92)], closed=False)
        r.add("knee-line", "radius-fn", "detail", 5.5, False, 0.25, poly=[(0.96, 0.52), (1.0, 0.50)], closed=False)
        r.add("hock-line", "shank-hn", "detail", 5.5, False, 0.25, poly=[(-0.12, 0.57), (-0.06, 0.52)], closed=False)
        r.add("hoof-line-h", "hoof-hn", "detail", 5.5, False, 0.25, poly=[(0.01, 0.07), (0.1, 0.03)], closed=False)
        r.add("hoof-line-f", "hoof-fn", "detail", 5.5, False, 0.25, poly=[(1.02, 0.07), (1.11, 0.03)], closed=False)
        for i, (a, b) in enumerate([((1.18, 1.3), (1.10, 1.1)), ((0.80, 1.45), (0.78, 1.25)), ((0.30, 1.45), (0.28, 1.25))]):
            r.add(f"coat-line-{i+1}", "thorax" if i < 2 else "lumbar", "texture", 5.5, False, 0.35, poly=[a, b], closed=False)
        for i in range(3):
            r.add(f"tail-hair-{i+1}", "tail-3", "texture", 5.6, False, 0.35, poly=[(TAIL[1][0]-0.03+0.02*i-0.05, TAIL[2][1]+0.1), (TAIL[3][0]-0.05+0.03*i, TAIL[3][1]-0.05)], closed=False)
    return r.shapes

PENS = {500: dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
if __name__ == "__main__":
    for den in (50, 100, 200, 500):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/horse-{den}-zoom.png", 1000/den, zoom={50: 1.5, 100: 3, 200: 6, 500: 12}[den], pad_m=0.05)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
