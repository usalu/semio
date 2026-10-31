# [scratch] NW-A1 sheep representations (shapes only; pens/decisions in sheep.py)
from lib import *

TR, NK, UL, LL, FA, EA, TA, HO = "fleece-trunk", "fleece-neck", "wool-upper-legs", "lower-legs", "face", "ears", "tail", "hooves"
HOOF_REL = [(-0.050, 0.058), (-0.020, 0.066), (0.002, 0.040), (0.032, 0.0), (-0.028, 0.0), (-0.034, 0.018), (-0.046, 0.040)]
HOOF_REL_S = [(-0.045, 0.055), (0.002, 0.045), (0.034, 0.0), (-0.030, 0.0), (-0.044, 0.030)]
HOOF_BIG = [(a * 1.35, b * 1.35) for a, b in HOOF_REL]
HOOF_BIG_S = [(a * 1.4, b * 1.4) for a, b in HOOF_REL_S]
HOOF_BLOCK = [(-0.05, 0.075), (0.045, 0.055), (0.052, 0.0), (-0.05, 0.0)]


def whoof(sh, id, bone, z, tone, rel=None, **kw):
    rel = HOOF_BIG if rel is None else rel
    e = sh.sk.end(bone)
    pts = [sh.sk.local(bone, (e[0] + a, e[1] + b)) for a, b in rel]
    sh.poly(id, bone, pts, z, tone, region=HO, **kw)


def sides():
    return (("f", 1.0, 0.55), ("n", 5.0, 0.72))


def trunk50(sh):
    sh.blob("hindbody", "lumbar", [(0.62, 0.762), (0.50, 0.757), (0.38, 0.752), (0.30, 0.758), (0.20, 0.756), (0.10, 0.752), (0.02, 0.742), (-0.06, 0.728), (-0.12, 0.700), (-0.155, 0.655), (-0.165, 0.60), (-0.155, 0.54), (-0.10, 0.47), (-0.02, 0.40), (0.08, 0.36), (0.18, 0.335), (0.28, 0.325), (0.38, 0.322), (0.50, 0.318), (0.62, 0.33), (0.62, 0.55)], 3.0, 0.72, world=True, region=TR, hatch="fleece")
    sh.blob("chest", "thorax", [(0.44, 0.764), (0.50, 0.766), (0.58, 0.768), (0.64, 0.755), (0.70, 0.725), (0.735, 0.66), (0.745, 0.58), (0.745, 0.50), (0.74, 0.43), (0.69, 0.37), (0.62, 0.335), (0.52, 0.318), (0.44, 0.316), (0.44, 0.54)], 3.2, 0.72, world=True, region=TR, hatch="fleece")


def head50(sh, ear_len=0.095):
    sh.shell("neck", "neck",  [(-0.08, 0.11, -0.10), (0.0, 0.105, -0.145), (0.08, 0.095, -0.13), (0.16, 0.082, -0.108), (0.23, 0.068, -0.082), (0.27, 0.058, -0.064)], 2.9, 0.72, region=NK, hatch="fleece")
    sh.shell("head", "head", [(-0.035, 0.045, -0.062), (0.02, 0.066, -0.074), (0.07, 0.066, -0.07), (0.12, 0.058, -0.06), (0.17, 0.045, -0.047), (0.205, 0.034, -0.034), (0.226, 0.022, -0.022), (0.234, 0.008, -0.012)], 4.0, 0.72, region=FA)
    sh.shell("jaw", "jaw", [(-0.02, 0.018, -0.03), (0.04, 0.016, -0.04), (0.09, 0.012, -0.035), (0.125, 0.008, -0.02)], 3.9, 0.72, region=FA)
    sh.shell("ear-n", "ear-n", [(-0.012, 0.018, -0.018), (0.03, 0.022, -0.022), (0.07, 0.014, -0.014), (ear_len, 0.0, 0.0)], 4.5, 0.6, region=EA)
    sh.shell("ear-f", "ear-f", [(-0.012, 0.018, -0.018), (0.03, 0.022, -0.022), (0.07, 0.014, -0.014), (ear_len, 0.0, 0.0)], 0.5, 0.5, region=EA)
    sh.shell("tail-root", "tail-1", [(-0.015, 0.032, -0.032), (0.05, 0.03, -0.03), (0.1, 0.026, -0.026)], 2.8, 0.66, region=TA, hatch="fleece")
    sh.shell("tail-tip", "tail-2", [(-0.01, 0.026, -0.026), (0.05, 0.023, -0.023), (0.09, 0.012, -0.012)], 2.8, 0.66, region=TA)


