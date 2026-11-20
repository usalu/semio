"""🧍 1:200 of `humans/courier-with-hand-truck`, stated anew for a 30 mm feature: trunk, head, cap and each limb segment as one simple mass, the fist as one ellipsoid, the boot with its sole in one piece; the truck is two rails with the handle bar, a strut per side, the plate and disc wheels; the skeleton is reduced to spine, two rib hoops, girdles and one bar per limb segment."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(__file__), "r3-courier-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
S, F, loft, cap, ell, box, rev, ext, both, of, mem, st = k.S, k.F, k.loft, k.cap, k.ell, k.box, k.rev, k.ext, k.both, k.of, k.mem, k.st
JACKET, WORK, BOOT, GLOVE, CAP, SKIN, STEEL, RUBBER, CARD = "hivis-softshell", "cotton-workwear", "leather-work-boot", "fabric-glove", "wool-cap", "skin", "steel-tube", "tyre-rubber", "cardboard"
BONE = (SKIN, 0.9)
TWICE = {"transform": {"translation": [0, 0, -0.2425]}, "repeat": {"count": 2, "translation": [0, 0, 0.485]}}


def regions(b, patch):
    t = []
    for x in b:
        i = x["id"]
        if i in ("head", "neck"):
            t.append(of(x, "face-skin"))
        elif i == "cap":
            t.append(of(x, "cap", sectionFaces=["-u"]))
        elif i == "jacket" or i.startswith(("sleeve", "forearm")):
            t.append(of(x, "jacket"))
        elif i.startswith("glove"):
            t.append(of(x, "gloves"))
        elif i.startswith(("thigh", "shank")):
            t.append(of(x, "trousers"))
        elif i.startswith("boot"):
            t.append(of(x, "boots"))
        elif i == "tyre":
            t.append(of(x, "tyres"))
        elif x["bone"] == "hand-truck" or i == "axle":
            t.append(of(x, "hand-truck"))
        elif i.startswith("parcel"):
            t.append(of(x, "parcels"))
    r = patch
    return t + [
        ell("th-nose-breath", "head", SKIN, 0.5, [0.012, -0.112 + r, 0], [r, r, r], region="face-skin", tags=["emitter", "nose-breath"]),
        ell("th-mouth-breath", "head", SKIN, 0.5, [-0.022, -0.096 + r, 0], [r, r, r], region="face-skin", tags=["emitter", "mouth-breath"]),
        ell("th-body-plume", "head", CAP, 0.5, [0.18 - r, -0.002, 0], [r, 0.05, 0.042], region="jacket", tags=["emitter", "body-plume"]),
    ]


def carried(b):
    s = []
    for x in b:
        if x["bone"] == "hand-truck":
            s.append(mem(x, "hand-truck"))
        elif x["id"] in ("axle", "tyre"):
            s.append(mem(x, "truck-lift"))
        elif x["id"].startswith("parcel-"):
            s.append(mem(x, "parcels"))
    return s



def body():
    limb = [
        loft("sleeve-l", "arm-l", JACKET, 0.62, [S(-0.018, 0, -0.022, 0.030, 0.032, 2.2), S(0.04, 0, 0.005, 0.062, 0.066, 2.2), S(0.37, -0.003, 0, 0.049, 0.052, 2.2)]),
        loft("forearm-l", "forearm-l", JACKET, 0.62, [S(-0.03, 0, 0, 0.042, 0.044, 2.2), S(0.06, 0, 0, 0.046, 0.046, 2.2), S(0.262, 0, 0, 0.038, 0.034, 2.3)]),
        ell("glove-l", "hand-l", GLOVE, 0.36, [0.064, -0.016, 0], [0.070, 0.042, 0.048]),
        loft("thigh-l", "thigh-l", WORK, 0.4, [S(-0.02, 0.004, 0, 0.087, 0.100, 2.3), S(0.14, 0.003, -0.005, 0.088, 0.098, 2.3), S(0.47, 0.004, -0.006, 0.061, 0.066, 2.2)]),
        loft("shank-l", "shank-l", WORK, 0.4, [S(-0.03, 0.004, -0.005, 0.050, 0.056, 2.2), S(0.12, -0.005, 0, 0.061, 0.067, 2.2), S(0.378, 0.003, 0, 0.053, 0.062, 2.3)]),
        loft("boot-l", "foot-l", BOOT, 0.25, [F(-0.068, 0.05, 0.05, 0, 0.032, 2.6), F(0.0, 0.074, 0.074, 0, 0.045, 2.6), F(0.09, 0.046, 0.046, 0.001, 0.049, 2.7), F(0.19, 0.037, 0.037, 0.003, 0.054, 2.8)]),
        loft("boot-toe-l", "toe-l", BOOT, 0.25, [S(-0.012, 0.037, 0.003, 0.054, 0.037, 2.8), S(0.04, 0.032, 0.003, 0.049, 0.032, 2.6), S(0.070, 0.024, 0.002, 0.032, 0.022, 2.4)]),
    ]
    return [
        loft("jacket", "torso", JACKET, 0.62, [S(-0.125, -0.002, 0, 0.187, 0.139, 2.6), S(0.0, 0.004, 0, 0.181, 0.130, 2.6), S(0.27, -0.008, 0, 0.179, 0.132, 2.5), S(0.435, 0.010, 0, 0.222, 0.102, 2.2), S(0.522, 0.004, 0, 0.074, 0.074, 2.0)]),
        loft("trousers-seat", "pelvis", WORK, 0.4, [S(-0.105, 0, 0, 0.078, 0.086, 2.3), S(0.0, 0.008, 0, 0.172, 0.120, 2.6)]),
        loft("neck", "neck", SKIN, 0.7, [S(-0.03, 0.004, 0, 0.064, 0.064, 2.1), S(0.125, -0.008, 0, 0.057, 0.064, 2.0)]),
        loft("head", "head", SKIN, 0.7, [S(-0.055, -0.066, 0, 0.032, 0.030, 2.0), S(0.0, -0.03, 0, 0.066, 0.086, 2.2), S(0.09, -0.008, 0, 0.0775, 0.100, 2.2), S(0.174, -0.002, 0, 0.034, 0.044, 2.0)]),
        loft("cap", "head", CAP, 0.3, [S(0.062, -0.012, 0, 0.087, 0.112, 2.2), S(0.12, -0.006, 0, 0.083, 0.104, 2.2), S(0.180, -0.002, 0, 0.034, 0.044, 2.0)]),
        *both(limb),
        cap("rail-l", "hand-truck", STEEL, 0.35, [-0.115, -0.14, 0.1465], [1.0065, -0.14, 0.1465], 0.0135, slender=True),
        cap("rail-r", "hand-truck", STEEL, 0.35, [-0.115, -0.14, -0.1465], [1.0065, -0.14, -0.1465], 0.0135, slender=True),
        cap("handle", "hand-truck", STEEL, 0.2, [1.005, -0.14, -0.1465], [1.005, -0.14, 0.1465], 0.015),
        cap("strut-l", "hand-truck", STEEL, 0.35, [0.10, -0.14, 0.1465], [0, 0, 0.165], 0.015),
        cap("strut-r", "hand-truck", STEEL, 0.35, [0.10, -0.14, -0.1465], [0, 0, -0.165], 0.015),
        ext("plate", "hand-truck", STEEL, 0.35, [[-0.130, -0.120], [-0.130, -0.4035], [-0.100, -0.4035], [-0.100, -0.120]], "xy", -0.16, 0.16),
        cap("axle", "truck-lift", STEEL, 0.2, [0, 0, -0.279], [0, 0, 0.279], 0.010, slender=True),
        rev("tyre", "truck-lift", RUBBER, 0.18, [[-0.0425, 0], [-0.0425, 0.112], [-0.022, 0.13], [0.022, 0.13], [0.0425, 0.112], [0.0425, 0]], [0, 0, 1], [0, 0, 0], **TWICE),
        box("parcel-a", "parcels", CARD, 0.52, [0.078, -0.3575, 0], [0.40, 0.408, 0.46]),
        box("parcel-b", "parcels", CARD, 0.52, [0.428, -0.3375, 0], [0.30, 0.368, 0.40]),
        box("parcel-c", "parcels", CARD, 0.52, [0.695, -0.2995, 0], [0.234, 0.292, 0.36]),
    ]


def structure(b):
    hoop = lambda i, u, depth, width: st(rev(f"rib-hoop-{i}", "torso", *BONE, [[0.015, depth - 0.015], [0.015, depth + 0.015], [-0.015, depth + 0.015], [-0.015, depth - 0.015], [0.015, depth - 0.015]], [1, 0, 0], [0, 0, 0], lateralScale=round(width / depth, 4), transform={"rotation": [0, 0, -0.17365, 0.98481], "translation": [u, -0.012, 0]}), "torso")
    limb = [
        st(cap("humerus-l", "arm-l", *BONE, [0, 0, 0], [0.3366, 0, 0], 0.015), "arm-l"),
        st(cap("forearm-bones-l", "forearm-l", *BONE, [0, 0, 0], [0.252, 0, 0], 0.015), "forearm-l"),
        st(ell("hand-bones-l", "hand-l", *BONE, [0.06, -0.008, 0], [0.05, 0.016, 0.038]), "hand-l"),
        st(cap("femur-l", "thigh-l", *BONE, [0.03, 0, 0.045], [0.4437, 0, 0], 0.017), "thigh-l"),
        st(cap("femoral-neck-l", "thigh-l", *BONE, [0, 0, 0], [0.03, 0, 0.045], 0.017), "thigh-l"),
        st(cap("tibia-l", "shank-l", *BONE, [0, 0, 0], [0.4335, 0, 0], 0.016), "shank-l"),
        st(loft("foot-bones-l", "foot-l", *BONE, [F(-0.05, 0.05, 0.02, 0, 0.022, 2.4), F(0.02, 0.064, 0.02, 0, 0.028, 2.6), F(0.185, 0.04, 0.015, 0.003, 0.043, 3.0)]), "foot-l"),
    ]
    return [
        st(ell("skull", "head", *BONE, [0.075, -0.014, 0], [0.095, 0.092, 0.07]), "head"),
        st(cap("cervical-spine", "neck", *BONE, [0, 0.012, 0], [0.1026, 0.004, 0], 0.016), "neck"),
        st(loft("spine", "torso", *BONE, [S(-0.01, 0.036, 0, 0.026, 0.021, 2), S(0.26, 0.064, 0, 0.02, 0.017, 2), S(0.482, 0.014, 0, 0.016, 0.015, 2)]), "torso"),
        hoop(0, 0.25, 0.092, 0.128),
        hoop(1, 0.36, 0.088, 0.124),
        st(cap("shoulder-girdle", "torso", *BONE, [0.44, 0.0, -0.19], [0.44, 0.0, 0.19], 0.015), "torso"),
        *[st(ell(f"ilium-{n}", "pelvis", *BONE, [0.068, 0.022, sg * 0.108], [0.07, 0.054, 0.016]), "pelvis") for n, sg in (("l", 1), ("r", -1))],
        st(cap("pubic-arch", "pelvis", *BONE, [-0.01, -0.03, -0.095], [-0.01, -0.03, 0.095], 0.016), "pelvis"),
        st(loft("sacrum", "pelvis", *BONE, [S(-0.03, 0.078, 0, 0.016, 0.015, 2.4), S(0.1026, 0.044, 0, 0.052, 0.024, 2.4)]), "pelvis"),
        *both(limb),
        *carried(b),
        st(ell("mass-torso", "torso", SKIN, 0.4, [0.2205, 0, 0], [0.07, 0.05, 0.07], "detail"), "torso", "mass"),
        st(ell("mass-parcels", "parcels", CARD, 0.4, [0.2909, -0.3415, 0], [0.08, 0.08, 0.08], "detail"), "parcels", "mass"),
    ]


DECISIONS = [
    {"en": "Body 1:200, authored for a 30 mm feature: jacket with collar as one trunk mass, the trousers seat, neck, head and cap as simple lofts, three sections per limb segment, the fist as one ellipsoid on the handle, the boot with its sole in one piece and the toe cap; bands, nose, brim, pockets and fingers are omitted. The truck is two rails at their true 27 mm (slender), the handle bar, one strut per side, the toe plate generalised to 30 mm, the axle and disc wheels 260 x 85 mm; braces, bow corners and rims are omitted.", "de": "Körper 1:200, für 30 mm Mindestmaß gezeichnet: Jacke mit Kragen als eine Rumpfmasse, Hosenboden, Hals, Kopf und Mütze als einfache Lofts, drei Schnitte je Gliedmaßenabschnitt, die Faust als ein Ellipsoid am Griff, der Stiefel mit Sohle in einem Stück und die Zehenkappe; Streifen, Nase, Schirm, Taschen und Finger entfallen. Die Karre besteht aus zwei Holmen in wahren 27 mm (schlank), der Griffstange, einer Strebe je Seite, der auf 30 mm verallgemeinerten Schaufel, der Achse und Scheibenrädern 260 x 85 mm; Querstreben, Bügelecken und Felgen entfallen."},
    {"en": "Thermal 1:200: one surface solid per region, breath and plume outlets as patches of feature size. Structure 1:200: skull, cervical and trunk spine, two rib hoops inclined 20 degrees, shoulder girdle, a pelvis of two iliac blades, pubic arch and sacrum, one bar per limb segment with the femoral neck, foot plate, truck frame, axle, wheels and cartons as their own members.", "de": "Thermik 1:200: ein Flächenkörper je Region, Atem- und Fahnenauslässe als Flecken in Mindestmaßgröße. Tragwerk 1:200: Schädel, Hals- und Rumpfwirbelsäule, zwei um 20 Grad geneigte Rippenringe, Schultergürtel, ein Becken aus zwei Darmbeinschaufeln, Schambogen und Kreuzbein, ein Stab je Gliedmaßenabschnitt mit Schenkelhals, Fußplatte, Karrenrahmen, Achse, Räder und Kartons als eigene Stäbe."},
]


def build():
    b = body()
    return {"status": "authored", "denominator": 200, "body": b, "thermal": regions(b, 0.015), "structure": structure(b), "decisions": DECISIONS}
