# [scratch] NW-A1 pig representations
from lib import *
from sheep_shapes import whoof, sides, HOOF_REL, HOOF_REL_S, HOOF_BIG, HOOF_BIG_S, HOOF_BLOCK

TR, LG, HD, SN, EA, TA, HO = "skin-trunk", "skin-legs", "skin-head", "snout", "ears", "tail", "hooves"
HIND = [(0.64, 0.70), (0.52, 0.718), (0.38, 0.725), (0.28, 0.728), (0.16, 0.72), (0.05, 0.70), (-0.05, 0.67), (-0.12, 0.62), (-0.15, 0.55), (-0.14, 0.47), (-0.10, 0.40), (-0.03, 0.34), (0.06, 0.29), (0.16, 0.255), (0.28, 0.235), (0.38, 0.23), (0.50, 0.225), (0.64, 0.23), (0.64, 0.5)]
CHEST = [(0.44, 0.722), (0.56, 0.70), (0.65, 0.675), (0.72, 0.62), (0.76, 0.55), (0.78, 0.46), (0.75, 0.37), (0.69, 0.30), (0.60, 0.25), (0.50, 0.225), (0.44, 0.225), (0.44, 0.5)]
TONES = (("f", 1.0, 0.38), ("n", 5.0, 0.52))


def ground_ok(x):
    return x


def body(sh, pts_h, pts_c, hatch=None, regions=None):
    kw = dict(region=TR) if regions is None else dict(regions=regions)
    h = dict(hatch=hatch) if hatch else {}
    sh.blob("hindbody", "lumbar", pts_h, 3.0, 0.52, world=True, **kw, **h)
    sh.blob("chest", "thorax", pts_c, 3.2, 0.52, world=True, region=TR, **h)


def head(sh, jaw=True, snout=True, ear2=True):
    sh.shell("neck", "neck", [(-0.1, 0.1, -0.12), (0.0, 0.1, -0.12), (0.1, 0.095, -0.125), (0.17, 0.09, -0.105)], 2.9, 0.52, region=TR)
    sh.shell("head", "head", [(-0.05, 0.09, -0.10), (0.03, 0.105, -0.12), (0.10, 0.095, -0.11), (0.17, 0.075, -0.085), (0.23, 0.055, -0.058), (0.28, 0.045, -0.045), (0.31, 0.038, -0.038)], 4.0, 0.52, region=HD)
    if snout:
        sh.ellipse("snout", "head", (0.312, 0.0), 0.02, 0.045, 4.2, 0.8, role="body", region=SN)
    if jaw:
        sh.shell("jaw", "jaw", [(-0.02, 0.025, -0.035), (0.06, 0.022, -0.045), (0.14, 0.018, -0.035), (0.2, 0.012, -0.02)], 3.9, 0.52, region=HD)
    sh.shell("ear-n", "ear-n", [(-0.012, 0.05, -0.05), (0.05, 0.07, -0.07), (0.12, 0.06, -0.06), (0.17, 0.0, 0.0)], 4.5, 0.45, region=EA)
    if ear2:
        sh.shell("ear-f", "ear-f", [(-0.012, 0.05, -0.05), (0.05, 0.07, -0.07), (0.12, 0.06, -0.06), (0.17, 0.0, 0.0)], 0.5, 0.35, region=EA)


def legs(sh, detail=1, hoofrel=None):
    hoofrel = HOOF_BIG if hoofrel is None else hoofrel
    for sd, z, tone in TONES:
        h, f = "h" + sd, "f" + sd
        if detail >= 1:
            sh.blob("gaskin-" + sd, "shank-" + h, [(0.10, 0.37), (0.085, 0.30), (0.055, 0.23), (0.03, 0.18), (0.005, 0.16), (-0.02, 0.19), (-0.02, 0.26), (0.0, 0.33), (0.03, 0.375)], z + 0.2, tone, world=True, region=LG)
            sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.025, 0.028, -0.03), (0.04, 0.021, -0.021), (0.09, 0.021, -0.021), (0.12, 0.026, -0.024), (0.13, 0.019, -0.017)], z + 0.04, tone, region=LG)
            sh.shell("forearm-" + sd, "forearm-" + f, [(-0.03, 0.052, -0.058), (0.04, 0.046, -0.05), (0.1, 0.037, -0.038), (0.15, 0.034, -0.034)], z + 0.05, tone, region=LG)
            sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.03, -0.03), (0.05, 0.024, -0.024), (0.1, 0.024, -0.024), (0.112, 0.028, -0.026)], z + 0.04, tone, region=LG)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=hoofrel)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=hoofrel)


def shapes50(sk):
    sh = Shapes(sk)
    legs(sh)
    sh.blob("ham-n", "thigh-hn", [(-0.13, 0.60), (-0.14, 0.50), (-0.10, 0.41), (-0.02, 0.35), (0.06, 0.33), (0.115, 0.36), (0.13, 0.46), (0.10, 0.58), (0.0, 0.66)], 5.15, 0.52, world=True, region=TR, hatch="bristles")
    sh.blob("shoulder-n", "scapula-fn", [(0.52, 0.70), (0.64, 0.69), (0.72, 0.62), (0.74, 0.52), (0.70, 0.42), (0.62, 0.40), (0.55, 0.48), (0.50, 0.60)], 5.15, 0.52, world=True, region=TR, hatch="bristles")
    body(sh, HIND, CHEST, hatch="bristles")
    head(sh)
    sh.ellipse("eye", "head", (0.1, 0.03), 0.0105, 0.0095, 4.6, 1.0, region=HD)
    sh.stroke("mouth-line", "head", [(0.29, -0.02), (0.2, -0.045), (0.12, -0.062)], 4.6, 0.9, role="detail", pen=0.18, region=HD)
    sh.shell("tail-root", "tail-1", [(-0.012, 0.016, -0.016), (0.04, 0.014, -0.014), (0.085, 0.012, -0.012)], 2.8, 0.45, region=TA)
    sh.shell("tail-curl", "tail-2", [(-0.008, 0.013, -0.013), (0.035, 0.012, -0.012), (0.075, 0.008, -0.008)], 2.8, 0.45, region=TA)
    return sh