def shapes50(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides():
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.115, 0.385), (0.095, 0.33), (0.062, 0.26), (0.04, 0.208), (0.018, 0.178), (-0.022, 0.185), (-0.03, 0.25), (-0.02, 0.31), (0.01, 0.365), (0.05, 0.395)], z + 0.2, tone, world=True, region=UL, hatch="fleece")
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.03, -0.036), (0.03, 0.021, -0.022), (0.1, 0.017, -0.017), (0.15, 0.022, -0.02), (0.165, 0.016, -0.014)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.04, 0.052, -0.056), (0.04, 0.046, -0.048), (0.12, 0.036, -0.036), (0.19, 0.029, -0.029), (0.208, 0.026, -0.026)], z + 0.05, tone, region=UL, hatch="fleece")
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.026, -0.026), (0.04, 0.018, -0.018), (0.11, 0.017, -0.017), (0.148, 0.023, -0.021)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone)
    sh.stroke("fold-stifle", "thigh-hn", [(0.07, 0.07), (0.15, 0.0), (0.2, -0.05)], 5.5, 0.9, role="detail", pen=0.18, smooth=True, region=TR)
    sh.stroke("fold-elbow", "humerus-fn", [(0.0, 0.07), (0.1, 0.0), (0.17, -0.07)], 5.5, 0.9, role="detail", pen=0.18, smooth=True, region=TR)
    trunk50(sh)
    head50(sh)
    sh.ellipse("eye", "head", (0.07, 0.03), 0.0105, 0.0095, 4.6, 1.0, region=FA)
    sh.stroke("mouth-line", "head", [(0.215, -0.012), (0.17, -0.034), (0.12, -0.04)], 4.6, 0.9, role="detail", pen=0.18, region=FA)
    return sh


def shapes100(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides():
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.115, 0.385), (0.085, 0.31), (0.05, 0.235), (0.02, 0.185), (-0.024, 0.19), (-0.03, 0.26), (-0.01, 0.33), (0.03, 0.385)], z + 0.2, tone, world=True, region=UL, hatch="fleece")
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.03, -0.036), (0.03, 0.022, -0.022), (0.1, 0.018, -0.018), (0.152, 0.022, -0.02), (0.165, 0.016, -0.014)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HOOF_BIG_S)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.04, 0.054, -0.058), (0.05, 0.047, -0.049), (0.14, 0.038, -0.038), (0.208, 0.03, -0.03)], z + 0.05, tone, region=UL, hatch="fleece")
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.026, -0.026), (0.05, 0.019, -0.019), (0.11, 0.018, -0.018), (0.148, 0.023, -0.021)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HOOF_BIG_S)
    sh.blob("hindbody", "lumbar", [(0.62, 0.762), (0.50, 0.757), (0.38, 0.752), (0.20, 0.756), (0.02, 0.742), (-0.12, 0.700), (-0.165, 0.60), (-0.12, 0.49), (-0.02, 0.40), (0.1, 0.35), (0.28, 0.325), (0.38, 0.322), (0.50, 0.318), (0.62, 0.33), (0.62, 0.55)], 3.0, 0.72, world=True, region=TR, hatch="fleece")
    sh.blob("chest", "thorax", [(0.44, 0.764), (0.50, 0.766), (0.62, 0.762), (0.70, 0.725), (0.74, 0.62), (0.74, 0.50), (0.70, 0.395), (0.62, 0.335), (0.44, 0.316), (0.44, 0.54)], 3.2, 0.72, world=True, region=TR, hatch="fleece")
    sh.shell("neck", "neck",  [(-0.08, 0.11, -0.10), (0.05, 0.1, -0.14), (0.16, 0.082, -0.108), (0.27, 0.06, -0.066)], 2.9, 0.72, region=NK)
    sh.shell("head", "head", [(-0.035, 0.045, -0.062), (0.04, 0.066, -0.076), (0.12, 0.058, -0.064), (0.19, 0.04, -0.044), (0.226, 0.022, -0.024), (0.234, 0.008, -0.014)], 4.0, 0.72, region=FA)
    sh.shell("ear-n", "ear-n", [(-0.012, 0.02, -0.02), (0.04, 0.024, -0.024), (0.095, 0.0, 0.0)], 4.5, 0.6, region=EA)
    sh.shell("ear-f", "ear-f", [(-0.012, 0.02, -0.02), (0.04, 0.024, -0.024), (0.095, 0.0, 0.0)], 0.5, 0.5, region=EA)
    sh.shell("tail", "tail-1", [(-0.015, 0.034, -0.034), (0.08, 0.03, -0.03), (0.19, 0.012, -0.012)], 2.8, 0.66, region=TA)
    return sh


