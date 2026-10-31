"""NW-H1 child-on-kick-scooter: four individually drawn representations."""
from scooter_phys import *
from figure import *
from docbuild import D

HELMET = [(0.40, 0.66), (0.42, 0.84), (0.30, 1.0), (0.05, 1.12), (-0.25, 1.06), (-0.45, 0.86), (-0.47, 0.62), (-0.35, 0.52), (0.0, 0.56), (0.30, 0.60)]
HELMET_C = [(0.42, 0.66), (0.30, 1.0), (0.0, 1.14), (-0.40, 0.95), (-0.47, 0.60), (0.0, 0.56)]
PONY = [(-0.40, 0.58), (-0.50, 0.42), (-0.62, 0.20), (-0.66, -0.10), (-0.55, -0.18), (-0.46, 0.05), (-0.38, 0.30)]

OUT = dict(
    reg=dict(skin="face-skin", hair="hair", hat="helmet", hands="hands", upper="jacket", lower="leggings", shoes="sneakers", boot="sneakers", skirt="jacket"),
    tone=dict(skin=0.85, hair=0.30, hat=0.55, upper=0.68, upper_far=0.55, lower=0.28, lower_far=0.22, shoes=0.85, collar=0.6, boot=0.85),
    torso=[(-0.12, .092, .084), (0.0, .088, .088), (0.25, .084, .090), (0.5, .090, .102), (0.8, .094, .098), (0.93, .084, .082), (1.0, .060, .058)],
    hip=[(-1.0, .092, .090), (0.0, .094, .090), (1.0, .092, .088), (1.3, .090, .086)], skirt=None,
    sleeve_up=[(-0.10, .040, .038), (0.4, .038, .037), (1.08, .033, .033)], sleeve_fore=[(-0.12, .033, .033), (0.4, .031, .032), (0.93, .029, .029), (1.0, .031, .031)],
    thigh=[(-0.05, .066, .062), (0.5, .058, .056), (1.06, .042, .040)], shank=[(-0.06, .042, .040), (0.2, .046, .040), (0.5, .042, .036), (0.85, .030, .029), (0.97, .032, .031)],
    collar=[(-0.6, .060, .058), (0.1, .060, .058), (0.6, .050, .052)],
    hat_pts=HELMET, hat_c=HELMET_C, hair_pts=PONY, shoe="sneaker", boot_side=None, boot_shoe="sneaker", hatch="fleece", far_hatch="far-side", glasses=False, nub=False, buttons=True, cuff=None,
    thigh500=[(-0.05, .068, .064), (0.5, .060, .058), (1.05, .046, .044)], shank500=[(-0.05, .046, .044), (0.5, .044, .038), (1.0, .036, .034)],
    skirt500=None, torso500=[(-0.12, .092, .084), (0.5, .090, .102), (0.93, .084, .082), (1.0, .060, .058)], hip500=[(-1.0, .092, .090), (1.3, .090, .086)],
    pen_arm500=0.14, pen_hand500=0.13, skirt_hatch=None)
