"""standing-phone-user geometry (hand-authored station tables, one function per scale)."""
from a_phone_poses import *

FACE, HAIR, HAND, HOOD, JEANS, SHOE, HP, PH = "face-skin", "hair", "hand-skin", "hoodie", "jeans", "sneakers", "headphones", "phone"
ALL = [FACE, HAIR, HAND, HOOD, JEANS, SHOE, HP, PH]
Lb = {b["id"]: b["lengthM"] for b in rig["bones"]}
sF, sA, sH = 0.88, 0.95, 0.97
dp = POSES["texting"][0]
F = Fig(a, {"bones": dp, "root": {}}, signs={"foot-l": 1, "foot-r": 1, "toe-l": 1, "toe-r": 1})


def sg(bone):
    return F.sgn(bone)


def S(id, bone, role, pts, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, raw=False, smooth=True, hatch=None, pen=None):
    return F.sh(id, bone, role, pts, z, tone, region=region, regions=regions, closed=closed, corners=corners, filled=filled, raw=raw, smooth=smooth, hatch=hatch, pen=pen)


# ---- shared tables (metres, s along the bone, f forward / raw local y)
TORSO_F = [(-0.17, 0.088), (-0.06, 0.093), (0.08, 0.086), (0.22, 0.104), (0.33, 0.098), (0.40, 0.075), (0.4422, 0.05)]
TORSO_B = [(-0.17, 0.098), (-0.05, 0.104), (0.10, 0.094), (0.28, 0.102), (0.38, 0.092), (0.4422, 0.05)]
HEAD = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.084), (0.053, -0.092), (0.034, -0.101), (0.022, -0.093), (0.008, -0.082), (-0.007, -0.076), (-0.039, -0.066), (-0.05, -0.04), (-0.043, -0.016), (-0.01, 0.045)]
HAIRCAP = [(0.139, -0.058), (0.172, -0.04), (0.183, 0.034), (0.17, 0.075), (0.13, 0.118), (0.07, 0.126), (0.02, 0.10), (0.0, 0.07), (0.05, 0.07), (0.1, 0.06), (0.125, 0.02)]
PONY = [(0.10, 0.10), (0.06, 0.15), (-0.02, 0.19), (-0.14, 0.20), (-0.20, 0.17), (-0.12, 0.13), (-0.03, 0.11), (0.04, 0.10)]
HAND100 = [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)]
THUMB = [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)]
SHOEPTS = [(-0.052, 0.022), (-0.056, -0.035), (-0.042, -0.078), (-0.0109, -0.0868), (0.1858, -0.002), (0.19, 0.012), (0.13, 0.044), (0.06, 0.05), (0.028, 0.078), (-0.03, 0.05)]
TOEBOX = [(0.0, -0.002), (0.045, -0.002), (0.066, 0.004), (0.073, 0.018), (0.062, 0.034), (0.03, 0.042), (0.0, 0.05)]


THIGH_F = [(-0.03, 0.078), (0.10, 0.08), (0.26, 0.066), (0.4133, 0.052), (0.445, 0.048)]
THIGH_B = [(-0.03, 0.088), (0.10, 0.088), (0.26, 0.066), (0.4133, 0.05), (0.445, 0.046)]
THIGH_F5 = [(-0.03, 0.078), (0.26, 0.068), (0.445, 0.052)]
THIGH_B5 = [(-0.03, 0.088), (0.26, 0.068), (0.445, 0.05)]
SHANK_F = [(-0.035, 0.052), (0.12, 0.055), (0.28, 0.045), (0.4038, 0.04), (0.425, 0.038)]
SHANK_B = [(-0.035, 0.054), (0.12, 0.064), (0.28, 0.046), (0.4038, 0.04), (0.425, 0.038)]
SHANK_F5 = [(-0.035, 0.054), (0.2, 0.052), (0.425, 0.042)]
SHANK_B5 = [(-0.035, 0.058), (0.2, 0.058), (0.425, 0.042)]
SLEEVE_F = [(-0.03, 0.058), (0.12, 0.057), (0.25, 0.05), (0.3135, 0.047), (0.335, 0.045)]
SLEEVE_B = [(-0.03, 0.062), (0.12, 0.058), (0.25, 0.05), (0.3135, 0.047), (0.335, 0.045)]
SLEEVE_F5 = [(-0.03, 0.058), (0.335, 0.05)]
SLEEVE_B5 = [(-0.03, 0.062), (0.335, 0.05)]
FORE_F = [(-0.035, 0.046), (0.12, 0.045), (0.2, 0.041), (0.25, 0.042), (0.265, 0.04)]
FORE_B = [(-0.035, 0.046), (0.12, 0.045), (0.2, 0.041), (0.25, 0.042), (0.265, 0.04)]
FORE_F5 = [(-0.035, 0.05), (0.265, 0.05)]
FORE_B5 = [(-0.035, 0.05), (0.265, 0.05)]

