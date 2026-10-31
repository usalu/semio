# [scratch CVF-A] labrador-dog v2: four individually drawn scales (world coordinates of the standing-alert default pose, metres)
# Limbs sit BEHIND the trunk masses (lower z) so the upper limb reads as part of the body and only the free part of the leg shows.
from draw import *
ASSET = load_v1("labrador-dog"); DEFAULT = "standing-alert"
W = pose_world(ASSET, DEFAULT)
H = (0, 0.513); K = (0.010, 0.329); HK = (-0.055, 0.166); HF = (-0.043, 0.032)
SH = (0.460, 0.573); EL0 = (0.535, 0.432); EL = (0.449, 0.285); CA = (0.503, 0.093); FF = (0.508, 0.033)
HEAD_PT = [(0.70, 0.64), (0.72, 0.69), (0.775, 0.715), (0.83, 0.708), (0.875, 0.69), (0.93, 0.678), (0.972, 0.672), (0.985, 0.655), (0.972, 0.635), (0.93, 0.628), (0.86, 0.62), (0.79, 0.612), (0.74, 0.60), (0.705, 0.605)]
HIND_PAW = [(-0.065, 0.035), (-0.02, 0.046), (0.03, 0.036), (0.056, 0.018), (0.054, 0.004), (0.03, 0.0), (-0.04, 0.0), (-0.062, 0.014)]
FORE_PAW = [(0.482, 0.048), (0.53, 0.052), (0.575, 0.037), (0.601, 0.018), (0.597, 0.004), (0.572, 0.0), (0.49, 0.0), (0.481, 0.022)]
TRUNK_HIND = [(-0.085, 0.575), (-0.03, 0.593), (0.10, 0.590), (0.31, 0.594, 'c'), (0.40, 0.45), (0.31, 0.386, 'c'), (0.20, 0.376), (0.08, 0.386), (-0.03, 0.41), (-0.10, 0.452), (-0.125, 0.515)]
TRUNK_FORE = [(0.28, 0.594, 'c'), (0.40, 0.602), (0.50, 0.597), (0.58, 0.588), (0.625, 0.50), (0.617, 0.40), (0.585, 0.31), (0.52, 0.283), (0.44, 0.282), (0.36, 0.30), (0.30, 0.34), (0.27, 0.38), (0.265, 0.5, 'c')]
NECK = [(0.50, 0.597), (0.58, 0.625), (0.66, 0.668), (0.735, 0.70), (0.765, 0.62), (0.74, 0.55), (0.68, 0.485), (0.60, 0.455), (0.55, 0.47)]
EAR = [(0.775, 0.70), (0.80, 0.716), (0.84, 0.692), (0.862, 0.64), (0.852, 0.595), (0.81, 0.60), (0.78, 0.65)]
JAW = [(0.775, 0.622), (0.86, 0.626), (0.935, 0.622), (0.946, 0.607), (0.93, 0.59), (0.87, 0.572), (0.80, 0.568), (0.765, 0.585)]
TONGUE = [(0.80, 0.612), (0.87, 0.618), (0.925, 0.608), (0.93, 0.592), (0.89, 0.585), (0.83, 0.59)]
LR, FR = "coat-hindlimbs", "coat-forelimbs"
SIDES = (("f", 1.0, 0.55), ("n", 2.0, 0.72))

