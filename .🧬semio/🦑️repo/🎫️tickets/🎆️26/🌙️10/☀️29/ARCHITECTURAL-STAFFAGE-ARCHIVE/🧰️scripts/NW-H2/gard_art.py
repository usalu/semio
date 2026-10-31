"""Kneeling gardener drawings, one function per scale (local frames: see human.py). Far legs are hidden behind the near leg below 1:50."""
from lib import *
from art import *
from gard_rig import H, L, TROWEL_L, PAD_T

F = H / 1.82
LT, LS, LU, LF, LH, LTO = L["thigh"], L["shank"], L["upper"], L["fore"], L["hand"], L["torso"]
PELVIS = [(0.047, -0.105), (0.047, 0.108), (-0.04, 0.115), (-0.080, 0.065), (-0.088, 0.0), (-0.065, -0.085), (0.0, -0.108)]
CLOG = [(-0.058, -0.0754, "c"), (-0.060, -0.01), (-0.046, 0.034), (0.030, 0.046), (0.100, 0.030), (0.160, -0.010), (0.200, -0.046), (0.214, -0.0754, "c")]
BLADE = [(0.115, 0.0, "c"), (0.13, 0.030), (0.19, 0.040), (0.26, 0.028), (0.30, 0.0, "c"), (0.26, -0.020), (0.19, -0.030), (0.13, -0.022)]


def tab(rows):
    return [(q[0], q[2], q[1]) + q[3:] for q in rows]


def props(level):
    s = []
    A = s.append
    if level == 500:
        A(shape("pad", "pad", "body", 0.5, 0.45, rect(-0.56, 0.0, 0.76, 0.10, 0.03), region="pad", material="eva-foam"))
        A(shape("trug", "trug", "body", 0.5, 0.5, path([(-0.22, 0.0, "c"), (0.22, 0.0, "c"), (0.26, 0.20, "c"), (0.14, 0.28), (0.0, 0.31), (-0.14, 0.28), (-0.26, 0.20, "c")]), region="trug", material="polyethylene"))
    else:
        A(shape("pad", "pad", "body", 0.5, 0.55, rect(-0.56, 0.0, 0.76, PAD_T, 0.012), region="pad", material="eva-foam"))
        A(shape("trug", "trug", "body", 0.5, 0.72, lines([(-0.22, 0.0, "c"), (0.22, 0.0, "c"), (0.255, 0.21, "c"), (-0.255, 0.21, "c")]), region="trug", material="polyethylene"))
        A(shape("trug-weeds", "trug", "body", 0.55, 0.45, path([(-0.20, 0.205, "c"), (-0.12, 0.27), (0.0, 0.31), (0.12, 0.28), (0.20, 0.205, "c")]), region="trug", material="polyethylene"))
    if level <= 100:
        A(shape("trug-rim", "trug", "detail", 0.56, 0.2, poly([(-0.2, 0.21), (0.2, 0.21)]), filled=False, region="trug"))
    return s


def s200():
    s = props(200)
    A = s.append
    A(shape("thigh-l", "thigh-l", "body", 4.0, 0.4, path(limb(0, [(0, .104, .102), (0.22, .090, .088), (LT, .068, .066)], 0.09, 0.014)), region="trousers"))
    A(shape("shank-l", "shank-l", "body", 4.02, 0.4, path(limb(0, [(0, .066, .066), (0.16, .063, .068), (LS - 0.01, .054, .054)], 0.02, 0.0)), region="trousers"))
    A(shape("clog-l", "foot-l", "body", 4.05, 0.25, path(CLOG), region="clogs"))
    A(shape("shirt", "torso", "body", 3.0, 0.88, path(limb(0, tab([(0.0, 0.105, 0.115, "c"), (0.12, 0.108, 0.120), (0.26, 0.098, 0.128), (0.36, 0.092, 0.122), (0.42, 0.080, 0.100), (LTO, 0.062, 0.058)]))), region="shirt-torso"))
    A(shape("head", "head", "body", 6.0, 0.86, path(head(F)), region="face"))
    A(shape("hat", "head", "body", 6.4, 0.8, path([(0.200, -0.25, "c"), (0.226, -0.18), (0.240, -0.10), (0.290, -0.085), (0.312, 0.0), (0.290, 0.085), (0.240, 0.10), (0.226, 0.18), (0.200, 0.25, "c"), (0.180, 0.0)]), region="hat"))
    for side, z in (("l", 4.5), ("r", 3.6)):
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.86, path(limb(0, [(0, .050, .050), (LU, .042, .042)], 0.03, 0.01)), region="shirt-sleeves"))
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.86, path(limb(0, [(0, .042, .042), (LF - 0.01, .030, .030), (LF + 0.06, .040, .036), (LF + 0.15, .024, .022)], 0.012, 0.02)), regions=["forearms", "gloves"]))
    A(shape("trowel", "trowel", "body", 4.8, 0.6, path(BLADE + []), region="trowel"))
    return s


