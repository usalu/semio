"""courier-with-hand-truck geometry: one function per drawing scale (hand-authored station tables)."""
from a_courier_poses import *

FACE, CAP, GLOVE, JACKET, TROU, BOOT, TRUCK, TYRE, PARC = "face-skin", "cap", "gloves", "jacket", "trousers", "boots", "hand-truck", "tyres", "parcels"
sF, sA, sH = 1.07, 1.02, 1.0
dp = POSES["waiting"][0]
F = Fig(a, {"bones": dp, "root": {}}, signs={"foot-l": 1, "foot-r": 1, "toe-l": 1, "toe-r": 1})


def S(id, bone, role, pts, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, raw=False, smooth=True, hatch=None, pen=None):
    return F.sh(id, bone, role, pts, z, tone, region=region, regions=regions, closed=closed, corners=corners, filled=filled, raw=raw, smooth=smooth, hatch=hatch, pen=pen)


TORSO_F = [(-0.19, 0.105), (-0.07, 0.108), (0.10, 0.100), (0.24, 0.118), (0.36, 0.112), (0.44, 0.085), (0.4773, 0.055)]
TORSO_B = [(-0.19, 0.112), (-0.05, 0.118), (0.12, 0.108), (0.30, 0.115), (0.42, 0.098), (0.4773, 0.055)]
SLEEVE_F = [(-0.03, 0.064), (0.12, 0.064), (0.26, 0.056), (0.3366, 0.052), (0.358, 0.05)]
SLEEVE_B = [(-0.03, 0.07), (0.12, 0.066), (0.26, 0.056), (0.3366, 0.052), (0.358, 0.05)]
SLEEVE_F5 = [(-0.03, 0.064), (0.358, 0.056)]
SLEEVE_B5 = [(-0.03, 0.07), (0.358, 0.056)]
FORE_F = [(-0.04, 0.054), (0.13, 0.052), (0.22, 0.048), (0.2652, 0.05), (0.285, 0.048)]
FORE_B = [(-0.04, 0.054), (0.13, 0.052), (0.22, 0.048), (0.2652, 0.05), (0.285, 0.048)]
FORE_F5 = [(-0.04, 0.056), (0.285, 0.054)]
FORE_B5 = [(-0.04, 0.056), (0.285, 0.054)]
THIGH_F = [(-0.03, 0.088), (0.12, 0.09), (0.30, 0.076), (0.4437, 0.06), (0.4737, 0.056)]
THIGH_B = [(-0.03, 0.098), (0.12, 0.098), (0.30, 0.076), (0.4437, 0.058), (0.4737, 0.054)]
THIGH_F5 = [(-0.03, 0.09), (0.30, 0.078), (0.4737, 0.058)]
THIGH_B5 = [(-0.03, 0.098), (0.30, 0.078), (0.4737, 0.056)]
SHANK_F = [(-0.04, 0.06), (0.14, 0.062), (0.30, 0.054), (0.4335, 0.05), (0.455, 0.048)]
SHANK_B = [(-0.04, 0.062), (0.14, 0.074), (0.30, 0.056), (0.4335, 0.05), (0.455, 0.048)]
SHANK_F5 = [(-0.04, 0.062), (0.2, 0.06), (0.455, 0.05)]
SHANK_B5 = [(-0.04, 0.064), (0.2, 0.068), (0.455, 0.05)]
HEAD = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.086), (0.053, -0.097), (0.032, -0.108), (0.02, -0.097), (0.008, -0.086), (-0.007, -0.082), (-0.039, -0.074), (-0.052, -0.045), (-0.045, -0.016), (-0.01, 0.045)]
CAPDOME = [(0.055, 0.098), (0.12, 0.118), (0.172, 0.095), (0.19, 0.02), (0.172, -0.05), (0.135, -0.076), (0.098, -0.082), (0.075, -0.03), (0.06, 0.03)]
CAPBRIM = [(0.098, -0.08), (0.116, -0.14), (0.095, -0.155), (0.072, -0.12), (0.06, -0.085)]
HAND100 = [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)]
THUMB = [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)]
BOOTPTS = [(-0.058, 0.07), (-0.064, -0.035), (-0.048, -0.08), (-0.0117, -0.0929), (0.1988, -0.002), (0.205, 0.015), (0.15, 0.05), (0.07, 0.062), (0.03, 0.095), (-0.03, 0.075)]
TOEBOX = [(0.0, -0.002), (0.05, -0.002), (0.072, 0.004), (0.08, 0.02), (0.066, 0.038), (0.03, 0.046), (0.0, 0.056)]
GL = 1.12


