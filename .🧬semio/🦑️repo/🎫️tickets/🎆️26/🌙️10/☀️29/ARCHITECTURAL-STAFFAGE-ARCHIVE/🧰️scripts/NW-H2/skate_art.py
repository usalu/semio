"""Skateboarder drawings at 1:100 and 1:50 (1:500 and 1:200 are declared unavailable). Local frames: see human.py.
Design rules learned from the print measure: joints are T-junctions (child below parent, or flat starts), outlines of neighbouring parts cross instead of running parallel."""
from lib import *
from art import *
from art import HEAD
from skate_rig import H, WB, WHEEL_R, WHEEL_Y, DECK_TOP, build_rig

_, L = build_rig()
F = H / 1.82
LT, LS, LU, LF, LH, LTO = L["thigh"], L["shank"], L["upper"], L["fore"], L["hand"], L["torso"]
SOLE = -0.016
import json, os
PARAMS = {"stl": 0.05, "str": 0.03}
if os.path.exists("/tmp/nwh2/skate_params.json"):
    PARAMS.update(json.load(open("/tmp/nwh2/skate_params.json")))


def deck_pts(thick=0.012, tip=0.405, flat=0.28, rise=0.075, n=6):
    def c(x):
        d = max(0.0, (abs(x) - flat) / (tip - flat))
        return rise * d ** 1.6

    xs = [-tip + (tip - flat) * i / n for i in range(n + 1)] + [-flat + 2 * flat * i / 4 for i in range(1, 4)] + [flat + (tip - flat) * i / n for i in range(n + 1)]
    top = [(x, c(x) + thick) for x in xs]
    bot = [(x, c(x)) for x in reversed(xs)]
    for lst in (top, bot):
        lst[0] = lst[0] + ("c",)
        lst[-1] = lst[-1] + ("c",)
    return top + bot


def deck_truck_pts(thick, level):
    """Deck outline with the two truck blocks hanging below it as one closed outline."""
    def c(x):
        d = max(0.0, (abs(x) - 0.28) / 0.125)
        return 0.075 * d ** 1.6

    xs = [-0.405, -0.37, -0.33, -0.28]
    top = [(x, c(x) + thick) for x in [-0.405, -0.37, -0.33, -0.28, -0.14, 0.0, 0.14, 0.28, 0.33, 0.37, 0.405]]
    top[0] = top[0] + ("c",)
    top[-1] = top[-1] + ("c",)
    wy = WHEEL_Y - 0.004
    hw = 0.034 if level == 100 else 0.031
    right_under = [(0.405, c(0.405), "c"), (0.37, c(0.37)), (0.33, c(0.33)), (0.28, 0.0, "c")]
    bot = []
    # underside from nose to tail with two truck blocks
    bot += right_under
    bot += [(WB + 0.040, 0.0, "c"), (WB + hw, wy, "c"), (WB - hw, wy, "c"), (WB - 0.040, 0.0, "c")]
    bot += [(-WB + 0.040, 0.0, "c"), (-WB + hw, wy, "c"), (-WB - hw, wy, "c"), (-WB - 0.040, 0.0, "c")]
    bot += [(-0.28, 0.0, "c"), (-0.33, c(0.33)), (-0.37, c(0.37)), (-0.405, c(0.405), "c")]
    return top + bot


def board(level):
    s = []
    A = s.append
    A(shape("deck", "board", "body", 0.5, 0.82, path(deck_truck_pts(0.046 if level == 100 else 0.028, level)), regions=["deck", "trucks"], material="maple"))
    for sd, x in (("f", WB), ("r", -WB)):
        A(shape(f"wheel-{sd}", f"wheel-{sd}", "body", 0.3, 0.9, ellipse(0, 0, WHEEL_R, WHEEL_R), region="wheels", material="polyurethane"))
    return s


def shoe(side, z, level):
    f = F
    up = [(-0.058 * f, SOLE - 0.0808 * f + 0.0808 * f, "c")]
    a = 0.0808 * f
    pts = [(-0.058 * f, -a + SOLE + 0.0, "c"), (-0.060 * f, -0.010 * f), (-0.052 * f, 0.034 * f), (0.020 * f, 0.040 * f), (0.060 * f, 0.018 * f), (0.110 * f, -0.010 * f), (0.170 * f, -a + 0.060 * f), (0.236 * f - 0.012, -a + 0.040 * f), (0.240 * f, -a + SOLE, "c")]
    o = [shape(f"shoe-{side}", f"foot-{side}", "body", z, 0.92, path(pts), region="shoes", material="skate-shoe")]
    if level == 50:
        o.append(shape(f"sole-line-{side}", f"foot-{side}", "detail", z + 0.01, 0.2, poly([(-0.045 * f, -a + 0.026 * f), (0.225 * f, -a + 0.026 * f)]), filled=False, region="shoes"))
    return o


