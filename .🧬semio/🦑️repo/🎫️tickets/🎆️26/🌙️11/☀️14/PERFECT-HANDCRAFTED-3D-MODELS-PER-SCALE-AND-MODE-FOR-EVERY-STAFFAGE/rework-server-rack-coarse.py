"""🗄️ Server rack, second rework round: the 1:100, 1:200 and 1:500 lists modelled for their own feature size.

Hand-chosen numbers only. Frame of every solid: u = up along the bone, v = lateral, w = depth (door at w = -0.5).
Writes the volumes source; with `--catalog` it also redraws the 2D shapes that follow the changed solids.
See rework/assets/appliances--server-rack.md.
"""
import json
import math
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
SOURCE = T / "volumes/appliances/server-rack.json"
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/server-rack/🔣️.json")

STEEL, MESH, CHASSIS, RUBBER = "rack-steel", "mesh-steel", "chassis", "caster-rubber"
REAR_A = {"translation": [-0.1, -0.2735, -0.4935]}
REAR_B = {"translation": [-0.1, 0.2735, -0.4935]}
PIN = {"translation": [-0.1, -0.2735, 0.4935]}
PAIR = {"count": 2, "translation": [0.0, -0.5, 0.0]}
THREE = {"count": 3, "translation": [0.72, 0.0, 0.0]}
FANS = {"count": 2, "translation": [0.0, 0.0, 0.22]}
DEPTH = {"count": 2, "translation": [0.0, 0.0, 0.91]}


def r(value):
    return round(value, 5)


def rect(u0, v0, u1, v1):
    return [[r(u0), r(v1)], [r(u0), r(v0)], [r(u1), r(v0)], [r(u1), r(v1)]]


def grid(u0, u1, v0, v1, rows, cols, web_u, web_v):
    pitch_u, pitch_v = (u1 - u0) / rows, (v1 - v0) / cols
    holes = []
    for row in range(rows):
        a = u0 + row * pitch_u + web_u / 2
        for col in range(cols):
            b = v0 + col * pitch_v + web_v / 2
            holes.append(rect(a, b, a + pitch_u - web_u, b + pitch_v - web_v))
    return holes


def octagon(cw, radius):
    return [[r(radius * math.sin(k * math.pi / 4)), r(cw + radius * math.cos(k * math.pi / 4))] for k in range(8)]


def box(id, bone, material, tone, size, at, role="body", **more):
    return {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": at}, **more}


def extrude(id, bone, material, tone, plane, profile, start, end, role="body", **more):
    return {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "plane": plane, "from": start, "to": end, **more}


def cylinder(id, bone, material, tone, origin, axis, length, radius, role="body", **more):
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[0.0, radius], [length, radius]], "axis": axis, "origin": origin, **more}


DOORS = {
    "50": {
        "door": dict(field=(0.0015, 1.8655, -0.2685, 0.2685), rows=30, cols=8, web_u=0.0106, web_v=0.01),
        "rear": dict(field=(0.1015, 1.9655, 0.0025, 0.2685), rows=30, cols=4, web_u=0.0103, web_v=0.01),
    },
    "100": {
        "door": dict(field=(0.0015, 1.8655, -0.2685, 0.2685), rows=12, cols=5, web_u=0.025, web_v=0.0175),
        "rear": dict(field=(0.17917, 1.88783, 0.0025, 0.2685), rows=11, cols=3, web_u=0.025, web_v=0.015),
    },
    "200": {
        "door": dict(field=(0.03, 1.837, -0.24, 0.24), rows=6, cols=3, web_u=0.045, web_v=0.032),
        "rear": dict(field=(0.13, 1.937, 0.031, 0.24), rows=5, cols=2, web_u=0.05, web_v=0.0325),
    },
    "500": {
        "door": dict(field=(0.0015, 1.8655, -0.2685, 0.2685), rows=6, cols=1, web_u=0.08, web_v=0.15),
        "rear": dict(field=(0.1015, 1.9655, 0.0025, 0.2685), rows=6, cols=1, web_u=0.08, web_v=0.15),
    },
}


def door_holes(scale, leaf):
    spec = DOORS[scale]["door" if leaf == "door" else "rear"]
    u0, u1, v0, v1 = spec["field"]
    if leaf == "rear-door-b":
        v0, v1 = -v1, -v0
    return grid(u0, u1, v0, v1, spec["rows"], spec["cols"], spec["web_u"], spec["web_v"])


