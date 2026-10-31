"""NW-H1 person-on-crutches: four individually drawn representations."""
import copy
from crutches_phys import *
from figure import *
from docbuild import D

HAIR_M = [(0.30, 0.74), (0.18, 0.86), (0.0, 0.93), (-0.2, 0.89), (-0.34, 0.74), (-0.37, 0.52), (-0.32, 0.40), (-0.25, 0.52), (-0.1, 0.60), (0.08, 0.66), (0.25, 0.67)]
HAIR_C = [(0.30, 0.74), (0.0, 0.94), (-0.25, 0.88), (-0.37, 0.55), (-0.30, 0.42), (-0.05, 0.64), (0.25, 0.67)]

OUT = dict(
    reg=dict(skin="face-skin", hair="hair", hat=None, hands="hands", upper="hoodie", lower="legs-skin", shoes="sneaker", boot="boot", skirt="shorts"),
    tone=dict(skin=0.80, hair=0.15, hat=0.3, upper=0.42, upper_far=0.34, lower=0.76, lower_far=0.66, shoes=0.85, collar=0.38, boot=0.18, skirt=0.30, skirt_far=0.24),
    torso=[(-0.12, .112, .102), (0.0, .106, .106), (0.25, .100, .108), (0.5, .108, .122), (0.8, .116, .122), (0.93, .102, .100), (1.0, .072, .070)],
    hip=[(-1.0, .112, .106), (0.0, .110, .106), (1.0, .106, .102), (1.3, .104, .100)],
    skirt=[(-0.1, .102, .097), (0.2, .098, .092), (0.5, .096, .090), (0.58, .098, .092)],
    sleeve_up=[(-0.10, .056, .054), (0.4, .054, .052), (0.8, .050, .050), (1.08, .047, .047)],
    sleeve_fore=[(-0.12, .048, .048), (0.4, .045, .046), (0.8, .043, .043), (0.93, .044, .044), (1.0, .044, .044)],
    thigh=[(-0.02, .082, .074), (0.5, .076, .071), (0.85, .062, .058), (1.06, .054, .050)],
    shank=[(-0.06, .054, .050), (0.2, .058, .050), (0.5, .054, .046), (0.8, .040, .038), (0.97, .038, .036)],
    collar=[(-0.6, .082, .062), (0.2, .086, .062), (0.7, .072, .052)],
    hat_pts=None, hat_c=None, hair_pts=HAIR_M, shoe="sneaker", boot_side="l", boot_shoe="boot", hatch="knit", far_hatch="far-side", glasses=False, nub=False, buttons=False, cuff=None,
    boot_shaft=[(0.28, .066, .070), (0.5, .074, .080), (0.8, .066, .068), (1.02, .060, .058)],
    boot_shaft500=[(0.28, .066, .070), (0.6, .074, .078), (1.02, .060, .058)],
    thigh500=[(-0.02, .084, .078), (0.5, .076, .072), (1.05, .056, .052)], shank500=[(-0.05, .056, .052), (0.5, .054, .046), (1.0, .042, .040)],
    skirt500=[(-0.1, .102, .097), (0.58, .098, .092)], torso500=[(-0.12, .112, .102), (0.5, .108, .122), (0.93, .102, .100), (1.0, .072, .070)], hip500=[(-1.0, .112, .106), (1.3, .104, .100)],
    pen_arm500=0.17, pen_hand500=0.14, skirt_hatch=None)