def glove(sd, z, thumb=True, pre="glove"):
    out = [S(f"{pre}-{sd}", f"hand-{sd}", "body", scl(HAND100, sA * GL), z, 0.4, region=GLOVE, raw=True)]
    if thumb:
        out.append(S(f"{pre}-thumb-{sd}", f"hand-{sd}", "body", scl(THUMB, sA * GL), z + 0.02, 0.4, region=GLOVE, raw=True))
    return out


def truck_common(den):
    """truck, wheel and parcels; den selects the construction detail"""
    s = []
    R = True
    # parcels in the frame coordinates of the parcels bone (s up the frame, f negative to the front)
    A = [(0.06, -0.02), (0.46, -0.02), (0.46, -0.42), (0.06, -0.42)]
    B = [(0.46, -0.02), (0.76, -0.02), (0.76, -0.38), (0.46, -0.38)]
    C = [(0.76, -0.02), (0.98, -0.02), (0.98, -0.30), (0.76, -0.30)]
    for nm, pts, tone in (("parcel-a", A, 0.52), ("parcel-b", B, 0.45), ("parcel-c", C, 0.6)):
        s.append(S(nm, "parcels", "body", pts, 2.0, tone, region=PARC, raw=R, smooth=False))
    if den <= 100:
        for nm, (p0, p1) in (("tape-a", ((0.06, -0.22), (0.46, -0.22))), ("tape-b", ((0.46, -0.20), (0.76, -0.20))), ("tape-c", ((0.76, -0.16), (0.98, -0.16)))):
            s.append(S(nm, "parcels", "detail", [p0, p1], 2.1, 0.2, region=PARC, closed=False, filled=False, raw=R, smooth=False))
    if den <= 50:
        s.append(S("label-a", "parcels", "texture", [(0.30, -0.30), (0.40, -0.30), (0.40, -0.38), (0.30, -0.38)], 2.2, 0.9, region=PARC, raw=R, smooth=False))
        s.append(S("strap-b", "parcels", "texture", [(0.52, -0.02), (0.52, -0.38)], 2.2, 0.2, region=PARC, closed=False, filled=False, raw=R, smooth=False))
    return s