def door_fields(scale):
    return [
        extrude("door-field", "door", MESH, 0.18, "xy", rect(0.0015, -0.2685, 1.8655, 0.2685), -0.5, -0.4985, holes=door_holes(scale, "door")),
        extrude("rear-door-a", "rear-door-a", MESH, 0.18, "xy", rect(0.1015, 0.0025, 1.9655, 0.2685), 0.4985, 0.5, holes=door_holes(scale, "rear-door-a"), transform=REAR_A),
        extrude("rear-door-b", "rear-door-b", MESH, 0.18, "xy", rect(0.1015, -0.2685, 1.9655, -0.0025), 0.4985, 0.5, holes=door_holes(scale, "rear-door-b"), transform=REAR_B),
    ]


def hinges(radius, length):
    start = 0.3 - length / 2
    return [
        cylinder("door-hinge-pin", "door-hinge", STEEL, 0.3, [start, 0.2735, -0.4935], [1, 0, 0], length, radius, role="detail", repeat=THREE, transform=PIN),
        cylinder("rear-door-a-hinge", "rear-door-a", MESH, 0.3, [start, 0.2735, 0.4935], [1, 0, 0], length, radius, role="detail", repeat=THREE, transform=REAR_A),
        cylinder("rear-door-b-hinge", "rear-door-b", MESH, 0.3, [start, -0.2735, 0.4935], [1, 0, 0], length, radius, role="detail", repeat=THREE, transform=REAR_B),
    ]


def supports(old):
    return [solid for solid in old if solid.get("part") == "support"]


def keep(old, *ids):
    by_id = {solid["id"]: solid for solid in old}
    return [json.loads(json.dumps(by_id[id])) for id in ids]


def ref(id, of, **more):
    return {"id": id, "of": of, **more}


def strips_to_rail(body):
    for solid in body:
        if solid["id"] == "pdu":
            solid["transform"]["translation"][2] = 0.385


def scale_50(volume):
    strips_to_rail(volume["body"])
    for solid in volume["body"]:
        if solid["id"] in ("door-field", "rear-door-a", "rear-door-b"):
            solid["holes"] = door_holes("50", "door" if solid["id"] == "door-field" else solid["id"])
        if solid["id"] == "fan-hub":
            solid["profile"] = [[0.0, 0.03], [0.036, 0.03], [0.039, 0.024]]
        if solid["id"] == "fan-housing":
            solid["to"] = 1.9955
        if solid["id"] == "cable-brush":
            solid["size"] = [0.006, 0.28, 0.02]
    return volume["body"], volume["thermal"], volume["structure"]


def scale_100(volume):
    gone = ("socket-", "caster-", "door-hinge-pin", "rear-door-a-hinge", "rear-door-b-hinge")
    body = [solid for solid in volume["body"] if not solid["id"].startswith(gone) and solid["id"] not in ("door-field", "rear-door-a", "rear-door-b")]
    strips_to_rail(body)
    for solid in body:
        if solid["id"] == "fan-housing":
            solid["to"] = 1.9988
    body += door_fields("100") + hinges(0.008, 0.1)
    body += [
        cylinder("fan-hub", "frame", CHASSIS, 0.15, [1.9565, 0.0, 0.1], [1, 0, 0], 0.04, 0.03, repeat=FANS),
        box("fan-spoke-0", "frame", CHASSIS, 0.15, [0.015, 0.165, 0.015], [1.99, 0.0, 0.1], repeat=FANS),
        box("fan-spoke-1", "frame", CHASSIS, 0.15, [0.015, 0.015, 0.165], [1.99, 0.0, 0.1], repeat=FANS),
        cylinder("caster-head-f", "feet", STEEL, 0.3, [0.044, 0.25, -0.425], [1, 0, 0], 0.026, 0.027, repeat=PAIR),
        cylinder("caster-wheel-f", "feet", RUBBER, 0.2, [0.025, 0.265, -0.45], [0, -1, 0], 0.03, 0.025, repeat=PAIR),
        cylinder("caster-head-r", "feet", STEEL, 0.3, [0.044, 0.25, 0.425], [1, 0, 0], 0.026, 0.027, repeat=PAIR),
        cylinder("caster-wheel-r", "feet", RUBBER, 0.2, [0.025, 0.265, 0.45], [0, -1, 0], 0.03, 0.025, repeat=PAIR),
    ]
    ids = {solid["id"] for solid in body}
    thermal = [solid for solid in volume["thermal"] if ("of" not in solid or solid["of"] in ids) and not solid["id"].startswith("th-caster-")]
    thermal += [ref(f"th-caster-{part}-{end}", f"caster-{part}-{end}", region="feet") for end in "fr" for part in ("head", "wheel")]
    dropped = ("member-caster-", "joint-rear-door-", "member-hinge-pin")
    structure = [solid for solid in volume["structure"] if ("of" not in solid or solid["of"] in ids) and not solid["id"].startswith(dropped)]
    structure += [ref(f"member-caster-{part}-{end}", f"caster-{part}-{end}", member="feet", part="member") for end in "fr" for part in ("head", "wheel")]
    structure += [
        ref("member-hinge-pin", "door-hinge-pin", member="door-hinge", part="member"),
        ref("joint-rear-door-a", "rear-door-a-hinge", member="rear-door-a", part="joint"),
        ref("joint-rear-door-b", "rear-door-b-hinge", member="rear-door-b", part="joint"),
    ]
    return body, thermal, structure