def s500():
    s = props(500)
    A = s.append
    A(shape("thigh-l", "thigh-l", "body", 4.0, 0.15, path(limb(0, [(0, .095, .093), (LT, .068, .066)], 0.03, 0.02)), region="trousers"))
    A(shape("shank-l", "shank-l", "body", 4.02, 0.15, path(limb(0, [(0, .068, .068), (LS - 0.01, .060, .060)], 0.02, 0.0)), region="trousers"))
    A(shape("clog-l", "foot-l", "body", 4.05, 0.15, path([(-0.060, -0.0754, "c"), (-0.060, 0.025), (0.050, 0.046), (0.140, 0.0), (0.216, -0.040), (0.216, -0.0754, "c")]), region="clogs"))
    A(shape("pelvis", "pelvis", "body", 2.9, 0.15, path(PELVIS), region="trousers"))
    A(shape("shirt", "torso", "body", 3.0, 0.2, path(limb(0, tab([(0.0, 0.105, 0.115, "c"), (0.16, 0.104, 0.122), (0.32, 0.095, 0.130), (0.40, 0.085, 0.108), (LTO + 0.02, 0.065, 0.060)]))), region="shirt-torso"))
    A(shape("head", "head", "body", 6.0, 0.4, path([(0.0, 0.062), (0.12, 0.085), (0.215, 0.098), (0.280, 0.0), (0.230, -0.085), (0.165, -0.105), (0.100, -0.092), (0.060, -0.060), (0.0, -0.058)]), region="face"))
    A(shape("hat", "head", "body", 6.4, 0.5, path([(0.205, -0.27, "c"), (0.240, -0.20), (0.310, -0.10), (0.320, 0.0), (0.310, 0.10), (0.240, 0.20), (0.205, 0.27, "c"), (0.190, 0.0)]), region="hat"))
    for side, z in (("l", 4.5), ("r", 3.6)):
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.2, path(limb(0, [(0, .052, .052), (LU, .050, .050)], 0.03, 0.01)), region="shirt-sleeves"))
    A(shape("forearm-l", "forearm-l", "body", 4.52, 0.2, path(limb(0, [(0, .050, .050), (LF + LH - 0.04, .050, .050)], 0.01, 0.03)), regions=["forearms", "gloves"]))
    A(shape("forearm-r", "forearm-r", "body", 1.52, 0.2, path(limb(0, [(0, .050, .050), (LF + LH - 0.02, .050, .050)], 0.01, 0.03)), regions=["forearms", "gloves", "trowel"]))
    return s


def s200():
    s = props(200)
    A = s.append
    A(shape("thigh-l", "thigh-l", "body", 4.0, 0.4, path(limb(0, [(0, .104, .102), (0.22, .090, .088), (LT, .068, .066)], 0.09, 0.014)), region="trousers"))
    A(shape("shank-l", "shank-l", "body", 4.02, 0.4, path(limb(0, [(0, .066, .066), (0.16, .063, .068), (LS - 0.01, .054, .054)], 0.02, 0.0)), region="trousers"))
    A(shape("clog-l", "foot-l", "body", 4.05, 0.25, path(CLOG), region="clogs"))
    A(shape("shirt", "torso", "body", 3.0, 0.88, path(limb(0, tab([(0.0, 0.105, 0.115, "c"), (0.12, 0.108, 0.120), (0.26, 0.098, 0.128), (0.36, 0.092, 0.122), (0.42, 0.080, 0.100), (LTO, 0.062, 0.058)]))), region="shirt-torso"))
    A(shape("head", "head", "body", 6.0, 0.86, path(head(F)), region="face"))
    A(shape("hat", "head", "body", 6.4, 0.8, path([(0.200, -0.25, "c"), (0.226, -0.18), (0.240, -0.10), (0.290, -0.085), (0.312, 0.0), (0.290, 0.085), (0.240, 0.10), (0.226, 0.18), (0.200, 0.25, "c"), (0.180, 0.0)]), region="hat"))
    for side, z in (("l", 4.5), ("r", 3.6)):
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.86, path(limb(0, [(0, .050, .050), (LU, .042, .042)], 0.03, 0.01)), region="shirt-sleeves"))
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.86, path(limb(0, [(0, .042, .042), (LF - 0.01, .030, .030)], 0.012, 0.008)), region="forearms"))
        A(shape(f"glove-{side}", f"hand-{side}", "body", z + 0.04, 0.6, path(limb(0, [(0.0, .036, .034), (0.08, .040, .034), (0.15, .026, .021)], 0.0, 0.02)), region="gloves"))
    A(shape("trowel", "trowel", "body", 4.8, 0.6, path(BLADE + []), region="trowel"))
    return s


