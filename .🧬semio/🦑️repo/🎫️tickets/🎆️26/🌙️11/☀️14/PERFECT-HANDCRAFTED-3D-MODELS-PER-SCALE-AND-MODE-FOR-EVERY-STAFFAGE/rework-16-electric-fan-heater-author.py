"""🔥 Hand-authored rework of appliances/electric-fan-heater: fan bone, fan-spin clip and the four scale models.

Usage: python3 rework-16-electric-fan-heater-author.py [--write]
Without --write: source volumes + candidate document under 🗑️generated/r3-electric-fan-heater/.
With --write: also the catalog document (rig, poses, clips, volumes).
Frames: body bone u ahead, v up, w left. Fan bone (abductionDeg 90): u left, v up, w back, origin on the axle at the blade plane.
"""
import copy
import glob
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(T, "volumes/appliances/electric-fan-heater.json")
CAT = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/electric-fan-heater/🔣️.json")[0]
OUT = os.path.join(T, "🗑️generated/r3-electric-fan-heater")

doc = json.load(open(CAT))
asset = doc["asset"]
src = json.load(open(SRC))
V = src["volumes"]

ABS, PC, MESH, DIAL, RUBBER, ZINC, COIL = "abs-cream", "pc-heat", "steel-mesh", "pp-dial", "rubber-foot", "steel-zinc", "nichrome-coil"
FAN_U, FAN_V = -0.04, 0.145


def by_id(items):
    return {o["id"]: o for o in items}


def rr(v0, v1, w0, w1, r, n=4):
    """⭕ Rounded rectangle in a yz profile ([v, w]), n segments per corner."""
    pts = []
    for cv, cw, a0 in ((v0 + r, w0 + r, 180), (v1 - r, w0 + r, 270), (v1 - r, w1 - r, 0), (v0 + r, w1 - r, 90)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([round(cv + r * math.cos(a), 5), round(cw + r * math.sin(a), 5)])
    return pts


def solid(id, kind, bone, material, role, tone, **rest):
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **rest}


def box(id, bone, material, role, tone, size, at, **rest):
    return solid(id, "box", bone, material, role, tone, size=size, transform={"translation": at}, **rest)


def th(id, of, region, layer=None):
    o = {"id": id, "of": of, "region": region}
    if layer:
        o["layer"] = layer
    return o


def st(id, of, member, part, anchor=None):
    o = {"id": id, "of": of, "member": member, "part": part}
    if anchor:
        o["anchor"] = anchor
    return o


# ---------------------------------------------------------------- rig, poses, clips
bones = [b for b in asset["rig"]["bones"] if b["id"] != "fan"]
body_bone = next(b for b in bones if b["id"] == "body")
body_bone["mass"]["kg"] = 1.41
bones.append({
    "id": "fan", "parent": "body",
    "title": {"en": "Axial fan wheel on the motor shaft", "de": "Axiallüfterrad auf der Motorwelle"},
    "x": FAN_U, "y": FAN_V, "z": 0, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0.065,
    "joint": {"type": "revolute"},
    "mass": {"kg": 0.06, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"},
})
asset["rig"]["bones"] = bones
FAN_POSE = {"off": 0, "heating-low": 24, "heating": 48}
for p in asset["poses"]:
    p["bones"]["fan"] = {"angleDeg": FAN_POSE[p["id"]]}
feet = [{"anchor": "foot-back", "fromS": 0, "toS": 6}, {"anchor": "foot-front", "fromS": 0, "toS": 6}]
spin = {
    "id": "fan-spin",
    "title": {"en": "Fan runs up through both stages", "de": "Lüfter läuft über beide Stufen hoch"},
    "description": {
        "en": "Slowed down so that the wheel can be followed: the fan starts with the first stage, the dial is turned on to the second stage and the wheel keeps turning.",
        "de": "Verlangsamt, damit das Rad verfolgbar bleibt: Der Lüfter läuft mit der ersten Stufe an, der Regler wird auf die zweite Stufe gedreht und das Rad dreht weiter.",
    },
    "durationS": 6, "fps": 15, "loop": False, "rootMotion": "none",
    "keys": [
        {"t": 0, "pose": "off", "ease": "in", "bones": {"fan": {"angleDeg": 0}}},
        {"t": 1, "pose": "heating-low", "ease": "linear", "bones": {"fan": {"angleDeg": 90}}},
        {"t": 3, "pose": "heating-low", "ease": "linear", "bones": {"fan": {"angleDeg": 450}}},
        {"t": 4, "pose": "heating", "ease": "linear", "bones": {"fan": {"angleDeg": 630}}},
        {"t": 6, "pose": "heating", "ease": "linear", "bones": {"fan": {"angleDeg": 990}}},
    ],
    "contacts": feet,
    "phases": [
        {"id": "run-up", "fromS": 0, "toS": 1, "title": {"en": "Run-up", "de": "Anlauf"}},
        {"id": "low", "fromS": 1, "toS": 3, "title": {"en": "1000 W", "de": "1000 W"}},
        {"id": "full", "fromS": 3, "toS": 6, "title": {"en": "2000 W", "de": "2000 W"}},
    ],
    "events": [
        {"t": 0, "kind": "operating", "title": {"en": "First stage and fan on", "de": "Erste Stufe und Lüfter an"}, "target": "stage-1"},
        {"t": 3, "kind": "operating", "title": {"en": "Second stage on", "de": "Zweite Stufe an"}, "target": "stage-2"},
    ],
    "tracks": [
        {"target": "operating:stage-1", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 6, "v": 1}]},
        {"target": "operating:stage-2", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}]},
        {"target": "emitter:warm-air-1", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 1}, {"t": 6, "v": 1}]},
        {"target": "emitter:warm-air-2", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}]},
    ],
}
asset["clips"] = [c for c in asset["clips"] if c["id"] != "fan-spin"] + [spin]

