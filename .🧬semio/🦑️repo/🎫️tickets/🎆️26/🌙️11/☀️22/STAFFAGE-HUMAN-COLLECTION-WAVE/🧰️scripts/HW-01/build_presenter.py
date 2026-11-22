"""🧍️ Builds `presenter-at-flipchart`: a workshop presenter of 1.70 m (68 kg) in softshell jacket, cap and lanyard badge beside a tripod
flipchart, marker raised to the paper, the free hand gesturing to the audience. Body, rig and outlines are derived from `tour-guide`
(scaled by F); flag and pointer are removed; marker and flipchart are drawn here per scale and view with their solids; poses, clips,
thermal and structure are authored here.

python3 build_presenter.py [--out path]
"""
import math
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from hw import *  # noqa: E402,F403
from texts_presenter import TEXTS  # noqa: E402

ID = "presenter-at-flipchart"
F = 1.03
MASS = 82.0
BASE_MASS = 78.59
G = 1 / F
ROOT_Y = 0.9231

TILT = 8.0
FOOT_X = 0.17
BOARD_Y0, BOARD_H, BOARD_W = 0.97, 1.00 * G, 0.70 * G
BOARD_Z = -0.85
LEG_T, LEG_W = 0.03 * G, 0.028 * G
LEG_L = 2.02 * G
REAR_FOOT_X = FOOT_X - 0.62 * G
MARKER_L, MARKER_D = 0.14 * G, 0.018 * G