def shapes200(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides():
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.115, 0.385), (0.075, 0.28), (0.03, 0.2), (-0.026, 0.195), (-0.03, 0.27), (0.0, 0.34), (0.04, 0.39)], z + 0.2, tone, world=True, region=UL)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.03, -0.034), (0.1, 0.024, -0.024), (0.16, 0.024, -0.022)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HOOF_BLOCK)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.045, 0.058, -0.06), (0.1, 0.044, -0.044), (0.208, 0.032, -0.032)], z + 0.05, tone, region=UL)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.026, -0.026), (0.1, 0.024, -0.024), (0.15, 0.024, -0.022)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HOOF_BLOCK)
    sh.blob("hindbody", "lumbar", [(0.62, 0.762), (0.50, 0.757), (0.38, 0.752), (0.1, 0.752), (-0.12, 0.70), (-0.165, 0.60), (-0.1, 0.47), (0.05, 0.37), (0.2, 0.33), (0.38, 0.322), (0.50, 0.318), (0.62, 0.33), (0.62, 0.55)], 3.0, 0.72, world=True, region=TR)
    sh.blob("chest", "thorax", [(0.44, 0.764), (0.55, 0.768), (0.68, 0.74), (0.74, 0.6), (0.71, 0.42), (0.62, 0.335), (0.44, 0.316), (0.44, 0.54)], 3.2, 0.72, world=True, region=TR)
    sh.shell("neck", "neck",  [(-0.08, 0.11, -0.10), (0.1, 0.092, -0.12), (0.27, 0.06, -0.068)], 2.9, 0.72, region=NK)
    sh.shell("head", "head", [(-0.035, 0.05, -0.065), (0.06, 0.07, -0.076), (0.18, 0.045, -0.05), (0.234, 0.01, -0.016)], 4.0, 0.72, region=FA)
    sh.shell("ear", "ear-n", [(-0.012, 0.024, -0.024), (0.045, 0.026, -0.026), (0.095, 0.0, 0.0)], 4.5, 0.6, region=EA)
    sh.shell("tail", "tail-1", [(-0.015, 0.036, -0.036), (0.19, 0.016, -0.016)], 2.8, 0.66, region=TA)
    return sh


def shapes500(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides()[1:]:
        h, f = "h" + sd, "f" + sd
        pen = 0.2
        e = sk.end("hoof-" + h)
        sh.stroke("leg-hind-upper-" + sd, "shank-" + h, [(0.0, 0.0), (sk.by["shank-" + h]["lengthM"], 0.0)], z, tone, pen=pen, regions=[UL])
        sh.stroke("leg-hind-lower-" + sd, "metatarsus-" + h, [(-0.17, 0.0), (0.0, 0.0), (sk.by["metatarsus-" + h]["lengthM"], 0.0), sk.local("metatarsus-" + h, e)], z, tone, pen=pen, regions=[LL, HO])
        e = sk.end("hoof-" + f)
        sh.stroke("leg-fore-upper-" + sd, "forearm-" + f, [(0.0, 0.0), (sk.by["forearm-" + f]["lengthM"], 0.0)], z, tone, pen=pen, regions=[UL])
        sh.stroke("leg-fore-lower-" + sd, "metacarpus-" + f, [(-0.17, 0.0), (0.0, 0.0), (sk.by["metacarpus-" + f]["lengthM"], 0.0), sk.local("metacarpus-" + f, e)], z, tone, pen=pen, regions=[LL, HO])
    sh.blob("hindbody", "lumbar", [(0.62, 0.762), (0.50, 0.757), (0.38, 0.752), (0.1, 0.752), (-0.12, 0.70), (-0.165, 0.60), (-0.1, 0.47), (0.05, 0.37), (0.2, 0.33), (0.38, 0.322), (0.50, 0.318), (0.62, 0.33), (0.62, 0.55)], 3.0, 0.72, world=True, regions=[TR, TA, UL])
    sh.blob("chest", "thorax", [(0.44, 0.764), (0.55, 0.768), (0.68, 0.74), (0.74, 0.6), (0.71, 0.42), (0.62, 0.335), (0.44, 0.316), (0.44, 0.54)], 3.2, 0.72, world=True, regions=[TR, UL])
    sh.shell("neck", "neck",  [(-0.08, 0.11, -0.10), (0.1, 0.092, -0.12), (0.27, 0.06, -0.068)], 2.9, 0.72, regions=[NK])
    sh.shell("head", "head", [(-0.035, 0.05, -0.065), (0.06, 0.07, -0.076), (0.18, 0.045, -0.05), (0.234, 0.01, -0.016)], 4.0, 0.72, regions=[FA, EA])
    return sh