def caster_side(sign):
    points = [[-0.385, 0.07], [-0.465, 0.07], [-0.465, 0.042], [-0.482, 0.03], [-0.482, 0.012], [-0.468, 0.0], [-0.44, 0.0], [-0.425, 0.012], [-0.415, 0.04], [-0.385, 0.045]]
    return points if sign < 0 else [[-w, u] for w, u in reversed(points)]


def scale_200(volume):
    old_body, old_thermal, old_structure = volume["body"], volume["thermal"], volume["structure"]
    body = keep(old_body, "base-frame", "side-a", "side-b", "server-block", "top-group", "pdu")
    strips_to_rail(body)
    body += [
        extrude("top-frame", "frame", STEEL, 0.45, "yz", [[0.28, -0.475], [-0.28, -0.475], [-0.28, 0.475], [0.28, 0.475]], 1.917, 1.967, holes=[[[0.23, 0.435], [-0.23, 0.435], [-0.23, -0.435], [0.23, -0.435]]]),
        box("post-a", "frame", STEEL, 0.45, [1.797, 0.04, 0.04], [1.0185, 0.26, -0.455], repeat=DEPTH),
        box("post-b", "frame", STEEL, 0.45, [1.797, 0.04, 0.04], [1.0185, -0.26, -0.455], repeat=DEPTH),
        extrude("roof", "frame", STEEL, 0.5, "yz", [[0.3, -0.475], [-0.3, -0.475], [-0.3, 0.475], [0.3, 0.475]], 1.967, 2.0, holes=[octagon(0.1, 0.08), octagon(0.32, 0.08)]),
        box("fan-spoke-0", "frame", CHASSIS, 0.15, [0.03, 0.17, 0.03], [1.982, 0.0, 0.1], repeat=FANS),
        box("fan-spoke-1", "frame", CHASSIS, 0.15, [0.03, 0.03, 0.17], [1.982, 0.0, 0.1], repeat=FANS),
        extrude("door-return", "door", MESH, 0.25, "xy", rect(0.0, -0.27, 1.867, 0.27), -0.505, -0.475, holes=[rect(0.03, -0.24, 1.837, 0.24)]),
        extrude("rear-door-a-return", "rear-door-a", MESH, 0.25, "xy", rect(0.1, 0.001, 1.967, 0.27), 0.475, 0.505, holes=[rect(0.13, 0.031, 1.937, 0.24)], transform=REAR_A),
        extrude("rear-door-b-return", "rear-door-b", MESH, 0.25, "xy", rect(0.1, -0.27, 1.967, -0.001), 0.475, 0.505, holes=[rect(0.13, -0.24, 1.937, -0.031)], transform=REAR_B),
        box("door-handle", "door", CHASSIS, 0.12, [0.16, 0.03, 0.03], [0.92, -0.255, -0.5]),
        box("rail-front-a", "equipment", STEEL, 0.55, [1.8669, 0.03, 0.03], [0.93345, 0.2355, -0.389]),
        box("rail-front-b", "equipment", STEEL, 0.55, [1.8669, 0.03, 0.03], [0.93345, -0.2355, -0.389]),
        box("rail-rear-a", "equipment", STEEL, 0.55, [1.8669, 0.03, 0.03], [0.93345, 0.2355, 0.349]),
        box("rail-rear-b", "equipment", STEEL, 0.55, [1.8669, 0.03, 0.03], [0.93345, -0.2355, 0.349]),
        extrude("caster-f", "feet", RUBBER, 0.25, "zx", caster_side(-1), 0.23, 0.27, repeat=PAIR),
        extrude("caster-r", "feet", RUBBER, 0.25, "zx", caster_side(1), 0.23, 0.27, repeat=PAIR),
    ]
    body += door_fields("200") + hinges(0.015, 0.12)
    thermal = keep(old_thermal, "th-side-a", "th-side-b", "th-base", "th-servers", "th-blanks", "th-switch", "th-exhaust-air")
    for solid in thermal:
        if solid["id"] == "th-servers":
            solid["transform"]["translation"][2] = -0.431
    thermal += [
        ref("th-roof", "roof", region="cabinet", layer="cabinet-sheet", sectionFaces=["-u"]),
        ref("th-door-field", "door-field", region="door", layer="door-sheet"),
        ref("th-door-return", "door-return", region="door", layer="door-sheet"),
        ref("th-rear-door-a", "rear-door-a", region="rear-doors", layer="rear-sheet"),
        ref("th-rear-door-a-return", "rear-door-a-return", region="rear-doors", layer="rear-sheet"),
        ref("th-rear-door-b", "rear-door-b", region="rear-doors", layer="rear-sheet"),
        ref("th-rear-door-b-return", "rear-door-b-return", region="rear-doors", layer="rear-sheet"),
        ref("th-caster-f", "caster-f", region="feet"),
        ref("th-caster-r", "caster-r", region="feet"),
        ref("th-pdu", "pdu", region="power-strips", layer="strip-housing"),
    ]
    structure = [ref(f"member-{id}", id, member="frame", part="member") for id in ("post-a", "post-b", "base-frame", "top-frame")]
    structure += [ref(f"member-{id}", id, member="equipment", part="member") for id in ("rail-front-a", "rail-front-b", "rail-rear-a", "rail-rear-b")]
    structure += [
        ref("member-caster-f", "caster-f", member="feet", part="member"),
        ref("member-caster-r", "caster-r", member="feet", part="member"),
        box("joint-corner-a-low", "frame", STEEL, 0.6, [0.06, 0.06, 0.06], [0.095, 0.26, -0.455], role="detail", repeat=DEPTH, member="frame", part="joint"),
        box("joint-corner-a-high", "frame", STEEL, 0.6, [0.06, 0.06, 0.06], [1.942, 0.26, -0.455], role="detail", repeat=DEPTH, member="frame", part="joint"),
        box("joint-corner-b-low", "frame", STEEL, 0.6, [0.06, 0.06, 0.06], [0.095, -0.26, -0.455], role="detail", repeat=DEPTH, member="frame", part="joint"),
        box("joint-corner-b-high", "frame", STEEL, 0.6, [0.06, 0.06, 0.06], [1.942, -0.26, -0.455], role="detail", repeat=DEPTH, member="frame", part="joint"),
        ref("member-door", "door-return", member="door", part="member"),
        ref("member-rear-door-a", "rear-door-a-return", member="rear-door-a", part="member"),
        ref("member-rear-door-b", "rear-door-b-return", member="rear-door-b", part="member"),
        ref("member-hinge-pin", "door-hinge-pin", member="door-hinge", part="member"),
        ref("joint-rear-door-a", "rear-door-a-hinge", member="rear-door-a", part="joint"),
        ref("joint-rear-door-b", "rear-door-b-hinge", member="rear-door-b", part="joint"),
        box("mass-server-block", "equipment", CHASSIS, 0.15, [0.56, 0.3, 0.5], [0.3556, 0.0, -0.065], role="detail", member="equipment", part="mass"),
        box("mass-top-group", "equipment", CHASSIS, 0.15, [0.1, 0.3, 0.3], [1.80023, 0.0, -0.215], role="detail", member="equipment", part="mass"),
        box("mass-frame", "frame", STEEL, 0.15, [0.12, 0.12, 0.12], [1.0, 0.0, 0.0], role="detail", member="frame", part="mass"),
        ref("mass-front-leaf", "door-field", member="door", part="mass"),
        ref("mass-rear-a", "rear-door-a", member="rear-door-a", part="mass"),
        ref("mass-rear-b", "rear-door-b", member="rear-door-b", part="mass"),
    ]
    structure += supports(old_structure)
    return body, thermal, structure


