# [scratch CV-A] domestic-cat: hand-authored representations (world coordinates of the standing-alert default pose)
from draw import *
ASSET = load_v1("domestic-cat"); DEFAULT = "standing-alert"
W = pose_world(ASSET, DEFAULT)
H = (0.015, 0.198); K = (0.076, 0.107); HK = (-0.034, 0.094); HF = (-0.023, 0.020)
SH = (0.255, 0.253); E0 = (0.290, 0.187); EL = (0.226, 0.139); CA = (0.255, 0.059); FF = (0.259, 0.019)
TAIL = [(-0.040, 0.240), (-0.099, 0.251), (-0.155, 0.273), (-0.207, 0.303), (-0.253, 0.332), (-0.297, 0.342)]
HIND = [(-0.055, 0.215), (-0.04, 0.25), (-0.01, 0.268), (0.06, 0.272), (0.14, 0.270), (0.20, 0.268, 'c'), (0.20, 0.19, 'c'), (0.14, 0.18), (0.06, 0.185), (-0.01, 0.19), (-0.045, 0.19)]
MID = [(0.13, 0.270, 'c'), (0.20, 0.272), (0.26, 0.274), (0.29, 0.270, 'c'), (0.29, 0.19, 'c'), (0.27, 0.165), (0.21, 0.162), (0.15, 0.178), (0.12, 0.19, 'c')]
CHEST = [(0.25, 0.272, 'c'), (0.30, 0.276), (0.335, 0.26), (0.345, 0.215), (0.325, 0.175), (0.28, 0.16), (0.24, 0.165), (0.235, 0.22, 'c')]
NECK = [(0.27, 0.272), (0.31, 0.288), (0.355, 0.303), (0.378, 0.262), (0.345, 0.215), (0.30, 0.205)]
HEADP = [(0.352, 0.27), (0.358, 0.305), (0.388, 0.324), (0.425, 0.319), (0.450, 0.300), (0.467, 0.285), (0.463, 0.272), (0.442, 0.262), (0.41, 0.252), (0.38, 0.246), (0.358, 0.248)]
JAWP = [(0.385, 0.249), (0.43, 0.251), (0.456, 0.246), (0.456, 0.236), (0.436, 0.226), (0.40, 0.224), (0.385, 0.232)]
EAR_N = [(0.376, 0.308), (0.383, 0.357), (0.412, 0.322)]
EAR_F = [(0.392, 0.309), (0.399, 0.356), (0.428, 0.323)]
HPAW = [(-0.045, 0.03), (-0.012, 0.034), (0.02, 0.022), (0.03, 0.01), (0.022, 0.0), (-0.03, 0.0), (-0.044, 0.012)]
FPAW = [(0.238, 0.03), (0.27, 0.034), (0.302, 0.022), (0.312, 0.01), (0.304, 0.0), (0.255, 0.0), (0.238, 0.012)]
PADH = [(-0.035, 0.004), (0.015, 0.004), (0.026, 0.01), (0.01, 0.018), (-0.025, 0.018)]
PADF = [(0.248, 0.004), (0.298, 0.004), (0.308, 0.01), (0.29, 0.018), (0.255, 0.018)]
LR = "coat-hindlimbs"; FR = "coat-forelimbs"

