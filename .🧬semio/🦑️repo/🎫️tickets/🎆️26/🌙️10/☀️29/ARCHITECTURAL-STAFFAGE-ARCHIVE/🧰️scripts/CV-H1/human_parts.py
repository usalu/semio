"""Hand-authored limb outline tables (figure coordinates, 1:200 and 1:100 versions) of the commuter proportions; other humans use them
scaled by their bone-length ratio k (along) and kw (width). Clothing identity is carried by the region, tone and the per-asset torso paths."""
from lib import *


def sc(pts, k, kw=None):
    kw = kw or k
    return [(round(s * k, 4), round(f * kw, 4)) for s, f in pts]


SLEEVE200 = [(-0.01, -0.058), (0.15, -0.056), (0.33, -0.046), (0.365, 0.0), (0.33, 0.05), (0.15, 0.058), (0.0, 0.06), (-0.03, 0.0)]
FORE200 = [(-0.03, 0.0), (-0.01, -0.047), (0.12, -0.044), (0.25, -0.039), (0.25, 0.04), (0.12, 0.046), (-0.01, 0.05)]
HAND200 = [(-0.015, -0.03), (0.1, -0.032), (0.185, -0.015), (0.2, 0.012), (0.176, 0.034), (0.08, 0.04), (-0.015, 0.037)]
SLEEVE100 = [(-0.007, -0.058), (0.15, -0.056), (0.325, -0.046), (0.363, 0.0), (0.325, 0.05), (0.2, 0.057), (0.075, 0.064), (0.003, 0.062), (-0.033, 0.03), (-0.033, -0.03)]
FORE100 = [(-0.01, -0.047), (-0.036, -0.02), (-0.036, 0.025), (-0.01, 0.05), (0.12, 0.047), (0.25, 0.04), (0.25, -0.039), (0.12, -0.044)]
HAND100 = [(-0.015, -0.03), (0.07, -0.034), (0.15, -0.03), (0.185, -0.012), (0.19, 0.008), (0.17, 0.022), (0.176, 0.033), (0.12, 0.036), (0.06, 0.04), (-0.015, 0.037)]
THUMB100 = [(-0.004, 0.024), (0.04, 0.045), (0.098, 0.082), (0.112, 0.074), (0.094, 0.05), (0.046, 0.022)]
THIGH200 = [(-0.02, -0.095), (0.2, -0.088), (0.415, -0.062), (0.45, 0.0), (0.415, 0.066), (0.2, 0.085), (-0.02, 0.09)]
THIGH100 = [(-0.02, -0.095), (0.2, -0.088), (0.4, -0.062), (0.445, -0.02), (0.445, 0.03), (0.415, 0.066), (0.2, 0.085), (-0.025, 0.09)]
SHANK200 = [(0.0, -0.062), (-0.04, -0.03), (-0.045, 0.0), (-0.032, 0.062), (0.2, 0.052), (0.415, 0.041), (0.415, -0.043), (0.25, -0.058), (0.1, -0.076)]
SHANK100 = [(0.0, -0.062), (-0.035, -0.04), (-0.045, 0.0), (-0.035, 0.05), (0.0, 0.064), (0.2, 0.052), (0.415, 0.041), (0.415, -0.043), (0.25, -0.06), (0.08, -0.076)]
SHOE200 = [(-0.045, -0.02), (-0.03, -0.075), (-0.006, -0.0848), (0.19, 0.0), (0.2, 0.025), (0.16, 0.06), (0.03, 0.047), (-0.03, 0.012)]
SHOE100 = [(-0.0435, -0.03), (-0.03, -0.075), (-0.0063, -0.0848), (0.1867, 0.0004), (0.16, 0.0626), (0.034, 0.0476), (0.018, 0.046), (-0.028, 0.015), (-0.05, -0.012)]
TOE200 = [(-0.002, 0.0), (0.045, 0.0), (0.07, 0.015), (0.07, 0.03), (0.03, 0.062), (-0.002, 0.068)]
TOE100 = [(-0.001, 0.0), (0.045, 0.0), (0.068, 0.0), (0.076, 0.014), (0.062, 0.045), (0.03, 0.062), (-0.001, 0.068)]
HEAD = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.1, -0.078), (0.083, -0.079), (0.065, -0.084), (0.053, -0.094), (0.032, -0.107), (0.02, -0.095), (0.008, -0.084), (-0.007, -0.08), (-0.039, -0.07), (-0.05, -0.04), (-0.043, -0.016), (-0.01, 0.045)]
HEAD200 = [(0.011, 0.074), (0.05, 0.104), (0.11, 0.108), (0.156, 0.069), (0.172, 0.0), (0.139, -0.061), (0.083, -0.079), (0.053, -0.094), (0.03, -0.106), (0.008, -0.084), (-0.02, -0.082), (-0.045, -0.06), (-0.04, -0.015), (-0.01, 0.059)]
HAIR = [(0.139, -0.062), (0.173, -0.048), (0.185, 0.034), (0.161, 0.072), (0.136, 0.114), (0.071, 0.122), (0.021, 0.096), (0.013, 0.072), (0.051, 0.076), (0.101, 0.064), (0.123, 0.024), (0.139, -0.016)]
HAIR200 = [(0.139, -0.062), (0.172, -0.035), (0.176, 0.034), (0.16, 0.072), (0.12, 0.112), (0.071, 0.121), (0.021, 0.096), (0.013, 0.072), (0.06, 0.07), (0.1, 0.055), (0.125, 0.015)]
NECK100 = [(0.0928, 0.0585), (0.0221, 0.0586), (-0.0133, 0.0587), (-0.0029, -0.0509), (0.0265, -0.0447), (0.0609, -0.0238)]
COWL = [(0.0242, 0.0791), (-0.0073, 0.0878), (-0.0326, 0.0742), (-0.0373, 0.0352), (-0.0231, -0.0659), (-0.0093, -0.0922), (0.0235, -0.0957), (0.0516, -0.0796), (0.0543, -0.0126), (0.0467, 0.0268)]


