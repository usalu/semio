"""seated-reader: hand-authored 1:500 / 1:200 / 1:100 geometry (default pose seated-upright)."""
from lib import *
import human_parts as hp

SKIN, HAIR, SW, JE, SHOE, HAND, NB, BENCH, CUP = "face-skin", "hair", "sweater", "jeans", "shoes", "hand-skin", "notebook", "bench", "cup"
K = 0.945
TONES = {"sleeve": 0.45, "hand": 0.3, "leg": 0.3, "shoe": 0.85}
REG = {"sleeve": SW, "hand": HAND, "leg": JE, "shoe": SHOE}


def bench(F, scale):
    s = []
    if scale == 500:
        pts = [(-0.38, 0.86), (-0.345, 0.86), (-0.265, 0.42), (-0.245, 0.42), (-0.245, 0.525), (-0.185, 0.525), (-0.185, 0.42), (0.2, 0.42), (0.2, 0.38), (0.17, 0.38),
               (0.17, 0.0), (0.11, 0.0), (0.11, 0.38), (-0.2, 0.38), (-0.2, 0.0), (-0.26, 0.0), (-0.26, 0.38), (-0.28, 0.38), (-0.3, 0.42)]
        s.append(F.wsh("sil-bench", "seat", "body", pts, 1.4, 0.6, regions=[BENCH, CUP], smooth=False))
        return s
    s.append(F.wsh("bench-seat", "seat", "body", [(-0.28, 0.38), (0.2, 0.38), (0.2, 0.42), (-0.28, 0.42)], 1.5, 0.6, region=BENCH, smooth=False))
    s.append(F.wsh("bench-leg-front", "seat", "body", [(0.12, 0.0), (0.17, 0.0), (0.17, 0.38), (0.12, 0.38)], 1.4, 0.6, region=BENCH, smooth=False))
    s.append(F.wsh("bench-leg-rear", "seat", "body", [(-0.25, 0.0), (-0.2, 0.0), (-0.2, 0.38), (-0.25, 0.38)], 1.4, 0.6, region=BENCH, smooth=False))
    s.append(F.wsh("bench-back", "seat", "body", [(-0.3, 0.42), (-0.265, 0.42), (-0.345, 0.86), (-0.38, 0.86)], 0.8, 0.6, region=BENCH, smooth=False))
    s.append(F.wsh("cup-body", "cup", "body", [(-0.245, 0.42), (-0.185, 0.42), (-0.19, 0.51), (-0.24, 0.51)], 1.7, 0.4, region=CUP, smooth=False))
    s.append(F.wsh("cup-lid", "cup", "body", [(-0.243, 0.51), (-0.187, 0.51), (-0.185, 0.525), (-0.245, 0.525)], 1.75, 0.2, region=CUP, smooth=False))
    if scale == 100:
        s.append(F.wsh("bench-slat", "seat", "detail", [(-0.28, 0.4), (0.2, 0.4)], 1.6, 0.1, region=BENCH, closed=False, filled=False, smooth=False))
        s.append(F.wsh("bench-back-slat", "seat", "detail", [(-0.29, 0.5), (-0.275, 0.5)], 0.9, 0.1, region=BENCH, closed=False, filled=False, smooth=False))
        s.append(F.wsh("cup-sleeve", "cup", "detail", [(-0.244, 0.46), (-0.186, 0.46)], 1.76, 0.2, region=CUP, closed=False, filled=False, smooth=False))
    return s


def notebook(F, scale):
    if scale == 500:
        return [F.sh("sil-notebook", "pad", "body", [(0.0, -0.015), (0.2, -0.015), (0.2, 0.015), (0.0, 0.015)], 6.4, 0.7, region=NB, raw=True, smooth=False)]
    t = 0.015 if scale == 200 else 0.014
    out = [F.sh("pad-cover", "pad", "body", [(0.0, -t), (0.2, -t), (0.2, t), (0.0, t)], 6.4, 0.75, region=NB, raw=True, smooth=False)]
    out.append(F.sh("page-sheet", "page", "detail", [(0.0, 0.014), (0.19, 0.014)], 6.5, 0.1, region=NB, closed=False, filled=False, raw=True, smooth=False))
    if scale == 100:
        out.append(F.sh("pad-edge", "pad", "detail", [(0.0, 0.0), (0.2, 0.0)], 6.45, 0.1, region=NB, closed=False, filled=False, raw=True, smooth=False))
    return out


