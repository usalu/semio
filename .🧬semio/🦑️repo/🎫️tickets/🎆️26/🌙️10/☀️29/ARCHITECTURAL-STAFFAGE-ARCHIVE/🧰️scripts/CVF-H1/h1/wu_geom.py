"""wheelchair-user: hand-authored 1:500 / 1:200 / 1:100 geometry (default pose seated-level). Frame in frame-bone coordinates (rear axle origin)."""
import math
from lib import *
import human_parts as hp

SKIN, BEAN, HSK, GLV, HOOD, JE, SHOE, FRM, TYR, CUSH = "face-skin", "beanie", "hand-skin", "gloves", "hoodie", "jeans", "shoes", "frame-metal", "tyres", "cushion"
K = 0.979
TONES = {"sleeve": 0.45, "hand": 0.7, "leg": 0.3, "shoe": 0.85}
GROUND = {"id": "ground", "bone": "frame", "role": "ground", "z": 0, "filled": False, "tone": 0.0, "kind": "poly", "points": [[-0.02, -0.305], [1.7, -0.305]], "closed": False}


def ring(F, id, bone, role, r, z, region, pen=None, filled=False, tone=0.3):
    return shape(id, bone, role, {"kind": "ellipse", "cx": 0, "cy": 0, "rx": r, "ry": r}, z, filled, tone, region=region, penMm=pen)


def stroke(F, id, bone, pts, z, region=None, regions=None, pen=None, role="body"):
    return F.sh(id, bone, role, pts, z, 0.3, region=region, regions=regions, closed=False, filled=False, smooth=False, raw=True, pen=pen)


def spokes(F, ids, r0, r1, region, z=7.1):
    out = []
    for i in ids:
        a = math.radians(30 * i)
        out.append(stroke(F, f"spoke-{i}", "rear-wheel", [(round(r0 * math.cos(a), 4), round(r0 * math.sin(a), 4)), (round(r1 * math.cos(a), 4), round(r1 * math.sin(a), 4))], z, region=region, role="detail"))
    return out


def person(F, den, hood_pts):
    s = []
    tones, reg = TONES, {"sleeve": HOOD, "hand": GLV, "leg": JE, "shoe": SHOE}
    s += hp.arms(F, den, None, K, K, (("l", 0.5), ("r", 6.0)), tones, reg, thumb=(den == 100))
    for x in s:
        if x["id"].startswith("hand-"):
            x["regions"] = [GLV, HSK]; del x["region"]
    s += hp.legs(F, den, K, K, (("l", 1.0), ("r", 4.0)), tones, reg)
    s.append(F.sh("pelvis-jeans", "pelvis", "body", [(0.1029, 0.1127), (0.0539, 0.1352), (-0.0049, 0.1323), (-0.0735, 0.1078), (-0.0735, -0.098), (-0.0049, -0.1274), (0.0539, -0.1294), (0.1029, -0.1156)], 3, 0.3, region=JE, raw=True, corners=(3, 4)))
    hd = [(-0.1323, 0.1323), (0.0147, 0.1254), (0.1617, 0.1215), (0.2695, 0.1372), (0.4067, 0.1058), (0.4381, 0.0608), (0.4557, -0.0294), (0.4459, -0.0833), (0.3381, -0.1156), (0.2, -0.11), (0.0637, -0.1127), (-0.0833, -0.1225), (-0.1323, -0.1176)]
    s.append(F.sh("hoodie", "torso", "body", hd, 4, 0.45, region=HOOD, corners=(0, 12)))
    s.append(F.sh("hood", "neck", "body", hood_pts, 4.5, 0.45, region=HOOD, raw=True))
    if den == 100:
        s.append(F.sh("neck", "neck", "body", hp.NECK100, 3.8, 0.25, region=SKIN, corners=(0, 5), raw=True))
        s.append(F.sh("hoodie-pocket", "torso", "detail", [(-0.0147, 0.0), (-0.0343, -0.1176), (-0.0833, -0.1176), (-0.0735, 0.0)], 4.1, 0.2, region=HOOD, closed=False, filled=False, raw=True, smooth=False))
    s += [x for x in hp.head(F, den, K, 5, SKIN, BEAN) if x["id"] != "hair"]
    s.append(F.sh("beanie", "head", "body", [(0.0647, 0.0939), (0.1382, 0.1233), (0.2019, 0.0724), (0.197, -0.0109), (0.1921, -0.0746), (0.1431, -0.0923), (0.1069, -0.0844), (0.1206, -0.0452), (0.1147, 0.0479)], 5.5, 0.6, region=BEAN, raw=True))
    if den == 100:
        s.append(F.sh("beanie-cuff", "head", "detail", [(0.0677, 0.0949), (0.1108, 0.0528), (0.1186, -0.0452), (0.1069, -0.0854)], 5.55, 0.2, region=BEAN, closed=False, filled=False, raw=True, smooth=False))
    return s


