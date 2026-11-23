"""⛱️ Tied canopy, coarse scales modelled for their own smallest feature (15 / 30 / 75 mm): the 1:100, 1:200 and 1:500 lists of `volumes/components/tied-canopy.json` rebuilt by hand to what the 2D drawing of each scale retains, merges and omits.

Usage: python3 rework-tied-canopy-coarse.py   (rewrites the three coarse scales of the volumes source; 1:50 stays as it is)
Frame of bone `canopy`: u out of the wall along the beam, v up, w along the wall; origin on the outer wall face at the beam axis.
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "components", "tied-canopy.json")

BEAM_LENGTH = 5.5
BEAM_TOP = 0.08
GLASS_TOP = 0.1048
TUBE_RADIUS = 0.03015
PIN_BEAM = [3.763889, 0.17]
PIN_WALL = [0.09, 2.8152]
STRUT_AXIS = [-0.811534, 0.584305, 0.0]
PIN_TO_PIN = math.hypot(PIN_BEAM[0] - PIN_WALL[0], PIN_WALL[1] - PIN_BEAM[1])
STRUT_ROOF_END = -(3.854444 - PIN_BEAM[0]) / 0.811534
STRUT_WALL_END = PIN_BEAM[0] / 0.811534
WALL_OUTLINE = [[-2.33, -0.5], [-2.33, 0.5], [3.02, 0.5], [3.02, -0.5]]
PAD_HOLES = [[[-0.12, -0.1], [-0.12, 0.1], [0.12, 0.1], [0.12, -0.1]], [[2.76, -0.1], [2.76, 0.1], [3.0, 0.1], [3.0, -0.1]]]
NODE_BEAM = [[3.665206, 0.204084], [3.735, 0.0775], [3.995, 0.0775], [3.786112, 0.203259], [3.700265, 0.252777]]
NODE_WALL = [[-0.002, 2.785], [0.153624, 2.732423], [0.188683, 2.781115], [-0.002, 2.975]]
NODE_HALF_WIDTH = 0.02
STUD_GROUP = [0.25, 0.162, 0.132]

DECISIONS = {
    "100": [
        {"en": "Smallest feature 15 mm: nuts, clamp plates and the 8 mm tip plate are left out; the beam runs the full 5.5 m; each pane and its EPDM bearing are one 24.8 mm glazing layer on the top flange; the two 10 mm cheeks of each fork head are one 37 mm head around the 15 mm gusset; the studs end flush with the 20 mm plates.", "de": "Kleinstes Merkmal 15 mm: Muttern, Klemmplatten und die 8-mm-Stirnplatte entfallen; der Träger läuft über die vollen 5,5 m; jede Scheibe und ihr EPDM-Auflager sind eine 24,8 mm starke Verglasungsschicht auf dem Obergurt; die zwei 10-mm-Wangen jedes Gabelkopfs sind ein 37 mm breiter Kopf um das 15-mm-Knotenblech; die Anker enden bündig mit den 20-mm-Platten."},
        {"en": "Wall layers, anchor pads and plates, the four panes with their joint gaskets and the notch, the tube with both turnbuckle bodies, pins and gussets stay separate solids.", "de": "Wandschichten, Trennklötze und Platten, die vier Scheiben mit Fugendichtungen und Aussparung, das Rohr mit beiden Spannschlosskörpern, Bolzen und Knotenblechen bleiben eigene Körper."},
    ],
    "200": [
        {"en": "Smallest feature 30 mm (24.8 mm drawn for the glazing): render and insulation are one 200 mm outer layer in front of the concrete; each anchor pad takes in its 20 mm plate; the roof is one glazing layer over the whole beam; gusset, fork head, bridge and pin of each strut end are one 40 mm node lug; the strut is one 60.3 mm bar from pin to pin.", "de": "Kleinstes Merkmal 30 mm (24,8 mm gezeichnet für die Verglasung): Putz und Dämmung sind eine 200 mm starke Außenschicht vor dem Beton; jeder Trennklotz nimmt seine 20-mm-Platte auf; das Dach ist eine Verglasungsschicht über den ganzen Träger; Knotenblech, Gabelkopf, Steg und Bolzen jedes Stabendes sind eine 40 mm breite Knotenlasche; der Stab ist ein 60,3-mm-Stab von Bolzen zu Bolzen."},
        {"en": "Left out: nuts, clamp plates, joint gaskets, the fork notch and the studs in the concrete (the structure list carries each stud group as one anchorage block behind its pad).", "de": "Entfallen: Muttern, Klemmplatten, Fugendichtungen, die Gabelaussparung und die Anker im Beton (die Tragwerksliste führt jede Ankergruppe als einen Verankerungsblock hinter ihrem Klotz)."},
    ],
    "500": [
        {"en": "Smallest feature 75 mm: the wall is one 500 mm bar, the beam a plain 82 x 160 mm bar, the roof one glazing layer on it and the strut one line of true 60.3 mm section from the wall face to the roof, as the drawing shows it.", "de": "Kleinstes Merkmal 75 mm: Die Wand ist ein 500 mm starker Balken, der Träger ein glatter Stab 82 x 160 mm, das Dach eine Verglasungsschicht darauf und der Stab eine Linie mit wahrem Querschnitt 60,3 mm von der Wandfläche bis zum Dach, wie die Zeichnung ihn zeigt."},
        {"en": "Left out: wall layers, anchor pads and plates, node lugs, joints and every fixing; the thermal list keeps the render face and the two plate faces as layer shells.", "de": "Entfallen: Wandschichten, Trennklötze und Platten, Knotenlaschen, Fugen und alle Befestigungen; die Thermikliste behält die Putzfläche und die zwei Plattenflächen als Schichtschalen."},
    ],
}


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def keep(solids, ids):
    table = by_id(solids)
    return [table[name] for name in ids]


def bar(a, b):
    return [[a, 0.0], [a, TUBE_RADIUS], [b, TUBE_RADIUS], [b, 0.0]]


def glazing(identifier, profile):
    return {"id": identifier, "kind": "extrude", "bone": "canopy", "material": "glass", "role": "glazing", "tone": 0.92, "profile": profile, "plane": "zx", "from": BEAM_TOP, "to": GLASS_TOP}


def lug(identifier, profile):
    return {"id": identifier, "kind": "extrude", "bone": "canopy", "material": "steel", "role": "body", "tone": 0.3, "profile": profile, "plane": "xy", "from": -NODE_HALF_WIDTH, "to": NODE_HALF_WIDTH}


def plate_shell(identifier, source):
    shell = {key: value for key, value in source.items() if key not in ("id", "role")}
    return {"id": identifier, **shell, "role": "body", "region": "wall-plate", "layer": "plate-steel", "sectionFaces": ["-u"]}


def scale_100(volume, fine):
    body = by_id(volume["body"])
    body["beam"]["to"] = BEAM_LENGTH
    for name in ("glass-0", "glass-1", "glass-2", "glass-3", "notch-seal"):
        body[name]["from"] = BEAM_TOP
    for name in ("joint-wall", "joint-0", "joint-1", "joint-2"):
        body[name]["size"][1] = round(GLASS_TOP - BEAM_TOP, 6)
        body[name]["transform"]["translation"][1] = round((GLASS_TOP + BEAM_TOP) / 2, 6)
    for name in ("clevis-beam", "clevis-wall"):
        body[name]["from"], body[name]["to"] = -0.0185, 0.0185
        body[name].pop("repeat", None)
    for name in ("stud-beam-top", "stud-beam-bottom", "stud-strut-top", "stud-strut-bottom"):
        body[name]["profile"] = [[0.0, 0.0], [0.0, 0.008], [0.45, 0.008], [0.45, 0.0]]
    gone = {"nut-beam-top", "nut-beam-bottom", "nut-strut-top", "nut-strut-bottom", "beam-end-plate", "bearing-wall", "bearing-tip", "clamp-0", "clamp-1", "clamp-2", "clamp-3", "clamp-4"}
    volume["body"] = [solid for solid in volume["body"] if solid["id"] not in gone]
    volume["thermal"] = [solid for solid in volume["thermal"] if solid.get("of") not in gone]
    volume["structure"] = [solid for solid in volume["structure"] if solid.get("of") not in gone]
    structure = by_id(volume["structure"])
    structure["m-beam"]["to"] = BEAM_LENGTH
    for name in ("j-stud-beam-top", "j-stud-beam-bottom", "j-stud-strut-top", "j-stud-strut-bottom"):
        structure[name]["profile"] = [[0.0, 0.0], [0.0, 0.008], [0.45, 0.008], [0.45, 0.0]]


def scale_200(volume, fine):
    body = by_id(volume["body"])
    thermal = by_id(volume["thermal"])
    structure = by_id(volume["structure"])
    plate = by_id(fine["body"])["plate-beam"]
    body["beam"]["to"] = BEAM_LENGTH
    outer = {"id": "wall-outer", "kind": "extrude", "bone": "canopy", "material": "mineral-wool", "role": "body", "tone": 0.9, "profile": WALL_OUTLINE, "holes": PAD_HOLES, "plane": "yz", "from": -0.2, "to": 0.0}
    for name in ("pad-beam", "pad-strut"):
        body[name]["to"] = 0.0
        body[name].pop("holes", None)
    strut = {"id": "strut-tube", "kind": "revolve", "bone": "canopy", "material": "steel", "role": "body", "tone": 0.45, "profile": bar(0.0, round(PIN_TO_PIN, 6)), "axis": STRUT_AXIS, "origin": [PIN_BEAM[0], PIN_BEAM[1], 0.0]}
    volume["body"] = [body["wall-core"], outer, body["pad-beam"], body["pad-strut"], body["beam"], glazing("glass-roof", [[-0.5, 0.0], [0.5, 0.0], [0.5, BEAM_LENGTH], [-0.5, BEAM_LENGTH]]), lug("node-beam", NODE_BEAM), lug("node-wall", NODE_WALL), strut]
    render = [solid for solid in volume["thermal"] if solid["id"].startswith("th-render-face")]
    walls = [solid for solid in volume["thermal"] if solid.get("of") in ("wall-core", "pad-beam", "pad-strut") or solid["id"].startswith(("th-concrete", "th-wall-wool"))]
    volume["thermal"] = walls + [
        plate_shell("th-plate-beam", plate),
        thermal["th-plate-strut"],
        {"id": "th-beam", "of": "beam", "region": "canopy-steel"},
        {"id": "th-glass-roof", "of": "glass-roof", "region": "canopy-glass", "sectionFaces": ["-v"]},
        {"id": "th-node-beam", "of": "node-beam", "region": "rod-steel"},
        {"id": "th-node-wall", "of": "node-wall", "region": "rod-steel"},
        {"id": "th-strut-tube", "of": "strut-tube", "region": "rod-steel"},
    ] + render

    def studs(identifier, height):
        return {"id": identifier, "kind": "box", "bone": "canopy", "material": "steel", "role": "body", "tone": 0.2, "size": STUD_GROUP, "transform": {"translation": [-0.325, height, 0.0]}, "member": "canopy", "part": "joint"}

    volume["structure"] = [
        {"id": "m-beam", "of": "beam", "member": "canopy", "part": "member"},
        {"id": "m-strut", "of": "strut-tube", "member": "canopy", "part": "member"},
        {"id": "j-node-beam", "of": "node-beam", "member": "canopy", "part": "joint"},
        {"id": "j-node-wall", "of": "node-wall", "member": "canopy", "part": "joint"},
        {"id": "j-pad-beam", "of": "pad-beam", "member": "canopy", "part": "joint"},
        {"id": "j-pad-strut", "of": "pad-strut", "member": "canopy", "part": "joint"},
        studs("j-studs-beam", 0.0),
        studs("j-studs-strut", 2.88),
        structure["support-wall-pin"],
        structure["support-tie-rod"],
        structure["mass-beam"],
    ]


def scale_500(volume, fine):
    thermal = by_id(volume["thermal"])
    structure = by_id(volume["structure"])
    plate = by_id(fine["body"])["plate-beam"]
    wall = {"id": "wall", "kind": "extrude", "bone": "canopy", "material": "concrete", "role": "body", "tone": 0.6, "profile": WALL_OUTLINE, "plane": "yz", "from": -0.5, "to": 0.0}
    beam = {"id": "beam", "kind": "box", "bone": "canopy", "material": "steel", "role": "body", "tone": 0.4, "size": [BEAM_LENGTH, 0.16, 0.082], "transform": {"translation": [BEAM_LENGTH / 2, 0.0, 0.0]}}
    strut = {"id": "strut-tube", "kind": "revolve", "bone": "canopy", "material": "steel", "role": "body", "tone": 0.45, "slender": True, "profile": bar(round(STRUT_ROOF_END, 6), round(STRUT_WALL_END, 6)), "axis": STRUT_AXIS, "origin": [PIN_BEAM[0], PIN_BEAM[1], 0.0]}
    volume["body"] = [wall, beam, glazing("glass-roof", [[-0.5, 0.0], [0.5, 0.0], [0.5, BEAM_LENGTH], [-0.5, BEAM_LENGTH]]), strut]
    render = [solid for solid in volume["thermal"] if solid["id"].startswith("th-render-face")]
    volume["thermal"] = [
        plate_shell("th-plate-beam", plate),
        thermal["th-plate-strut"],
        {"id": "th-beam", "of": "beam", "region": "canopy-steel"},
        {"id": "th-glass-roof", "of": "glass-roof", "region": "canopy-glass", "sectionFaces": ["-v"]},
        {"id": "th-strut-tube", "of": "strut-tube", "region": "rod-steel"},
    ] + render
    volume["structure"] = [
        {"id": "m-beam", "of": "beam", "member": "canopy", "part": "member"},
        {"id": "m-strut", "of": "strut-tube", "member": "canopy", "part": "member"},
        structure["support-wall-pin"],
        structure["support-tie-rod"],
        structure["mass-beam"],
    ]


def main():
    document = json.load(open(SOURCE, encoding="utf-8"))
    volumes = document["volumes"]
    fine = volumes["50"]
    if "nut-beam-top" not in by_id(volumes["100"]["body"]):
        raise SystemExit("already applied: the coarse scales no longer carry the 1:50 parts list")
    for scale, build in (("100", scale_100), ("200", scale_200), ("500", scale_500)):
        build(volumes[scale], fine)
        volumes[scale]["decisions"] = DECISIONS[scale]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    for scale in ("100", "200", "500"):
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