def scale_500(volume):
    old_thermal, old_structure = volume["thermal"], volume["structure"]
    depth = [[-0.4985, 0.07], [-0.4985, 2.0], [0.4985, 2.0], [0.4985, 0.07]]
    plan = [[0.3, -0.4985], [-0.3, -0.4985], [-0.3, 0.4985], [0.3, 0.4985]]
    body = [
        extrude("side-a", "frame", STEEL, 0.5, "zx", depth, 0.2988, 0.3),
        extrude("side-b", "frame", STEEL, 0.5, "zx", depth, -0.3, -0.2988),
        extrude("roof", "frame", STEEL, 0.5, "yz", plan, 1.9988, 2.0, holes=[octagon(0.1, 0.08), octagon(0.32, 0.08)]),
        box("base-frame", "frame", STEEL, 0.4, [0.075, 0.5976, 0.997], [0.1075, 0.0, 0.0]),
        box("server-block", "equipment", CHASSIS, 0.42, [0.7108, 0.4826, 0.72], [0.3554, 0.0, -0.048]),
        box("top-group", "equipment", CHASSIS, 0.4, [0.13295, 0.4826, 0.43], [1.80043, 0.0, -0.193]),
        box("rail-front-a", "equipment", STEEL, 0.55, [1.8669, 0.075, 0.075], [0.93345, 0.26125, -0.389]),
        box("rail-front-b", "equipment", STEEL, 0.55, [1.8669, 0.075, 0.075], [0.93345, -0.26125, -0.389]),
        box("pdu", "frame", CHASSIS, 0.2, [1.65, 0.075, 0.075], [1.025, 0.26, 0.395], repeat={"count": 2, "translation": [0.0, -0.52, 0.0]}),
        box("caster-f", "feet", RUBBER, 0.25, [0.075, 0.075, 0.1], [0.0375, 0.25, -0.425], repeat=PAIR),
        box("caster-r", "feet", RUBBER, 0.25, [0.075, 0.075, 0.1], [0.0375, 0.25, 0.425], repeat=PAIR),
        cylinder("door-hinge-pin", "door-hinge", STEEL, 0.3, [0.1, 0.2735, -0.4935], [1, 0, 0], 1.867, 0.006, role="detail", slender=True, transform=PIN),
    ]
    body += door_fields("500")
    thermal = keep(old_thermal, "th-side-a", "th-side-b", "th-base", "th-servers", "th-blanks", "th-switch", "th-exhaust-air")
    thermal += [
        ref("th-roof", "roof", region="cabinet", layer="cabinet-sheet", sectionFaces=["-u"]),
        ref("th-door-field", "door-field", region="door", layer="door-sheet"),
        ref("th-rear-door-a", "rear-door-a", region="rear-doors", layer="rear-sheet"),
        ref("th-rear-door-b", "rear-door-b", region="rear-doors", layer="rear-sheet"),
        ref("th-caster-f", "caster-f", region="feet"),
        ref("th-caster-r", "caster-r", region="feet"),
        ref("th-pdu", "pdu", region="power-strips", layer="strip-housing"),
    ]
    structure = keep(old_structure, "member-base-frame", "member-top-frame", "member-post-a", "member-post-b", "mass-server-block", "mass-top-group")
    structure += [
        ref("member-rail-front-a", "rail-front-a", member="equipment", part="member"),
        ref("member-rail-front-b", "rail-front-b", member="equipment", part="member"),
        ref("member-caster-f", "caster-f", member="feet", part="member"),
        ref("member-caster-r", "caster-r", member="feet", part="member"),
        ref("member-door", "door-field", member="door", part="member"),
        ref("member-rear-door-a", "rear-door-a", member="rear-door-a", part="member"),
        ref("member-rear-door-b", "rear-door-b", member="rear-door-b", part="member"),
        ref("member-hinge-pin", "door-hinge-pin", member="door-hinge", part="member"),
        box("mass-frame", "frame", STEEL, 0.15, [0.15, 0.15, 0.15], [1.0, 0.0, 0.0], role="detail", member="frame", part="mass"),
    ]
    structure += supports(old_structure)
    return body, thermal, structure


