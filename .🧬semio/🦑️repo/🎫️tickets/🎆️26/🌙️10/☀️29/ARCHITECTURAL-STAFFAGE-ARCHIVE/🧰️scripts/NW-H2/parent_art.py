"""Parent carrying a toddler: drawings at 1:200, 1:100 and 1:50 (1:500 is unavailable), one table set per scale."""
from lib import *
from art import *
from parent_rig import H, L, TL

F = H / 1.82
LT, LS, LU, LF, LH, LTO = L["thigh"], L["shank"], L["upper"], L["fore"], L["hand"], L["torso"]

TAB = {
    200: dict(thigh=[(0, .092, .090), (0.22, .082, .080), (LT, .064, .062)], shank=[(0, .064, .064), (0.16, .062, .066), (LS - 0.01, .052, .052)], uarm=[(0, .056, .056), (LU, .046, .046)], farm=[(0, .047, .047), (LF - 0.01, .036, .036)],
              coat=[(0.0, .128, .122, "c"), (0.16, .130, .124), (0.34, .132, .128), (0.41, .114, .102), (LTO - 0.02, .072, .068)], skirt=-0.30),
    100: dict(thigh=[(0, .095, .093), (0.15, .088, .086), (0.28, .078, .076), (LT, .063, .061)], shank=[(0, .063, .063), (0.12, .061, .067), (0.26, .055, .056), (LS - 0.01, .051, .051)], uarm=[(0, .055, .055), (0.14, .051, .051), (LU, .045, .045)], farm=[(0, .046, .046), (0.12, .042, .042), (LF - 0.01, .036, .036)],
              coat=[(0.0, .126, .120, "c"), (0.12, .128, .122), (0.24, .124, .122), (0.34, .130, .126), (0.41, .112, .100), (LTO - 0.02, .074, .070), (LTO + 0.025, .066, .064)], skirt=-0.31),
    50: dict(thigh=[(0, .090, .088), (0.15, .087, .085), (0.28, .077, .075), (LT, .062, .060)], shank=[(0, .062, .062), (0.12, .060, .066), (0.26, .054, .055), (LS - 0.01, .050, .050)], uarm=[(0, .054, .054), (0.14, .050, .050), (LU, .044, .044)], farm=[(0, .045, .045), (0.12, .041, .041), (LF - 0.01, .035, .035)],
             coat=[(0.0, .126, .120, "c"), (0.12, .128, .122), (0.24, .124, .122), (0.34, .130, .126), (0.41, .112, .100), (LTO - 0.02, .074, .070), (LTO + 0.025, .066, .064)], skirt=-0.31),
}
TT = {
    200: dict(suit=[(0, .084, .082), (0.12, .092, .087), (0.22, .088, .078), (TL["torso"] + 0.01, .058, .058)], thigh=[(0, .048, .048), (0.13, .043, .043)], arm=[(0, .037, .037), (0.17, .031, .031)]),
    100: dict(suit=[(0, .082, .080), (0.12, .090, .085), (0.22, .086, .076), (TL["torso"] + 0.01, .056, .056)], thigh=[(0, .047, .047), (0.13, .042, .042)], arm=[(0, .036, .036), (0.12, .033, .033), (0.17, .030, .030)]),
    50: dict(suit=[(0, .080, .078), (0.06, .086, .082), (0.14, .090, .084), (0.22, .085, .075), (TL["torso"] + 0.01, .055, .055)], thigh=[(0, .046, .046), (0.07, .045, .045), (0.13, .041, .041)], arm=[(0, .035, .035), (0.10, .033, .033), (0.17, .029, .029)]),
}


def toddler_shapes(level):
    s = []
    A = s.append
    t = TT[level]
    A(shape("toddler-suit", "toddler-torso", "body", 4.2, 0.35, path(limb(0, t["suit"], 0.045, 0.0)), region="toddler-suit", material="padded-suit"))
    hd = [(x * 0.68, y * 0.86) for x, y in HEAD]
    if level == 200:
        hd = [(x * 0.68, y * 0.90) for x, y in HEAD[::2]] + [(0.0, -0.043)]
    A(shape("toddler-head", "toddler-head", "body", 4.6, 0.86, path(hd), region="toddler-face", material="skin"))
    A(shape("toddler-hat", "toddler-head", "body", 4.7, 0.3, path([(0.130, -0.075, "c"), (0.168, -0.070), (0.192, -0.030), (0.200, 0.012), (0.186, 0.065), (0.150, 0.082), (0.105, 0.080, "c"), (0.118, 0.020), (0.128, -0.040)]), region="toddler-hat", material="knit-hat"))
    for sd, zb in (("n", 4.4), ("f", 3.0)):
        if sd == "f":
            continue
        A(shape(f"toddler-thigh-{sd}", f"toddler-thigh-{sd}", "body", zb, 0.35, path(limb(0, t["thigh"], 0.03, 0.012)), region="toddler-suit", material="padded-suit"))
        A(shape(f"toddler-shank-{sd}", f"toddler-shank-{sd}", "body", zb + 0.01, 0.35, path(limb(0, [(0, .042, .042), (0.10, .038, .038)], 0.02, 0.0)), region="toddler-suit", material="padded-suit"))
        A(shape(f"toddler-boot-{sd}", f"toddler-shank-{sd}", "body", zb + 0.02, 0.2, path([(0.100, -0.036, "c"), (0.100, 0.038), (0.140, 0.044), (0.160, 0.092), (0.200, 0.100), (0.208, 0.06, "c"), (0.206, -0.036, "c")]), region="toddler-boots", material="rubber"))
    for sd, zb in (("n", 4.9), ("f", 3.1)):
        if sd == "f":
            continue
        if True:
            A(shape(f"toddler-arm-{sd}", f"toddler-arm-{sd}", "body", zb, 0.35, path(limb(0, t["arm"] + [(0.215, 0.030, 0.030)], 0.08, 0.025)), regions=["toddler-suit", "toddler-mittens"], material="padded-suit"))
    return s


