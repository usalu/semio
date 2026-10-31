"""NW-H1 person-with-white-cane: four individually drawn representations."""
import copy
from cane_phys import *
from figure import *
from docbuild import D

BOB = [(0.30, 0.80), (0.12, 0.90), (-0.08, 0.92), (-0.28, 0.84), (-0.40, 0.62), (-0.42, 0.32), (-0.36, 0.14), (-0.22, 0.12), (-0.18, 0.28), (-0.20, 0.50), (-0.05, 0.66), (0.12, 0.70), (0.28, 0.70)]
BOB_C = [(0.30, 0.78), (0.0, 0.94), (-0.30, 0.84), (-0.42, 0.40), (-0.30, 0.14), (-0.18, 0.40), (-0.05, 0.66), (0.28, 0.70)]

OUT = dict(
    reg=dict(skin="face-skin", hair="hair", hat=None, hands="hands", upper="jacket", lower="jeans", shoes="boots", boot="boots", skirt="jacket"),
    tone=dict(skin=0.82, hair=0.28, hat=0.3, upper=0.50, upper_far=0.40, lower=0.30, lower_far=0.24, shoes=0.14, collar=0.46, boot=0.14),
    torso=[(-0.12, .108, .100), (0.0, .102, .102), (0.25, .094, .102), (0.5, .100, .122), (0.8, .106, .116), (0.93, .094, .092), (1.0, .066, .064)],
    hip=[(-1.0, .108, .104), (0.0, .108, .104), (1.0, .106, .102), (1.6, .100, .098)],
    skirt=None,
    sleeve_up=[(-0.10, .050, .048), (0.4, .048, .047), (1.08, .042, .042)],
    sleeve_fore=[(-0.12, .042, .042), (0.4, .040, .041), (0.93, .038, .038), (1.0, .040, .040)],
    thigh=[(-0.05, .082, .074), (0.5, .072, .068), (1.06, .052, .050)],
    shank=[(-0.06, .054, .050), (0.2, .058, .050), (0.5, .052, .044), (0.85, .040, .038), (0.97, .042, .040)],
    collar=[(-0.6, .066, .068), (0.1, .066, .068), (0.6, .054, .060)],
    hat_pts=None, hat_c=None, hair_pts=BOB, shoe="boot", boot_side=None, boot_shoe="boot", hatch="denim", far_hatch="far-side", glasses=False, nub=False, buttons=True, cuff=0.30,
    thigh500=[(-0.05, .084, .078), (0.5, .074, .070), (1.05, .056, .052)], shank500=[(-0.05, .056, .052), (0.5, .054, .046), (1.0, .044, .042)],
    skirt500=None, torso500=[(-0.12, .108, .100), (0.5, .100, .122), (0.93, .094, .092), (1.0, .066, .064)], hip500=[(-1.0, .108, .104), (1.6, .100, .098)],
    pen_arm500=0.16, pen_hand500=0.14, skirt_hatch=None)