def make_phone(den):
    return {"kind": "rect", "x": -0.074, "y": -0.005, "w": 0.148, "h": 0.010, "r": 0.003}


def rep_500():
    s = []
    HEAD500 = [(0.0, 0.075), (0.05, 0.118), (0.12, 0.128), (0.172, 0.08), (0.186, 0.0), (0.14, -0.062), (0.1, -0.078), (0.065, -0.084), (0.034, -0.1), (0.01, -0.08), (-0.04, -0.066), (-0.05, -0.03), (-0.01, 0.0)]
    s.append(S("sil-pony", "head", "body", scl(PONY, sH), 5.4, 0.5, region=HAIR, raw=True))
    s.append(S("sil-head", "head", "body", scl(HEAD500, sH), 5, 0.4, regions=[FACE, HAIR], raw=True))
    s.append(S("sil-body", "torso", "body", fb(TORSO_F[:-1] + [(0.43, 0.06), (0.4422, 0.03)], TORSO_B[:-1] + [(0.43, 0.06), (0.4422, 0.03)]), 4, 0.4, regions=[HOOD, HP, PH]))
    s.append(S("sil-arm", "arm-r", "body", fb(SLEEVE_F5, SLEEVE_B5), 6, 0.4, region=HOOD))
    s.append(S("sil-forearm", "forearm-r", "body", fb(FORE_F5, FORE_B5), 6, 0.4, region=HOOD))
    s.append(S("sil-hand", "hand-r", "body", scl(HAND100, sA), 6.3, 0.3, region=HAND, raw=True))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"sil-thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F5, THIGH_B5), z, 0.6, region=JEANS))
        s.append(S(f"sil-shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F5, SHANK_B5), z + 0.02, 0.6, region=JEANS))
        s.append(S(f"sil-shoe-{sd}", f"foot-{sd}", "body", scl(SHOEPTS, sF), z + 0.1, 0.25, region=SHOE, raw=True, corners=(3, 4)))
        s.append(S(f"sil-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.25, region=SHOE, raw=True))
    return s


def hands(sd, z, thumb, tone=0.3, pre="hand"):
    out = [S(f"{pre}-{sd}", f"hand-{sd}", "body", scl(HAND100, sA), z, tone, region=HAND, raw=True)]
    if thumb:
        out.append(S(f"thumb-{sd}", f"hand-{sd}", "body", scl(THUMB, sA), z + 0.02, tone, region=HAND, raw=True))
    return out


def phone_line(den_pen):
    return {"kind": "poly", "points": [[-0.074, 0.0], [0.074, 0.0]], "closed": False}


def rep_200():
    s = []
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"sleeve-{sd}", f"arm-{sd}", "body", fb(SLEEVE_F5, SLEEVE_B5), z, 0.45, region=HOOD))
        s.append(S(f"forearm-{sd}", f"forearm-{sd}", "body", fb(FORE_F5, FORE_B5), z, 0.45, region=HOOD, corners=(3, 4)))
        s += hands(sd, z + 0.2, False)
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F5, THIGH_B5), z, 0.62, region=JEANS))
        s.append(S(f"shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F5, SHANK_B5), z + 0.02, 0.62, region=JEANS, corners=(3, 4)))
        s.append(S(f"shoe-{sd}", f"foot-{sd}", "body", scl(SHOEPTS, sF), z + 0.1, 0.15, region=SHOE, raw=True, corners=(3, 4)))
        s.append(S(f"shoe-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.15, region=SHOE, raw=True, corners=(0, 6)))
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.05), (0.095, 0.04)], [(-0.02, 0.054), (0.095, 0.042)]), 3.8, 0.25, region=FACE))
    s.append(S("hoodie", "torso", "body", fb(TORSO_F[::2] + [TORSO_F[-1]], TORSO_B[::2] + [TORSO_B[-1]]), 4, 0.45, region=HOOD, corners=(0, 8)))
    s.append(S("hood", "torso", "body", [(0.36, -0.088), (0.42, -0.125), (0.49, -0.135), (0.535, -0.105), (0.52, -0.065), (0.46, -0.045), (0.40, -0.06)], 3.9, 0.4, region=HOOD))
    s.append(S("pocket", "torso", "detail", [(0.02, 0.092), (0.0, 0.0), (0.02, -0.092)], 4.1, 0.1, region=HOOD, closed=False, filled=False, smooth=False))
    s.append(S("headphones", "neck", "body", [(0.0, -0.02), (0.075, -0.02), (0.075, 0.05), (0.0, 0.05)], 4.4, 0.85, region=HP, smooth=True))
    s.append(S("head", "head", "body", scl(HEAD[::2], sH), 5, 0.25, region=FACE, raw=True))
    s.append(S("hair", "head", "body", scl(HAIRCAP, sH) + [], 5.5, 0.75, region=HAIR, raw=True))
    s.append(S("ponytail", "head", "body", scl(PONY, sH), 5.4, 0.75, region=HAIR, raw=True))
    s.append(S("phone", "phone", "detail", [(-0.074, 0.0), (0.074, 0.0)], 6.5, 0.9, region=PH, closed=False, filled=False, smooth=False, raw=True, pen=0.18))
    return s


