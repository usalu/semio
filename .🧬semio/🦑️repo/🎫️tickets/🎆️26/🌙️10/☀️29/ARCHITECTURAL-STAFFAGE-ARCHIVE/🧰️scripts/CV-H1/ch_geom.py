"""running-child: hand-authored 1:500 / 1:200 / 1:100 geometry (default pose standing-ready). Limb tables from human_parts scaled to child bones."""
from lib import *
import human_parts as hp

SKIN, HAIR, TS, SH, SOCK, SHOE, PACK, ARM, LEG = "face-skin", "hair", "tshirt", "shorts", "socks", "shoes", "backpack", "arm-skin", "leg-skin"
KA, KL, KH = 0.56, 0.545, 0.85
SHORTS = [(-0.0082, -0.0529), (0.0191, -0.0556), (0.0627, -0.0534), (0.1117, -0.0491), (0.1117, 0.0501), (0.0627, 0.0534), (0.0191, 0.0545), (-0.0136, 0.0491)]
SLV = [(-0.0039, -0.0325), (0.0196, -0.0336), (0.0868, -0.0302), (0.0868, 0.0336), (0.0196, 0.037), (0.0017, 0.0347), (-0.0185, 0.0224), (-0.0185, -0.0168)]
TSH = [(-0.0595, 0.087), (0.0455, 0.09), (0.1155, 0.092), (0.1925, 0.102), (0.2905, 0.076), (0.3129, 0.043), (0.3255, -0.021), (0.3185, -0.0595), (0.2415, -0.084), (0.1855, -0.0854), (0.0455, -0.0826), (-0.0595, -0.084)]
PACKP = [(0.1555, 0.046), (0.14, 0.095), (0.03, 0.1055), (-0.0405, 0.088), (-0.0685, 0.06), (-0.0545, 0.0355), (0.1415, 0.0355)]
COLLAR = [(-0.0078, 0.0555), (-0.0202, 0.0577), (-0.0304, 0.0442), (-0.0287, 0.0202), (-0.0186, -0.0511), (-0.0103, -0.0701), (0.01, -0.0713), (0.0248, -0.0611), (0.0135, -0.0385), (0.0046, 0.0249)]