def build(a, pose):
    F = Fig(a, pose)
    R = {}
    # ------------------------------------------------------------------ 1:500
    s = [dict(GROUND)]
    s.append(ring(F, "wheel-ring", "rear-wheel", "body", 0.298, 7, None, tone=0.3) | {"regions": [TYR, FRM]})
    s.append(shape("castor-disc", "castor", "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.0678, "ry": 0.0678}, 7, True, 0.3, region=TYR))
    s.append(stroke(F, "frame-main", "frame", [(-0.2, 0.605), (-0.115, 0.625), (-0.07, 0.165), (0.42, 0.19), (0.49, -0.205)], 6.8, regions=[FRM, CUSH]))
    s.append(stroke(F, "frame-lower", "frame", [(0.0, 0.0), (0.4, -0.005), (0.49, -0.185)], 6.7, region=FRM))
    s.append(stroke(F, "footplate", "frame", [(0.46, -0.2175), (0.78, -0.2175)], 6.7, region=FRM))
    sil = [(-0.12, 0.57), (-0.125, 0.75), (-0.12, 0.95), (-0.1, 1.08), (-0.115, 1.15), (-0.1, 1.2), (-0.1, 1.3), (-0.115, 1.37), (-0.09, 1.43), (0.0, 1.455), (0.1, 1.42), (0.115, 1.36),
           (0.12, 1.3), (0.105, 1.22), (0.09, 1.17), (0.12, 1.12), (0.137, 1.0), (0.135, 0.8), (0.13, 0.65), (0.12, 0.57)]
    s.append(F.wsh("sil-torso", "torso", "body", sil, 5, 0.4, regions=[HOOD, SKIN, BEAN], corners=(0, 19)))
    s.append(F.sh("sil-sleeve-r", "arm-r", "body", hp.sc([(-0.02, -0.058), (0.15, -0.056), (0.33, -0.046), (0.36, 0.0), (0.33, 0.05), (0.15, 0.058), (-0.02, 0.06)], K), 6, 0.45, region=HOOD, raw=True))
    s.append(F.sh("sil-forearm-r", "forearm-r", "body", hp.sc([(-0.03, 0.0), (-0.01, -0.047), (0.15, -0.043), (0.245, -0.036), (0.33, -0.03), (0.43, -0.012), (0.435, 0.015), (0.33, 0.035), (0.245, 0.04), (0.12, 0.046), (-0.01, 0.05)], K), 6.1, 0.5, regions=[HOOD, GLV, HSK], raw=True))
    s.append(F.sh("sil-thigh-r", "thigh-r", "body", hp.sc([(-0.02, -0.095), (0.2, -0.088), (0.415, -0.062), (0.45, 0.0), (0.415, 0.066), (0.2, 0.085), (-0.02, 0.09)], K), 4, 0.3, region=JE, raw=True))
    s.append(F.sh("sil-shank-r", "shank-r", "body", hp.sc([(0.0, -0.062), (0.15, -0.078), (0.3, -0.055), (0.415, -0.052), (0.415, 0.05), (0.3, 0.05), (0.1, 0.056), (0.0, 0.062)], K), 4.02, 0.3, region=JE, raw=True))
    s.append(F.sh("sil-shoe-r", "foot-r", "body", hp.sc([(-0.045, -0.02), (-0.03, -0.075), (-0.006, -0.0848), (0.1867, 0.0004), (0.225, 0.022), (0.244, 0.045), (0.23, 0.062), (0.16, 0.0626), (0.034, 0.0476), (-0.03, 0.012)], K), 4.1, 0.85, region=SHOE, raw=True, corners=(2, 3)))
    R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000},
                "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [], "geometry": {"elevation": s}}
    # ------------------------------------------------------------------ 1:200
    s = [dict(GROUND)]
    s.append(ring(F, "rear-tyre", "rear-wheel", "body", 0.2935, 7, TYR))
    s += spokes(F, (0, 2, 4, 6, 8, 10), 0.06, 0.285, FRM)
    s.append(shape("rear-hub", "rear-wheel", "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.04, "ry": 0.04}, 7.2, True, 0.6, region=FRM))
    s.append(shape("castor-tyre", "castor", "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.0678, "ry": 0.0678}, 7, True, 0.3, region=TYR))
    s.append(stroke(F, "castor-spoke", "castor", [(-0.05, 0.0), (0.05, 0.0)], 7.1, region=FRM, role="detail"))
    for nm, pts, z in (("frame-back-tube", [(-0.2, 0.605), (-0.115, 0.625), (-0.07, 0.165)], 6.8), ("frame-seat-rail", [(-0.07, 0.165), (0.42, 0.19)], 6.8), ("frame-hanger", [(0.42, 0.19), (0.49, -0.2)], 6.8),
                       ("frame-lower-rail", [(0.0, 0.0), (0.4, -0.005), (0.49, -0.185)], 6.7), ("castor-fork", [(0.46, -0.09), (0.44, -0.2272)], 6.7), ("footplate", [(0.46, -0.2175), (0.78, -0.2175)], 6.7)):
        s.append(stroke(F, nm, "frame", pts, z, region=FRM, pen=0.15))
    s.append(F.sh("seat-cushion", "frame", "body", [(-0.07, 0.165), (0.42, 0.19), (0.42, 0.227), (0.3, 0.237), (0.0, 0.233), (-0.07, 0.223)], 2.4, 0.6, region=CUSH, raw=True, smooth=False))
    s.append(stroke(F, "back-sling", "frame", [(-0.0875, 0.225), (-0.106, 0.555)], 2.3, region=CUSH, pen=0.13))
    s += person(F, 200, hp.COWL)
    R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350},
                "pens": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.2}, "hatches": [], "geometry": {"elevation": s}}
    # ------------------------------------------------------------------ 1:100
    s = [dict(GROUND)]
    outer = 0.305; inner = 0.282
    def circ(r, sweep):
        k = 0.5523 * r
        pts = [(r, 0, r, k, k, r, 0, r), (0, r, -k, r, -r, k, -r, 0), (-r, 0, -r, -k, -k, -r, 0, -r), (0, -r, k, -r, r, -k, r, 0)]
        if sweep == 0:
            pts = [(x3, y3, x2, y2, x1, y1, x0, y0) for (x0, y0, x1, y1, x2, y2, x3, y3) in reversed(pts)]
        d = f"M {pts[0][0]} {pts[0][1]}"
        for p in pts:
            d += f" C {p[2]:.4f} {p[3]:.4f} {p[4]:.4f} {p[5]:.4f} {p[6]:.4f} {p[7]:.4f}"
        return d + " Z"
    s.append(shape("rear-tyre", "rear-wheel", "body", path(circ(outer, 1) + " " + circ(inner, 0)), 7, True, 0.3, region=TYR))
    s.append(ring(F, "hand-rim", "rear-wheel", "detail", 0.262, 7.05, FRM))
    s += spokes(F, range(12), 0.05, 0.28, FRM)
    s.append(shape("rear-hub", "rear-wheel", "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.034, "ry": 0.034}, 7.2, True, 0.6, region=FRM))
    s.append(shape("castor-tyre", "castor", "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.0678, "ry": 0.0678}, 7, True, 0.3, region=TYR))
    s.append(shape("castor-hub", "castor", "detail", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.018, "ry": 0.018}, 7.2, True, 0.6, region=FRM))
    s.append(stroke(F, "castor-spoke", "castor", [(-0.05, 0.0), (0.05, 0.0)], 7.1, region=FRM, role="detail"))
    for nm, (x0, y0, x1, y1, r) in {"frame-seat-rail": (-0.07, 0.165, 0.42, 0.19, 0.012), "frame-back-tube": (-0.07, 0.165, -0.115, 0.625, 0.012), "frame-push-handle": (-0.115, 0.625, -0.2, 0.605, 0.014),
                                    "frame-hanger": (0.42, 0.19, 0.49, -0.195, 0.012), "frame-lower-rail": (0.0, 0.0, 0.4, -0.005, 0.011), "frame-lower-front": (0.4, -0.005, 0.49, -0.185, 0.011),
                                    "castor-fork": (0.46, -0.09, 0.44, -0.2272, 0.01)}.items():
        s.append(shape(nm, "frame", "body", {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r, "r1": r}, 6.8, True, 0.6, region=FRM, penMm=0.1))
    s.append(F.sh("castor-housing", "frame", "body", [(0.43, -0.055), (0.49, -0.055), (0.49, -0.115), (0.43, -0.115)], 6.75, 0.6, region=FRM, raw=True, smooth=False))
    s.append(F.sh("footplate", "frame", "body", [(0.46, -0.205), (0.78, -0.205), (0.78, -0.23), (0.46, -0.23)], 6.7, 0.6, region=FRM, raw=True, smooth=False))
    s.append(F.sh("seat-cushion", "frame", "body", [(-0.07, 0.165), (0.42, 0.19), (0.42, 0.227), (0.3, 0.237), (0.0, 0.233), (-0.07, 0.223)], 2.4, 0.6, region=CUSH, raw=True, smooth=False))
    s.append(F.sh("back-sling", "frame", "body", [(-0.075, 0.225), (-0.092, 0.555), (-0.12, 0.555), (-0.1, 0.225)], 2.3, 0.6, region=CUSH, raw=True, smooth=False))
    s.append(F.sh("brake-lever", "frame", "detail", [(0.08, 0.175), (0.15, 0.315), (0.165, 0.31), (0.095, 0.17)], 6.9, 0.5, region=FRM, raw=True, smooth=False))
    s += person(F, 100, hp.COWL)
    R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150},
                "pens": {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [], "geometry": {"elevation": s}}
    return R