DECISIONS = {
    "100": {
        "en": "Modelled for a 15 mm feature: the welded frame of posts and rings, side trays, the roof with two fan frames, hub and guard cross, the units as front plate and chassis. The 5 mm perforation of the doors is generalised to 90 by 130 mm apertures with 17 to 25 mm webs at the same 70 % free area (front 5 by 12, each rear leaf 3 by 12 in the same rows, so the empty units stay as transparent as at 1:50). One 16 mm barrel stands for each hinge, and a caster is its swivel head and a 30 mm wide wheel.",
        "de": "Für ein kleinstes Merkmal von 15 mm modelliert: der geschweißte Rahmen aus Pfosten und Ringen, Seitenwannen, das Dach mit zwei Lüfterrahmen, Nabe und Schutzkreuz, die Geräte als Frontplatte und Gehäuse. Die 5-mm-Lochung der Türen ist zu Öffnungen von 90 mal 130 mm mit Stegen von 17 bis 25 mm bei gleichen 70 % freiem Querschnitt verallgemeinert (vorn 5 mal 12, je hinterem Flügel 3 mal 12 in denselben Reihen, damit die leeren Einheiten so durchsichtig bleiben wie bei 1:50). Ein Zylinder von 16 mm steht für jedes Scharnier, eine Rolle besteht aus Drehkopf und 30 mm breitem Rad.",
    },
    "200": {
        "en": "Modelled for a 30 mm feature: 40 mm posts between 50 mm base and top rings, sheet sides, a 33 mm roof slab with two fan openings under a guard cross, the servers as one block and the top group as a second. Each door is a 30 mm frame around a sheet with a few large apertures (front 3 by 6, rear leaves 2 by 5), hinges are 30 mm barrels, the handle one 30 mm bar, the rails 30 mm bars and each caster one block cut to the side view of head and wheel.",
        "de": "Für ein kleinstes Merkmal von 30 mm modelliert: 40-mm-Pfosten zwischen Sockel- und Kopfring von 50 mm, Blechseiten, eine 33 mm starke Dachplatte mit zwei Lüfteröffnungen unter einem Schutzkreuz, die Server als ein Block und die Obergruppe als zweiter. Jede Tür ist ein 30-mm-Rahmen um ein Blech mit wenigen großen Öffnungen (vorn 3 mal 6, hintere Flügel 2 mal 5), Scharniere sind 30-mm-Zylinder, der Griff ein 30-mm-Stab, die Schienen 30-mm-Stäbe und jede Rolle ein Block im Seitenriss von Kopf und Rad.",
    },
    "500": {
        "en": "Modelled for a 75 mm feature: a shell of two side sheets, a roof sheet with the two 160 mm fan openings and a 75 mm plinth, the server block and the top group on two 75 mm mounting bars, the power strips as 75 mm bars and four 75 mm caster blocks. Each door leaf is one sheet with six slots between 80 mm bars, which reads as a vented door at 4 mm drawing height; door frames, hinges, handle and posts are below the feature and left out, the hinge line of the front door stays as a slender pin.",
        "de": "Für ein kleinstes Merkmal von 75 mm modelliert: eine Hülle aus zwei Seitenblechen, einem Dachblech mit den beiden Lüfteröffnungen von 160 mm und einem 75-mm-Sockel, Serverblock und Obergruppe an zwei Montagestäben von 75 mm, die Steckdosenleisten als 75-mm-Stäbe und vier Rollenblöcke von 75 mm. Jeder Türflügel ist ein Blech mit sechs Schlitzen zwischen 80-mm-Stegen und liest sich bei 4 mm Zeichenhöhe als belüftete Tür; Türrahmen, Scharniere, Griff und Pfosten liegen unter dem Merkmal und entfallen, die Scharnierlinie der Fronttür bleibt als schlanker Bolzen.",
    },
}