HATCH50 = [{"id": "fleece", "title": tx("Fleece lines", "Fleecelinien"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.8, "penMm": 0.1},
           {"id": "far-side", "title": tx("Far side", "Abgewandte Seite"), "pattern": "lines", "angleDeg": 60, "spacingMm": 0.6, "penMm": 0.1},
           {"id": "griptape", "title": tx("Grip tape", "Griffband"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}]


def f(x, y):
    return (X_R0 + x, y)


def fp(pts):
    return [(X_R0 + p[0], p[1]) + tuple(p[2:]) for p in pts]


def cap(sh, id_, p0, p1, r, region, z, tone, pen):
    return sh.add(id_, "frame", sh.capsule("frame", f(*p0), f(*p1), r, r), "body", z, True, tone, region=region, pen=pen)


def stroke(sh, id_, pts, region, z, pen, regions=None):
    kw = dict(regions=regions) if regions else dict(region=region)
    return sh.add(id_, "frame", sh.poly("frame", [f(*p) for p in pts], False), "body", z, False, 0.4, pen=pen, **kw)


def wheels(sh, level):
    A = sh.add
    for nm, bone, cx in (("rear", "wheel-rear", 0.0), ("front", "wheel-front", WB)):
        c = f(cx, R_W)
        A("tyre-" + nm, bone, sh.ellipse(bone, c, R_W, R_W), "body", 9.2, True, 0.14, region="wheels", pen=0.1 if level == 100 else None)
        if level in (50, 100):
            A("rim-" + nm, bone, sh.ellipse(bone, c, 0.046, 0.046), "body", 9.3, True, 0.8, region="wheels", pen=0.1)
            A("hub-" + nm, bone, sh.ellipse(bone, c, 0.014, 0.014), "detail", 9.4, True, 0.3, region="wheels", pen=0.1)
        if level == 50:
            for k in range(3):
                a = math.radians(90 + 120 * k)
                A("spoke-%s%d" % (nm, k), bone, sh.poly(bone, [(c[0] + 0.014 * math.cos(a), c[1] + 0.014 * math.sin(a)), (c[0] + 0.046 * math.cos(a), c[1] + 0.046 * math.sin(a))], False), "detail", 9.45, False, 0.4, region="wheels")


def scooter(sh, level):
    A = sh.add
    if level == 500:
        stroke(sh, "deck", [(0.03, 0.095), (0.62, 0.095)], "deck", 9.0, 0.10)
        stroke(sh, "stem", [(0.69, 0.16), (0.60, 0.84)], None, 9.0, 0.10, regions=["stem", "grips"])
        stroke(sh, "fork", [(0.70, 0.16), (0.78, R_W)], "stem", 9.0, 0.10)
        wheels(sh, 500)
        return
    if level == 200:
        stroke(sh, "deck", [(0.04, 0.095), (0.62, 0.095)], "deck", 9.0, 0.17)
        stroke(sh, "stem", [(0.69, 0.16), (0.60, 0.80)], "stem", 9.0, 0.13)
        stroke(sh, "fork", [(0.70, 0.16), (0.78, R_W)], "stem", 9.0, 0.12)
        A("grip", "frame", sh.ellipse("frame", f(*BAR), 0.02, 0.02), "body", 9.5, True, 0.12, region="grips")
        A("fender", "frame", sh.path("frame", fp([(-0.06, 0.115, 'c'), (0.02, 0.16), (0.12, 0.15), (0.14, 0.125, 'c')]), False), "body", 9.0, False, 0.5, region="deck", pen=0.13)
        wheels(sh, 200)
        return
    A("deck", "frame", sh.path("frame", fp([(0.05, 0.08, 'c'), (0.60, 0.08, 'c'), (0.64, 0.11), (0.60, 0.13, 'c'), (0.05, 0.13, 'c')])), "body", 9.0, True, 0.3, region="deck", hatch="griptape" if level == 50 else None)
    cap(sh, "head-tube", (0.64, 0.10), (0.71, 0.18), 0.018, "stem", 9.1, 0.7, 0.1)
    cap(sh, "stem", (0.69, 0.17), (0.60, 0.82), 0.0135, "stem", 9.1, 0.7, 0.15 if level == 50 else 0.1)
    cap(sh, "fork", (0.71, 0.17), (0.78, R_W), 0.012, "stem", 9.1, 0.7, 0.15 if level == 50 else 0.1)
    A("fender", "frame", sh.path("frame", fp([(-0.07, 0.12, 'c'), (0.02, 0.165), (0.12, 0.155), (0.15, 0.125, 'c'), (0.06, 0.14), (-0.02, 0.125)])), "body", 9.0, True, 0.2, region="deck", pen=0.1 if level == 100 else None)
    A("grip", "frame", sh.ellipse("frame", f(*BAR), 0.0175, 0.0175), "body", 9.5, True, 0.12, region="grips", pen=0.1)
    A("clamp", "frame", sh.ellipse("frame", f(BAR[0] + 0.012, BAR[1] - 0.045), 0.022, 0.014, 70.0), "detail", 9.4, True, 0.5, region="stem")
    if level == 50:
        cap(sh, "strut", (0.06, 0.095), (0.0, R_W), 0.009, "stem", 9.0, 0.7, 0.1)
        A("brake-pad", "frame", sh.poly("frame", [f(0.02, 0.14), f(0.10, 0.145)], False), "detail", 9.6, False, 0.2, region="deck")
    wheels(sh, level)


def extras(sh, level):
    A = sh.add
    W2 = lambda b, x, y: REST[b].ap(x, y)
    pts = [W2("torso", 0.06 * h.Lt, 0.092), W2("torso", 0.06 * h.Lt, 0.19), W2("torso", 0.85 * h.Lt, 0.19), W2("torso", 0.85 * h.Lt, 0.092)]
    if level == 500:
        A("backpack", "torso", sh.path("torso", pts, True), "body", 4.0, True, 0.4, region="backpack")
        return
    A("backpack", "torso", sh.path("torso", pts, True), "body", 4.0, True, 0.4, region="backpack", hatch=None)
    if level >= 100:
        return
    A("backpack-strap", "torso", sh.poly("torso", [W2("torso", 0.80 * h.Lt, 0.086), W2("torso", 0.15 * h.Lt, 0.09)], False), "detail", 5.5, False, 0.3, region="backpack")
    A("backpack-pocket", "torso", sh.poly("torso", [W2("torso", 0.12 * h.Lt, 0.19), W2("torso", 0.5 * h.Lt, 0.19), W2("torso", 0.5 * h.Lt, 0.15)], False), "detail", 4.1, False, 0.3, region="backpack")


def build(sh, level):
    dress(sh, h, rig, REST, OUT, level)
    extras(sh, level)
    scooter(sh, level)


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