# ---------------------------------------------------------------- shared hand-made parts
AXLE = [0, 0, -1]


def fan_hub():
    return solid("fan-hub", "revolve", "fan", PC, "detail", 0.35,
                 profile=[[0.0, 0.0], [0.0, 0.016], [0.018, 0.018], [0.024, 0.012], [0.024, 0.0]], axis=AXLE, origin=[0, 0, 0.012])


BLADE = [[0.012, -0.008], [0.02, -0.013], [0.034, -0.021], [0.048, -0.027], [0.058, -0.028], [0.0635, -0.022], [0.065, -0.01],
         [0.0645, 0.004], [0.061, 0.016], [0.054, 0.022], [0.044, 0.022], [0.032, 0.017], [0.021, 0.011], [0.012, 0.007]]


def fan_blades(thickness, texture):
    o = solid("fan-blades", "extrude", "fan", PC, "detail", 0.4,
              profile=[[w, v] for v, w in reversed(BLADE)], plane="xy", **{"from": -thickness / 2, "to": thickness / 2},
              transform={"rotation": [0.0, 0.241922, 0.0, 0.970296]},
              repeat={"count": 5, "rotationAxis": AXLE, "rotationRad": 1.256637, "origin": [0, 0, 0]})
    if texture:
        o["texture"] = True
    return o


def lug(v0, depth):
    """🧳 Carry handle as a solid lug where its opening is below the feature of the scale."""
    return solid("handle", "extrude", "body", ABS, "body", 0.75,
                 profile=[[v0, -0.073], [0.295, -0.073], [0.31, -0.058], [0.31, 0.058], [0.295, 0.073], [v0, 0.073]],
                 plane="yz", **{"from": -depth / 2, "to": depth / 2})


def warm_air(u):
    return solid("th-warm-air", "revolve", "body", MESH, "detail", 0.6,
                 profile=[[0, 0], [0, 0.05], [0.0015, 0.05], [0.0015, 0]], axis=[1, 0, 0], origin=[u, 0.145, 0.0],
                 tags=["warm-air-1", "warm-air-2"], region="grille", layer="bars")


def pads():
    """🦶 The four 10 mm rubber pads at true size, the thermal feet of every coarse scale."""
    return [solid("th-foot-" + name, "revolve", "body", RUBBER, "body", 0.15,
                  profile=[[0.0, 0.0], [0.0, 0.0125], [0.0025, 0.0145], [0.01, 0.015], [0.01, 0.0]], axis=[0, 1, 0], origin=[u, 0.0, -0.075],
                  repeat={"count": 2, "translation": [0, 0, 0.15]}, region="feet", layer="foot") for name, u in (("back", -0.058), ("front", 0.058))]


