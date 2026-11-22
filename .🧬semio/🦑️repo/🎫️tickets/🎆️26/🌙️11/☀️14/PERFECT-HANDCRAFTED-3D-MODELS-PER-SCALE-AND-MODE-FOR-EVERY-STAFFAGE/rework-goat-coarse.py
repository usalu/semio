"""🔭 Chisel of the goat author: the coarse scales of `animals/goat` modelled for their own smallest feature.

1:200 (30 mm): one cloven hoof per foot instead of two claws, no teats. 1:500 (75 mm): one foot per leg, one udder,
bones as members of the thickness the drawing of their limb shows, pelvis as one block, skull with the jaw.
Every number is a half extent in metres chosen for that scale.
"""
import json
import sys

SOURCE = "volumes/animals/goat.json"
FEET = ("hn", "hf", "fn", "ff")
LIMB = [0.5, 0.5, 0.5, 0.5]


def drop(listing, idents):
    listing[:] = [e for e in listing if e.get("id") not in idents and e.get("of") not in idents]


def get(listing, ident):
    return next((e for e in listing if e.get("id") == ident), None)


def floor(entry, sagittal, lateral=None):
    lateral = sagittal if lateral is None else lateral
    if entry is None or "of" in entry:
        return
    if entry["kind"] == "loft":
        inner = entry["sections"][1:-1] if len(entry["sections"]) > 3 else entry["sections"]
        for section in inner:
            section["rx"] = max(section["rx"], sagittal)
            section["ry"] = max(section["ry"], lateral)
    elif entry["kind"] == "capsule":
        entry["r0"] = max(entry["r0"], sagittal)
        entry["r1"] = max(entry["r1"], sagittal)
    elif entry["kind"] == "ellipsoid":
        entry["radii"] = [max(r, sagittal) for r in entry["radii"]]


def centre(entry):
    for section in entry["sections"]:
        section["at"][2] = 0


def hoof(body, foot, sagittal, lateral, stations=None):
    claw = get(body, "claw-%s-o" % foot)
    centre(claw)
    for section in claw["sections"]:
        section["rx"] = max(section["rx"], sagittal)
        section["ry"] = lateral
    if stations:
        claw["sections"] = [dict(claw["sections"][0], at=[u, v, 0]) for u, v in stations]
    return ["claw-%s-i" % foot]


def scale200(volume):
    body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
    gone = ["teat-near", "teat-far"]
    for foot in FEET:
        gone += hoof(body, foot, 0.016, 0.0165)
    for listing in (body, thermal, structure):
        drop(listing, gone)
    lamina = get(structure, "median-suspensory-lamina")
    for section in lamina["sections"]:
        section["rx"] = max(section["rx"], 0.0155)
        section["ry"] = max(section["ry"], 0.0155)


def scale500(volume):
    body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
    gone = ["teat-near", "teat-far", "udder-far", "mandible", "caudal-1", "caudal-2", "hip-bone-n", "hip-bone-f"]
    for foot in FEET:
        gone += hoof(body, foot, 0.028, 0.024, [(0.092, -0.026), (0.078, -0.018), (0.06, -0.011), (0.042, -0.006)]) + ["coffin-%s-o" % foot, "coffin-%s-i" % foot, "phalanx-%s-i" % foot]
        pastern = get(body, "pastern-" + foot)
        centre(pastern)
        floor(pastern, 0.024)
        digit = get(structure, "phalanx-%s-o" % foot)
        digit["p0"][2] = digit["p1"][2] = 0
        digit["p1"][0] = 0.07
        floor(digit, 0.0238)
    for listing in (body, thermal, structure):
        drop(listing, gone)
    udder = get(body, "udder-near")
    centre(udder)
    for section in udder["sections"][1:-1]:
        section["rx"] = min(max(section["rx"], 0.045), 0.058)
        section["ry"] = max(section["ry"], 0.04)
    for side in ("n", "f"):
        floor(get(body, "horn-" + side), 0.0138)
        floor(get(body, "ear-" + side), 0.0115)
        floor(get(body, "arm-" + side), 0.038)
        floor(get(body, "forearm-" + side), 0.0305)
        floor(get(body, "shoulder-" + side), 0.033)
        floor(get(structure, "scapula-" + side), 0.0328)
        floor(get(structure, "humerus-" + side), 0.0378)
        floor(get(structure, "radius-ulna-" + side), 0.0302)
        floor(get(structure, "metacarpal-" + side), 0.017)
        floor(get(structure, "femur-" + side), 0.0378)
        floor(get(structure, "tibia-" + side), 0.0378)
        floor(get(structure, "metatarsal-" + side), 0.0148)
    floor(get(structure, "cervical"), 0.0378)
    floor(get(structure, "thoracic"), 0.0378)
    floor(get(structure, "lumbar-column"), 0.0378)
    floor(get(structure, "median-suspensory-lamina"), 0.0378)
    sacrum = get(structure, "sacrum")
    for section in sacrum["sections"]:
        section["rx"] = max(section["rx"], 0.072)
        section["ry"] = max(section["ry"], 0.062)
    structure.insert(1, {"id": "mandible", "of": "head", "member": "jaw", "part": "member"})
    structure.insert(2, {"id": "caudal-1", "of": "tail", "member": "tail-1", "part": "member"})
    structure.insert(3, {"id": "caudal-2", "of": "tail", "member": "tail-2", "part": "member"})


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    volumes = document["volumes"]
    lamina = get(volumes["100"]["structure"], "median-suspensory-lamina")
    for section in lamina["sections"]:
        section["rx"] = max(section["rx"], 0.008)
    scale200(volumes["200"])
    scale500(volumes["500"])
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] coarse scales written", file=sys.stderr)


main()
