# [scratch] NW-A1 goat representations
from lib import *
from sheep_shapes import whoof, sides, HOOF_REL, HOOF_REL_S
SIDES_G = sides()

TR, NK, UL, LL, FA, EA, HN, TA, HO = "coat-trunk", "coat-neck", "coat-upper-legs", "lower-legs", "face", "ears", "horns", "tail", "hooves"
HR = [(a * 1.2, b * 1.2) for a, b in HOOF_REL]
HRS = [(a * 1.35, b * 1.35) for a, b in HOOF_REL_S]
HBLOCK = [(-0.05, 0.075), (0.045, 0.055), (0.052, 0.0), (-0.05, 0.0)]
HIND = [(0.72, 0.785), (0.58, 0.782), (0.45, 0.778), (0.38, 0.775), (0.25, 0.78), (0.10, 0.785), (0.0, 0.78), (-0.08, 0.76), (-0.13, 0.72), (-0.145, 0.66), (-0.13, 0.60), (-0.09, 0.56), (-0.02, 0.50), (0.06, 0.455), (0.16, 0.43), (0.28, 0.42), (0.38, 0.42), (0.50, 0.405), (0.60, 0.40), (0.72, 0.42), (0.72, 0.6)]
CHEST = [(0.44, 0.784), (0.58, 0.795), (0.68, 0.78), (0.74, 0.74), (0.78, 0.68), (0.795, 0.60), (0.78, 0.52), (0.74, 0.45), (0.68, 0.41), (0.60, 0.395), (0.50, 0.39), (0.44, 0.393), (0.44, 0.6)]


def common_head(sh, detail=True, jaw=True, horn=True, ear2=True):
    sh.shell("neck", "neck", [(-0.07, 0.095, -0.10), (0.0, 0.088, -0.105), (0.1, 0.076, -0.088), (0.2, 0.066, -0.072), (0.28, 0.056, -0.062), (0.31, 0.05, -0.055)], 2.9, 0.72, region=NK)
    sh.shell("head", "head", [(-0.035, 0.045, -0.052), (0.02, 0.058, -0.066), (0.07, 0.06, -0.064), (0.12, 0.052, -0.054), (0.17, 0.04, -0.042), (0.205, 0.03, -0.031), (0.225, 0.02, -0.022), (0.233, 0.008, -0.012)], 4.0, 0.72, region=FA)
    if jaw:
        sh.shell("jaw", "jaw", [(-0.02, 0.016, -0.028), (0.04, 0.014, -0.036), (0.09, 0.01, -0.032), (0.118, 0.006, -0.018)], 3.9, 0.72, region=FA)
        sh.blob("beard", "jaw", [(0.06, -0.012), (0.112, -0.018), (0.108, -0.055), (0.092, -0.088), (0.078, -0.052)], 3.95, 0.72, region=FA)
    sh.shell("ear-n", "ear-n", [(-0.012, 0.022, -0.022), (0.04, 0.03, -0.03), (0.085, 0.016, -0.016), (0.108, 0.0, 0.0)], 4.5, 0.6, region=EA)
    if ear2:
        sh.shell("ear-f", "ear-f", [(-0.012, 0.022, -0.022), (0.04, 0.03, -0.03), (0.085, 0.016, -0.016), (0.108, 0.0, 0.0)], 0.5, 0.5, region=EA)
    if horn:
        sh.blob("horn-n", "head", [(-0.005, 0.05), (0.035, 0.054), (0.012, 0.11), (-0.06, 0.135), (-0.14, 0.115), (-0.178, 0.078), (-0.15, 0.062), (-0.1, 0.085), (-0.05, 0.075)], 4.4, 0.9, region=HN)
        sh.blob("horn-f", "head", [(-0.015, 0.045), (0.03, 0.05), (0.0, 0.105), (-0.07, 0.125), (-0.15, 0.105), (-0.188, 0.065), (-0.158, 0.05), (-0.11, 0.075), (-0.06, 0.062)], 0.4, 0.8, region=HN)
    sh.shell("tail-root", "tail-1", [(-0.012, 0.026, -0.026), (0.04, 0.024, -0.024), (0.07, 0.02, -0.02)], 2.8, 0.66, region=TA)
    sh.shell("tail-tip", "tail-2", [(-0.008, 0.02, -0.02), (0.03, 0.019, -0.019), (0.06, 0.008, -0.008)], 2.8, 0.66, region=TA)