HATCH50 = [{"id": "knit", "title": tx("Fleece knit", "Fleecegewebe"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.8, "penMm": 0.1},
           {"id": "far-side", "title": tx("Far side", "Abgewandte Seite"), "pattern": "lines", "angleDeg": 60, "spacingMm": 0.6, "penMm": 0.1}]


def W2(bone, lx, ly):
    return REST[bone].ap(lx, ly)


def crutch_parts(sh, s, level, near=True):
    A = sh.add
    cb = "crutch-" + s
    fb = "forearm-" + s
    z0 = 9.0 if near else 0.8
    tone = 0.62 if near else 0.5
    o = REST[cb].ap(0, 0)
    tip = REST[cb].ap(L_C, 0)
    up = lambda d: REST[cb].ap(-d, 0)
    if level == 500:
        sh.add("crutch-tube-" + s, cb, sh.poly(cb, [REST[cb].ap(-0.04, 0), tip], False), "body", z0, False, 0.5, regions=["crutch-frame", "crutch-grips", "crutch-tips"], pen=0.10)
        sh.add("crutch-cuff-" + s, fb, sh.poly(fb, [W2(fb, 0.05, 0.0), W2(fb, 0.10, 0.0)], False), "body", z0 + 0.2, False, 0.2, region="crutch-cuffs", pen=0.14)
        return
    if level == 200:
        sh.add("crutch-tube-" + s, cb, sh.poly(cb, [REST[cb].ap(-0.03, 0), REST[cb].ap(L_C - 0.05, 0)], False), "body", z0, False, 0.5, region="crutch-frame", pen=0.11)
        sh.add("crutch-tip-" + s, cb, sh.poly(cb, [REST[cb].ap(L_C - 0.05, 0), tip], False), "body", z0 + 0.1, False, 0.2, region="crutch-tips", pen=0.17)
        sh.add("crutch-grip-" + s, cb, sh.poly(cb, [W2(cb, 0.0, -0.045), W2(cb, 0.0, 0.055)], False), "body", z0 + 0.2, False, 0.2, region="crutch-grips", pen=0.16)
        sh.add("crutch-cuff-" + s, fb, sh.poly(fb, [W2(fb, 0.05, 0.0), W2(fb, 0.10, 0.0)], False), "body", z0 + 0.2, False, 0.2, region="crutch-cuffs", pen=0.2)
        sh.add("crutch-bracket-" + s, fb, sh.poly(fb, [W2(fb, 0.08, -0.058), W2(fb, 0.27, -0.045)], False), "body", z0 + 0.1, False, 0.5, region="crutch-frame", pen=0.11)
        return
    A("crutch-tube-" + s, cb, sh.capsule(cb, REST[cb].ap(-0.03, 0), REST[cb].ap(L_C - 0.045, 0), 0.011, 0.011), "body", z0, True, tone, region="crutch-frame", pen=0.15 if level == 50 else 0.1)
    A("crutch-tip-" + s, cb, sh.capsule(cb, REST[cb].ap(L_C - 0.06, 0), tip, 0.016, 0.013), "body", z0 + 0.1, True, 0.1, region="crutch-tips", pen=0.15 if level == 50 else 0.1)
    A("crutch-grip-" + s, cb, sh.capsule(cb, W2(cb, 0.0, -0.05), W2(cb, 0.0, 0.06), 0.015, 0.015), "body", z0 + 0.2, True, 0.12, region="crutch-grips", pen=0.15 if level == 50 else 0.1)
    A("crutch-bracket-" + s, fb, sh.capsule(fb, W2(fb, 0.085, -0.060), W2(fb, 0.27, -0.045), 0.010, 0.010), "body", z0 + 0.1, True, tone, region="crutch-frame", pen=0.15 if level == 50 else 0.1)
    A("crutch-cuff-" + s, fb, sh.path(fb, [W2(fb, 0.035, -0.058, ), W2(fb, 0.035, 0.056), W2(fb, 0.115, 0.056), W2(fb, 0.115, -0.058)], True), "body", z0 + 0.25, True, 0.15, region="crutch-cuffs")
    if level == 50:
        A("crutch-cuff-gap-" + s, fb, sh.poly(fb, [W2(fb, 0.075, 0.056), W2(fb, 0.075, 0.020)], False), "detail", z0 + 0.3, False, 0.8, region="crutch-cuffs")
        A("crutch-adjust-" + s, cb, sh.poly(cb, [W2(cb, 0.50, -0.012), W2(cb, 0.50, 0.012)], False), "detail", z0 + 0.3, False, 0.9, region="crutch-frame")
        for i, d in enumerate((0.55, 0.60, 0.65)):
            A("crutch-hole-%d-%s" % (i, s), cb, sh.ellipse(cb, W2(cb, d, 0.0), 0.0035, 0.0035), "detail", z0 + 0.3, True, 0.1, region="crutch-frame")


def build(sh, level):
    dress(sh, h, rig, REST, OUT, level)
    if level >= 200:
        crutch_parts(sh, "r", level)
    else:
        crutch_parts(sh, "r", level)
        crutch_parts(sh, "l", level, near=False)


PENS = {"500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.13},
        "200": {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18},
        "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.25},
        "50": {"body": 0.25, "detail": 0.15, "texture": 0.1, "glazing": None, "ground": 0.25}}
RANGES = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}
HATCHES = {"500": [], "200": [], "100": [dict(HATCH50[0], spacingMm=0.6), dict(HATCH50[1], spacingMm=0.5)], "50": HATCH50}


def reps(DECS):
    out = {}
    for k in ("500", "200", "100", "50"):
        sh = Shapes(rig)
        build(sh, int(k))
        out[k] = dict(range=RANGES[k], pens=PENS[k], hatches=HATCHES[k], shapes=sh.items, decisions=DECS[k])
    return out