def plates():
    """🪟 Outlet and intake as the thin plates that stand proud of a solid block; their backs are section faces."""
    return [
        box("th-grille", "body", MESH, "body", 0.2, [0.004, 0.17, 0.188], [0.092, 0.145, 0.0], sectionFaces=["-u"], region="grille", layer="bars"),
        box("th-intake-panel", "body", PC, "body", 0.3, [0.006, 0.164, 0.194], [-0.095, 0.145, 0.0], sectionFaces=["+u"], region="intake", layer="inlet"),
    ]


def true_dial():
    return solid("th-dial", "revolve", "knob", DIAL, "body", 0.2, profile=[[0, 0], [0, 0.022], [0.0145, 0.022], [0.0145, 0]], axis=[0, 0, 1],
                 origin=[0, 0, -0.1265], region="control", layer="dial")


def decisions(*pairs):
    return [{"en": en, "de": de} for en, de in pairs]


# ---------------------------------------------------------------- 1:50 (feature 7.5 mm)
s50 = V["50"]
b = [o for o in s50["body"] if o["id"] not in ("fan-hub", "fan-blades", "fan-shaft", "grille-ribs")]
b += [
    fan_hub(),
    fan_blades(0.0015, True),
    solid("fan-shaft", "revolve", "fan", ZINC, "detail", 0.5,
          profile=[[0, 0], [0, 0.004], [0.046, 0.004], [0.046, 0]], axis=AXLE, origin=[0, 0, 0.034]),
    box("grille-ribs", "body", MESH, "detail", 0.32, [0.004, 0.16, 0.004], [0.088, 0.145, -0.03],
        repeat={"count": 2, "translation": [0, 0, 0.06]}, texture=True),
]
s50["body"] = b
s50["structure"] = [o for o in s50["structure"] if o["id"] not in ("member-fan-shaft", "joint-fan")] + [
    st("member-fan-shaft", "fan-shaft", "fan", "member"),
    solid("joint-fan", "revolve", "fan", ZINC, "detail", 0.25,
          profile=[[0, 0], [0, 0.0075], [0.008, 0.0075], [0.008, 0]], axis=AXLE, origin=[0, 0, 0.04], member="fan", part="joint"),
]
s50["decisions"] = decisions(
    ("The housing is built as the real mouldings: a U-shaped shell and a top cover with 2.5 mm walls and 22 mm edge radii, extruded along the air path, closed by a front and a rear frame; four moulded bosses carry 10 mm rubber pads. The mains cord is left to the scene, only its grommet is modelled.",
     "Das Gehäuse ist wie die echten Spritzgussteile gebaut: eine U-förmige Schale und eine Haube mit 2,5 mm Wand und 22 mm Kantenradius, entlang des Luftwegs extrudiert, geschlossen durch Front- und Rückrahmen; vier angeformte Sockel tragen 10 mm hohe Gummipuffer. Die Netzleitung bleibt der Szene überlassen, modelliert ist nur ihre Tülle."),
    ("The outlet is a steel cassette with eight 1.5 mm louvre slats and two vertical ribs that keep them apart; the intake is a polycarbonate panel with seven slots; the carry handle is one loft across the cover with a 100 mm by 26 mm opening; the dial is a turned knob with a grip bar on the knob bone.",
     "Der Auslass ist eine Stahlkassette mit acht 1,5 mm dicken Lamellen und zwei senkrechten Stegen, die sie auf Abstand halten; der Einlass ist eine Polycarbonatplatte mit sieben Schlitzen; der Tragegriff ist ein Loft quer über der Haube mit einer Öffnung von 100 mm auf 26 mm; der Regler ist ein gedrehter Knopf mit Griffsteg am Knopfknochen."),
    ("The fan wheel (hub, five pitched blades of 130 mm diameter, 8 mm shaft) rides on its own bone whose joint axis is the motor shaft, so poses and the fan-spin clip turn it inside the fixed shroud ring; the shaded-pole motor hangs on a bridge between the side walls, the two heating stages are three coil runs each in a sheet-metal box.",
     "Das Lüfterrad (Nabe, fünf angestellte Flügel mit 130 mm Durchmesser, 8 mm Welle) sitzt auf einem eigenen Knochen, dessen Gelenkachse die Motorwelle ist, sodass Posen und der Clip fan-spin es im festen Düsenring drehen; der Spaltpolmotor hängt an einer Brücke zwischen den Seitenwänden, die zwei Heizstufen sind je drei Wendelzüge in einem Blechkasten."),
    ("Thermal: housing walls and cover as true 2.5 mm shells, grille with coils, intake, controls and feet as their own regions, the warm-air emitter as a disc in the outlet. Structure: feet, bosses, shell, end frames, cover band and handle are the load path; motor and heater hang on bridge and brackets with their masses, dial spindle and fan shaft are members of their bones with a bearing joint each.",
     "Thermisch: Gehäusewände und Haube als echte 2,5-mm-Schalen, Gitter mit Wendeln, Einlass, Bedienteile und Füße als eigene Regionen, der Warmluft-Emitter als Scheibe im Auslass. Statisch: Füße, Sockel, Schale, Endrahmen, Haubenband und Griff bilden den Lastpfad; Motor und Heizregister hängen mit ihren Massen an Brücke und Haltern, Reglerachse und Lüfterwelle sind Stäbe ihrer Knochen mit je einem Lagergelenk."),
)