def s100():
    s = props(100)
    A = s.append
    A(shape("thigh-l", "thigh-l", "body", 4.0, 0.42, path(limb(0, [(0, .105, .103), (0.14, .095, .093), (0.27, .081, .079), (LT, .066, .064)], 0.09, 0.012)), region="trousers"))
    A(shape("shank-l", "shank-l", "body", 4.02, 0.42, path(limb(0, [(0, .065, .065), (0.12, .063, .068), (0.26, .057, .058), (LS - 0.01, .052, .052)], 0.02, 0.0)), region="trousers"))
    A(shape("patch-l", "shank-l", "detail", 4.03, 0.35, ellipse(0.045, 0.0, 0.040, 0.044), filled=False, region="trousers"))
    A(shape("clog-l", "foot-l", "body", 4.05, 0.3, path(CLOG), region="clogs"))
    A(shape("clog-strap-l", "foot-l", "detail", 4.06, 0.2, poly([(-0.02, 0.040), (0.0, -0.040)]), filled=False, region="clogs"))
    A(shape("shirt", "torso", "body", 3.0, 0.88, path(limb(0, tab([(-0.07, 0.100, 0.112, "c"), (0.0, 0.105, 0.115), (0.10, 0.105, 0.118), (0.22, 0.100, 0.125), (0.32, 0.095, 0.130), (0.40, 0.085, 0.108), (LTO, 0.062, 0.058)]))), region="shirt-torso"))
    A(shape("head", "head", "body", 6.0, 0.86, path(head(F)), region="face"))
    A(shape("eye", "head", "detail", 6.2, 0.1, ellipse(0.186 * F, -0.081 * F, 0.012, 0.014), region="face"))
    A(shape("hat-crown", "head", "body", 6.4, 0.78, path([(0.205, -0.092, "c"), (0.245, -0.095), (0.292, -0.060), (0.312, 0.0), (0.292, 0.062), (0.245, 0.098), (0.205, 0.100, "c")]), region="hat"))
    A(shape("hat-brim", "head", "body", 6.3, 0.8, path([(0.215, -0.26, "c"), (0.226, -0.20), (0.226, 0.20), (0.215, 0.26, "c"), (0.186, 0.20), (0.186, -0.20)]), region="hat"))
    A(shape("hat-band", "head", "detail", 6.5, 0.25, poly([(0.230, -0.094), (0.230, 0.102)]), filled=False, region="hat"))
    for side, z in (("l", 4.5), ("r", 3.6)):
        if side == "r":
            A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.86, path(limb(0, [(0.0, .046, .046, "c"), (0.15, .047, .047), (LU, .041, .041)], 0.0, 0.01)), region="shirt-sleeves"))
        else:
            A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.86, path(limb(0, [(0, .050, .050), (0.15, .047, .047), (LU, .041, .041)], 0.06, 0.01)), region="shirt-sleeves"))
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.86, path(limb(0, [(0, .040, .040), (0.12, .037, .037), (LF - 0.01, .029, .029), (LF + 0.05, .041, .037), (LF + 0.10, .038, .031), (LF + 0.16, .024, .020)], 0.012, 0.02)), regions=["forearms", "gloves"]))
    A(shape("trowel-blade", "trowel", "body", 4.8, 0.7, path([(0.0, -0.022, "c"), (0.0, 0.022, "c"), (0.04, 0.030), (0.19, 0.040), (0.26, 0.028), (0.30, 0.0, "c"), (0.26, -0.020), (0.19, -0.030), (0.10, -0.022)]), region="trowel"))
    return s