def path(outer, holes):
    def ring(points, reverse):
        points = [points[3], points[2], points[1], points[0]] if reverse else points
        return "M " + " L ".join(f"{u:.6f} {v:.6f}" for u, v in points) + " Z"

    return " ".join([ring(outer, False)] + [ring(hole, True) for hole in holes])


def shifted(points, du, dv):
    return [[u + du, v + dv] for u, v in points]


def redraw(asset, volumes):
    offsets = {"door": (0.0, 0.0), "rear-door-a": (-0.1, -0.2735), "rear-door-b": (-0.1, 0.2735)}
    targets = {"door-field": "door-field", "rear-door-a-field": "rear-door-a", "rear-door-b-field": "rear-door-b", "door-frame": "door-return", "rear-door-a-frame": "rear-door-a-return", "rear-door-b-frame": "rear-door-b-return"}
    for scale in ("50", "100", "200", "500"):
        body = {solid["id"]: solid for solid in volumes[scale]["body"]}
        geometry = asset["representations"][scale]["geometry"]
        for view, shapes in geometry.items():
            kept = []
            for shape in shapes:
                if shape["id"] in targets and shape.get("kind") == "path" and "solid" in shape:
                    solid = body.get(targets[shape["id"]])
                    if solid is None:
                        continue
                    if scale in ("200", "500") or not shape["id"].endswith("-frame"):
                        du, dv = offsets[shape["bone"]]
                        shape["d"] = path(shifted(solid["profile"], du, dv), [shifted(hole, du, dv) for hole in solid["holes"]])
                        shape["solid"]["z0"], shape["solid"]["z1"] = solid["from"], solid["to"]
                if scale == "500" and shape["id"] == "portal-frame":
                    continue
                kept.append(shape)
            geometry[view] = kept
        by_id = {view: {shape["id"]: shape for shape in shapes} for view, shapes in geometry.items()}
        elevation, front = by_id["elevation"], by_id["front"]
        if scale == "200":
            elevation["rail-a"].update(y=0.2205, h=0.03)
            elevation["rail-b"].update(y=-0.2505, h=0.03)
            elevation["base-rail"].update(x=0.07, w=0.05)
        if scale == "500" and "casters" in front:
            elevation["base-rail"].update(x=0.07, w=0.075, y=-0.2988, h=0.5976)
            elevation["caster-left"].update(x=0.0, w=0.075, y=0.2125, h=0.075)
            elevation["caster-right"].update(x=0.0, w=0.075, y=-0.2875, h=0.075)
            elevation["pdu-a"].update(y=0.2225, h=0.075)
            elevation["pdu-b"].update(y=-0.2975, h=0.075)
            elevation["rail-a"].update(y=0.22375, h=0.075)
            elevation["rail-b"].update(y=-0.29875, h=0.075)
            front["cabinet"].update(x=-0.4985, y=0.07, w=0.997, h=1.93)
            band = front["casters"]
            band.update(id="caster-front", x=-0.475, y=0.0, w=0.1, h=0.075)
            rear = json.loads(json.dumps(band))
            rear.update(id="caster-rear", x=0.375)
            geometry["front"].insert(geometry["front"].index(band) + 1, rear)
            by_id["plan"]["roof"].update(x=-0.3, y=-0.4985, w=0.6, h=0.997)


