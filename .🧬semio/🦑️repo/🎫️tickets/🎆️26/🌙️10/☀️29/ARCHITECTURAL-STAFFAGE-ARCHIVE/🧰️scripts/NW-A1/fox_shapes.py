# [scratch] NW-A1 red fox representations (no 1:500)
from lib import *

TR, LG, FA, EA, BR, PW = "coat-trunk", "coat-legs", "face", "ears", "brush", "paws"
HIND = [(0.46, 0.468), (0.36, 0.466), (0.25, 0.46), (0.15, 0.465), (0.05, 0.462), (-0.03, 0.45), (-0.08, 0.425), (-0.095, 0.39), (-0.085, 0.35), (-0.05, 0.315), (0.02, 0.29), (0.10, 0.275), (0.18, 0.27), (0.25, 0.275), (0.36, 0.26), (0.46, 0.255), (0.46, 0.4)]
CHEST = [(0.30, 0.468), (0.40, 0.47), (0.47, 0.455), (0.52, 0.42), (0.54, 0.37), (0.54, 0.31), (0.50, 0.26), (0.44, 0.24), (0.36, 0.245), (0.30, 0.252), (0.30, 0.42)]
PAW_REL = [(a * 1.3, b * 1.3) for a, b in [(-0.034, 0.0), (0.034, 0.0), (0.038, 0.012), (0.022, 0.024), (-0.004, 0.036), (-0.034, 0.05), (-0.042, 0.03)]]
PAW_REL_S = [(a * 1.5, b * 1.5) for a, b in [(-0.034, 0.0), (0.036, 0.0), (0.018, 0.03), (-0.036, 0.05), (-0.042, 0.025)]]
SIDES = (("f", 1.0, 0.55), ("n", 5.0, 0.7))


def paw(sh, id, bone, z, tone, rel):
    e = sh.sk.end(bone)
    sh.poly(id, bone, [sh.sk.local(bone, (e[0] + a, e[1] + b)) for a, b in rel], z, tone, region=PW)


def front(sh, k):
    sh.blob("hindbody", "lumbar", HIND[::k] + ([HIND[-1]] if (len(HIND) - 1) % k else []) if k > 1 else HIND, 3.0, 0.7, world=True, region=TR, **({"hatch": "fur"} if k == 1 else {}))
    sh.blob("chest", "thorax", CHEST[::k] + ([CHEST[-1]] if (len(CHEST) - 1) % k else []) if k > 1 else CHEST, 3.2, 0.7, world=True, region=TR, **({"hatch": "fur"} if k == 1 else {}))


def tail(sh, detail):
    d = dict(hatch="fur") if detail else {}
    sh.shell("brush-root", "tail-1", [(-0.02, 0.03, -0.03), (0.05, 0.05, -0.05), (0.12, 0.06, -0.06), (0.17, 0.058, -0.058)], 2.8, 0.62, region=BR, **d)
    sh.shell("brush-mid", "tail-2", [(-0.02, 0.058, -0.058), (0.06, 0.066, -0.066), (0.12, 0.058, -0.058), (0.15, 0.045, -0.045)], 2.7, 0.62, region=BR, **d)
    sh.shell("brush-tip", "tail-3", [(-0.01, 0.045, -0.045), (0.05, 0.04, -0.04), (0.1, 0.02, -0.02), (0.125, 0.0, 0.0)], 2.6, 0.3, region=BR)


def head(sh, jaw=True, ear2=True, detail=True):
    sh.shell("neck", "neck", [(-0.07, 0.06, -0.07), (0.0, 0.058, -0.065), (0.07, 0.052, -0.055), (0.14, 0.045, -0.045), (0.17, 0.04, -0.04)], 2.9, 0.7, region=TR)
    sh.shell("head", "head", [(-0.03, 0.042, -0.046), (0.02, 0.054, -0.052), (0.06, 0.052, -0.05), (0.1, 0.036, -0.036), (0.14, 0.022, -0.022), (0.165, 0.014, -0.014), (0.174, 0.007, -0.008)], 4.0, 0.7, region=FA)
    if jaw:
        sh.shell("jaw", "jaw", [(0.0, 0.012, -0.022), (0.04, 0.01, -0.024), (0.09, 0.005, -0.01)], 3.9, 0.7, region=FA)
    sh.shell("ear-n", "ear-n", [(-0.012, 0.032, -0.032), (0.04, 0.03, -0.03), (0.092, 0.0, 0.0)], 4.5, 0.85, region=EA)
    if ear2:
        sh.shell("ear-f", "ear-f", [(-0.012, 0.032, -0.032), (0.04, 0.03, -0.03), (0.092, 0.0, 0.0)], 0.5, 0.75, region=EA)