GLOVE50 = [(-0.045, 0.034, "c"), (0.02, 0.040), (0.06, 0.047), (0.095, 0.058), (0.118, 0.050), (0.122, 0.040), (0.150, 0.034), (0.178, 0.018), (0.185, 0.0), (0.176, -0.020), (0.140, -0.033), (0.08, -0.036), (0.02, -0.036), (-0.045, -0.034, "c")]


def s50():
    s = props(50)
    A = s.append
    A(shape("thigh-l", "thigh-l", "body", 4.0, 0.42, path(limb(0, [(0, .105, .103), (0.14, .095, .093), (0.27, .081, .079), (LT, .066, .064)], 0.09, 0.012)), region="trousers"))
    A(shape("shank-l", "shank-l", "body", 4.02, 0.42, path(limb(0, [(0, .065, .065), (0.12, .063, .068), (0.26, .057, .058), (LS - 0.01, .052, .052)], 0.02, 0.008)), region="trousers"))
    A(shape("knee-patch-l", "shank-l", "detail", 4.03, 0.35, ellipse(0.05, 0.0, 0.034, 0.040), filled=False, region="trousers"))
    A(shape("clog-l", "foot-l", "body", 4.05, 0.3, path(CLOG), region="clogs", material="rubber-clog"))
    A(shape("clog-strap-l", "foot-l", "detail", 4.06, 0.2, poly([(-0.02, 0.040), (0.0, -0.040)]), filled=False, region="clogs"))
    for side in ("r", "l"):
        z = 3.6 if side == "r" else 4.5
        if side == "r":
            A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.88, path(limb(0, [(0.04, .046, .046, "c"), (0.15, .047, .047), (LU, .042, .042)], 0.0, 0.01)), region="shirt-sleeves", material="linen"))
        else:
            A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.88, path(limb(0, [(0.05, .050, .050, "c"), (0.15, .047, .047), (LU, .042, .042)], 0.0, 0.01)), region="shirt-sleeves", material="linen"))
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.86, path(limb(0, [(0, .040, .040), (0.12, .037, .037), (LF - 0.01, .028, .028)], 0.012, 0.03)), region="forearms", material="skin"))
        A(shape(f"glove-{side}", f"hand-{side}", "body", z + 0.04, 0.6, path(GLOVE50), region="gloves", material="glove"))
    A(shape("shirt", "torso", "body", 3.0, 0.88, path(limb(0, tab([(-0.07, 0.100, 0.112, "c"), (0.0, 0.105, 0.115), (0.10, 0.105, 0.118), (0.22, 0.100, 0.125), (0.32, 0.095, 0.130), (0.40, 0.085, 0.108), (LTO, 0.062, 0.058)]))), region="shirt-torso", material="linen"))
    A(shape("head", "head", "body", 6.0, 0.86, path(head(F)), region="face", material="skin"))
    A(shape("eye", "head", "detail", 6.2, 0.1, ellipse(0.186 * F, -0.081 * F, 0.012, 0.014), region="face"))
    A(shape("hat-crown", "head", "body", 6.4, 0.78, path([(0.205, -0.092, "c"), (0.245, -0.095), (0.292, -0.060), (0.312, 0.0), (0.292, 0.062), (0.245, 0.098), (0.205, 0.100, "c")]), region="hat", material="straw", hatch="straw-weave"))
    A(shape("hat-brim", "head", "body", 6.3, 0.8, path([(0.215, -0.22, "c"), (0.226, -0.17), (0.226, 0.17), (0.215, 0.22, "c"), (0.186, 0.17), (0.186, -0.17)]), region="hat", material="straw"))
    A(shape("trowel-blade", "trowel", "body", 4.8, 0.7, path([(0.03, 0.0, "c"), (0.07, 0.030), (0.19, 0.040), (0.26, 0.028), (0.30, 0.0, "c"), (0.26, -0.020), (0.19, -0.030), (0.095, -0.022)]), region="trowel", material="steel"))
    return s


def all_shapes():
    return {200: s200(), 100: s100(), 50: s50()}
