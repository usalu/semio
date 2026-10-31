"""runner-with-blade-prosthesis geometry: station tables and one function per scale."""
from a_run_poses import *

FACE, HAIR, ARM, HAND, SING, SHORT, LEGSK, SHOE, SOCK, BLADE = "face-skin", "hair", "arm-skin", "hand-skin", "singlet", "shorts", "leg-skin", "running-shoe", "socket", "blade"
sF, sA, sH = 0.98, 1.0, 1.0
dp = POSES["standing-ready"][0]
F = Fig(a, {"bones": dp, "root": {}}, signs={"foot-l": 1, "toe-l": 1, "hand-l": 1, "hand-r": 1, "blade-r": 1, "blade-tip-r": 1, "shank-r": 1})


def S(id, bone, role, pts, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, raw=False, smooth=True, hatch=None, pen=None):
    return F.sh(id, bone, role, pts, z, tone, region=region, regions=regions, closed=closed, corners=corners, filled=filled, raw=raw, smooth=smooth, hatch=hatch, pen=pen)


SING_F = [(-0.07, 0.085), (0.05, 0.085), (0.18, 0.095), (0.30, 0.105), (0.40, 0.082), (0.45, 0.06), (0.48, 0.05)]
SING_B = [(-0.07, 0.09), (0.05, 0.088), (0.2, 0.095), (0.32, 0.1), (0.42, 0.088), (0.48, 0.05)]
SHORT_PEL_F = [(-0.04, 0.1), (0.06, 0.09), (0.1, 0.086)]
SHORT_PEL_B = [(-0.04, 0.108), (0.06, 0.098), (0.1, 0.094)]
ARM_F = [(-0.03, 0.047), (0.12, 0.048), (0.26, 0.04), (0.34, 0.037)]
ARM_B = [(-0.03, 0.05), (0.12, 0.05), (0.26, 0.04), (0.34, 0.037)]
ARM_F5 = [(-0.03, 0.05), (0.34, 0.046)]
ARM_B5 = [(-0.03, 0.052), (0.34, 0.046)]
FORE_F = [(-0.035, 0.04), (0.12, 0.039), (0.22, 0.034), (0.27, 0.033), (0.285, 0.03)]
FORE_B = [(-0.035, 0.04), (0.12, 0.039), (0.22, 0.034), (0.27, 0.033), (0.285, 0.03)]
FORE_F5 = [(-0.035, 0.05), (0.285, 0.05)]
FORE_B5 = [(-0.035, 0.05), (0.285, 0.05)]
THIGH_F = [(-0.03, 0.078), (0.12, 0.08), (0.3, 0.064), (0.435, 0.05), (0.465, 0.046)]
THIGH_B = [(-0.03, 0.088), (0.12, 0.09), (0.3, 0.062), (0.435, 0.048), (0.465, 0.044)]
THIGH_F5 = [(-0.03, 0.08), (0.3, 0.068), (0.465, 0.052)]
THIGH_B5 = [(-0.03, 0.09), (0.3, 0.066), (0.465, 0.052)]
SHORT_F = [(-0.03, 0.086), (0.12, 0.087), (0.21, 0.074)]
SHORT_B = [(-0.03, 0.094), (0.12, 0.094), (0.21, 0.076)]
SHANK_F = [(-0.035, 0.05), (0.12, 0.052), (0.3, 0.04), (0.425, 0.032), (0.447, 0.032)]
SHANK_B = [(-0.035, 0.05), (0.12, 0.066), (0.3, 0.042), (0.425, 0.032), (0.447, 0.032)]
SHANK_F5 = [(-0.035, 0.052), (0.2, 0.052), (0.447, 0.05)]
SHANK_B5 = [(-0.035, 0.06), (0.2, 0.062), (0.447, 0.05)]
SOCK_F = [(-0.05, 0.06), (0.03, 0.062), (0.12, 0.052), (0.17, 0.04), (0.195, 0.034)]
SOCK_B = [(-0.05, 0.064), (0.03, 0.068), (0.12, 0.054), (0.17, 0.04), (0.195, 0.034)]
SOCK_F5 = [(-0.05, 0.06), (0.12, 0.054), (0.195, 0.05)]
SOCK_B5 = [(-0.05, 0.064), (0.12, 0.056), (0.195, 0.05)]
HEAD = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.086), (0.053, -0.096), (0.032, -0.107), (0.02, -0.096), (0.008, -0.085), (-0.007, -0.08), (-0.039, -0.072), (-0.05, -0.042), (-0.043, -0.016), (-0.01, 0.045)]
CROP = [(0.139, -0.05), (0.17, -0.03), (0.18, 0.03), (0.165, 0.08), (0.12, 0.108), (0.07, 0.114), (0.03, 0.095), (0.015, 0.07), (0.06, 0.065), (0.1, 0.05), (0.125, 0.01)]
HAND100 = [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)]
THUMB = [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)]
SHOEPTS = [(-0.05, 0.03), (-0.054, -0.03), (-0.04, -0.078), (-0.0109, -0.0868), (0.1858, -0.002), (0.19, 0.01), (0.12, 0.042), (0.06, 0.05), (0.03, 0.074), (-0.028, 0.05)]
TOEBOX = [(0.0, -0.002), (0.048, -0.002), (0.07, 0.006), (0.075, 0.02), (0.06, 0.034), (0.03, 0.04), (0.0, 0.046)]


