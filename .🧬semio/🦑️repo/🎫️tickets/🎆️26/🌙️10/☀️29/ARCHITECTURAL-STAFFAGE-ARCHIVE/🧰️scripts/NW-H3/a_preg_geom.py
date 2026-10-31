"""pregnant-person geometry: station tables and one function per scale."""
from a_preg_poses import *

FACE, HAIR, HAND, SWEAT, BELLY_R, LEG, BOOT = "face-skin", "hair", "hand-skin", "sweater", "belly-sweater", "leggings", "ankle-boots"
sF, sA, sH = 0.86, 0.94, 0.97
dp = POSES["hand-on-back"][0]
F = Fig(a, {"bones": dp, "root": {}}, signs={"foot-l": 1, "foot-r": 1, "toe-l": 1, "toe-r": 1})


def S(id, bone, role, pts, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, raw=False, smooth=True, hatch=None, pen=None):
    return F.sh(id, bone, role, pts, z, tone, region=region, regions=regions, closed=closed, corners=corners, filled=filled, raw=raw, smooth=smooth, hatch=hatch, pen=pen)


TORSO_F = [(-0.16, 0.098), (-0.08, 0.095), (0.0, 0.095), (0.12, 0.105), (0.22, 0.125), (0.30, 0.118), (0.37, 0.095), (0.42, 0.068), (0.4313, 0.05)]
TORSO_B = [(-0.16, 0.108), (-0.05, 0.112), (0.08, 0.088), (0.2, 0.092), (0.32, 0.104), (0.39, 0.092), (0.4313, 0.05)]
BELLY = [(0.04, -0.17), (0.12, -0.165), (0.19, -0.12), (0.225, -0.04), (0.225, 0.05), (0.195, 0.12), (0.13, 0.17), (0.05, 0.2), (0.0, 0.14), (0.0, -0.14)]
BELLY5 = [(0.02, -0.17), (0.14, -0.16), (0.215, -0.08), (0.226, 0.02), (0.19, 0.13), (0.1, 0.19), (0.0, 0.18), (0.0, -0.15)]
SLEEVE_F = [(-0.03, 0.055), (0.12, 0.056), (0.25, 0.05), (0.3102, 0.046), (0.33, 0.044)]
SLEEVE_B = [(-0.03, 0.06), (0.12, 0.06), (0.25, 0.052), (0.3102, 0.048), (0.33, 0.046)]
SLEEVE_F5 = [(-0.02, 0.052), (0.33, 0.05)]
SLEEVE_B5 = [(-0.02, 0.054), (0.33, 0.05)]
FORE_F = [(-0.035, 0.045), (0.12, 0.044), (0.2, 0.04), (0.2444, 0.041), (0.262, 0.04)]
FORE_B = [(-0.035, 0.045), (0.12, 0.044), (0.2, 0.04), (0.2444, 0.041), (0.262, 0.04)]
FORE_F5 = [(-0.035, 0.05), (0.262, 0.05)]
FORE_B5 = [(-0.035, 0.05), (0.262, 0.05)]
FOREHAND_F5 = [(-0.035, 0.05), (0.2444, 0.05), (0.375, 0.05)]
FOREHAND_B5 = [(-0.035, 0.05), (0.2444, 0.05), (0.375, 0.05)]
THIGH_F = [(-0.03, 0.078), (0.1, 0.08), (0.25, 0.064), (0.4067, 0.05), (0.436, 0.046)]
THIGH_B = [(-0.03, 0.09), (0.1, 0.088), (0.25, 0.064), (0.4067, 0.048), (0.436, 0.044)]
THIGH_F5 = [(-0.03, 0.08), (0.25, 0.066), (0.436, 0.05)]
THIGH_B5 = [(-0.03, 0.09), (0.25, 0.066), (0.436, 0.05)]
SHANK_F = [(-0.035, 0.05), (0.12, 0.052), (0.27, 0.044), (0.3974, 0.038), (0.418, 0.036)]
SHANK_B = [(-0.035, 0.052), (0.12, 0.062), (0.27, 0.044), (0.3974, 0.038), (0.418, 0.036)]
SHANK_F5 = [(-0.035, 0.052), (0.2, 0.052), (0.418, 0.05)]
SHANK_B5 = [(-0.035, 0.054), (0.2, 0.06), (0.418, 0.05)]
HEAD = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.084), (0.053, -0.092), (0.034, -0.101), (0.022, -0.093), (0.008, -0.082), (-0.007, -0.076), (-0.039, -0.066), (-0.05, -0.04), (-0.043, -0.016), (-0.01, 0.045)]
BOB = [(0.139, -0.058), (0.172, -0.04), (0.183, 0.034), (0.17, 0.08), (0.13, 0.122), (0.07, 0.13), (0.02, 0.112), (-0.04, 0.124), (-0.078, 0.102), (-0.062, 0.05), (0.0, 0.05), (0.1, 0.06), (0.125, 0.02)]
HAND100 = [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)]
THUMB = [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)]
ABOOT = [(-0.054, 0.06), (-0.058, -0.035), (-0.044, -0.08), (-0.0109, -0.0868), (0.1858, -0.002), (0.19, 0.012), (0.13, 0.046), (0.06, 0.052), (0.028, 0.082), (-0.03, 0.065)]
ABOOT5 = [(-0.054, 0.06), (-0.058, -0.035), (-0.044, -0.08), (-0.0109, -0.0868), (0.1858, -0.002), (0.248, 0.0), (0.26, 0.02), (0.22, 0.046), (0.13, 0.054), (0.06, 0.056), (0.028, 0.082), (-0.03, 0.065)]
TOEBOX = [(0.0, -0.002), (0.045, -0.002), (0.066, 0.004), (0.073, 0.018), (0.062, 0.034), (0.03, 0.042), (0.0, 0.05)]


