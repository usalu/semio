"""🪟 Cavity masonry wall with lintel and sill, coarse scales modelled for their own smallest feature (15 / 30 / 75 mm): bricks and joints (1:50) become course bands in front of whole leaves (1:100), one wall mass with a lintel band, sill and parapet cap (1:200) and one wall mass with lintel and sill band (1:500).

Usage: python3 rework-masonry-wall-with-lintel-coarse.py   (rewrites the three coarse scales of `volumes/components/masonry-wall-with-lintel.json`; 1:50 stays as it is)
Frame of bone `wall`: u along the wall (0 to 3 m), v up (0 to 2.75 m), w through the wall from the outer face (-0.2225) to the room face (0.2225). Bone `lintel` starts at (0.625, 2.25), bone `sill` at (0.8, 0.83333).
Layers from outside: 115 mm facing brick in 33 courses, 140 mm mineral wool, 175 mm calcium silicate blocks, 15 mm plaster. Opening 1.26 x 1.333 m; two 175 mm lintels 1.75 x 0.25 m with 80 mm wool between them; precast sill 1.4 m with a 50 mm nose.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "components", "masonry-wall-with-lintel.json")

LENGTH, HEIGHT = 3.0, 2.75
OUTER, FACING_BACK, LEAF_FRONT, LEAF_BACK, ROOM = -0.2225, -0.1075, 0.0325, 0.2075, 0.2225
LINTEL_BACK = -0.0475
NOSE = -0.2725
COURSES = 33
RISE = round(HEIGHT / COURSES, 6)
SILL_BOTTOM, SILL_TOP, HEAD, LINTEL_TOP = 0.83333, 0.91667, 2.25, 2.5
ZONES = [(0, 10, [(0.0, 3.0)]), (10, 11, [(0.0, 0.8), (2.2, 3.0)]), (11, 27, [(0.0, 0.87), (2.13, 3.0)]), (27, 30, [(0.0, 0.625), (2.375, 3.0)]), (30, 33, [(0.0, 3.0)])]
OUTLINE = [[0.0, 0.0], [LENGTH, 0.0], [LENGTH, HEIGHT], [0.0, HEIGHT]]
POCKET = [[0.625, LINTEL_TOP], [2.375, LINTEL_TOP], [2.375, HEAD], [2.13, HEAD], [2.13, SILL_TOP], [2.2, SILL_TOP], [2.2, SILL_BOTTOM], [0.8, SILL_BOTTOM], [0.8, SILL_TOP], [0.87, SILL_TOP], [0.87, HEAD], [0.625, HEAD]]
LEAF_POCKET = [[0.625, LINTEL_TOP], [2.375, LINTEL_TOP], [2.375, HEAD], [2.145, HEAD], [2.145, SILL_TOP], [0.855, SILL_TOP], [0.855, HEAD], [0.625, HEAD]]
SILL_PROFILE = [[0.0, LINTEL_BACK], [0.0, NOSE], [0.06715, NOSE], [0.08334, LINTEL_BACK]]
FEATURE = {"100": 0.015, "200": 0.03, "500": 0.075}
OUTER_FACE = ["-u", "+u", "-v", "+v", "+w"]

DECISIONS = {
    "100": [
        {"en": "Smallest feature 15 mm: the 10 mm head joints and 12 mm bed joints are left out, so the facing is 53 course bands of 83 mm (33 courses, cut by the opening, the lintel and the sill) with their bed joints in them; the calcium silicate leaf is one 175 mm leaf without block joints; the bed under the outer lintel lies in the lintel, which loses its chamfer; the sill and its raised ends are one weathered profile without drip groove; the wall ties are left out.", "de": "Kleinstes Merkmal 15 mm: Die 10-mm-Stoßfugen und 12-mm-Lagerfugen entfallen, die Verblendschale besteht aus 53 Schichtbändern von 83 mm (33 Schichten, von Öffnung, Sturz und Fensterbank unterbrochen) samt Lagerfugen; die Kalksandsteinschale ist eine 175-mm-Schale ohne Steinfugen; die Lagerfuge unter dem äußeren Sturz liegt im Sturz, der seine Fase verliert; die Fensterbank und ihre Aufkantungen sind ein Profil mit Gefälle ohne Tropfnut; die Maueranker entfallen."},
        {"en": "Cavity wool, the wool between the lintels and behind the sill, the 15 mm plaster with reveal and soffit plaster, both lintels and the sill stay separate solids; the bearing joints of the structure list are pads one feature thick.", "de": "Kerndämmung, die Dämmung zwischen den Stürzen und hinter der Fensterbank, der 15-mm-Putz mit Laibungs- und Sturzputz, beide Stürze und die Fensterbank bleiben eigene Körper; die Auflagerfugen der Tragwerksliste sind Polster von einem Merkmal Stärke."},
    ],
    "200": [
        {"en": "Smallest feature 30 mm: the four leaves are one 445 mm wall mass around the opening, as the drawing shows one poché; both lintels and the wool between them are one lintel band 1.75 x 0.25 m; the sill is a plain bar with its 50 mm nose and the parapet cap lies behind it.", "de": "Kleinstes Merkmal 30 mm: Die vier Schalen sind eine 445 mm starke Wandmasse um die Öffnung, wie die Zeichnung eine Poché zeigt; beide Stürze und die Dämmung dazwischen sind ein Sturzband 1,75 x 0,25 m; die Fensterbank ist ein glatter Stab mit 50 mm Überstand und der Brüstungsabschluss liegt dahinter."},
        {"en": "Left out: coursing, joints, plaster, the sill weathering. The structure list keeps the bearing leaf and the facing as two members with one lintel and one sill; the thermal list keeps the facing in 21 strips, the lintel shell and the sill.", "de": "Entfallen: Schichtung, Fugen, Putz, das Gefälle der Fensterbank. Die Tragwerksliste behält die tragende Schale und die Verblendschale als zwei Bauteile mit einem Sturz und einer Fensterbank; die Thermikliste behält die Verblendschale in 21 Streifen, die Sturzschale und die Fensterbank."},
    ],
    "500": [
        {"en": "Smallest feature 75 mm: one wall mass around the opening, the lintel band with its bearings and the 83 mm sill zone as one band through the wall whose nose keeps the true depth, as the drawing shows them.", "de": "Kleinstes Merkmal 75 mm: eine Wandmasse um die Öffnung, das Sturzband mit seinen Auflagern und die 83-mm-Fensterbankzone als ein Band durch die Wand, dessen Überstand die wahre Tiefe behält, wie die Zeichnung sie zeigt."},
        {"en": "Left out: all layers, coursing, joints, the parapet cap and the sill bed. The structure list carries wall, lintel and sill band as one member each.", "de": "Entfallen: alle Schichten, Schichtung, Fugen, der Brüstungsabschluss und das Mörtelbett der Fensterbank. Die Tragwerksliste führt Wand, Sturz und Fensterbankband als je ein Bauteil."},
    ],
}


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def box(identifier, bone, material, tone, size, centre, **more):
    return {"id": identifier, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": [round(value, 6) for value in size], "transform": {"translation": [round(value, 6) for value in centre]}, **more}


def slab(identifier, material, tone, front, back, holes):
    return {"id": identifier, "kind": "extrude", "bone": "wall", "material": material, "role": "body", "tone": tone, "profile": OUTLINE, "holes": [holes], "plane": "xy", "from": front, "to": back}


def courses(tone):
    bands = []
    for first, last, spans in ZONES:
        for course in range(first, last):
            for piece, (start, end) in enumerate(spans):
                bands.append(box(f"course-{course}-{piece}", "wall", "brick", tone, [end - start, RISE, FACING_BACK - OUTER], [(start + end) / 2, (course + 0.5) * RISE, (OUTER + FACING_BACK) / 2]))
    return bands


def lintel(identifier, tone, front, back, **more):
    return box(identifier, "lintel", "concrete", tone, [1.75, 0.25, back - front], [0.875, 0.125, (front + back) / 2], **more)


def sill_bar(identifier, tone, back, **more):
    return box(identifier, "sill", "concrete", tone, [1.4, SILL_TOP - SILL_BOTTOM, back - NOSE], [0.7, (SILL_TOP - SILL_BOTTOM) / 2, (NOSE + back) / 2], **more)


def pad(identifier, bone, scale, length, start, front, back):
    thickness = FEATURE[scale]
    return box(identifier, bone, "mortar", 0.7, [length, thickness, back - front], [start + length / 2, -thickness / 2, (front + back) / 2], member=bone, part="joint")


def rest(volume):
    return [solid for solid in volume["structure"] if solid["part"] in ("support", "mass")]


def facing_strips(volume):
    return [solid for solid in volume["thermal"] if solid["id"].startswith("th-facing")]


def scale_100(volume):
    body = by_id(volume["body"])
    structure = by_id(volume["structure"])
    tones = {name: body[name]["tone"] for name in ("brick-0-0", "block-0-0", "cavity-wool", "lintel-outer", "sill")}
    wool = {**body["cavity-wool"], "holes": [POCKET]}
    lintel_wool = {**body["lintel-wool"], "size": [1.75, 0.25, 0.08], "transform": {"translation": [1.5, 2.375, -0.0075]}}
    leaf = slab("inner-leaf", "block", tones["block-0-0"], LEAF_FRONT, LEAF_BACK, LEAF_POCKET)
    sill = {"id": "sill", "kind": "extrude", "bone": "sill", "material": "concrete", "role": "body", "tone": tones["sill"], "profile": SILL_PROFILE, "plane": "yz", "from": 0.0, "to": 1.4}
    volume["body"] = courses(tones["brick-0-0"]) + [wool, lintel_wool, body["sill-wool"], leaf, body["plaster"], body["reveal-plaster-left"], body["reveal-plaster-right"], body["soffit-plaster"], lintel("lintel-outer", tones["lintel-outer"], OUTER, LINTEL_BACK), body["lintel-inner"], sill]
    volume["thermal"] = facing_strips(volume) + [
        {"id": "th-lintel-shell", "of": "lintel-outer", "region": "lintel", "layer": "lintel-shell", "sectionFaces": OUTER_FACE},
        {"id": "th-sill-concrete", "of": "sill", "region": "sill", "layer": "sill-concrete", "sectionFaces": OUTER_FACE},
    ]
    volume["structure"] = [
        structure["member-wall-leaf"],
        structure["member-wall-facing"],
        structure["member-lintel-inner"],
        structure["member-lintel-outer"],
        structure["member-sill"],
        pad("joint-lintel-inner-left", "lintel", "100", 0.23, 0.0, LEAF_FRONT, LEAF_BACK),
        pad("joint-lintel-inner-right", "lintel", "100", 0.23, 1.52, LEAF_FRONT, LEAF_BACK),
        pad("joint-lintel-outer-left", "lintel", "100", 0.245, 0.0, OUTER, FACING_BACK),
        pad("joint-lintel-outer-right", "lintel", "100", 0.245, 1.505, OUTER, FACING_BACK),
        pad("joint-sill-bed", "sill", "100", 1.4, 0.0, OUTER, FACING_BACK),
        pad("joint-beam-bearing", "load-carrier", "100", 0.15, 0.0, LEAF_FRONT, LEAF_BACK),
    ] + rest(volume)


def scale_200(volume):
    body = by_id(volume["body"])
    structure = by_id(volume["structure"])
    tones = {name: body[name]["tone"] for name in ("brick-0-0", "block-0-0", "lintel-outer", "sill")}
    volume["body"] = [
        slab("wall", "brick", tones["brick-0-0"], OUTER, ROOM, POCKET),
        lintel("lintel", tones["lintel-outer"], OUTER, ROOM),
        sill_bar("sill", tones["sill"], LINTEL_BACK),
        box("parapet-cap", "sill", "block", tones["block-0-0"], [1.4, SILL_TOP - SILL_BOTTOM, ROOM - LINTEL_BACK], [0.7, (SILL_TOP - SILL_BOTTOM) / 2, (LINTEL_BACK + ROOM) / 2]),
    ]
    volume["thermal"] = facing_strips(volume) + [
        lintel("th-lintel-shell", tones["lintel-outer"], OUTER, LINTEL_BACK, region="lintel", layer="lintel-shell", sectionFaces=OUTER_FACE),
        {"id": "th-sill-concrete", "of": "sill", "region": "sill", "layer": "sill-concrete", "sectionFaces": OUTER_FACE},
    ]
    volume["structure"] = [
        structure["member-wall-leaf"],
        structure["member-wall-facing"],
        {"id": "member-lintel", "of": "lintel", "member": "lintel", "part": "member"},
        {"id": "member-sill", "of": "sill", "member": "sill", "part": "member"},
        pad("joint-lintel-left", "lintel", "200", 0.245, 0.0, OUTER, LEAF_BACK),
        pad("joint-lintel-right", "lintel", "200", 0.245, 1.505, OUTER, LEAF_BACK),
        pad("joint-sill-bed", "sill", "200", 1.4, 0.0, OUTER, FACING_BACK),
        pad("joint-beam-bearing", "load-carrier", "200", 0.15, 0.0, LEAF_FRONT, LEAF_BACK),
    ] + rest(volume)


def scale_500(volume):
    body = by_id(volume["body"])
    tones = {name: body[name]["tone"] for name in ("brick-0-0", "lintel-outer", "sill")}
    volume["body"] = [
        slab("wall", "brick", tones["brick-0-0"], OUTER, ROOM, POCKET),
        lintel("lintel", tones["lintel-outer"], OUTER, ROOM),
        sill_bar("sill-band", tones["sill"], ROOM),
    ]
    volume["thermal"] = facing_strips(volume) + [
        lintel("th-lintel-shell", tones["lintel-outer"], OUTER, LINTEL_BACK, region="lintel", layer="lintel-shell", sectionFaces=OUTER_FACE),
        sill_bar("th-sill-concrete", tones["sill"], LINTEL_BACK, region="sill", layer="sill-concrete", sectionFaces=OUTER_FACE),
    ]
    volume["structure"] = [
        {"id": "member-wall", "of": "wall", "member": "wall", "part": "member"},
        {"id": "member-lintel", "of": "lintel", "member": "lintel", "part": "member"},
        {"id": "member-sill", "of": "sill-band", "member": "sill", "part": "member"},
        pad("joint-lintel-left", "lintel", "500", 0.245, 0.0, OUTER, LEAF_BACK),
        pad("joint-lintel-right", "lintel", "500", 0.245, 1.505, OUTER, LEAF_BACK),
        pad("joint-beam-bearing", "load-carrier", "500", 0.15, 0.0, LEAF_FRONT, LEAF_BACK),
    ] + rest(volume)


def main():
    document = json.load(open(SOURCE, encoding="utf-8"))
    volumes = document["volumes"]
    if "brick-0-0" not in by_id(volumes["100"]["body"]):
        raise SystemExit("already applied: the coarse scales no longer carry the 1:50 parts list")
    for scale, build in (("100", scale_100), ("200", scale_200), ("500", scale_500)):
        build(volumes[scale])
        volumes[scale]["decisions"] = DECISIONS[scale]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    for scale in ("100", "200", "500"):
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