HATCH50 = [{"id": "denim", "title": tx("Denim twill", "Jeansköper"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1},
           {"id": "far-side", "title": tx("Far side", "Abgewandte Seite"), "pattern": "lines", "angleDeg": 60, "spacingMm": 0.6, "penMm": 0.1}]


def W2(bone, lx, ly):
    return REST[bone].ap(lx, ly)


def hp(u, v):
    return h.head_pt(u, v)


def extras(sh, level):
    A = sh.add
    HD = h.hd
    cb = "cane"
    tip = REST[cb].ap(L_CANE, 0)
    if level == 500:
        A("sunglasses", "head", sh.poly("head", [hp(0.20, 0.50), hp(0.40, 0.50)], False), "body", 6.8, False, 0.1, region="sunglasses", pen=0.1)
        A("bag", "pelvis", sh.path("pelvis", [W2("pelvis", -0.12, -0.01), W2("pelvis", -0.12, 0.15), W2("pelvis", -0.01, 0.15), W2("pelvis", -0.01, -0.01)], True), "body", 7.5, True, 0.35, region="bag")
        A("cane-shaft", cb, sh.poly(cb, [REST[cb].ap(-0.08, 0), REST[cb].ap(L_CANE - 0.03, 0)], False), "body", 9.0, False, 0.9, regions=["cane-shaft", "cane-grip", "cane-tip"], pen=0.1)
        return
    if level == 200:
        A("sunglasses", "head", sh.ellipse("head", hp(0.28, 0.50), 0.14 * HD, 0.08 * HD), "detail", 6.8, True, 0.05, region="sunglasses")
        A("bag", "pelvis", sh.path("pelvis", [W2("pelvis", -0.13, -0.01), W2("pelvis", -0.13, 0.15), W2("pelvis", 0.0, 0.15), W2("pelvis", 0.0, -0.01)], True), "body", 7.5, True, 0.35, region="bag")
        A("cane-shaft", cb, sh.poly(cb, [REST[cb].ap(0.08, 0), REST[cb].ap(L_CANE - 0.05, 0)], False), "body", 9.0, False, 0.9, region="cane-shaft", pen=0.1)
        A("cane-grip", cb, sh.poly(cb, [REST[cb].ap(-0.09, 0), REST[cb].ap(0.09, 0)], False), "body", 9.2, False, 0.2, region="cane-grip", pen=0.16)
        A("cane-tip", cb, sh.poly(cb, [REST[cb].ap(L_CANE - 0.05, 0), tip], False), "body", 9.2, False, 0.2, region="cane-tip", pen=0.14)
        return
    if level == 100:
        A("sunglasses", "head", sh.path("head", [hp(0.20, 0.55), hp(0.40, 0.55), hp(0.40, 0.45), hp(0.22, 0.43)], True), "detail", 6.8, True, 0.05, region="sunglasses")
        A("bag", "pelvis", sh.path("pelvis", [W2("pelvis", -0.13, -0.01), W2("pelvis", -0.13, 0.15), W2("pelvis", 0.0, 0.15), W2("pelvis", 0.0, -0.01)], True), "body", 7.5, True, 0.35, region="bag")
        A("bag-strap", "torso", sh.poly("torso", [W2("torso", h.Lt * 0.9, 0.06), W2("pelvis", -0.07, 0.10)], False), "detail", 7.4, False, 0.3, region="bag")
        A("cane-shaft", cb, sh.poly(cb, [REST[cb].ap(0.09, 0), REST[cb].ap(L_CANE - 0.07, 0)], False), "body", 9.0, False, 0.9, region="cane-shaft", pen=0.12)
        A("cane-band", cb, sh.poly(cb, [REST[cb].ap(L_CANE - 0.32, 0), REST[cb].ap(L_CANE - 0.07, 0)], False), "body", 9.1, False, 0.3, region="cane-shaft", pen=0.12)
        A("cane-grip", cb, sh.capsule(cb, REST[cb].ap(-0.10, 0), REST[cb].ap(0.10, 0), 0.014, 0.014), "body", 9.2, True, 0.12, region="cane-grip", pen=0.1)
        A("cane-tip", cb, sh.capsule(cb, REST[cb].ap(L_CANE - 0.07, 0), REST[cb].ap(L_CANE - 0.018, 0), 0.010, 0.018), "body", 9.2, True, 0.7, region="cane-tip", pen=0.1)
        return
    A("sunglasses", "head", sh.path("head", [hp(0.20, 0.56), hp(0.41, 0.56), hp(0.41, 0.45), hp(0.30, 0.42), hp(0.22, 0.44)], True), "detail", 6.8, True, 0.05, region="sunglasses")
    A("sunglasses-arm", "head", sh.poly("head", [hp(0.20, 0.52), hp(-0.03, 0.52)], False), "detail", 6.8, False, 0.1, region="sunglasses")
    A("bag", "pelvis", sh.path("pelvis", [W2("pelvis", -0.13, -0.01), W2("pelvis", -0.13, 0.15), W2("pelvis", 0.0, 0.15), W2("pelvis", 0.0, -0.01)], True), "body", 7.5, True, 0.35, region="bag", hatch="denim")
    A("bag-flap", "pelvis", sh.poly("pelvis", [W2("pelvis", -0.13, 0.06), W2("pelvis", 0.0, 0.06)], False), "detail", 7.6, False, 0.2, region="bag")
    A("bag-strap", "torso", sh.poly("torso", [W2("torso", h.Lt * 0.9, 0.06), W2("pelvis", -0.07, 0.10)], False), "detail", 7.4, False, 0.3, region="bag")
    A("cane-shaft", cb, sh.capsule(cb, REST[cb].ap(0.10, 0), REST[cb].ap(L_CANE - 0.08, 0), 0.006, 0.006), "body", 9.0, True, 0.95, region="cane-shaft", pen=0.1)
    A("cane-band", cb, sh.capsule(cb, REST[cb].ap(L_CANE - 0.33, 0), REST[cb].ap(L_CANE - 0.08, 0), 0.0062, 0.0062), "body", 9.1, True, 0.35, region="cane-shaft", pen=0.1)
    A("cane-grip", cb, sh.capsule(cb, REST[cb].ap(-0.10, 0), REST[cb].ap(0.10, 0), 0.0145, 0.0145), "body", 9.2, True, 0.12, region="cane-grip", pen=0.12)
    A("cane-strap", cb, sh.path(cb, [W2(cb, -0.10, 0.0), W2(cb, -0.17, 0.03), W2(cb, -0.20, 0.0), W2(cb, -0.15, -0.03)], False), "detail", 9.3, False, 0.3, region="cane-grip")
    A("cane-tip-holder", cb, sh.capsule(cb, REST[cb].ap(L_CANE - 0.08, 0), REST[cb].ap(L_CANE - 0.025, 0), 0.0085, 0.0085), "body", 9.2, True, 0.6, region="cane-tip", pen=0.1)
    A("cane-roller", cb, sh.ellipse(cb, REST[cb].ap(L_CANE - 0.0170, 0), 0.0170, 0.0170), "body", 9.3, True, 0.8, region="cane-tip", pen=0.1)


def build(sh, level):
    dress(sh, h, rig, REST, OUT, level)
    extras(sh, level)


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