def parent_shapes(level):
    s = []
    A = s.append
    f = F
    t = TAB[level]
    for side, zb in (("r", 1.0), ("l", 4.0)):
        A(shape(f"thigh-{side}", f"thigh-{side}", "body", zb, 0.3, path(limb(0, t["thigh"], 0.09, 0.012)), region="jeans", material="denim"))
        A(shape(f"shank-{side}", f"shank-{side}", "body", zb + 0.02, 0.3, path(limb(0, t["shank"], 0.02, 0.008)), region="jeans", material="denim"))
        up = [(-0.058, -0.0772, "c"), (-0.060, 0.0), (-0.048, 0.060), (0.020, 0.072), (0.090, 0.040), (0.150, -0.008), (0.205, -0.036), (0.222, -0.0772, "c")]
        if level == 200:
            up = [(-0.058, -0.0772, "c"), (-0.060, 0.02), (0.02, 0.07), (0.15, -0.01), (0.22, -0.04), (0.222, -0.0772, "c")]
        A(shape(f"boot-{side}", f"foot-{side}", "body", zb + 0.05, 0.2, path(up), region="boots", material="leather"))
        z = 3.6 if side == "r" else 4.2
        zu = 3.0 if side == "r" else 3.1
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", zu, 0.3, path(limb(0, [(0, 0.044, 0.044)] + t["uarm"][1:], 0.03, 0.01)), region="coat-sleeves", material="wool"))
        if True:
            fa = t["farm"][:-1] + [(LF - 0.01, t["farm"][-1][1], t["farm"][-1][2]), (LF + 0.07, 0.040, 0.038), (LF + LH - 0.02, 0.026, 0.024)]
            A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.3, path(limb(0, fa, 0.012, 0.012)), regions=["coat-sleeves", "gloves"], material="wool"))
    sk = t["skirt"]
    A(shape("coat-skirt", "pelvis", "body", 3.2, 0.3, lines([(0.16, -0.140, "c"), (0.16, 0.136, "c"), (sk, 0.15, "c"), (sk, -0.16 if level != 50 else -0.175, "c")]), region="coat-torso", material="wool"))
    A(shape("coat", "torso", "body", 3.4, 0.3, path(limb(0, t["coat"])), region="coat-torso" if level == 50 else None, regions=None if level == 50 else ["coat-torso", "scarf"], material="wool"))
    if level == 50:
        A(shape("scarf", "torso", "body", 3.5, 0.55, path([(0.38, -0.100, "c"), (0.43, -0.105), (0.470, -0.075), (0.475, 0.03), (0.44, 0.080), (0.39, 0.088, "c")]), region="scarf", material="knit-hat"))
    hd = head(f) if level != 200 else [(x * f, y * f) for x, y in HEAD[::2]] + [(0.0, -0.047)]
    A(shape("head", "head", "body", 6.0, 0.86, path(hd), region="face", material="skin"))
    A(shape("beanie", "head", "body", 6.4, 0.4, path([(0.200, -0.093, "c"), (0.250, -0.083), (0.280, -0.040), (0.298, 0.012), (0.278, 0.076), (0.226, 0.104), (0.165, 0.104), (0.122, 0.086, "c")]), region="beanie", material="knit-hat"))
    if level == 50:
        A(shape("eye", "head", "detail", 6.2, 0.1, ellipse(0.186 * f, -0.081 * f, 0.006, 0.0075), region="face"))
        A(shape("ear", "head", "detail", 6.5, 0.7, ellipse(0.148 * f, 0.016, 0.026 * f, 0.013), filled=False, region="face"))
    return s


def all_shapes():
    return {sc: parent_shapes(sc) + toddler_shapes(sc) for sc in (200, 100, 50)}
