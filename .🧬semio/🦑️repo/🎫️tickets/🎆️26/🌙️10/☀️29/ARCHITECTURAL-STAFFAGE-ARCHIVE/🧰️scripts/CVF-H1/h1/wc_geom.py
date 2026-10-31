"""walking-commuter: hand-authored 1:500 / 1:200 / 1:100 geometry in figure coordinates (s along bone, f forward), converted to bone-local.
Raw feet/hands use bone-local coordinates. 1:50 comes from the v1 detail drawing (re-bound, lod removed)."""
from lib import *

SKIN, HAIR, COAT, SCARF, TROU, SHOE, BAG, HAND = "face-skin", "hair", "coat", "scarf", "trousers", "shoes", "bag-leather", "hand-skin"


def build(a, pose):
    F = Fig(a, pose, signs={"foot-l": 1, "foot-r": 1, "toe-l": 1, "toe-r": 1, "hand-l": 1, "hand-r": 1, "bag": 1})
    R = {}

    # ------------------------------------------------------------------ 1:500 (3.6 mm tall): one composed mass + legs + bag
    s5 = []
    sil = [(-0.40, 0.158), (-0.20, 0.145), (0.10, 0.130), (0.30, 0.135), (0.42, 0.115), (0.47, 0.09), (0.52, 0.105), (0.60, 0.125),
           (0.70, 0.105), (0.745, 0.02), (0.72, -0.07), (0.64, -0.105), (0.52, -0.095), (0.46, -0.08), (0.35, -0.125), (0.10, -0.118),
           (-0.20, -0.138), (-0.40, -0.146)]
    s5.append(F.sh("sil-torso", "torso", "body", sil, 5, 0.38, regions=[COAT, SKIN, HAIR, SCARF, HAND], corners=(0, 17)))
    s5.append(F.sh("sil-bag", "bag", "body", [(0.43, -0.13), (0.52, -0.135), (0.63, -0.13), (0.635, 0.0), (0.63, 0.13), (0.52, 0.135), (0.43, 0.13)], 5.5, 0.55, region=BAG, corners=(0, 6)))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s5.append(F.sh(f"sil-thigh-{sd}", f"thigh-{sd}", "body", [(-0.02, -0.095), (0.2, -0.088), (0.42, -0.062), (0.45, 0.0), (0.42, 0.064), (0.2, 0.083), (-0.02, 0.09)], z, 0.55, region=TROU, corners=()))
        s5.append(F.sh(f"sil-shank-{sd}", f"shank-{sd}", "body", [(0.0, -0.062), (0.15, -0.078), (0.3, -0.055), (0.425, -0.052), (0.425, 0.05), (0.3, 0.048), (0.1, 0.056), (0.0, 0.062)], z+0.02, 0.55, region=TROU))
        s5.append(F.sh(f"sil-shoe-{sd}", f"foot-{sd}", "body", [(-0.045, -0.02), (-0.03, -0.075), (-0.006, -0.0848), (0.1867, 0.0004), (0.225, 0.022), (0.244, 0.045), (0.23, 0.062), (0.16, 0.0626), (0.034, 0.0476), (-0.03, 0.012)], z+0.1, 0.85, region=SHOE, raw=True, corners=(2, 3)))
    R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000},
                "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [],
                "geometry": {"elevation": s5}}

    # ------------------------------------------------------------------ 1:200 (8.9 mm tall): main masses separated
    s2 = []
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s2.append(F.sh(f"sleeve-{sd}", f"arm-{sd}", "body", [(-0.01, -0.058), (0.15, -0.056), (0.33, -0.046), (0.365, 0.0), (0.33, 0.05), (0.15, 0.058), (0.0, 0.06), (-0.03, 0.0)], z, 0.35, region=COAT))
        s2.append(F.sh(f"forearm-{sd}", f"forearm-{sd}", "body", [(-0.03, 0.0), (-0.01, -0.047), (0.12, -0.044), (0.25, -0.039), (0.25, 0.04), (0.12, 0.046), (-0.01, 0.05)], z, 0.35, region=COAT, corners=(2, 3)))
        s2.append(F.sh(f"hand-{sd}", f"hand-{sd}", "body", [(-0.015, -0.03), (0.1, -0.032), (0.185, -0.015), (0.2, 0.012), (0.176, 0.034), (0.08, 0.04), (-0.015, 0.037)], z+0.2, 0.3, region=HAND, raw=True))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s2.append(F.sh(f"thigh-{sd}", f"thigh-{sd}", "body", [(-0.02, -0.095), (0.2, -0.088), (0.415, -0.062), (0.45, 0.0), (0.415, 0.066), (0.2, 0.085), (-0.02, 0.09)], z, 0.55, region=TROU))
        s2.append(F.sh(f"shank-{sd}", f"shank-{sd}", "body", [(0.0, -0.062), (-0.04, -0.03), (-0.045, 0.0), (-0.032, 0.062), (0.2, 0.052), (0.415, 0.041), (0.415, -0.043), (0.25, -0.058), (0.1, -0.076)], z+0.02, 0.55, region=TROU, corners=(5, 6)))
        s2.append(F.sh(f"shoe-{sd}", f"foot-{sd}", "body", [(-0.045, -0.02), (-0.03, -0.075), (-0.006, -0.0848), (0.19, 0.0), (0.2, 0.025), (0.16, 0.06), (0.03, 0.047), (-0.03, 0.012)], z+0.1, 0.85, region=SHOE, raw=True, corners=(2, 3)))
        s2.append(F.sh(f"shoe-toe-{sd}", f"toe-{sd}", "body", [(-0.002, 0.0), (0.045, 0.0), (0.07, 0.015), (0.07, 0.03), (0.03, 0.062), (-0.002, 0.068)], z+0.1, 0.85, region=SHOE, raw=True, corners=(0, 5)))
    s2.append(F.sh("scarf", "neck", "body", [(0.024, 0.079), (-0.007, 0.088), (-0.033, 0.074), (-0.037, 0.035), (-0.023, -0.066), (-0.009, -0.092), (0.024, -0.096), (0.052, -0.08), (0.054, -0.013), (0.047, 0.027)], 4.5, 0.5, region=SCARF, raw=True))
    s2.append(F.sh("coat", "torso", "body", [(-0.395, 0.158), (-0.30, 0.15), (-0.005, 0.128), (0.265, 0.138), (0.37, 0.125), (0.447, 0.062), (0.465, 0.02), (0.46, -0.03), (0.40, -0.1), (0.345, -0.12), (0.065, -0.118), (-0.1, -0.135), (-0.335, -0.152), (-0.40, -0.15)], 4, 0.35, region=COAT, corners=(0, 13)))
    s2.append(F.sh("coat-front", "torso", "detail", [(0.447, 0.062), (0.315, 0.062), (0.065, 0.100), (-0.39, 0.136)], 4.1, 0.1, region=COAT, closed=False, filled=False, smooth=False))
    s2.append(F.sh("head", "head", "body", [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.083, -0.079), (0.053, -0.094), (0.03, -0.106), (0.008, -0.084), (-0.02, -0.082), (-0.045, -0.06), (-0.04, -0.015), (-0.01, 0.059)], 5, 0.25, region=SKIN, raw=True))
    s2.append(F.sh("hair", "head", "body", [(0.139, -0.062), (0.172, -0.035), (0.176, 0.034), (0.16, 0.072), (0.12, 0.112), (0.071, 0.121), (0.021, 0.096), (0.013, 0.072), (0.06, 0.07), (0.1, 0.055), (0.125, 0.015)], 5.5, 0.75, region=HAIR, raw=True))
    s2.append(F.sh("bag-body", "bag", "body", [(0.435, -0.145), (0.575, -0.148), (0.625, -0.14), (0.64, 0.0), (0.625, 0.135), (0.575, 0.145), (0.435, 0.145), (0.425, 0.0)], 5.5, 0.55, region=BAG, corners=(0, 2, 4, 6)))
    s2.append(F.sh("bag-flap", "bag", "detail", [(0.435, -0.147), (0.525, -0.14), (0.525, 0.14), (0.435, 0.147)], 5.55, 0.4, region=BAG, closed=False, filled=False, smooth=False))
    s2.append(F.sh("bag-strap", "bag", "detail", [(0.0, 0.0), (0.425, 0.12)], 5.4, 0.2, region=BAG, closed=False, filled=False, raw=True))
    R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350},
                "pens": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.2}, "hatches": [],
                "geometry": {"elevation": s2}}

    # ------------------------------------------------------------------ 1:100 (17.8 mm tall): articulated clothing, mitten hands, shoes
    s1 = []
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s1.append(F.sh(f"sleeve-{sd}", f"arm-{sd}", "body", [(-0.007, -0.058), (0.15, -0.056), (0.325, -0.046), (0.363, 0.0), (0.325, 0.05), (0.2, 0.057), (0.075, 0.064), (0.003, 0.062), (-0.033, 0.03), (-0.033, -0.03)], z, 0.35, region=COAT))
        s1.append(F.sh(f"forearm-{sd}", f"forearm-{sd}", "body", [(-0.01, -0.047), (-0.036, -0.02), (-0.036, 0.025), (-0.01, 0.05), (0.12, 0.047), (0.25, 0.04), (0.25, -0.039), (0.12, -0.044)], z, 0.35, region=COAT, corners=(5, 6)))
        s1.append(F.sh(f"cuff-{sd}", f"forearm-{sd}", "detail", [(0.228, -0.043), (0.228, 0.043)], z+0.02, 0.1, region=COAT, closed=False, filled=False, smooth=False))
        s1.append(F.sh(f"hand-{sd}", f"hand-{sd}", "body", [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)], z+0.2, 0.3, region=HAND, raw=True))
        s1.append(F.sh(f"thumb-{sd}", f"hand-{sd}", "body", [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)], z+0.22, 0.3, region=HAND, raw=True, corners=()))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s1.append(F.sh(f"thigh-{sd}", f"thigh-{sd}", "body", [(-0.02, -0.095), (0.2, -0.088), (0.4, -0.062), (0.445, -0.02), (0.445, 0.03), (0.415, 0.066), (0.2, 0.085), (-0.025, 0.09)], z, 0.55, region=TROU))
        s1.append(F.sh(f"shank-{sd}", f"shank-{sd}", "body", [(0.0, -0.062), (-0.035, -0.04), (-0.045, 0.0), (-0.035, 0.05), (0.0, 0.064), (0.2, 0.052), (0.415, 0.041), (0.415, -0.043), (0.25, -0.06), (0.08, -0.076)], z+0.02, 0.55, region=TROU, corners=(6, 7)))
        s1.append(F.sh(f"trouser-crease-{sd}", f"shank-{sd}", "detail", [(0.02, 0.0), (0.3, 0.004), (0.4, 0.0)], z+0.03, 0.1, region=TROU, closed=False, filled=False))
        s1.append(F.sh(f"shoe-{sd}", f"foot-{sd}", "body", [(-0.0435, -0.03), (-0.03, -0.075), (-0.0063, -0.0848), (0.1867, 0.0004), (0.16, 0.0626), (0.034, 0.0476), (0.018, 0.046), (-0.028, 0.015), (-0.05, -0.012)], z+0.1, 0.85, region=SHOE, raw=True, corners=(2, 3)))
        s1.append(F.sh(f"shoe-toe-{sd}", f"toe-{sd}", "body", [(-0.001, 0.0), (0.045, 0.0), (0.068, 0.0), (0.076, 0.014), (0.062, 0.045), (0.03, 0.062), (-0.001, 0.068)], z+0.1, 0.85, region=SHOE, raw=True, corners=(0, 6)))
        s1.append(F.sh(f"shoe-sole-{sd}", f"foot-{sd}", "detail", [(-0.0236, -0.0705), (0.1777, 0.0183)], z+0.15, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
    s1.append(F.sh("neck", "neck", "body", [(0.0928, 0.0585), (0.0221, 0.0586), (-0.0133, 0.0587), (-0.0029, -0.0509), (0.0265, -0.0447), (0.0609, -0.0238)], 3.8, 0.25, region=SKIN, corners=(0, 5), raw=True))
    s1.append(F.sh("scarf", "neck", "body", [(0.0242, 0.0791), (-0.0073, 0.0878), (-0.0326, 0.0742), (-0.0373, 0.0352), (-0.0231, -0.0659), (-0.0093, -0.0922), (0.0235, -0.0957), (0.0516, -0.0796), (0.0543, -0.0126), (0.0467, 0.0268)], 4.5, 0.5, region=SCARF, raw=True))
    s1.append(F.sh("coat", "torso", "body", [(-0.395, 0.158), (-0.33, 0.152), (-0.18, 0.135), (-0.005, 0.128), (0.135, 0.133), (0.265, 0.138), (0.37, 0.125), (0.447, 0.062), (0.465, 0.02), (0.46, -0.03), (0.40, -0.1), (0.345, -0.12), (0.2, -0.115), (0.065, -0.118), (-0.1, -0.135), (-0.335, -0.152), (-0.40, -0.15)], 4, 0.35, region=COAT, corners=(0, 16), hatch="wool"))
    s1.append(F.sh("coat-lapel", "torso", "detail", [(0.447, 0.062), (0.365, 0.098), (0.315, 0.062), (0.425, 0.03)], 4.1, 0.2, region=COAT, filled=True))
    s1.append(F.sh("coat-front-edge", "torso", "detail", [(0.315, 0.062), (0.065, 0.1), (-0.39, 0.136)], 4.1, 0.1, region=COAT, closed=False, filled=False, smooth=False))
    s1.append(F.sh("coat-pocket", "torso", "detail", [(-0.135, -0.005), (-0.135, 0.105), (-0.18, 0.105), (-0.18, -0.005)], 4.1, 0.2, region=COAT, closed=False, filled=False, smooth=False))
    s1.append(F.sh("head", "head", "body", [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.084), (0.053, -0.094), (0.032, -0.107), (0.02, -0.095), (0.008, -0.084), (-0.007, -0.08), (-0.039, -0.07), (-0.05, -0.04), (-0.043, -0.016), (-0.01, 0.045)], 5, 0.25, region=SKIN, raw=True))
    s1.append(F.sh("hair", "head", "body", [(0.139, -0.062), (0.173, -0.048), (0.185, 0.034), (0.161, 0.072), (0.136, 0.114), (0.071, 0.122), (0.021, 0.096), (0.013, 0.072), (0.051, 0.076), (0.101, 0.064), (0.123, 0.024), (0.139, -0.016)], 5.5, 0.75, region=HAIR, raw=True))
    s1.append(F.ell("ear", "head", "detail", 0.046, 0.026, 0.012, 0.02, 5.2, 0.3, region=SKIN, raw=True))
    s1.append(F.sh("bag-body", "bag", "body", [(0.435, -0.145), (0.505, -0.15), (0.625, -0.14), (0.64, 0.0), (0.625, 0.135), (0.505, 0.148), (0.435, 0.145), (0.425, 0.0)], 5.5, 0.55, region=BAG, corners=(0, 2, 4, 6), hatch=None))
    s1.append(F.sh("bag-flap", "bag", "detail", [(0.435, -0.147), (0.435, 0.147), (0.525, 0.14), (0.54, 0.0), (0.525, -0.14)], 5.55, 0.4, region=BAG, filled=True, corners=(0, 1, 2, 4)))
    s1.append(F.sh("bag-buckle", "bag", "detail", [(0.5, -0.012), (0.5, 0.012), (0.53, 0.012), (0.53, -0.012)], 5.6, 0.7, region=BAG, smooth=False))
    for nm, y1 in (("front", 0.12), ("back", -0.12)):
        s1.append(F.sh(f"bag-strap-{nm}", "bag", "detail", [(-0.007, 0.0), (0.425, y1)], 5.4, 0.2, region=BAG, closed=False, filled=False, raw=True, smooth=False))
    R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150},
                "pens": {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
                "hatches": [{"id": "wool", "title": T("Wool overcoat hatching", "Schraffur Wollmantel"), "pattern": "lines", "angleDeg": 65, "spacingMm": 0.6, "penMm": 0.1}],
                "geometry": {"elevation": s1}}
    return R