def hands(sd, z, thumb, skin=True):
    o = [S(f"hand-{sd}", f"hand-{sd}", "body", scl(HAND100, sA), z, 0.3, region=HAND, raw=True)]
    if thumb:
        o.append(S(f"thumb-{sd}", f"hand-{sd}", "body", scl(THUMB, sA), z + 0.02, 0.3, region=HAND, raw=True))
    return o


def blade_upper(width, tone=0.15):
    n = 7
    pts_u, pts_l = [], []
    for k in range(n + 1):
        s = 0.22 * k / n
        yc = 0.026 * math.sin(math.pi * k / n)
        pts_u.append((s, yc + width / 2))
        pts_l.append((s, yc - width / 2))
    return pts_u + pts_l[::-1]


def blade_tip(width):
    n = 8
    up, lo = [], []
    for k in range(n + 1):
        s = 0.28 * k / n
        yc = -0.03 * math.sin(math.pi * k / n) * (1 - k / n)
        up.append((s, yc + width / 2 * (1 - 0.35 * k / n)))
        lo.append((s, yc - width / 2 * (1 - 0.35 * k / n)))
    return up + lo[::-1]


def blade_line_upper():
    return [(0.0, 0.0), (0.06, 0.016), (0.12, 0.026), (0.18, 0.018), (0.22, 0.0)]


def blade_line_tip():
    return [(0.0, 0.0), (0.07, -0.02), (0.14, -0.03), (0.21, -0.02), (0.28, 0.0)]


def blade_shapes(den):
    s = []
    if den == 500 or den == 200:
        pen = 0.13 if den == 500 else 0.18
        s.append(S("blade-upper", "blade-r", "body", blade_line_upper(), 2.5, 0.1, region=BLADE, closed=False, filled=False, raw=True, pen=pen))
        s.append(S("blade-curl", "blade-tip-r", "body", blade_line_tip(), 2.5, 0.1, region=BLADE, closed=False, filled=False, raw=True, pen=pen))
        s.append(S("blade-toe", "blade-tip-r", "body", [(0.2, -0.014), (0.29, -0.012), (0.3, 0.004), (0.2, 0.01)], 2.6, 0.95, region=BLADE, raw=True, smooth=False))
    else:
        w = 0.024
        s.append(S("blade-upper", "blade-r", "body", blade_upper(w), 2.5, 0.2, region=BLADE, raw=True, corners=(0, 7, 8, 15)))
        s.append(S("blade-curl", "blade-tip-r", "body", blade_tip(w), 2.5, 0.2, region=BLADE, raw=True, corners=(0, 8, 9, 17)))
        s.append(S("blade-toe", "blade-tip-r", "body", [(0.2, -0.016), (0.29, -0.012), (0.3, 0.006), (0.2, 0.012)], 2.6, 0.95, region=BLADE, raw=True, smooth=False))
    return s


