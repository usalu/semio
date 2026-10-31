"""Seated guitarist drawings, one function per scale (local frames: see human.py; guitar local x along the neck, +y = top face side)."""
from lib import *
from art import *
from guit_rig import H, L, SEAT, RUNG, GUITAR_L

F = H / 1.82
LT, LS, LU, LF, LH, LTO = L["thigh"], L["shank"], L["upper"], L["fore"], L["hand"], L["torso"]
PELVIS = [(0.052, -0.118), (0.052, 0.122), (-0.04, 0.128), (-0.088, 0.075), (-0.098, 0.0), (-0.075, -0.092), (0.0, -0.122)]
BOOT = [(-0.058, -0.0775, "c"), (-0.060, -0.005), (-0.045, 0.05), (0.030, 0.060), (0.100, 0.030), (0.165, -0.010), (0.215, -0.040), (0.232, -0.0775, "c")]
BODY_TOP = [(-0.215, 0.0, "c"), (-0.205, 0.095), (-0.14, 0.175), (-0.04, 0.192), (0.06, 0.168), (0.13, 0.128), (0.20, 0.140), (0.275, 0.134), (0.322, 0.075), (0.335, 0.026, "c")]


def tab(rows):
    return [(q[0], q[2], q[1]) + q[3:] for q in rows]


def body_path(k=1.0, wk=1.0):
    top = [(x * k, y * wk) + tuple(c) for (x, y, *c) in BODY_TOP]
    bot = [(x, -y) + tuple(c) for (x, y, *c) in reversed(top[1:-1])]
    return top + bot


def stool(level):
    s = []
    A = s.append
    if level == 500:
        A(shape("stool-seat", "stool", "body", 0.6, 0.5, rect(-0.20, SEAT - 0.10, 0.40, 0.10, 0.02), region="stool"))
        A(shape("stool-leg-f", "stool", "body", 0.5, 0.5, lines([(0.08, SEAT - 0.09), (0.19, SEAT - 0.09), (0.22, 0.0), (0.12, 0.0)]), region="stool"))
        A(shape("stool-leg-r", "stool", "body", 0.5, 0.5, lines([(-0.19, SEAT - 0.09), (-0.08, SEAT - 0.09), (-0.12, 0.0), (-0.22, 0.0)]), region="stool"))
        return s
    w = {200: 0.046, 100: 0.042, 50: 0.040}[level]
    top = SEAT - 0.04
    t = (top - (RUNG - 0.014)) / top
    xo = 0.14
    xi = xo - w
    foot_o = 0.205 + (w - 0.04) / 2
    foot_i = foot_o - w
    def xin(y):
        k = (top - y) / top
        return xi + (foot_i - xi) * k
    outer = [(-0.19, SEAT, "c"), (0.19, SEAT, "c"), (0.19, top, "c"), (xo, top, "c"), (foot_o, 0.0, "c"), (foot_i, 0.0, "c"), (xin(RUNG - 0.014), RUNG - 0.014, "c"), (-xin(RUNG - 0.014), RUNG - 0.014, "c"), (-foot_i, 0.0, "c"), (-foot_o, 0.0, "c"), (-xo, top, "c"), (-0.19, top, "c")]
    hole = [(xin(RUNG + 0.014), RUNG + 0.014, "c"), (xi, top, "c"), (-xi, top, "c"), (-xin(RUNG + 0.014), RUNG + 0.014, "c")]
    d = lines(outer)["d"] + " " + lines(hole)["d"]
    A(shape("stool", "stool", "body", 0.6, 0.62, {"kind": "path", "d": d}, region="stool", material="pine"))
    return s


