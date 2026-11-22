"""🧍️ Builds `standing-desk-worker`: a tall slim woman (1.76 m barefoot, 66 kg) typing on a 15.6-inch laptop at a dual-motor sit-stand
desk raised to standing height. Body, rig and outlines are derived from `standing-adult` (scaled by F, de Leva female segment table);
desk and laptop are drawn here per scale and view with their solids; poses, clips, thermal and structure are authored here.

python3 build_desk.py [--out path]   (default: the catalog file)
"""
import copy
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw import *  # noqa: E402,F403
from fingers import RELAXED, TYPING, PRESS, TRACKPAD, both, hand  # noqa: E402
from texts_desk import TEXTS  # noqa: E402

ID = "standing-desk-worker"
F = 1.04
MASS = 66.0
BASE_MASS = 58.0

# Prop geometry in the unscaled frame of the base figure (final size = value × F). Final targets in metres: top 1.40 × 0.70 × 0.025
# at 1.008 m, column 70 × 65 mm, laptop 357 × 245 × 18 mm.
G = 1 / F
TOP_Y = 0.969
TOP_T = 0.025 * G
DESK_X0, DESK_X1 = 0.20, 0.20 + 0.70 * G
HALF_W = 0.70 * G
COL_Z = 0.60 * G
COL_X = (DESK_X0 + DESK_X1) / 2
LAP_X = 0.30
LAP_D, LAP_W, LAP_T = 0.245 * G, 0.357 * G, 0.018 * G
LID_L, LID_T = 0.232 * G, 0.006 * G


