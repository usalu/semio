"""🧍 1:100 of `humans/courier-with-hand-truck`, stated anew for a 15 mm feature: garments as single masses per segment, the boot as one upper with its sole, the glove as a mitten fist around the handle, head with nose and cap with a padded brim; the truck keeps its tubes (27 and 20 mm), one strut per side and plain wheels. Frames as in the 1:50 module."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(__file__), "r3-courier-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
S, F, loft, cap, ell, box, rev, ext, both, of, mem, st = k.S, k.F, k.loft, k.cap, k.ell, k.box, k.rev, k.ext, k.both, k.of, k.mem, k.st
JACKET, WORK, BOOT, GLOVE, CAP, SKIN, STEEL, RUBBER, CARD = "hivis-softshell", "cotton-workwear", "leather-work-boot", "fabric-glove", "wool-cap", "skin", "steel-tube", "tyre-rubber", "cardboard"
BONE = (SKIN, 0.9)
PARCELS = (("a", 0.078, -0.3575, (0.40, 0.408, 0.46)), ("b", 0.428, -0.3375, (0.30, 0.368, 0.40)), ("c", 0.695, -0.2995, (0.234, 0.292, 0.36)))


def body():
    limb = [
        loft("sleeve-l", "arm-l", JACKET, 0.62, [S(-0.018, 0, -0.022, 0.030, 0.032, 2.2), S(0.03, 0, 0.004, 0.062, 0.066, 2.2), S(0.16, 0.003, 0.008, 0.058, 0.062, 2.2), S(0.3366, -0.003, 0, 0.050, 0.054, 2.2), S(0.372, -0.002, 0, 0.049, 0.051, 2.2)]),
        loft("forearm-l", "forearm-l", JACKET, 0.62, [S(-0.03, -0.002, 0, 0.041, 0.043, 2.2), S(0.05, 0, 0, 0.046, 0.046, 2.2), S(0.16, 0, 0, 0.043, 0.040, 2.3), S(0.248, 0, 0, 0.038, 0.033, 2.4)]),
        loft("glove-l", "hand-l", GLOVE, 0.36, [S(-0.02, 0, 0, 0.033, 0.025, 2.5), S(0.03, 0.003, 0, 0.044, 0.022, 2.7), S(0.075, -0.012, 0, 0.048, 0.034, 2.8), S(0.108, -0.026, 0, 0.046, 0.030, 2.6), S(0.122, -0.032, 0, 0.040, 0.018, 2.4)]),
        cap("glove-fingers-l", "hand-l", GLOVE, 0.34, [0.076, -0.056, -0.034], [0.076, -0.056, 0.034], 0.013),
        loft("thigh-l", "thigh-l", WORK, 0.4, [S(-0.02, 0.004, 0, 0.086, 0.100, 2.3), S(0.08, 0.001, -0.004, 0.090, 0.101, 2.3), S(0.24, 0.006, -0.008, 0.080, 0.088, 2.3), S(0.4437, 0.004, -0.006, 0.063, 0.069, 2.2), S(0.474, 0.004, -0.006, 0.061, 0.066, 2.2)]),
        loft("shank-l", "shank-l", WORK, 0.4, [S(-0.03, 0.004, -0.006, 0.050, 0.055, 2.2), S(0.04, 0.004, -0.004, 0.056, 0.062, 2.2), S(0.14, -0.006, 0, 0.061, 0.067, 2.2), S(0.30, -0.003, 0, 0.053, 0.059, 2.3), S(0.378, 0.004, 0, 0.054, 0.064, 2.4)]),
        loft("boot-l", "foot-l", BOOT, 0.25, [F(-0.066, 0.058, 0.040, 0, 0.031, 2.5), F(-0.03, 0.084, 0.066, 0, 0.042, 2.6), F(0.03, 0.084, 0.066, 0, 0.045, 2.6), F(0.10, 0.058, 0.040, 0.001, 0.048, 2.6), F(0.19, 0.047, 0.029, 0.003, 0.053, 2.7)]),
        loft("boot-sole-l", "foot-l", BOOT, 0.12, [F(-0.069, 0.015, 0.015, 0, 0.031, 3.2), F(-0.02, 0.015, 0.015, 0, 0.044, 3.2), F(0.07, 0.014, 0.014, 0.001, 0.047, 3.2), F(0.19, 0.012, 0.012, 0.003, 0.055, 3.2)]),
        loft("boot-toe-l", "toe-l", BOOT, 0.25, [S(-0.012, 0.038, 0.003, 0.054, 0.038, 2.7), S(0.035, 0.034, 0.003, 0.050, 0.034, 2.6), S(0.070, 0.026, 0.002, 0.030, 0.022, 2.4)]),
    ]
    return [
        loft("jacket", "torso", JACKET, 0.62, [S(-0.125, -0.002, 0, 0.187, 0.139, 2.6), S(-0.06, 0.002, 0, 0.188, 0.139, 2.6), S(0.10, -0.004, 0, 0.174, 0.123, 2.6), S(0.27, -0.008, 0, 0.179, 0.132, 2.5), S(0.37, 0.002, 0, 0.184, 0.126, 2.4), S(0.435, 0.010, 0, 0.222, 0.102, 2.2), S(0.465, 0.012, 0, 0.200, 0.078, 2.1), S(0.522, 0.002, 0, 0.074, 0.074, 2.0)]),
        loft("reflective-bands", "torso", JACKET, 0.9, [S(0.005, 0.002, 0, 0.1845, 0.135, 2.6), S(0.155, -0.005, 0, 0.178, 0.1285, 2.6)]),
        loft("trousers-seat", "pelvis", WORK, 0.4, [S(-0.105, 0, 0, 0.075, 0.085, 2.3), S(-0.03, 0.009, 0, 0.172, 0.121, 2.6), S(0.13, 0, 0, 0.160, 0.115, 2.6)]),
        loft("neck", "neck", SKIN, 0.7, [S(-0.03, 0.004, 0, 0.066, 0.064, 2.1), S(0.05, -0.002, 0, 0.059, 0.061, 2.0), S(0.125, -0.010, 0, 0.056, 0.064, 2.0)]),
        loft("head", "head", SKIN, 0.7, [S(-0.054, -0.070, 0, 0.030, 0.024, 2.0), S(-0.025, -0.042, 0, 0.053, 0.058, 2.2), S(0.03, -0.016, 0, 0.070, 0.089, 2.2), S(0.09, -0.008, 0, 0.0775, 0.098, 2.2), S(0.15, -0.003, 0, 0.066, 0.084, 2.2), S(0.174, -0.002, 0, 0.030, 0.038, 2.0)]),
        loft("nose", "head", SKIN, 0.7, [S(0.052, -0.103, 0, 0.0075, 0.0075, 2.0), S(0.012, -0.111, 0, 0.016, 0.015, 2.2), S(0.000, -0.108, 0, 0.015, 0.011, 2.2)]),
        loft("cap", "head", CAP, 0.3, [S(0.060, -0.009, 0, 0.087, 0.108, 2.2), S(0.10, -0.007, 0, 0.085, 0.106, 2.2), S(0.15, -0.003, 0, 0.072, 0.090, 2.2), S(0.180, -0.002, 0, 0.030, 0.038, 2.0)]),
        loft("cap-brim", "head", CAP, 0.22, [{"at": [0.070, -0.098, 0], "rx": 0.0075, "ry": 0.072, "exponent": 2.6}, {"at": [0.062, -0.140, 0], "rx": 0.0075, "ry": 0.064, "exponent": 2.5}, {"at": [0.054, -0.174, 0], "rx": 0.0075, "ry": 0.030, "exponent": 2.2}]),
        *both(limb),
        cap("rail-l", "hand-truck", STEEL, 0.35, [-0.115, -0.14, 0.1465], [0.975, -0.14, 0.1465], 0.0135),
        cap("rail-r", "hand-truck", STEEL, 0.35, [-0.115, -0.14, -0.1465], [0.975, -0.14, -0.1465], 0.0135),
        cap("bow-l", "hand-truck", STEEL, 0.35, [0.975, -0.14, 0.1465], [1.0065, -0.14, 0.112], 0.0135),
        cap("bow-r", "hand-truck", STEEL, 0.35, [0.975, -0.14, -0.1465], [1.0065, -0.14, -0.112], 0.0135),
        cap("handle", "hand-truck", STEEL, 0.2, [1.0065, -0.14, -0.112], [1.0065, -0.14, 0.112], 0.0135),
        *[cap(f"brace-{i}", "hand-truck", STEEL, 0.35, [u, -0.14, -0.1465], [u, -0.14, 0.1465], 0.010) for i, u in enumerate((0.20, 0.47, 0.74))],
        cap("strut-l", "hand-truck", STEEL, 0.35, [0.10, -0.14, 0.1465], [0, 0, 0.170], 0.010),
        cap("strut-r", "hand-truck", STEEL, 0.35, [0.10, -0.14, -0.1465], [0, 0, -0.170], 0.010),
        ext("plate", "hand-truck", STEEL, 0.35, [[-0.130, -0.120], [-0.130, -0.4035], [-0.115, -0.4035], [-0.115, -0.120]], "xy", -0.16, 0.16),
        cap("axle", "truck-lift", STEEL, 0.2, [0, 0, -0.279], [0, 0, 0.279], 0.010),
        rev("tyre", "truck-lift", RUBBER, 0.18, [[-0.0425, 0.058], [-0.0425, 0.108], [-0.028, 0.127], [0, 0.13], [0.028, 0.127], [0.0425, 0.108], [0.0425, 0.058], [-0.0425, 0.058]], [0, 0, 1], [0, 0, 0], transform={"translation": [0, 0, -0.2425]}, repeat={"count": 2, "translation": [0, 0, 0.485]}),
        rev("rim", "truck-lift", STEEL, 0.5, [[-0.0375, 0], [-0.0375, 0.022], [-0.016, 0.059], [0.016, 0.059], [0.0375, 0.022], [0.0375, 0]], [0, 0, 1], [0, 0, 0], transform={"translation": [0, 0, -0.2425]}, repeat={"count": 2, "translation": [0, 0, 0.485]}),
        *[box(f"parcel-{n}", "parcels", CARD, 0.52, [u, v, 0], list(size)) for n, u, v, size in PARCELS],
    ]


def thermal(b):
    region = {"head": "face-skin", "neck": "face-skin", "nose": "face-skin", "jacket": "jacket", "tyre": "tyres"}
    t = []
    for x in b:
        i = x["id"]
        if i in region:
            t.append(of(x, region[i]))
        elif i == "cap":
            t.append(of(x, "cap", sectionFaces=["-u"]))
        elif i.startswith(("sleeve", "forearm")):
            t.append(of(x, "jacket"))
        elif i.startswith("glove"):
            t.append(of(x, "gloves"))
        elif i.startswith(("thigh", "shank")):
            t.append(of(x, "trousers"))
        elif i.startswith("boot"):
            t.append(of(x, "boots"))
        elif x["bone"] == "hand-truck" or i in ("axle", "rim"):
            t.append(of(x, "hand-truck"))
        elif i.startswith("parcel"):
            t.append(of(x, "parcels"))
    return t + [
        ell("th-nose-breath", "head", SKIN, 0.5, [0.006, -0.114, 0], [0.0075, 0.0075, 0.011], region="face-skin", tags=["emitter", "nose-breath"]),
        ell("th-mouth-breath", "head", SKIN, 0.5, [-0.02, -0.098, 0], [0.0075, 0.0075, 0.014], region="face-skin", tags=["emitter", "mouth-breath"]),
        ell("th-body-plume", "head", CAP, 0.5, [0.174, -0.002, 0], [0.0075, 0.05, 0.042], region="jacket", tags=["emitter", "body-plume"]),
    ]


def structure(b):
    hoop = lambda i, u, depth, width: st(rev(f"rib-hoop-{i}", "torso", *BONE, [[0.008 * math.cos(a), depth + 0.008 * math.sin(a)] for a in [j * math.pi / 3 for j in range(6)]] + [[0.008, depth]], [1, 0, 0], [0, 0, 0], lateralScale=round(width / depth, 4), transform={"rotation": [0, 0, -0.17365, 0.98481], "translation": [u, -0.012, 0]}), "torso")
    limb = [
        st(ell("shoulder-l", "arm-l", *BONE, [0, 0, -0.004], [0.022, 0.022, 0.022]), "arm-l", "joint"),
        st(cap("humerus-l", "arm-l", *BONE, [0.012, 0, 0], [0.33, 0, 0], 0.012), "arm-l"),
        st(ell("elbow-l", "forearm-l", *BONE, [0, 0, 0], [0.016, 0.016, 0.028]), "forearm-l", "joint"),
        st(cap("forearm-bones-l", "forearm-l", *BONE, [0, -0.003, 0], [0.258, 0, 0], 0.013, 0.011, lateralScale=1.8), "forearm-l"),
        st(ell("wrist-l", "hand-l", *BONE, [0.01, 0, 0], [0.016, 0.011, 0.026]), "hand-l", "joint"),
        st(loft("hand-bones-l", "hand-l", *BONE, [S(0.025, 0.002, 0, 0.028, 0.008, 3), S(0.09, -0.006, 0, 0.040, 0.008, 3), S(0.112, -0.03, 0, 0.040, 0.008, 3)]), "hand-l"),
        st(ell("hip-l", "thigh-l", *BONE, [0, 0, 0], [0.024, 0.024, 0.024]), "thigh-l", "joint"),
        st(cap("femoral-neck-l", "thigh-l", *BONE, [0, 0, 0], [0.036, 0, 0.052], 0.016), "thigh-l"),
        st(cap("femur-l", "thigh-l", *BONE, [0.036, 0, 0.052], [0.43, 0.003, 0], 0.016, 0.02), "thigh-l"),
        st(ell("knee-l", "shank-l", *BONE, [0, 0.004, 0], [0.024, 0.032, 0.04]), "shank-l", "joint"),
        st(cap("tibia-l", "shank-l", *BONE, [0.02, 0.006, -0.005], [0.425, 0, -0.003], 0.016, 0.013), "shank-l"),
        st(cap("fibula-l", "shank-l", *BONE, [0.034, -0.016, 0.033], [0.436, -0.008, 0.027], 0.0075), "shank-l"),
        st(ell("ankle-l", "foot-l", *BONE, [0, 0, 0], [0.022, 0.018, 0.022]), "foot-l", "joint"),
        st(loft("foot-bones-l", "foot-l", *BONE, [F(-0.05, 0.05, 0.022, 0, 0.02, 2.4), F(0.02, 0.066, 0.02, 0, 0.027, 2.6), F(0.185, 0.036, 0.009, 0.003, 0.043, 3.0)]), "foot-l"),
        st(loft("toe-bones-l", "toe-l", *BONE, [S(0, 0.033, 0.003, 0.042, 0.008, 3), S(0.058, 0.028, 0.002, 0.028, 0.0075, 2.6)]), "toe-l"),
    ]
    s = [
        st(ell("skull", "head", *BONE, [0.09, -0.006, 0], [0.076, 0.09, 0.07]), "head"),
        st(ell("jaw", "head", *BONE, [-0.012, -0.056, 0], [0.036, 0.04, 0.048]), "head"),
        st(ell("atlas", "head", *BONE, [0, 0.004, 0], [0.013, 0.016, 0.024]), "head", "joint"),
        st(loft("cervical-spine", "neck", *BONE, [S(-0.012, 0.016, 0, 0.016, 0.014, 2), S(0.105, 0.004, 0, 0.013, 0.012, 2)]), "neck"),
        st(ell("c7", "neck", *BONE, [0, 0.014, 0], [0.014, 0.016, 0.02]), "neck", "joint"),
        st(loft("spine", "torso", *BONE, [S(-0.01, 0.036, 0, 0.026, 0.021, 2), S(0.08, 0.031, 0, 0.024, 0.02, 2), S(0.26, 0.066, 0, 0.019, 0.016, 2), S(0.40, 0.046, 0, 0.017, 0.015, 2), S(0.482, 0.014, 0, 0.015, 0.014, 2)]), "torso"),
        st(ell("lumbosacral", "torso", *BONE, [0, 0.036, 0], [0.015, 0.022, 0.028]), "torso", "joint"),
        *[hoop(i, u, d, w) for i, (u, d, w) in enumerate(((0.225, 0.094, 0.128), (0.30, 0.103, 0.139), (0.37, 0.094, 0.126), (0.428, 0.06, 0.082)))],
        st({**box("sternum", "torso", *BONE, [0.325, -0.098, 0], [0.185, 0.015, 0.036]), "transform": {"translation": [0.325, -0.098, 0], "rotation": [0, 0, 0.10453, 0.99452]}}, "torso"),
        *[st(cap(f"clavicle-{n}", "torso", *BONE, [0.418, -0.076, sg * 0.022], [0.455, 0.004, sg * 0.188], 0.0075), "torso") for n, sg in (("l", 1), ("r", -1))],
        *[st(ell(f"scapula-{n}", "torso", *BONE, [0.375, 0.086, sg * 0.112], [0.078, 0.009, 0.056]), "torso") for n, sg in (("l", 1), ("r", -1))],
        st(loft("sacrum", "pelvis", *BONE, [S(-0.03, 0.08, 0, 0.014, 0.011, 2.4), S(0.10, 0.044, 0, 0.052, 0.024, 2.4)]), "pelvis"),
        *[x for n, sg in (("l", 1), ("r", -1)) for x in (
            st(ell(f"ilium-{n}", "pelvis", *BONE, [0.07, 0.02, sg * 0.112], [0.07, 0.052, 0.014]), "pelvis"),
            st(cap(f"hip-bone-{n}", "pelvis", *BONE, [0.05, 0.05, sg * 0.062], [-0.03, -0.062, sg * 0.01], 0.012), "pelvis"),
            st(cap(f"ischium-{n}", "pelvis", *BONE, [-0.006, 0.008, sg * 0.09], [-0.07, 0.03, sg * 0.058], 0.012), "pelvis"),
        )],
        *both(limb),
    ]
    for x in b:
        if x["bone"] == "hand-truck":
            s.append(mem(x, "hand-truck"))
        elif x["id"] in ("axle", "tyre", "rim"):
            s.append(mem(x, "truck-lift"))
        elif x["id"].startswith("parcel-"):
            s.append(mem(x, "parcels"))
    return s + [
        st(ell("axle-pivot", "hand-truck", STEEL, 0.3, [0, 0, 0], [0.018, 0.018, 0.018]), "hand-truck", "joint"),
        st(ell("mass-torso", "torso", SKIN, 0.4, [0.2205, 0, 0], [0.07, 0.05, 0.07], "detail"), "torso", "mass"),
        st(ell("mass-pelvis", "pelvis", SKIN, 0.4, [0.0616, 0, 0], [0.05, 0.045, 0.06], "detail"), "pelvis", "mass"),
        *[st(ell(f"mass-parcel-{n}", "parcels", CARD, 0.4, [u, v, 0], [r, r, r], "detail"), "parcels", "mass") for (n, u, v, _), r in zip(PARCELS, (0.07, 0.058, 0.048))],
    ]


DECISIONS = [
    {"en": "Body 1:100, authored for a 15 mm feature: one mass per garment segment (jacket with collar, one band zone for the two reflective bands, sleeves, trouser legs without pockets), the boot as upper, sole and toe cap, the glove as a mitten fist with one finger roll under the handle, the head with nose only, the cap with a padded brim; ears, eyes, cuffs, thumb and tape are omitted.", "de": "Körper 1:100, für 15 mm Mindestmaß gezeichnet: eine Masse je Kleidungsabschnitt (Jacke mit Kragen, eine Bandzone für die zwei Reflexstreifen, Ärmel, Hosenbeine ohne Taschen), der Stiefel als Schaft, Sohle und Zehenkappe, der Handschuh als Fäustling mit einer Fingerrolle unter dem Griff, der Kopf nur mit Nase, die Mütze mit gepolstertem Schirm; Ohren, Augen, Bündchen, Daumen und Klebeband entfallen."},
    {"en": "Hand truck 1:100: rails, bow corners as one chamfer tube each, handle bar, three braces, one strut per side, toe plate generalised to 15 mm, tyres with a simple shoulder and conical rims; the three cartons are plain boxes.", "de": "Sackkarre 1:100: Holme, Bügelecken als je ein schräges Rohr, Griffstange, drei Querstreben, eine Strebe je Seite, Schaufel auf 15 mm verallgemeinert, Reifen mit einfacher Schulter und konischen Felgen; die drei Kartons sind schlichte Quader."},
    {"en": "Thermal 1:100: one surface solid per region (layers merged into the garment surface), cap with its outer dome, brim and trousers seat left out as at 1:50, breath and plume outlets as patches. Structure 1:100: skull and jaw, spine with its curvature, four rib hoops, sternum, clavicles, scapulae, pelvis of sacrum, ilia and hip bones, long bones with forearm and hand bones merged, truck frame and cartons as their own members.", "de": "Thermik 1:100: ein Flächenkörper je Region (Schichten in der Kleidungsoberfläche zusammengefasst), Mütze mit äußerer Kuppel, Schirm und Hosenboden entfallen wie bei 1:50, Atem- und Fahnenauslässe als Flecken. Tragwerk 1:100: Schädel und Kiefer, Wirbelsäule mit Krümmung, vier Rippenringe, Brustbein, Schlüsselbeine, Schulterblätter, Becken aus Kreuzbein, Darmbeinen und Hüftbeinen, Röhrenknochen mit zusammengefassten Unterarm- und Handknochen, Karrenrahmen und Kartons als eigene Stäbe."},
]


def build():
    b = body()
    return {"status": "authored", "denominator": 100, "body": b, "thermal": thermal(b), "structure": structure(b), "decisions": DECISIONS}