def truck_frame(den):
    s = []
    R = True
    pen = None
    if den == 500:
        s.append(S("rail", "hand-truck", "body", [(-0.08, 0.0), (1.02, 0.0)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False, pen=0.13))
        s.append(S("plate", "hand-truck", "body", [(0.045, 0.0), (0.045, -0.30)], 2.4, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False, pen=0.13))
        s.append(S("handle", "hand-truck", "body", [(0.98, 0.0), (1.04, 0.07), (1.0, 0.13)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False, pen=0.13))
    elif den == 200:
        s.append(S("rail", "hand-truck", "body", [(-0.08, 0.0), (1.02, 0.0)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False, pen=0.18))
        s.append(S("plate", "hand-truck", "body", [(0.045, 0.0), (0.045, -0.30)], 2.4, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False, pen=0.18))
        s.append(S("handle", "hand-truck", "body", [(0.98, 0.0), (1.05, 0.06), (1.04, 0.12), (0.98, 0.14)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, pen=0.18))
        s.append(S("brace", "hand-truck", "detail", [(0.0, 0.0), (0.0, -0.1), (0.045, -0.3)], 2.45, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False))
    else:
        w = 0.028
        s.append(S("rail", "hand-truck", "body", [(-0.08, -w / 2), (1.0, -w / 2), (1.0, w / 2), (-0.08, w / 2)], 2.5, 0.35, region=TRUCK, raw=R, smooth=False))
        s.append(S("plate", "hand-truck", "body", [(0.035, 0.0), (0.035, -0.30), (0.06, -0.30), (0.06, 0.0)], 2.4, 0.35, region=TRUCK, raw=R, smooth=False))
        s.append(S("handle", "hand-truck", "body", [(0.96, 0.0), (1.04, 0.05), (1.06, 0.1), (1.03, 0.14), (0.97, 0.15)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, pen=0.25 if den == 50 else 0.22))
        s.append(S("cross-rail", "hand-truck", "detail", [(0.5, 0.0), (0.5, 0.0)], 2.5, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False)) if False else None
        s.append(S("brace", "hand-truck", "detail", [(0.02, 0.0), (0.02, -0.12), (0.06, -0.29)], 2.45, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False))
        s.append(S("axle-stay", "hand-truck", "detail", [(0.0, 0.0), (0.0, 0.07)], 2.45, 0.1, region=TRUCK, closed=False, filled=False, raw=R, smooth=False))
        s = [x for x in s if x]
    return s


def wheel(den):
    s = [F.ell("tyre", "truck-lift", "body", 0.0, 0.0, 0.13, 0.13, 2.6, 0.85, region=TYRE, raw=True)]
    if den <= 100:
        s.append(F.ell("hub", "truck-lift", "detail", 0.0, 0.0, 0.055, 0.055, 2.65, 0.5, region=TYRE, raw=True))
    if den <= 50:
        s.append(F.ell("hub-cap", "truck-lift", "texture", 0.0, 0.0, 0.02, 0.02, 2.7, 0.1, region=TYRE, raw=True))
        for k in range(6):
            ang = math.radians(60 * k + 15)
            s.append(S(f"tread-{k}", "truck-lift", "texture", [(0.118 * math.cos(ang), 0.118 * math.sin(ang)), (0.13 * math.cos(ang), 0.13 * math.sin(ang))], 2.7, 0.1, region=TYRE, closed=False, filled=False, raw=True, smooth=False))
    return s


def rep_500():
    s = []
    s.append(S("sil-head", "head", "body", scl([(0.0, 0.075), (0.05, 0.112), (0.12, 0.122), (0.172, 0.095), (0.19, 0.02), (0.14, -0.05), (0.1, -0.1), (0.116, -0.14), (0.075, -0.125), (0.03, -0.108), (0.01, -0.085), (-0.04, -0.07), (-0.05, -0.03), (-0.01, 0.0)], sH), 5, 0.4, regions=[FACE, CAP], raw=True))
    pb = fb(TORSO_F[:-1] + [(0.45, 0.07), (0.4773, 0.04)], TORSO_B[:-1] + [(0.45, 0.07), (0.4773, 0.04)])
    s.append(S("sil-body", "torso", "body", pb, 4, 0.45, region=JACKET, corners=(0, len(pb) - 1)))
    s.append(S("sil-arm", "arm-r", "body", fb(SLEEVE_F5, SLEEVE_B5), 6, 0.45, region=JACKET))
    s.append(S("sil-forearm", "forearm-r", "body", fb(FORE_F5, FORE_B5), 6, 0.45, region=JACKET))
    s.append(S("sil-glove", "hand-r", "body", scl(HAND100, sA * GL), 6.3, 0.35, region=GLOVE, raw=True))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"sil-thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F5, THIGH_B5), z, 0.6, region=TROU))
        s.append(S(f"sil-shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F5, SHANK_B5), z + 0.02, 0.6, region=TROU))
        s.append(S(f"sil-boot-{sd}", f"foot-{sd}", "body", scl(BOOTPTS, sF), z + 0.1, 0.2, region=BOOT, raw=True, corners=(3, 4)))
        s.append(S(f"sil-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.2, region=BOOT, raw=True))
    return s + truck_common(500) + truck_frame(500) + wheel(500)


def arms_sleeves(den, z0, z1):
    s = []
    for sd, z in (("l", z0), ("r", z1)):
        fs, bs = (SLEEVE_F5, SLEEVE_B5) if den == 200 else (SLEEVE_F, SLEEVE_B)
        ff, fbk = (FORE_F5, FORE_B5) if den == 200 else (FORE_F, FORE_B)
        s.append(S(f"sleeve-{sd}", f"arm-{sd}", "body", fb(fs, bs), z, 0.45, region=JACKET))
        s.append(S(f"forearm-{sd}", f"forearm-{sd}", "body", fb(ff, fbk), z, 0.45, region=JACKET, corners=(3, 4) if den != 200 else ()))
    return s


def legs(den, hatch=None, details=False):
    s = []
    for sd, z in (("l", 1.0), ("r", 3.0)):
        tf, tb_ = (THIGH_F5, THIGH_B5) if den == 200 else (THIGH_F, THIGH_B)
        kf, kb = (SHANK_F5, SHANK_B5) if den == 200 else (SHANK_F, SHANK_B)
        s.append(S(f"thigh-{sd}", f"thigh-{sd}", "body", fb(tf, tb_), z, 0.62, region=TROU, hatch=hatch))
        s.append(S(f"shank-{sd}", f"shank-{sd}", "body", fb(kf, kb), z + 0.02, 0.62, region=TROU, corners=(3, 4) if den != 200 else (), hatch=hatch))
        s.append(S(f"boot-{sd}", f"foot-{sd}", "body", scl(BOOTPTS, sF), z + 0.1, 0.18, region=BOOT, raw=True, corners=(3, 4)))
        s.append(S(f"boot-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.18, region=BOOT, raw=True, corners=(0, 6)))
    return s


def rep_200():
    s = arms_sleeves(200, 0.5, 6.0)
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s += glove(sd, z + 0.2, False)
    s += legs(200)
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.056), (0.1026, 0.046)], [(-0.02, 0.06), (0.1026, 0.048)]), 3.8, 0.25, region=FACE))
    s.append(S("jacket", "torso", "body", fb(TORSO_F[::2] + [TORSO_F[-1]], TORSO_B[::2] + [TORSO_B[-1]]), 4, 0.45, region=JACKET, corners=(0, 8)))
    s.append(S("reflective-band", "torso", "detail", [(0.04, 0.109), (0.04, -0.12)], 4.1, 0.9, region=JACKET, closed=False, filled=False, smooth=False))
    s.append(S("head", "head", "body", scl(HEAD[::2], sH), 5, 0.25, region=FACE, raw=True))
    s.append(S("cap", "head", "body", scl(CAPDOME, sH), 5.5, 0.3, region=CAP, raw=True))
    s.append(S("cap-brim", "head", "body", scl(CAPBRIM, sH), 5.4, 0.3, region=CAP, raw=True))
    return s + truck_common(200) + truck_frame(200) + wheel(200)


def rep_100():
    s = arms_sleeves(100, 0.5, 6.0)
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"cuff-{sd}", f"forearm-{sd}", "detail", [(0.235, -0.052), (0.235, 0.052)], z + 0.02, 0.1, region=JACKET, closed=False, filled=False, smooth=False))
        s += glove(sd, z + 0.2, True)
    s += legs(100, hatch="cotton")
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"cargo-pocket-{sd}", f"thigh-{sd}", "detail", [(0.14, 0.088), (0.14, 0.0), (0.26, 0.0), (0.26, 0.075)], z + 0.03, 0.1, region=TROU, closed=False, filled=False, smooth=False))
        s.append(S(f"boot-sole-{sd}", f"foot-{sd}", "detail", scl([(-0.0117, -0.0929), (0.1988, -0.002)], sF), z + 0.15, 0.1, region=BOOT, closed=False, filled=False, raw=True, smooth=False))
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.056), (0.1026, 0.046)], [(-0.02, 0.06), (0.1026, 0.048)]), 3.8, 0.25, region=FACE))
    s.append(S("jacket", "torso", "body", fb(TORSO_F, TORSO_B), 4, 0.45, region=JACKET, corners=(0, 12)))
    s.append(S("reflective-band-1", "torso", "detail", [(0.04, 0.109), (0.04, -0.12)], 4.1, 0.9, region=JACKET, closed=False, filled=False, smooth=False))
    s.append(S("reflective-band-2", "torso", "detail", [(0.09, 0.103), (0.09, -0.11)], 4.1, 0.9, region=JACKET, closed=False, filled=False, smooth=False))
    s.append(S("jacket-zip", "torso", "detail", [(-0.17, 0.104), (0.2, 0.113), (0.43, 0.08)], 4.1, 0.1, region=JACKET, closed=False, filled=False))
    s.append(S("collar", "torso", "detail", [(0.44, 0.085), (0.47, 0.05), (0.4773, 0.0)], 4.1, 0.1, region=JACKET, closed=False, filled=False))
    s.append(S("head", "head", "body", scl(HEAD, sH), 5, 0.25, region=FACE, raw=True))
    s.append(S("cap", "head", "body", scl(CAPDOME, sH), 5.5, 0.3, region=CAP, raw=True))
    s.append(S("cap-brim", "head", "body", scl(CAPBRIM, sH), 5.4, 0.3, region=CAP, raw=True))
    s.append(F.ell("ear", "head", "detail", 0.046, 0.026, 0.012, 0.02, 5.2, 0.3, region=FACE, raw=True))
    return s + truck_common(100) + truck_frame(100) + wheel(100)