# ---------------------------------------------------------------- 1:100 (feature 15 mm)
s100 = V["100"]
old = by_id(s100["body"])
shroud = copy.deepcopy(old["fan-shroud"])
shroud["from"], shroud["to"] = -0.0475, -0.0325
slot_bars = []
for k in range(4):
    v0 = 0.079 + k * 0.037
    slot_bars.append(rr(v0, v0 + 0.021, -0.08, 0.08, 0.004, 2))
circle = [[round(0.022 * math.cos(math.radians(a)), 5), round(0.022 * math.sin(math.radians(a)), 5)] for a in range(120, 421, 20)]
s100["body"] = [
    old["shell-block"], old["cover-block"], old["front-cap"], old["rear-cap"],
    solid("grille-frame", "extrude", "body", MESH, "body", 0.2, profile=rr(0.06, 0.23, -0.094, 0.094, 0.006, 2), plane="yz",
          **{"from": 0.079, "to": 0.094}, holes=[rr(0.075, 0.215, -0.079, 0.079, 0.003, 1)]),
    box("louvres", "body", MESH, "detail", 0.32, [0.015, 0.015, 0.164], [0.086, 0.085, 0.0],
        rotation=None, repeat={"count": 4, "translation": [0, 0.04, 0]}, texture=True),
    solid("intake-panel", "extrude", "body", PC, "body", 0.3, profile=rr(0.063, 0.227, -0.097, 0.097, 0.012, 3), plane="yz",
          **{"from": -0.098, "to": -0.083}, holes=slot_bars),
    fan_hub(), fan_blades(0.015, False),
    box("motor-bridge", "body", ZINC, "detail", 0.5, [0.015, 0.015, 0.223], [-0.0755, 0.145, 0.0]),
    old["motor-stack"], shroud,
    solid("heater-box", "extrude", "body", ZINC, "detail", 0.5, profile=rr(0.074, 0.216, -0.08, 0.08, 0.004, 2), plane="yz",
          **{"from": 0.004, "to": 0.058}, holes=[rr(0.089, 0.201, -0.065, 0.065, 0.002, 1)]),
    solid("coil-stage-1", "capsule", "body", COIL, "detail", 0.45, p0=[0.021, 0.105, -0.0725], p1=[0.021, 0.105, 0.0725],
          r0=0.0075, r1=0.0075, repeat={"count": 3, "translation": [0, 0.04, 0]}),
    solid("coil-stage-2", "capsule", "body", COIL, "detail", 0.45, p0=[0.041, 0.105, -0.0725], p1=[0.041, 0.105, 0.0725],
          r0=0.0075, r1=0.0075, repeat={"count": 3, "translation": [0, 0.04, 0]}),
    box("heater-brackets", "body", ZINC, "detail", 0.5, [0.015, 0.015, 0.0335], [0.031, 0.145, -0.09575],
        repeat={"count": 2, "translation": [0, 0, 0.1915]}),
    old["handle"], old["foot-back"], old["foot-front"],
    solid("dial", "extrude", "knob", DIAL, "body", 0.2, profile=circle, plane="xy", **{"from": -0.1275, "to": -0.1125}),
    box("mode-switch", "body", DIAL, "body", 0.2, [0.032, 0.016, 0.015], [0.0, 0.112, -0.1145], cornerRadius=0.002),
]
for o in s100["body"]:
    o.pop("rotation", None)
