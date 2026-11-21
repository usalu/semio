"""🧍 Round 5, 1:50 of `humans/courier-with-hand-truck`, stated by hand: body (garments over a 1.82 m male, gripping gloves, work boots, knit cap, steel tube hand truck after the fetra B1125L main dimensions, three cartons), thermal (regions with nested layer shells) and structure (skeleton, truck frame, carton stack). Frames: u along the bone, v its sagittal normal (front of trunk and head is -v, front of the hanging limbs +v, back of the gripping hand +v), w lateral (left +). Lengths in metres."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(__file__), "r3-courier-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
S, F, loft, cap, ell, box, rev, ext, both, side, shell, of, mem, st = k.S, k.F, k.loft, k.cap, k.ell, k.box, k.rev, k.ext, k.both, k.side, k.shell, k.of, k.mem, k.st
_fspec = importlib.util.spec_from_file_location("forms", os.path.join(os.path.dirname(__file__), "r5-courier-with-hand-truck-forms.py"))
fm = importlib.util.module_from_spec(_fspec)
_fspec.loader.exec_module(fm)

JACKET, FLEECE, SHIRT, WORK, BRIEF, BOOT, SOCK, GLOVE, CAP, SKIN, HAIR, STEEL, RUBBER, CARD = "hivis-softshell", "fleece-liner", "cotton-underwear", "cotton-workwear", "cotton-underwear", "leather-work-boot", "cotton-sock", "fabric-glove", "wool-cap", "skin", "hair", "steel-tube", "tyre-rubber", "cardboard"
RV = -0.14
RW = 0.1465
TUBE = 0.0135
BAR = (0.082, -0.032)


PHALANGES = ("proximal", "middle", "distal")


def wrap(deg, r=0.026):
    a = math.radians(deg)
    return [BAR[0] + r * math.cos(a), BAR[1] + r * math.sin(a)]


def rotz(deg):
    a = math.radians(deg) / 2
    return [0, 0, round(math.sin(a), 5), round(math.cos(a), 5)]


def trunk():
    return [
        loft("jacket", "torso", JACKET, 0.62, fm.sections(S, fm.JACKET)),
        loft("jacket-collar", "torso", JACKET, 0.55, [S(0.47, 0.010, 0, 0.084, 0.078, 2.1), S(0.50, 0.006, 0, 0.074, 0.074, 2.0), S(0.522, 0.002, 0, 0.071, 0.073, 2.0)]),
        loft("reflective-band-0", "torso", JACKET, 0.9, fm.band(S, 0.005, 0.055)),
        loft("reflective-band-1", "torso", JACKET, 0.9, fm.band(S, 0.105, 0.155)),
        loft("trousers-seat", "pelvis", WORK, 0.4, fm.sections(S, fm.SEAT)),
    ]


def head():
    return [
        loft("neck", "neck", SKIN, 0.7, [S(-0.03, 0.004, 0, 0.068, 0.064, 2.1), S(0.03, 0.0, 0, 0.060, 0.060, 2.0), S(0.08, -0.004, 0, 0.058, 0.062, 2.0), S(0.125, -0.010, 0, 0.056, 0.064, 2.0)]),
        loft("head", "head", SKIN, 0.7, [S(-0.054, -0.072, 0, 0.024, 0.020, 2.0), S(-0.044, -0.062, 0, 0.040, 0.034, 2.1), S(-0.025, -0.042, 0, 0.053, 0.058, 2.2), S(-0.005, -0.028, 0, 0.062, 0.076, 2.2), S(0.02, -0.020, 0, 0.068, 0.086, 2.2), S(0.055, -0.012, 0, 0.074, 0.094, 2.2), S(0.09, -0.008, 0, 0.0775, 0.098, 2.2), S(0.13, -0.004, 0, 0.074, 0.094, 2.2), S(0.16, -0.002, 0, 0.058, 0.074, 2.1), S(0.174, -0.002, 0, 0.030, 0.038, 2.0)]),
        loft("nose", "head", SKIN, 0.7, [S(0.054, -0.103, 0, 0.007, 0.006, 2.0), S(0.030, -0.109, 0, 0.011, 0.011, 2.0), S(0.010, -0.112, 0, 0.016, 0.014, 2.2), S(0.000, -0.109, 0, 0.015, 0.010, 2.2)]),
        ell("ear-l", "head", SKIN, 0.68, [0.036, 0.006, 0.0745], [0.031, 0.017, 0.008], "detail", rotation=rotz(-12)),
        ell("ear-r", "head", SKIN, 0.68, [0.036, 0.006, -0.0745], [0.031, 0.017, 0.008], "detail", rotation=rotz(-12)),
        ell("brow", "head", SKIN, 0.66, [0.066, -0.097, 0], [0.008, 0.012, 0.052], "detail"),
        ell("eye-l", "head", SKIN, 0.2, [0.052, -0.099, 0.031], [0.006, 0.006, 0.012], "detail"),
        ell("eye-r", "head", SKIN, 0.2, [0.052, -0.099, -0.031], [0.006, 0.006, 0.012], "detail"),
        ell("chin", "head", SKIN, 0.7, [-0.040, -0.080, 0], [0.016, 0.014, 0.024]),
        loft("hair-nape", "head", HAIR, 0.25, [S(-0.012, 0.036, 0, 0.050, 0.046, 2.2), S(0.02, 0.022, 0, 0.066, 0.068, 2.2), S(0.07, 0.022, 0, 0.074, 0.072, 2.2)]),
        loft("cap", "head", CAP, 0.3, [S(0.071, -0.008, 0, 0.0835, 0.1045, 2.2), S(0.10, -0.007, 0, 0.0835, 0.1040, 2.2), S(0.14, -0.004, 0, 0.078, 0.097, 2.2), S(0.166, -0.002, 0, 0.060, 0.075, 2.1), S(0.180, -0.002, 0, 0.030, 0.038, 2.0)]),
        loft("cap-cuff", "head", CAP, 0.26, [S(0.063, -0.009, 0, 0.0865, 0.1075, 2.2), S(0.075, -0.008, 0, 0.0875, 0.1085, 2.2), S(0.089, -0.008, 0, 0.0865, 0.1070, 2.2)]),
        loft("cap-brim", "head", CAP, 0.22, [{"at": [0.070, -0.100, 0], "rx": 0.005, "ry": 0.072, "exponent": 2.6}, {"at": [0.064, -0.135, 0], "rx": 0.005, "ry": 0.066, "exponent": 2.6}, {"at": [0.057, -0.160, 0], "rx": 0.005, "ry": 0.050, "exponent": 2.4}, {"at": [0.053, -0.174, 0], "rx": 0.004, "ry": 0.026, "exponent": 2.2}]),
    ]


def arm_l():
    wk = [wrap(70), wrap(-20), wrap(-95), wrap(-150)]
    digits = {
        "index": [(0.09, -0.005, -0.032, 0.0105), (0.105, -0.037, -0.033, 0.0100), (0.082, -0.055, -0.034, 0.0086), (0.063, -0.046, -0.034, 0.0072)],
        "middle": [(0.092, -0.006, -0.011, 0.011), (0.108, -0.040, -0.011, 0.0104), (0.081, -0.059, -0.011, 0.0092), (0.058, -0.044, -0.012, 0.0078)],
        "ring": [(0.089, -0.008, 0.011, 0.0102), (0.104, -0.04, 0.0105, 0.0097), (0.08, -0.057, 0.0115, 0.0084), (0.061, -0.045, 0.012, 0.0074)],
        "little": [(0.08, -0.009, 0.031, 0.0092), (0.096, -0.034, 0.032, 0.0085), (0.08, -0.05, 0.033, 0.0074), (0.064, -0.044, 0.033, 0.0066)],
    }
    phalanges = [cap(f"glove-{d}-{part}-l", "hand-l", GLOVE, 0.34, list(rows[i][:3]), list(rows[i + 1][:3]), rows[i][3], rows[i + 1][3]) for i, part in enumerate(PHALANGES) for d, rows in digits.items()]
    return [
        loft("sleeve-l", "arm-l", JACKET, 0.62, fm.sections(S, fm.SLEEVE)),
        loft("forearm-l", "forearm-l", JACKET, 0.62, fm.sections(S, fm.FOREARM)),
        loft("cuff-strap-l", "forearm-l", JACKET, 0.3, [S(0.212, 0.0, 0, 0.0415, 0.037, 2.4), S(0.242, 0.0, 0, 0.0405, 0.0355, 2.4)]),
        loft("glove-cuff-l", "hand-l", GLOVE, 0.3, [S(-0.028, 0.0, 0, 0.033, 0.026, 2.4), S(0.0, 0.0, 0, 0.033, 0.024, 2.4), S(0.022, 0.002, 0, 0.036, 0.023, 2.5)]),
        loft("glove-palm-l", "hand-l", GLOVE, 0.36, [S(0.005, 0.001, 0, 0.034, 0.022, 2.6), S(0.035, 0.004, 0, 0.044, 0.021, 2.8), S(0.07, 0.004, 0, 0.047, 0.020, 2.8), S(0.098, 0.0, 0, 0.046, 0.017, 2.6)]),
        *phalanges,
        cap("glove-thumb-0-l", "hand-l", GLOVE, 0.34, [0.020, -0.006, -0.030], [0.052, -0.030, -0.050], 0.014, 0.012),
        cap("glove-thumb-1-l", "hand-l", GLOVE, 0.34, [0.052, -0.030, -0.050], [0.084, -0.058, -0.038], 0.012, 0.010),
    ]


def leg_l():
    return [
        loft("thigh-l", "thigh-l", WORK, 0.4, fm.sections(S, fm.THIGH)),
        loft("cargo-pocket-l", "thigh-l", WORK, 0.36, fm.sections(S, fm.POCKET)),
        loft("shank-l", "shank-l", WORK, 0.4, fm.sections(S, fm.SHANK)),
        loft("boot-shaft-l", "shank-l", BOOT, 0.25, [S(0.300, -0.006, 0, 0.047, 0.054, 2.3), S(0.36, -0.004, 0, 0.043, 0.051, 2.3), S(0.415, -0.002, 0, 0.042, 0.054, 2.3), S(0.445, 0.0, 0, 0.042, 0.056, 2.3)]),
        loft("boot-upper-l", "foot-l", BOOT, 0.25, [F(-0.066, 0.062, 0.036, 0, 0.030, 2.4), F(-0.040, 0.078, 0.054, 0, 0.040, 2.5), F(0.0, 0.086, 0.062, 0, 0.044, 2.5), F(0.045, 0.078, 0.054, 0, 0.045, 2.5), F(0.09, 0.062, 0.040, 0, 0.047, 2.5), F(0.14, 0.052, 0.031, 0.002, 0.050, 2.6), F(0.19, 0.047, 0.027, 0.003, 0.052, 2.6)]),
        loft("boot-sole-l", "foot-l", BOOT, 0.12, [F(-0.069, 0.016, 0.014, 0, 0.030, 3.0), F(-0.045, 0.015, 0.015, 0, 0.041, 3.2), F(0.018, 0.015, 0.015, 0, 0.045, 3.2), F(0.034, 0.019, 0.010, 0, 0.042, 3.2), F(0.08, 0.016, 0.010, 0.001, 0.047, 3.2), F(0.13, 0.011, 0.011, 0.002, 0.052, 3.2), F(0.19, 0.011, 0.011, 0.003, 0.055, 3.2)]),
        loft("boot-toe-l", "toe-l", BOOT, 0.25, [S(-0.012, 0.047, 0.003, 0.052, 0.027, 2.6), S(0.028, 0.045, 0.003, 0.050, 0.026, 2.6), S(0.054, 0.040, 0.002, 0.041, 0.020, 2.5), S(0.068, 0.034, 0.002, 0.026, 0.012, 2.3)]),
        loft("boot-toe-sole-l", "toe-l", BOOT, 0.12, [S(-0.012, 0.011, 0.003, 0.055, 0.011, 3.2), S(0.04, 0.012, 0.003, 0.052, 0.011, 3.2), S(0.062, 0.014, 0.002, 0.041, 0.010, 3.0), S(0.0725, 0.016, 0.002, 0.026, 0.008, 2.6)]),
    ]


def truck(tube=TUBE):
    bend = lambda s, a0, a1, i: cap(f"bow-{'l' if s > 0 else 'r'}-{i}", "hand-truck", STEEL, 0.35, [0.9615 + 0.045 * math.sin(math.radians(a0)), RV, s * (0.1015 + 0.045 * math.cos(math.radians(a0)))], [0.9615 + 0.045 * math.sin(math.radians(a1)), RV, s * (0.1015 + 0.045 * math.cos(math.radians(a1)))], tube)
    tyre = [[-0.030, 0.056], [-0.0425, 0.078], [-0.0425, 0.104], [-0.037, 0.119], [-0.024, 0.1275], [0, 0.13], [0.024, 0.1275], [0.037, 0.119], [0.0425, 0.104], [0.0425, 0.078], [0.030, 0.056]]
    rim = [[-0.0375, 0], [-0.0375, 0.020], [-0.014, 0.028], [-0.014, 0.046], [-0.032, 0.052], [-0.032, 0.059], [0.032, 0.059], [0.032, 0.052], [0.014, 0.046], [0.014, 0.028], [0.0375, 0.020], [0.0375, 0]]
    twice = {"transform": {"translation": [0, 0, -0.2425]}, "repeat": {"count": 2, "translation": [0, 0, 0.485]}}
    return [
        cap("rail-l", "hand-truck", STEEL, 0.35, [-0.118, RV, RW], [0.9615, RV, RW], tube),
        cap("rail-r", "hand-truck", STEEL, 0.35, [-0.118, RV, -RW], [0.9615, RV, -RW], tube),
        *[bend(s, a, a + 30, i) for s in (1, -1) for i, a in enumerate((0, 30, 60))],
        cap("handle", "hand-truck", STEEL, 0.2, [1.0065, RV, -0.1015], [1.0065, RV, 0.1015], tube),
        *[cap(f"brace-{i}", "hand-truck", STEEL, 0.35, [u, RV, -RW], [u, RV, RW], 0.010) for i, u in enumerate((0.20, 0.47, 0.74))],
        *[cap(f"strut-{n}-{'l' if s > 0 else 'r'}", "hand-truck", STEEL, 0.35, [u, RV, s * RW], [0, 0, s * 0.170], 0.011) for s in (1, -1) for n, u in (("low", -0.105), ("high", 0.17))],
        ext("plate", "hand-truck", STEEL, 0.35, [[-0.130, -0.120], [-0.130, -0.392], [-0.1265, -0.4035], [-0.122, -0.4035], [-0.122, -0.120]], "xy", -0.16, 0.16),
        cap("axle", "truck-lift", STEEL, 0.2, [0, 0, -0.279], [0, 0, 0.279], 0.010),
        rev("tyre", "truck-lift", RUBBER, 0.18, tyre + [tyre[0]], [0, 0, 1], [0, 0, 0], **twice),
        rev("rim", "truck-lift", STEEL, 0.5, rim, [0, 0, 1], [0, 0, 0], **twice),
    ]


PARCELS = (("a", 0.078, -0.3575, (0.40, 0.408, 0.46)), ("b", 0.428, -0.3375, (0.30, 0.368, 0.40)), ("c", 0.69485, -0.2994, (0.2337, 0.2918, 0.36)))


def parcels(radius=0.004, tape=True):
    out = [box(f"parcel-{n}", "parcels", CARD, 0.52, [u, v, 0], list(size), radius) for n, u, v, size in PARCELS]
    if tape:
        out += [box(f"tape-{n}", "parcels", CARD, 0.3, [u, v - size[1] / 2 - 0.001, 0], [size[0], 0.004, 0.05], role="detail", texture=True) for n, u, v, size in PARCELS]
        out += [box(f"tape-top-{n}", "parcels", CARD, 0.3, [u + size[0] / 2 + 0.001, v, 0], [0.004, size[1], 0.05], role="detail", texture=True) for n, u, v, size in PARCELS[2:]]
    return out


def body():
    return trunk() + head() + both(arm_l()) + both(leg_l()) + truck() + parcels()


def thermal(b):
    by = {x["id"]: x for x in b}
    t = []
    for i in ("head", "neck", "nose", "ear-l", "ear-r", "chin", "brow"):
        t.append(of(by[i], "face-skin"))
    t += [of(by["cap"], "cap", "cap", sectionFaces=["-u"]), of(by["cap-cuff"], "cap", "cap", sectionFaces=["-u"])]
    for s in ("l", "r"):
        for i in ("glove-palm", *[f"glove-{d}-{part}" for part in PHALANGES for d in ("index", "middle", "ring", "little")], "glove-thumb-0", "glove-thumb-1"):
            t.append(of(by[f"{i}-{s}"], "gloves", "glove"))
    for i in ("jacket", "jacket-collar", "sleeve-l", "forearm-l", "sleeve-r", "forearm-r"):
        t += [of(by[i], "jacket", "softshell"), shell(by[i], 0.002, f"th-{i}-fleece", "jacket", "fleece"), shell(by[i], 0.006, f"th-{i}-shirt", "jacket", "shirt")]
    for s in ("l", "r"):
        t += [of(by[f"thigh-{s}"], "trousers", "workwear"), of(by[f"shank-{s}"], "trousers", "workwear")]
        briefs = shell(by[f"thigh-{s}"], 0.0015, f"th-thigh-{s}-briefs", "trousers", "briefs")
        briefs["sections"] = briefs["sections"][:3]
        t.append(briefs)
        for i in ("boot-upper", "boot-sole", "boot-toe", "boot-toe-sole"):
            t.append(of(by[f"{i}-{s}"], "boots", "boot"))
        for i in ("boot-upper", "boot-toe"):
            t.append(shell(by[f"{i}-{s}"], 0.005, f"th-{i}-{s}-socks", "boots", "socks"))
    for x in b:
        if x["bone"] == "hand-truck" or x["id"] in ("axle", "rim"):
            t.append(of(x, "hand-truck"))
        elif x["id"] == "tyre":
            t.append(of(x, "tyres"))
        elif x["id"].startswith("parcel-"):
            t.append(of(x, "parcels"))
    t += [
        ell("th-nose-breath", "head", SKIN, 0.5, [-0.001, -0.114, 0], [0.004, 0.006, 0.010], region="face-skin", tags=["emitter", "nose-breath"]),
        ell("th-mouth-breath", "head", SKIN, 0.5, [-0.019, -0.103, 0], [0.004, 0.005, 0.013], region="face-skin", tags=["emitter", "mouth-breath"]),
        ell("th-body-plume", "head", CAP, 0.5, [0.176, -0.002, 0], [0.006, 0.050, 0.042], region="jacket", tags=["emitter", "body-plume"]),
    ]
    return t


def skeleton_l():
    BONE = (SKIN, 0.9)
    wk = [wrap(70, 0.022), wrap(-20, 0.022), wrap(-95, 0.022), wrap(-150, 0.022)]
    ph = lambda i, a, b: st(cap(f"phalanx-{i}-l", "hand-l", *BONE, [a[0], a[1], -0.033], [b[0], b[1], -0.033], 0.005, repeat={"count": 4, "translation": [0, 0, 0.022]}), "hand-l")
    fa = lambda dx, y: [round(dx * math.cos(k.FOOT_A) - (y - 0.0803) * math.sin(k.FOOT_A), 5), round(dx * math.sin(k.FOOT_A) + (y - 0.0803) * math.cos(k.FOOT_A), 5), 0]
    return [
        st(ell("humeral-head-l", "arm-l", *BONE, [0, 0, -0.004], [0.023, 0.023, 0.023]), "arm-l", "joint"),
        st(cap("humerus-l", "arm-l", *BONE, [0.012, 0, 0], [0.322, 0, 0], 0.012, 0.011), "arm-l"),
        st(ell("humeral-condyles-l", "arm-l", *BONE, [0.3366, 0, 0], [0.014, 0.016, 0.031]), "arm-l"),
        st(ell("elbow-l", "forearm-l", *BONE, [0, 0, 0], [0.014, 0.014, 0.014]), "forearm-l", "joint"),
        st(cap("ulna-l", "forearm-l", *BONE, [-0.016, -0.014, -0.010], [0.258, -0.004, -0.017], 0.009, 0.006), "forearm-l"),
        st(cap("radius-l", "forearm-l", *BONE, [0.010, 0.006, 0.015], [0.262, 0.004, 0.012], 0.007, 0.009), "forearm-l"),
        st(ell("wrist-l", "hand-l", *BONE, [0.012, 0.0, 0], [0.018, 0.012, 0.027]), "hand-l", "joint"),
        st(loft("metacarpals-l", "hand-l", *BONE, [S(0.028, 0.002, 0, 0.028, 0.007, 3), S(0.06, 0.003, 0, 0.037, 0.007, 3), S(0.092, -0.004, 0, 0.041, 0.007, 3)]), "hand-l"),
        ph(0, wk[0], wk[1]),
        ph(1, wk[1], wk[2]),
        ph(2, wk[2], wk[3]),
        st(cap("thumb-bones-l", "hand-l", *BONE, [0.022, -0.006, -0.028], [0.08, -0.054, -0.04], 0.006, 0.005), "hand-l"),
        st(ell("femoral-head-l", "thigh-l", *BONE, [0, 0, 0], [0.024, 0.024, 0.024]), "thigh-l", "joint"),
        st(cap("femoral-neck-l", "thigh-l", *BONE, [0, 0, 0], [0.036, 0, 0.052], 0.015, 0.018), "thigh-l"),
        st(cap("femur-l", "thigh-l", *BONE, [0.036, 0, 0.052], [0.425, 0.004, 0.002], 0.016, 0.015), "thigh-l"),
        st(ell("femoral-condyles-l", "thigh-l", *BONE, [0.438, 0.0, 0], [0.022, 0.031, 0.041]), "thigh-l"),
        st(ell("patella-l", "thigh-l", *BONE, [0.436, 0.044, 0], [0.021, 0.010, 0.021]), "thigh-l"),
        st(ell("knee-l", "shank-l", *BONE, [0, 0, 0], [0.012, 0.012, 0.012]), "shank-l", "joint"),
        st(ell("tibial-plateau-l", "shank-l", *BONE, [0.014, 0.0, 0], [0.014, 0.027, 0.038]), "shank-l"),
        st(cap("tibia-l", "shank-l", *BONE, [0.022, 0.008, -0.006], [0.424, 0.0, -0.004], 0.015, 0.012), "shank-l"),
        st(cap("fibula-l", "shank-l", *BONE, [0.034, -0.016, 0.034], [0.438, -0.008, 0.028], 0.007, 0.008), "shank-l"),
        st(ell("talus-l", "foot-l", *BONE, [0, 0, 0], [0.022, 0.018, 0.022]), "foot-l", "joint"),
        st(ell("calcaneus-l", "foot-l", *BONE, fa(-0.026, 0.056), [0.034, 0.024, 0.021]), "foot-l"),
        st(loft("metatarsals-l", "foot-l", *BONE, [F(0.02, 0.066, 0.020, 0, 0.027, 2.6), F(0.10, 0.050, 0.013, 0.002, 0.036, 2.8), F(0.185, 0.036, 0.008, 0.003, 0.043, 3.0)]), "foot-l"),
        st(ell("toe-joints-l", "toe-l", *BONE, [0, 0.034, 0.003], [0.009, 0.008, 0.042]), "toe-l", "joint"),
        st(loft("toe-phalanges-l", "toe-l", *BONE, [S(0.0, 0.033, 0.003, 0.042, 0.007, 3), S(0.035, 0.030, 0.003, 0.038, 0.006, 3), S(0.058, 0.028, 0.002, 0.028, 0.005, 2.6)]), "toe-l"),
    ]


def structure(b):
    by = {x["id"]: x for x in b}
    BONE = (SKIN, 0.9)
    rib = lambda i, u, depth, width: st(rev(f"rib-hoop-{i}", "torso", *BONE, [[0.006 * math.cos(a), depth + 0.006 * math.sin(a)] for a in [j * math.pi / 4 for j in range(8)]] + [[0.006, depth]], [1, 0, 0], [0, 0, 0], lateralScale=round(width / depth, 4), transform={"rotation": rotz(-20), "translation": [u, -0.012, 0]}), "torso")
    s = [
        st(ell("cranium", "head", *BONE, [0.094, -0.004, 0], [0.074, 0.088, 0.070]), "head"),
        st(loft("facial-skeleton", "head", *BONE, [S(0.066, -0.060, 0, 0.052, 0.026, 2.6), S(0.040, -0.068, 0, 0.050, 0.028, 2.4), S(0.014, -0.072, 0, 0.040, 0.027, 2.2), S(-0.012, -0.074, 0, 0.030, 0.022, 2.2)]), "head"),
        *[st(cap(f"zygomatic-arch-{n}", "head", *BONE, [0.034, -0.078, sg * 0.050], [0.036, -0.012, sg * 0.068], 0.006), "head") for n, sg in (("l", 1), ("r", -1))],
        st(cap("mandible-body", "head", *BONE, [-0.046, -0.084, -0.012], [-0.046, -0.084, 0.012], 0.009), "head"),
        st(cap("mandible-l", "head", *BONE, [-0.016, -0.026, 0.046], [-0.044, -0.082, 0.008], 0.008), "head"),
        st(cap("mandible-r", "head", *BONE, [-0.016, -0.026, -0.046], [-0.044, -0.082, -0.008], 0.008), "head"),
        st(cap("mandible-ramus-l", "head", *BONE, [0.028, -0.014, 0.052], [-0.016, -0.026, 0.046], 0.008), "head"),
        st(cap("mandible-ramus-r", "head", *BONE, [0.028, -0.014, -0.052], [-0.016, -0.026, -0.046], 0.008), "head"),
        st(ell("atlas", "head", *BONE, [0, 0.004, 0], [0.012, 0.016, 0.024]), "head", "joint"),
        st(loft("cervical-spine", "neck", *BONE, [S(-0.012, 0.016, 0, 0.016, 0.014, 2), S(0.04, 0.006, 0, 0.014, 0.013, 2), S(0.105, 0.004, 0, 0.013, 0.012, 2)]), "neck"),
        st(ell("c7", "neck", *BONE, [0, 0.014, 0], [0.013, 0.016, 0.02]), "neck", "joint"),
        st(loft("spine", "torso", *BONE, [S(-0.01, 0.036, 0, 0.026, 0.021, 2), S(0.06, 0.030, 0, 0.025, 0.020, 2), S(0.14, 0.044, 0, 0.022, 0.018, 2), S(0.26, 0.066, 0, 0.019, 0.016, 2), S(0.37, 0.056, 0, 0.017, 0.015, 2), S(0.445, 0.030, 0, 0.016, 0.014, 2), S(0.482, 0.014, 0, 0.015, 0.014, 2)]), "torso"),
        st(ell("lumbosacral", "torso", *BONE, [0, 0.036, 0], [0.014, 0.022, 0.028]), "torso", "joint"),
        *[rib(i, u, d, w) for i, (u, d, w) in enumerate(((0.215, 0.092, 0.126), (0.262, 0.100, 0.136), (0.308, 0.104, 0.140), (0.352, 0.100, 0.134), (0.394, 0.086, 0.116), (0.432, 0.058, 0.078)))],
        st({**box("sternum", "torso", *BONE, [0.325, -0.098, 0], [0.185, 0.012, 0.036], 0.004), "transform": {"translation": [0.325, -0.098, 0], "rotation": rotz(12)}}, "torso"),
        *[st(cap(f"clavicle-{n}", "torso", *BONE, [0.418, -0.076, sg * 0.022], [0.455, 0.004, sg * 0.190], 0.0065), "torso") for n, sg in (("l", 1), ("r", -1))],
        *[st(ell(f"scapula-{n}", "torso", *BONE, [0.375, 0.088, sg * 0.112], [0.078, 0.008, 0.056], rotation=[round(sg * 0.1305, 4), 0, 0, 0.9914]), "torso") for n, sg in (("l", 1), ("r", -1))],
        st(loft("sacrum", "pelvis", *BONE, [S(-0.03, 0.082, 0, 0.012, 0.010, 2.4), S(0.03, 0.074, 0, 0.034, 0.016, 2.4), S(0.10, 0.044, 0, 0.052, 0.024, 2.4)]), "pelvis"),
        *[x for n, sg in (("l", 1), ("r", -1)) for x in (
            st(ell(f"ilium-{n}", "pelvis", *BONE, [0.075, 0.018, sg * 0.118], [0.066, 0.052, 0.012], rotation=[0, round(sg * -0.1736, 4), 0, 0.9848]), "pelvis"),
            st(cap(f"ilium-body-{n}", "pelvis", *BONE, [0.05, 0.05, sg * 0.062], [0.006, 0.004, sg * 0.098], 0.016), "pelvis"),
            st(cap(f"pubic-ramus-{n}", "pelvis", *BONE, [0.0, -0.01, sg * 0.086], [-0.028, -0.066, sg * 0.008], 0.009), "pelvis"),
            st(cap(f"ischium-{n}", "pelvis", *BONE, [-0.006, 0.008, sg * 0.09], [-0.072, 0.030, sg * 0.058], 0.011), "pelvis"),
            st(cap(f"ischiopubic-{n}", "pelvis", *BONE, [-0.072, 0.030, sg * 0.058], [-0.036, -0.064, sg * 0.01], 0.008), "pelvis"),
        )],
        *both(skeleton_l()),
    ]
    for x in b:
        if x["bone"] == "hand-truck":
            s.append(mem(x, "hand-truck"))
        elif x["id"] in ("axle", "tyre", "rim"):
            s.append(mem(x, "truck-lift"))
        elif x["id"].startswith("parcel-"):
            s.append(mem(x, "parcels"))
    s += [
        st(ell("axle-pivot", "hand-truck", STEEL, 0.3, [0, 0, 0], [0.014, 0.014, 0.014]), "hand-truck", "joint"),
        st(ell("mass-pelvis", "pelvis", SKIN, 0.4, [0.0616, 0, 0], [0.05, 0.045, 0.06], "detail"), "pelvis", "mass"),
        st(ell("mass-torso", "torso", SKIN, 0.4, [0.2205, 0, 0], [0.07, 0.05, 0.07], "detail"), "torso", "mass"),
        st(ell("mass-thigh-l", "thigh-l", SKIN, 0.4, [0.1817, 0.02, -0.01], [0.045, 0.03, 0.03], "detail"), "thigh-l", "mass"),
        st(ell("mass-thigh-r", "thigh-r", SKIN, 0.4, [0.1817, 0.02, 0.01], [0.045, 0.03, 0.03], "detail"), "thigh-r", "mass"),
        st(ell("mass-hand-truck", "hand-truck", STEEL, 0.4, [0.14, -0.132, 0], [0.008, 0.008, 0.008], "detail"), "hand-truck", "mass"),
        *[st(ell(f"mass-parcel-{n}", "parcels", CARD, 0.4, [u, v, 0], [r, r, r], "detail"), "parcels", "mass") for (n, u, v, _), r in zip(PARCELS, (0.07, 0.058, 0.048))],
    ]
    return s


DECISIONS = [
    {"en": "Body 1:50: a 1.82 m male courier modelled as garments over the body: hip-length softshell jacket with stand-up collar and two 50 mm reflective bands, set-in sleeves with cuff straps, cargo trousers with thigh pockets, 6 inch work boots with heel, arch and toe cap, knit cap with rolled cuff and short brim over a head with jaw, chin, wedge nose, brow, eyes and ears; the jacket yoke widens over the shoulder joints to the acromion and the sleeve heads sit inside it, trouser knees and elbows are nested so that every pose stays closed; the gloves are fists closed around the bow handle with four fingers of three phalanges and the thumb underneath.", "de": "Körper 1:50: ein 1,82 m großer Kurier als Kleidung über dem Körper modelliert: hüftlange Softshelljacke mit Stehkragen und zwei 50 mm breiten Reflexstreifen, eingesetzte Ärmel mit Bündchenriegel, Cargohose mit Schenkeltaschen, knöchelhohe Arbeitsstiefel mit Absatz, Gelenk und Zehenkappe, Strickmütze mit Umschlag und kurzem Schirm über einem Kopf mit Kiefer, Kinn, Keilnase, Brauen, Augen und Ohren; die Jackenpasse verbreitert sich über den Schultergelenken bis zum Akromion und die Ärmelköpfe sitzen darin, Hosenknie und Ellbogen sind ineinandergesteckt, damit jede Pose geschlossen bleibt; die Handschuhe sind um den Bügelgriff geschlossene Fäuste mit vier Fingern aus drei Gliedern und dem Daumen darunter."},
    {"en": "Hand truck after the main dimensions of the fetra B1125L steel tube truck (1.15 m high, 0.57 m wide, toe plate 320 x 250 mm, pneumatic wheels 260 x 85 mm, 11 kg): 27 mm tube bow frame with three cross braces, the axle 0.14 m behind the rail plane on two 22 mm struts per side, the toe plate at ground level when upright (modelled 8 mm thick for its folded edge), tyres with round shoulders on dished rims with a 75 mm hub; three cartons stand on the plate against the rails, sealed with tape.", "de": "Sackkarre nach den Hauptmaßen der fetra-Stahlrohrkarre B1125L (1,15 m hoch, 0,57 m breit, Schaufel 320 x 250 mm, Lufträder 260 x 85 mm, 11 kg): Bügelrahmen aus 27-mm-Rohr mit drei Querstreben, die Achse 0,14 m hinter der Holmebene an zwei 22-mm-Streben je Seite, die Schaufel aufrecht auf Bodenhöhe (wegen der gekanteten Kante 8 mm dick modelliert), Reifen mit runden Schultern auf gekröpften Felgen mit 75-mm-Nabe; drei Kartons stehen auf der Schaufel an den Holmen, mit Klebeband verschlossen."},
    {"en": "Thermal 1:50: jacket as three nested shells (shirt, fleece, softshell with their real thicknesses), trousers with briefs under the upper thigh, boots with socks, single shells for gloves and cap; the cap counts with its outer dome only (its inner face lies on the scalp), brim, glove cuffs and cargo pockets are left out as unheated second layers; the trousers seat covers the pelvis, which the physiological partition counts with the trunk under the jacket region, and is not a thermal solid (the thigh shells reach up into the jacket, so the thermal figure stays closed); breath outlets are patches at the nostrils and the mouth, the body plume a patch on the crown.", "de": "Thermik 1:50: Jacke als drei geschachtelte Schalen (Hemd, Fleece, Softshell in wahrer Dicke), Hose mit Unterhose am oberen Schenkel, Stiefel mit Socken, einfache Schalen für Handschuhe und Mütze; die Mütze zählt nur mit ihrer äußeren Kuppel (die Innenseite liegt auf der Kopfhaut), Schirm, Handschuhstulpen und Cargotaschen entfallen als unbeheizte Zweitlagen; der Hosenboden bedeckt das Becken, das die physiologische Aufteilung beim Rumpf unter der Jackenregion führt, und ist kein thermischer Körper (die Schenkelschalen reichen in die Jacke hinauf, die thermische Figur bleibt geschlossen); Atemauslässe sind Flecken an Nasenlöchern und Mund, die Körperfahne ein Fleck auf dem Scheitel."},
    {"en": "Structure 1:50: the skeleton as load path: cranium, facial skeleton with zygomatic arches and mandible with body and rami, cervical spine, thoracolumbar spine with lordosis and kyphosis, six rib hoops inclined 20 degrees with sternum, clavicles and scapulae, pelvis of sacrum, ilia, pubic and ischial rami, femur with head, neck and condyles, patella, tibia and fibula, calcaneus, metatarsals and toe phalanges, humerus, radius and ulna, carpus, metacarpals and curled phalanges; the truck frame, axle and wheels and the cartons carry themselves; masses sit at the de Leva centres and the centres of truck and cartons.", "de": "Tragwerk 1:50: das Skelett als Lastpfad: Hirnschädel, Gesichtsschädel mit Jochbögen und Unterkiefer mit Körper und Ästen, Halswirbelsäule, Brust- und Lendenwirbelsäule mit Lordose und Kyphose, sechs um 20 Grad geneigte Rippenringe mit Brustbein, Schlüsselbeine und Schulterblätter, Becken aus Kreuzbein, Darmbeinen, Scham- und Sitzbeinästen, Femur mit Kopf, Hals und Kondylen, Kniescheibe, Tibia und Fibula, Fersenbein, Mittelfuß und Zehenglieder, Humerus, Radius und Ulna, Handwurzel, Mittelhand und gekrümmte Fingerglieder; Karrenrahmen, Achse und Räder sowie die Kartons tragen sich selbst; Massen sitzen in den Schwerpunkten nach de Leva und in denen von Karre und Kartons."},
]


def build():
    b = body()
    return {"status": "authored", "denominator": 50, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