def rep_50():
    s = [x for x in rep_100() if x["id"] not in ("cuff-l", "cuff-r", "boot-sole-l", "boot-sole-r", "jacket-zip", "collar", "reflective-band-1", "reflective-band-2")]
    for x in s:
        if x["id"] in ("jacket", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r"):
            x["hatch"] = "softshell"
        if x["id"].startswith(("thigh", "shank")):
            x["hatch"] = "cotton"
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"cuff-strap-{sd}", f"forearm-{sd}", "body", fb([(0.225, 0.054), (0.275, 0.054)], [(0.225, 0.054), (0.275, 0.054)]), z + 0.03, 0.55, region=JACKET, smooth=False))
        for k, y in enumerate((0.016, 0.0, -0.016)):
            s.append(S(f"finger-{sd}-{k}", f"hand-{sd}", "texture", scl([(0.12, y + 0.004), (0.188, y + 0.01)], sA * GL), z + 0.25, 0.1, region=GLOVE, closed=False, filled=False, raw=True, smooth=False))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"boot-sole-{sd}", f"foot-{sd}", "body", scl([(-0.052, -0.07), (-0.0117, -0.0929), (0.1988, -0.002), (0.2, 0.01), (0.02, -0.06)], sF), z + 0.15, 0.9, region=BOOT, raw=True, corners=(1, 2)))
        s.append(S(f"boot-toe-sole-{sd}", f"toe-{sd}", "body", scl([(0.0, -0.002), (0.074, -0.002), (0.08, 0.01), (0.0, 0.012)], sF), z + 0.15, 0.9, region=BOOT, raw=True))
        for k, (u, v) in enumerate(((0.06, 0.05), (0.1, 0.045), (0.14, 0.04))):
            s.append(S(f"boot-lace-{sd}-{k}", f"foot-{sd}", "texture", scl([(u, v), (u + 0.014, v + 0.03)], sF), z + 0.2, 0.1, region=BOOT, closed=False, filled=False, raw=True, smooth=False))
        s.append(S(f"trouser-hem-{sd}", f"shank-{sd}", "detail", [(0.40, -0.05), (0.40, 0.05)], z + 0.03, 0.1, region=TROU, closed=False, filled=False, smooth=False))
        s.append(S(f"knee-patch-{sd}", f"thigh-{sd}", "detail", [(0.39, -0.055), (0.46, -0.05), (0.46, 0.058), (0.39, 0.06)], z + 0.03, 0.4, region=TROU, closed=False, filled=False, smooth=False))
    s.append(S("jacket-zip", "torso", "detail", [(-0.17, 0.104), (0.2, 0.113), (0.43, 0.08)], 4.1, 0.1, region=JACKET, closed=False, filled=False))
    for k, sv in enumerate((0.04, 0.09)):
        s.append(S(f"reflective-band-{k}", "torso", "body", fb([(sv - 0.014, 0.108), (sv + 0.014, 0.108)], [(sv - 0.014, 0.115), (sv + 0.014, 0.115)]), 4.12, 0.92, region=JACKET, smooth=False))
    s.append(S("collar", "torso", "detail", [(0.44, 0.085), (0.47, 0.05), (0.4773, 0.0)], 4.1, 0.1, region=JACKET, closed=False, filled=False))
    s.append(S("chest-pocket", "torso", "detail", [(0.28, 0.105), (0.28, 0.06), (0.36, 0.06), (0.36, 0.108)], 4.1, 0.1, region=JACKET, closed=False, filled=False, smooth=False))
    s.append(F.ell("eye", "head", "detail", 0.085, -0.07, 0.008, 0.005, 5.3, 0.9, region=FACE, raw=True))
    s.append(S("mouth", "head", "texture", [(0.014, -0.08), (0.011, -0.064)], 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True, smooth=False))
    s.append(S("cap-seam", "head", "texture", [(0.12, 0.115), (0.13, 0.0), (0.098, -0.08)], 5.6, 0.1, region=CAP, closed=False, filled=False, raw=True))
    s.append(S("cap-button", "head", "texture", [(0.188, 0.0), (0.19, 0.01)], 5.6, 0.1, region=CAP, closed=False, filled=False, raw=True, smooth=False))
    return s
