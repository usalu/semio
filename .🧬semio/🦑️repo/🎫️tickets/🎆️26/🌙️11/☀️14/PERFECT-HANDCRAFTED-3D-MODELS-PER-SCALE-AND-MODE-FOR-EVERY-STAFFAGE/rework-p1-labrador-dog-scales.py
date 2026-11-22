"""🔍 Models the head, paws and skeleton of the labrador for the feature of each scale: no eyelids below 1:50, eyes, nose and tongue as thermal surfaces only from 1:200, the jaw merged into the head at 1:500, the skeleton at 1:200 and 1:500 as bars of at least the pen width, and the ear cartilage rooted on the skull.

Usage: python3 rework-p1-labrador-dog-scales.py
"""
import copy
import json

SOURCE = "volumes/animals/labrador-dog.json"
STRAIGHT = [0.5, 0.5, 0.5, 0.5]
PEN = 0.0376
HEAD_500 = [
    (-0.030, 0.0105, 0.0265, 0.034, 2.25), (0.023, 0.014, 0.051, 0.066, 2.48), (0.072, 0.007, 0.064, 0.069, 2.5),
    (0.084, 0.006, 0.062, 0.065, 2.5), (0.094, 0.003, 0.058, 0.060, 2.45), (0.104, -0.003, 0.051, 0.054, 2.4),
    (0.115, -0.007, 0.043, 0.048, 2.45), (0.126, -0.0095, 0.0405, 0.044, 2.5), (0.174, -0.0105, 0.0405, 0.044, 2.6),
    (0.216, -0.010, 0.035, 0.039, 2.5), (0.229, -0.007, 0.028, 0.033, 2.4), (0.237, -0.004, 0.016, 0.023, 2.3),
]
LUMBAR_200 = [(-0.002, 0.012, 0.017, 0.034), (0.060, 0.0135, 0.0235, 0.047), (0.130, 0.0135, 0.0235, 0.040), (0.197, 0.013, 0.022, 0.030)]


def loft(identifier, bone, rows, extra):
    sections = [{"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": exponent, "rotation": STRAIGHT} for x, y, rx, ry, exponent in rows]
    return {"id": identifier, "kind": "loft", "bone": bone, "material": extra.pop("material"), "role": "body", "tone": extra.pop("tone"), "sections": sections, "capStart": True, "capEnd": True, **extra}


def drop(entries, names):
    return [entry for entry in entries if entry["id"] not in names and entry.get("of") not in names]


def own(scale, names):
    body = {entry["id"]: entry for entry in scale["body"]}
    for index, entry in enumerate(scale["thermal"]):
        if entry.get("of") in names:
            solid = copy.deepcopy(body[entry["of"]])
            solid.update({key: value for key, value in entry.items() if key != "of"})
            solid["role"] = "detail"
            scale["thermal"][index] = solid


def sides(names):
    return [name + side for name in names for side in ("-n", "-f")]


def find(entries, identifier):
    return next(entry for entry in entries if entry["id"] == identifier)


def ear_roots(structure, radius):
    skull = find(structure, "skull")
    for side, sign in (("n", -1), ("f", 1)):
        structure.append({"id": "ear-root-" + side, "kind": "capsule", "bone": "head", "material": skull["material"], "role": "body", "tone": skull["tone"], "p0": [0.04, 0.034, sign * 0.036], "p1": [0.04, 0.04, sign * 0.06], "r0": radius, "r1": radius, "member": "head", "part": "member"})