def build(a, pose):
    F = Fig(a, pose)
    R = {}
    # ------------------------------------------------------------------ 1:500
    s5 = []
    sil = [(-0.09, 0.52), (-0.098, 0.6), (-0.146, 0.62), (-0.15, 0.72), (-0.14, 0.83), (-0.085, 0.86), (-0.065, 0.92), (-0.075, 1.0), (-0.06, 1.1), (0.0, 1.143), (0.07, 1.135),
           (0.115, 1.09), (0.118, 1.02), (0.1, 0.97), (0.06, 0.94), (0.075, 0.9), (0.109, 0.8), (0.108, 0.6), (0.1, 0.52)]
    s5.append(F.wsh("sil-torso", "torso", "body", sil, 5, 0.4, regions=[TS, SKIN, HAIR, PACK], corners=(0, 18)))
    s5.append(F.sh("sil-arm-r", "arm-r", "body", [(-0.0, 0.0), (0.185, 0.0)], 6, 0.25, region=ARM, closed=False, filled=False, smooth=False, raw=True, pen=0.14))
    s5.append(F.sh("sil-forearm-r", "forearm-r", "body", [(0.0, 0.0), (0.25, 0.0)], 6.1, 0.25, region=ARM, closed=False, filled=False, smooth=False, raw=True, pen=0.14))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s5.append(F.sh(f"sil-thigh-{sd}", f"thigh-{sd}", "body", hp.sc([(-0.02, -0.095), (0.2, -0.088), (0.415, -0.062), (0.45, 0.0), (0.415, 0.066), (0.2, 0.085), (-0.02, 0.09)], KL, KL * 1.1), z, 0.4, regions=[SH, LEG], raw=True))
        s5.append(F.sh(f"sil-shank-{sd}", f"shank-{sd}", "body", [(0.0, 0.0), (0.232, 0.0)], z + 0.02, 0.25, regions=[LEG, SOCK], closed=False, filled=False, smooth=False, raw=True, pen=0.14))
        s5.append(F.sh(f"sil-shoe-{sd}", f"foot-{sd}", "body", [(-0.02, -0.03), (0.04, -0.035), (0.11, 0.015)], z + 0.1, 0.85, region=SHOE, closed=False, filled=False, smooth=False, raw=True, pen=0.14))
    R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000},
                "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [], "geometry": {"elevation": s5}}
    # ------------------------------------------------------------------ 1:200 and 1:100
    for den in (200, 100):
        s = []
        tones = {"sleeve": 0.25, "hand": 0.25, "leg": 0.25, "shoe": 0.85}
        zs = (("l", 0.5), ("r", 6.0)); zl = (("l", 1.0), ("r", 3.0))
        for sd, z in zs:
            s += hp.arms(F, den, None, KA, KA, ((sd, z),), tones, {"sleeve": ARM, "hand": ARM, "leg": ARM, "shoe": ARM}, thumb=True)
            s.append(F.sh(f"sleeve-cap-{sd}", f"arm-{sd}", "body", hp.sc(SLV, 1.0, 1.04) if den == 100 else hp.sc(SLV, 1.0), z + 0.05, 0.5, region=TS, raw=True, corners=(2, 3)))
        for sd, z in zl:
            s += hp.legs(F, den, KL, KL, ((sd, z),), tones, {"leg": LEG, "shoe": SHOE}, crease=False)
            s.append(F.sh(f"shorts-{sd}", f"thigh-{sd}", "body", SHORTS, z + 0.01, 0.6, region=SH, raw=True, corners=(3, 4)))
        for s_ in s:
            if s_["id"].startswith("sleeve-") and not s_["id"].startswith("sleeve-cap"):
                s_["id"] = "arm-skin-" + s_["id"][-1]
            if s_["id"].startswith("thigh-") or s_["id"].startswith("shank-"):
                s_["region"] = LEG
        if True:
            for sd, z in zl:
                s.append(F.sh(f"sock-{sd}", f"shank-{sd}", "detail", [(0.2262 * 0.9, -0.0234), (0.2262 * 0.9, 0.0223)], z + 0.04, 0.1, region=SOCK, closed=False, filled=False, raw=True, smooth=False))
            for sd, z in zl:
                if den == 100: s.append(F.sh(f"shoe-stripe-{sd}", f"foot-{sd}", "detail", [(0.0, 0.01), (0.07, 0.035)], z + 0.16, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
        s.append(F.sh("pelvis-shorts", "pelvis", "body", [(0.0735, 0.0805), (0.0385, 0.0966), (-0.0035, 0.0945), (-0.0385, 0.077), (-0.0385, -0.07), (-0.0035, -0.091), (0.0385, -0.0924), (0.0735, -0.0826)], 3, 0.6, region=SH, raw=True, corners=(3, 4)))
        s.append(F.sh("tshirt", "torso", "body", TSH, 4, 0.5, region=TS, corners=(0, 11)))
        s.append(F.sh("tshirt-hem", "torso", "detail", [(-0.035, 0.087), (-0.035, -0.084)], 4.1, 0.1, region=TS, closed=False, filled=False, smooth=False))
        s.append(F.sh("collar", "neck", "body", COLLAR, 4.5, 0.5, region=TS, raw=True))
        s.append(F.sh("neck", "neck", "body", hp.sc(hp.NECK100, 0.8), 3.8, 0.25, region=SKIN, corners=(0, 5), raw=True))
        s += hp.head(F, den, KH, 5, SKIN, HAIR)
        s.append(F.sh("pack-body", "pack", "body", PACKP, 3.5, 0.65, region=PACK, raw=True, corners=(0, 5, 6)))
        if den == 100:
            s.append(F.sh("pack-strap", "pack", "detail", [(0.1415, 0.0355), (0.1975, -0.045), (0.1275, -0.094), (-0.0195, -0.059)], 4.4, 0.2, region=PACK, closed=False, filled=False, raw=True, smooth=False))
            s.append(F.sh("pack-pocket", "pack", "detail", [(0.0155, 0.095), (0.0155, 0.046)], 3.6, 0.1, region=PACK, closed=False, filled=False, raw=True, smooth=False))
            s.append(F.sh("tshirt-print", "torso", "detail", [(0.2, 0.0), (0.2, -0.06), (0.14, -0.06), (0.14, 0.0)], 4.2, 0.2, region=TS, filled=True, smooth=False))
        else:
            s.append(F.sh("pack-strap", "pack", "detail", [(0.1415, 0.0355), (0.1975, -0.045), (0.1275, -0.094)], 4.4, 0.2, region=PACK, closed=False, filled=False, raw=True, smooth=False))
        pens = {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.2} if den == 200 else {"body": 0.2, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}
        R[str(den)] = {"status": "authored", "denominator": den, "validRange": {"minDenominator": 150, "maxDenominator": 350} if den == 200 else {"minDenominator": 75, "maxDenominator": 150},
                       "pens": pens, "hatches": [], "geometry": {"elevation": s}}
    return R
