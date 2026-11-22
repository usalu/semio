"""🦴 Author pass 1 (second round), animals/horse: the skeleton joined into one load path and cut for the feature of every scale.

Usage: python3 rework-p1-horse-structure.py
Coordinates are bone frames (u along the bone, v across in the sagittal plane, z lateral; near side z < 0).
Cervical spine: C7 to C5 sag towards the throat, C4 to C1 rise towards the crest (the S of the equine neck); tuber sacrale beside the midline on the sacrum,
tuber coxae 0.39 m apart, pubis under the hip sockets, the dock continues the sacrum, the ear cartilage stands on the skull.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "animals", "horse.json")
FLOOR = {"50": 0.0, "100": 0.0, "200": 0.0, "500": 0.0378}
CERVICAL = {
    "c7": ("neck-1", (0.0, 0.0), (0.15, -0.03), 0.036, 1.4),
    "c6": ("neck-1", (0.15, -0.03), (0.312, -0.04), 0.035, 1.4),
    "c5": ("neck-1", (0.312, -0.04), (0.482, 0.0), 0.033, 1.35),
    "c4": ("neck-2", (0.0, 0.0), (0.14, 0.012), 0.032, 1.3),
    "c3": ("neck-2", (0.14, 0.012), (0.28, 0.0), 0.031, 1.3),
    "c2-axis": ("neck-3", (0.0, 0.0), (0.155, 0.012), 0.03, 1.2),
    "c1-atlas": ("neck-3", (0.155, 0.012), (0.222, 0.0), 0.034, 1.9),
}


def find(listing, ident):
    for entry in listing:
        if entry.get("id") == ident:
            return entry
    return None


def thicken(entry, floor):
    if entry.get("part") != "member" or floor <= 0:
        return
    if entry["kind"] == "capsule":
        entry["r0"] = max(entry["r0"], floor)
        entry["r1"] = max(entry["r1"], floor)
    elif entry["kind"] == "loft":
        for section in entry["sections"]:
            section["rx"] = max(section["rx"], floor)
            section["ry"] = max(section["ry"], floor)


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, volume in document["volumes"].items():
        structure = volume["structure"]
        for ident, (bone, start, end, radius, lateral) in CERVICAL.items():
            entry = find(structure, ident)
            if entry is not None:
                entry.update({"bone": bone, "p0": [start[0], start[1], 0.0], "p1": [end[0], end[1], 0.0], "r0": radius, "r1": radius, "lateralScale": lateral, "member": bone})
        for side, sign in (("near", -1), ("far", 1)):
            ear = find(structure, side + "-ear-cartilage")
            if ear is not None:
                ear["p0"] = [-0.078, 0.0, sign * 0.05]
                if scale == "500":
                    ear["p1"] = [0.1, 0.0, sign * 0.06]
            crest = find(structure, side + "-iliac-crest")
            if crest is not None:
                crest["p0"] = [0.236, 0.03, sign * 0.03]
            ischium = find(structure, side + "-ischium")
            if ischium is not None:
                ischium["p1"][2] = sign * 0.098
                ischium["r1"] = 0.03
            scapula = find(structure, side + "-scapula")
            if scale == "500":
                for section, z in zip(scapula["sections"], (0.095, 0.158, 0.168)):
                    section["at"][2] = sign * z
            orbit = find(structure, side + "-orbit")
            if orbit is not None:
                orbit["transform"]["translation"][2] = sign * 0.071
        sacrum = find(structure, "sacrum")
        sacrum["sections"][-1]["at"] = [-0.106, 0.272, 0.0]
        find(structure, "caudal-1")["p0"] = [-0.025, 0.0, 0.0]
        arch = find(structure, "ischial-arch")
        if arch is not None:
            arch["p0"][2], arch["p1"][2] = -0.09, 0.09
        for entry in structure:
            if entry["id"].startswith("vertebra-t") and entry["kind"] == "ellipsoid":
                entry["radii"][0] = 0.0265
        pubis = find(structure, "pubis")
        if pubis is None and find(structure, "j-pelvis") is not None:
            pubis = dict(find(structure, "caudal-1"))
            pubis.update({"id": "pubis", "bone": "pelvis", "member": "pelvis", "p0": [-0.024, -0.028, -0.12], "p1": [-0.024, -0.028, 0.12], "r0": 0.018, "r1": 0.018})
            structure.insert(structure.index(find(structure, "sacrum")) + 1, pubis)
        if pubis is not None:
            pubis["p0"][:2] = [-0.024, -0.028]
            pubis["p1"][:2] = [-0.024, -0.028]
        if scale == "500" and find(structure, "ilium-wings") is None:
            hip = find(structure, "near-hip-bone")
            wings = dict(hip)
            wings.update({"id": "ilium-wings", "p0": [0.2, 0.0, -0.15], "p1": [0.2, 0.0, 0.15]})
            structure.insert(structure.index(hip) + 1, wings)
        for entry in structure:
            thicken(entry, FLOOR[scale])
        if scale == "500":
            for entry in volume["body"]:
                if entry["id"].endswith("-hoof"):
                    first, last = entry["sections"][1]["at"], entry["sections"][-1]["at"]
                    length = sum((a - b) ** 2 for a, b in zip(first, last)) ** 0.5
                    if length < 0.06:
                        entry["sections"][-1]["at"] = [round(b + (b - a) / length * 0.006, 5) for a, b in zip(first, last)]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] written", SOURCE)


main()