def build(a, pose):
    F = Fig(a, pose)
    R = {}
    # ---------------------------------------------------------------------------------- 1:500
    s5 = bench(F, 500)
    sil = [(-0.125, 0.47), (-0.128, 0.62), (-0.12, 0.8), (-0.105, 0.95), (-0.06, 1.03), (-0.03, 1.07), (-0.025, 1.12), (-0.02, 1.2), (0.0, 1.27), (0.03, 1.33),
           (0.1, 1.34), (0.14, 1.3), (0.17, 1.26), (0.178, 1.2), (0.175, 1.15), (0.15, 1.1), (0.12, 1.07), (0.115, 1.03), (0.15, 0.97), (0.16, 0.85), (0.155, 0.72),
           (0.15, 0.6), (0.13, 0.5), (0.12, 0.46)]
    s5.append(F.wsh("sil-torso", "torso", "body", sil, 5, 0.4, regions=[SW, SKIN, HAIR], corners=(0, 23)))
    s5.append(F.sh("sil-sleeve-r", "arm-r", "body", [(-0.02, -0.055), (0.15, -0.05), (0.3, -0.045), (0.325, 0.0), (0.3, 0.047), (0.15, 0.055), (-0.02, 0.055)], 6, 0.45, region=SW, raw=True))
    s5.append(F.sh("sil-forearm-r", "forearm-r", "body", [(-0.03, 0.0), (-0.01, -0.047), (0.15, -0.043), (0.245, -0.035), (0.30, -0.02), (0.37, 0.01), (0.34, 0.04), (0.245, 0.04), (0.1, 0.046), (-0.01, 0.05)], 6, 0.4, regions=[SW, HAND], raw=True))
    s5 += notebook(F, 500)
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s5.append(F.sh(f"sil-thigh-{sd}", f"thigh-{sd}", "body", hp.sc([(-0.02, -0.095), (0.2, -0.088), (0.415, -0.062), (0.45, 0.0), (0.415, 0.066), (0.2, 0.085), (-0.02, 0.09)], K), z, 0.3, region=JE, raw=True))
        s5.append(F.sh(f"sil-shank-{sd}", f"shank-{sd}", "body", hp.sc([(0.0, -0.062), (0.15, -0.078), (0.3, -0.055), (0.415, -0.052), (0.415, 0.05), (0.3, 0.05), (0.1, 0.056), (0.0, 0.062)], K), z + 0.02, 0.3, region=JE, raw=True))
        s5.append(F.sh(f"sil-shoe-{sd}", f"foot-{sd}", "body", hp.sc([(-0.045, -0.02), (-0.03, -0.075), (-0.006, -0.0848), (0.1867, 0.0004), (0.225, 0.022), (0.244, 0.045), (0.23, 0.062), (0.16, 0.0626), (0.034, 0.0476), (-0.03, 0.012)], K), z + 0.1, 0.85, region=SHOE, raw=True, corners=(2, 3)))
    R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000},
                "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [],
                "geometry": {"elevation": s5}}
    # ---------------------------------------------------------------------------------- 1:200
    s2 = bench(F, 200)
    s2 += hp.arms(F, 200, None, K, K, (("l", 0.5), ("r", 6.0)), TONES, REG)
    s2 += hp.legs(F, 200, K, K, (("l", 1.0), ("r", 3.0)), TONES, REG)
    s2.append(F.sh("pelvis-jeans", "pelvis", "body", [(0.0991, 0.1086), (0.0519, 0.1303), (-0.0047, 0.1274), (-0.0708, 0.1038), (-0.0708, -0.0944), (-0.0047, -0.1227), (0.0519, -0.1246), (0.0991, -0.1114)], 3, 0.3, region=JE, raw=True, corners=(3, 4)))
    s2.append(F.sh("sweater", "torso", "body", [(-0.127, 0.127), (0.06, 0.119), (0.2, 0.128), (0.26, 0.137), (0.335, 0.127), (0.422, 0.059), (0.439, -0.028), (0.4, -0.09), (0.3257, -0.111), (0.156, -0.102), (0.061, -0.109), (-0.127, -0.113)], 4, 0.45, region=SW, corners=(0, 11)))
    s2.append(F.sh("sweater-front", "torso", "detail", [(0.06, 0.119), (0.0, -0.0), (-0.12, -0.0)], 4.1, 0.1, region=SW, closed=False, filled=False, smooth=False)) if False else None
    s2 = [x for x in s2 if x]
    s2.append(F.sh("cowl", "neck", "body", hp.COWL, 4.5, 0.45, region=SW, raw=True))
    s2 += hp.head(F, 200, K, 5, SKIN, HAIR)
    s2.append(F.ell("hair-bun", "head", "body", 0.1566, 0.0933, 0.0378, 0.035, 5.6, 0.75, region=HAIR, raw=True))
    s2 += notebook(F, 200)
    R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350},
                "pens": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.2}, "hatches": [],
                "geometry": {"elevation": s2}}
    # ---------------------------------------------------------------------------------- 1:100
    s1 = bench(F, 100)
    s1 += hp.arms(F, 100, None, K, K, (("l", 0.5), ("r", 6.0)), TONES, REG)
    s1 += hp.legs(F, 100, K, K, (("l", 1.0), ("r", 3.0)), TONES, REG)
    s1.append(F.sh("pelvis-jeans", "pelvis", "body", [(0.0991, 0.1086), (0.0519, 0.1303), (-0.0047, 0.1274), (-0.0708, 0.1038), (-0.0708, -0.0944), (-0.0047, -0.1227), (0.0519, -0.1246), (0.0991, -0.1114)], 3, 0.3, region=JE, raw=True, corners=(3, 4)))
    s1.append(F.sh("sweater", "torso", "body", [(-0.127, 0.127), (0.0, 0.122), (0.06, 0.119), (0.2, 0.128), (0.26, 0.137), (0.335, 0.127), (0.4, 0.09), (0.422, 0.059), (0.439, -0.028), (0.4, -0.09), (0.3257, -0.111), (0.156, -0.102), (0.061, -0.109), (-0.127, -0.113)], 4, 0.45, region=SW, corners=(0, 13), hatch="knit"))
    s1.append(F.sh("sweater-rib", "torso", "detail", [(-0.1, 0.125), (-0.1, -0.113)], 4.1, 0.1, region=SW, closed=False, filled=False, smooth=False))
    s1.append(F.sh("neck", "neck", "body", hp.NECK100, 3.8, 0.25, region=SKIN, corners=(0, 5), raw=True))
    s1.append(F.sh("cowl", "neck", "body", hp.COWL, 4.5, 0.45, region=SW, raw=True))
    s1 += hp.head(F, 100, K, 5, SKIN, HAIR)
    s1.append(F.ell("hair-bun", "head", "body", 0.1566, 0.0933, 0.0378, 0.035, 5.6, 0.75, region=HAIR, raw=True))
    s1.append(F.sh("hair-tie", "head", "detail", [(0.128, 0.0933), (0.128, 0.1)], 5.65, 0.1, region=HAIR, closed=False, filled=False, raw=True, smooth=False))
    s1 += notebook(F, 100)
    R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150},
                "pens": {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
                "hatches": [{"id": "knit", "title": T("Knitted sweater hatching", "Schraffur Strickpullover"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1}],
                "geometry": {"elevation": s1}}
    return R