def rep_100():
    s = []
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"sleeve-{sd}", f"arm-{sd}", "body", fb(SLEEVE_F, SLEEVE_B), z, 0.45, region=HOOD))
        s.append(S(f"forearm-{sd}", f"forearm-{sd}", "body", fb(FORE_F, FORE_B), z, 0.45, region=HOOD, corners=(3, 4)))
        s.append(S(f"cuff-{sd}", f"forearm-{sd}", "detail", [(0.215, -0.043), (0.215, 0.043)], z + 0.02, 0.1, region=HOOD, closed=False, filled=False, smooth=False))
        s += hands(sd, z + 0.2, True)
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F, THIGH_B), z, 0.62, region=JEANS, hatch="denim"))
        s.append(S(f"shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F, SHANK_B), z + 0.02, 0.62, region=JEANS, corners=(3, 4), hatch="denim"))
        s.append(S(f"jeans-seam-{sd}", f"thigh-{sd}", "detail", [(0.06, 0.0), (0.2, 0.002), (0.4, 0.0)], z + 0.03, 0.1, region=JEANS, closed=False, filled=False))
        s.append(S(f"shoe-{sd}", f"foot-{sd}", "body", scl(SHOEPTS, sF), z + 0.1, 0.12, region=SHOE, raw=True, corners=(3, 4)))
        s.append(S(f"shoe-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.12, region=SHOE, raw=True, corners=(0, 6)))
        s.append(S(f"sole-{sd}", f"foot-{sd}", "detail", scl([(-0.0109, -0.0868), (0.1858, -0.002)], sF), z + 0.15, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.05), (0.095, 0.04)], [(-0.02, 0.054), (0.095, 0.042)]), 3.8, 0.25, region=FACE))
    s.append(S("hoodie", "torso", "body", fb(TORSO_F, TORSO_B), 4, 0.45, region=HOOD, corners=(0, 12)))
    s.append(S("hood", "torso", "body", [(0.36, -0.088), (0.42, -0.125), (0.49, -0.135), (0.535, -0.105), (0.52, -0.065), (0.46, -0.045), (0.40, -0.06)], 3.9, 0.4, region=HOOD))
    s.append(S("hem-rib", "torso", "detail", [(-0.135, -0.098), (-0.135, 0.088)], 4.1, 0.1, region=HOOD, closed=False, filled=False, smooth=False))
    s.append(S("pocket", "torso", "detail", [(0.02, 0.092), (-0.0, 0.0), (0.02, -0.092)], 4.1, 0.1, region=HOOD, closed=False, filled=False, smooth=False))
    s.append(S("collar", "torso", "detail", [(0.40, 0.075), (0.43, 0.04), (0.4422, 0.0)], 4.1, 0.1, region=HOOD, closed=False, filled=False))
    s.append(S("headphone-band", "neck", "detail", [(0.0, -0.05), (0.04, -0.062), (0.085, -0.04)], 4.3, 0.3, region=HP, closed=False, filled=False))
    s.append(S("headphone-cup", "neck", "body", [(0.015, -0.01), (0.04, -0.02), (0.06, 0.0), (0.04, 0.02)], 4.4, 0.85, region=HP))
    s.append(S("head", "head", "body", scl(HEAD, sH), 5, 0.25, region=FACE, raw=True))
    s.append(S("hair", "head", "body", scl(HAIRCAP, sH), 5.5, 0.75, region=HAIR, raw=True))
    s.append(S("ponytail", "head", "body", scl(PONY, sH), 5.4, 0.75, region=HAIR, raw=True))
    s.append(F.ell("ear", "head", "detail", 0.046 * sH, 0.026 * sH, 0.012, 0.02, 5.2, 0.3, region=FACE, raw=True))
    s.append(S("phone", "phone", "detail", [(-0.074, 0.0), (0.074, 0.0)], 6.5, 0.9, region=PH, closed=False, filled=False, smooth=False, raw=True, pen=0.2))
    return s


def rep_50():
    drop = {"phone", "hem-rib", "pocket", "cuff-l", "cuff-r", "sole-l", "sole-r", "collar", "jeans-seam-l", "jeans-seam-r", "headphone-cup", "headphone-band"}
    s = [x for x in rep_100() if x["id"] not in drop]
    for x in s:
        if x["id"] in ("hoodie", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r"):
            x["hatch"] = "knit"
        if x["id"].startswith(("thigh", "shank")):
            x["hatch"] = "denim"
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"cuff-rib-{sd}", f"forearm-{sd}", "body", fb([(0.212, 0.043), (0.25, 0.043)], [(0.212, 0.043), (0.25, 0.043)]), z + 0.03, 0.35, region=HOOD, smooth=False))
        for k, y in enumerate((0.016, 0.0, -0.016)):
            s.append(S(f"finger-{sd}-{k}", f"hand-{sd}", "texture", scl([(0.12, y + 0.004), (0.188, y + 0.01)], sA), z + 0.25, 0.1, region=HAND, closed=False, filled=False, raw=True, smooth=False))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"sole-{sd}", f"foot-{sd}", "body", scl([(-0.043, -0.074), (-0.0109, -0.0868), (0.1858, -0.002), (0.188, 0.008), (0.012, -0.068)], sF), z + 0.15, 0.88, region=SHOE, raw=True, corners=(1, 2)))
        s.append(S(f"toe-sole-{sd}", f"toe-{sd}", "body", scl([(0.0, -0.002), (0.066, -0.002), (0.073, 0.008), (0.0, 0.01)], sF), z + 0.15, 0.88, region=SHOE, raw=True))
        for k, (u, v0, v1) in enumerate(((0.045, 0.02, 0.058), (0.085, 0.022, 0.046), (0.122, 0.018, 0.04))):
            s.append(S(f"lace-{sd}-{k}", f"foot-{sd}", "texture", scl([(u, v0 + 0.01), (u + 0.014, v1 + 0.01)], sF), z + 0.2, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
        s.append(S(f"jeans-side-seam-{sd}", f"thigh-{sd}", "detail", [(0.0, 0.05), (0.2, 0.04), (0.4, 0.03)], z + 0.03, 0.1, region=JEANS, closed=False, filled=False))
        s.append(S(f"jeans-knee-{sd}", f"thigh-{sd}", "texture", [(0.36, -0.03), (0.39, 0.0), (0.37, 0.03)], z + 0.03, 0.1, region=JEANS, closed=False, filled=False))
        s.append(S(f"jeans-hem-{sd}", f"shank-{sd}", "detail", [(0.375, -0.041), (0.375, 0.04)], z + 0.03, 0.1, region=JEANS, closed=False, filled=False, smooth=False))
    s.append(S("hem-rib", "torso", "body", fb([(-0.17, 0.088), (-0.13, 0.091)], [(-0.17, 0.098), (-0.13, 0.102)]), 4.1, 0.35, region=HOOD, smooth=False))
    s.append(S("kangaroo-pocket", "torso", "detail", [(0.0, 0.088), (-0.13, 0.092), (-0.12, -0.098), (0.0, -0.095), (0.03, 0.0)], 4.1, 0.45, region=HOOD, closed=True, filled=False, smooth=False))
    for k, y in enumerate((0.03, 0.06)):
        s.append(S(f"drawstring-{k}", "torso", "detail", [(0.40, y), (0.30, y + 0.006), (0.22, y + 0.003)], 4.15, 0.1, region=HOOD, closed=False, filled=False))
    s.append(S("collar-rib", "torso", "detail", [(0.40, 0.075), (0.43, 0.04), (0.4422, 0.0)], 4.1, 0.1, region=HOOD, closed=False, filled=False))
    s.append(S("hood-fold", "torso", "texture", [(0.40, -0.09), (0.46, -0.1), (0.5, -0.09)], 4.0, 0.1, region=HOOD, closed=False, filled=False))
    s.append(S("headphone-band", "neck", "detail", [(0.0, -0.05), (0.04, -0.062), (0.085, -0.04)], 4.3, 0.3, region=HP, closed=False, filled=False, pen=0.2))
    s.append(S("headphone-cup", "neck", "body", [(0.012, -0.012), (0.04, -0.024), (0.064, 0.0), (0.04, 0.024)], 4.4, 0.85, region=HP))
    s.append(S("headphone-cushion", "neck", "detail", [(0.02, 0.016), (0.044, 0.026), (0.058, 0.01)], 4.45, 0.4, region=HP, closed=False, filled=False))
    s.append(F.ell("eye", "head", "detail", 0.085 * sH, -0.068 * sH, 0.008, 0.005, 5.3, 0.9, region=FACE, raw=True))
    s.append(S("brow", "head", "texture", scl([(0.098, -0.07), (0.1, -0.058), (0.094, -0.046)], sH), 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True))
    s.append(S("mouth", "head", "texture", scl([(0.014, -0.078), (0.011, -0.062)], sH), 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True, smooth=False))
    for k, (s0, y0) in enumerate(((0.15, 0.03), (0.13, 0.07), (0.09, 0.1))):
        s.append(S(f"hair-strand-{k}", "head", "texture", scl([(s0, y0 - 0.04), (s0 - 0.03, y0 + 0.01)], sH), 5.6, 0.1, region=HAIR, closed=False, filled=False, raw=True))
    s.append(S("pony-tie", "head", "detail", scl([(0.075, 0.1), (0.085, 0.14)], sH), 5.6, 0.1, region=HAIR, closed=False, filled=False, raw=True, smooth=False))
    s.append({"id": "phone", "bone": "phone", "role": "body", "z": 6.5, "filled": True, "tone": 0.92, "kind": "rect", "x": -0.074, "y": -0.005, "w": 0.148, "h": 0.010, "r": 0.003, "region": PH})
    s.append(S("phone-screen", "phone", "detail", [(-0.066, 0.0055), (0.066, 0.0055)], 6.6, 0.1, region=PH, closed=False, filled=False, raw=True, smooth=False))
    return s
