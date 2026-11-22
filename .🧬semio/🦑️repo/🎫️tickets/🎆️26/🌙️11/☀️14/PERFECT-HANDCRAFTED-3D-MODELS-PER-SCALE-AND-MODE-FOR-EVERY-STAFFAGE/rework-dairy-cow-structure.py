"""🦴 Dairy cow skeleton chisel: carries femur, humerus and scapula members to the corrected bone lengths at every scale and brings the 1:500 spine and tail members to the printable feature.

Usage: python3 rework-dairy-cow-structure.py <asset document> (in place)
"""
import copy
import json
import math
import sys

LENGTHS = {"thigh-h": 0.42 / 0.43132, "humerus-f": 0.32 / 0.36, "scapula-f": 0.44 / 0.46}
FEATURE_500 = 0.0377


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    volumes = document["asset"]["volumes"]
    for scale in volumes.values():
        for entry in scale["structure"]:
            factor = LENGTHS.get(entry.get("bone", "")[:-1])
            if factor is None or entry.get("part") != "member":
                continue
            for section in entry.get("sections", []):
                section["at"][0] = round(section["at"][0] * factor, 5)
            for key in ("p0", "p1"):
                if key in entry:
                    entry[key][0] = round(entry[key][0] * factor, 5)
            if "transform" in entry and "translation" in entry["transform"]:
                entry["transform"]["translation"][0] = round(entry["transform"]["translation"][0] * factor, 5)
    for entry in volumes["500"]["structure"]:
        if entry["id"] in ("spine-thoracic", "spine-lumbar"):
            for section in entry["sections"]:
                section["rx"], section["ry"] = max(section["rx"], 0.038), max(section["ry"], 0.04)
        if entry["id"] in ("caudal-1", "caudal-2", "caudal-3"):
            limit = {"caudal-1": FEATURE_500, "caudal-2": 0.0302, "caudal-3": 0.0352}[entry["id"]]
            for section in entry["sections"]:
                section["rx"], section["ry"] = limit, limit
    bones = {bone["id"]: bone for bone in document["asset"]["rig"]["bones"]}
    pelvis_turn = math.radians(bones["pelvis"]["angleDeg"])
    udder = bones["udder"]
    pubis = (-0.03, -0.04)
    dx, dy = pubis[0] - udder["x"], pubis[1] - udder["y"]
    back = math.radians(-udder["angleDeg"])
    apex = [round(dx * math.cos(back) - dy * math.sin(back), 4), round(dx * math.sin(back) + dy * math.cos(back), 4)]
    for scale, listing in ((key, value["structure"]) for key, value in volumes.items()):
        for entry in listing:
            ident = entry["id"]
            if ident.endswith("-ear-cartilage"):
                first = copy.deepcopy(entry["sections"][0])
                first["at"][2] = round(first["at"][2] * 0.062 / 0.085, 4)
                entry["sections"].insert(0, first)
            if ident in ("near-scapula", "far-scapula") and scale in ("50", "100"):
                first = copy.deepcopy(entry["sections"][0])
                sign = -1 if ident.startswith("near") else 1
                first["at"] = [0.0, 0.0, sign * 0.062]
                first["rx"] = round(first["rx"] * 0.75, 4)
                entry["sections"].insert(0, first)
            if ident == "atlas":
                entry["radii"][0] = 0.058
            if ident == "joint-pelvis" and scale == "50":
                entry["radii"] = [0.036, 0.036, 0.036]
    for scale in ("50", "100"):
        listing = volumes[scale]["structure"]
        skull = next(entry for entry in listing if entry["id"] == "skull")
        style = {key: value for key, value in skull.items() if key not in ("id", "kind", "sections", "capStart", "capEnd")}
        listing.insert(listing.index(skull) + 1, {"id": "occipital-condyles", "kind": "ellipsoid", **style, "radii": [0.04, 0.045, 0.05], "transform": {"translation": [0.005, -0.06, 0]}})
    for scale in ("200", "500"):
        for entry in volumes[scale]["structure"]:
            if entry["id"] == "cervical-column":
                last = copy.deepcopy(entry["sections"][-1])
                last["at"] = [0.675, -0.004, 0]
                entry["sections"].append(last)
    coarse = volumes["500"]["structure"]
    template = next(entry for entry in coarse if entry["id"] == "near-hip-bone")
    style = {key: value for key, value in template.items() if key not in ("id", "kind", "sections", "capStart", "capEnd")}
    fresh = []
    for entry in coarse:
        if entry["id"] in ("near-hip-bone", "far-hip-bone"):
            sign = -1 if entry["id"].startswith("near") else 1
            side = entry["id"][: entry["id"].index("-")]
            fresh.append({"id": entry["id"], "kind": "capsule", **style, "p0": [0.308, -0.082, sign * 0.225], "p1": [0.0, 0.0, sign * 0.15], "r0": FEATURE_500, "r1": FEATURE_500})
            fresh.append({"id": side + "-ischium", "kind": "capsule", **style, "p0": [0.0, 0.0, sign * 0.15], "p1": [-0.159, 0.227, sign * 0.135], "r0": FEATURE_500, "r1": FEATURE_500})
            continue
        if entry["id"] == "udder-ligament":
            points = entry["profile"] + [apex]
            cx, cy = sum(point[0] for point in points) / len(points), sum(point[1] for point in points) / len(points)
            entry["profile"] = sorted(points, key=lambda point: math.atan2(point[1] - cy, point[0] - cx))
            print("[DEBUG] udder ligament apex", apex)
        fresh.append(entry)
        if entry["id"] == "sacrum":
            fresh.append({"id": "ilium-wings", "kind": "capsule", **style, "p0": [0.262, -0.03, -0.215], "p1": [0.262, -0.03, 0.215], "r0": FEATURE_500, "r1": FEATURE_500})
            fresh.append({"id": "pubis", "kind": "capsule", **style, "p0": [pubis[0], pubis[1], -0.15], "p1": [pubis[0], pubis[1], 0.15], "r0": FEATURE_500, "r1": FEATURE_500})
    coarse[:] = fresh
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


main()
