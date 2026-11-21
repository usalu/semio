"""🧍 Round 5, 1:500 of `humans/courier-with-hand-truck`, stated anew for a 75 mm feature: the figure is trunk, head with cap, and two-section limbs with fists and boots; the truck is two rails with the handle, the plate, the axle and two wheels under the carton stack; the load path is a stick skeleton."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(__file__), "r3-courier-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
S, F, loft, cap, ell, box, rev, ext, both, of, mem, st = k.S, k.F, k.loft, k.cap, k.ell, k.box, k.rev, k.ext, k.both, k.of, k.mem, k.st
_fspec = importlib.util.spec_from_file_location("forms", os.path.join(os.path.dirname(__file__), "r5-courier-with-hand-truck-forms.py"))
fm = importlib.util.module_from_spec(_fspec)
_fspec.loader.exec_module(fm)
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
        loft("sleeve-l", "arm-l", JACKET, 0.62, fm.sections(S, fm.pick(fm.SLEEVE, [0, 2, 5, 9]))),
        loft("forearm-l", "forearm-l", JACKET, 0.62, fm.sections(S, fm.pick(fm.FOREARM, [0, 5]))),
        ell("glove-l", "hand-l", GLOVE, 0.36, [0.064, -0.014, 0], [0.070, 0.047, 0.050]),
        loft("thigh-l", "thigh-l", WORK, 0.4, fm.sections(S, fm.pick(fm.THIGH, [0, 3, 6]))),
        loft("shank-l", "shank-l", WORK, 0.4, fm.sections(S, fm.pick(fm.SHANK, [0, 1, 2, 5]))),
        loft("boot-l", "foot-l", BOOT, 0.25, [F(-0.068, 0.05, 0.05, 0, 0.037, 2.6), F(0.0, 0.07, 0.07, 0, 0.049, 2.6), F(0.19, 0.040, 0.040, 0.003, 0.054, 2.8)]),
        loft("boot-toe-l", "toe-l", BOOT, 0.25, [S(-0.012, 0.038, 0.003, 0.054, 0.038, 2.8), S(0.070, 0.040, 0.002, 0.044, 0.040, 2.5)]),
    ]
    return [
        loft("jacket", "torso", JACKET, 0.62, fm.sections(S, fm.pick(fm.JACKET, [0, 3, 5, 8, 10, 12]))),
        loft("neck", "neck", SKIN, 0.7, [S(-0.02, 0.004, 0, 0.062, 0.064, 2.0), S(0.125, -0.008, 0, 0.058, 0.064, 2.0)]),
        ell("head", "head", SKIN, 0.7, [0.058, -0.014, 0], [0.114, 0.098, 0.0775]),
        loft("cap", "head", CAP, 0.3, [S(0.064, -0.012, 0, 0.087, 0.112, 2.2), S(0.125, -0.007, 0, 0.081, 0.102, 2.2), S(0.163, -0.004, 0, 0.062, 0.078, 2.1), S(0.182, -0.002, 0, 0.026, 0.032, 2.0)]),
        *both(limb),
        cap("rail-l", "hand-truck", STEEL, 0.35, [-0.115, -0.14, 0.1465], [1.0065, -0.14, 0.1465], 0.0135, slender=True),
        cap("rail-r", "hand-truck", STEEL, 0.35, [-0.115, -0.14, -0.1465], [1.0065, -0.14, -0.1465], 0.0135, slender=True),
        cap("handle", "hand-truck", STEEL, 0.35, [1.0065, -0.14, -0.1465], [1.0065, -0.14, 0.1465], 0.056),
        cap("axle", "truck-lift", STEEL, 0.2, [0, 0, -0.279], [0, 0, 0.279], 0.010, slender=True),
        rev("tyre", "truck-lift", RUBBER, 0.18, [[-0.0425, 0], [-0.0425, 0.13], [0.0425, 0.13], [0.0425, 0]], [0, 0, 1], [0, 0, 0], **TWICE),
        box("parcel-a", "parcels", CARD, 0.52, [0.078, -0.3575, 0], [0.40, 0.408, 0.46]),
        box("parcel-b", "parcels", CARD, 0.52, [0.428, -0.3375, 0], [0.30, 0.368, 0.40]),
        box("parcel-c", "parcels", CARD, 0.52, [0.695, -0.2995, 0], [0.234, 0.292, 0.36]),
    ]


def structure(b):
    R = 0.0375
    limb = [
        st(cap("arm-bar-l", "arm-l", *BONE, [0.02, 0, 0], [0.33, 0, 0], R), "arm-l"),
        st(cap("forearm-bar-l", "forearm-l", *BONE, [0.01, 0, 0], [0.24, 0, 0], 0.0375, 0.034), "forearm-l"),
        st(ell("hand-bar-l", "hand-l", *BONE, [0.062, -0.012, 0], [0.05, 0.0375, 0.04]), "hand-l"),
        st(cap("thigh-bar-l", "thigh-l", *BONE, [0, 0, 0], [0.4437, 0, 0], R), "thigh-l"),
        st(cap("shank-bar-l", "shank-l", *BONE, [0, 0, 0], [0.39, 0, 0], R), "shank-l"),
        st(loft("foot-bar-l", "foot-l", *BONE, [F(-0.04, 0.04, 0.0375, 0, 0.0375, 2.4), F(0.18, 0.04, 0.0375, 0.003, 0.0375, 2.4)]), "foot-l"),
    ]
    return [
        st(ell("skull", "head", *BONE, [0.075, -0.014, 0], [0.09, 0.085, 0.065]), "head"),
        st(cap("neck-bar", "neck", *BONE, [0, 0.008, 0], [0.1026, 0.0, 0], R), "neck"),
        st(loft("spine", "torso", *BONE, [S(-0.01, 0.048, 0, R, R, 2), S(0.10, 0.040, 0, R, R, 2), S(0.28, 0.072, 0, R, R, 2), S(0.47, 0.022, 0, R, R, 2)]), "torso"),
        st(cap("shoulder-girdle", "torso", *BONE, [0.435, 0.01, -0.15], [0.435, 0.01, 0.15], R), "torso"),
        st(cap("pelvic-girdle", "pelvis", *BONE, [0.03, 0.02, -0.1], [0.03, 0.02, 0.1], R), "pelvis"),
        *both(limb),
        *carried(b),
        st(ell("mass-parcels", "parcels", CARD, 0.4, [0.2909, -0.3415, 0], [0.08, 0.08, 0.08], "detail"), "parcels", "mass"),
    ]


DECISIONS = [
    {"en": "Body 1:500, authored for a 75 mm feature: trunk with hip-length jacket and collar as one loft of six sections (hem, waist, chest, armpit, shoulder slope, neck base) with the proportions of the finer scales, neck, head as one ellipsoid under the cap dome, sleeve and forearm, fist, thigh, shank and boot with toe as one simple mass each. The truck keeps two rails at their true 27 mm and the axle (slender), and two plain wheels 260 x 85 mm; the handle bar is generalised to 112 mm, the smallest bar the print profile can show where it stands free in the plan of the parked pose, and the fists close on it; the 8 mm toe plate under the stack, struts and braces are below the feature and omitted. The three cartons stay three boxes because their stepped stack is the silhouette; the drawing merges them into one stepped outline of the mean width, merges the two rails into one 72 mm strip and draws the tyres 100 mm wide, the smallest areas the print profile can show down to 1:1000.", "de": "Körper 1:500, für 75 mm Mindestmaß modelliert: Rumpf mit hüftlanger Jacke und Kragen als ein Loft aus sechs Schnitten (Saum, Taille, Brust, Achsel, Schulterschräge, Halsansatz) in den Proportionen der feineren Maßstäbe, Hals, Kopf als ein Ellipsoid unter der Mützenkuppel, Ärmel und Unterarm, Faust, Oberschenkel, Unterschenkel und Stiefel mit Zehe als je eine einfache Masse. Die Karre behält zwei Holme in wahren 27 mm und die Achse (schlank) sowie zwei schlichte Räder 260 x 85 mm; der Griffbügel ist auf 112 mm generalisiert, den kleinsten Stab, den das Druckprofil zeigen kann, wo er im Grundriss der abgestellten Pose frei steht, und die Fäuste umschließen ihn; die 8 mm dicke Schaufel unter dem Stapel, Streben und Querstreben liegen unter dem Mindestmaß und entfallen. Die drei Kartons bleiben drei Quader, weil ihr gestufter Stapel die Silhouette ist; die Zeichnung fasst sie zu einem gestuften Umriss mittlerer Breite zusammen, fasst die zwei Holme zu einem 72 mm breiten Streifen zusammen und zeichnet die Reifen 100 mm breit, die kleinsten Flächen, die das Druckprofil bis 1:1000 zeigen kann."},
    {"en": "Thermal 1:500: one surface solid per region and outlet patches of feature size. Structure 1:500: a stick skeleton of skull, neck bar and a spine that runs dorsally with lumbar lordosis and thoracic kyphosis, shoulder and pelvic girdle and one bar per limb segment, with the truck frame, axle, wheels and cartons as their own members.", "de": "Thermik 1:500: ein Flächenkörper je Region und Auslassflecken in Mindestmaßgröße. Tragwerk 1:500: ein Stabskelett aus Schädel, Halsstab und einer dorsal verlaufenden Wirbelsäule mit Lendenlordose und Brustkyphose, Schulter- und Beckengürtel und einem Stab je Gliedmaßenabschnitt, dazu Karrenrahmen, Achse, Räder und Kartons als eigene Stäbe."},
]


def build():
    b = body()
    return {"status": "authored", "denominator": 500, "body": b, "thermal": regions(b, 0.0375), "structure": structure(b), "decisions": DECISIONS}
