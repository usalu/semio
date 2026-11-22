"""🧥 Rework 05, parent carrying a toddler: every solid of the four scales stated by hand (body, thermal, structure) on the corrected rig, written into the candidate document and the volumes source.

Usage: python3 rework-05-parent-carrying-toddler-model.py  (after `…-roots.py`; reads and writes `🗑️generated/r3-parent-carrying-toddler/🔣️.json`, writes `volumes/humans/parent-carrying-toddler.json`)
Frames: u along the bone, v its normal (upright bones: back; hanging limbs: ahead; feet: up), w left. Station rows are (u, v, w, half width along w, half depth along v[, exponent]).
@see ./rework-05-kit.py
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "rework-05-kit.py"))
kit = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kit)

SLUG = "parent-carrying-toddler"
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-" + SLUG, "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "humans", SLUG + ".json")
STEP = {50: 0.02, 100: 0.03, 200: 0.045, 500: 0.07}
L, C, E, X, B = kit.loft, kit.capsule, kit.ellipsoid, kit.extrude, kit.both


def tag(solids, region, layer=None):
    """🏷️ Marks body solids with the thermal region (and outer layer) they are the surface of."""
    for s in solids if isinstance(solids, list) else [solids]:
        s["_r"] = (region, layer)
    return solids


def clav(p):
    """📐 A left-side point given in the thorax frame, in the frame of the left clavicle bone (joint at thorax 0.228, -0.10, 0.02; 80 degrees out of the sagittal plane)."""
    du, dv, dw = p[0] - 0.228, p[1] + 0.10, p[2] - 0.02
    return [0.17365 * du + 0.98481 * dw, dv, -0.98481 * du + 0.17365 * dw]


def parent_body(s):
    """🧍 The parent at scale 1:s: head with face under a beanie, scarf, wool coat with yoke, set-in sleeves and a skirt that follows the thighs, gloved hands with finger chains, jeans and ankle boots."""
    st = STEP[s]
    fine, mid = s == 50, s <= 100
    out = []
    head = L("head", "head", "skin", 0.8, [(-0.082, -0.05, 0, 0.026, 0.028), (-0.068, -0.044, 0, 0.042, 0.046), (-0.045, -0.02, 0, 0.058, 0.074), (-0.012, -0.006, 0, 0.068, 0.089), (0.03, 0, 0, 0.074, 0.095), (0.07, 0.004, 0, 0.076, 0.096), (0.105, 0.008, 0, 0.068, 0.086), (0.13, 0.01, 0, 0.048, 0.062)], step=st, round1=0.0115, round0=0.006)
    out += tag([head], "face")
    out += tag([L("neck", "neck", "skin", 0.76, [(-0.02, 0, 0, 0.056, 0.058), (0.04, -0.006, 0, 0.052, 0.054), (0.1, -0.012, 0, 0.052, 0.056), (0.13, -0.014, 0, 0.05, 0.054)], step=st)], "face")
    beanie = L("beanie", "head", "knit-hat", 0.36, [(0.036 if s < 500 else 0.048, 0.002, 0, 0.084, 0.105), (0.07, 0.005, 0, 0.085, 0.106), (0.105, 0.009, 0, 0.078, 0.097), (0.135, 0.011, 0, 0.06, 0.076), (0.152, 0.012, 0, 0.036, 0.046)], step=st, round1=0.012)
    out += tag([beanie], "beanie", "beanie")
    if s <= 200:
        out += tag([L("beanie-cuff", "head", "knit-hat", 0.3, [(0.03, 0.002, 0, 0.088, 0.109), (0.05, 0.003, 0, 0.0915, 0.1125), (0.074, 0.005, 0, 0.09, 0.111)], step=st, round0=0.006, round1=0.006)], "beanie", "beanie")
        out += tag([E("hair", "head", "hair", 0.2, (0.0, 0.052, 0), (0.05, 0.05, 0.072))], "beanie", "beanie")
    if mid:
        out += tag([L("head-nose", "head", "skin", 0.84, [(0.03, -0.092, 0, 0.0085, 0.008), (0.012, -0.102, 0, 0.0095, 0.0095), (-0.006, -0.112, 0, 0.0125, 0.0115), (-0.016, -0.108, 0, 0.0165, 0.0095)], step=0.01, round0=0.004, round1=0.004)], "face")
        out += tag(B(E("head-ear-l", "head", "skin", 0.74, (0.004, 0.012, 0.076), (0.031, 0.017, 0.0075), rotation=kit.quat((1, 0, 0), -12))), "face")
    if fine:
        out += tag([E("head-brow", "head", "skin", 0.78, (0.04, -0.086, 0), (0.011, 0.012, 0.056)), E("head-lips", "head", "skin", 0.7, (-0.041, -0.089, 0), (0.0085, 0.009, 0.025)), E("head-chin", "head", "skin", 0.8, (-0.066, -0.074, 0), (0.016, 0.014, 0.024))], "face")
        out += tag(B(E("head-eye-l", "head", "skin", 0.5, (0.026, -0.084, 0.032), (0.0065, 0.0075, 0.0135), role="detail")), "face")
        out += tag(B(E("head-cheek-l", "head", "skin", 0.8, (-0.006, -0.068, 0.046), (0.022, 0.018, 0.022))), "face")
    q = 0.012 if s == 500 else 0.0
    scarf = L("scarf", "thorax", "knit-hat", 0.42, [(0.275 - 2 * q, -0.03, 0, 0.082 + q, 0.088 + q), (0.3, -0.03, 0, 0.094 + q, 0.1 + q), (0.335, -0.028, 0, 0.092 + q, 0.098 + q), (0.362 + 0.4 * q, -0.026, 0, 0.076 + q, 0.082 + q)], step=st, round0=0.012, round1=0.012)
    out += tag([scarf], "scarf", "scarf")
    if s <= 200:
        t = 0.013 if mid else 0.017
        out += tag([L("scarf-end", "thorax", "knit-hat", 0.46, [(0.3, -0.118, 0.045, 0.036, t), (0.24, -0.152, 0.05, 0.038, t), (0.16, -0.19, 0.052, 0.039, t), (0.075, -0.189, 0.054, 0.04, t)], step=st, role="detail" if fine else "body", round1=0.004)], "scarf", "scarf")
    coat = [
        L("coat-chest", "thorax", "wool", 0.3, [(-0.05, -0.045, 0, 0.166, 0.127, 2.3), (0.04, -0.048, 0, 0.172, 0.134, 2.3), (0.13, -0.05, 0, 0.182, 0.144, 2.4), (0.2, -0.042, 0, 0.19, 0.132, 2.6), (0.245, -0.036, 0, 0.176, 0.106, 2.8), (0.275, -0.032, 0, 0.125, 0.09, 2.4), (0.3, -0.03, 0, 0.082, 0.08, 2.0)], step=st),
        L("coat-waist", "lumbar", "wool", 0.3, [(-0.04, -0.04, 0, 0.176, 0.133, 2.3), (0.05, -0.043, 0, 0.167, 0.127, 2.3), (0.12, -0.045, 0, 0.165, 0.126, 2.3), (0.2, -0.047, 0, 0.169, 0.131, 2.3)], step=st),
        L("coat-hip", "pelvis", "wool", 0.3, [(-0.07, 0.006, 0, 0.196, 0.146, 2.4), (0.0, 0.006, 0, 0.196, 0.146, 2.4), (0.06, 0.002, 0, 0.186, 0.14, 2.3), (0.13, -0.002, 0, 0.176, 0.133, 2.3)], step=st),
    ]
    if s == 500:
        coat = [
            L("coat-chest", "thorax", "wool", 0.3, [(-0.05, -0.04, 0, 0.166, 0.132, 2.3), (0.04, -0.043, 0, 0.172, 0.139, 2.3), (0.13, -0.044, 0, 0.182, 0.15, 2.4), (0.2, -0.036, 0, 0.19, 0.138, 2.6), (0.245, -0.032, 0, 0.176, 0.11, 2.8), (0.275, -0.03, 0, 0.125, 0.09, 2.4), (0.3, -0.03, 0, 0.082, 0.08, 2.0)], step=st),
            L("coat-waist", "lumbar", "wool", 0.3, [(-0.12, -0.036, 0, 0.19, 0.142, 2.4), (-0.04, -0.04, 0, 0.178, 0.134, 2.3), (0.05, -0.043, 0, 0.167, 0.127, 2.3), (0.12, -0.045, 0, 0.165, 0.126, 2.3), (0.2, -0.047, 0, 0.169, 0.131, 2.3)], step=st),
        ]
    out += tag(coat, "coat-torso", "coat")
    skirt = L("coat-skirt-l", "pelvis", "wool", 0.28, [(0.0, 0.006, 0.084, 0.112, 0.142, 2.5), (-0.1, 0.008, 0.088, 0.114, 0.146, 2.5), (-0.2, 0.01, 0.092, 0.116, 0.15, 2.6), (-0.3, 0.012, 0.096, 0.118, 0.154, 2.7)], step=st, weights=[{"bone": "pelvis", "weight": 0.5}, {"bone": "thigh-l", "weight": 0.5}])
    out += tag(B(skirt), "coat-torso", "coat")
    yoke = L("coat-yoke-l", "clavicle-l", "wool", 0.32, [clav_row((0.238, -0.036, 0.05), 0.042, 0.088), clav_row((0.228, -0.034, 0.11), 0.042, 0.08), clav_row((0.212, -0.03, 0.165), 0.044, 0.07)], step=st, round1=0.03)
    out += tag(B(yoke), "coat-torso", "coat")
    sleeve = [
        L("sleeve-upper-l", "upper-arm-l", "wool", 0.3, [(0.0, 0, 0, 0.064, 0.07), (0.07, 0.003, 0, 0.062, 0.068), (0.16, 0.002, 0, 0.057, 0.061), (0.25, 0, 0, 0.053, 0.055), (0.316, 0, 0, 0.051, 0.053)], step=st, round0=0.05, round1=0.04),
        L("sleeve-fore-l", "forearm-l", "wool", 0.3, [(0.0, 0, 0, 0.051, 0.053), (0.06, 0, 0, 0.05, 0.051), (0.14, 0, 0, 0.044, 0.044), (0.21, 0, 0, 0.038, 0.041), (0.236, 0, 0, 0.039, 0.042)], step=st, round0=0.04),
    ]
    for x in sleeve:
        out += tag(B(x), "coat-sleeves", "sleeve")
    out += hand(s, "", "leather-glove", 0.2, 1.0, "gloves", "glove", "glove")
    legs = [
        L("jeans-thigh-l", "thigh-l", "denim", 0.42, [(-0.02, 0, 0, 0.09, 0.098), (0.06, 0.004, 0, 0.088, 0.096), (0.16, 0.008, 0, 0.08, 0.087), (0.26, 0.006, 0, 0.068, 0.074), (0.34, 0.004, 0, 0.063, 0.069), (0.4, 0.006, 0, 0.06, 0.066)], step=st, round0=0.03, round1=0.05),
        L("jeans-shank-l", "shank-l", "denim", 0.42, [(-0.02, 0.006, 0, 0.06, 0.066), (0.03, 0.002, 0, 0.059, 0.064), (0.1, -0.01, 0, 0.062, 0.068), (0.17, -0.01, 0, 0.06, 0.063), (0.26, -0.006, 0, 0.056, 0.058), (0.34, -0.004, 0, 0.055, 0.057), (0.39, -0.002, 0, 0.056, 0.059)], step=st, round0=0.04),
    ]
    for x in legs:
        out += tag(B(x), "jeans", "denim")
    low = -0.088 if s >= 200 else -0.071
    hi = 0.008 if s == 500 else 0.0
    boot = [
        L("boot-shaft-l", "shank-l", "leather", 0.16, [(0.32, -0.004, 0, 0.044, 0.047), (0.38, -0.002, 0, 0.043, 0.05), (0.43, 0.002, 0, 0.042, 0.052)], step=st),
        L("boot-l", "foot-l", "leather", 0.16, [(-0.07, (0.02 + low) / 2, 0, 0.03, (0.02 - low) / 2, 2.6), (-0.045, (0.046 + low) / 2, 0, 0.038, (0.046 - low) / 2, 2.6), (-0.005, (0.062 + low) / 2, 0, 0.042, (0.062 - low) / 2, 2.6), (0.04, (0.03 + low) / 2, 0, 0.045, (0.03 - low) / 2, 2.6), (0.09, (-0.004 + low) / 2, 0, 0.049, (-0.004 - low) / 2, 2.6), (0.138, (-0.02 + low) / 2, 0, 0.051, (-0.02 - low) / 2, 2.6)], step=st, round0=0.008),
        L("boot-toe-l", "toe-l", "leather", 0.16, [(-0.012, (0.068 + hi + low + 0.088) / 2, 0, 0.051, (0.068 + hi - low - 0.088) / 2, 2.6), (0.03, (0.06 + hi + low + 0.088) / 2, 0, 0.048, (0.06 + hi - low - 0.088) / 2, 2.6), (0.06, (0.05 + hi + low + 0.088) / 2, 0, 0.039, (0.05 + hi - low - 0.088) / 2, 2.4), (0.076, (0.044 + hi + low + 0.088) / 2, 0, 0.026, (0.044 + hi - low - 0.088) / 2, 2.2)], step=st, round1=0.01),
    ]
    if s <= 100:
        boot += [
            X("boot-sole-l", "foot-l", "leather", 0.1, [(-0.03, -0.08), (0.03, -0.08), (0.041, -0.055), (0.043, 0.0), (0.048, 0.07), (0.053, 0.138), (-0.053, 0.138), (-0.048, 0.07), (-0.043, 0.0), (-0.041, -0.055)], "zx", -0.088, -0.068),
            X("boot-toe-sole-l", "toe-l", "leather", 0.1, [(-0.053, -0.012), (0.053, -0.012), (0.05, 0.03), (0.041, 0.062), (0.022, 0.087), (-0.022, 0.087), (-0.041, 0.062), (-0.05, 0.03)], "zx", 0.0, 0.02),
        ]
    for x in boot:
        out += tag(B(x), "boots", "boot")
    if mid:
        for k, (u, w) in enumerate([(0.1, 0.05), (0.02, 0.05)] + ([(0.1, -0.05), (0.02, -0.05)] if fine else [])):
            out += tag([E("coat-button-%d" % (k + 1), "thorax", "wool", 0.18, (u, -0.185 if u > 0.05 else -0.176, w), (0.011, 0.005 if fine else 0.0075, 0.011), role="detail")], "coat-torso", "coat")
    if fine:
        out += tag(B(kit.box("coat-pocket-l", "pelvis", "wool", 0.24, (-0.03, -0.134, 0.12), (0.022, 0.012, 0.075), corner=0.004, role="detail", weights=[{"bone": "pelvis", "weight": 0.5}, {"bone": "thigh-l", "weight": 0.5}])), "coat-torso", "coat")
        out += tag(B(L("coat-lapel-l", "thorax", "wool", 0.26, [(0.262, -0.128, 0.05, 0.03, 0.009), (0.2, -0.172, 0.042, 0.04, 0.009), (0.135, -0.19, 0.02, 0.022, 0.008)], step=st, role="detail")), "coat-torso", "coat")
    return out


def clav_row(p, rw, rv):
    """📏 A station row on the left clavicle bone from a thorax-frame point; `rw` is the half height of the section, `rv` its half depth."""
    u, v, w = clav(p)
    return (u, v, w, rw, rv, 2.3)


def hand(s, pre, material, tone, k, region, layer, name):
    """🧤 A pair of hands at scale 1:s on the hand, finger and thumb bones; `k` scales the stated adult glove (1.0) and is not used for the toddler, whose mittens are stated separately."""
    out = []
    P = pre
    cuff = L(P + name + "-cuff-l", P + "forearm-l", material, tone, [(0.19, 0, 0, 0.041, 0.044), (0.225, 0, 0, 0.039, 0.042), (0.256, 0, 0, 0.03, 0.036)], step=0.02)
    if s <= 200:
        out += tag(B(cuff), region, layer)
    if s <= 100:
        palm = L(P + name + "-l", P + "hand-l", material, tone, [(-0.012, 0, 0, 0.019, 0.031, 2.4), (0.02, 0.001, 0, 0.0165, 0.04, 2.5), (0.06, -0.002, 0, 0.0155, 0.044, 2.5), (0.098, -0.004, 0, 0.013, 0.043, 2.5)], step=0.015, round1=0.006)
        out += tag(B(palm), region, layer)
        out += tag(B(E(P + name + "-thenar-l", P + "hand-l", material, tone, (0.034, 0.026, -0.011), (0.032, 0.017, 0.014))), region, layer)
        out += tag(B(C(P + name + "-thumb-l", P + "thumb-l", material, tone, (0.0, 0, -0.004), (0.062, 0, -0.006), 0.0135, 0.0115)), region, layer)
        out += tag(B(C(P + name + "-thumbtip-l", P + "thumbtip-l", material, tone, (0.0, 0, -0.006), (0.024, 0, -0.006), 0.0115, 0.0095)), region, layer)
        if s == 50:
            for i, (v, r, tip) in enumerate([(0.0315, 0.0105, 0.04), (0.0105, 0.0108, 0.046), (-0.0105, 0.0102, 0.041), (-0.0305, 0.0092, 0.03)]):
                out += tag(B(C(P + name + "-finger-%d-l" % (i + 1), P + "fingers-l", material, tone, (-0.004, v, 0), (0.045, v, 0), r, r * 0.95)), region, layer)
                out += tag(B(C(P + name + "-fingertip-%d-l" % (i + 1), P + "fingertips-l", material, tone, (0.0, v, 0), (tip, v * 0.96, 0), r * 0.95, r * 0.8)), region, layer)
        else:
            out += tag(B(L(P + name + "-fingers-l", P + "fingers-l", material, tone, [(-0.006, 0, 0, 0.0115, 0.042, 3), (0.045, 0, 0, 0.011, 0.041, 3)], smooth=False, round1=0.006)), region, layer)
            out += tag(B(L(P + name + "-fingertips-l", P + "fingertips-l", material, tone, [(-0.004, 0, 0, 0.011, 0.041, 3), (0.026, 0.001, 0, 0.01, 0.038, 3), (0.04, 0.003, 0, 0.009, 0.03, 2.5)], smooth=False, round1=0.006)), region, layer)
    else:
        t = 0.017 if s == 200 else 0.0355
        mitt = L(P + name + "-l", P + "hand-l", material, tone, [(-0.012, 0, 0, t + 0.002, 0.032, 2.4), (0.03, 0.004, 0, t, 0.046, 2.5), (0.095, 0, 0, t, 0.045, 2.6), (0.15, -0.002, -0.006, t, 0.04, 2.6), (0.183, -0.003, -0.012, t, 0.028, 2.4)], step=0.03, round1=0.006)
        out += tag(B(mitt), region, layer)
    return out


def toddler_body(s):
    """👶 The toddler at scale 1:s: a large round head with a small low face under a knit hat with pompom, almost no neck, a barrel trunk with a belly and a nappy seat in a padded overall, short thick limbs, mittens and rubber boots."""
    st = STEP[s] * 0.7
    pad = 0.011 if s == 500 else 0.0
    fine, mid = s == 50, s <= 100
    out = []
    out += tag([L("toddler-head", "toddler-head", "skin", 0.84, [(-0.04, -0.036, 0, 0.03, 0.03), (-0.026, -0.022, 0, 0.05, 0.054), (0.0, -0.007, 0, 0.063, 0.074), (0.04, 0, 0, 0.066, 0.082), (0.085, 0.004, 0, 0.066, 0.084), (0.125, 0.006, 0, 0.055, 0.07), (0.148, 0.008, 0, 0.036, 0.046)], step=st, round0=0.005, round1=0.012)], "toddler-face")
    out += tag([L("toddler-neck", "toddler-neck", "skin", 0.8, [(-0.02, 0, 0, 0.04, 0.04), (0.03, -0.004, 0, 0.037, 0.038), (0.07, -0.008, 0, 0.038, 0.04)], step=st)], "toddler-face")
    out += tag([L("toddler-hat", "toddler-head", "knit-hat", 0.4, [(0.052 if s < 500 else 0.06, 0.003, 0, 0.073, 0.09), (0.09, 0.005, 0, 0.074, 0.092), (0.13, 0.007, 0, 0.062, 0.078), (0.157, 0.008, 0, 0.038, 0.048)], step=st, round1=0.012)], "toddler-hat", "hat")
    if s <= 200:
        out += tag([L("toddler-hat-cuff", "toddler-head", "knit-hat", 0.34, [(0.046, 0.003, 0, 0.077, 0.094), (0.064, 0.004, 0, 0.08, 0.097), (0.084, 0.005, 0, 0.078, 0.096)], step=st, round0=0.005, round1=0.005)], "toddler-hat", "hat")
    out += tag([E("toddler-pompom", "toddler-head", "knit-hat", 0.46, (0.188, 0.008, 0), (0.028, 0.028, 0.028) if s <= 200 else (0.038, 0.038, 0.038))], "toddler-hat", "hat")
    if mid:
        out += tag([E("toddler-nose", "toddler-head", "skin", 0.86, (0.004, -0.083, 0), (0.0095, 0.0105, 0.0105))], "toddler-face")
        out += tag(B(E("toddler-ear-l", "toddler-head", "skin", 0.76, (0.022, 0.012, 0.065), (0.022, 0.013, 0.0075))), "toddler-face")
        out += tag(B(E("toddler-cheek-l", "toddler-head", "skin", 0.84, (-0.006, -0.05, 0.036), (0.026, 0.024, 0.026))), "toddler-face")
    if fine:
        out += tag([E("toddler-lips", "toddler-head", "skin", 0.7, (-0.019, -0.073, 0), (0.0065, 0.0075, 0.016)), E("toddler-brow", "toddler-head", "skin", 0.82, (0.05, -0.074, 0), (0.014, 0.012, 0.046))], "toddler-face")
        out += tag(B(E("toddler-eye-l", "toddler-head", "skin", 0.5, (0.03, -0.074, 0.028), (0.0075, 0.0075, 0.012), role="detail")), "toddler-face")
    suit = [L("toddler-suit", "toddler-torso", "padded-suit", 0.55, [(-0.03, -0.004, 0, 0.096, 0.086, 2.3), (0.0, -0.008, 0, 0.106, 0.096, 2.3), (0.07, -0.016, 0, 0.106, 0.101, 2.2), (0.14, -0.01, 0, 0.1, 0.092, 2.2), (0.21, -0.004, 0, 0.097, 0.08, 2.4), (0.25, 0, 0, 0.072, 0.064, 2.2), (0.285, 0, 0, 0.05, 0.05, 2.0)], step=st, round0=0.028)]
    if s <= 200:
        suit.append(L("toddler-collar", "toddler-torso", "padded-suit", 0.5, [(0.262, 0, 0, 0.056, 0.056), (0.285, 0, 0, 0.058, 0.058), (0.31, -0.002, 0, 0.052, 0.053)], step=st, round0=0.006, round1=0.006))
    if fine:
        suit.append(C("toddler-zip", "toddler-torso", "padded-suit", 0.36, (0.02, -0.107, 0), (0.285, -0.052, 0), 0.0042, role="detail", slender=True))
    out += tag(suit, "toddler-suit", "suit")
    limbs = [
        L("toddler-sleeve-l", "toddler-arm-l", "padded-suit", 0.55, [(0.0, 0, 0, 0.04 + pad, 0.042 + pad), (0.07, 0, 0, 0.038 + pad, 0.039 + pad), (0.135, 0, 0, 0.036 + pad, 0.036 + pad)], step=st, round0=0.03, round1=0.03),
        L("toddler-forearm-l", "toddler-forearm-l", "padded-suit", 0.55, [(0.0, 0, 0, 0.036, 0.036), (0.05, 0, 0, 0.034, 0.034), (0.096, 0, 0, 0.03, 0.03), (0.108, 0, 0, 0.027, 0.027)], step=st, round0=0.028),
        L("toddler-thigh-l", "toddler-thigh-l", "padded-suit", 0.55, [(0.0, 0.002, 0, 0.058, 0.062), (0.06, 0.003, 0, 0.057, 0.06), (0.13, 0.002, 0, 0.051, 0.053), (0.18, 0.002, 0, 0.047, 0.05)], step=st, round0=0.045, round1=0.04),
        L("toddler-shank-l", "toddler-shank-l", "padded-suit", 0.55, [(0.0, 0.002, 0, 0.047, 0.05), (0.05, -0.004, 0, 0.046, 0.049), (0.1, -0.002, 0, 0.043, 0.045), (0.128, 0, 0, 0.041, 0.043)], step=st, round0=0.036),
    ]
    for x in limbs:
        out += tag(B(x), "toddler-suit", "suit")
    g = 0.012 if s == 500 else 0.0
    mitten = [L("toddler-mitten-l", "toddler-hand-l", "knit-hat", 0.4, [(-0.014, 0, 0, 0.026 + g, 0.028 + g), (0.02, 0.002, 0, 0.022 + g, 0.033 + g, 2.4), (0.05, 0.0, 0, 0.021 + g, 0.031 + g, 2.4), (0.066, -0.002, 0, 0.017 + g, 0.024 + g, 2.2)], step=st, round1=0.012)]
    if mid:
        mitten.append(C("toddler-mitten-thumb-l", "toddler-hand-l", "knit-hat", 0.4, (0.012, 0.026, -0.004), (0.036, 0.04, -0.006), 0.0105, 0.009))
    for x in mitten:
        out += tag(B(x), "toddler-mittens", "mitten")
    low = -0.04 if s >= 200 else -0.03
    boots = [
        L("toddler-boot-shaft-l", "toddler-shank-l", "rubber", 0.28, [(0.075, -0.002, 0, 0.044, 0.046), (0.12, 0, 0, 0.042, 0.045), (0.178, 0.002, 0, 0.038, 0.044)], step=st, round0=0.004),
        L("toddler-boot-l", "toddler-foot-l", "rubber", 0.28, [(-0.04, (0.02 + low) / 2, 0, 0.027, (0.02 - low) / 2, 2.5), (-0.012, (0.036 + low) / 2, 0, 0.034, (0.036 - low) / 2, 2.5), (0.03, (0.018 + low) / 2, 0, 0.036, (0.018 - low) / 2, 2.5), (0.075, (0.006 + low) / 2, 0, 0.037, (0.006 - low) / 2, 2.5), (0.1, (0.002 + low) / 2, 0, 0.031, (0.002 - low) / 2, 2.3)], step=st, round0=0.006, round1=0.014),
    ]
    if s <= 100:
        boots.append(X("toddler-boot-sole-l", "toddler-foot-l", "rubber", 0.18, [(-0.022, -0.047), (0.022, -0.047), (0.033, -0.02), (0.037, 0.04), (0.036, 0.085), (0.022, 0.113), (-0.022, 0.113), (-0.036, 0.085), (-0.037, 0.04), (-0.033, -0.02)], "zx", -0.042, -0.027))
    for x in boots:
        out += tag(B(x), "toddler-boots", "boot")
    return out


def thermal(body, regions, s, dia):
    """🌡️ The thermal list: every body surface as a reference with its region and outer layer, at 1:50 every inner layer as a nested shell, and the breath and plume emitters as solids."""
    by = {r["id"]: r for r in regions}
    out = []
    exposed = {"head": 0.04, "toddler-head": 0.052}
    for b in body:
        region, layer = b["_r"]
        if b["id"] in ("neck", "toddler-neck"):
            continue
        if b["id"] in exposed:
            skin = {k: v for k, v in b.items() if not k.startswith("_")}
            skin["id"] = "th-" + b["id"] + "-skin"
            skin["sections"] = [x for x in b["sections"] if x["at"][0] <= exposed[b["id"]]]
            skin["region"] = region
            out.append(skin)
            continue
        e = {"region": region}
        if by[region]["layers"]:
            e["layer"] = by[region]["layers"][-1]["id"]
        out.append(kit.ref("th-" + b["id"], b["id"], **e))
        layers = by[region]["layers"]
        trunk = b["id"] in ("coat-chest", "coat-waist", "coat-hip", "toddler-suit", "jeans-thigh-l", "jeans-thigh-r", "beanie", "toddler-hat", "boot-l", "boot-r", "toddler-boot-l", "toddler-boot-r")
        if (s <= 100 or trunk) and len(layers) > 1 and b["kind"] in ("loft", "capsule", "ellipsoid") and b.get("role") != "detail" and not b.get("slender"):
            inset = 0.0
            for inner, outer in zip(reversed(layers[:-1]), reversed(layers[1:])):
                inset += outer.get("thicknessM", 0.004)
                small = max([min(x["rx"], x["ry"]) for x in b["sections"]] if b["kind"] == "loft" else [b.get("r0", 1), b.get("r1", 1)] if b["kind"] == "capsule" else [min(b["radii"])])
                if small - inset < 0.004:
                    break
                out.append(kit.shell(b, "th-%s-%s" % (b["id"], inner["id"]), inset, material=inner["material"], tone=0.6, region=region, layer=inner["id"]))
    em = lambda id, bone, c, r, region, name: E(id, bone, "skin", 0.5, c, r, region=region, tags=["emitter", name])
    nose = s <= 100
    t = max(0.004, s * 0.000075 + 0.0005)
    out += [
        em("th-nose-breath", "head", (-0.018, -0.116 if nose else -0.095, 0), (0.003, 0.004, dia["nose-breath"] / 2), "face", "nose-breath"),
        em("th-mouth-breath", "head", (-0.041, -0.094, 0), (0.003, 0.004, dia["mouth-breath"] / 2), "face", "mouth-breath"),
        em("th-toddler-nose-breath", "toddler-head", (-0.002, -0.091 if nose else -0.081, 0), (dia["toddler-nose-breath"] * 0.3, dia["toddler-nose-breath"] * 0.4, dia["toddler-nose-breath"] / 2), "toddler-face", "toddler-nose-breath"),
        em("th-toddler-mouth-breath", "toddler-head", (-0.019, -0.078, 0), (dia["toddler-mouth-breath"] * 0.3, dia["toddler-mouth-breath"] * 0.4, dia["toddler-mouth-breath"] / 2), "toddler-face", "toddler-mouth-breath"),
        em("th-plume", "head", (0.164 + t, 0.012, 0), (t, 0.05, 0.045), "coat-torso", "plume"),
        em("th-toddler-plume", "toddler-head", (0.216 + t, 0.008, 0), (t, 0.04, 0.04), "toddler-suit", "toddler-plume"),
    ]
    return out


def stated(s):
    """🪚 The loft constructor of a structure list: splined between the stated stations at 1:50, the stated stations alone at the coarser scales (so a coarse bone keeps the true size of its 1:50 source)."""
    return lambda *a, **k: kit.loft(*a, **{**k, "smooth": k.get("smooth", True) and s == 50})


def bone_solid(x, member, part="member"):
    """🦴 Tags a solid as a structural part of a member bone."""
    x["member"] = member
    x["part"] = part
    return x


def ring(id, bone, tone, back, front, half_width, r, sweep_deg, side):
    """🫁 One rib as an arc of an elliptical ring through its head at the spine (`back`) and its sternal end (`front`), both (u, v) in the bone frame; `side` -1 is the right rib, +1 the left."""
    du, dv = front[0] - back[0], front[1] - back[1]
    d = math.hypot(du, dv)
    f = (du / d, dv / d)
    n = (-f[1], f[0], 0) if -f[1] > 0 else (f[1], -f[0], 0)
    n = n if side < 0 else (-n[0], -n[1], 0)
    centre = ((back[0] + front[0]) / 2, (back[1] + front[1]) / 2, 0)
    loop = [(r * math.cos(a * math.pi / 4), half_width + r * math.sin(a * math.pi / 4)) for a in range(8)]
    loop.append(loop[0])
    return kit.revolve(id, bone, "bone", tone, loop, n, centre, sweepRad=round(math.radians(sweep_deg), 4), lateralScale=round((d / 2) / half_width, 4))


def spine(pre, bone, n, first, pitch, bow, body, process, tone, names):
    """🪜 `n` vertebrae on a bone from `first` upwards every `pitch`: body ellipsoid (half height, half depth, half width) bowed by `bow(t)` along v, and a spinous process pointing back and down."""
    out = []
    for i in range(n):
        t = (i + 0.5) / n
        u, v = first + pitch * (i + 0.5), bow(t)
        h, d, w = body(t)
        out.append(bone_solid(E("%s%s" % (pre, names[i]), bone, "bone", tone, (u, v, 0), (h, d, w)), bone))
        if process:
            out.append(bone_solid(C("%s%s-process" % (pre, names[i]), bone, "bone", tone, (u, v + d * 0.8, 0), (u - process * 0.45, v + d + process, 0), 0.0055 if process > 0.02 else 0.0042, 0.0045 if process > 0.02 else 0.0038), bone))
    return out


def parent_structure(s, rig):
    """🩻 The parent's skeleton at scale 1:s: skull with mandible, cervical, thoracic and lumbar vertebrae on their curves, ribs from the spine to the sternum, clavicles, scapulae, pelvis with sacrum, long bones with their epiphyses, hands and feet, plus joints and segment masses."""
    T = 0.72
    out = []
    L = stated(s)
    S = lambda x, m: bone_solid(x, m)
    out += [S(E("cranium", "head", "bone", T, (0.058, 0.014, 0), (0.078, 0.09, 0.07)), "head"), S(L("facial-skeleton", "head", "bone", T, [(0.03, -0.056, 0, 0.058, 0.03), (0.0, -0.062, 0, 0.05, 0.034), (-0.03, -0.062, 0, 0.034, 0.03)], step=0.02, round0=0.006, round1=0.006), "head")]
    out += [S(x, "head") for x in B(C("mandible-ramus-l", "head", "bone", T, (-0.004, -0.004, 0.05), (-0.05, -0.012, 0.046), 0.009, 0.008))]
    out += [S(x, "head") for x in B(C("mandible-body-l", "head", "bone", T, (-0.05, -0.012, 0.046), (-0.072, -0.078, 0.01), 0.0085, 0.01))]
    if s == 50:
        out += [S(x, "head") for x in B(C("zygomatic-l", "head", "bone", T, (0.012, -0.006, 0.064), (0.01, -0.07, 0.052), 0.006, 0.0075))]
    cn = ["c7", "c6", "c5", "c4", "c3", "c2", "c1"]
    tn = ["t12", "t11", "t10", "t9", "t8", "t7", "t6", "t5", "t4", "t3", "t2", "t1"]
    ln = ["l5", "l4", "l3", "l2", "l1"]
    kyph = lambda t: 0.026 * math.sin(math.pi * t) * (0.85 + 0.3 * t)
    lord = lambda t: -0.016 * math.sin(math.pi * t)
    out += spine("", "neck", 7, -0.008, 0.0165, lambda t: -0.008 * math.sin(math.pi * t) - 0.01 * t, lambda t: (0.0088, 0.0085, 0.012), 0.016 if s == 50 else 0, T, cn)
    out += spine("", "thorax", 12, 0.0, 0.0253, kyph, lambda t: (0.0134, 0.0155 - 0.004 * t, 0.02 - 0.006 * t), 0.026 if s == 50 else 0, T, tn)
    out += spine("", "lumbar", 5, 0.0, 0.034, lord, lambda t: (0.0178, 0.017, 0.0235), 0.024 if s == 50 else 0, T, ln)
    if s == 50:
        out += [S(C("%s-transverse" % n, "lumbar", "bone", T, (0.034 * (i + 0.5), lord((i + 0.5) / 5) + 0.012, -0.045), (0.034 * (i + 0.5), lord((i + 0.5) / 5) + 0.012, 0.045), 0.005), "lumbar") for i, n in enumerate(ln)]
    ribs = [(1, 0.052, 180), (2, 0.078, 180), (3, 0.098, 180), (4, 0.112, 180), (5, 0.122, 180), (6, 0.129, 180), (7, 0.134, 180), (8, 0.137, 158), (9, 0.136, 146), (10, 0.132, 132), (11, 0.124, 84), (12, 0.112, 58)]
    chosen = ribs if s == 50 else [x for x in ribs if x[0] in ((1, 3, 5, 7, 9, 11) if s == 100 else (2, 5, 8, 11) if s == 200 else (3, 7, 10))]
    rr = 0.0048 if s == 50 else 0.0075
    for k, hw, sweep in chosen:
        t = 1 - (k - 0.5) / 12
        back = (0.3035 * t, kyph(t) + 0.014)
        front = (0.228 - (k - 1) * 0.0235, -0.104 - (k - 1) * 0.0058) if k <= 7 else (0.228 - 6 * 0.0235 - (k - 7) * 0.03, -0.139 + (k - 7) * 0.012)
        out += [S(ring("rib-%d-r" % k, "thorax", 0.86, back, front, hw, rr, sweep, -1), "thorax"), S(ring("rib-%d-l" % k, "thorax", 0.86, back, front, hw, rr, sweep, 1), "thorax")]
    out += [S(L("sternum", "thorax", "bone", T, [(0.238, -0.104, 0, 0.03, 0.007), (0.2, -0.112, 0, 0.022, 0.0065), (0.14, -0.126, 0, 0.019, 0.006), (0.09, -0.139, 0, 0.017, 0.006), (0.062, -0.142, 0, 0.008, 0.005)], step=0.02, round1=0.006), "thorax")]
    if s <= 200:
        out += [S(x, "thorax") for x in B(C("costal-arch-l", "thorax", "bone", 0.86, (0.085, -0.139, 0.016), (0.0, -0.098, 0.105), 0.006, 0.005))]
    out += [S(x, x["bone"]) for x in B(L("clavicle-l", "clavicle-l", "bone", T, [(0.0, 0.0, 0, 0.0085, 0.0085), (0.05, -0.01, 0, 0.0065, 0.0065), (0.11, 0.04, 0, 0.0065, 0.0065), (0.155, 0.088, 0.0, 0.007, 0.009)], step=0.02, round0=0.005, round1=0.005))]
    blade = L("scapula-l", "clavicle-l", "bone", T, [clav_row2((0.09, 0.072, 0.086), 0.008, 0.012), clav_row2((0.15, 0.066, 0.102), 0.0075, 0.034), clav_row2((0.205, 0.056, 0.116), 0.0075, 0.05), clav_row2((0.238, 0.046, 0.112), 0.0075, 0.044)], step=0.02, round0=0.004, round1=0.004)
    out += [S(x, x["bone"]) for x in B(blade)]
    out += [S(x, "scapula" + x["id"][-2:]) for x in B(C("scapular-spine-l", "clavicle-l", "bone", T, clav((0.214, 0.062, 0.078)), clav((0.25, -0.012, 0.176)), 0.0065, 0.0085))]
    out += [S(x, "scapula" + x["id"][-2:]) for x in B(C("scapula-neck-l", "clavicle-l", "bone", T, clav((0.215, 0.04, 0.13)), clav((0.224, -0.02, 0.158)), 0.01))]
    out += [S(x, "scapula" + x["id"][-2:]) for x in B(E("glenoid-l", "scapula-l", "bone", T, (0.03, -0.004, -0.018), (0.02, 0.016, 0.008)))]
    out += [S(L("sacrum", "pelvis", "bone", T, [(0.105, 0.044, 0, 0.052, 0.022), (0.065, 0.07, 0, 0.046, 0.018), (0.025, 0.086, 0, 0.03, 0.012), (-0.012, 0.086, 0, 0.012, 0.008)], step=0.02, round1=0.006), "pelvis")]
    out += [S(x, "pelvis") for x in B(X("ilium-l", "pelvis", "bone", T, [(0.072, 0.036), (0.0827, 0.0529), (-0.0473, 0.1349), (-0.058, 0.118)], "yz", 0.0, 0.13))]
    out += [S(x, "pelvis") for x in B(C("ischium-l", "pelvis", "bone", T, (0.012, 0.016, 0.09), (-0.078, 0.036, 0.062), 0.019, 0.015))]
    out += [S(x, "hip-bone" + x["id"][-2:]) for x in B(C("iliac-column-l", "hip-bone-l", "bone", T, (0.004, 0, 0), (0.078, 0, 0), 0.017, 0.021))]
    out += [S(x, "hip-bone" + x["id"][-2:]) for x in B(E("acetabulum-l", "hip-bone-l", "bone", T, (0.1, 0, 0), (0.03, 0.03, 0.026)))]
    out += [S(x, "pelvis") for x in B(C("pubis-l", "pelvis", "bone", T, (-0.006, -0.026, 0.082), (-0.046, -0.072, 0.008), 0.0115, 0.0105))]
    if s <= 200:
        out += [S(x, "pelvis") for x in B(C("ischiopubic-ramus-l", "pelvis", "bone", T, (-0.05, -0.068, 0.012), (-0.08, 0.03, 0.056), 0.0085, 0.01))]
    leg = [
        E("femur-head-l", "thigh-l", "bone", T, (0, 0, 0), (0.0235, 0.0235, 0.0235)),
        C("femur-neck-l", "thigh-l", "bone", T, (0, 0, 0), (0.03, 0.0, 0.05), 0.015, 0.017),
        E("femur-trochanter-l", "thigh-l", "bone", T, (0.024, 0.0, 0.062), (0.024, 0.018, 0.015)),
        L("femur-l", "thigh-l", "bone", T, [(0.035, 0.0, 0.052, 0.017, 0.016), (0.12, 0.008, 0.036, 0.0142, 0.0142), (0.24, 0.01, 0.018, 0.0138, 0.0138), (0.34, 0.004, 0.006, 0.0185, 0.0165), (0.378, 0.0, 0.0, 0.034, 0.024)], step=0.03),
        E("femur-condyles-l", "thigh-l", "bone", T, (0.392, -0.004, 0), (0.022, 0.031, 0.04)),
        E("patella-l", "thigh-l", "bone", T, (0.388, 0.035, 0), (0.021, 0.01, 0.021)),
        E("tibia-plateau-l", "shank-l", "bone", T, (0.018, 0.0, 0), (0.016, 0.027, 0.038)),
        L("tibia-l", "shank-l", "bone", T, [(0.025, 0.002, -0.002, 0.03, 0.023), (0.08, 0.008, -0.004, 0.0165, 0.0175), (0.22, 0.006, -0.006, 0.0125, 0.0135), (0.36, 0.0, -0.006, 0.0135, 0.0135), (0.408, 0.0, -0.006, 0.021, 0.018)], step=0.03),
        E("malleolus-medial-l", "shank-l", "bone", T, (0.416, 0.002, -0.027), (0.014, 0.012, 0.009)),
        C("fibula-l", "shank-l", "bone", T, (0.03, -0.012, 0.031), (0.425, -0.006, 0.026), 0.0068, 0.0075),
        E("talus-l", "foot-l", "bone", T, (0.008, -0.02, 0), (0.028, 0.017, 0.02)),
        L("calcaneus-l", "foot-l", "bone", T, [(-0.058, -0.052, 0.002, 0.016, 0.019), (-0.03, -0.047, 0.002, 0.019, 0.022), (0.012, -0.046, 0.004, 0.018, 0.017)], step=0.02, round0=0.006),
        L("tarsus-l", "foot-l", "bone", T, [(0.028, -0.03, 0, 0.026, 0.017), (0.062, -0.04, 0, 0.032, 0.013)], smooth=False),
    ]
    rays = [(-0.03, 0.0075, 0.066), (-0.015, 0.0058, 0.072), (0.0, 0.0055, 0.068), (0.014, 0.0052, 0.064), (0.028, 0.0058, 0.06)]
    for i, (w, r, ln_) in enumerate(rays):
        leg.append(C("metatarsal-%d-l" % (i + 1), "foot-l", "bone", T, (0.066, -0.046, w * 0.8), (0.066 + ln_, -0.076, w * 1.25), r, r * 0.95))
        leg.append(C("toe-%d-l" % (i + 1), "toe-l", "bone", T, (0.004, 0.012, w * 1.25), (0.05 - 0.006 * i if i else 0.058, 0.01, w * 1.3), r * 0.95, r * 0.8))
    arm = [
        E("humerus-head-l", "upper-arm-l", "bone", T, (0.002, 0, -0.004), (0.023, 0.023, 0.023)),
        L("humerus-l", "upper-arm-l", "bone", T, [(0.02, 0.0, 0.004, 0.017, 0.017), (0.08, 0.0, 0.004, 0.0115, 0.0115), (0.22, 0.0, 0.002, 0.0105, 0.0105), (0.29, 0.0, 0.0, 0.019, 0.011), (0.312, 0.0, 0.0, 0.03, 0.012)], step=0.03),
        E("humerus-condyles-l", "upper-arm-l", "bone", T, (0.314, 0.002, 0), (0.012, 0.014, 0.031)),
        C("ulna-l", "forearm-l", "bone", T, (-0.016, -0.012, -0.002), (0.244, -0.016, -0.004), 0.0105, 0.0055),
        C("radius-l", "forearm-l", "bone", T, (0.012, 0.012, 0.008), (0.246, 0.012, 0.0), 0.0062, 0.0105),
        E("carpus-l", "hand-l", "bone", T, (0.014, 0.0, 0), (0.016, 0.026, 0.01)),
    ]
    fingers = [(0.031, 0.0048, 0.04), (0.0105, 0.005, 0.046), (-0.0105, 0.0046, 0.041), (-0.0305, 0.0042, 0.03)]
    if True:
        for i, (v, r, tip) in enumerate(fingers):
            arm.append(C("metacarpal-%d-l" % (i + 2), "hand-l", "bone", T, (0.028, v * 0.55, 0), (0.094, v - 0.004, 0), r * 1.05, r))
            arm.append(bone_solid(C("phalanx-proximal-%d-l" % (i + 2), "fingers-l", "bone", T, (0.002, v, 0), (0.043, v, 0), r, r * 0.9), "fingers-l"))
            arm.append(bone_solid(C("phalanx-distal-%d-l" % (i + 2), "fingertips-l", "bone", T, (0.002, v, 0), (tip - 0.004, v * 0.96, 0), r * 0.9, r * 0.75), "fingertips-l"))
        arm.append(bone_solid(C("metacarpal-1-l", "thumb-l", "bone", T, (0.0, 0, -0.004), (0.036, 0, -0.005), 0.0056, 0.0052), "thumb-l"))
        arm.append(bone_solid(C("thumb-proximal-l", "thumb-l", "bone", T, (0.04, 0, -0.005), (0.06, 0, -0.006), 0.005, 0.0046), "thumb-l"))
        arm.append(bone_solid(C("thumb-distal-l", "thumbtip-l", "bone", T, (0.002, 0, -0.006), (0.022, 0, -0.006), 0.0046, 0.004), "thumbtip-l"))
    for x in leg + arm:
        for y in B(x):
            out.append(y if "member" in y else S(y, y["bone"]))
    return out + joints_and_masses(rig, lambda b: not b.startswith("toddler"), s)