def arms(F, scale, ids, k, kw, z, tones, regions, thumb=True, tail=""):
    """ids: (side, far?) pairs; returns sleeve, forearm, (cuff), hand, thumb shapes for each side. tones/regions: dict sleeve/hand."""
    out = []
    for sd, zz in z:
        if scale == 200:
            out.append(F.sh(f"sleeve-{sd}", f"arm-{sd}", "body", sc(SLEEVE200, k, kw), zz, tones["sleeve"], region=regions["sleeve"], raw=True))
            out.append(F.sh(f"forearm-{sd}", f"forearm-{sd}", "body", sc(FORE200, k, kw), zz, tones["sleeve"], region=regions["sleeve"], corners=(2, 3), raw=True))
            out.append(F.sh(f"hand-{sd}", f"hand-{sd}", "body", sc(HAND200, k, kw), zz + 0.2, tones["hand"], region=regions["hand"], raw=True))
        else:
            out.append(F.sh(f"sleeve-{sd}", f"arm-{sd}", "body", sc(SLEEVE100, k, kw), zz, tones["sleeve"], region=regions["sleeve"], raw=True))
            out.append(F.sh(f"forearm-{sd}", f"forearm-{sd}", "body", sc(FORE100, k, kw), zz, tones["sleeve"], region=regions["sleeve"], corners=(5, 6), raw=True))
            out.append(F.sh(f"hand-{sd}", f"hand-{sd}", "body", sc(HAND100, k, kw), zz + 0.2, tones["hand"], region=regions["hand"], raw=True))
            if thumb:
                out.append(F.sh(f"thumb-{sd}", f"hand-{sd}", "body", sc(THUMB100, k, kw), zz + 0.22, tones["hand"], region=regions["hand"], raw=True))
    return out


def legs(F, scale, k, kw, z, tones, regions, crease=True):
    out = []
    for sd, zz in z:
        if scale == 200:
            out.append(F.sh(f"thigh-{sd}", f"thigh-{sd}", "body", sc(THIGH200, k, kw), zz, tones["leg"], region=regions["leg"], raw=True))
            out.append(F.sh(f"shank-{sd}", f"shank-{sd}", "body", sc(SHANK200, k, kw), zz + 0.02, tones["leg"], region=regions["leg"], corners=(5, 6), raw=True))
            out.append(F.sh(f"shoe-{sd}", f"foot-{sd}", "body", sc(SHOE200, k, kw), zz + 0.1, tones["shoe"], region=regions["shoe"], raw=True, corners=(2, 3)))
            out.append(F.sh(f"shoe-toe-{sd}", f"toe-{sd}", "body", sc(TOE200, k, kw), zz + 0.1, tones["shoe"], region=regions["shoe"], raw=True, corners=(0, 5)))
        else:
            out.append(F.sh(f"thigh-{sd}", f"thigh-{sd}", "body", sc(THIGH100, k, kw), zz, tones["leg"], region=regions["leg"], raw=True))
            out.append(F.sh(f"shank-{sd}", f"shank-{sd}", "body", sc(SHANK100, k, kw), zz + 0.02, tones["leg"], region=regions["leg"], corners=(6, 7), raw=True))
            if crease:
                out.append(F.sh(f"leg-seam-{sd}", f"shank-{sd}", "detail", sc([(0.02, 0.0), (0.3, 0.004), (0.4, 0.0)], k, kw), zz + 0.03, 0.1, region=regions["leg"], closed=False, filled=False, raw=True))
            out.append(F.sh(f"shoe-{sd}", f"foot-{sd}", "body", sc(SHOE100, k, kw), zz + 0.1, tones["shoe"], region=regions["shoe"], raw=True, corners=(2, 3)))
            out.append(F.sh(f"shoe-toe-{sd}", f"toe-{sd}", "body", sc(TOE100, k, kw), zz + 0.1, tones["shoe"], region=regions["shoe"], raw=True, corners=(0, 6)))
            out.append(F.sh(f"shoe-sole-{sd}", f"foot-{sd}", "detail", sc([(-0.0236, -0.0705), (0.1777, 0.0183)], k, kw), zz + 0.15, 0.1, region=regions["shoe"], closed=False, filled=False, raw=True, smooth=False))
    return out


def head(F, scale, k, zhead, skin, hair, tone_skin=0.25, tone_hair=0.75, ear=True, kw=None):
    out = []
    H = HEAD200 if scale == 200 else HEAD
    HR = HAIR200 if scale == 200 else HAIR
    out.append(F.sh("head", "head", "body", sc(H, k, kw), zhead, tone_skin, region=skin, raw=True))
    out.append(F.sh("hair", "head", "body", sc(HR, k, kw), zhead + 0.5, tone_hair, region=hair, raw=True))
    if scale == 100 and ear:
        out.append(F.ell("ear", "head", "detail", 0.046 * k, 0.026 * k, 0.012 * k, 0.02 * k, zhead + 0.2, 0.3, region=skin, raw=True))
    return out