def shapes50(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides():
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.125, 0.44), (0.10, 0.37), (0.065, 0.30), (0.035, 0.245), (0.012, 0.205), (-0.024, 0.212), (-0.028, 0.27), (-0.012, 0.34), (0.012, 0.40), (0.05, 0.455)], z + 0.2, tone, world=True, region=UL)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.028, -0.032), (0.04, 0.018, -0.019), (0.11, 0.015, -0.016), (0.16, 0.022, -0.02), (0.178, 0.015, -0.013)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HR)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.04, 0.046, -0.05), (0.05, 0.037, -0.04), (0.14, 0.028, -0.028), (0.21, 0.025, -0.025), (0.228, 0.022, -0.022)], z + 0.05, tone, region=UL)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.023, -0.023), (0.05, 0.016, -0.016), (0.12, 0.016, -0.016), (0.172, 0.022, -0.02)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HR)
    sh.blob("thigh-n", "thigh-hn", [(-0.12, 0.64), (-0.11, 0.57), (-0.07, 0.51), (0.0, 0.455), (0.07, 0.42), (0.115, 0.43), (0.13, 0.49), (0.11, 0.57), (0.06, 0.64), (-0.03, 0.68)], 5.15, 0.72, world=True, region=UL)
    sh.blob("shoulder-n", "scapula-fn", [(0.56, 0.78), (0.66, 0.79), (0.74, 0.75), (0.76, 0.68), (0.75, 0.60), (0.71, 0.54), (0.65, 0.55), (0.58, 0.62), (0.54, 0.70)], 5.15, 0.72, world=True, region=UL)
    sh.blob("hindbody", "lumbar", HIND, 3.0, 0.72, world=True, region=TR)
    sh.blob("chest", "thorax", CHEST, 3.2, 0.72, world=True, region=TR)
    common_head(sh)
    sh.ellipse("eye", "head", (0.075, 0.03), 0.0105, 0.0095, 4.6, 1.0, region=FA)
    sh.stroke("mouth-line", "head", [(0.216, -0.01), (0.17, -0.03), (0.12, -0.037)], 4.6, 0.9, role="detail", pen=0.18, region=FA)
    sh.stroke("hock-line", "shank-hn", [(0.19, -0.012), (0.2, 0.0)], 5.4, 0.9, role="detail", pen=0.18, region=UL)
    return sh


def shapes100(sk):
    sh = Shapes(sk)
    for sd, z, tone in sides():
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.125, 0.44), (0.09, 0.34), (0.05, 0.26), (0.012, 0.205), (-0.026, 0.215), (-0.03, 0.29), (0.0, 0.38), (0.05, 0.455)], z + 0.2, tone, world=True, region=UL)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.03, -0.034), (0.05, 0.02, -0.021), (0.12, 0.017, -0.018), (0.16, 0.022, -0.02), (0.178, 0.016, -0.014)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HRS)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.04, 0.05, -0.054), (0.06, 0.04, -0.043), (0.15, 0.03, -0.03), (0.228, 0.025, -0.025)], z + 0.05, tone, region=UL)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.025, -0.025), (0.06, 0.019, -0.019), (0.12, 0.018, -0.018), (0.172, 0.023, -0.021)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HRS)
    sh.blob("thigh-n", "thigh-hn", [(-0.12, 0.64), (-0.09, 0.53), (0.0, 0.455), (0.07, 0.42), (0.13, 0.49), (0.06, 0.64), (-0.03, 0.68)], 5.15, 0.72, world=True, region=UL)
    sh.blob("shoulder-n", "scapula-fn", [(0.56, 0.78), (0.70, 0.78), (0.76, 0.68), (0.72, 0.55), (0.62, 0.58), (0.54, 0.70)], 5.15, 0.72, world=True, region=UL)
    hb, cb = hc(2)
    sh.blob("hindbody", "lumbar", hb, 3.0, 0.72, world=True, region=TR)
    sh.blob("chest", "thorax", cb, 3.2, 0.72, world=True, region=TR)
    common_head(sh, jaw=False)
    return sh


def hc(k):
    return HIND[::k] + ([HIND[-1]] if (len(HIND) - 1) % k else []), CHEST[::k] + ([CHEST[-1]] if (len(CHEST) - 1) % k else [])