def rig_props(a):
    up = 90 + TILT
    top = (FOOT_X - (BOARD_Y0 + BOARD_H) * math.tan(math.radians(TILT)), BOARD_Y0 + BOARD_H)
    rear_ang = math.degrees(math.atan2(top[1], top[0] - REAR_FOOT_X))
    rear_len = math.hypot(top[1], top[0] - REAR_FOOT_X)
    a["rig"]["bones"] += [
        {"id": "easel-lift", "parent": "pelvis", "title": t("Floor datum of the flipchart easel (vertical float)", "Bodenbezug des Flipchart-Gestells (senkrechte Führung)"), "x": -ROOT_Y, "y": 0, "angleDeg": -90, "lengthM": 0.2, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.15, "maxM": 0.15}},
        {"id": "easel", "parent": "easel-lift", "title": t("Tripod flipchart easel: two front legs in the board plane", "Dreibein-Flipchart: zwei Vorderbeine in der Tafelebene"), "x": round(FOOT_X, 5), "y": 0, "z": round(BOARD_Z + BOARD_W / 2, 5), "angleDeg": up, "lengthM": round(LEG_L, 5), "joint": {"type": "fixed"}, "mass": {"kg": 4.6, "comX": round(LEG_L * 0.55, 4), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "rear-leg", "parent": "easel-lift", "title": t("Rear leg of the easel, hinged under the board head", "Hinterbein des Gestells, unter dem Tafelkopf angelenkt"), "x": round(REAR_FOOT_X, 5), "y": 0, "z": round(BOARD_Z + BOARD_W / 2, 5), "angleDeg": round(rear_ang, 3), "lengthM": round(rear_len, 5), "joint": {"type": "fixed"}, "mass": {"kg": 0.9, "comX": round(rear_len / 2, 4), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "marker", "parent": "hand-r", "title": t("Whiteboard marker held in the right hand", "Flipchart-Marker in der rechten Hand"), "x": 0.07, "y": 0.02, "z": 0.024, "angleDeg": 0, "lengthM": round(MARKER_L, 5), "joint": {"type": "fixed"}, "mass": {"kg": 0.015, "comX": round(MARKER_L / 2, 4), "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]
    return rear_len


def parts(scale, rear_len):
    """🧱️ Flipchart and marker parts of one scale: the easel along its front legs (u up the board, v back, w lateral)."""
    p = []
    y0 = BOARD_Y0 / math.cos(math.radians(TILT))
    half = BOARD_W / 2
    for side, w in (("l", half + LEG_W / 2), ("r", -half - LEG_W / 2)):
        p.append(Part(f"leg-{side}", "easel", [LEG_L, LEG_T, LEG_W], [LEG_L / 2, LEG_T / 2, w], "easel-frame", "aluminium-anodised", 0.5, corner=0.002 if scale == "50" else 0, slender=True))
        if scale == "50":
            p.append(Part(f"glide-{side}", "easel", [0.03 * G, 0.04 * G, 0.04 * G], [0.015 * G, LEG_T / 2, w], "easel-frame", "glide-rubber", 0.25, corner=0.003))
    p.append(Part("board", "easel", [BOARD_H, 0.018 * G, BOARD_W], [y0 + BOARD_H / 2, 0.009 * G, 0], "board", "board-melamine", 0.72, corner=0.002, sheet=scale in ("200", "500")))
    pad_h = 0.98 * G if scale != "500" else BOARD_H
    p.append(Part("paper-pad", "easel", [pad_h, 0.006 * G, 0.65 * G], [y0 + BOARD_H - pad_h / 2 - (0.01 * G if scale != "500" else 0), -0.003 * G, 0], "paper", "paper", 0.92, sheet=True))
    if scale != "500":
        p.append(Part("tray", "easel", [0.03 * G, 0.075 * G, BOARD_W], [y0 - 0.015 * G, -0.025 * G, 0], "easel-frame", "aluminium-anodised", 0.45))
        p.append(Part("clamp-bar", "easel", [0.04 * G, 0.022 * G, 0.70 * G], [y0 + BOARD_H - 0.03 * G, -0.017 * G, 0], "easel-frame", "aluminium-anodised", 0.4, corner=0.003))
    p.append(Part("rear-tube", "rear-leg", [rear_len, LEG_T, LEG_W], [rear_len / 2, 0, 0], "easel-frame", "aluminium-anodised", 0.5, corner=0.002 if scale == "50" else 0, slender=True))
    if scale in ("50", "100"):
        for i, (yy, ww) in enumerate(((1.62, 0.42), (1.49, 0.30), (1.36, 0.36))):
            p.append(Part(f"writing-{i + 1}", "easel", [0.03 * G, 0.001, ww * G], [yy / math.cos(math.radians(TILT)) * G * F, -0.0065 * G, 0.04 * G - (0.42 - ww) / 2 * G], None, "paper", 0.3, role="detail", filled=False, views=("front",), solid=False))
    if scale != "500":
        p.append(Part("marker-body", "marker", [MARKER_L * (0.78 if scale == "50" else 1), MARKER_D, MARKER_D], [MARKER_L * (0.39 if scale == "50" else 0.5), 0, 0], None, "marker-polymer", 0.2, corner=0.004 if scale == "50" else 0))
    if scale == "50":
        p.append(Part("marker-cap", "marker", [MARKER_L * 0.22, MARKER_D * 0.9, MARKER_D * 0.9], [MARKER_L * 0.89, 0, 0], None, "marker-polymer", 0.05))
    for x in p:
        if x.bone == "marker":
            x.swept = True
        else:
            x.at = [x.at[0], x.at[1], x.at[2] - BOARD_W / 2]
    return p


ORDER = ["glide", "leg", "rear-tube", "board", "tray", "paper-pad", "clamp-bar", "writing", "marker-body", "marker-cap"]


def paint_order(ps):
    """🎨️ Authored paint order per view: seen from the right the easel stands nearer than the person, so it is painted over them, the far
    front leg first; seen from the audience and from above the person stands in front of (over) the easel, the marker over everything."""
    for p in ps:
        rank = next(i for i, k in enumerate(ORDER) if p.id.startswith(k))
        far = p.id.endswith("-l")
        mk = p.id.startswith("marker")
        p.zv = {"elevation": round((9.0 if mk else (7.0 if not far else 6.9)) + rank * 0.01, 3), "front": round((9.0 if mk else 0.1) + rank * 0.01, 3), "plan": round((9.0 if mk else 0.1) + rank * 0.01, 3)}
    return ps


def add_props(a, frames, rear_len):
    for sc in SCALES:
        ps = paint_order(parts(sc, rear_len))
        add_parts(a, sc, ps, frames, 3.0)
        vol = a["volumes"][sc]
        for p in ps:
            if p.solid and p.region:
                vol["thermal"].append({"id": f"th-{p.id}", "of": p.id, "region": p.region})
            if p.solid:
                vol["structure"].append({"id": f"st-{p.id}", "of": p.id, "member": p.bone, "part": "member"})
        if sc == "500":
            for x in vol["structure"]:
                if x.get("member") == "hand-r" and x.get("part") == "member":
                    x["members"] = sorted(set(x.get("members", [])) | {"marker"})
                    break
            vol["structure"].append({"id": "st-lift", "kind": "box", "bone": "easel-lift", "material": "aluminium-anodised", "role": "body", "tone": 0.5, "size": [round(0.62 * G, 4), 0.075, 0.08], "transform": {"translation": [round((FOOT_X + REAR_FOOT_X) / 2, 4), 0.0375, BOARD_Z]}, "member": "easel-lift", "part": "member"})
        else:
            vol["structure"].append({"id": "st-lift", "kind": "box", "bone": "easel-lift", "material": "aluminium-anodised", "role": "body", "tone": 0.5, "size": [round(0.62 * G, 4), 0.04, 0.04], "transform": {"translation": [round((FOOT_X + REAR_FOOT_X) / 2, 4), 0.02, BOARD_Z]}, "member": "easel-lift", "part": "member"})
        vol["structure"] += [
            {"id": "mass-easel", "kind": "ellipsoid", "bone": "easel", "material": "aluminium-anodised", "role": "detail", "tone": 0.2, "radii": [0.03, 0.03, 0.03], "transform": {"translation": [round(LEG_L * 0.55, 4), 0.0, 0]}, "member": "easel", "part": "mass"},
            {"id": "easel-feet-bar", "kind": "box", "bone": "easel-lift", "material": "glide-rubber", "role": "detail", "tone": 0.3, "size": [round(0.66 * G, 4), 0.03, round(BOARD_W + 0.08, 4)], "transform": {"translation": [round((FOOT_X + REAR_FOOT_X) / 2, 4), 0.015, BOARD_Z]}, "sheet": True, "member": "easel-lift", "part": "member"},
        ]


MATERIALS = [
    {"id": "aluminium-anodised", "title": t("Anodised aluminium tube", "Eloxiertes Aluminiumrohr"), "emissivity": 0.8, "solarAbsorptivity": 0.4, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "board-melamine", "title": t("Melamine-faced writing board", "Melaminbeschichtete Schreibtafel"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "source": "literature", "reference": "ashrae-2021"},
    {"id": "paper", "title": t("Flipchart paper, 80 g/m2", "Flipchart-Papier, 80 g/m2"), "emissivity": 0.93, "solarAbsorptivity": 0.25, "source": "literature", "reference": "ashrae-2021"},
    {"id": "glide-rubber", "title": t("Rubber glide", "Gummigleiter"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "source": "authored-estimate"},
    {"id": "marker-polymer", "title": t("Marker barrel, polypropylene", "Markerschaft, Polypropylen"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "source": "authored-estimate"},
]


def regions():
    return [
        {"id": "board", "title": t("Writing board of the flipchart", "Schreibtafel des Flipcharts"), "kind": "surface", "model": "passive", "material": "board-melamine", "areaM2": 1.45, "characteristicLengthM": 0.7, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 14000, "source": "authored-estimate", "reference": "ashrae-2021"},
        {"id": "paper", "title": t("Paper pad on the board", "Papierblock auf der Tafel"), "kind": "surface", "model": "passive", "material": "paper", "areaM2": 0.66, "characteristicLengthM": 0.65, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 6000, "source": "authored-estimate", "reference": "ashrae-2021"},
        {"id": "easel-frame", "title": t("Aluminium legs, tray and clamp bar", "Aluminiumbeine, Ablage und Klemmleiste"), "kind": "surface", "model": "passive", "material": "aluminium-anodised", "areaM2": 0.62, "characteristicLengthM": 0.03, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 2500, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]


LEGS = {"thigh-l": {"angleDeg": 4}, "shank-l": {"angleDeg": -7.68}, "foot-l": {"angleDeg": 3.68}, "thigh-r": {"angleDeg": 2}, "shank-r": {"angleDeg": -7}, "foot-r": {"angleDeg": 5}, "toe-l": {"angleDeg": 0}, "toe-r": {"angleDeg": 0}}
CONTACTS = ["heel-l", "contact-l", "heel-r", "contact-r", "easel-feet"]
MARK_PRESENT = {"upper-arm-r": {"angleDeg": 38.72, "abductionDeg": -43.72, "twistDeg": 66.15}, "forearm-r": {"angleDeg": 120.09}, "hand-r": {"angleDeg": -65, "abductionDeg": -40}}
MARK_WRITE = {"upper-arm-r": {"angleDeg": 21.06, "abductionDeg": -17.61, "twistDeg": 73.79}, "forearm-r": {"angleDeg": 109.95}, "hand-r": {"angleDeg": -62.25, "abductionDeg": -40}}
GESTURE_L = {"upper-arm-l": {"angleDeg": 28, "abductionDeg": 22}, "forearm-l": {"angleDeg": 62}, "hand-l": {"angleDeg": -12}}
HANG_L = {"upper-arm-l": {"angleDeg": 4, "abductionDeg": 4}, "forearm-l": {"angleDeg": 12}, "hand-l": {"angleDeg": 2}}
HOLD_R = {"upper-arm-r": {"angleDeg": 10, "abductionDeg": -4}, "forearm-r": {"angleDeg": 38}, "hand-r": {"angleDeg": -4}}


def thermal_binding(a, met, breathing):
    regs = {x["id"]: {"exposedFraction": 1, "sunlitFraction": 0} for x in a["thermal"]["regions"]}
    regs["shoes"] = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 21}}
    regs["board"] = {"exposedFraction": 0.6, "sunlitFraction": 0}
    regs["hand-skin"] = {"exposedFraction": 0.95, "sunlitFraction": 0}
    return {"activityMet": met, "breathing": breathing, "operating": {}, "regions": regs, "emitters": {"plume": 0.8}}


def poses(a):
    base = {**LEGS, "clavicle-r": {"angleDeg": 10}}
    present = {**base, "torso": {"angleDeg": -2, "twistDeg": -8}, "neck": {"angleDeg": -1, "twistDeg": 4}, "head": {"angleDeg": 1, "twistDeg": 6}, **MARK_PRESENT, **GESTURE_L}
    write = {**base, "torso": {"angleDeg": -2, "twistDeg": -8}, "neck": {"angleDeg": -4, "twistDeg": -14}, "head": {"angleDeg": -6, "twistDeg": -16}, **MARK_WRITE, **HANG_L}
    listen = {**LEGS, "torso": {"angleDeg": 0}, "neck": {"angleDeg": 0}, "head": {"angleDeg": 1}, **HOLD_R, **HANG_L}
    out = []
    for pid, title, desc, bones, met, br in [
        ("presenting", t("Marker raised, explaining", "Marker erhoben, erklärend"), t("Standing beside the flipchart and facing the audience, the right hand raises the marker to a word near the top of the sheet, the left hand opens toward the group; the trunk is turned a little toward the chart and the person talks.", "Neben dem Flipchart stehend und dem Publikum zugewandt, hebt die rechte Hand den Marker an ein Wort oben auf dem Blatt, die linke Hand öffnet sich zur Gruppe; der Rumpf ist leicht zum Flipchart gedreht, die Person spricht."), present, 1.5, "talk"),
        ("writing", t("Writing on the flipchart", "Auf das Flipchart schreiben"), t("Head and trunk turned to the chart, the marker on the paper at chest height, the left arm hanging; the person writes in silence.", "Kopf und Rumpf zum Flipchart gedreht, der Marker auf dem Papier in Brusthöhe, der linke Arm hängt; die Person schreibt schweigend."), write, 1.6, "rest"),
        ("listening", t("Listening, marker lowered", "Zuhören, Marker gesenkt"), t("Both arms down, the marker held loosely in the right hand at hip height, head level: the presenter listens to a question from the group.", "Beide Arme unten, der Marker locker in der rechten Hand auf Hüfthöhe, Kopf gerade: die vortragende Person hört einer Frage aus der Gruppe zu."), listen, 1.3, "rest"),
    ]:
        out.append({"id": pid, "title": title, "description": desc, "bones": bones, "ground": "auto", "contacts": CONTACTS, "thermal": thermal_binding(a, met, br), "loads": []})
    return out


def full(b):
    return {k: dict(v) for k, v in b.items()}


def clips(ps):
    pr, wr, li = (p["bones"] for p in ps)
    ex = []
    for tt, dl, dfl, dh, dn in [(0, 0, 0, 0, 0), (1.0, 10, -14, 4, 4), (2.0, -6, 12, -2, -2), (3.0, 14, -20, 6, 6), (4.0, -4, 8, 0, 0), (5.0, 8, -10, 3, 2), (6.0, 0, 0, 0, 0)]:
        b = full(pr)
        b["upper-arm-l"] = {"angleDeg": pr["upper-arm-l"]["angleDeg"] + dl, "abductionDeg": pr["upper-arm-l"]["abductionDeg"]}
        b["forearm-l"] = {"angleDeg": pr["forearm-l"]["angleDeg"] + dfl}
        b["head"] = {"angleDeg": pr["head"]["angleDeg"], "twistDeg": pr["head"]["twistDeg"] + dh}
        b["neck"] = {"angleDeg": pr["neck"]["angleDeg"], "twistDeg": pr["neck"]["twistDeg"] + dn}
        ex.append({"t": tt, "pose": "presenting", "ease": "smooth", "bones": b})
    explain = {"id": "explaining", "title": t("Explaining at the flipchart", "Am Flipchart erklären"), "description": t("A six-second loop: the marker stays on the word at the top of the sheet while the open left hand beats out the explanation and the head sweeps across the audience.", "Eine Schleife von sechs Sekunden: der Marker bleibt am Wort oben auf dem Blatt, während die offene linke Hand die Erklärung begleitet und der Kopf über das Publikum schwenkt."), "durationS": 6.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": ex,
               "contacts": [{"anchor": c, "fromS": 0, "toS": 6.0} for c in CONTACTS], "phases": [{"id": "explain", "fromS": 0, "toS": 6.0, "title": t("Explaining", "Erklären")}], "events": [{"t": 3.0, "kind": "motion", "title": t("Widest gesture to the group", "Weiteste Geste zur Gruppe")}],
               "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 6.0, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.5}, {"t": 3.0, "v": 1.6}, {"t": 6.0, "v": 1.5}]}]}
    wk = []
    for tt, da, dfa, dh in [(0, 0, 0, 0), (0.75, 2.5, -4, 3), (1.5, 5, -8, 6), (2.25, 2.5, -2, 3), (3.0, 0, 0, 0)]:
        b = full(wr)
        b["upper-arm-r"] = {"angleDeg": wr["upper-arm-r"]["angleDeg"] + da, "abductionDeg": wr["upper-arm-r"]["abductionDeg"], "twistDeg": wr["upper-arm-r"]["twistDeg"]}
        b["forearm-r"] = {"angleDeg": wr["forearm-r"]["angleDeg"] + dfa}
        b["hand-r"] = {"angleDeg": wr["hand-r"]["angleDeg"] + dh, "abductionDeg": wr["hand-r"]["abductionDeg"]}
        wk.append({"t": tt, "pose": "writing", "ease": "smooth", "bones": b})
    write = {"id": "writing-stroke", "title": t("Writing a line", "Eine Zeile schreiben"), "description": t("A three-second loop of handwriting on the flipchart: shoulder, elbow and wrist carry the marker along the line and back to its start.", "Eine Schleife von drei Sekunden Handschrift auf dem Flipchart: Schulter, Ellbogen und Handgelenk führen den Marker die Zeile entlang und zurück an ihren Anfang."), "durationS": 3.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": wk,
             "contacts": [{"anchor": c, "fromS": 0, "toS": 3.0} for c in CONTACTS], "phases": [{"id": "stroke", "fromS": 0, "toS": 1.5, "title": t("Stroke along the line", "Strich die Zeile entlang")}, {"id": "return", "fromS": 1.5, "toS": 3.0, "title": t("Back to the start", "Zurück zum Anfang")}], "events": [],
             "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.0, "v": 0}]}]}
    ik = []
    for tt, dt, dh in [(0, 0, 0), (1.25, -0.4, 1.4), (2.5, 0.3, 0.3), (3.75, -0.3, 1.0), (5.0, 0, 0)]:
        b = full(li)
        b["torso"] = {"angleDeg": li["torso"]["angleDeg"] + dt}
        b["head"] = {"angleDeg": li["head"]["angleDeg"] + dh}
        ik.append({"t": tt, "pose": "listening", "ease": "smooth", "bones": b})
    idle = {"id": "idle-breathing", "title": t("Idle breathing while listening", "Ruhiges Atmen beim Zuhören"), "description": t("A five-second loop of quiet standing beside the chart: the chest follows the breath and the head nods slightly while a question is asked.", "Eine Schleife von fünf Sekunden ruhigen Stehens neben dem Flipchart: die Brust folgt dem Atem, der Kopf nickt leicht, während eine Frage gestellt wird."), "durationS": 5.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": ik,
            "contacts": [{"anchor": c, "fromS": 0, "toS": 5.0} for c in CONTACTS], "phases": [{"id": "listen", "fromS": 0, "toS": 5.0, "title": t("Listening", "Zuhören")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5.0, "v": 0}]}]}
    return [explain, write, idle]


def build(out):
    d = load("tour-guide")
    a = d["asset"]
    a["id"] = ID
    remove_bones(a, ["pointer", "flag-pole"], drop_regions=("pointer-metal", "pole-metal", "flag-cloth"))
    a["anchors"] = [x for x in a["anchors"] if x["id"] != "pointer-tip"]
    rear_len = rig_props(a)
    tmp = f"{GEN}/tmp/{ID}-rig.json"
    save(d, tmp)
    frames = rest_frames(tmp)
    add_props(a, frames, rear_len)
    a["representations"]["500"]["pens"]["body"] = max(a["representations"]["500"]["pens"]["body"], 0.18)
    a["materials"] += [m for m in MATERIALS if m["id"] not in {x["id"] for x in a["materials"]}]
    a["anchors"] += [
        {"id": "easel-feet", "bone": "easel-lift", "x": round((FOOT_X + REAR_FOOT_X) / 2, 5), "y": 0, "z": BOARD_Z, "kind": "contact", "title": t("Easel feet on the floor", "Füße des Gestells am Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": round(0.62 * G, 4), "footprintWidthM": round(BOARD_W + 0.05, 4), "frictionCoefficient": 0.6},
        {"id": "marker-tip", "bone": "marker", "x": round(MARKER_L, 5), "y": 0, "kind": "attachment", "title": t("Marker tip on the paper", "Markerspitze auf dem Papier")},
    ]
    th = a["thermal"]
    th["regions"] += regions()
    a["poses"] = poses(a)
    a["clips"] = clips(a["poses"])
    a["presentation"] = {"defaultPose": "presenting", "defaultClip": "explaining", "defaultScale": 100}
    a["structure"]["dynamics"] = "quasi-static"
    scale_asset(a, F, MASS / BASE_MASS)
    for b in a["rig"]["bones"]:
        if b["id"] in ("easel", "rear-leg", "marker"):
            b["mass"]["kg"] = {"easel": 4.6, "rear-leg": 0.9, "marker": 0.015}[b["id"]]
    body = th["body"]
    stature = 1.75 * F
    body["massKg"] = MASS
    body["skinAreaM2"] = round(dubois(MASS, stature), 3)
    body["basalMetabolismW"] = round(mifflin(MASS, stature, 41, False), 1)
    TEXTS(a, stature)
    apply_ops(a, ID)
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
