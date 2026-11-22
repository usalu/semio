"""🪜 Hand-set ladder cords of components/external-venetian-blind: the cords that carry and tilt the Z 90 lamellae.

Usage: python3 rework-external-venetian-blind-ladder.py [--write]
Without --write the candidate document goes to 🗑️generated/r3-external-venetian-blind/🔣️.json (for r3-validate/r3-sheet --file).
With --write the members rig, poses and structure of the catalog file are spliced in place and the volumes source is rewritten
(publish it afterwards with volumes-insert.py). The script is idempotent: it removes its own bones and solids first.

Reference (Z 90 raffstore, manufacturer catalogues): each lamella is clipped by its two beaded edges into a pair of polyester
ladder cords of 2.5 mm beside every lift tape; the cords run up into the tilt bearing on the 12 mm shaft, whose rocking pulls one
cord up and lets the other down, so every lamella turns about its centre line while the cords stay plumb. The lift tape only
carries the bottom rail, which is the guided, non-turning type: the cords end at the lowest lamella. Rig: one counter-rotating bone per bead (`cord-out-i`, `cord-in-i`, child of `slat-i`), so that a cord
segment weighted between two neighbouring bead bones translates with the beads, stays vertical in every tilt and shortens when
the pack stacks.
"""
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json"
SOURCE = os.path.join(T, "volumes/components/external-venetian-blind.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-external-venetian-blind/🔣️.json")

SLATS = 18
PITCH = 0.08
BEAD = {"out": (0.045, -0.008), "in": (-0.045, 0.008)}
CORD = 0.0025
TAPE_Z = 0.4
TAPE_SPAN = -0.8
BOX_Y = 2.32
SLAT0_Y = 2.405
BLIND_X = 0.085
BEARING_UNDERSIDE = 0.1505
BEARING_EXIT = {"out": 0.103, "in": 0.067}
SIDE = {"out": ({"en": "outer", "de": "äußere"}, {"en": "Outer", "de": "Äußere"}), "in": ({"en": "inner", "de": "innere"}, {"en": "Inner", "de": "Innere"})}


def bones():
    result = []
    for index in range(SLATS):
        for side, (x, y) in BEAD.items():
            result.append({
                "id": "cord-%s-%d" % (side, index),
                "parent": "slat-%d" % index,
                "title": {
                    "en": "%s ladder cord clip of slat %d (stays plumb while the slat tilts)" % (SIDE[side][1]["en"], index + 1),
                    "de": "%s Leiterkordel-Clip der Lamelle %d (bleibt lotrecht, während die Lamelle kippt)" % (SIDE[side][1]["de"], index + 1),
                },
                "x": x,
                "y": y,
                "angleDeg": 0,
                "lengthM": 0,
                "joint": {"type": "revolute", "minDeg": -35, "maxDeg": 90},
            })
    return result


def cord(identifier, bone, size_y, centre, weights, rotation=None):
    transform = {"translation": [round(centre[0], 6), round(centre[1], 6), TAPE_Z]}
    if rotation is not None:
        transform["rotation"] = [0, 0, round(math.sin(rotation / 2), 6), round(math.cos(rotation / 2), 6)]
    return {
        "id": identifier, "kind": "box", "bone": bone, "material": "polyamide", "role": "texture", "tone": 0.25,
        "size": [CORD, round(size_y, 6), CORD], "transform": transform,
        "repeat": {"count": 2, "translation": [0, 0, TAPE_SPAN]},
        "weights": [{"bone": name, "weight": weight} for name, weight in weights], "texture": True,
    }


def cords():
    result = []
    for side, (x, y) in BEAD.items():
        start = (BLIND_X + x, SLAT0_Y + y - BOX_Y)
        end = (BEARING_EXIT[side], BEARING_UNDERSIDE)
        dx, dy = end[0] - start[0], end[1] - start[1]
        result.append(cord("ladder-%s-top" % side, "box", math.hypot(dx, dy), ((start[0] + end[0]) / 2, (start[1] + end[1]) / 2),
                           [("box", 0.5), ("cord-%s-0" % side, 0.5)], math.atan2(-dx, dy)))
        for index in range(SLATS - 1):
            here, below = "cord-%s-%d" % (side, index), "cord-%s-%d" % (side, index + 1)
            result.append(cord("ladder-%s-%d-a" % (side, index), here, PITCH / 2, (0, -PITCH / 4), [(here, 0.75), (below, 0.25)]))
            result.append(cord("ladder-%s-%d-b" % (side, index), here, PITCH / 2, (0, -3 * PITCH / 4), [(here, 0.25), (below, 0.75)]))
    return result


def owner(solid):
    found = re.search(r"-(\d+)-[ab]$", solid["id"])
    return "slat-%s" % found.group(1) if found else ("slat-0" if solid["id"].endswith("-top") else "slat-%d" % (SLATS - 1))


