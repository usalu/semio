"""🐑 Sheep at 1:200 and 1:500: the two claws of a foot become one horn block, and the 1:500 skeleton becomes true slender shafts (no half-scale 1:50 sections); hand-chosen numbers, written to the volumes source.

Usage: python3 rework-sheep-coarse.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
ALONG = [0.0, -0.70711, 0.0, 0.70711]
SLOPE = {"hind": 0.7719 / 0.6357, "fore": 0.7944 / 0.6074}
CLAW_LENGTH = {"hind": 0.0401, "fore": 0.039}
PASTERN_LENGTH = {"hind": 0.07, "fore": 0.068}
HORN_STATIONS = [(-0.006, 0.018, 0.021), (0.012, 0.020, 0.023), (0.028, 0.017, 0.021), (0.038, 0.011, 0.016)]


def scope():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    return names["dump"]


def section(x, y, z, rx, ry, exponent=2):
    return {"at": [round(x, 5), round(y, 5), round(z, 5)], "rx": rx, "ry": ry, "exponent": exponent, "rotation": ALONG}


def horn(side, limb, bone, shift):
    """One horn block for both claws: its sole lies on the ground plane through the toe tip, its back rises with the dorsal wall."""
    sections = [section(x + shift, vertical - SLOPE[limb] * (CLAW_LENGTH[limb] - x), 0, lateral, vertical, 3) for x, vertical, lateral in HORN_STATIONS]
    return {"id": f"{side}-{limb}-claws", "kind": "loft", "bone": bone, "material": "hoof-horn", "role": "body", "tone": 0.86, "sections": sections, "capStart": True, "capEnd": True}


def merge_claws(scale, on_pastern):
    body, thermal = scale["body"], scale["thermal"]
    for side, limb in ((s, l) for s in ("near", "far") for l in ("hind", "fore")):
        code = ("h" if limb == "hind" else "f") + side[0]
        bone, shift = ("pastern-" + code, PASTERN_LENGTH[limb]) if on_pastern else ("hoof-" + code, 0.0)
        first = next(i for i, solid in enumerate(body) if solid["id"] == f"{side}-{limb}-claw-1")
        body[first] = horn(side, limb, bone, shift)
        body[:] = [solid for solid in body if solid["id"] != f"{side}-{limb}-claw-2"]
        for entry in thermal:
            if entry.get("of") == f"{side}-{limb}-claw-1":
                entry["id"], entry["of"] = f"th-{side}-{limb}-claws", f"{side}-{limb}-claws"
        thermal[:] = [entry for entry in thermal if entry.get("of") != f"{side}-{limb}-claw-2"]


def shaft(solid, stations):
    template = solid["sections"][0]
    solid["sections"] = [dict(template, at=[x, y, z], rx=rx, ry=ry) for x, y, z, rx, ry in stations]
    solid["slender"] = True


def skeleton_500(structure):
    by_id = {solid["id"]: solid for solid in structure}
    for name in ("lumbar-spine", "thoracic-spine", "cervical-spine"):
        by_id[name]["slender"] = True
    for side, sign in (("near", -1), ("far", 1)):
        for rib in ("rib-2", "rib-7", "rib-11"):
            by_id[f"{side}-{rib}"]["slender"] = True
        for name in ("hind-thigh", "hind-shank", "hind-metatarsus", "fore-forearm"):
            by_id[f"{side}-{name}"]["slender"] = True
        shaft(by_id[f"{side}-fore-scapula"], [(-0.012, 0, 0, 0.012, 0.018), (0.093, 0, 0, 0.010, 0.015), (0.185, 0, 0, 0.012, 0.016)])
        shaft(by_id[f"{side}-fore-humerus"], [(0, 0, 0, 0.015, 0.015), (0.06, 0, 0, 0.011, 0.0115), (0.152, 0, 0, 0.015, 0.015)])
        shaft(by_id[f"{side}-fore-metacarpus"], [(-0.014, 0, 0, 0.015, 0.013), (0.03, 0, 0, 0.0105, 0.008), (0.14, 0, 0, 0.0135, 0.0105)])
        ilium = by_id[f"{side}-ilium"]
        for part in ilium["sections"]:
            part["rx"], part["ry"] = 0.013, 0.015
        ilium["slender"] = True
        for part in by_id[f"{side}-ear-cartilage"]["sections"]:
            part["ry"] = 0.01
        for limb in ("hind", "fore"):
            code = ("h" if limb == "hind" else "f") + side[0]
            digits = by_id[f"{side}-{limb}-digit-1"]
            digits["id"] = f"{side}-{limb}-digits"
            digits["sections"] = [section(0, 0, 0, 0.0165, 0.0165), section(0.078, -0.013, 0, 0.0165, 0.0165)]
            structure[:] = [solid for solid in structure if solid["id"] != f"{side}-{limb}-digit-2"]
    sacrum = by_id["sacrum"]
    for part, radius in zip(sacrum["sections"], (0.017, 0.015, 0.012, 0.008)):
        part["rx"], part["ry"] = radius, min(radius, 0.014)
    sacrum["slender"] = True
    shaft(by_id["mandible"], [(0.005, -0.026, 0, 0.018, 0.018), (0.027, -0.046, 0, 0.0185, 0.0185), (0.112, -0.018, 0, 0.013, 0.012), (0.186, 0.01, 0, 0.01, 0.008)])
    shaft(by_id["caudal-1"], [(0, 0, 0, 0.0065, 0.006), (0.1, 0, 0, 0.005, 0.0048), (0.1526, 0.0161, 0, 0.0045, 0.0045)])
    shaft(by_id["caudal-2"], [(-0.0622, 0.019, 0, 0.006, 0.0055), (0, 0, 0, 0.005, 0.0048), (0.088, 0, 0, 0.0042, 0.0042)])


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    volumes = source["volumes"]
    if any(solid["id"] == "near-hind-claws" for solid in volumes["200"]["body"]):
        raise SystemExit("already applied")
    merge_claws(volumes["200"], False)
    merge_claws(volumes["500"], True)
    for solid in volumes["500"]["body"]:
        if solid["id"].endswith("-ear"):
            for part in solid["sections"]:
                part["ry"] = max(part["ry"], 0.01)
        if solid["id"].endswith("-hind-lower"):
            for part in solid["sections"]:
                part["rx"] = max(part["rx"], 0.0195)
    skeleton_500(volumes["500"]["structure"])
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + scope()(volumes, 0) + "}\n")


main()