def clav_row2(p, rw, rv):
    """📏 A station row of a flat blade on the left clavicle bone from a thorax-frame point; `rw` half thickness front to back is given along v here, `rv` the half width in the blade."""
    u, v, w = clav(p)
    return (u, v, w, rv, rw, 2.0)


BIG = ("pelvis", "lumbar", "thorax", "neck", "head", "upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "toddler-head", "toddler-thigh-l", "toddler-thigh-r", "toddler-shank-l", "toddler-shank-r")
HEAVY = ("pelvis", "lumbar", "thorax", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "toddler-torso", "toddler-head")


def joints_and_masses(rig, keep, s):
    """⚙️ A joint sphere at the pivot of every kept bone that has a parent and a mass sphere at every stated segment centre of mass (radius from the mass), never smaller than the feature of the scale; at 1:500 only the pivots and masses whose limb is thick enough to hold a 75 mm sphere."""
    out = []
    half = {50: 0.0, 100: 0.0075, 200: 0.015, 500: 0.0375}[s]
    small = ("fingers", "fingertips", "thumb", "thumbtip", "toe", "toddler-hand", "scapula")
    for b in rig:
        if not keep(b["id"]):
            continue
        tiny = b["id"].rsplit("-", 1)[0] in small
        if tiny and s > 50:
            continue
        if b["parent"] and b["id"] != "toddler-torso" and (s < 500 or b["id"] in BIG):
            r = max((0.006 if tiny else 0.011 + 0.03 * b["lengthM"]) * (0.7 if b["id"].startswith("toddler") else 1), half)
            out.append(bone_solid(E("joint-" + b["id"], b["id"], "bone", 0.4, (0, 0, 0), (r, r, r)), b["id"], "joint"))
        if "mass" in b and (s < 500 or b["id"] in HEAVY):
            r = max(0.0125 * b["mass"]["kg"] ** (1 / 3), half)
            out.append(bone_solid(E("mass-" + b["id"], b["id"], "bone", 0.2, (b["mass"]["comX"], b["mass"]["comY"], 0), (r, r, r), role="detail"), b["id"], "mass"))
    return out


def thicken(x, half):
    """🖊️ A bone drawn with the pen of its scale: no radius of a capsule, ellipsoid or loft station below half the smallest feature."""
    if x.get("kind") == "capsule":
        x["r0"], x["r1"] = max(x["r0"], half), max(x["r1"], half)
    elif x.get("kind") == "ellipsoid":
        x["radii"] = [max(r, half) for r in x["radii"]]
    elif x.get("kind") == "loft":
        for q in x["sections"]:
            q["rx"], q["ry"] = max(q["rx"], half), max(q["ry"], half)
    return x


def schematic(s, rig):
    """🩻 Both skeletons at 1:200 and 1:500, every member at least as thick as the smallest feature (30 or 75 mm): cranium, spine as three curved columns, rib pairs or one rib barrel, sternum, clavicles, shoulder blades, pelvis, long bones with their ends, hand and foot plates; where a limb is itself thinner than a feature-thick bone, the member is the limb solid by reference."""
    T, h = 0.72, {200: 0.015, 500: 0.0375}[s]
    two = s == 200
    L = stated(s)
    out = []
    def S(x, m):
        return bone_solid(thicken(x, h), m)
    def of(id, body, m):
        return {"id": id, "of": body, "member": m, "part": "member"}
    def pair(x, m=None):
        return [S(y, m.replace("-l", y["id"][-2:]) if m else y["bone"]) for y in B(x)]
    def pair_of(id, body, m):
        return [of(id, body, m), of(id[:-2] + "-r", body[:-2] + "-r", m[:-2] + "-r")]
    out.append(S(E("cranium", "head", "bone", T, (0.058, 0.014, 0), (0.078, 0.09, 0.07)) if two else E("cranium", "head", "bone", T, (0.045, 0.004, 0), (0.09, 0.094, 0.072)), "head"))
    if two:
        out.append(S(L("facial-skeleton", "head", "bone", T, [(0.03, -0.056, 0, 0.058, 0.03), (0.0, -0.062, 0, 0.05, 0.034), (-0.03, -0.062, 0, 0.034, 0.03)], round0=0.006, round1=0.006), "head"))
        out += pair(C("mandible-body-l", "head", "bone", T, (-0.02, -0.01, 0.046), (-0.068, -0.074, 0.012), 0.015), "head")
    out.append(S(C("cervical-spine", "neck", "bone", T, (-0.01, 0, 0), (0.108, -0.014, 0), 0.012), "neck"))
    out.append(S(L("thoracic-spine", "thorax", "bone", T, [(0.0, 0.0, 0, 0.02, 0.0155), (0.1, 0.022, 0, 0.018, 0.014), (0.2, 0.024, 0, 0.016, 0.013), (0.3035, 0.0, 0, 0.014, 0.0115)]), "thorax"))
    out.append(S(L("lumbar-spine", "lumbar", "bone", T, [(0.0, 0.0, 0, 0.0235, 0.017), (0.085, -0.016, 0, 0.0235, 0.017), (0.17, 0.0, 0, 0.0225, 0.0165)]), "lumbar"))
    kyph = lambda t: 0.026 * math.sin(math.pi * t) * (0.85 + 0.3 * t)
    if two:
        for k, hw, sweep in [(3, 0.098, 180), (6, 0.129, 180), (9, 0.136, 146)]:
            t = 1 - (k - 0.5) / 12
            back = (0.3035 * t, kyph(t) + 0.014)
            front = (0.228 - (k - 1) * 0.0235, -0.104 - (k - 1) * 0.0058) if k <= 7 else (0.228 - 6 * 0.0235 - (k - 7) * 0.03, -0.139 + (k - 7) * 0.012)
            out += [bone_solid(ring("rib-%d-r" % k, "thorax", 0.86, back, front, hw, h, sweep, -1), "thorax"), bone_solid(ring("rib-%d-l" % k, "thorax", 0.86, back, front, hw, h, sweep, 1), "thorax")]
        out.append(S(L("sternum", "thorax", "bone", T, [(0.238, -0.1, 0, 0.03, 0.015), (0.14, -0.122, 0, 0.022, 0.015), (0.075, -0.136, 0, 0.017, 0.015)], round1=0.006), "thorax"))
        out += pair(L("scapula-l", "clavicle-l", "bone", T, [clav_row2((0.09, 0.064, 0.086), 0.015, 0.016), clav_row2((0.15, 0.06, 0.102), 0.015, 0.034), clav_row2((0.205, 0.05, 0.116), 0.015, 0.05), clav_row2((0.238, 0.042, 0.112), 0.015, 0.044)], round0=0.004, round1=0.004))
        out.append(S(L("sacrum", "pelvis", "bone", T, [(0.105, 0.044, 0, 0.052, 0.022), (0.065, 0.066, 0, 0.046, 0.018), (0.025, 0.078, 0, 0.03, 0.015)], round1=0.006), "pelvis"))
        out += pair(X("ilium-l", "pelvis", "bone", T, [(0.072, 0.036), (0.088, 0.0614), (-0.042, 0.1434), (-0.058, 0.118)], "yz", 0.0, 0.13), "pelvis")
        out += pair(C("scapular-spine-l", "clavicle-l", "bone", T, clav((0.214, 0.055, 0.085)), clav((0.25, -0.008, 0.176)), 0.015), "scapula-l")
        out += pair(C("ischium-l", "pelvis", "bone", T, (0.012, 0.016, 0.09), (-0.07, 0.034, 0.062), 0.019, 0.015), "pelvis")
        out += pair(C("pubis-l", "pelvis", "bone", T, (-0.006, -0.026, 0.082), (-0.046, -0.066, 0.012), 0.015), "pelvis")
    else:
        out.append(S(E("ribcage", "thorax", "bone", 0.86, (0.15, -0.045, 0), (0.125, 0.1, 0.135)), "thorax"))
        out.append(S(E("pelvic-girdle", "pelvis", "bone", T, (0.04, 0.02, 0), (0.085, 0.08, 0.15)), "pelvis"))
    out += pair(C("iliac-column-l", "hip-bone-l", "bone", T, (0.004, 0, 0), (0.1, 0, 0), 0.017, 0.021))
    out += pair(C("clavicle-l", "clavicle-l", "bone", T, (0.0, 0.0, 0), (0.155, 0.08, 0), 0.0085))
    out += pair(E("glenoid-l", "scapula-l", "bone", T, (0.03, -0.004, -0.018 if two else 0), (0.02, 0.016, 0.008)))
    if two:
        for x in [E("femur-head-l", "thigh-l", "bone", T, (0, 0, 0), (0.0235, 0.0235, 0.0235)), C("femur-neck-l", "thigh-l", "bone", T, (0, 0, 0), (0.03, 0.0, 0.05), 0.016, 0.017), E("femur-trochanter-l", "thigh-l", "bone", T, (0.024, 0.0, 0.062), (0.024, 0.018, 0.015)),
                  L("femur-l", "thigh-l", "bone", T, [(0.035, 0.0, 0.052, 0.017, 0.016), (0.12, 0.008, 0.036, 0.015, 0.015), (0.24, 0.01, 0.018, 0.015, 0.015), (0.34, 0.004, 0.006, 0.0185, 0.0165), (0.378, 0.0, 0.0, 0.034, 0.024)]),
                  E("femur-condyles-l", "thigh-l", "bone", T, (0.392, -0.004, 0), (0.022, 0.031, 0.04)), E("tibia-plateau-l", "shank-l", "bone", T, (0.018, 0.0, 0), (0.016, 0.027, 0.038)),
                  L("tibia-l", "shank-l", "bone", T, [(0.025, 0.002, -0.002, 0.03, 0.023), (0.08, 0.008, -0.004, 0.0165, 0.0175), (0.22, 0.006, -0.004, 0.015, 0.015), (0.36, 0.0, -0.004, 0.015, 0.015), (0.408, 0.0, -0.004, 0.021, 0.018)]),
                  E("talus-l", "foot-l", "bone", T, (0.008, -0.02, 0), (0.028, 0.017, 0.02)), L("calcaneus-l", "foot-l", "bone", T, [(-0.058, -0.052, 0.002, 0.016, 0.019), (-0.03, -0.047, 0.002, 0.019, 0.022), (0.012, -0.046, 0.004, 0.018, 0.017)], round0=0.006),
                  L("metatarsus-l", "foot-l", "bone", T, [(0.028, -0.034, 0, 0.026, 0.017), (0.062, -0.044, 0, 0.032, 0.015), (0.132, -0.07, 0, 0.042, 0.015)]), L("toes-l", "toe-l", "bone", T, [(0.0, 0.018, 0, 0.042, 0.015), (0.056, 0.018, -0.006, 0.034, 0.015)], round1=0.004),
                  E("humerus-head-l", "upper-arm-l", "bone", T, (0.002, 0, -0.004), (0.023, 0.023, 0.023)), L("humerus-l", "upper-arm-l", "bone", T, [(0.02, 0.0, 0.004, 0.017, 0.017), (0.08, 0.0, 0.004, 0.015, 0.015), (0.22, 0.0, 0.002, 0.015, 0.015), (0.29, 0.0, 0.0, 0.019, 0.015), (0.312, 0.0, 0.0, 0.03, 0.015)]),
                  L("radius-ulna-l", "forearm-l", "bone", T, [(-0.014, 0, 0, 0.015, 0.026), (0.12, 0, 0, 0.015, 0.02), (0.245, 0, 0, 0.015, 0.022)]), E("carpus-l", "hand-l", "bone", T, (0.014, 0.0, 0), (0.016, 0.026, 0.015)),
                  L("metacarpus-l", "hand-l", "bone", T, [(0.026, 0, 0, 0.015, 0.026), (0.094, -0.002, 0, 0.015, 0.038)]), L("phalanges-l", "hand-l", "bone", T, [(0.098, -0.002, 0, 0.015, 0.038), (0.15, -0.002, -0.006, 0.015, 0.034), (0.176, -0.003, -0.01, 0.015, 0.024)])]:
            out += pair(x)
    else:
        out += pair(C("femur-l", "thigh-l", "bone", T, (0, 0, 0), (0.4, 0.004, 0), h)) + pair(C("tibia-l", "shank-l", "bone", T, (0, 0.002, 0), (0.4, 0, 0), h)) + pair(C("humerus-l", "upper-arm-l", "bone", T, (0.0, 0, 0), (0.31, 0, 0), h))
        out += pair_of("foot-bones-l", "boot-l", "foot-l") + pair_of("toe-bones-l", "boot-toe-l", "toe-l") + pair_of("forearm-bones-l", "sleeve-fore-l", "forearm-l") + pair_of("hand-bones-l", "glove-l", "hand-l")
    T = 0.78
    out.append(S(E("toddler-cranium", "toddler-head", "bone", T, (0.078, 0.01, 0), (0.07, 0.08, 0.063)), "toddler-head"))
    if two:
        out.append(S(L("toddler-facial-skeleton", "toddler-head", "bone", T, [(0.03, -0.046, 0, 0.044, 0.024), (0.0, -0.048, 0, 0.036, 0.024), (-0.018, -0.046, 0, 0.026, 0.02)], round0=0.005, round1=0.005), "toddler-head"))
        out.append(S(C("toddler-cervical-spine", "toddler-neck", "bone", T, (-0.006, 0, 0), (0.05, -0.004, 0), 0.008), "toddler-neck"))
        out.append(S(L("toddler-spine", "toddler-torso", "bone", T, [(0.03, 0, 0, 0.015, 0.015), (0.15, 0.012, 0, 0.015, 0.015), (0.28, 0, 0, 0.015, 0.015)]), "toddler-torso"))
        out.append(S(L("toddler-sacrum", "toddler-torso", "bone", T, [(0.035, 0.012, 0, 0.026, 0.015), (0.0, 0.026, 0, 0.018, 0.015)], round1=0.004), "toddler-torso"))
        bow = lambda t: 0.012 * math.sin(math.pi * t)
        for k, hw, sweep in [(3, 0.054, 180), (7, 0.073, 180)]:
            t = 1 - (k - 0.5) / 12
            back = (0.115 + 0.165 * t, bow(0.4 + 0.6 * t) + 0.008)
            front = (0.232 - (k - 1) * 0.0125, -0.058 - (k - 1) * 0.0042)
            out += [bone_solid(ring("toddler-rib-%d-r" % k, "toddler-torso", 0.9, back, front, hw, h, sweep, -1), "toddler-torso"), bone_solid(ring("toddler-rib-%d-l" % k, "toddler-torso", 0.9, back, front, hw, h, sweep, 1), "toddler-torso")]
        out.append(S(L("toddler-sternum", "toddler-torso", "bone", T, [(0.238, -0.056, 0, 0.015, 0.015), (0.15, -0.078, 0, 0.015, 0.015)], round1=0.004), "toddler-torso"))
        out += pair(E("toddler-ilium-l", "toddler-torso", "bone", T, (0.04, 0.008, 0.04), (0.036, 0.03, 0.015)), "toddler-torso")
        out += pair(C("toddler-clavicle-l", "toddler-torso", "bone", T, (0.242, -0.05, 0.012), (0.258, -0.012, 0.082), 0.015), "toddler-torso")
        for x in [E("toddler-femur-head-l", "toddler-thigh-l", "bone", T, (0, 0, 0), (0.015, 0.015, 0.015)), L("toddler-femur-l", "toddler-thigh-l", "bone", T, [(0.004, 0, 0.012, 0.015, 0.015), (0.1, 0.004, 0.008, 0.015, 0.015), (0.176, 0, 0, 0.02, 0.0155)]),
                  L("toddler-tibia-l", "toddler-shank-l", "bone", T, [(0.004, 0, 0, 0.019, 0.015), (0.11, 0.002, -0.003, 0.015, 0.015), (0.164, 0, -0.003, 0.015, 0.015)]),
                  L("toddler-tarsus-l", "toddler-foot-l", "bone", T, [(-0.028, -0.016, 0, 0.015, 0.015), (0.012, -0.014, 0, 0.015, 0.015), (0.09, -0.02, 0, 0.022, 0.015)], round0=0.004, round1=0.003),
                  L("toddler-humerus-l", "toddler-arm-l", "bone", T, [(0.004, 0, 0, 0.015, 0.015), (0.132, 0, 0, 0.015, 0.015)]), C("toddler-radius-ulna-l", "toddler-forearm-l", "bone", T, (-0.004, 0, 0), (0.106, 0, 0), 0.015),
                  L("toddler-hand-bones-l", "toddler-hand-l", "bone", T, [(0.002, 0, 0, 0.015, 0.016), (0.03, 0.001, 0, 0.015, 0.021), (0.062, 0, 0, 0.015, 0.018)], round1=0.003)]:
            out += pair(x)
    else:
        out.append(S(E("toddler-ribcage", "toddler-torso", "bone", 0.9, (0.17, -0.008, 0), (0.075, 0.07, 0.08)), "toddler-torso"))
        out.append(S(E("toddler-pelvis", "toddler-torso", "bone", T, (0.02, 0.0, 0), (0.045, 0.06, 0.085)), "toddler-torso"))
        out.append(S(C("toddler-spine", "toddler-torso", "bone", T, (0.0, 0.012, 0), (0.28, 0.0, 0), h), "toddler-torso"))
        out.append(of("toddler-cervical-spine", "toddler-neck", "toddler-neck"))
        out += pair(C("toddler-femur-l", "toddler-thigh-l", "bone", T, (0, 0, 0), (0.176, 0, 0), h))
        out += pair_of("toddler-shank-bones-l", "toddler-shank-l", "toddler-shank-l") + pair_of("toddler-foot-bones-l", "toddler-boot-l", "toddler-foot-l") + pair_of("toddler-arm-bones-l", "toddler-sleeve-l", "toddler-arm-l") + pair_of("toddler-forearm-bones-l", "toddler-forearm-l", "toddler-forearm-l") + pair_of("toddler-hand-bones-l", "toddler-mitten-l", "toddler-hand-l")
    r = max(0.032, h)
    out.append(bone_solid(E("joint-toddler-seat", "forearm-r", "bone", 0.4, (0.2, 0.04, 0.02), (r, r, 0.05)), "toddler-torso", "joint"))
    return out + joints_and_masses(rig, lambda b: True, s)


def toddler_structure(s, rig):
    """🩻 The toddler's skeleton at scale 1:s: a large cranium over a small face and mandible, a gently curved spine, a round ribcage on the sternum, clavicles and scapulae, a small pelvis, short long bones with thick cartilaginous ends, hands and feet."""
    T = 0.78
    out = []
    L = stated(s)
    S = bone_solid
    out += [S(E("toddler-cranium", "toddler-head", "bone", T, (0.078, 0.01, 0), (0.07, 0.08, 0.063)), "toddler-head"), S(L("toddler-facial-skeleton", "toddler-head", "bone", T, [(0.03, -0.05, 0, 0.044, 0.024), (0.0, -0.052, 0, 0.036, 0.024), (-0.018, -0.05, 0, 0.026, 0.02)], step=0.02, round0=0.005, round1=0.005), "toddler-head")]
    out += [S(x, "toddler-head") for x in B(C("toddler-mandible-l", "toddler-head", "bone", T, (0.002, -0.004, 0.04), (-0.036, -0.058, 0.008), 0.0065, 0.0075))]
    bow = lambda t: 0.012 * math.sin(math.pi * t)
    out += spine("toddler-", "toddler-neck", 4, -0.006, 0.0135, lambda t: -0.004 * t, lambda t: (0.0075, 0.0065, 0.009), 0, T, ["c7", "c5", "c3", "c1"])
    out += spine("toddler-", "toddler-torso", 9, 0.03, 0.0278, bow, lambda t: (0.0147, 0.01, 0.014 - 0.003 * t), 0.012 if s == 50 else 0, T, ["l5", "l3", "l1", "t11", "t9", "t7", "t5", "t3", "t1"])
    out += [S(L("toddler-sacrum", "toddler-torso", "bone", T, [(0.035, 0.012, 0, 0.026, 0.012), (0.0, 0.03, 0, 0.018, 0.008), (-0.02, 0.034, 0, 0.008, 0.006)], smooth=False, round1=0.004), "toddler-torso")]
    ribs = [(1, 0.034, 180), (3, 0.054, 180), (5, 0.066, 180), (7, 0.073, 180), (9, 0.074, 150), (11, 0.068, 84)]
    chosen = ribs if s <= 100 else [x for x in ribs if x[0] in ((3, 7, 11) if s == 200 else (3, 7))]
    for k, hw, sweep in chosen:
        t = 1 - (k - 0.5) / 12
        back = (0.115 + 0.165 * t, bow(0.4 + 0.6 * t) + 0.008)
        front = (0.232 - (k - 1) * 0.0125, -0.058 - (k - 1) * 0.0042) if k <= 7 else (0.157 - (k - 7) * 0.016, -0.083 + (k - 7) * 0.008)
        rr = 0.004 if s == 50 else 0.0075
        out += [S(ring("toddler-rib-%d-r" % k, "toddler-torso", 0.9, back, front, hw, rr, sweep, -1), "toddler-torso"), S(ring("toddler-rib-%d-l" % k, "toddler-torso", 0.9, back, front, hw, rr, sweep, 1), "toddler-torso")]
    out += [S(L("toddler-sternum", "toddler-torso", "bone", T, [(0.238, -0.058, 0, 0.014, 0.005), (0.19, -0.072, 0, 0.011, 0.0045), (0.15, -0.084, 0, 0.007, 0.004)], smooth=False, round1=0.004), "toddler-torso")]
    out += [S(x, "toddler-torso") for x in B(C("toddler-clavicle-l", "toddler-torso", "bone", T, (0.242, -0.056, 0.008), (0.262, -0.012, 0.086), 0.0048, 0.0045))]
    out += [S(x, "toddler-torso") for x in B(L("toddler-scapula-l", "toddler-torso", "bone", T, [(0.17, 0.05, 0.046, 0.008, 0.005), (0.215, 0.046, 0.058, 0.024, 0.005), (0.25, 0.038, 0.062, 0.026, 0.005)], smooth=False, round0=0.003, round1=0.003))]
    out += [S(x, "toddler-torso") for x in B(X("toddler-ilium-l", "toddler-torso", "bone", T, [(0.032, 0.016), (0.039, 0.03), (-0.033, 0.066), (-0.04, 0.052)], "yz", 0.0, 0.07))]
    out += [S(x, "toddler-torso") for x in B(C("toddler-acromion-l", "toddler-torso", "bone", T, (0.25, 0.036, 0.066), (0.262, -0.008, 0.086), 0.0048))]
    out.append(bone_solid(E("joint-toddler-seat", "forearm-r", "bone", 0.4, (0.2, 0.038, 0.02), (0.03, 0.032, 0.05)), "toddler-torso", "joint"))
    out += [S(x, "toddler-torso") for x in B(C("toddler-pubis-l", "toddler-torso", "bone", T, (-0.004, -0.012, 0.046), (-0.022, -0.034, 0.005), 0.0062))]
    out += [S(x, "toddler-torso") for x in B(C("toddler-ischium-l", "toddler-torso", "bone", T, (0.004, 0.008, 0.05), (-0.034, 0.016, 0.034), 0.009, 0.0075))]
    limbs = [
        E("toddler-femur-head-l", "toddler-thigh-l", "bone", T, (0, 0, 0), (0.0125, 0.0125, 0.0125)),
        L("toddler-femur-l", "toddler-thigh-l", "bone", T, [(0.004, 0, 0.012, 0.011, 0.0105), (0.03, 0.002, 0.018, 0.0085, 0.0085), (0.1, 0.004, 0.008, 0.0078, 0.0078), (0.155, 0.002, 0.002, 0.012, 0.0105), (0.176, 0, 0, 0.02, 0.0155)], step=0.02),
        L("toddler-tibia-l", "toddler-shank-l", "bone", T, [(0.004, 0, 0, 0.019, 0.015), (0.03, 0.003, -0.002, 0.01, 0.0105), (0.11, 0.002, -0.003, 0.0072, 0.0078), (0.164, 0, -0.003, 0.012, 0.0105)], step=0.02),
        C("toddler-fibula-l", "toddler-shank-l", "bone", T, (0.014, -0.006, 0.017), (0.168, -0.004, 0.015), 0.0042, 0.0045),
        L("toddler-tarsus-l", "toddler-foot-l", "bone", T, [(-0.03, -0.02, 0, 0.012, 0.012), (0.012, -0.014, 0, 0.015, 0.013), (0.04, -0.02, 0, 0.018, 0.008)], smooth=False, round0=0.004),
        L("toddler-metatarsus-l", "toddler-foot-l", "bone", T, [(0.04, -0.02, 0, 0.018, 0.007), (0.074, -0.028, 0, 0.022, 0.0055), (0.098, -0.03, 0, 0.02, 0.0045)], smooth=False, round1=0.003),
        E("toddler-humerus-head-l", "toddler-arm-l", "bone", T, (0.002, 0, 0), (0.011, 0.011, 0.011)),
        L("toddler-humerus-l", "toddler-arm-l", "bone", T, [(0.008, 0, 0, 0.009, 0.009), (0.04, 0, 0, 0.0066, 0.0066), (0.1, 0, 0, 0.0064, 0.0064), (0.132, 0, 0, 0.014, 0.008)], step=0.02),
        C("toddler-ulna-l", "toddler-forearm-l", "bone", T, (-0.006, -0.006, 0), (0.106, -0.008, 0), 0.0058, 0.0042),
        C("toddler-radius-l", "toddler-forearm-l", "bone", T, (0.006, 0.006, 0.003), (0.108, 0.006, 0), 0.0042, 0.0056),
        L("toddler-hand-bones-l", "toddler-hand-l", "bone", T, [(0.002, 0, 0, 0.0055, 0.014), (0.03, 0.001, 0, 0.005, 0.021), (0.066, 0, 0, 0.004, 0.018)], smooth=False, round1=0.003),
    ]
    for x in limbs:
        out += [S(y, y["bone"]) for y in B(x)]
    return out + joints_and_masses(rig, lambda b: b.startswith("toddler"), s)


DECISIONS = {
    50: [
        ("Body: both figures are lofts through stated anatomical sections on a rig with a lumbar and a thoracic spine bone, clavicles, scapulae and finger and thumb chains; the parent's coat has a yoke on the clavicles, set-in sleeves with rounded shoulder and elbow ends and a skirt that follows each thigh by half, the gloved fingers are separate and close around the child's thigh.",
         "Body: Beide Figuren sind Lofts durch festgelegte anatomische Querschnitte auf einem Rig mit Lenden- und Brustwirbelsäule, Schlüsselbeinen, Schulterblättern sowie Finger- und Daumenketten; der Mantel hat eine Passe auf den Schlüsselbeinen, eingesetzte Ärmel mit gerundeten Schulter- und Ellbogenenden und einen Schoß, der jedem Oberschenkel zur Hälfte folgt, die behandschuhten Finger sind einzeln und schließen sich um den Oberschenkel des Kindes."),
        ("Toddler: the 24-month child keeps its own proportions, a head of 0.205 m on 0.865 m of stature with a small low face, almost no neck, a barrel trunk with a belly and a nappy seat, short thick limbs in a padded overall, mittens with thumbs and rubber boots.",
         "Kleinkind: Das 24 Monate alte Kind behält seine eigenen Proportionen, ein Kopf von 0,205 m bei 0,865 m Körperhöhe mit kleinem, tief sitzendem Gesicht, fast kein Hals, ein tonnenförmiger Rumpf mit Bauch und Windelpo, kurze kräftige Gliedmaßen im gefütterten Overall, Fäustlinge mit Daumen und Gummistiefel."),
        ("Thermal: every garment and skin surface is its region; at this scale the inner layers (air, sweater, sock) are nested shells inside the outer one, and the breath and plume emitters are small solids at nose, mouth and crown of both figures.",
         "Thermal: Jede Kleidungs- und Hautfläche ist ihre Region; in diesem Maßstab liegen die inneren Schichten (Luft, Pullover, Socke) als Schalen in der äußeren, und die Atem- und Auftriebsquellen sind kleine Körper an Nase, Mund und Scheitel beider Figuren."),
        ("Structure: two anatomical skeletons, each vertebra with its spinous process on the cervical, thoracic and lumbar curves, twelve rib pairs as arcs from the spine to the sternum or the costal arch, clavicle, scapula with spine and glenoid, pelvis with sacrum, ilium, ischium and pubis, long bones with their epiphyses, carpus, metacarpals and phalanges, tarsus, metatarsals and toes; the toddler's weight enters the parent through the near forearm.",
         "Struktur: Zwei anatomische Skelette, jeder Wirbel mit Dornfortsatz auf der Hals-, Brust- und Lendenkrümmung, zwölf Rippenpaare als Bögen von der Wirbelsäule zum Brustbein oder Rippenbogen, Schlüsselbein, Schulterblatt mit Gräte und Gelenkpfanne, Becken mit Kreuzbein, Darmbein, Sitzbein und Schambein, Röhrenknochen mit ihren Epiphysen, Handwurzel, Mittelhand und Fingerglieder, Fußwurzel, Mittelfuß und Zehen; das Gewicht des Kleinkinds geht über den nahen Unterarm in das Elternteil."),
    ],
    100: [
        ("Body: the same sections with fewer stations; the four fingers are one slab per phalanx row so the hand still closes, the face keeps nose, ears and the toddler's cheeks, eyes, lips, lapels and pockets are left out because they fall below 15 mm.",
         "Body: Dieselben Querschnitte mit weniger Stationen; die vier Finger sind je Gliedreihe eine Platte, sodass die Hand weiter greift, das Gesicht behält Nase, Ohren und die Wangen des Kleinkinds, Augen, Lippen, Revers und Taschen entfallen, weil sie unter 15 mm liegen."),
        ("Thermal: one solid per surface and region without inner shells; the emitters stay.", "Thermal: Ein Körper je Fläche und Region ohne innere Schalen; die Quellen bleiben."),
        ("Structure: vertebrae without processes, every second rib pair, the hand and foot rays kept, the toddler's spine as one curved column.", "Struktur: Wirbel ohne Fortsätze, jedes zweite Rippenpaar, Hand- und Fußstrahlen bleiben, die Wirbelsäule des Kleinkinds als eine gekrümmte Säule."),
    ],
    200: [
        ("Body: heads without facial features, hands as one mitt form on the hand bone, boots modelled with their sole in one form, the scarf end thickened to 34 mm; proportions and bones are those of 1:50.",
         "Body: Köpfe ohne Gesichtszüge, Hände als eine Fäustlingsform auf dem Handknochen, Stiefel mit Sohle in einer Form, das Schalende auf 34 mm verstärkt; Proportionen und Knochen sind die von 1:50."),
        ("Thermal: one solid per surface and region; the emitters stay.", "Thermal: Ein Körper je Fläche und Region; die Quellen bleiben."),
        ("Structure: spine as three curved columns, four rib pairs, sternum, girdles and long bones with epiphyses, hands and feet as plates.", "Struktur: Wirbelsäule als drei gekrümmte Säulen, vier Rippenpaare, Brustbein, Gürtel und Röhrenknochen mit Epiphysen, Hände und Füße als Platten."),
    ],
    500: [
        ("Body: the two figures reduced to their masses, head with hat, trunk, yoke, limbs, mitt hands and boots, the toddler still with its large head, belly and short limbs; hat cuffs, scarf end and collar are left out below 75 mm.",
         "Body: Die beiden Figuren auf ihre Massen reduziert, Kopf mit Mütze, Rumpf, Passe, Gliedmaßen, Fäustlingshände und Stiefel, das Kleinkind weiter mit großem Kopf, Bauch und kurzen Gliedmaßen; Mützenränder, Schalende und Kragen entfallen unter 75 mm."),
        ("Thermal: one solid per surface and region; the emitters stay.", "Thermal: Ein Körper je Fläche und Region; die Quellen bleiben."),
        ("Structure: the skeleton with three rib pairs and the spine as three columns; no bone is left out of the load path.", "Struktur: Das Skelett mit drei Rippenpaaren und der Wirbelsäule als drei Säulen; kein Knochen fehlt im Lastpfad."),
    ],
}


def main():
    """▶️ Builds the four scales and writes them into the candidate document and the volumes source."""
    doc = kit.read_json(CANDIDATE)
    a = doc["asset"]
    rig = a["rig"]["bones"]
    volumes = {}
    for s in (50, 100, 200, 500):
        body = parent_body(s) + toddler_body(s)
        th = thermal(body, a["thermal"]["regions"], s, {e["id"]: e["diameterM"] for e in a["thermal"]["emitters"]})
        structure = parent_structure(s, rig) + toddler_structure(s, rig) if s <= 100 else schematic(s, rig)
        if s == 100:
            structure = [x if x.get("kind") == "revolve" else thicken(x, 0.0075) for x in structure]
        for b in body:
            b.pop("_r")
        ids = [x["id"] for x in body + th + structure]
        dup = {i for i in ids if ids.count(i) > 1}
        if dup:
            raise SystemExit("duplicate ids %s" % sorted(dup)[:8])
        volumes[str(s)] = {"status": "authored", "denominator": s, "body": body, "thermal": th, "structure": structure, "decisions": [kit.text(en, de) for en, de in DECISIONS[s]]}
        print("[DEBUG] 1:%d body %d thermal %d structure %d" % (s, len(body), len(th), len(structure)))
    a["volumes"] = volumes
    kit.write_json(CANDIDATE, doc)
    if "--publish" in __import__("sys").argv:
        kit.write_json(SOURCE, {"volumes": volumes})


if __name__ == "__main__":
    main()