def hind(r, lvl):
    for s, z, tone in SIDES:
        n = "far" if s == "f" else "near"
        if lvl == 500:
            r.add(f"{n}-hind-thigh", f"thigh-h{s}", "body", z, True, tone, d=prof(H, K, [(0, 0.10, 0.10), (0.5, 0.09, 0.10), (1.0, 0.05, 0.05)]), region=LR)
            r.add(f"{n}-hind-shank", f"shank-h{s}", "body", z, False, tone, poly=[K, HK], closed=False, region=LR, penMm=0.2)
            r.add(f"{n}-hind-foot", f"metatarsus-h{s}", "body", z, False, tone, poly=[HK, HF, (0.045, 0.004)], closed=False, regions=[LR, "pads"], penMm=0.2)
            continue
        if lvl == 200:
            r.add(f"{n}-hind-thigh", f"thigh-h{s}", "body", z, True, tone, d=prof(H, K, [(0, 0.115, 0.115), (0.5, 0.10, 0.11), (1.0, 0.055, 0.055)]), region=LR)
            r.add(f"{n}-hind-shank", f"shank-h{s}", "body", z, True, tone, d=prof(K, HK, [(0, 0.05, 0.075), (0.5, 0.045, 0.05), (1.0, 0.035, 0.04)]), region=LR)
            r.add(f"{n}-hind-metatarsus", f"metatarsus-h{s}", "body", z, True, tone, d=prof(HK, HF, [(0, 0.04, 0.04), (1.0, 0.04, 0.04)]), region=LR)
            r.add(f"{n}-hind-paw", f"paw-h{s}", "body", z, True, tone, pts=HIND_PAW, regions=[LR, "pads"])
            continue
        r.add(f"{n}-hind-thigh", f"thigh-h{s}", "body", z, True, tone, d=prof(H, K, [(0, 0.115, 0.12), (0.45, 0.105, 0.115), (1.0, 0.052, 0.058)]), region=LR)
        r.add(f"{n}-hind-shank", f"shank-h{s}", "body", z, True, tone, d=prof(K, HK, [(0, 0.05, 0.08), (0.5, 0.04, 0.05), (1.0, 0.03, 0.035)]), region=LR)
        r.add(f"{n}-hind-metatarsus", f"metatarsus-h{s}", "body", z, True, tone, d=prof(HK, HF, [(0, 0.032, 0.04), (0.5, 0.024, 0.026), (1.0, 0.03, 0.03)]), region=LR)
        r.add(f"{n}-hind-paw", f"paw-h{s}", "body", z, True, tone, pts=HIND_PAW, region=LR)

def fore(r, lvl):
    for s, z, tone in SIDES:
        n = "far" if s == "f" else "near"
        if lvl == 500:
            r.add(f"{n}-fore-upper", f"humerus-f{s}", "body", z, True, tone, d=prof(EL0, EL, [(0, 0.075, 0.075), (1.0, 0.045, 0.045)]), region=FR)
            r.add(f"{n}-fore-forearm", f"forearm-f{s}", "body", z, False, tone, poly=[EL, CA], closed=False, region=FR, penMm=0.2)
            r.add(f"{n}-fore-foot", f"metacarpus-f{s}", "body", z, False, tone, poly=[CA, FF, (0.595, 0.004)], closed=False, regions=[FR, "pads"], penMm=0.2)
            continue
        r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=prof(SH, EL0, [(0, 0.075, 0.075), (0.5, 0.08, 0.08), (1.0, 0.065, 0.065)]), region=FR)
        r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=prof(EL0, EL, [(0, 0.075, 0.07), (0.5, 0.06, 0.055), (1.0, 0.045, 0.045)]), region=FR)
        if lvl == 200:
            r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=prof(EL, CA, [(0, 0.05, 0.05), (1.0, 0.035, 0.035)]), region=FR)
            r.add(f"{n}-metacarpus", f"metacarpus-f{s}", "body", z, True, tone, d=prof(CA, FF, [(0, 0.04, 0.04), (1.0, 0.04, 0.04)]), region=FR)
            r.add(f"{n}-forepaw", f"paw-f{s}", "body", z, True, tone, pts=FORE_PAW, regions=[FR, "pads"])
            continue
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=prof(EL, CA, [(0, 0.048, 0.05), (0.5, 0.036, 0.035), (1.0, 0.03, 0.03)]), region=FR)
        r.add(f"{n}-metacarpus", f"metacarpus-f{s}", "body", z, True, tone, d=prof(CA, FF, [(0, 0.032, 0.03), (1.0, 0.03, 0.03)]), region=FR)
        r.add(f"{n}-forepaw", f"paw-f{s}", "body", z, True, tone, pts=FORE_PAW, region=FR)

def tail(r, lvl):
    if lvl == 500:
        r.add("tail", "tail-2", "body", 2.8, False, 0.66, poly=[(-0.03, 0.56), (-0.18, 0.569), (-0.32, 0.574), (-0.44, 0.586)], closed=False, region="coat-tail", penMm=0.2)
        return
    if lvl == 200:
        r.add("tail", "tail-2", "body", 2.8, True, 0.66, pts=[(-0.03, 0.60), (-0.2, 0.605), (-0.33, 0.605), (-0.44, 0.592), (-0.44, 0.58), (-0.33, 0.55), (-0.2, 0.53), (-0.03, 0.52)], region="coat-tail")
        return
    for nm, b, A, B, wa, wb in (("tail-root", "tail-1", (-0.03, 0.558), (-0.18, 0.569), 0.095, 0.075), ("tail-mid", "tail-2", (-0.18, 0.569), (-0.32, 0.574), 0.075, 0.055), ("tail-tip", "tail-3", (-0.32, 0.574), (-0.439, 0.586), 0.055, 0.03)):
        r.add(nm, b, "body", 2.8, True, 0.66, d=prof(A, B, [(0, wa/2, wa/2), (1.0, wb/2, wb/2)]), region="coat-tail")