DECISIONS = {
    "body": {
        "en": "Ladder cords: beside each of the two lift tapes a pair of 2.5 mm polyester cords clips into the outer and the inner bead of every lamella and runs up through the open underside of the head rail into the tilt bearing; they carry the lamellae and tilt them and end at the lowest lamella; the lift tape only carries the bottom rail, which is the guided type that does not turn. Each cord is modelled as two half segments per 80 mm bay, weighted between the bead bones of the two neighbouring lamellae, so the cords stay plumb at every tilt, the outer one sinks and the inner one rises by up to 45 mm, and the segments close up when the pack stacks.",
        "de": "Leiterkordeln: neben jedem der zwei Aufzugsbänder ist ein Paar 2,5-mm-Polyesterkordeln in den äußeren und den inneren Bördel jeder Lamelle geclipst und läuft durch die offene Unterseite der Oberschiene in das Wendelager; sie tragen und wenden die Lamellen und enden an der untersten Lamelle; das Aufzugsband trägt nur die Unterschiene, die der geführte, nicht wendende Typ ist. Jede Kordel ist je 80-mm-Feld als zwei halbe Abschnitte modelliert, gewichtet zwischen den Bördelknochen der zwei benachbarten Lamellen; so bleiben die Kordeln bei jeder Wendung lotrecht, die äußere sinkt und die innere steigt um bis zu 45 mm, und die Abschnitte rücken zusammen, wenn sich das Paket stapelt.",
    },
    "structure": {
        "en": "Structure: every lamella hangs by its two beads on the ladder cords (members of that lamella), which lead the lamella weights up into the tilt bearings of the head rail; the lift tapes remain the hanger of the bottom rail alone.",
        "de": "Tragwerk: jede Lamelle hängt mit ihren zwei Bördeln an den Leiterkordeln (Stäbe dieser Lamelle), die die Lamellengewichte in die Wendelager der Oberschiene leiten; die Aufzugsbänder bleiben allein die Aufhängung der Unterschiene.",
    },
}
ASSUMPTION_OLD_EN = "The blind hangs on two lift tapes from the head rail"
ASSUMPTION_NEW_EN = "The lamellae hang on two pairs of ladder cords from the tilt bearings and the bottom rail on two lift tapes from the head rail"
ASSUMPTION_OLD_DE = "Der Behang hängt an zwei Aufzugsbändern an der Oberschiene"
ASSUMPTION_NEW_DE = "Die Lamellen hängen an zwei Leiterkordelpaaren an den Wendelagern und die Unterschiene an zwei Aufzugsbändern an der Oberschiene"


def mine(identifier):
    return identifier.startswith(("ladder-", "member-ladder-", "cord-"))


def apply(asset):
    rig = asset["rig"]
    rig["bones"] = [bone for bone in rig["bones"] if not mine(bone["id"])] + bones()
    for pose in asset["poses"]:
        pose["bones"] = {name: value for name, value in pose["bones"].items() if not mine(name)}
        for index in range(SLATS):
            angle = pose["bones"].get("slat-%d" % index, {}).get("angleDeg", 0)
            if angle:
                for side in BEAD:
                    pose["bones"]["cord-%s-%d" % (side, index)] = {"angleDeg": -angle}
    first = asset["structure"]["assumptions"][0]
    first["en"] = first["en"].replace(ASSUMPTION_OLD_EN, ASSUMPTION_NEW_EN)
    first["de"] = first["de"].replace(ASSUMPTION_OLD_DE, ASSUMPTION_NEW_DE)
    for scale, volume in asset["volumes"].items():
        fine = scale in ("50", "100")
        solids = cords()
        volume["body"] = [solid for solid in volume["body"] if not mine(solid["id"])] + (solids if fine else [])
        volume["structure"] = [solid for solid in volume["structure"] if not mine(solid["id"])]
        for solid in solids:
            member = {"id": "member-" + solid["id"], "of": solid["id"]} if fine else dict(solid, id="member-" + solid["id"])
            member.update({"member": owner(solid), "part": "member"})
            volume["structure"].append(member)
        texts = [DECISIONS["body"], DECISIONS["structure"]] if fine else [DECISIONS["structure"]]
        volume["decisions"] = [entry for entry in volume["decisions"] if entry not in DECISIONS.values()] + texts