by_id(s100["body"])["louvres"]["transform"]["rotation"] = [0.0, 0.0, -0.19509, 0.980785]
oldt = by_id(s100["thermal"])
THIN_SLAT_TURN = [0.0, 0.0, -0.19509, 0.980785]
s100["thermal"] = [
    oldt["th-front-cap"], oldt["th-rear-cap"], oldt["th-cover-block"], oldt["th-shell-left"], oldt["th-shell-right"],
    {**copy.deepcopy(by_id(s50["body"])["grille-frame"]), "id": "th-grille-frame", "region": "grille", "layer": "bars"},
    {**box("th-louvres", "body", MESH, "detail", 0.32, [0.012, 0.0015, 0.164], [0.088, 0.085, 0.0], repeat={"count": 4, "translation": [0, 0.04, 0]}, texture=True),
     "region": "grille", "layer": "bars"},
    warm_air(0.0925),
    solid("th-intake-panel", "extrude", "body", PC, "body", 0.3, profile=rr(0.063, 0.227, -0.097, 0.097, 0.012, 3), plane="yz",
          **{"from": -0.098, "to": -0.095}, holes=slot_bars, region="intake", layer="inlet"),
    th("th-handle", "handle", "casing"), *pads(),
    th("th-dial", "dial", "control", "dial"), th("th-mode-switch", "mode-switch", "control", "dial"),
    solid("th-coil-stage-1", "capsule", "body", COIL, "detail", 0.45, p0=[0.022, 0.105, -0.0755], p1=[0.022, 0.105, 0.0755],
          r0=0.0035, r1=0.0035, repeat={"count": 3, "translation": [0, 0.04, 0]}, slender=True, region="grille"),
    solid("th-coil-stage-2", "capsule", "body", COIL, "detail", 0.45, p0=[0.04, 0.105, -0.0755], p1=[0.04, 0.105, 0.0755],
          r0=0.0035, r1=0.0035, repeat={"count": 3, "translation": [0, 0.04, 0]}, slender=True, region="grille"),
    th("th-motor-stack", "motor-stack", "casing"),
]
by_id(s100["thermal"])["th-louvres"]["transform"]["rotation"] = THIN_SLAT_TURN
olds = by_id(s100["structure"])
s100["structure"] = [
    olds["member-shell"], olds["member-top-band"],
    st("member-front-ring", "front-cap", "body", "member"), st("member-rear-ring", "rear-cap", "body", "member"),
    st("member-handle", "handle", "body", "member"), olds["joint-handle"],
    st("member-motor-bridge", "motor-bridge", "body", "member"),
    st("member-heater-brackets", "heater-brackets", "body", "member"),
    olds["member-knob"], olds["joint-knob"],
    solid("member-fan-shaft", "revolve", "fan", ZINC, "detail", 0.5,
          profile=[[0, 0], [0, 0.0075], [0.052, 0.0075], [0.052, 0]], axis=AXLE, origin=[0, 0, 0.04], member="fan", part="member"),
    solid("joint-fan", "revolve", "fan", ZINC, "detail", 0.25,
          profile=[[0, 0], [0, 0.012], [0.015, 0.012], [0.015, 0]], axis=AXLE, origin=[0, 0, 0.043], member="fan", part="joint"),
    box("mass-motor", "body", ZINC, "detail", 0.15, [0.02, 0.046, 0.054], [-0.062, 0.145, 0.0], member="body", part="mass"),
    box("mass-heater", "body", ZINC, "detail", 0.15, [0.03, 0.04, 0.05], [0.031, 0.145, 0.0], member="body", part="mass"),
    st("support-foot-back", "foot-back", "body", "support", "foot-back"), st("support-foot-front", "foot-front", "body", "support", "foot-front"),
]
s100["decisions"] = decisions(
    ("Modelled for 15 mm: the housing is a through tube with end caps and a cover, the outlet cassette has 15 mm rails and four louvre slats of 15 mm that stand for the eight real ones, the intake panel four slots of 21 mm for the fourteen real ones.",
     "Für 15 mm modelliert: Das Gehäuse ist ein durchgehendes Rohr mit Endkappen und Haube, die Auslasskassette hat 15 mm breite Holme und vier Lamellen von 15 mm für die acht echten, die Einlassplatte vier Schlitze von 21 mm für die vierzehn echten."),
    ("Inside, the fan wheel keeps its five pitched blades at 15 mm thickness on the fan bone, the shroud ring, the motor bridge, the heater frame and the coil runs (one 15 mm bar per run) are thickened to the feature; the dial is a 44 mm disc of 15 mm with a flat as pointer, the grip bar and the cord grommet are left out.",
     "Innen behält das Lüfterrad seine fünf angestellten Flügel mit 15 mm Dicke am Lüfterknochen, Düsenring, Motorbrücke, Heizrahmen und Wendelzüge (ein 15-mm-Stab je Zug) sind auf das Merkmal verstärkt; der Regler ist eine 44-mm-Scheibe von 15 mm mit einer Abflachung als Zeiger, Griffsteg und Kabeltülle entfallen."),
    ("The thermal list keeps the true gauge where the body is thickened: the steel cassette with 5 mm rails and four slats of 1.5 mm sheet, the 7 mm coil runs, the 3 mm intake plate and the 10 mm rubber pads, so the surfaces that exchange heat stay those of the real device.",
     "Die thermische Liste behält die wahre Stärke, wo der Körper verstärkt ist: die Stahlkassette mit 5-mm-Holmen und vier Lamellen aus 1,5-mm-Blech, die 7-mm-Wendelzüge, die 3-mm-Einlassplatte und die 10-mm-Gummipuffer, damit die wärmetauschenden Flächen die des echten Geräts bleiben."),
    ("Thermal regions and the structural load path are those of 1:50: shell, end rings, cover band and handle carry on the four feet, motor and heater hang on bridge and brackets, dial spindle and fan shaft are 15 mm members with their bearings.",
     "Thermische Regionen und statischer Lastpfad sind die von 1:50: Schale, Endringe, Haubenband und Griff tragen auf den vier Füßen, Motor und Heizregister hängen an Brücke und Haltern, Reglerachse und Lüfterwelle sind 15-mm-Stäbe mit ihren Lagern."),
)

