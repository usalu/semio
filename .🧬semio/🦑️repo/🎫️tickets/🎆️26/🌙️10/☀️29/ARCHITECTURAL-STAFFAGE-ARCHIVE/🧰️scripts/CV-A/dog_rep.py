# [scratch CV-A] labrador-dog: hand-authored representations (world coordinates of the standing-alert default pose)
from draw import *
ASSET = load_v1("labrador-dog"); DEFAULT = "standing-alert"
W = pose_world(ASSET, DEFAULT)
ALL = ["coat-trunk", "coat-head", "ears", "coat-forelimbs", "coat-hindlimbs", "coat-tail", "nose-skin", "tongue", "pads"]
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

def hind(r, side, z, tone, detail):
    n = "far" if side == "f" else "near"; s = side
    lk = {"thigh": f"thigh-h{s}", "shank": f"shank-h{s}", "meta": f"metatarsus-h{s}", "paw": f"paw-h{s}"}
    r.add(f"{n}-thigh", lk["thigh"], "body", z, True, tone, d=limb(H, K, 0.23, 0.085, 0.012, 0.5), region="coat-hindlimbs")
    r.add(f"{n}-shank", lk["shank"], "body", z, True, tone, d=limb(K, HK, 0.09, 0.05, 0.01, 0.35), region="coat-hindlimbs")
    r.add(f"{n}-hock", lk["meta"], "body", z, True, tone, d=limb(HK, HF, 0.048, 0.036), region="coat-hindlimbs")
    r.add(f"{n}-hind-paw", lk["paw"], "body", z, True, tone, pts=HIND_PAW, region="coat-hindlimbs")

def fore(r, side, z, tone):
    n = "far" if side == "f" else "near"; s = side
    r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=limb(SH, EL0, 0.15, 0.13, 0.0, 0.5), region="coat-forelimbs")
    r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=limb(EL0, EL, 0.14, 0.085, 0.012, 0.5), region="coat-forelimbs")
    r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=limb(EL, CA, 0.085, 0.05, 0.006, 0.5), region="coat-forelimbs")
    r.add(f"{n}-wrist", f"metacarpus-f{s}", "body", z, True, tone, d=limb(CA, FF, 0.05, 0.04), region="coat-forelimbs")
    r.add(f"{n}-forepaw", f"paw-f{s}", "body", z, True, tone, pts=FORE_PAW, region="coat-forelimbs")

def rep50():
    r = Rep(W)
    hind(r, "f", 1.0, 0.55, True); fore(r, "f", 1.0, 0.55)
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=[(x+0.004, y) for x, y in EAR], region="ears")
    r.add("tail-root", "tail-1", "body", 2.8, True, 0.66, d=limb((-0.03, 0.558), (-0.18, 0.569), 0.095, 0.075, 0, 0.5), region="coat-tail")
    r.add("tail-mid", "tail-2", "body", 2.8, True, 0.66, d=limb((-0.18, 0.569), (-0.32, 0.574), 0.075, 0.055, 0, 0.5), region="coat-tail")
    r.add("tail-tip", "tail-3", "body", 2.8, True, 0.66, d=limb((-0.32, 0.574), (-0.439, 0.586), 0.055, 0.016, 0, 0.5), region="coat-tail")
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=TRUNK_HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=TRUNK_FORE, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAW, region="coat-head")
    r.add("tongue", "jaw", "body", 3.95, True, 0.42, pts=TONGUE, region="tongue")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, region="coat-head")
    r.ellipse("nose-leather", "head", "body", 4.2, True, 0.08, (0.975, 0.662), 0.013, 0.012, region="nose-skin")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (0.862, 0.681), 0.0065, 0.006)
    r.add("brow", "head", "detail", 5.0, False, 0.2, poly=[(0.878, 0.696), (0.862, 0.692), (0.845, 0.695)], closed=False)
    r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(0.965, 0.636), (0.91, 0.626), (0.84, 0.62), (0.79, 0.614)], closed=False)
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR, region="ears")
    hind(r, "n", 5.0, 0.72, True); fore(r, "n", 5.0, 0.72)
    r.add("near-hind-pad", "paw-hn", "detail", 5.1, True, 0.12, pts=[(-0.045, 0.006), (0.03, 0.006), (0.046, 0.012), (0.02, 0.022), (-0.03, 0.022)], region="pads")
    r.add("far-hind-pad", "paw-hf", "detail", 1.1, True, 0.12, pts=[(-0.045, 0.006), (0.03, 0.006), (0.046, 0.012), (0.02, 0.022), (-0.03, 0.022)], region="pads")
    r.add("near-fore-pad", "paw-fn", "detail", 5.1, True, 0.12, pts=[(0.495, 0.006), (0.575, 0.006), (0.592, 0.012), (0.565, 0.024), (0.51, 0.024)], region="pads")
    r.add("far-fore-pad", "paw-ff", "detail", 1.1, True, 0.12, pts=[(0.495, 0.006), (0.575, 0.006), (0.592, 0.012), (0.565, 0.024), (0.51, 0.024)], region="pads")
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