def legs(r, detail):
    for s, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        n = "far" if s == "f" else "near"
        r.add(f"{n}-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(H, K, 0.095, 0.04, 0.008, 0.5), region=LR)
        r.add(f"{n}-shank", f"shank-h{s}", "body", z, True, tone, d=limb(K, HK, 0.042, 0.026, 0, 0.4), region=LR)
        r.add(f"{n}-hock", f"metatarsus-h{s}", "body", z, True, tone, d=limb(HK, HF, 0.026, 0.022), region=LR)
        r.add(f"{n}-hind-paw", f"paw-h{s}", "body", z, True, tone, pts=HPAW, region=LR)
        r.add(f"{n}-scapula", f"scapula-f{s}", "body", z, True, tone, d=limb(SH, E0, 0.07, 0.065, 0, 0.5), region=FR)
        r.add(f"{n}-humerus", f"humerus-f{s}", "body", z, True, tone, d=limb(E0, EL, 0.065, 0.04, 0.005, 0.5), region=FR)
        r.add(f"{n}-forearm", f"forearm-f{s}", "body", z, True, tone, d=limb(EL, CA, 0.04, 0.025), region=FR)
        r.add(f"{n}-wrist", f"metacarpus-f{s}", "body", z, True, tone, d=limb(CA, FF, 0.025, 0.022), region=FR)
        r.add(f"{n}-fore-paw", f"paw-f{s}", "body", z, True, tone, pts=FPAW, region=FR)
        if detail:
            r.add(f"{n}-hind-pad", f"paw-h{s}", "detail", z+0.1, True, 0.12, pts=PADH, region="pads")
            r.add(f"{n}-fore-pad", f"paw-f{s}", "detail", z+0.1, True, 0.12, pts=PADF, region="pads")

def tail(r, mode):
    ws = [0.034, 0.032, 0.03, 0.028, 0.026, 0.02]
    for i, bone in enumerate(("tail-1", "tail-2", "tail-3", "tail-4", "tail-5")):
        r.add(f"tail-{i+1}", bone, "body", 2.8, True, 0.66, d=limb(TAIL[i], TAIL[i+1], ws[i], ws[i+1], 0, 0.5), region="coat-tail")

def head(r, lvl=50):
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAWP, region="coat-head")
    r.add("tongue", "jaw", "body", 3.95, True, 0.42, pts=[(0.40, 0.239), (0.44, 0.243), (0.452, 0.238), (0.43, 0.231), (0.405, 0.232)], region="tongue")
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, **({"region": "coat-head"} if lvl == 50 else {"regions": ["coat-head", "nose-skin"]}))
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=EAR_F, region="ears")
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR_N, region="ears")
    if lvl == 50:
        r.ellipse("nose-leather", "head", "body", 4.2, True, 0.08, (0.463, 0.279), 0.005, 0.004, region="nose-skin")
    r.ellipse("eye", "head", "detail", 5.0, True, 0.04, (0.432, 0.292), 0.0055 if lvl == 50 else 0.0105, 0.0055 if lvl == 50 else 0.0105, region="eyes")