def rep_500():
    s = []
    s.append(S("sil-head", "head", "body", scl([(0.0, 0.07), (0.05, 0.108), (0.11, 0.112), (0.156, 0.075), (0.18, 0.0), (0.139, -0.061), (0.1, -0.078), (0.065, -0.086), (0.032, -0.105), (0.008, -0.085), (-0.04, -0.072), (-0.05, -0.03), (-0.01, 0.03)], sH), 5, 0.4, regions=[FACE, HAIR], raw=True))
    pb = fb(SING_F[:-1] + [(0.45, 0.07), (0.48, 0.04)], SING_B[:-1] + [(0.48, 0.04)])
    s.append(S("sil-body", "torso", "body", pb, 4, 0.5, regions=[SING, SHORT], corners=(0, len(pb) - 1)))
    s.append(S("sil-pelvis", "pelvis", "body", fb(SHORT_PEL_F, SHORT_PEL_B), 3.5, 0.8, region=SHORT))
    for sd, z in (("l", 0.5), ("r", 6.0)):
        s.append(S(f"sil-arm-{sd}", f"arm-{sd}", "body", fb(ARM_F5, ARM_B5), z, 0.35, region=ARM))
        s.append(S(f"sil-forearm-{sd}", f"forearm-{sd}", "body", fb([(-0.035, 0.05), (0.26, 0.05), (0.41, 0.05)], [(-0.035, 0.05), (0.26, 0.05), (0.41, 0.05)]), z, 0.35, regions=[ARM, HAND]))
    s.append(S("sil-thigh-l", "thigh-l", "body", fb(THIGH_F5, THIGH_B5), 1.0, 0.45, regions=[LEGSK, SHORT]))
    s.append(S("sil-shank-l", "shank-l", "body", fb(SHANK_F5, SHANK_B5), 1.02, 0.4, region=LEGSK))
    pf = scl(SHOEPTS[:5], sF)
    s.append(S("sil-shoe-l", "foot-l", "body", scl(SHOEPTS, sF), 1.1, 0.25, region=SHOE, raw=True, corners=(3, 4)))
    s.append(S("sil-toe-l", "toe-l", "body", scl(TOEBOX, sF), 1.1, 0.25, region=SHOE, raw=True))
    s.append(S("sil-thigh-r", "thigh-r", "body", fb(THIGH_F5, THIGH_B5), 3.0, 0.45, regions=[LEGSK, SHORT]))
    s.append(S("sil-socket", "shank-r", "body", fb(SOCK_F5, SOCK_B5), 3.02, 0.85, region=SOCK))
    return s + blade_shapes(500)