def shapes100(sk):
    sh = Shapes(sk)
    legs(sh, hoofrel=HOOF_BIG_S)
    sh.blob("ham-n", "thigh-hn", [(-0.13, 0.60), (-0.12, 0.45), (-0.02, 0.35), (0.10, 0.37), (0.12, 0.52), (0.0, 0.66)], 5.15, 0.52, world=True, region=TR)
    body(sh, HIND[::2] + [HIND[-1]], CHEST[::2] + [CHEST[-1]])
    head(sh, jaw=False)
    sh.shell("tail", "tail-1", [(-0.012, 0.022, -0.022), (0.08, 0.018, -0.018), (0.15, 0.014, -0.014)], 2.8, 0.45, region=TA)
    return sh


def shapes200(sk):
    sh = Shapes(sk)
    for sd, z, tone in TONES:
        h, f = "h" + sd, "f" + sd
        sh.blob("gaskin-" + sd, "shank-" + h, [(0.10, 0.37), (0.06, 0.25), (0.01, 0.17), (-0.025, 0.19), (-0.025, 0.28), (0.03, 0.375)], z + 0.2, tone, world=True, region=LG)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.03, 0.034, -0.034), (0.1, 0.026, -0.026), (0.13, 0.026, -0.024)], z + 0.04, tone, region=LG)
        whoof(sh, "hoof-h-" + sd, "hoof-" + h, z + 0.06, tone, rel=HOOF_BLOCK)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.04, 0.058, -0.062), (0.08, 0.044, -0.044), (0.15, 0.036, -0.036)], z + 0.05, tone, region=LG)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.02, 0.032, -0.032), (0.1, 0.028, -0.028), (0.112, 0.028, -0.026)], z + 0.04, tone, region=LG)
        whoof(sh, "hoof-f-" + sd, "hoof-" + f, z + 0.06, tone, rel=HOOF_BLOCK)
    sh.blob("hindbody", "lumbar", HIND[::2] + [HIND[-1]], 3.0, 0.52, world=True, regions=[TR, TA])
    sh.blob("chest", "thorax", CHEST[::2] + [CHEST[-1]], 3.2, 0.52, world=True, region=TR)
    sh.shell("neck", "neck", [(-0.1, 0.1, -0.12), (0.1, 0.095, -0.115), (0.17, 0.09, -0.105)], 2.9, 0.52, region=TR)
    sh.shell("head", "head", [(-0.05, 0.09, -0.10), (0.05, 0.105, -0.12), (0.17, 0.075, -0.085), (0.31, 0.04, -0.04)], 4.0, 0.52, regions=[HD, SN])
    sh.shell("ear", "ear-n", [(-0.012, 0.055, -0.055), (0.07, 0.07, -0.07), (0.17, 0.0, 0.0)], 4.5, 0.45, region=EA)
    return sh


def shapes500(sk):
    sh = Shapes(sk)
    for sd, z, tone in TONES[1:]:
        h, f = "h" + sd, "f" + sd
        e = sk.end("hoof-" + h)
        sh.stroke("leg-hind-upper-" + sd, "shank-" + h, [(0.0, 0.0), (sk.by["shank-" + h]["lengthM"], 0.0)], z, tone, pen=0.2, regions=[LG])
        sh.stroke("leg-hind-lower-" + sd, "metatarsus-" + h, [(-0.2, 0.0), (0.0, 0.0), (sk.by["metatarsus-" + h]["lengthM"], 0.0), sk.local("metatarsus-" + h, e)], z, tone, pen=0.2, regions=[LG, HO])
        e = sk.end("hoof-" + f)
        sh.stroke("leg-fore-upper-" + sd, "forearm-" + f, [(0.0, 0.0), (sk.by["forearm-" + f]["lengthM"], 0.0)], z, tone, pen=0.2, regions=[LG])
        sh.stroke("leg-fore-lower-" + sd, "metacarpus-" + f, [(-0.17, 0.0), (0.0, 0.0), (sk.by["metacarpus-" + f]["lengthM"], 0.0), sk.local("metacarpus-" + f, e)], z, tone, pen=0.2, regions=[LG, HO])
    sh.blob("hindbody", "lumbar", HIND[::2] + [HIND[-1]], 3.0, 0.52, world=True, regions=[TR, TA])
    sh.blob("chest", "thorax", CHEST[::2] + [CHEST[-1]], 3.2, 0.52, world=True, region=TR)
    sh.shell("neck", "neck", [(-0.1, 0.1, -0.12), (0.1, 0.095, -0.115), (0.17, 0.09, -0.105)], 2.9, 0.52, region=TR)
    sh.shell("head", "head", [(-0.05, 0.09, -0.10), (0.05, 0.105, -0.12), (0.17, 0.075, -0.085), (0.31, 0.04, -0.04)], 4.0, 0.52, regions=[HD, SN, EA])
    return sh