ALPHA = math.asin(0.075 / 0.1858)


def boot_merged(sf, tones=None):
    ball = 0.1858 * sf
    toe_world = [(0.0, 0.0), (0.05 * sf, 0.0), (0.068 * sf, 0.014 * sf), (0.062 * sf, 0.032 * sf), (0.03 * sf, 0.046 * sf)]
    toe_local = [(ball + dx * math.cos(ALPHA) - dy * math.sin(ALPHA), dx * math.sin(ALPHA) + dy * math.cos(ALPHA)) for dx, dy in toe_world]
    return scl(ABOOT[:5], sf) + [(round(x, 4), round(y, 4)) for x, y in toe_local[1:]] + scl(ABOOT[6:], sf)


def mitt(sd, z, thumb):
    o = [S(f"hand-{sd}", f"hand-{sd}", "body", scl(HAND100, sA), z, 0.3, region=HAND, raw=True)]
    if thumb:
        o.append(S(f"thumb-{sd}", f"hand-{sd}", "body", scl(THUMB, sA), z + 0.02, 0.3, region=HAND, raw=True))
    return o


def torso_pts(front, back):
    return fb(front, back)


def rep_500():
    s = []
    s.append(S("sil-head", "head", "body", scl([(0.0, 0.075), (0.05, 0.112), (0.12, 0.124), (0.172, 0.08), (0.186, 0.0), (0.14, -0.062), (0.1, -0.078), (0.065, -0.084), (0.034, -0.1), (0.01, -0.08), (-0.04, -0.066), (-0.07, 0.0), (-0.078, 0.08)], sH), 5, 0.4, regions=[FACE, HAIR], raw=True))
    pb = torso_pts(TORSO_F[:-2] + [(0.42, 0.07), (0.4313, 0.03)], TORSO_B[:-1] + [(0.4313, 0.03)])
    s.append(S("sil-body", "torso", "body", pb, 4, 0.45, region=SWEAT, corners=(0, len(pb) - 1)))
    s.append(S("sil-belly", "belly", "body", scl(BELLY5, 1.0), 4.5, 0.4, region=BELLY_R, raw=True))
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"sil-arm-{sd}", f"arm-{sd}", "body", fb(SLEEVE_F5, SLEEVE_B5), z, 0.55, region=SWEAT))
        s.append(S(f"sil-forearm-{sd}", f"forearm-{sd}", "body", fb(FOREHAND_F5, FOREHAND_B5), z, 0.55, regions=[SWEAT, HAND]))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"sil-thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F5, THIGH_B5), z, 0.85, region=LEG))
        s.append(S(f"sil-shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F5, SHANK_B5), z + 0.02, 0.85, region=LEG))
        s.append(S(f"sil-boot-{sd}", f"foot-{sd}", "body", boot_merged(sF), z + 0.1, 0.7, region=BOOT, raw=True, corners=(3, 4)))
    return s


def common(den, z0, z1, thumb, hatch=None):
    s = []
    for sd, z in (("l", z0), ("r", z1)):
        cs = den == 200
        s.append(S(f"sleeve-{sd}", f"arm-{sd}", "body", fb(SLEEVE_F5, SLEEVE_B5) if cs else fb(SLEEVE_F, SLEEVE_B), z, 0.55, region=SWEAT, hatch=hatch))
        s.append(S(f"forearm-{sd}", f"forearm-{sd}", "body", fb(FORE_F5, FORE_B5) if cs else fb(FORE_F, FORE_B), z, 0.55, region=SWEAT, corners=() if cs else (3, 4), hatch=hatch))
        s += mitt(sd, z + 0.2, thumb)
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"thigh-{sd}", f"thigh-{sd}", "body", fb(THIGH_F5, THIGH_B5) if den == 200 else fb(THIGH_F, THIGH_B), z, 0.85, region=LEG))
        s.append(S(f"shank-{sd}", f"shank-{sd}", "body", fb(SHANK_F5, SHANK_B5) if den == 200 else fb(SHANK_F, SHANK_B), z + 0.02, 0.85, region=LEG, corners=() if den == 200 else (3, 4)))
        s.append(S(f"boot-{sd}", f"foot-{sd}", "body", scl(ABOOT, sF), z + 0.1, 0.7, region=BOOT, raw=True, corners=(3, 4)))
        s.append(S(f"boot-toe-{sd}", f"toe-{sd}", "body", scl(TOEBOX, sF), z + 0.1, 0.7, region=BOOT, raw=True, corners=(0, 6)))
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.046), (0.0928, 0.038)], [(-0.02, 0.05), (0.0928, 0.04)]), 3.8, 0.25, region=FACE))
    s.append(S("sweater", "torso", "body", torso_pts(TORSO_F[::2] + [TORSO_F[-1]], TORSO_B[::2] + [TORSO_B[-1]]) if den == 200 else torso_pts(TORSO_F, TORSO_B), 4, 0.6, region=SWEAT, corners=(0, 11 if den == 200 else 15), hatch=hatch))
    s.append(S("belly-cover", "belly", "body", scl(BELLY5, 1.0) if den == 200 else scl(BELLY, 1.0), 4.4, 0.55, region=BELLY_R, raw=True, hatch=hatch))
    s.append(S("head", "head", "body", scl(HEAD[::2], sH) if den == 200 else scl(HEAD, sH), 5, 0.25, region=FACE, raw=True))
    s.append(S("hair", "head", "body", scl(BOB, sH), 5.5, 0.72, region=HAIR, raw=True))
    return s