def rep50():
    r = Rep(W); legs(r, True); tail(r, 0)
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=HIND, region="coat-trunk")
    r.add("midbody", "thorax-a", "body", 3.1, True, 0.72, pts=MID, region="coat-trunk")
    r.add("chest", "thorax-b", "body", 3.2, True, 0.72, pts=CHEST, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    head(r)
    r.add("inner-ear", "ear-n", "detail", 4.6, False, 0.2, poly=[(0.383, 0.318), (0.386, 0.345), (0.402, 0.327)], closed=False)
    r.add("brow", "head", "detail", 5.0, False, 0.2, poly=[(0.442, 0.299), (0.43, 0.299), (0.42, 0.297)], closed=False)
    r.add("mouth-line", "head", "detail", 5.0, False, 0.2, poly=[(0.462, 0.272), (0.44, 0.263), (0.41, 0.257)], closed=False)
    r.add("shoulder-line", "thorax-b", "detail", 5.5, False, 0.25, poly=[(0.285, 0.265), (0.30, 0.22), (0.29, 0.19, 'c')], closed=False)
    r.add("hip-line", "lumbar", "detail", 5.5, False, 0.25, poly=[(0.03, 0.26), (0.0, 0.22), (0.02, 0.195)], closed=False)
    r.add("near-hind-toes", "paw-hn", "detail", 5.2, False, 0.2, poly=[(0.0, 0.03), (0.01, 0.015), (0.013, 0.003)], closed=False)
    r.add("near-fore-toes", "paw-fn", "detail", 5.2, False, 0.2, poly=[(0.28, 0.03), (0.29, 0.015), (0.293, 0.003)], closed=False)
    r.add("stripe-1", "thorax-a", "texture", 5.5, False, 0.35, poly=[(0.17, 0.268), (0.175, 0.24), (0.17, 0.21)], closed=False)
    r.add("stripe-2", "thorax-a", "texture", 5.5, False, 0.35, poly=[(0.21, 0.27), (0.215, 0.24), (0.21, 0.205)], closed=False)
    r.add("stripe-3", "lumbar", "texture", 5.5, False, 0.35, poly=[(0.09, 0.27), (0.095, 0.24), (0.09, 0.205)], closed=False)
    r.add("tail-ring", "tail-3", "texture", 5.5, False, 0.35, poly=[(-0.18, 0.282), (-0.178, 0.262)], closed=False)
    return r.shapes

def rep100():
    r = Rep(W); legs(r, True); tail(r, 0)
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=HIND, region="coat-trunk")
    r.add("midbody", "thorax-a", "body", 3.1, True, 0.72, pts=MID, region="coat-trunk")
    r.add("chest", "thorax-b", "body", 3.2, True, 0.72, pts=CHEST, region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    head(r, 100)
    r.add("shoulder-line", "thorax-b", "detail", 5.5, False, 0.25, poly=[(0.285, 0.265), (0.30, 0.22), (0.29, 0.19, 'c')], closed=False)
    return r.shapes

def rep200():
    r = Rep(W)
    for s, z, tone in (("f", 1.0, 0.55), ("n", 5.0, 0.72)):
        r.add(f"{s}-hind-thigh", f"thigh-h{s}", "body", z, True, tone, d=limb(H, K, 0.10, 0.05, 0.008, 0.5), region=LR)
        r.add(f"{s}-hind-lower", f"shank-h{s}", "body", z, False, tone, poly=[K, HK], closed=False, region=LR, penMm=0.2)
        r.add(f"{s}-hind-foot", f"metatarsus-h{s}", "body", z, False, tone, poly=[HK, HF, (0.015, 0.005)], closed=False, regions=[LR, "pads"], penMm=0.2)
        r.add(f"{s}-fore-upper", f"humerus-f{s}", "body", z, True, tone, d=limb(SH, EL, 0.075, 0.05, 0.004, 0.5), region=FR)
        r.add(f"{s}-fore-lower", f"forearm-f{s}", "body", z, False, tone, poly=[EL, CA], closed=False, region=FR, penMm=0.2)
        r.add(f"{s}-fore-foot", f"metacarpus-f{s}", "body", z, False, tone, poly=[CA, FF, (0.29, 0.005)], closed=False, regions=[FR, "pads"], penMm=0.2)
    r.add("tail-a", "tail-2", "body", 2.8, False, 0.66, poly=TAIL[:3], closed=False, region="coat-tail", penMm=0.2)
    r.add("tail-b", "tail-4", "body", 2.8, False, 0.66, poly=TAIL[2:], closed=False, region="coat-tail", penMm=0.2)
    r.add("hindbody", "lumbar", "body", 3.0, True, 0.72, pts=HIND, region="coat-trunk")
    r.add("chest", "thorax-b", "body", 3.2, True, 0.72, pts=MID[:4] + CHEST[1:], region="coat-trunk")
    r.add("neck", "neck", "body", 3.3, True, 0.72, pts=NECK, region="coat-trunk")
    r.add("jaw", "jaw", "body", 3.9, True, 0.72, pts=JAWP, regions=["coat-head", "tongue"])
    r.add("head", "head", "body", 4.0, True, 0.72, pts=HEADP, regions=["coat-head", "nose-skin", "eyes"])
    r.add("near-ear", "ear-n", "body", 4.5, True, 0.6, pts=EAR_N, region="ears")
    r.add("far-ear", "ear-f", "body", 2.0, True, 0.5, pts=EAR_F, region="ears")
    return r.shapes

if __name__ == "__main__":
    for den, fn in ((50, rep50), (100, rep100), (200, rep200)):
        mm = 1000/den
        pens = {50: dict(body=0.35, detail=0.18, texture=0.1, glazing=None, ground=0.5), 100: dict(body=0.25, detail=0.13, texture=None, glazing=None, ground=0.35), 200: dict(body=0.18, detail=0.1, texture=None, glazing=None, ground=0.25)}[den]
        rep = {"pens": pens, "geometry": {"elevation": fn()}}
        preview(ASSET, rep, DEFAULT, f"{OUT}/cat-{den}-zoom.png", mm, zoom={50: 3.0, 100: 6.0, 200: 12.0}[den], pad_m=0.03)
        print(den, len(rep["geometry"]["elevation"]), [round(v, 3) for v in bbox_of(rep["geometry"]["elevation"], W)])