def member_range(text, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.start() + 1, index + 1
        index += 1


FIXING = {"100": (0.1675, 0.2), "200": (0.163, 0.2), "500": (0.176, 0.25)}
EMBED_START = -0.19
BEARING = {"50": ([0.175, 0.113, 0.02], [-0.1675, 2.3065, 0.59]), "100": ([0.175, 0.113, 0.02], [-0.1675, 2.3065, 0.59]),
           "200": ([0.19, 0.12, 0.03], [-0.175, 2.31, 0.585]), "500": ([0.19, 0.125, 0.075], [-0.175, 2.3125, 0.5625])}
LINTEL_ANCHOR = {
    "id": "lintel-bearing", "bone": "facade", "x": -0.1675, "y": 2.25, "kind": "support",
    "title": {"en": "Lintel bearings on the jamb masonry beside the strip", "de": "Sturzauflager auf dem Leibungsmauerwerk neben dem Streifen"},
    "support": "pin",
}
COARSE_DECISIONS = {
    "100": {
        "en": "Fixings at 1:100: the 8 mm guide pins and bottom rail pins (0.08 mm on paper) are left out, lamellae and bottom rail end 1 to 5 mm before the guide rails; the 12 mm tilt shaft stays as a slender member. Each bracket is tied into the clay block by one M10 injection anchor, a slender 200 mm rod through render and insulation with 110 mm embedment, so the head rail load reaches the wall; the head wall rests on the lintel bearings, which are supports of the strip.",
        "de": "Befestigungen in 1:100: die 8-mm-Führungs- und Unterschienennippel (0,08 mm auf dem Papier) entfallen, Lamellen und Unterschiene enden 1 bis 5 mm vor den Führungsschienen; die 12-mm-Wendewelle bleibt als schlanker Stab. Jeder Träger ist mit einem M10-Injektionsanker im Ziegel verankert, einem schlanken 200-mm-Stab durch Putz und Dämmung mit 110 mm Einbindetiefe, sodass die Last der Oberschiene die Wand erreicht; die Sturzwand ruht auf den Sturzauflagern, die Auflager des Streifens sind.",
    },
    "200": {
        "en": "Fixings at 1:200: guide pins and bottom rail pins are left out, the tilt shaft stays as a slender member; one slender M10 anchor of 200 mm per fixing block ties the head rail through the insulation into the masonry, and the head wall rests on the lintel bearings, which are supports of the strip.",
        "de": "Befestigungen in 1:200: Führungs- und Unterschienennippel entfallen, die Wendewelle bleibt als schlanker Stab; je Befestigungsklotz bindet ein schlanker M10-Anker von 200 mm die Oberschiene durch die Dämmung in das Mauerwerk, und die Sturzwand ruht auf den Sturzauflagern, die Auflager des Streifens sind.",
    },
    "500": {
        "en": "Fixings at 1:500: guide pins and bottom rail pins are left out; the bracket is drawn together with its anchor as one slender 250 mm rod from the masonry into the head rail, and the head wall rests on two 75 mm lintel bearing blocks, which are supports of the strip.",
        "de": "Befestigungen in 1:500: Führungs- und Unterschienennippel entfallen; der Träger ist mit seinem Anker zu einem schlanken 250-mm-Stab vom Mauerwerk in die Oberschiene zusammengezogen, und die Sturzwand ruht auf zwei 75-mm-Sturzauflagerklötzen, die Auflager des Streifens sind.",
    },
}
FINE_DECISION = {
    "en": "Load path closed: the last length of each lift tape now ends on the top face of the bottom rail (it stopped 3 mm above it), and the head wall above the opening rests on the two lintel bearings, which are pin supports of the strip beside the fixed wall foot.",
    "de": "Lastpfad geschlossen: das letzte Stück jedes Aufzugsbands endet jetzt auf der Oberseite der Unterschiene (es endete 3 mm darüber), und die Sturzwand über der Öffnung ruht auf den zwei Sturzauflagern, die neben dem eingespannten Wandfuß gelenkige Auflager des Streifens sind.",
}


MOTOR_RADIUS = 0.0225
HOLDER_SCREW = (0.004, 0.16)
HOLDER_DECISION = {
    "en": "Fixings into the wall: every guide rail holder is screwed through render and insulation into the clay block by an 8 mm frame screw of 160 mm (70 mm embedment), a member of the structure, so the wind on the guide rails reaches the masonry; the tubular motor is the 45 mm type that lies against the web of the head rail.",
    "de": "Befestigung in der Wand: jeder Führungsschienenhalter ist mit einer 8-mm-Rahmenschraube von 160 mm (70 mm Einbindetiefe) durch Putz und Dämmung in den Ziegel geschraubt, ein Stab des Tragwerks, sodass der Wind auf die Führungsschienen das Mauerwerk erreicht; der Rohrmotor ist der 45-mm-Typ, der am Steg der Oberschiene anliegt.",
}


def holders(volume):
    volume["body"] = [solid for solid in volume["body"] if not solid["id"].endswith("-screw")]
    volume["structure"] = [solid for solid in volume["structure"] if not solid["id"].endswith("-screw")]
    volume["decisions"] = [entry for entry in volume["decisions"] if entry != HOLDER_DECISION]
    for solid in volume["body"] + volume["structure"]:
        if solid["id"] in ("motor", "mass-motor") and "profile" in solid:
            solid["profile"] = [[0.0, MOTOR_RADIUS], [solid["profile"][1][0], MOTOR_RADIUS]]
    found = [solid for solid in volume["body"] if solid["id"].startswith("rail-holder")]
    for solid in found:
        radius, length = HOLDER_SCREW
        volume["body"].append({
            "id": solid["id"] + "-screw", "kind": "revolve", "bone": "box", "material": "steel", "role": "detail", "tone": 0.25,
            "profile": [[0.0, radius], [length, radius]], "axis": [1, 0, 0], "origin": [-length, solid["origin"][1], solid["origin"][2]],
            "repeat": solid["repeat"], "slender": True,
        })
        volume["structure"].append({"id": "member-" + solid["id"] + "-screw", "of": solid["id"] + "-screw", "member": "box", "part": "member"})
    if found:
        volume["decisions"].append(HOLDER_DECISION)


def pin(solid):
    return solid.get("id", "").startswith("guide-pin-") or solid.get("id") == "bottom-rail-pin"


def coarse(asset):
    asset["anchors"] = [anchor for anchor in asset["anchors"] if anchor["id"] != "lintel-bearing"] + [LINTEL_ANCHOR]
    asset["structure"]["supports"] = [name for name in asset["structure"]["supports"] if name != "lintel-bearing"] + ["lintel-bearing"]
    for scale, volume in asset["volumes"].items():
        for solid in volume["body"]:
            if solid["id"] == "lift-tape-rail":
                solid["size"] = [0.0005, 0.05, 0.008]
                solid["transform"]["translation"] = [0.0, -0.025, 0.4]
        holders(volume)
        size, centre = BEARING[scale]
        volume["structure"] = [solid for solid in volume["structure"] if solid["id"] not in ("joint-lintel-bearing", "support-lintel-bearing", "member-anchor-rod")]
        volume["body"] = [solid for solid in volume["body"] if solid["id"] != "anchor-rod"]
        volume["structure"].append({
            "id": "support-lintel-bearing", "kind": "box", "bone": "facade", "material": "masonry", "role": "ground", "tone": 0.2,
            "size": size, "transform": {"translation": centre}, "repeat": {"count": 2, "translation": [0, 0, -2 * centre[2]]},
            "member": "facade", "part": "support", "anchor": "lintel-bearing",
        })
        texts = list(COARSE_DECISIONS.values()) + [FINE_DECISION]
        volume["decisions"] = [entry for entry in volume["decisions"] if entry not in texts]
        if scale == "50":
            volume["decisions"].append(FINE_DECISION)
            continue
        for solid in volume["body"]:
            if solid["id"] == "tilt-shaft":
                solid["slender"] = True
        gone = {solid["id"] for solid in volume["body"] if pin(solid)}
        for name in ("body", "thermal", "structure"):
            volume[name] = [solid for solid in volume[name] if solid.get("id") not in gone and solid.get("of") not in gone]
        height, length = FIXING[scale]
        volume["body"].append({
            "id": "anchor-rod", "kind": "revolve", "bone": "box", "material": "steel", "role": "detail", "tone": 0.25,
            "profile": [[0.0, 0.005], [length, 0.005]], "axis": [1, 0, 0], "origin": [EMBED_START, height, 0.45],
            "repeat": {"count": 2, "translation": [0, 0, -0.9]}, "slender": True,
        })
        volume["structure"].append({"id": "member-anchor-rod", "of": "anchor-rod", "member": "box", "part": "member"})
        volume["decisions"].append(COARSE_DECISIONS[scale])


def indented(value):
    return json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n    ")


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    asset = document["asset"]
    asset["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    keys = ("rig", "anchors", "poses", "structure")
    ranges = {key: member_range(text, key) for key in keys}
    for key in keys:
        start, end = ranges[key]
        if text[start:end] != '    "%s": %s' % (key, indented(asset[key])):
            raise SystemExit("member %s is not in the expected formatting; not splicing" % key)
    apply(asset)
    coarse(asset)
    if "--write" not in sys.argv:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        print(CANDIDATE)
        return
    for key in sorted(keys, key=lambda name: -ranges[name][0]):
        start, end = ranges[key]
        text = text[:start] + '    "%s": %s' % (key, indented(asset[key])) + text[end:]
    check = json.loads(text)
    if any(check["asset"][key] != asset[key] for key in keys):
        raise SystemExit("splice did not round-trip")
    open(CATALOG, "w", encoding="utf-8").write(text)
    json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("written", CATALOG, SOURCE)


main()