def shapes50(sk):
    sh = Shapes(sk)
    for sd, z, tone in SIDES:
        h, f = "h" + sd, "f" + sd
        sh.shell("gaskin-" + sd, "shank-" + h, [(-0.02, 0.034, -0.036), (0.04, 0.032, -0.036), (0.1, 0.021, -0.023), (0.14, 0.017, -0.017), (0.152, 0.013, -0.015)], z + 0.2, tone + 0.15, region=LG)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.015, 0.018, -0.022), (0.05, 0.013, -0.013), (0.11, 0.015, -0.015)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-h-" + sd, "paw-" + h, z + 0.06, tone + 0.15, PAW_REL)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.025, 0.032, -0.034), (0.04, 0.026, -0.028), (0.1, 0.018, -0.02), (0.126, 0.015, -0.016)], z + 0.05, tone + 0.15, region=LG)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.01, 0.015, -0.017), (0.04, 0.012, -0.013), (0.085, 0.014, -0.014)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-f-" + sd, "paw-" + f, z + 0.06, tone + 0.15, PAW_REL)
    sh.blob("thigh-n", "thigh-hn", [(-0.08, 0.40), (-0.07, 0.34), (-0.03, 0.30), (0.03, 0.275), (0.07, 0.29), (0.08, 0.34), (0.05, 0.40), (-0.02, 0.43)], 5.15, 0.7, world=True, region=TR)
    front(sh, 1)
    head(sh)
    tail(sh, True)
    sh.ellipse("eye", "head", (0.07, 0.026), 0.0105, 0.0095, 4.6, 1.0, region=FA)
    sh.ellipse("nose", "head", (0.168, 0.004), 0.011, 0.01, 4.6, 1.0, region=FA)
    sh.stroke("mouth-line", "head", [(0.16, -0.006), (0.12, -0.02), (0.08, -0.03)], 4.6, 0.9, role="detail", pen=0.18, region=FA)
    return sh


def shapes100(sk):
    sh = Shapes(sk)
    for sd, z, tone in SIDES:
        h, f = "h" + sd, "f" + sd
        sh.shell("gaskin-" + sd, "shank-" + h, [(-0.02, 0.036, -0.038), (0.05, 0.032, -0.036), (0.1, 0.024, -0.026), (0.152, 0.016, -0.016)], z + 0.2, tone + 0.15, region=LG)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.015, 0.021, -0.024), (0.05, 0.016, -0.016), (0.11, 0.017, -0.017)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-h-" + sd, "paw-" + h, z + 0.06, tone + 0.15, PAW_REL_S)
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.025, 0.034, -0.036), (0.05, 0.027, -0.03), (0.126, 0.017, -0.018)], z + 0.05, tone + 0.15, region=LG)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.01, 0.018, -0.02), (0.04, 0.015, -0.015), (0.085, 0.016, -0.016)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-f-" + sd, "paw-" + f, z + 0.06, tone + 0.15, PAW_REL_S)
    sh.blob("thigh-n", "thigh-hn", [(-0.08, 0.40), (-0.05, 0.31), (0.03, 0.275), (0.08, 0.34), (0.0, 0.43)], 5.15, 0.7, world=True, region=TR)
    front(sh, 2)
    head(sh, jaw=False)
    sh.shell("brush", "tail-1", [(-0.02, 0.03, -0.03), (0.08, 0.055, -0.055), (0.17, 0.06, -0.06)], 2.8, 0.62, region=BR)
    sh.shell("brush-2", "tail-2", [(-0.02, 0.06, -0.06), (0.08, 0.066, -0.066), (0.15, 0.045, -0.045)], 2.7, 0.62, region=BR)
    sh.shell("brush-tip", "tail-3", [(-0.01, 0.045, -0.045), (0.06, 0.04, -0.04), (0.125, 0.0, 0.0)], 2.6, 0.3, region=BR)
    return sh


def shapes200(sk):
    sh = Shapes(sk)
    for sd, z, tone in SIDES:
        h, f = "h" + sd, "f" + sd
        sh.shell("gaskin-" + sd, "shank-" + h, [(-0.02, 0.04, -0.042), (0.08, 0.034, -0.036), (0.152, 0.02, -0.02)], z + 0.2, tone + 0.15, region=LG)
        sh.shell("cannon-h-" + sd, "metatarsus-" + h, [(-0.015, 0.024, -0.026), (0.11, 0.021, -0.021)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-h-" + sd, "paw-" + h, z + 0.06, tone + 0.15, [(-0.06, 0.0), (0.06, 0.0), (0.0, 0.06), (-0.065, 0.07)])
        sh.shell("forearm-" + sd, "forearm-" + f, [(-0.025, 0.04, -0.04), (0.126, 0.022, -0.022)], z + 0.05, tone + 0.15, region=LG)
        sh.shell("cannon-f-" + sd, "metacarpus-" + f, [(-0.01, 0.022, -0.024), (0.085, 0.021, -0.021)], z + 0.04, tone + 0.15, region=LG)
        paw(sh, "paw-f-" + sd, "paw-" + f, z + 0.06, tone + 0.15, [(-0.06, 0.0), (0.06, 0.0), (0.0, 0.06), (-0.065, 0.07)])
    front(sh, 3)
    sh.shell("neck", "neck", [(-0.07, 0.06, -0.07), (0.07, 0.052, -0.055), (0.17, 0.04, -0.04)], 2.9, 0.7, region=TR)
    sh.shell("head", "head", [(-0.03, 0.046, -0.05), (0.06, 0.056, -0.052), (0.174, 0.008, -0.01)], 4.0, 0.7, region=FA)
    sh.shell("ear", "ear-n", [(-0.012, 0.034, -0.034), (0.092, 0.0, 0.0)], 4.5, 0.85, region=EA)
    sh.shell("brush", "tail-1", [(-0.02, 0.03, -0.03), (0.17, 0.06, -0.06)], 2.8, 0.62, region=BR)
    sh.shell("brush-2", "tail-2", [(-0.02, 0.06, -0.06), (0.15, 0.045, -0.045)], 2.7, 0.62, region=BR)
    sh.shell("brush-tip", "tail-3", [(-0.01, 0.045, -0.045), (0.125, 0.0, 0.0)], 2.6, 0.3, region=BR)
    return sh