def rep_200():
    return common(200, 0.5, 6.0, False)


def rep_100():
    s = common(100, 0.5, 6.0, True, hatch="knit")
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"cuff-{sd}", f"forearm-{sd}", "detail", [(0.225, -0.044), (0.225, 0.044)], z + 0.02, 0.1, region=SWEAT, closed=False, filled=False, smooth=False))
    s.append(S("sweater-hem", "torso", "detail", [(-0.12, -0.108), (-0.12, 0.098)], 4.1, 0.1, region=SWEAT, closed=False, filled=False, smooth=False))
    s.append(S("sweater-collar", "torso", "detail", [(0.39, 0.085), (0.42, 0.05), (0.4313, 0.0)], 4.1, 0.1, region=SWEAT, closed=False, filled=False))
    s.append(S("linea-fold", "belly", "detail", [(0.01, 0.0), (0.12, 0.0), (0.2, 0.0)], 4.5, 0.1, region=BELLY_R, closed=False, filled=False, raw=True))
    s.append(F.ell("ear", "head", "detail", 0.046 * sH, 0.026 * sH, 0.012, 0.02, 5.2, 0.3, region=FACE, raw=True))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"boot-sole-{sd}", f"foot-{sd}", "detail", scl([(-0.0109, -0.0868), (0.1858, -0.002)], sF), z + 0.15, 0.1, region=BOOT, closed=False, filled=False, raw=True, smooth=False))
    return s