def common(den, thumb, hatch=None):
    s = []
    for sd, z in (("l", 0.5), ("r", 6.0)):
        cs = den == 200
        s.append(S(f"arm-{sd}", f"arm-{sd}", "body", fb(ARM_F5, ARM_B5) if cs else fb(ARM_F, ARM_B), z, 0.32, region=ARM))
        s.append(S(f"forearm-{sd}", f"forearm-{sd}", "body", fb(FORE_F5, FORE_B5) if cs else fb(FORE_F, FORE_B), z, 0.32, region=ARM, corners=() if cs else (3, 4)))
        s += hands(sd, z + 0.2, thumb)
    s.append(S("thigh-l", "thigh-l", "body", fb(THIGH_F5, THIGH_B5) if den == 200 else fb(THIGH_F, THIGH_B), 1.0, 0.35, region=LEGSK))
    s.append(S("shorts-l", "thigh-l", "body", fb(SHORT_F, SHORT_B), 1.01, 0.8, region=SHORT, hatch=hatch))
    s.append(S("shank-l", "shank-l", "body", fb(SHANK_F5, SHANK_B5) if den == 200 else fb(SHANK_F, SHANK_B), 1.02, 0.35, region=LEGSK, corners=() if den == 200 else (3, 4)))
    s.append(S("shoe-l", "foot-l", "body", scl(SHOEPTS, sF), 1.1, 0.2, region=SHOE, raw=True, corners=(3, 4)))
    s.append(S("shoe-toe-l", "toe-l", "body", scl(TOEBOX, sF), 1.1, 0.2, region=SHOE, raw=True, corners=(0, 6)))
    s.append(S("thigh-r", "thigh-r", "body", fb(THIGH_F5, THIGH_B5) if den == 200 else fb(THIGH_F, THIGH_B), 3.0, 0.35, region=LEGSK))
    s.append(S("shorts-r", "thigh-r", "body", fb(SHORT_F, SHORT_B), 3.01, 0.8, region=SHORT, hatch=hatch))
    s.append(S("socket", "shank-r", "body", fb(SOCK_F5, SOCK_B5) if den == 200 else fb(SOCK_F, SOCK_B), 3.02, 0.85, region=SOCK))
    s.append(S("pelvis-shorts", "pelvis", "body", fb(SHORT_PEL_F, SHORT_PEL_B), 3.5, 0.8, region=SHORT))
    s.append(S("neck", "neck", "body", fb([(-0.02, 0.05), (0.1032, 0.042)], [(-0.02, 0.054), (0.1032, 0.044)]), 3.8, 0.3, region=FACE))
    s.append(S("singlet", "torso", "body", fb(SING_F[::2] + [SING_F[-1]], SING_B[::2] + [SING_B[-1]]) if den == 200 else fb(SING_F, SING_B), 4, 0.55, region=SING, corners=(0, 8 if den == 200 else 12)))
    s.append(S("head", "head", "body", scl(HEAD[::2], sH) if den == 200 else scl(HEAD, sH), 5, 0.3, region=FACE, raw=True))
    s.append(S("hair", "head", "body", scl(CROP, sH), 5.5, 0.8, region=HAIR, raw=True))
    return s


def rep_200():
    return common(200, False) + blade_shapes(200)