# ---------------------------------------------------------------- 1:200 (feature 30 mm)
s200 = V["200"]
s200["body"] = [
    solid("block", "extrude", "body", ABS, "body", 0.78, profile=rr(0.016, 0.272, -0.114, 0.114, 0.022, 3), plane="yz", **{"from": -0.092, "to": 0.09}),
    box("grille", "body", MESH, "body", 0.2, [0.03, 0.17, 0.188], [0.079, 0.145, 0.0], cornerRadius=0.004),
    box("intake-panel", "body", PC, "body", 0.3, [0.03, 0.164, 0.194], [-0.083, 0.145, 0.0], cornerRadius=0.004),
    lug(0.25, 0.03),
    box("foot-back", "body", RUBBER, "body", 0.15, [0.03, 0.03, 0.18], [-0.058, 0.015, 0.0]),
    box("foot-front", "body", RUBBER, "body", 0.15, [0.03, 0.03, 0.18], [0.058, 0.015, 0.0]),
    solid("dial", "revolve", "knob", DIAL, "body", 0.2, profile=[[0, 0], [0, 0.022], [0.03, 0.022], [0.03, 0]], axis=[0, 0, 1], origin=[0, 0, -0.1265]),
    solid("fan-wheel", "revolve", "fan", PC, "detail", 0.4, profile=[[0, 0], [0, 0.065], [0.03, 0.065], [0.03, 0]], axis=AXLE, origin=[0, 0, 0.015]),
]
low = [[0.232, -0.114]] + [p for p in rr(0.016, 0.272, -0.114, 0.114, 0.022, 3) if p[0] < 0.2] + [[0.232, 0.114]]
top = [[0.232, 0.114]] + [p for p in reversed(rr(0.016, 0.272, -0.114, 0.114, 0.022, 3)) if p[0] > 0.2] + [[0.232, -0.114]]
low = [[0.232, -0.114]] + sorted([p for p in rr(0.016, 0.272, -0.114, 0.114, 0.022, 3) if p[0] < 0.2], key=lambda p: (p[1], 0)) + [[0.232, 0.114]]
top = [[0.232, 0.114]] + sorted([p for p in rr(0.016, 0.272, -0.114, 0.114, 0.022, 3) if p[0] > 0.2], key=lambda p: -p[1]) + [[0.232, -0.114]]


