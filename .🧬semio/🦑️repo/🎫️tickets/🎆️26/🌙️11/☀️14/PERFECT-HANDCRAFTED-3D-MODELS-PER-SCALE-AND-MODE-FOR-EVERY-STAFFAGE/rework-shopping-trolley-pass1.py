"""🛒 Shopping trolley, second round pass 1: hand edits of the volumes source (1:100 castor crown merged into the swivel block, 1:200 tray lips on posts, 1:500 true upright with handle arm, basket fixings and a closed basket cage in the structure).

Usage: python3 rework-shopping-trolley-pass1.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "objects", "shopping-trolley.json")
BASE = os.path.join(T, "🗑️generated", "rework-shopping-trolley", "source-before-pass1.json")
SIDES = (("l", 1.0), ("r", -1.0))


def wire(id, bone, p0, p1, r, material="zinc-wire", role="detail", tone=0.55):
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone, "p0": p0, "p1": p1, "r0": r, "r1": r, "slender": True}


def drop(items, ids):
    return [x for x in items if x.get("id") not in ids and x.get("of") not in ids]


def put(items, new):
    ids = {x["id"] for x in new}
    return [x for x in items if x["id"] not in ids] + new


def s100(v):
    crowns = {"castor-crown-%s-%s" % (e, s) for e in ("rear", "front") for s in "lr"}
    for name in ("body", "thermal", "structure"):
        v[name] = drop(v[name], crowns)
    v["decisions"][1] = {
        "en": "Chassis tubes o25 with one bend at the rail end and one at the handle; castor head, plate and the 8 mm fork crown are one 21 mm swivel block under the rail, both fork legs hang from it on the swivelling castor bone, tyre and hub are simplified profiles, the axle is dropped; the grip is a capsule.",
        "de": "Fahrgestellrohre o25 mit je einem Bogen am Holmende und am Griff; Rollenkopf, Platte und die 8 mm dicke Gabelbrücke sind ein Schwenkblock von 21 mm unter dem Holm, beide Gabelschenkel hängen daran am schwenkenden Rollenknochen, Reifen und Nabe sind vereinfachte Profile, die Achse entfällt; der Griff ist eine Kapsel.",
    }


def s200(v):
    panels = []
    for end, x, z in (("front", 0.545, 0.19492), ("rear", 0.065, 0.25444)):
        panels.append({"id": "tray-lip-panel-" + end, "kind": "box", "bone": "chassis", "material": "zinc-wire", "role": "glazing", "tone": 0.6, "size": [0.001, 0.04, 2 * z], "transform": {"translation": [x, 0.0355, 0.0]}})
    v["body"] = put(v["body"], panels)
    v["decisions"][0] = {
        "en": "1:200 (feature 30 mm): the basket is its translucent mesh planes outlined by the o8 rim, the o6 floor frame and corner wires and two hoops; pickets and floor wires are no longer drawn. The gate is its panel in its frame, the crate tray a plane in its frame with two 40 mm lips, each a mesh panel under its top wire (the o7.5 corner posts are below the feature).",
        "de": "1:200 (kleinstes Detail 30 mm): der Korb besteht aus seinen durchscheinenden Gitterebenen, umrissen vom Randdraht o8, Bodenrahmen und Eckdrähten o6 und zwei Bügeln; Stäbe und Bodendrähte sind nicht mehr gezeichnet. Die Klappe ist ihre Tafel im Rahmen, die Kistenablage eine Ebene im Rahmen mit zwei 40 mm hohen Kanten, jede eine Gittertafel unter ihrem oberen Draht (die Eckpfosten o7,5 liegen unter dem Detailmaß).",
    }


def s500(v, joints200):
    body = {x["id"]: x for x in v["body"]}
    new_body, new_thermal, new_structure = [], [], []
    for side, sign in SIDES:
        z = sign * 0.2725
        body["castor-housing-rear-" + side]["size"] = [0.075, 0.075, 0.075]
        body["castor-housing-front-" + side]["size"] = [0.075, 0.075, 0.075]
        rim = wire("bracket-rim-" + side, "chassis", [-0.2, 0.7075, z], [-0.005, 0.7075, sign * 0.2524], 0.004, "zinc-wire", "detail", 0.5)
        floor = wire("bracket-floor-" + side, "chassis", [-0.1218, 0.3725, z], [0.035, 0.3765, sign * 0.2061], 0.004, "zinc-wire", "detail", 0.5)
        new_body += [rim, floor]
        new_thermal += [{"id": "th-" + x["id"], "of": x["id"], "region": "frame-tube"} for x in (rim, floor)]
        new_structure += [
            {"id": "st-bracket-rim-" + side, "of": rim["id"], "member": "chassis", "part": "member"},
            {"id": "st-bracket-floor-" + side, "of": floor["id"], "member": "chassis", "part": "member"},
        ]
        cage = [
            wire("st-floor-edge-" + side, "basket", [0.0, 0.0, sign * 0.21], [0.62, 0.025, sign * 0.186], 0.004),
            wire("st-corner-rear-" + side, "basket", [0.0, 0.0, sign * 0.21], [-0.035, 0.335, sign * 0.255], 0.004),
            wire("st-corner-front-" + side, "basket", [0.62, 0.025, sign * 0.186], [0.88, 0.335, sign * 0.215], 0.004),
        ]
        for member in cage:
            member.update({"member": "basket", "part": "member"})
        new_structure += cage
    v["body"] = put(v["body"], new_body)
    v["thermal"] = put(v["thermal"], new_thermal)
    v["structure"] = put(v["structure"], new_structure)
    v["decisions"] = [
        {
            "en": "1:500 (feature 75 mm): the basket is its four translucent mesh planes and the gate panel inside the o8 rim. Two straight o25 rails run from the front cross tube to the rear bend, the uprights run straight to the handle bar (the 80 mm handle arms are merged into them); two o19 front struts and four o8 fixing wires at rim and floor carry the basket on the frame, so nothing floats.",
            "de": "1:500 (kleinstes Detail 75 mm): der Korb besteht aus seinen vier durchscheinenden Gitterebenen und der Klappentafel im Randdraht o8. Zwei gerade Holme o25 laufen vom vorderen Querrohr zum hinteren Bogen, die Stützen laufen gerade bis zur Griffstange (die 80 mm langen Griffarme sind darin aufgegangen); zwei vordere Stützen o19 und vier Befestigungsdrähte o8 an Rand und Boden tragen den Korb am Rahmen, nichts schwebt.",
        },
        {
            "en": "Each castor is one o125 x 75 wheel on its axle bone under one 75 mm fork housing on the swivelling kingpin bone and the 21 mm swivel block on the rail; fork legs, crown, axle, hub ribs, tray, coin lock, caps and every mesh wire are below 75 mm and dropped.",
            "de": "Jede Rolle ist ein Rad o125 x 75 an seinem Achsknochen unter einem Gabelgehäuse von 75 mm am schwenkenden Königszapfenknochen und dem Schwenkblock von 21 mm am Holm; Gabelschenkel, Brücke, Achse, Nabenrippen, Ablage, Pfandschloss, Kappen und alle Gitterdrähte liegen unter 75 mm und entfallen.",
        },
        {
            "en": "Thermal: tubes, struts, fixings, rim, gate frame, castors and wheels by reference, the grip as its own skin over the handle bar, and thermal-only wire bundles that keep the mesh surface of walls, floor and gate. Structure: the load path is closed from the goods to the ground: floor edges, corner wires and rim form the basket cage, it hangs on the uprights at rim and floor and stands on the struts, rails and cross tube carry it to the swivel blocks, housings and wheels, with pads at the four wheel contacts and the masses at their centres.",
            "de": "Thermik: Rohre, Stützen, Befestigungen, Rand, Klappenrahmen, Rollen und Räder als Verweise, der Griff als eigene Haut über der Griffstange, dazu rein thermische Drahtbündel, welche die Gitteroberfläche von Wänden, Boden und Klappe erhalten. Tragwerk: der Lastpfad ist von der Ware bis zum Boden geschlossen: Bodenkanten, Eckdrähte und Rand bilden den Korbkäfig, er hängt an Rand und Boden an den Stützen und steht auf den vorderen Stützen, Holme und Querrohr tragen ihn zu Schwenkblöcken, Gehäusen und Rädern, mit Kontaktflächen an den vier Radaufstandspunkten und den Massen in ihren Schwerpunkten.",
        },
    ]


def main():
    document = json.load(open(BASE if os.path.exists(BASE) else SOURCE, encoding="utf-8"))
    volumes = document["volumes"]
    s100(volumes["100"])
    s200(volumes["200"])
    s500(volumes["500"], [x for x in volumes["200"]["structure"] if x.get("part") == "joint"])
    json.dump(document, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    print("written", SOURCE)


main()