def rep_100():
    s = common(100, True)
    s.append(S("singlet-neck", "torso", "detail", [(0.44, 0.07), (0.42, 0.02), (0.44, -0.04)], 4.1, 0.1, region=SING, closed=False, filled=False))
    s.append(S("singlet-number", "torso", "detail", [(0.22, 0.0), (0.3, 0.0)], 4.1, 0.9, region=SING, closed=False, filled=False, raw=True, smooth=False)) if False else None
    s.append(S("socket-liner-edge", "shank-r", "detail", [(0.0, -0.06), (0.0, 0.06)], 3.1, 0.1, region=SOCK, closed=False, filled=False, raw=True, smooth=False))
    s.append(S("shoe-sole", "foot-l", "detail", scl([(-0.0109, -0.0868), (0.1858, -0.002)], sF), 1.15, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
    s.append(F.ell("ear", "head", "detail", 0.046, 0.026, 0.012, 0.02, 5.2, 0.3, region=FACE, raw=True))
    s = [x for x in s if x]
    return s + blade_shapes(100)


def rep_50():
    s = [x for x in rep_100() if x["id"] not in ("socket-liner-edge", "shoe-sole", "singlet-neck") and not x["id"].startswith("blade")]
    for sd, z in (("l", 0.5), ("r", 6.0)):
        for k, y in enumerate((0.016, 0.0, -0.016)):
            s.append(S(f"finger-{sd}-{k}", f"hand-{sd}", "texture", scl([(0.12, y + 0.004), (0.188, y + 0.01)], sA), z + 0.25, 0.1, region=HAND, closed=False, filled=False, raw=True, smooth=False))
        s.append(S(f"muscle-{sd}", f"arm-{sd}", "texture", [(0.09, -0.01), (0.15, 0.02), (0.22, -0.005)], z + 0.03, 0.1, region=ARM, closed=False, filled=False))
    s.append(S("calf-line", "shank-l", "texture", [(0.1, 0.0), (0.2, 0.025), (0.3, 0.005)], 1.05, 0.1, region=LEGSK, closed=False, filled=False))
    s.append(S("shoe-sole", "foot-l", "body", scl([(-0.043, -0.074), (-0.0109, -0.0868), (0.1858, -0.002), (0.188, 0.01), (0.01, -0.066)], sF), 1.15, 0.9, region=SHOE, raw=True, corners=(1, 2)))
    s.append(S("shoe-toe-sole", "toe-l", "body", scl([(0.0, -0.002), (0.07, -0.002), (0.075, 0.008), (0.0, 0.012)], sF), 1.15, 0.9, region=SHOE, raw=True))
    for k, (u, v) in enumerate(((0.05, 0.04), (0.09, 0.045), (0.13, 0.04))):
        s.append(S(f"lace-{k}", "foot-l", "texture", scl([(u, v), (u + 0.014, v + 0.025)], sF), 1.2, 0.1, region=SHOE, closed=False, filled=False, raw=True, smooth=False))
    s.append(S("singlet-neck", "torso", "detail", [(0.44, 0.07), (0.42, 0.02), (0.44, -0.04)], 4.1, 0.1, region=SING, closed=False, filled=False))
    s.append(S("singlet-armhole", "torso", "detail", [(0.36, 0.09), (0.40, 0.0), (0.36, -0.09)], 4.1, 0.1, region=SING, closed=False, filled=False))
    s.append(S("singlet-hem", "torso", "body", fb([(-0.07, 0.085), (-0.05, 0.085)], [(-0.07, 0.09), (-0.05, 0.09)]), 4.1, 0.35, region=SING, smooth=False))
    s.append(S("bib", "torso", "detail", [(0.12, 0.096), (0.12, -0.096), (0.3, -0.1), (0.3, 0.104)], 4.15, 0.9, region=SING, closed=True, filled=False, smooth=False))
    s.append(S("socket-trim", "shank-r", "body", fb([(-0.05, 0.06), (-0.02, 0.062)], [(-0.05, 0.064), (-0.02, 0.066)]), 3.1, 0.3, region=SOCK, smooth=False))
    s.append(S("socket-pyramid", "shank-r", "body", [(0.17, -0.02), (0.2, -0.02), (0.2, 0.02), (0.17, 0.02)], 3.1, 0.3, region=SOCK, raw=True, smooth=False))
    s.append(S("socket-seam", "shank-r", "texture", [(0.0, -0.05), (0.08, -0.01), (0.16, -0.02)], 3.12, 0.1, region=SOCK, closed=False, filled=False, raw=True))
    s.append(F.ell("eye", "head", "detail", 0.085, -0.07, 0.008, 0.005, 5.3, 0.9, region=FACE, raw=True))
    s.append(S("brow", "head", "texture", [(0.098, -0.072), (0.1, -0.06), (0.094, -0.048)], 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True))
    s.append(S("mouth", "head", "texture", [(0.014, -0.08), (0.011, -0.064)], 5.3, 0.1, region=FACE, closed=False, filled=False, raw=True, smooth=False))
    return s + blade_shapes(50) + [S("blade-bolt", "blade-r", "detail", [(0.01, -0.012), (0.01, 0.012)], 2.7, 0.1, region=BLADE, closed=False, filled=False, raw=True, smooth=False),
                                   S("blade-lamination", "blade-r", "texture", [(0.03, 0.0), (0.1, 0.02), (0.19, 0.012)], 2.7, 0.9, region=BLADE, closed=False, filled=False, raw=True),
                                   S("toe-tread", "blade-tip-r", "texture", [(0.22, -0.016), (0.24, 0.012), (0.26, -0.014), (0.28, 0.01)], 2.7, 0.1, region=BLADE, closed=False, filled=False, raw=True, smooth=False)]
