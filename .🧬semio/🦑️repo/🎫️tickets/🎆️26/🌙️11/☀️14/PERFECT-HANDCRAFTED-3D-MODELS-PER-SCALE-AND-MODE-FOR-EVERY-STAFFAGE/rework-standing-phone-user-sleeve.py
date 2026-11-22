"""🧥 Upper sleeves of `humans/standing-phone-user` taken in to the arm of a slim woman under a sweatshirt (girth 0.26 m plus fleece and ease), at every scale, in the body solids and in the three 2D drawings.

Usage: python3 rework-standing-phone-user-sleeve.py [--apply]
Without `--apply` the candidate document is written to `🗑️generated/rework-standing-phone-user/🔣️.json`.
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "standing-phone-user", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "standing-phone-user.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-phone-user", "🔣️.json")
SCALES = ("50", "100", "200", "500")

TAPER = ((0.0, 0.93), (0.10, 0.87), (0.25, 0.87), (0.3135, 0.90))
SHOULDER_Y = 1.39751
SHOULDER_Z = 0.153
SLEEVE_OFFSET = 0.016
TEE_CLEARANCE = 0.004
BREADTH = 0.465


def lateral():
    spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def taper(x):
    if x <= TAPER[0][0]:
        return TAPER[0][1]
    for (x0, f0), (x1, f1) in zip(TAPER, TAPER[1:]):
        if x <= x1:
            return f0 + (f1 - f0) * (x - x0) / (x1 - x0)
    return TAPER[-1][1]


def centre(x, sign):
    return sign * (SHOULDER_Z + SLEEVE_OFFSET * max(0.0, 1 - max(x, 0.0) / 0.3))


def solids(volumes, log):
    for scale in SCALES:
        for side in "lr":
            sleeve = next(solid for solid in volumes[scale]["body"] if solid["id"] == "sleeve-" + side)
            for section in sleeve["sections"]:
                f = taper(section["at"][0])
                section["rx"], section["ry"] = round(section["rx"] * f, 5), round(section["ry"] * f, 5)
            for solid in volumes[scale]["thermal"]:
                if solid.get("id") == "th-tee-sleeve-" + side and solid.get("kind") == "loft":
                    for section in solid["sections"]:
                        f = taper(section["at"][0])
                        limit = 0.0586 * f - TEE_CLEARANCE
                        before = (section["rx"], section["ry"])
                        section["rx"], section["ry"] = round(min(section["rx"], limit), 5), round(min(section["ry"], limit * 1.1), 5)
                        if before != (section["rx"], section["ry"]):
                            log.append(f"{scale} {solid['id']} at {section['at'][0]}: {before} -> {(section['rx'], section['ry'])}")


def points_of(shape):
    if isinstance(shape.get("points"), list):
        return shape["points"]
    return None


def drawings(asset, log):
    for scale, representation in asset["representations"].items():
        geometry = representation.get("geometry", {})
        for view in ("elevation", "front", "plan"):
            for shape in geometry.get(view, []):
                bone = shape.get("bone")
                if bone not in ("arm-l", "arm-r"):
                    continue
                sign = 1 if bone == "arm-l" else -1
                points = points_of(shape)
                if points is not None and shape.get("solid", {}).get("kind") == "revolved":
                    for point in points:
                        point[1] = round(point[1] * taper(point[0]), 4)
                    log.append(f"{scale} {view} {shape['id']} profile of {len(points)} points redrawn")
                    continue
                if points is None or "solid" in shape:
                    log.append(f"{scale} {view} {shape['id']} kind {shape.get('kind')} solid {'solid' in shape}: not redrawn")
                    continue
                for point in points:
                    if view == "elevation":
                        point[1] = round(point[1] * taper(point[0]), 5)
                    elif view == "front":
                        x = SHOULDER_Y - point[1]
                        c = centre(x, sign)
                        point[0] = round(c + (point[0] - c) * taper(x), 5)
                    else:
                        c = centre(0.15, sign)
                        f = taper(0.15)
                        point[0], point[1] = round(point[0] * f, 5), round(c + (point[1] - c) * f, 5)
                log.append(f"{scale} {view} {shape['id']} {len(points)} points redrawn")


def main():
    module = lateral()
    original = text = open(CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"])
    source_text = open(SOURCE, encoding="utf8").read()
    source = json.loads(source_text)
    log = []
    solids(source["volumes"], log)
    drawings(document["asset"], log)
    document["asset"]["dimensions"]["depthM"] = BREADTH
    print("\n".join(log))
    if "--apply" not in sys.argv:
        document["asset"]["volumes"] = source["volumes"]
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    text = module.splice(text, "representations", before["representations"], document["asset"]["representations"])
    text = module.splice(text, "dimensions", before["dimensions"], document["asset"]["dimensions"])
    if json.loads(text)["asset"]["representations"] != document["asset"]["representations"]:
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf8").read() != original or open(SOURCE, encoding="utf8").read() != source_text:
        raise SystemExit("a file changed while converting: run again")
    open(CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")
    print("applied")


if __name__ == "__main__":
    main()