def rep_50():
    drop = {"cuff-l", "cuff-r", "sweater-hem", "sweater-collar", "linea-fold", "boot-sole-l", "boot-sole-r"}
    s = [x for x in rep_100() if x["id"] not in drop]
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"cuff-rib-{sd}", f"forearm-{sd}", "body", fb([(0.215, 0.044), (0.262, 0.042)], [(0.215, 0.044), (0.262, 0.042)]), z + 0.03, 0.4, region=SWEAT, smooth=False))
        for k, y in enumerate((0.016, 0.0, -0.016)):
            s.append(S(f"finger-{sd}-{k}", f"hand-{sd}", "texture", scl([(0.12, y + 0.004), (0.188, y + 0.01)], sA), z + 0.25, 0.1, region=HAND, closed=False, filled=False, raw=True, smooth=False))
    for sd, z in (("l", 1.0), ("r", 3.0)):
        s.append(S(f"boot-sole-{sd}", f"foot-{sd}", "body", scl([(-0.042, -0.074), (-0.0109, -0.0868), (0.1858, -0.002), (0.188, 0.008), (0.01, -0.068)], sF), z + 0.15, 0.9, region=BOOT, raw=True, corners=(1, 2)))
        s.append(S(f"boot-toe-sole-{sd}", f"toe-{sd}", "body", scl([(0.0, -0.002), (0.066, -0.002), (0.073, 0.008), (0.0, 0.01)], sF), z + 0.15, 0.9, region=BOOT, raw=True))
        s.append(S(f"boot-zip-{sd}", f"foot-{sd}", "texture", scl([(0.01, 0.07), (0.05, 0.03), (0.1, 0.04)], sF), z + 0.2, 0.1, region=BOOT, closed=False, filled=False, raw=True))
        s.append(S(f"legging-seam-{sd}", f"thigh-{sd}", "texture", [(0.0, 0.07), (0.2, 0.05), (0.4, 0.04)], z + 0.03, 0.9, region=LEG, closed=False, filled=False))
    s.append(S("sweater-hem-rib", "torso", "body", fb([(-0.16, 0.098), (-0.12, 0.096)], [(-0.16, 0.108), (-0.12, 0.11)]), 4.1, 0.45, region=SWEAT, smooth=False))
    s.append(S("sweater-collar", "torso", "detail", [(0.39, 0.085), (0.42, 0.05), (0.4313, 0.0)], 4.1, 0.1, region=SWEAT, closed=False, filled=False))
    s.append(S("cable-knit", "torso", "texture", [(0.16, 0.0), (0.26, 0.0)], 4.15, 0.1, region=SWEAT, closed=False, filled=False, raw=True, smooth=False)) if False else None
    s = [x for x in s if x]
    s.append(S("belly-stretch-1", "belly", "texture", [(0.03, 0.06), (0.2, 0.05)], 4.5, 0.1, region=BELLY_R, closed=False, filled=False, raw=True))
    s.append(S("belly-stretch-2", "belly", "texture", [(0.03, -0.05), (0.2, -0.045)], 4.5, 0.1, region=BELLY_R, closed=False, filled=False, raw=True))
    s.append(F.ell("eye", "head", "detail", 0.085 * sH, -0.068 * sH, 0.008, 0.005, 5.3, 0.9, region=FACE, raw=True))
    s.append(S("mouth", "head", "texture", scl([(0.014, -0.078), (0.011, -0.062)], sH), 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True, smooth=False))
    for k, (s0, y0) in enumerate(((0.15, 0.03), (0.12, 0.08), (0.05, 0.1))):
        s.append(S(f"hair-strand-{k}", "head", "texture", scl([(s0, y0 - 0.04), (s0 - 0.03, y0 + 0.01)], sH), 5.6, 0.1, region=HAIR, closed=False, filled=False, raw=True))
    return s