def guitar(level):
    s = []
    A = s.append
    if level == 500:
        A(shape("guitar", "guitar", "body", 4.2, 0.35, path(body_path(1.0, 0.95)[:10] + [(0.64, 0.05), (0.78, 0.05, "c"), (0.78, -0.05, "c"), (0.64, -0.05)] + body_path(1.0, 0.95)[10:]), region="guitar"))
        return s
    A(shape("guitar-body", "guitar", "body", 3.9, 0.72, path(body_path()), region="guitar", material="spruce"))
    nk = 0.030 if level == 200 else 0.024
    A(shape("guitar-neck", "guitar", "body", 3.85, 0.35, lines([(0.31, nk, "c"), (0.64, nk * 0.9, "c"), (0.64, -nk * 0.9, "c"), (0.31, -nk, "c")]), region="guitar", material="mahogany"))
    A(shape("guitar-head", "guitar", "body", 3.86, 0.3, path([(0.64, 0.028, "c"), (0.78, 0.044, "c"), (0.78, -0.044, "c"), (0.64, -0.028, "c")]), region="guitar", material="mahogany"))
    if level <= 100:
        A(shape("soundhole", "guitar", "detail", 3.95, 0.1, ellipse(0.075, 0.0, 0.052, 0.062), region="guitar"))
        A(shape("bridge", "guitar", "detail", 3.95, 0.2, rect(-0.115, -0.07, 0.03, 0.14, 0.005), region="guitar"))
    if level == 50:
        for i, x in enumerate((0.36, 0.41, 0.46, 0.505, 0.55, 0.59)):
            A(shape(f"fret-{i}", "guitar", "texture", 3.96, 0.2, poly([(x, 0.020), (x, -0.020)]), filled=False, region="guitar"))
        A(shape("string", "guitar", "texture", 3.97, 0.2, poly([(-0.10, 0.008), (0.62, 0.012)]), filled=False, region="guitar"))
        for i, y in enumerate((0.020, 0.0, -0.018)):
            A(shape(f"tuner-{i}", "guitar", "detail", 3.95, 0.3, ellipse(0.72 + i * 0.02, y, 0.007, 0.007), region="guitar"))
    return s