def rep(lvl):
    r = Rep(W)
    hind(r, lvl); fore(r, lvl); tail(r, lvl)
    if lvl in (50, 100): r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=[(x+0.004, y) for x, y in EAR], region="ears")
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=TRUNK_HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=TRUNK_FORE, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    if lvl == 500:
        r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, regions=["coat-head", "ears", "nose-skin", "tongue"])
        return r.shapes
    if lvl == 200:
        r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAW, regions=["coat-head", "tongue"])
        r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, regions=["coat-head", "nose-skin"])
        r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR, region="ears")
        return r.shapes
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAW, region="coat-head")
    r.add("tongue", "jaw", "body", 3.95, True, 0.42, pts=TONGUE, region="tongue")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, region="coat-head")
    r.ellipse("nose-leather", "head", "body", 4.2, True, 0.08, (0.975, 0.662), 0.013, 0.012, region="nose-skin")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (0.862, 0.681), 0.010 if lvl == 100 else 0.0065, 0.010 if lvl == 100 else 0.006)
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR, region="ears")
    for s, z, _ in SIDES:
        n = "far" if s == "f" else "near"
        r.add(f"{n}-hind-pad", f"paw-h{s}", "detail", z+0.1, True, 0.12, pts=[(-0.045, 0.006), (0.03, 0.006), (0.046, 0.012), (0.02, 0.022), (-0.03, 0.022)], region="pads")
        r.add(f"{n}-fore-pad", f"paw-f{s}", "detail", z+0.1, True, 0.12, pts=[(0.495, 0.006), (0.575, 0.006), (0.592, 0.012), (0.565, 0.024), (0.51, 0.024)], region="pads")
    if lvl == 100:
        r.add("shoulder-line", "thorax", "detail", 5.5, False, 0.25, poly=[(0.50, 0.58), (0.53, 0.5), (0.53, 0.43)], closed=False)
        return r.shapes
    r.add("brow", "head", "detail", 5.0, False, 0.2, poly=[(0.878, 0.696), (0.862, 0.692), (0.845, 0.695)], closed=False)
    r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(0.965, 0.636), (0.91, 0.626), (0.84, 0.62), (0.79, 0.614)], closed=False)
    r.add("near-hind-toes", "paw-hn", "detail", 5.2, False, 0.2, poly=[(0.02, 0.036), (0.03, 0.018), (0.034, 0.002)], closed=False)
    r.add("near-fore-toes", "paw-fn", "detail", 5.2, False, 0.2, poly=[(0.55, 0.04), (0.56, 0.02), (0.565, 0.002)], closed=False)
    r.add("shoulder-line", "thorax", "detail", 5.5, False, 0.25, poly=[(0.50, 0.58), (0.53, 0.5), (0.53, 0.43)], closed=False)
    r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.07, 0.57), (0.03, 0.51), (0.04, 0.45)], closed=False)
    r.add("elbow-line", "humerus-fn", "detail", 5.5, False, 0.25, poly=[(0.50, 0.40), (0.47, 0.35), (0.45, 0.29)], closed=False)
    r.add("fur-neck", "neck", "texture", 5.5, False, 0.35, poly=[(0.6, 0.6), (0.64, 0.63), (0.69, 0.655)], closed=False)
    r.add("fur-trunk-1", "thorax", "texture", 5.5, False, 0.35, poly=[(0.34, 0.56), (0.40, 0.565), (0.46, 0.56)], closed=False)
    r.add("fur-trunk-2", "thorax", "texture", 5.5, False, 0.35, poly=[(0.34, 0.34), (0.40, 0.33), (0.46, 0.335)], closed=False)
    r.add("fur-tail", "tail-2", "texture", 5.5, False, 0.35, poly=[(-0.21, 0.573), (-0.30, 0.576)], closed=False)
    return r.shapes

PENS = {500: dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.18), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25),
        100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5)}
if __name__ == "__main__":
    for den in (50, 100, 200, 500):
        rp = {"pens": PENS[den], "geometry": {"elevation": rep(den)}}
        preview(ASSET, rp, DEFAULT, f"{OUT}/dog-{den}-zoom.png", 1000/den, zoom={50: 2.0, 100: 4.0, 200: 8.0, 500: 14.0}[den], pad_m=0.05)
        print(den, len(rp["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rp["geometry"]["elevation"], W)])