def hoodie_rows(level):
    base = [(-0.075, 0.120, 0.112, "c"), (0.0, 0.134, 0.124), (0.10, 0.140, 0.126), (0.22, 0.146, 0.120), (0.33, 0.150, 0.114), (0.41, 0.128, 0.100), (LTO, 0.066, 0.078)]
    if level == 100:
        base = [(-0.075, 0.118, 0.110, "c"), (0.0, 0.132, 0.122), (0.12, 0.141, 0.124), (0.25, 0.148, 0.118), (0.35, 0.146, 0.110), (0.42, 0.124, 0.098), (LTO, 0.066, 0.078)]
    return base


def person(level):
    s = []
    A = s.append
    f = F
    th = {100: [(0, .104, .102), (0.16, .097, .095), (0.30, .084, .082), (LT, .071, .069)], 50: [(0, .105, .103), (0.10, .099, .097), (0.22, .089, .087), (0.33, .080, .078), (LT, .071, .069)]}[level]
    sh = {100: [(0, .070, .070), (0.12, .068, .074), (0.30, .060, .060), (LS - 0.012, .061, .061)], 50: [(0, .070, .070), (0.10, .068, .074), (0.22, .062, .063), (0.33, .058, .058), (LS - 0.012, .061, .061)]}[level]
    for side, zb in (("r", 1.0), ("l", 4.0)):
        thl = th if side == "l" else [(0.03, th[0][1] - 0.006, th[0][2] - 0.006, "c")] + th[1:]
        A(shape(f"thigh-{side}", f"thigh-{side}", "body", zb, 0.38, path(limb(0, thl, 0.09 if side == "l" else 0.0, 0.014)), region="trousers"))
        A(shape(f"shank-{side}", f"shank-{side}", "body", zb + 0.02, 0.38, path(limb(0, sh, 0.02, 0.008)), region="trousers"))
        for x in shoe(side, zb + 0.05, level):
            A(x)
        if level == 50:
            A(shape(f"cuff-{side}", f"shank-{side}", "detail", zb + 0.03, 0.3, poly([(0.352, 0.052), (0.352, -0.052)]), filled=False, region="trousers"))
        z = 3.6 if side == "r" else 4.5
        st0 = PARAMS["stl"] if side == "l" else PARAMS["str"]
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.55, path(limb(0, [(st0, .056, .056, "c"), (0.14 if level == 100 else 0.10, .054, .055), (LU, .046, .046)], 0.0, 0.018)), region="hoodie-sleeves"))
        fa = [(0, .046, .046), (0.12, .042, .042), (LF - 0.012, .035, .035), (LF + 0.05, .042, .037), (LF + 0.11, .040, .031), (LF + LH - 0.02, .022, .018)]
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.62, path(limb(0, fa, 0.012, 0.012)), regions=["hoodie-sleeves", "hands"]))
    A(shape("hoodie", "torso", "body", 3.0, 0.55, path(limb(0, [(q[0], q[2], q[1]) + q[3:] for q in hoodie_rows(level)])), region="hoodie-torso"))
    hood = [(0.392, 0.098, "c"), (0.425, 0.128), (0.470, 0.122), (0.503, 0.088), (0.492, 0.046), (0.455, 0.030, "c")]
    A(shape("hood", "torso", "body", 3.2, 0.5, path(hood), region="hoodie-torso"))
    A(shape("head", "head", "body", 6.0, 0.86, path(head(f)), region="face", material="skin"))
    A(shape("beanie", "head", "body", 6.4, 0.22, path([(0.205 * f, -0.093 * f, "c"), (0.252 * f, -0.083 * f), (0.278 * f, -0.040 * f), (0.296 * f, 0.012), (0.274 * f, 0.074), (0.226 * f, 0.102), (0.168 * f, 0.103), (0.128 * f, 0.084, "c")]), region="beanie", material="knit", hatch="knit-rib" if level == 50 else None))
    if level == 50:
        A(shape("eye", "head", "detail", 6.2, 0.1, ellipse(0.186 * f, -0.081 * f, 0.012, 0.014), region="face"))
    return s


def all_shapes():
    return {sc: board(sc) + person(sc) for sc in (100, 50)}