def casing(v_low, u0, u1, profile_low):
    return [
        solid("th-casing", "extrude", "body", ABS, "body", 0.78, profile=profile_low, plane="yz", **{"from": u0, "to": u1}, region="casing"),
    ]


s200["thermal"] = [
    solid("th-casing", "extrude", "body", ABS, "body", 0.78, profile=low, plane="yz", **{"from": -0.092, "to": 0.09}, sectionFaces=["+v"], region="casing"),
    solid("th-top-cover", "extrude", "body", ABS, "body", 0.7, profile=top, plane="yz", **{"from": -0.092, "to": 0.09}, sectionFaces=["-v"], region="top-cover"),
    th("th-handle", "handle", "casing"), *plates(), warm_air(0.0945), true_dial(), *pads(),
]
s200["structure"] = [
    st("member-shell", "block", "body", "member"), st("member-handle", "handle", "body", "member"),
    st("member-knob", "dial", "knob", "member"), st("member-fan", "fan-wheel", "fan", "member"),
    box("mass-motor", "body", ZINC, "detail", 0.15, [0.03, 0.046, 0.054], [-0.062, 0.145, 0.0], member="body", part="mass"),
    st("support-foot-back", "foot-back", "body", "support", "foot-back"), st("support-foot-front", "foot-front", "body", "support", "foot-front"),
]
s200["decisions"] = decisions(
    ("Modelled for 30 mm: the housing is one closed block with its 22 mm edge radii, outlet grille and intake are dark panels 4 mm and 6 mm proud of the end faces, because slats and slots are below the feature.",
     "Für 30 mm modelliert: Das Gehäuse ist ein geschlossener Block mit seinen 22 mm Kantenradien, Auslassgitter und Einlass sind dunkle Platten, die 4 mm und 6 mm vor den Stirnflächen stehen, weil Lamellen und Schlitze unter dem Merkmal liegen."),
    ("The carry handle is a solid lug 146 mm wide and 30 mm thick because its opening would be 8 mm high at this bar thickness; the four feet are two transverse rails of 30 mm, the dial is a 44 mm stub set 30 mm deep into the side wall, the mode switch is left out; the fan wheel is its swept disc on the fan bone inside the block.",
     "Der Tragegriff ist eine massive Lasche von 146 mm Breite und 30 mm Dicke, weil seine Öffnung bei dieser Stabdicke nur 8 mm hoch wäre; die vier Füße sind zwei Querleisten von 30 mm, der Regler ist ein 44-mm-Stumpf, 30 mm tief in der Seitenwand, der Betriebsartenschalter entfällt; das Lüfterrad ist seine überstrichene Scheibe am Lüfterknochen im Block."),
    ("Thermal: lower housing, cover, grille panel with the warm-air disc, intake panel, dial and foot rails are the six regions. Structure: the block and the lug carry on the two foot rails, the motor mass sits behind the intake.",
     "Thermisch: Gehäuseunterteil, Haube, Gitterplatte mit Warmluftscheibe, Einlassplatte, Regler und Fußleisten sind die sechs Regionen. Statisch: Block und Lasche tragen auf den zwei Fußleisten, die Motormasse sitzt hinter dem Einlass."),
)