WORDING = {
    "on a band of casters": "on four caster blocks",
    "on its caster band": "on two caster blocks",
    "auf einem Rollenband": "auf vier Rollenblöcken",
    "auf seinem Rollenband": "auf zwei Rollenblöcken",
}


def reword(asset):
    text = json.dumps(asset["representations"]["500"]["decisions"], ensure_ascii=False)
    for old, new in WORDING.items():
        text = text.replace(old, new)
    asset["representations"]["500"]["decisions"] = json.loads(text)


def main():
    source = json.loads(SOURCE.read_text())
    volumes = source["volumes"]
    before = json.loads((T / "rework-server-rack-base.json").read_text())["volumes"]
    for scale, build in (("50", scale_50), ("100", scale_100), ("200", scale_200), ("500", scale_500)):
        if scale != "50":
            volumes[scale] = before[scale]
        body, thermal, structure = build(volumes[scale])
        volumes[scale].update(body=body, thermal=thermal, structure=structure)
        if scale in DECISIONS:
            volumes[scale]["decisions"] = [DECISIONS[scale]]
    SOURCE.write_text(json.dumps(source, ensure_ascii=False, indent=1) + "\n")
    if "--catalog" in sys.argv:
        tool = {}
        exec((T / "volumes-insert.py").read_text().rsplit("main()", 1)[0], tool)
        text = CATALOG.read_text()
        document = json.loads(text)
        redraw(document["asset"], volumes)
        reword(document["asset"])
        indent = "    "
        start, end = tool["member_range"](text, indent, "representations")
        result = text[:start] + indent + '"representations": ' + tool["dump"](document["asset"]["representations"], 2) + text[end:]
        assert json.loads(result) == document
        assert CATALOG.read_text() == text
        CATALOG.write_text(result)
    for scale in volumes:
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


main()