def main():
    document = json.load(open(SOURCE))
    volumes = document["volumes"]
    lids = sides(["lid-upper", "lid-lower"])
    marks = lids + sides(["pupil", "naris"])

    fine = volumes["50"]
    ear_roots(fine["structure"], 0.006)
    for side in ("fn", "ff"):
        marker = find(fine["structure"], "joint-scapula-" + side)
        marker["r0"] = marker["r1"] = 0.0085

    hundred = volumes["100"]
    hundred["body"] = drop(hundred["body"], lids)
    hundred["thermal"] = drop(hundred["thermal"], lids)
    for section in find(hundred["body"], "tongue")["sections"]:
        section["rx"] = round(section["rx"] * 0.0076 / 0.0045, 5)
    for identifier in sides(["zygomatic"]):
        for section in find(hundred["structure"], identifier)["sections"]:
            section["rx"] = max(round(section["rx"] * 1.5, 5), 0.0056)
            section["ry"] = max(round(section["ry"] * 1.5, 5), 0.0056)
    ear_roots(hundred["structure"], 0.008)

    two = volumes["200"]
    own(two, sides(["eye"]) + ["tongue"])
    two["thermal"] = drop(two["thermal"], marks)
    two["body"] = drop(two["body"], marks + sides(["eye"]) + ["tongue"])
    nose = find(two["body"], "nose-leather")
    nose["vertices"] = [[round(0.2387 + (x - 0.2387) * 1.35, 5), round(0.0025 + (y - 0.0025) * 1.35, 5), z] for x, y, z in nose["vertices"]]
    for side, sign in (("n", -1), ("f", 1)):
        flew = find(two["body"], "flew-" + side)
        flew["vertices"] = [[x, round(-0.0144 + (y + 0.0144) * 1.05, 5), round(sign * 0.0247 + (z - sign * 0.0247) * 1.05, 5)] for x, y, z in flew["vertices"]]
    structure = drop(two["structure"], sides(["zygomatic"]))
    mandible = find(structure, "mandible")
    for section, rx in zip(mandible["sections"], (0.0155, 0.016, 0.0155, 0.0155, 0.012, 0.006)):
        section["rx"] = rx
    first = find(structure, "lumbar-l1")
    spine = loft("lumbar-spine", "lumbar", [(x, y, rx, ry, 2.5) for x, y, rx, ry in LUMBAR_200], {"material": first["material"], "tone": first["tone"], "member": "lumbar", "part": "member"})
    position = structure.index(first)
    structure = drop(structure, ["lumbar-l%d" % number for number in range(1, 8)])
    structure.insert(position, spine)
    find(structure, "caudal-1")["p0"] = [-0.022, 0.004, 0]
    two["structure"] = structure

    five = volumes["500"]
    own(five, sides(["eye"]) + ["tongue", "nose-leather"])
    gone = marks + sides(["eye", "flew"]) + ["tongue", "nose-leather", "jaw"]
    five["thermal"] = drop(five["thermal"], marks + sides(["flew"]) + ["jaw"])
    five["body"] = drop(five["body"], gone)
    head = find(five["body"], "head")
    head["sections"] = loft("head", "head", HEAD_500, {"material": head["material"], "tone": head["tone"]})["sections"]
    for identifier in ("paw-hn", "paw-hf", "paw-fn", "paw-ff"):
        for section in find(five["body"], identifier)["sections"]:
            section["ry"] = round(section["ry"] * 1.17, 5)
    for identifier in sides(["fore-foot"]):
        for section in find(five["body"], identifier)["sections"]:
            section["rx"] = round(section["rx"] * 1.12, 5)
    for identifier in sides(["ear"]):
        for section, ry in zip(find(five["body"], identifier)["sections"], (0.0075, 0.0097, 0.0097, 0.0075)):
            section["ry"] = ry
    structure = drop(five["structure"], sides(["zygomatic"]) + ["rib-8", "mandible"])
    for identifier in ("cervical", "thoracic", "scapula-n", "scapula-f"):
        for section in find(structure, identifier)["sections"]:
            section["rx"] = section["ry"] = PEN
    first = find(structure, "lumbar-l1")
    spine = loft("lumbar-spine", "lumbar", [(-0.002, 0.0135, PEN, PEN, 2), (0.197, 0.0135, PEN, PEN, 2)], {"material": first["material"], "tone": first["tone"], "member": "lumbar", "part": "member"})
    position = structure.index(first)
    structure = drop(structure, ["lumbar-l%d" % number for number in range(1, 8)])
    structure.insert(position, spine)
    for identifier, radius in (("caudal-1", PEN), ("caudal-2", 0.0305), ("caudal-3", PEN), ("phalanges-fn", 0.0231), ("phalanges-ff", 0.0231), ("phalanges-hn", 0.0229), ("phalanges-hf", 0.0229)):
        bar = find(structure, identifier)
        bar["r0"] = bar["r1"] = radius
    five["structure"] = structure
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False)


main()