def shapes200(sk):
    sh = Shapes(sk)
    for sd, z, tone in SIDES_G:
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.125, 0.44), (0.07, 0.30), (0.01, 0.205), (-0.03, 0.22), (-0.03, 0.3), (0.0, 0.39), (0.05, 0.455)], z + 0.2, tone, world=True, region=UL)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.03, -0.034), (0.1, 0.024, -0.024), (0.178, 0.024, -0.022)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HBLOCK)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.045, 0.055, -0.058), (0.1, 0.04, -0.04), (0.228, 0.03, -0.03)], z + 0.05, tone, region=UL)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.026, -0.026), (0.1, 0.024, -0.024), (0.172, 0.024, -0.022)], z + 0.04, tone, region=LL)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HBLOCK)
    sh.blob("thigh-n", "thigh-hn", [(-0.12, 0.64), (-0.08, 0.52), (0.07, 0.42), (0.13, 0.5), (0.04, 0.66)], 5.15, 0.72, world=True, region=UL)
    hb, cb = hc(3)
    sh.blob("hindbody", "lumbar", hb, 3.0, 0.72, world=True, regions=[TR, TA])
    sh.blob("chest", "thorax", cb, 3.2, 0.72, world=True, region=TR)
    sh.shell("neck", "neck", [(-0.07, 0.095, -0.10), (0.1, 0.078, -0.09), (0.31, 0.05, -0.055)], 2.9, 0.72, region=NK)
    sh.shell("head", "head", [(-0.035, 0.05, -0.055), (0.06, 0.062, -0.066), (0.18, 0.042, -0.044), (0.233, 0.01, -0.014)], 4.0, 0.72, region=FA)
    sh.blob("horn", "head", [(-0.005, 0.05), (0.04, 0.056), (0.02, 0.115), (-0.06, 0.14), (-0.15, 0.12), (-0.19, 0.088), (-0.17, 0.056), (-0.11, 0.088), (-0.05, 0.082)], 4.4, 0.9, region=HN)
    sh.shell("ear", "ear-n", [(-0.012, 0.026, -0.026), (0.045, 0.032, -0.032), (0.108, 0.0, 0.0)], 4.5, 0.6, region=EA)
    return sh


def shapes500(sk):
    sh = Shapes(sk)
    for sd, z, tone in SIDES_G[1:]:
        h, f = "h" + sd, "f" + sd
        e = sk.end("hoof-" + h)
        sh.stroke("leg-hind-upper-" + sd, "shank-" + h, [(0.0, 0.0), (sk.by["shank-" + h]["lengthM"], 0.0)], z, tone, pen=0.2, regions=[UL])
        sh.stroke("leg-hind-lower-" + sd, "metatarsus-" + h, [(-0.24, 0.0), (0.0, 0.0), (sk.by["metatarsus-" + h]["lengthM"], 0.0), sk.local("metatarsus-" + h, e)], z, tone, pen=0.2, regions=[LL, HO])
        e = sk.end("hoof-" + f)
        sh.stroke("leg-fore-upper-" + sd, "forearm-" + f, [(0.0, 0.0), (sk.by["forearm-" + f]["lengthM"], 0.0)], z, tone, pen=0.2, regions=[UL])
        sh.stroke("leg-fore-lower-" + sd, "metacarpus-" + f, [(-0.24, 0.0), (0.0, 0.0), (sk.by["metacarpus-" + f]["lengthM"], 0.0), sk.local("metacarpus-" + f, e)], z, tone, pen=0.2, regions=[LL, HO])
    hb, cb = hc(3)
    sh.blob("hindbody", "lumbar", hb, 3.0, 0.72, world=True, regions=[TR, UL, TA])
    sh.blob("chest", "thorax", cb, 3.2, 0.72, world=True, regions=[TR, UL])
    sh.shell("neck", "neck", [(-0.07, 0.095, -0.10), (0.1, 0.078, -0.09), (0.31, 0.05, -0.055)], 2.9, 0.72, regions=[NK])
    sh.shell("head", "head", [(-0.035, 0.05, -0.055), (0.06, 0.066, -0.07), (0.18, 0.05, -0.052), (0.226, 0.03, -0.034)], 4.0, 0.72, regions=[FA, EA])
    sh.stroke("horn", "head", [(0.0, 0.05), (-0.06, 0.112), (-0.17, 0.075)], 4.4, 0.9, pen=0.13, smooth=True, regions=[HN])
    return sh