def rig_props(a):
    bones = a["rig"]["bones"]
    bones += [
        {"id": "desk-lift", "parent": "pelvis", "title": t("Floor datum of the sit-stand desk (vertical float)", "Bodenbezug des Sitz-Steh-Tischs (senkrechte Führung)"), "x": -0.903, "y": 0, "angleDeg": -90, "lengthM": 0.2, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.15, "maxM": 0.15}},
        {"id": "desk", "parent": "desk-lift", "title": t("Sit-stand desk at standing height: top, two lifting columns, T-feet and frame", "Sitz-Steh-Tisch in Stehhöhe: Platte, zwei Hubsäulen, T-Füße und Gestell"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": round(DESK_X1, 5), "joint": {"type": "fixed"}, "mass": {"kg": 37.5, "comX": round(COL_X, 4), "comY": 0.62, "distribution": "point", "source": "authored-estimate"}},
        {"id": "laptop", "parent": "desk", "title": t("Laptop base with keyboard and touchpad (15.6 inch)", "Laptop-Unterteil mit Tastatur und Touchpad (15,6 Zoll)"), "x": LAP_X, "y": TOP_Y, "angleDeg": 0, "lengthM": round(LAP_D, 5), "joint": {"type": "fixed"}, "mass": {"kg": 1.35, "comX": round(LAP_D / 2, 4), "comY": round(LAP_T / 2, 4), "distribution": "point", "source": "authored-estimate"}},
        {"id": "screen", "parent": "laptop", "title": t("Laptop lid with display", "Laptop-Deckel mit Display"), "x": round(LAP_D, 5), "y": round(LAP_T, 5), "angleDeg": 108, "lengthM": round(LID_L, 5), "joint": {"type": "revolute", "minDeg": -108, "maxDeg": 22}, "mass": {"kg": 0.55, "comX": round(LID_L / 2, 4), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]


def parts(scale):
    """🧱️ Desk and laptop parts of one scale, coarser and fewer as the scale shrinks."""
    p = []
    top = Part("desk-top", "desk", [DESK_X1 - DESK_X0, TOP_T, 2 * HALF_W], [COL_X, TOP_Y - TOP_T / 2, 0], "desk-top", "desk-laminate", 0.78, corner=0.003, sheet=scale in ("200", "500"))
    p.append(top)
    col_top = TOP_Y - TOP_T - (0.03 * G if scale in ("50", "100") else 0)
    foot_h = 0.035 * G
    for side, z in (("l", COL_Z), ("r", -COL_Z)):
        p.append(Part(f"foot-{side}", "desk", [0.65 * G, foot_h, 0.07 * G], [COL_X, foot_h / 2, z], "desk-frame", "steel-powder", 0.35, corner=0.004))
        if scale == "50":
            p.append(Part(f"column-lower-{side}", "desk", [0.08 * G, 0.50 * G, 0.065 * G], [COL_X, foot_h + 0.25 * G, z], "desk-frame", "steel-powder", 0.4, corner=0.004))
            p.append(Part(f"column-upper-{side}", "desk", [0.07 * G, col_top - foot_h - 0.48 * G, 0.055 * G], [COL_X, (foot_h + 0.48 * G + col_top) / 2, z], "desk-frame", "steel-powder", 0.45, corner=0.003))
        else:
            p.append(Part(f"column-{side}", "desk", [0.075 * G, col_top - foot_h, 0.06 * G], [COL_X, (foot_h + col_top) / 2, z], "desk-frame", "steel-powder", 0.4, corner=0.003))
        if scale in ("50", "100"):
            p.append(Part(f"rail-{side}", "desk", [0.60 * G, 0.03 * G, 0.045 * G], [COL_X, TOP_Y - TOP_T - 0.015 * G, z], "desk-frame", "steel-powder", 0.35))
        dh = (0.08 if scale == "500" else 0.05) * G
        p.append(Part(f"drive-{side}", "desk", [0.11 * G, dh, 0.085 * G], [COL_X, col_top - dh / 2, z], "desk-drive", "drive-housing", 0.3, corner=0.004))
    if scale != "500":
        p.append(Part("crossbar", "desk", [0.05 * G, 0.06 * G, 2 * COL_Z - 0.06 * G], [COL_X, TOP_Y - TOP_T - 0.03 * G - 0.03 * G, 0], "desk-frame", "steel-powder", 0.35))
    if scale == "50":
        p.append(Part("control-box", "desk", [0.07 * G, 0.03 * G, 0.16 * G], [DESK_X0 + 0.06 * G, TOP_Y - TOP_T - 0.015 * G, -0.42 * G], "desk-drive", "drive-housing", 0.3, corner=0.003))
        p.append(Part("handset", "desk", [0.05 * G, 0.022 * G, 0.12 * G], [DESK_X0 + 0.02 * G, TOP_Y - TOP_T - 0.011 * G, -0.52 * G], "desk-drive", "drive-housing", 0.25, corner=0.003))
    p.append(Part("laptop-base", "laptop", [LAP_D, LAP_T, LAP_W], [LAP_D / 2, LAP_T / 2, 0], "laptop", "laptop-aluminium", 0.68, corner=0.004))
    p.append(Part("laptop-lid", "screen", [LID_L, LID_T, LAP_W], [LID_L / 2, LID_T / 2 + 0.0005, 0], "laptop", "laptop-aluminium", 0.68, corner=0.002, sheet=scale in ("100", "200", "500")))
    if scale in ("50", "100"):
        p.append(Part("keyboard", "laptop", [0.105 * G, 0.001, 0.29 * G], [0.17 * G, LAP_T, 0], None, "laptop-aluminium", 0.55, role="detail", filled=False, views=("plan",), solid=False))
        p.append(Part("display", "screen", [LID_L - 0.03 * G, 0.001, LAP_W - 0.03 * G], [LID_L / 2 + 0.005 * G, -0.0005, 0], None, "display-glass", 0.2, role="detail", filled=False, views=("plan",), solid=False))
    if scale == "50":
        p.append(Part("touchpad", "laptop", [0.075 * G, 0.001, 0.12 * G], [0.055 * G, LAP_T, 0], None, "laptop-aluminium", 0.55, role="detail", filled=False, views=("plan",), solid=False))
        p.append(Part("edge-band", "desk", [DESK_X1 - DESK_X0, 0.004 * G, 2 * HALF_W], [COL_X, TOP_Y - TOP_T / 2, 0], None, "desk-laminate", 0.7, role="detail", filled=False, views=("elevation", "front"), solid=False))
    return p


ORDER = ["foot", "column", "crossbar", "rail", "drive", "control-box", "handset", "desk-top", "edge-band", "laptop-base", "keyboard", "touchpad", "laptop-lid", "display"]


def paint_order(ps):
    """🎨️ Authored paint order per view: in the side elevation (seen from the right) the parts on the far (left) side lie behind the person,
    the near (right) side in front of everything, the rest behind the person; in the front elevation the whole desk and laptop stand
    between viewer and person; in plan the person's arms and hands lie over desk and laptop."""
    for p in ps:
        rank = next(i for i, k in enumerate(ORDER) if p.id.startswith(k))
        side = p.id.endswith("-r") and 2 or p.id.endswith("-l") and 0 or 1
        p.zv = {"elevation": round([0.1, 0.3, 7.0][side] + rank * 0.01, 3), "front": round(7.0 + rank * 0.01, 3), "plan": round(0.1 + rank * 0.05, 3)}
    return ps


def add_props(a, frames):
    for sc in SCALES:
        ps = paint_order(parts(sc))
        body_ids = add_parts(a, sc, ps, frames, 3.0)
        vol = a["volumes"][sc]
        for p in ps:
            if p.solid:
                vol["thermal"].append({"id": f"th-{p.id}", "of": p.id, "region": p.region})
                vol["structure"].append({"id": f"st-{p.id}", "of": p.id, "member": "desk-lift" if p.id.startswith("foot-") else p.bone, "part": "member"})
        vol["structure"] += [
            {"id": "mass-desk", "kind": "ellipsoid", "bone": "desk", "material": "steel-powder", "role": "detail", "tone": 0.2, "radii": [0.045, 0.045, 0.045], "transform": {"translation": [round(COL_X, 5), 0.62, 0]}, "member": "desk", "part": "mass"},
            {"id": "mass-laptop", "kind": "ellipsoid", "bone": "laptop", "material": "laptop-aluminium", "role": "detail", "tone": 0.2, "radii": [0.02, 0.01, 0.02], "transform": {"translation": [round(LAP_D / 2, 5), round(LAP_T / 2, 5), 0]}, "member": "laptop", "part": "mass"},
            {"id": "support-desk-feet", "kind": "box", "bone": "desk", "material": "steel-powder", "role": "detail", "tone": 0.3, "size": [round(0.62 * G, 5), 0.03, round(2 * COL_Z + 0.07 * G, 5)], "transform": {"translation": [round(COL_X, 5), 0.015, 0]}, "member": "desk", "part": "support", "anchor": "desk-feet"},
        ]
        vol["thermal"].append({"id": "th-laptop-vent", "kind": "box", "bone": "laptop", "material": "laptop-aluminium", "role": "body", "tone": 0.3, "size": [0.02, round(LAP_T + 0.001, 5), 0.06], "transform": {"translation": [round(LAP_D - 0.01, 5), round(LAP_T / 2 + 0.0005, 5), 0.11]}, "region": "laptop", "tags": ["emitter", "laptop-vent"]})
        del body_ids


MATERIALS = [
    {"id": "desk-laminate", "title": t("Melamine-faced chipboard desk top", "Melaminbeschichtete Spanplatte der Tischplatte"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": 650, "source": "literature", "reference": "ashrae-2021"},
    {"id": "steel-powder", "title": t("Powder-coated steel frame", "Pulverbeschichtetes Stahlgestell"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "drive-housing", "title": t("Polymer housing of the lifting drives and control box", "Kunststoffgehäuse der Hubantriebe und der Steuerbox"), "emissivity": 0.92, "solarAbsorptivity": 0.8, "source": "authored-estimate"},
    {"id": "laptop-aluminium", "title": t("Anodised aluminium laptop shell", "Eloxierte Aluminiumschale des Laptops"), "emissivity": 0.8, "solarAbsorptivity": 0.5, "source": "authored-estimate"},
    {"id": "display-glass", "title": t("Display cover glass", "Deckglas des Displays"), "emissivity": 0.9, "solarAbsorptivity": 0.1, "source": "literature", "reference": "ashrae-2021"},
]


def regions():
    return [
        {"id": "desk-top", "title": t("Desk top, laminated board 1.40 × 0.70 m", "Tischplatte, beschichtete Platte 1,40 × 0,70 m"), "kind": "surface", "model": "passive", "material": "desk-laminate", "areaM2": 2.04, "characteristicLengthM": 0.7, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 21000, "source": "authored-estimate", "reference": "ashrae-2021"},
        {"id": "desk-frame", "title": t("Steel frame: T-feet, lifting columns, rails and crossbar", "Stahlgestell: T-Füße, Hubsäulen, Zargen und Traverse"), "kind": "surface", "model": "passive", "material": "steel-powder", "areaM2": 0.82, "characteristicLengthM": 0.07, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 9000, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "desk-drive", "title": t("Two column drives and control box (idle)", "Zwei Säulenantriebe und Steuerbox (im Ruhezustand)"), "kind": "surface", "model": "passive", "material": "drive-housing", "areaM2": 0.09, "characteristicLengthM": 0.08, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 6000, "ratedPowerW": 200, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "en-527-1-2011"},
        {"id": "laptop", "title": t("Laptop base and lid", "Laptop-Unterteil und -Deckel"), "kind": "surface", "model": "operating", "material": "laptop-aluminium", "areaM2": 0.36, "characteristicLengthM": 0.25, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 4000, "ratedPowerW": 65, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "iec-62623-2012", "operating": {"key": "laptop-load", "heatW": 25}},
    ]


def thermal_binding(a, met, breathing, laptop):
    regs = {}
    for x in a["thermal"]["regions"]:
        regs[x["id"]] = {"exposedFraction": 1, "sunlitFraction": 0}
    regs["hand-skin"] = {"exposedFraction": 0.97 if laptop > 0.5 else 1, "sunlitFraction": 0}
    regs["desk-top"] = {"exposedFraction": 0.86, "sunlitFraction": 0}
    regs["laptop"] = {"exposedFraction": 0.95, "sunlitFraction": 0}
    regs["sneakers"] = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 21}}
    return {"activityMet": met, "breathing": breathing, "operating": {"laptop-load": laptop}, "regions": regs, "emitters": {"body-plume": 1, "laptop-vent": laptop}}


LEGS = {"thigh-l": {"angleDeg": -2.5}, "shank-l": {"angleDeg": -2.5}, "foot-l": {"angleDeg": 5}, "toe-l": {"angleDeg": 0}, "thigh-r": {"angleDeg": -2.5}, "shank-r": {"angleDeg": -2.5}, "foot-r": {"angleDeg": 5}, "toe-r": {"angleDeg": 0}}
ARMS_TYPE = {"arm-r": {"angleDeg": 11.39, "abductionDeg": -3.51}, "forearm-r": {"angleDeg": 73.16, "abductionDeg": 13.8, "twistDeg": -68.68}, "hand-r": {"angleDeg": 0.9, "abductionDeg": 0.22}, "arm-l": {"angleDeg": 11.39, "abductionDeg": 3.51}, "forearm-l": {"angleDeg": 73.16, "abductionDeg": -13.8, "twistDeg": 68.68}, "hand-l": {"angleDeg": 0.9, "abductionDeg": -0.22}}
ARMS_PAD = {"arm-r": {"angleDeg": 6.23, "abductionDeg": -8.76, "twistDeg": -30.04}, "forearm-r": {"angleDeg": 75.81, "twistDeg": -61.56}, "hand-r": {"angleDeg": -3.74, "abductionDeg": -2.48}}
SLEEVE_DEPTH = (-0.03, 0.055)
CONTACTS = ["heel-l", "ball-l", "heel-r", "ball-r", "desk-feet"]


def poses(a, crossed):
    typing = {**LEGS, "lumbar": {"angleDeg": 0}, "torso": {"angleDeg": 0}, "neck": {"angleDeg": -2}, "head": {"angleDeg": -4}, **ARMS_TYPE, **both(**TYPING)}
    pad = {**typing, **ARMS_PAD, **hand("r", **TRACKPAD), "head": {"angleDeg": -10}, "neck": {"angleDeg": -5}}
    reading = {**LEGS, "lumbar": {"angleDeg": -0.8}, "torso": {"angleDeg": -1.2}, "neck": {"angleDeg": -6}, "head": {"angleDeg": -12}, **{k: v for k, v in crossed.items() if k.split("-")[0] in ("arm", "forearm", "hand", "clavicle", "thumb", "index", "middle", "ring", "little")}}
    out = []
    for pid, title, desc, bones, met, br, lap in [
        ("typing", t("Typing at the standing desk", "Tippen am Stehpult"), t("Upright at the raised desk, feet at hip width, upper arms hanging close to the trunk, elbows at about 100 degrees, forearms pronated so that the palms face the keys and the curled fingers rest on the home row; head bowed to the screen.", "Aufrecht am hochgefahrenen Tisch, Füße hüftbreit, die Oberarme hängen nah am Rumpf, Ellbogen etwa 100 Grad gebeugt, die Unterarme so gedreht, dass die Handflächen zur Tastatur zeigen und die gekrümmten Finger auf der Grundreihe liegen; der Kopf zum Bildschirm geneigt."), typing, 1.5, "desk-work", 1),
        ("touchpad-scrolling", t("Scrolling on the touchpad", "Scrollen auf dem Touchpad"), t("The right hand has moved to the touchpad, index finger stretched on the pad and the other fingers tucked; the left hand stays on the keys; head a little lower toward the screen.", "Die rechte Hand liegt auf dem Touchpad, der Zeigefinger gestreckt auf der Fläche, die übrigen Finger eingerollt; die linke Hand bleibt auf den Tasten; der Kopf etwas tiefer zum Bildschirm."), pad, 1.4, "desk-work", 1),
        ("reading-arms-crossed", t("Reading with arms crossed", "Lesen mit verschränkten Armen"), t("Hands off the keyboard, arms folded across the belly, weight on both feet; the person reads the screen with the head bowed and the laptop idling.", "Die Hände von der Tastatur genommen, die Arme vor dem Bauch verschränkt, das Gewicht auf beiden Füßen; die Person liest am Bildschirm mit geneigtem Kopf, der Laptop läuft im Leerlauf."), reading, 1.3, "quiet-standing", 0.35),
    ]:
        out.append({"id": pid, "title": title, "description": desc, "bones": bones, "ground": "auto", "contacts": CONTACTS, "thermal": thermal_binding(a, met, br, lap), "loads": []})
    return out


def full(b):
    return {k: dict(v) for k, v in b.items()}


def clips(typing_pose, reading_pose):
    ty = typing_pose["bones"]
    keys = []
    press_r, press_l = hand("r", **PRESS), hand("l", **PRESS)
    rest_r, rest_l = hand("r", **TYPING), hand("l", **TYPING)
    seq = [(0.0, None), (0.5, "r"), (1.0, "l"), (1.5, "r"), (2.0, None), (2.5, "l"), (3.0, "r"), (3.5, "l"), (4.0, None)]
    for i, (tt, side) in enumerate(seq):
        b = full(ty)
        if side == "r":
            b.update(press_r)
            b["hand-r"] = {"angleDeg": 0.9 - 3, "abductionDeg": 0.22}
        elif side == "l":
            b.update(press_l)
            b["hand-l"] = {"angleDeg": 0.9 - 3, "abductionDeg": -0.22}
        else:
            b.update(rest_r)
            b.update(rest_l)
        b["head"] = {"angleDeg": -4 + (2.5 if 2.0 <= tt < 3.5 else 0)}
        keys.append({"t": tt, "pose": "typing", "ease": "smooth", "bones": b})
    typing = {"id": "typing", "title": t("Typing", "Tippen"), "description": t("A four-second typing loop: the fingers of the right and left hand strike in turn with a small dip of the wrist, the head lifts to the screen and settles again; feet and desk stay put.", "Eine Tippschleife von vier Sekunden: die Finger der rechten und linken Hand schlagen abwechselnd an, das Handgelenk senkt sich dabei leicht, der Kopf hebt sich zum Bildschirm und senkt sich wieder; Füße und Tisch bleiben, wo sie sind."), "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
              "contacts": [{"anchor": c, "fromS": 0, "toS": 4.0} for c in CONTACTS], "phases": [{"id": "burst", "fromS": 0, "toS": 2.0, "title": t("Typing burst", "Tippphase")}, {"id": "glance", "fromS": 2.0, "toS": 3.5, "title": t("Glance at the screen while typing on", "Blick zum Bildschirm beim Weitertippen")}, {"id": "settle", "fromS": 3.5, "toS": 4.0, "title": t("Back to the keys", "Zurück zu den Tasten")}],
              "events": [{"t": 2.0, "kind": "motion", "title": t("Head lifts to the screen", "Kopf hebt sich zum Bildschirm")}, {"t": 3.5, "kind": "motion", "title": t("Head bows to the keys", "Kopf neigt sich zur Tastatur")}],
              "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4.0, "v": 0}]}, {"target": "operating:laptop-load", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 2.0, "v": 0.85}, {"t": 4.0, "v": 1}]}, {"target": "emitter:laptop-vent", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 2.0, "v": 0.85}, {"t": 4.0, "v": 1}]}]}
    rb = reading_pose["bones"]
    idle_keys = []
    for tt, dt, dh, dl in [(0, 0, 0, 0), (1.25, -0.4, 1.6, -0.3), (2.5, 0.3, 0.4, 0.2), (3.75, -0.3, 1.2, -0.2), (5.0, 0, 0, 0)]:
        b = full(rb)
        b["torso"] = {"angleDeg": rb["torso"]["angleDeg"] + dt}
        b["head"] = {"angleDeg": rb["head"]["angleDeg"] + dh}
        b["lumbar"] = {"angleDeg": rb["lumbar"]["angleDeg"] + dl}
        idle_keys.append({"t": tt, "pose": "reading-arms-crossed", "ease": "smooth", "bones": b})
    idle = {"id": "idle-breathing", "title": t("Idle breathing while reading", "Ruhiges Atmen beim Lesen"), "description": t("A five-second loop at the desk with the arms crossed: the chest rises and falls with quiet breathing and the head sways a little while reading; the laptop idles.", "Eine Schleife von fünf Sekunden am Tisch mit verschränkten Armen: die Brust hebt und senkt sich mit ruhigem Atmen, der Kopf wiegt sich beim Lesen leicht; der Laptop läuft im Leerlauf."), "durationS": 5.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": idle_keys,
            "contacts": [{"anchor": c, "fromS": 0, "toS": 5.0} for c in CONTACTS], "phases": [{"id": "idle", "fromS": 0, "toS": 5.0, "title": t("Quiet reading", "Ruhiges Lesen")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 5.0, "v": 1}]}, {"target": "operating:laptop-load", "ease": "step", "keys": [{"t": 0, "v": 0.35}, {"t": 5.0, "v": 0.35}]}, {"target": "emitter:laptop-vent", "ease": "step", "keys": [{"t": 0, "v": 0.35}, {"t": 5.0, "v": 0.35}]}]}
    return [typing, idle]


def build(out):
    d = load("standing-adult")
    a = d["asset"]
    crossed = {p["id"]: p for p in a["poses"]}["arms-crossed"]["bones"]
    a["id"] = ID
    rig_props(a)
    tmp = f"{GEN}/tmp/{ID}-rig.json"
    save(d, tmp)
    frames = rest_frames(tmp)
    add_props(a, frames)
    a["representations"]["500"]["pens"]["body"] = 0.18
    a["materials"] += [m for m in MATERIALS if m["id"] not in {x["id"] for x in a["materials"]}]
    a["anchors"] += [
        {"id": "desk-feet", "bone": "desk-lift", "x": round(COL_X, 5), "y": 0, "kind": "contact", "title": t("Desk feet on the floor", "Tischfüße am Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": round(0.65 * G, 4), "footprintWidthM": round(2 * COL_Z + 0.07 * G, 4), "frictionCoefficient": 0.6},
        {"id": "laptop-vent", "bone": "laptop", "x": round(LAP_D, 5), "y": round(LAP_T / 2, 5), "z": 0.11, "kind": "emitter", "title": t("Laptop exhaust vent at the hinge", "Lüftungsschlitz des Laptops am Scharnier")},
    ]
    th = a["thermal"]
    th["regions"] += regions()
    th["operating"] = [{"id": "laptop-load", "title": t("Laptop load", "Laptop-Last"), "levels": [{"v": 0, "title": t("Off", "Aus")}, {"v": 0.35, "title": t("Idle, screen on", "Leerlauf, Bildschirm an")}, {"v": 1, "title": t("Typing and browsing", "Tippen und Surfen")}]}]
    th["emitters"].append({"id": "laptop-vent", "kind": "fan", "title": t("Laptop fan exhaust", "Abluft des Laptop-Lüfters"), "anchor": "laptop-vent", "directionDeg": 0, "diameterM": 0.03, "operatingKey": "laptop-load", "volumeFlowM3S": 0.002, "temperatureRiseK": 10, "source": "authored-estimate", "reference": "ashrae-2021"})
    resp = th["respiration"]
    resp["patterns"] = [x for x in resp["patterns"] if x["id"] == "quiet-standing"]
    resp["patterns"].insert(0, {"id": "desk-work", "title": t("Standing desk work", "Arbeit am Stehpult"), "rateBpm": 14, "tidalVolumeL": 0.6, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"})
    a["poses"] = poses(a, crossed)
    a["clips"] = clips(a["poses"][0], a["poses"][2])
    a["presentation"] = {"defaultPose": "typing", "defaultClip": "typing", "defaultScale": 100}
    a["structure"]["supports"] = CONTACTS
    a["structure"]["dynamics"] = "quasi-static"
    scale_asset(a, F, MASS / BASE_MASS)
    for b in a["rig"]["bones"]:
        if b["id"] in ("desk", "laptop", "screen") and "mass" in b:
            b["mass"]["kg"] = {"desk": 37.5, "laptop": 1.35, "screen": 0.55}[b["id"]]
    body = th["body"]
    stature = 1.69 * F
    body["massKg"] = MASS
    body["skinAreaM2"] = round(dubois(MASS, stature), 3)
    body["basalMetabolismW"] = round(mifflin(MASS, stature, 34, False), 1)
    TEXTS(a, stature)
    apply_ops(a, ID, keep_solids=("arm-l", "arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r"))
    save(d, tmp)
    frames = rest_frames(tmp)
    ops50 = {(o["view"], o["id"]): o["shape"]["points"] for o in json.load(open(f"{W}/ops/{ID}-50.json")) if o["op"] == "shape"}
    for side in ("l", "r"):
        sweep_trace(a, frames, ("50", "100"), f"sleeve-{side}", ops50[("front", f"sleeve-{side}")], SLEEVE_DEPTH)
    prune_materials(a)
    save(d, out)
    dims = json.loads(bun(f"{W}/dims.ts", out))
    a["dimensions"] = {"heightM": dims["heightM"], "widthM": dims["widthM"], "depthM": dims["depthM"], "massKg": dims["massKg"]}
    save(d, out)
    return d


if __name__ == "__main__":
    out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else f"{CAT}/{ID}/🔣️.json"
    build(out)
    print(out)
