"""🔥 Hand edit of `objects/patio-heater` (rework, second round, pass 1): door leaf on its own hinge bone, wheel set on a lateral axle bone, bottle with regulator and hose in the body list, per-scale wheel kits, 2D door layers, pose and clip for the bottle change.

Usage: python3 rework-patio-heater-edit.py            # candidate document under 🗑️generated/rework-patio-heater/cand/🔣️.json
       python3 rework-patio-heater-edit.py --apply    # also writes T/volumes/objects/patio-heater.json and the catalog roots

Idempotent: every solid, shape, bone, pose and clip is stated by id and replaces what carries that id.
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/patio-heater/🔣️.json"
SOURCE = os.path.join(T, "volumes", "objects", "patio-heater.json")
CAND = os.path.join(T, "🗑️generated", "rework-patio-heater", "cand", "🔣️.json")

HALF = 0.75
START = math.pi - HALF
END = math.pi + HALF
REST = 2 * math.pi - 2 * HALF
HINGE_R = 0.2094
HINGE = [0.1, round(HINGE_R * math.sin(HALF), 5), round(-HINGE_R * math.cos(HALF), 5)]
TO_DOOR = [-HINGE[0], -HINGE[1], -HINGE[2]]
SEAM_V = round(0.21 * math.sin(HALF), 4)
OPEN_DEG = -105


def quat(phi):
    return [round(math.sin(phi / 2), 6), 0, 0, round(math.cos(phi / 2), 6)]


def revolve(id, bone, material, role, tone, profile, **more):
    solid = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "axis": [1, 0, 0], "origin": [0, 0, 0]}
    solid.update(more)
    return solid


def ring(u0, u1, r0, r1):
    return [[u0, r0], [u0, r1], [u1, r1], [u1, r0], [u0, r0]]


def arc_wall(id, u0, u1, r0, r1, tone=0.6, **more):
    return revolve(id, "base", "powder-steel", "body", tone, ring(u0, u1, r0, r1), sweepRad=round(REST, 6), transform={"rotation": quat(END)}, **more)


def door_leaf(u0, u1, r0, r1, **more):
    return revolve("door", "door", "powder-steel", "body", 0.57, ring(u0, u1, r0, r1), sweepRad=2 * HALF, transform={"rotation": quat(START), "translation": TO_DOOR}, **more)


def capsule(id, bone, material, tone, p0, p1, r, **more):
    solid = {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": "detail", "tone": tone, "p0": p0, "p1": p1, "r0": r, "r1": r}
    solid.update(more)
    return solid


def replace(lst, drop, items):
    ids = {item["id"] for item in items} | set(drop)
    return [entry for entry in lst if entry["id"] not in ids] + items


def th(id, region):
    return {"id": "th-" + id, "of": id, "region": region}


def st(id, member, part="member"):
    return {"id": "st-" + id, "of": id, "member": member, "part": part}


WHEEL_AXIS = {"axis": [1, 0, 0]}
TYRE = [[-0.015, 0.029], [-0.015, 0.04], [-0.011, 0.045], [0.011, 0.045], [0.015, 0.04], [0.015, 0.029], [-0.015, 0.029]]
HUB = [[-0.014, 0.00625], [-0.014, 0.012], [-0.011, 0.014], [-0.011, 0.029], [0.011, 0.029], [0.011, 0.014], [0.014, 0.012], [0.014, 0.00625], [-0.014, 0.00625]]
AXLE = [[-0.18, 0], [-0.18, 0.008], [-0.179, 0.01], [-0.176, 0.01], [-0.176, 0.006], [0.176, 0.006], [0.176, 0.01], [0.179, 0.01], [0.18, 0.008], [0.18, 0]]
BRACKET = [[0.016, 0.185], [0.016, 0.253], [0.025, 0.272], [0.042, 0.281], [0.058, 0.281], [0.07, 0.271], [0.074, 0.249], [0.06, 0.185]]
WHEEL_OLD = [part + side for part in ("tyre-", "hub-", "bush-", "bracket-") for side in ("left", "right")] + ["axle"]


def wheel(id, at, material, role, tone, profile, **more):
    solid = revolve(id, "wheels", material, role, tone, profile, **more)
    solid["origin"] = [at, 0, 0]
    return solid


def bracket(id, z0, z1, profile=BRACKET, **more):
    solid = {"id": id, "bone": "base", "material": "powder-steel", "role": "detail", "tone": 0.51, "kind": "extrude", "profile": profile, "plane": "xy", "from": z0, "to": z1}
    solid.update(more)
    return solid


BOTTLE_BODY_50 = [[0.03, 0], [0.031, 0.04], [0.036, 0.075], [0.045, 0.102], [0.058, 0.124], [0.075, 0.14], [0.095, 0.15], [0.43, 0.15], [0.455, 0.146], [0.478, 0.132], [0.497, 0.11], [0.511, 0.082], [0.519, 0.055], [0.523, 0.03], [0.525, 0]]
HOSE = [[0.536, 0, -0.088], [0.575, 0.03, -0.125], [0.635, 0.03, -0.09], [0.672, 0, -0.02], [0.7, 0, 0]]


def bottle_50():
    collar_open = 1.4
    parts = [
        revolve("bottle-foot", "bottle", "bottle-steel", "detail", 0.4, ring(0, 0.058, 0.117, 0.123)),
        revolve("bottle-body", "bottle", "bottle-steel", "body", 0.45, BOTTLE_BODY_50),
        revolve("bottle-collar", "bottle", "bottle-steel", "detail", 0.4, ring(0.49, 0.58, 0.087, 0.092), sweepRad=round(2 * math.pi - collar_open, 6), transform={"rotation": quat(math.pi + collar_open / 2)}),
        revolve("bottle-valve", "bottle", "stainless-brushed", "detail", 0.3, [[0.515, 0], [0.515, 0.016], [0.552, 0.016], [0.552, 0.028], [0.562, 0.028], [0.562, 0]]),
        dict(revolve("regulator-nut", "bottle", "stainless-brushed", "detail", 0.35, [[0, 0], [0, 0.011], [0.022, 0.011], [0.022, 0]]), axis=[0, 0, -1], origin=[0.54, 0, -0.014]),
        dict(revolve("regulator", "bottle", "stainless-brushed", "detail", 0.4, [[0.515, 0], [0.515, 0.024], [0.521, 0.03], [0.545, 0.03], [0.553, 0.02], [0.557, 0.008], [0.557, 0]]), origin=[0, 0, -0.062]),
    ]
    for index in range(4):
        parts.append(capsule("hose-%d" % index, "bottle", "plastic-black", 0.2, HOSE[index], HOSE[index + 1], 0.0065))
    return parts


def louvres():
    rows = [(0.135, 0.143), (0.152, 0.16), (0.598, 0.606), (0.615, 0.623)]
    columns = [math.pi - 0.27, math.pi, math.pi + 0.27]
    out = []
    for r, (u0, u1) in enumerate(rows):
        for c, centre in enumerate(columns):
            out.append(revolve("louvre-%d" % (r * 3 + c), "door", "powder-steel", "texture", 0.15, ring(u0, u1, 0.2097, 0.2115), sweepRad=0.2, transform={"rotation": quat(centre - 0.1), "translation": TO_DOOR}, texture=True))
    return out


def hinges():
    return [revolve("hinge-low", "door", "stainless-brushed", "detail", 0.4, [[0.05, 0], [0.05, 0.006], [0.11, 0.006], [0.11, 0]]), revolve("hinge-high", "door", "stainless-brushed", "detail", 0.4, [[0.45, 0], [0.45, 0.006], [0.51, 0.006], [0.51, 0]])]


def latch():
    v, w = -0.1084, -0.1798
    return {"id": "door-latch", "kind": "revolve", "bone": "door", "material": "stainless-brushed", "role": "detail", "tone": 0.3, "profile": [[-0.002, 0], [-0.002, 0.011], [0.008, 0.011], [0.012, 0.007], [0.012, 0]], "axis": [0, 0, -1], "origin": [0.38 - HINGE[0], round(v - HINGE[1], 5), round(w - HINGE[2], 5)]}


def volumes_50(v):
    body = v["body"]
    drop = ["housing-wall", "door", "door-latch"] + ["louvre-%d" % i for i in range(12)] + WHEEL_OLD
    kept = {entry["id"]: entry for entry in body}
    new = [
        revolve("housing-sill", "base", "powder-steel", "body", 0.6, ring(0.04, 0.1, 0.2088, 0.21)),
        arc_wall("housing-wall", 0.1, 0.66, 0.2088, 0.21),
        revolve("housing-head", "base", "powder-steel", "body", 0.6, ring(0.66, 0.7, 0.2088, 0.21)),
        door_leaf(0.1, 0.66, 0.2088, 0.21),
    ] + louvres() + [latch()] + hinges() + bottle_50() + [
        wheel("tyre-left", 0.34, "plastic-black", "body", 0.23, TYRE), wheel("hub-left", 0.34, "stainless-brushed", "detail", 0.62, HUB),
        wheel("tyre-right", 0.02, "plastic-black", "body", 0.23, TYRE), wheel("hub-right", 0.02, "stainless-brushed", "detail", 0.62, HUB),
        wheel("axle", 0.18, "stainless-brushed", "detail", 0.48, AXLE),
        kept["bracket-left"], kept["bush-left"], kept["bracket-right"], kept["bush-right"],
    ]
    v["body"] = replace(body, drop, new)
    hidden = ["bottle-foot", "bottle-body", "bottle-collar", "bottle-valve", "regulator-nut", "regulator", "hose-0", "hose-1", "hose-2", "hose-3"]
    v["thermal"] = replace(v["thermal"], [], [th("hinge-low", "base-housing"), th("hinge-high", "base-housing")])
    bottle_refs(v, [s for s in new if s["id"] in hidden])
    structure = replace(v["structure"], ["st-" + i for i in WHEEL_OLD], [
        st("housing-sill", "base"), st("housing-head", "base"), st("door", "door"), st("hinge-low", "door", "joint"), st("hinge-high", "door", "joint"),
        st("tyre-left", "wheels"), st("hub-left", "wheels", "joint"), st("tyre-right", "wheels"), st("hub-right", "wheels", "joint"), st("axle", "wheels"),
        st("bracket-left", "base"), st("bush-left", "wheels", "joint"), st("bracket-right", "base"), st("bush-right", "wheels", "joint"),
    ])
    v["structure"] = structure


WHEEL_100 = [[-0.015, 0.006], [-0.015, 0.04], [-0.011, 0.045], [0.011, 0.045], [0.015, 0.04], [0.015, 0.006], [-0.015, 0.006]]
AXLE_PLAIN = [[-0.18, 0], [-0.18, 0.006], [0.18, 0.006], [0.18, 0]]


def wheel_refs(v, ids_wheels, ids_base):
    v["thermal"] = replace(v["thermal"], ["th-" + i for i in WHEEL_OLD], [th(i, "base-housing") for i in ids_wheels + ids_base])
    v["structure"] = replace(v["structure"], ["st-" + i for i in WHEEL_OLD], [st(i, "wheels") for i in ids_wheels] + [st(i, "base") for i in ids_base])


def bottle_refs(v, solids):
    v["thermal"] = replace(v["thermal"], [], [dict(s, id="th-" + s["id"], region="gas-bottle") for s in solids])


def knob(v, drop, stem=None, body=None):
    v["body"] = replace(v["body"], drop, [body] if body else [])
    v["thermal"] = replace(v["thermal"], ["th-" + i for i in drop], [])
    v["structure"] = replace(v["structure"], [] if stem else ["st-knob-stem"], [capsule("st-knob-stem", "knob", "stainless-brushed", 0.3, [-0.012, 0, 0], [0.004, 0, 0], stem, member="knob", part="joint")] if stem else [])


def volumes_100(v):
    old = {entry["id"]: entry for entry in v["thermal"] if "kind" in entry}
    bottle = []
    for name in ("bottle-body", "bottle-collar"):
        source = old.get("th-" + name) or next(entry for entry in v["body"] if entry["id"] == name)
        bottle.append(dict({k: val for k, val in source.items() if k != "region"}, id=name))
    new = [
        revolve("housing-sill", "base", "powder-steel", "body", 0.6, ring(0.04, 0.1, 0.2088, 0.21)),
        arc_wall("housing-wall", 0.1, 0.66, 0.2088, 0.21),
        revolve("housing-head", "base", "powder-steel", "body", 0.6, ring(0.66, 0.7, 0.2088, 0.21)),
        door_leaf(0.1, 0.66, 0.2088, 0.21),
    ] + bottle + [
        wheel("wheel-left", 0.34, "plastic-black", "body", 0.23, WHEEL_100), wheel("wheel-right", 0.02, "plastic-black", "body", 0.23, WHEEL_100),
        wheel("axle", 0.18, "stainless-brushed", "detail", 0.48, AXLE_PLAIN, slender=True),
        bracket("bracket-left", 0.086, 0.101), bracket("bracket-right", -0.101, -0.086),
    ]
    v["body"] = replace(v["body"], ["housing-wall", "door"] + WHEEL_OLD, new)
    bottle_refs(v, bottle)
    wheel_refs(v, ["wheel-left", "wheel-right", "axle"], ["bracket-left", "bracket-right"])
    v["structure"] = replace(v["structure"], [], [st("housing-sill", "base"), st("housing-head", "base"), st("door", "door")])
    knob(v, ["knob-grip"], stem=0.0075)


DISC = [[-0.015, 0], [-0.015, 0.045], [0.015, 0.045], [0.015, 0]]


def one_bottle(v):
    old = next((entry for entry in v["thermal"] if entry["id"] == "th-bottle-body" and "kind" in entry), None) or next(entry for entry in v["body"] if entry["id"] == "bottle-body")
    return dict({k: val for k, val in old.items() if k != "region"}, id="bottle-body")


def volumes_200(v):
    bottle = one_bottle(v)
    base = [[0, 0], [0, 0.235], [0.04, 0.235], [0.04, 0.21], [0.1, 0.21], [0.1, 0.2088], [0.04, 0.2088], [0.04, 0], [0, 0]]
    top = [[0.66, 0.2088], [0.66, 0.21], [0.7, 0.21], [0.712184, 0.206253], [0.718899, 0.195004], [0.723811, 0.176167], [0.727257, 0.149314], [0.729315, 0.112605], [0.7299, 0.05], [0.755, 0.05], [0.755, 0.03], [0.7288, 0.03], [0.728115, 0.112573], [0.726061, 0.14921], [0.722631, 0.175951], [0.71778, 0.194571], [0.711438, 0.205313], [0.699993, 0.2088], [0.66, 0.2088]]
    new = [
        revolve("housing", "base", "powder-steel", "body", 0.6, base), arc_wall("housing-wall", 0.1, 0.66, 0.2088, 0.21), revolve("housing-top", "base", "powder-steel", "body", 0.6, top),
        door_leaf(0.1, 0.66, 0.2088, 0.21), bottle,
        wheel("wheel-left", 0.34, "plastic-black", "body", 0.23, DISC), wheel("wheel-right", 0.02, "plastic-black", "body", 0.23, DISC),
        wheel("axle", 0.18, "stainless-brushed", "detail", 0.48, AXLE_PLAIN, slender=True),
        bracket("bracket-left", 0.08, 0.11), bracket("bracket-right", -0.11, -0.08),
    ]
    v["body"] = replace(v["body"], WHEEL_OLD, new)
    bottle_refs(v, [bottle])
    wheel_refs(v, ["wheel-left", "wheel-right", "axle"], ["bracket-left", "bracket-right"])
    v["structure"] = replace(v["structure"], [], [st("housing-wall", "base"), st("housing-top", "base"), st("door", "door")])
    knob(v, ["knob-grip", "knob-boss"])


def volumes_500(v):
    bottle = one_bottle(v)
    base = [[0, 0], [0, 0.225], [0.1, 0.225], [0.1, 0.15], [0.075, 0.15], [0.075, 0], [0, 0]]
    top = [[0.66, 0.0375], [0.66, 0.225], [0.695, 0.225], [0.718, 0.21], [0.73, 0.18], [0.735, 0.075], [0.755, 0.0375], [0.66, 0.0375]]
    roller = [[-0.18, 0], [-0.18, 0.045], [0.18, 0.045], [0.18, 0]]
    new = [
        revolve("housing", "base", "powder-steel", "body", 0.6, base), arc_wall("housing-wall", 0.1, 0.66, 0.15, 0.225), revolve("housing-top", "base", "powder-steel", "body", 0.6, top),
        door_leaf(0.1, 0.66, 0.15, 0.225), bottle,
        wheel("wheel-set", 0.18, "plastic-black", "body", 0.23, roller),
        bracket("wheel-bracket", -0.12, 0.12, [[0.005, 0.185], [0.005, 0.262], [0.08, 0.262], [0.08, 0.185]]),
    ]
    v["body"] = replace(v["body"], WHEEL_OLD, new)
    bottle_refs(v, [bottle])
    wheel_refs(v, ["wheel-set"], ["wheel-bracket"])
    v["structure"] = replace(v["structure"], [], [st("housing-wall", "base"), st("housing-top", "base"), st("door", "door")])
    knob(v, ["knob-grip", "knob-boss"], body=revolve("knob", "knob", "plastic-black", "body", 0.2, [[-0.0375, 0], [-0.0375, 0.0375], [0.03, 0.0375], [0.0375, 0.03], [0.0375, 0]]))


DECISIONS = {
    "50": {"en": "Rework: the access door is a real leaf of 1.2 mm sheet, flush in an opening cut out of the housing wall between a sill ring and a head ring; it hangs on two 12 mm pin hinges at its rear edge on its own bone and swings out by up to 110 degrees, carrying its twelve louvres and the latch. Behind it stands the 11 kg bottle as part of the body: foot ring, shell, collar with the handle opening towards the door, valve, the regulator on its union nut and the 13 mm hose rising in a loop into the post. Tyres, hubs and the 12 mm axle are one wheel set on a bone that lies along the axle and rolls in the bushes of the two pressed brackets.", "de": "Überarbeitung: die Zugangstür ist ein echtes Blatt aus 1,2 mm Blech, bündig in einer Öffnung, die zwischen einem Sockelring und einem Kopfring aus der Gehäusewand geschnitten ist; sie hängt an zwei Stiftbändern von 12 mm an ihrer hinteren Kante auf einem eigenen Knochen und schwenkt bis 110 Grad auf, mit ihren zwölf Lüftungsschlitzen und dem Riegel. Dahinter steht die 11-kg-Flasche als Teil des Körpers: Fußring, Mantel, Kragen mit der Grifföffnung zur Tür, Ventil, der Regler an seiner Überwurfmutter und der Schlauch von 13 mm, der in einer Schleife in den Mast steigt. Reifen, Naben und die Achse von 12 mm sind ein Radsatz auf einem Knochen, der in der Achse liegt und in den Buchsen der beiden gepressten Halter rollt."},
    "100": {"en": "Rework for the 15 mm feature: the door is a flush leaf on its hinge bone in a wall with a real opening (hinges, louvres and latch are under the feature); the bottle stands behind it as shell and collar; each wheel is one bored body of tyre and hub on the axle bone, the axle a slender 12 mm rod, each bracket one 15 mm plate.", "de": "Überarbeitung für die Einzelheit von 15 mm: die Tür ist ein bündiges Blatt auf ihrem Bandknochen in einer Wand mit echter Öffnung (Bänder, Schlitze und Riegel liegen unter der Einzelheit); dahinter steht die Flasche als Mantel und Kragen; jedes Rad ist ein gebohrter Körper aus Reifen und Nabe auf dem Achsknochen, die Achse ein schlanker Stab von 12 mm, jeder Halter eine Platte von 15 mm."},
    "200": {"en": "Rework for the 30 mm feature: the housing is pan with sill, the wall with the door opening and the top with lid and socket; the door is one leaf on its hinge bone, the bottle one body behind it; each wheel is one 90 x 30 mm disc on the axle bone, the axle a slender rod, each bracket a 30 mm block.", "de": "Überarbeitung für die Einzelheit von 30 mm: das Gehäuse besteht aus Wanne mit Sockel, der Wand mit der Türöffnung und dem Oberteil mit Deckel und Muffe; die Tür ist ein Blatt auf ihrem Bandknochen, die Flasche ein Körper dahinter; jedes Rad ist eine Scheibe 90 x 30 mm auf dem Achsknochen, die Achse ein schlanker Stab, jeder Halter ein Block von 30 mm."},
    "500": {"en": "Rework for the 75 mm feature: the can keeps its 75 mm wall, cut into base, wall with the door opening and top; the door is one 75 mm leaf on its hinge bone, the bottle one body behind it; wheels and axle are one 90 mm roller across the rear on the axle bone, carried by one bracket block on the pan.", "de": "Überarbeitung für die Einzelheit von 75 mm: die Dose behält ihre Wand von 75 mm, geteilt in Boden, Wand mit der Türöffnung und Oberteil; die Tür ist ein Blatt von 75 mm auf ihrem Bandknochen, die Flasche ein Körper dahinter; Räder und Achse sind eine Rolle von 90 mm quer am Heck auf dem Achsknochen, getragen von einem Halterblock an der Wanne."},
}


def head(vol):
    for scale in ("50", "100"):
        for solid in vol[scale]["body"]:
            if solid["id"] == "guard-rod":
                solid["p0"][0], solid["p1"][0] = 0.143, 0.357
    vol["50"]["structure"] = replace(vol["50"]["structure"], [], [st("emitter-screen", "burner-head"), st("screen-band-low", "burner-head", "joint"), st("screen-band-high", "burner-head", "joint")])
    for solid in vol["200"]["structure"]:
        if solid["id"] == "st-valve-housing":
            solid["profile"] = [[0, 0], [0, 0.036], [0.04, 0.085], [0.118, 0.085], [0.136, 0.114], [0.14, 0.114], [0.14, 0], [0.125, 0], [0.130285, 0.073571], [0.044955, 0.070842], [0.013572, 0.029613], [0.015, 0], [0, 0]]
        if solid["id"] == "st-spacer":
            solid["p0"][0], solid["p1"][0] = 0.01, 0.165


def volumes(vol):
    head(vol)
    volumes_50(vol["50"])
    volumes_100(vol["100"])
    volumes_200(vol["200"])
    volumes_500(vol["500"])
    for scale, text in DECISIONS.items():
        decisions = [d for d in vol[scale]["decisions"] if not d["en"].startswith("Rework")]
        decisions.insert(3 if len(decisions) >= 3 else len(decisions), text)
        vol[scale]["decisions"] = decisions
    return vol


def rig(asset):
    bones = [bone for bone in asset["rig"]["bones"] if bone["id"] not in ("door", "wheels")]
    for bone in bones:
        if bone["id"] == "base":
            bone["mass"]["kg"] = 6.4
    at = next(i for i, bone in enumerate(bones) if bone["id"] == "bottle")
    bones[at + 1:at + 1] = [
        {"id": "door", "parent": "base", "title": {"en": "Access door of the bottle housing on its pin hinges", "de": "Zugangstür des Flaschengehäuses an ihren Stiftbändern"}, "x": HINGE[0], "y": HINGE[1], "z": HINGE[2], "angleDeg": 0, "lengthM": 0.56, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, "mass": {"kg": 1.7, "comX": 0.28, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "wheels", "parent": "base", "title": {"en": "Wheel set: two wheels on the rear axle", "de": "Radsatz: zwei Räder auf der hinteren Achse"}, "x": 0.05, "y": 0.26, "z": -0.18, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0.36, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -180, "twistMaxDeg": 180}, "mass": {"kg": 0.6, "comX": 0.18, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]
    asset["rig"]["bones"] = bones


def poses(asset):
    kept = [pose for pose in asset["poses"] if pose["id"] != "bottle-change"]
    pose = json.loads(json.dumps(next(p for p in kept if p["id"] == "off-afternoon")))
    pose.update({"id": "bottle-change", "title": {"en": "Door open for the bottle change", "de": "Tür offen zum Flaschenwechsel"}, "description": {"en": "Cold heater, knob at off; the access door stands open by 105 degrees and shows the bottle with its regulator and hose.", "de": "Kalter Heizpilz, Knopf auf Aus; die Zugangstür steht 105 Grad offen und zeigt die Flasche mit Regler und Schlauch."}})
    pose["bones"] = {"knob": {"twistDeg": 0}, "door": {"twistDeg": OPEN_DEG}}
    pose["thermal"]["regions"]["gas-bottle"] = {"exposedFraction": 0.6, "sunlitFraction": 0.2}
    asset["poses"] = kept + [pose]
    clips = [clip for clip in asset["clips"] if clip["id"] != "open-for-bottle-change"]
    clips.append({"id": "open-for-bottle-change", "title": {"en": "Open the door for the bottle change", "de": "Tür zum Flaschenwechsel öffnen"}, "description": {"en": "The cold heater's access door swings open on its hinges, stays open while the bottle is changed and closes again.", "de": "Die Zugangstür des kalten Heizpilzes schwenkt an ihren Bändern auf, bleibt beim Flaschenwechsel offen und schließt wieder."}, "durationS": 8, "fps": 12, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "off-afternoon", "ease": "smooth"}, {"t": 2, "pose": "bottle-change", "ease": "smooth"}, {"t": 6, "pose": "bottle-change", "ease": "smooth"}, {"t": 8, "pose": "off-afternoon", "ease": "smooth"}], "contacts": [{"anchor": "base-contact", "fromS": 0, "toS": 8}], "phases": [{"id": "opening", "fromS": 0, "toS": 2, "title": {"en": "Door opens", "de": "Tür öffnet"}}, {"id": "open", "fromS": 2, "toS": 6, "title": {"en": "Door open", "de": "Tür offen"}}, {"id": "closing", "fromS": 6, "toS": 8, "title": {"en": "Door closes", "de": "Tür schließt"}}], "events": [], "tracks": []})
    asset["clips"] = clips
    for region in asset["thermal"]["regions"]:
        if region["id"] == "gas-bottle":
            region["hidden"] = {"en": "The bottle stands on the base pan inside the closed housing behind the access door; only the pose with the door open shows it.", "de": "Die Flasche steht auf der Bodenwanne im geschlossenen Gehäuse hinter der Zugangstür; nur die Pose mit offener Tür zeigt sie."}


def poly(id, bone, role, z, tone, points, filled, closed=True, **more):
    shape = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": points, "closed": closed}
    shape.update(more)
    return shape


def door_shapes(scale, tone):
    leaf = {"kind": "extruded", "sweep": "w", "z0": -0.21 if scale != "500" else -0.225, "z1": round(HINGE[2], 4)}
    width = round(2 * HINGE[1], 4)
    bottle = [[0.06, -0.143], [0.455, -0.143], [0.478, -0.132], [0.497, -0.11], [0.506, -0.092], [0.58, -0.092], [0.58, 0.092], [0.506, 0.092], [0.497, 0.11], [0.478, 0.132], [0.455, 0.143], [0.06, 0.143]]
    shapes = [
        poly("door-opening", "base", "detail" if scale != "500" else "body", 3.32, 0.12, [[0.1, SEAM_V], [0.1, -SEAM_V], [0.66, -SEAM_V], [0.66, SEAM_V]], True, region="base-housing"),
        poly("bottle", "bottle", "detail" if scale != "500" else "body", 3.34, 0.45, bottle, True),
        poly("door", "door", "body", 3.38, tone, [[0, 0], [0, -width], [0.56, -width], [0.56, 0]], True, solid=leaf, region="base-housing"),
    ]
    if scale != "500":
        shapes.append(poly("door-seam", "door", "detail", 3.4, 0.3, [[0, 0], [0, -width], [0.56, -width], [0.56, 0]], False, solid=leaf, region="base-housing"))
    if scale == "50":
        skin = {"kind": "extruded", "sweep": "w", "z0": -0.2125, "z1": -0.2095}
        rows = [0.139, 0.156, 0.602, 0.619]
        columns = [(-0.076662, -0.035867), (-0.021165, 0.021165), (0.035867, 0.076662)]
        for r, u in enumerate(rows):
            for c, (v0, v1) in enumerate(columns):
                shapes.append(poly("vent-slot-%d" % (r * 3 + c), "door", "texture", 3.45, 0.3, [[round(u - 0.1, 4), round(v0 - HINGE[1], 5)], [round(u - 0.1, 4), round(v1 - HINGE[1], 5)]], False, closed=False, solid=skin, region="base-housing"))
        knob = {"kind": "extruded", "sweep": "w", "z0": -0.222, "z1": -0.209}
        ring_points = [[round(0.28 + 0.011 * math.sin(k * math.pi / 5), 5), round(-0.1084 - HINGE[1] - 0.011 * math.cos(k * math.pi / 5), 5)] for k in range(10)]
        shapes.append(poly("door-latch", "door", "detail", 3.45, 0.3, ring_points, True, solid=knob))
        shapes.append(poly("door-latch-edge", "door", "detail", 3.5, 0.3, ring_points, False, solid=knob, region="base-housing"))
    return shapes


def drawings(asset):
    for scale, representation in asset["representations"].items():
        elevation = representation["geometry"]["elevation"]
        tone = next(shape["tone"] for shape in elevation if shape["id"] == "base")
        new = door_shapes(scale, tone)
        ids = {shape["id"] for shape in new} | {"door-seam", "door-latch", "door-latch-edge"} | {"vent-slot-%d" % i for i in range(12)}
        rest = [shape for shape in elevation if shape["id"] not in ids]
        at = next(i for i, shape in enumerate(rest) if shape["bone"] == "pole")
        representation["geometry"]["elevation"] = rest[:at] + new + rest[at:]
        outer = -0.225 if scale == "500" else -0.21
        role = "body" if scale == "500" else "detail"
        front = [shape for shape in representation["geometry"]["front"] if shape["id"] != "door"]
        front.insert(1, poly("door", "door", "body", 3.05, tone, [[outer, 0.1], [round(HINGE[2], 4), 0.1], [round(HINGE[2], 4), 0.66], [outer, 0.66]], True, region="base-housing"))
        representation["geometry"]["front"] = front
        arc = [[round(HINGE_R * math.sin(k * 0.25), 4), round(-HINGE_R * math.cos(k * 0.25), 4)] for k in range(-3, 4)]
        plan = [shape for shape in representation["geometry"]["plan"] if shape["id"] != "door"]
        plan.insert(0, poly("door", "door", role, 3.0, 0.3, arc, False, closed=False, region="base-housing"))
        representation["geometry"]["plan"] = plan
        text = representation["decisions"]["silhouette"]
        if "bottle change" not in text["en"]:
            text["en"] += " The access door is drawn as a leaf on its hinge bone in all three views over the dark opening and the bottle behind it, so the pose for the bottle change shows the open housing."
            text["de"] += " Die Zugangstür ist in allen drei Ansichten als Blatt auf ihrem Bandknochen über der dunklen Öffnung und der Flasche dahinter gezeichnet, so dass die Pose zum Flaschenwechsel das offene Gehäuse zeigt."


def main():
    document = json.load(open(CATALOG))
    asset = document["asset"]
    source = json.load(open(SOURCE))
    source["volumes"] = volumes(source["volumes"])
    rig(asset)
    poses(asset)
    drawings(asset)
    asset["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CAND), exist_ok=True)
    json.dump(document, open(CAND, "w"), ensure_ascii=False, indent=2)
    print("candidate", CAND)
    if "--apply" in sys.argv:
        json.dump(source, open(SOURCE, "w"), ensure_ascii=False, indent=2)
        json.dump(document, open(CATALOG, "w"), ensure_ascii=False, indent=2)
        print("applied", SOURCE, CATALOG)


main()