def rep100():
    r = Rep(W)
    for side, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        n = "far" if side == "f" else "near"
        r.add(f"{n}-thigh", f"thigh-h{side}", "body", z, True, tone, d=limb(H, K, 0.23, 0.085, 0.012, 0.5), region="coat-hindlimbs")
        r.add(f"{n}-shank", f"shank-h{side}", "body", z, True, tone, d=limb(K, HK, 0.09, 0.05, 0.01, 0.35), region="coat-hindlimbs")
        r.add(f"{n}-hock", f"metatarsus-h{side}", "body", z, True, tone, d=limb(HK, HF, 0.048, 0.036), region="coat-hindlimbs")
        r.add(f"{n}-hind-paw", f"paw-h{side}", "body", z, True, tone, pts=HIND_PAW, region="coat-hindlimbs")
        r.add(f"{n}-shoulder", f"scapula-f{side}", "body", z, True, tone, d=limb(SH, EL0, 0.15, 0.13, 0.0, 0.5), region="coat-forelimbs")
        r.add(f"{n}-upper-arm", f"humerus-f{side}", "body", z, True, tone, d=limb(EL0, EL, 0.14, 0.085, 0.012, 0.5), region="coat-forelimbs")
        r.add(f"{n}-forearm", f"forearm-f{side}", "body", z, True, tone, d=limb(EL, CA, 0.085, 0.05, 0.006, 0.5), region="coat-forelimbs")
        r.add(f"{n}-wrist", f"metacarpus-f{side}", "body", z, True, tone, d=limb(CA, FF, 0.05, 0.04), region="coat-forelimbs")
        r.add(f"{n}-forepaw", f"paw-f{side}", "body", z, True, tone, pts=FORE_PAW, region="coat-forelimbs")
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=[(x+0.004, y) for x, y in EAR], region="ears")
    r.add("tail-root", "tail-1", "body", 2.8, True, 0.66, d=limb((-0.03, 0.558), (-0.18, 0.569), 0.095, 0.075, 0, 0.5), region="coat-tail")
    r.add("tail-mid", "tail-2", "body", 2.8, True, 0.66, d=limb((-0.18, 0.569), (-0.32, 0.574), 0.075, 0.055, 0, 0.5), region="coat-tail")
    r.add("tail-tip", "tail-3", "body", 2.8, True, 0.66, d=limb((-0.32, 0.574), (-0.439, 0.586), 0.055, 0.016, 0, 0.5), region="coat-tail")
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=TRUNK_HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=TRUNK_FORE, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAW, region="coat-head")
    r.add("tongue", "jaw", "body", 3.95, True, 0.42, pts=TONGUE, region="tongue")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, region="coat-head")
    r.ellipse("nose-leather", "head", "body", 4.2, True, 0.08, (0.975, 0.662), 0.013, 0.012, region="nose-skin")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (0.862, 0.681), 0.010, 0.010)
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR, region="ears")
    r.add("near-hind-pad", "paw-hn", "detail", 5.1, True, 0.12, pts=[(-0.045, 0.004), (0.03, 0.004), (0.048, 0.014), (0.02, 0.026), (-0.03, 0.026)], region="pads")
    r.add("far-hind-pad", "paw-hf", "detail", 1.1, True, 0.12, pts=[(-0.045, 0.004), (0.03, 0.004), (0.048, 0.014), (0.02, 0.026), (-0.03, 0.026)], region="pads")
    r.add("near-fore-pad", "paw-fn", "detail", 5.1, True, 0.12, pts=[(0.495, 0.004), (0.575, 0.004), (0.594, 0.014), (0.565, 0.026), (0.51, 0.026)], region="pads")
    r.add("far-fore-pad", "paw-ff", "detail", 1.1, True, 0.12, pts=[(0.495, 0.004), (0.575, 0.004), (0.594, 0.014), (0.565, 0.026), (0.51, 0.026)], region="pads")
    r.add("shoulder-line", "thorax", "detail", 5.5, False, 0.25, poly=[(0.50, 0.58), (0.53, 0.5), (0.53, 0.43)], closed=False)
    r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.07, 0.57), (0.03, 0.51), (0.04, 0.45)], closed=False)
    return r.shapes