# ---------------------------------------------------------------- 1:500 (feature 75 mm)
s500 = V["500"]
s500["body"] = [
    solid("block", "extrude", "body", ABS, "body", 0.78,
          profile=[[0.0, -0.114], [0.242, -0.114], [0.272, -0.084], [0.272, 0.084], [0.242, 0.114], [0.0, 0.114]], plane="yz", **{"from": -0.092, "to": 0.09}),
    box("grille", "body", MESH, "body", 0.2, [0.075, 0.17, 0.188], [0.0565, 0.145, 0.0]),
    box("intake-panel", "body", PC, "body", 0.3, [0.075, 0.164, 0.194], [-0.0605, 0.145, 0.0]),
    lug(0.2, 0.075),
]
s500["thermal"] = [
    solid("th-casing", "extrude", "body", ABS, "body", 0.78, profile=[[0.016, -0.114], [0.232, -0.114], [0.232, 0.114], [0.016, 0.114]], plane="yz",
          **{"from": -0.092, "to": 0.09}, sectionFaces=["+v"], region="casing"),
    solid("th-top-cover", "extrude", "body", ABS, "body", 0.7, profile=[[0.232, -0.114], [0.242, -0.114], [0.272, -0.084], [0.272, 0.084], [0.242, 0.114], [0.232, 0.114]],
          plane="yz", **{"from": -0.092, "to": 0.09}, sectionFaces=["-v"], region="top-cover"),
    th("th-handle", "handle", "casing"), *plates(), warm_air(0.0945), true_dial(), *pads(),
]
s500["structure"] = [
    st("member-shell", "block", "body", "member"), st("member-handle", "handle", "body", "member"),
    solid("member-knob", "revolve", "knob", ZINC, "detail", 0.25, profile=[[0, 0], [0, 0.0075], [0.03, 0.0075], [0.03, 0]], axis=[0, 0, 1], origin=[0, 0, -0.114],
          member="knob", part="member"),
    solid("member-fan", "revolve", "fan", ZINC, "detail", 0.25, profile=[[0, 0], [0, 0.0075], [0.046, 0.0075], [0.046, 0]], axis=AXLE, origin=[0, 0, 0.034],
          member="fan", part="member"),
    box("mass-motor", "body", ZINC, "detail", 0.15, [0.03, 0.046, 0.054], [-0.062, 0.145, 0.0], member="body", part="mass"),
    box("support-foot-back", "body", RUBBER, "body", 0.15, [0.03, 0.016, 0.18], [-0.058, 0.008, 0.0], member="body", part="support", anchor="foot-back"),
    box("support-foot-front", "body", RUBBER, "body", 0.15, [0.03, 0.016, 0.18], [0.058, 0.008, 0.0], member="body", part="support", anchor="foot-front"),
]
s500["decisions"] = decisions(
    ("Modelled for 75 mm, three features across: one block from the floor to the cover with 30 mm chamfers for the edge radii, a dark outlet panel and a dark intake panel 4 mm and 6 mm proud of the end faces, and the carry handle as a lug 146 mm wide and 75 mm thick. Feet, dial, switch and everything inside are below the feature and left out of the body.",
     "Für 75 mm modelliert, drei Merkmale breit: ein Block vom Boden bis zur Haube mit 30-mm-Fasen für die Kantenradien, eine dunkle Auslassplatte und eine dunkle Einlassplatte, die 4 mm und 6 mm vor den Stirnflächen stehen, und der Tragegriff als Lasche von 146 mm Breite und 75 mm Dicke. Füße, Regler, Schalter und alles Innere liegen unter dem Merkmal und fehlen im Körper."),
    ("Thermal: the six regions stay spatial, dial and feet at their true size inside the outline. Structure: block and lug carry on the two foot lines, dial spindle and fan shaft remain as the members of their bones, the motor mass sits behind the intake.",
     "Thermisch: Die sechs Regionen bleiben räumlich, Regler und Füße in wahrer Größe innerhalb des Umrisses. Statisch: Block und Lasche tragen auf den zwei Fußlinien, Reglerachse und Lüfterwelle bleiben die Stäbe ihrer Knochen, die Motormasse sitzt hinter dem Einlass."),
)

# ---------------------------------------------------------------- write
json.dump(src, open(SRC, "w"), ensure_ascii=False, indent=1)
asset["volumes"] = V
os.makedirs(OUT, exist_ok=True)
json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w"), ensure_ascii=False, indent=2)
if "--write" in sys.argv:
    live = json.load(open(CAT))
    for key in ("rig", "poses", "clips", "volumes"):
        live["asset"][key] = asset[key]
    json.dump(live, open(CAT, "w"), ensure_ascii=False, indent=2)
    open(CAT, "a").write("\n")
print("written", SRC, OUT, "catalog" if "--write" in sys.argv else "candidate only")