def person(level):
    s = []
    A = s.append
    f = F
    for side, zb in (("r", 1.0), ("l", 4.0)):
        if level == 500:
            A(shape(f"thigh-{side}", f"thigh-{side}", "body", zb, 0.15, path(limb(0, [(0, .098, .096), (LT, .074, .072)], 0.03, 0.02)), region="jeans"))
            A(shape(f"shank-{side}", f"shank-{side}", "body", zb + 0.02, 0.15, path(limb(0, [(0, .072, .072), (LS - 0.01, .058, .058)], 0.02, 0.0)), region="jeans"))
            A(shape(f"boot-{side}", f"foot-{side}", "body", zb + 0.05, 0.12, path([(-0.062, -0.0775, "c"), (-0.062, 0.03), (0.05, 0.06), (0.15, 0.0), (0.235, -0.04), (0.235, -0.0775, "c")]), region="boots"))
            A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", (1.5 if side == "r" else 4.5), 0.2, path(limb(0, [(0, .056, .056), (LU, .050, .050)], 0.03, 0.01)), region="shirt-sleeves"))
            A(shape(f"forearm-{side}", f"forearm-{side}", "body", (1.52 if side == "r" else 4.52), 0.2, path(limb(0, [(0, .052, .052), (LF + LH - 0.03, .050, .050)], 0.01, 0.03)), regions=["shirt-sleeves", "hands"]))
            continue
        tabs = {200: [(0, .098, .096), (0.2, .088, .086), (LT, .070, .068)], 100: [(0, .098, .096), (0.14, .094, .092), (0.27, .084, .082), (LT, .069, .067)], 50: [(0, .098, .096), (0.14, .094, .092), (0.27, .084, .082), (LT, .069, .067)]}
        th = tabs[level] if side == "l" else [(0.03, tabs[level][0][1] - 0.006, tabs[level][0][2] - 0.006, "c")] + tabs[level][1:]
        A(shape(f"thigh-{side}", f"thigh-{side}", "body", zb, 0.3, path(limb(0, th, 0.09 if side == "l" else 0.0, 0.012)), region="jeans", material="denim"))
        A(shape(f"shank-{side}", f"shank-{side}", "body", zb + 0.02, 0.3, path(limb(0, [(0, .068, .068), (0.12, .066, .072), (0.26, .060, .061), (LS - 0.01, .056, .056)] if level != 200 else [(0, .068, .068), (0.15, .066, .070), (LS - 0.01, .056, .056)], 0.02, 0.008)), region="jeans", material="denim"))
        A(shape(f"boot-{side}", f"foot-{side}", "body", zb + 0.05, 0.18, path(BOOT), region="boots", material="leather"))
        if level <= 100:
            A(shape(f"boot-sole-{side}", f"foot-{side}", "detail", zb + 0.06, 0.1, poly([(-0.054, -0.060), (0.226, -0.060)]), filled=False, region="boots"))
        z = 3.6 if side == "r" else 4.5
        st0 = 0.05 if side == "l" else 0.03
        A(shape(f"upper-arm-{side}", f"upper-arm-{side}", "body", z, 0.62, path(limb(0, [(st0, .054, .054, "c"), (0.15, .052, .052), (LU, .045, .045)], 0.0, 0.01)), region="shirt-sleeves", material="flannel"))
        A(shape(f"forearm-{side}", f"forearm-{side}", "body", z + 0.02, 0.62, path(limb(0, [(0, .045, .045), (0.12, .041, .041), (LF - 0.01, .033, .033)], 0.012, 0.008)), region="shirt-sleeves", material="flannel"))
        hs = mitten_st(f, 1.0) if level >= 100 else [(0.0, .036, .034), (0.08, .040, .034), (0.15, .026, .021)]
        A(shape(f"hand-{side}", f"hand-{side}", "body", z + 0.04, 0.85, path(limb(0, hs, 0.0, 0.012)), region="hands", material="skin"))
        if level == 50:
            A(shape(f"thumb-{side}", f"hand-{side}", "body", z + 0.05, 0.85, thumb(f), region="hands", material="skin"))
            A(shape(f"cuff-{side}", f"forearm-{side}", "detail", z + 0.03, 0.3, poly([(LF - 0.03, 0.032), (LF - 0.03, -0.032)]), filled=False, region="shirt-sleeves"))
    rows = {500: [(0.0, .122, .128, "c"), (0.2, .124, .130), (0.34, .130, .125), (LTO + 0.02, .075, .075)], 200: [(0.0, .122, .128, "c"), (0.16, .124, .130), (0.34, .130, .125), (0.41, .118, .105), (LTO, .066, .070)], 100: [(0.0, .122, .128, "c"), (0.11, .125, .128), (0.24, .122, .130), (0.34, .130, .125), (0.41, .118, .105), (LTO, .066, .070)], 50: [(0.0, .122, .128, "c"), (0.11, .125, .128), (0.24, .122, .130), (0.34, .130, .125), (0.41, .118, .105), (LTO, .066, .070)]}[level]
    A(shape("shirt", "torso", "body", 3.0, 0.62 if level != 500 else 0.15, path(limb(0, tab(rows))), region="shirt-torso", material="flannel"))
    if level == 50:
        A(shape("shirt-open", "torso", "detail", 3.1, 0.2, poly([(0.05, -0.112), (0.20, -0.108), (0.38, -0.090)]), filled=False, region="shirt-torso"))
        A(shape("pocket-flap", "torso", "detail", 3.1, 0.2, poly([(0.26, -0.126), (0.26, -0.070), (0.30, -0.070)]), filled=False, region="shirt-torso"))
    if level == 500:
        A(shape("head", "head", "body", 6.0, 0.25, path([(0.0, 0.062), (0.12, 0.088), (0.22, 0.100), (0.285, 0.0), (0.23, -0.085), (0.16, -0.108), (0.09, -0.092), (0.06, -0.060), (0.0, -0.058)]), regions=["face", "hair"]))
        return s
    A(shape("head", "head", "body", 6.0, 0.85, path(head(f)), region="face", material="skin"))
    A(shape("hair", "head", "body", 6.4, 0.18, path([(0.205, -0.092, "c"), (0.250, -0.082), (0.280, -0.040), (0.294, 0.012), (0.272, 0.078), (0.222, 0.104), (0.165, 0.106), (0.118, 0.090, "c"), (0.150, 0.040), (0.178, -0.020)]), region="hair", material="hair"))
    if level == 50:
        A(shape("ear", "head", "detail", 6.5, 0.7, ellipse(0.148 * f, 0.016, 0.026 * f, 0.013), filled=False, region="face"))
    return s


def all_shapes():
    return {sc: person(sc) + guitar(sc) + stool(sc) for sc in (500, 200, 100, 50)}