def rep200():
    r = Rep(W)
    for side, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        n = "far" if side == "f" else "near"
        r.add(f"{n}-hind-thigh", f"thigh-h{side}", "body", z, True, tone, d=limb(H, K, 0.23, 0.10, 0.012, 0.5), region="coat-hindlimbs")
        r.add(f"{n}-hind-shank", f"shank-h{side}", "body", z, True, tone, d=limb(K, HK, 0.10, 0.075, 0.01, 0.35), region="coat-hindlimbs")
        r.add(f"{n}-hind-lower", f"metatarsus-h{side}", "body", z, True, tone, pts=[(-0.085, 0.17), (-0.01, 0.16), (-0.012, 0.045), (0.05, 0.03), (0.058, 0.004), (-0.05, 0.0), (-0.085, 0.03)], regions=["coat-hindlimbs", "pads"])
        r.add(f"{n}-fore-upper", f"humerus-f{side}", "body", z, True, tone, d=limb(SH, EL, 0.14, 0.10, 0.012, 0.5), region="coat-forelimbs")
        r.add(f"{n}-fore-forearm", f"forearm-f{side}", "body", z, True, tone, d=limb(EL, CA, 0.10, 0.065, 0.006, 0.5), region="coat-forelimbs")
        r.add(f"{n}-fore-lower", f"metacarpus-f{side}", "body", z, True, tone, pts=[(0.475, 0.10), (0.54, 0.10), (0.54, 0.05), (0.60, 0.03), (0.602, 0.004), (0.49, 0.0), (0.478, 0.04)], regions=["coat-forelimbs", "pads"])
    r.add("tail", "tail-2", "body", 2.8, True, 0.66, pts=[(-0.03, 0.60), (-0.2, 0.605), (-0.33, 0.605), (-0.44, 0.595), (-0.44, 0.578), (-0.33, 0.55), (-0.2, 0.53), (-0.03, 0.52)], region="coat-tail")
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=TRUNK_HIND, region="coat-trunk")
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=TRUNK_FORE, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT + [], regions=["coat-head", "nose-skin", "tongue"])
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAW, regions=["coat-head", "tongue"])
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR, region="ears")
    return r.shapes

def rep500():
    r = Rep(W)
    for side, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        pen = 0.13 if side == "f" else 0.18
        r.add(f"{side}-hind-thigh", f"thigh-h{side}", "body", z, True, tone, d=limb(H, K, 0.20, 0.10, 0, 0.5), regions=["coat-hindlimbs"])
        r.add(f"{side}-hind-leg", f"shank-h{side}", "body", z, False, tone, poly=[(0.01, 0.329), (-0.055, 0.166)], closed=False, regions=["coat-hindlimbs"], penMm=pen+0.07)
        r.add(f"{side}-hind-foot", f"metatarsus-h{side}", "body", z, False, tone, poly=[(-0.055, 0.166), (-0.043, 0.032), (0.03, 0.004)], closed=False, regions=["coat-hindlimbs", "pads"], penMm=pen+0.07)
        r.add(f"{side}-fore-upper", f"humerus-f{side}", "body", z, True, tone, d=limb(SH, EL, 0.14, 0.10, 0, 0.5), regions=["coat-forelimbs"])
        r.add(f"{side}-fore-leg", f"forearm-f{side}", "body", z, False, tone, poly=[(0.449, 0.285), (0.503, 0.093)], closed=False, regions=["coat-forelimbs"], penMm=pen+0.07)
        r.add(f"{side}-fore-foot", f"metacarpus-f{side}", "body", z, False, tone, poly=[(0.503, 0.093), (0.508, 0.033), (0.585, 0.004)], closed=False, regions=["coat-forelimbs", "pads"], penMm=pen+0.07)
    r.add("tail", "tail-2", "body", 2.8, False, 0.66, poly=[(-0.03, 0.56), (-0.18, 0.569), (-0.32, 0.574), (-0.44, 0.586)], closed=False, regions=["coat-tail"], penMm=0.2)
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=TRUNK_HIND, regions=["coat-trunk"])
    r.add("chest", "thorax", "body", 3.2, True, 0.72, pts=TRUNK_FORE, regions=["coat-trunk"])
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, regions=["coat-trunk"])
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEAD_PT, regions=["coat-head", "ears", "nose-skin", "tongue"])
    return r.shapes

if __name__ == "__main__":
    import sys
    for den, fn in ((50, rep50), (100, rep100), (200, rep200), (500, rep500)):
        mm = 1000/den
        pens = {50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.25), 100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.25),
                200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25), 500: dict(body=0.13, detail=None, texture=None, glazing=None, ground=0.25)}[den]
        rep = {"pens": pens, "geometry": {"elevation": fn()}}
        preview(ASSET, rep, DEFAULT, f"{OUT}/dog-{den}-print.png", mm, zoom=1, pad_m=0.1)
        preview(ASSET, rep, DEFAULT, f"{OUT}/dog-{den}-zoom.png", mm, zoom={50: 2.0, 100: 4.0, 200: 8.0, 500: 14.0}[den], pad_m=0.05)
        print(den, len(rep["geometry"]["elevation"]), "bbox", [round(v, 3) for v in bbox_of(rep["geometry"]["elevation"], W)])
