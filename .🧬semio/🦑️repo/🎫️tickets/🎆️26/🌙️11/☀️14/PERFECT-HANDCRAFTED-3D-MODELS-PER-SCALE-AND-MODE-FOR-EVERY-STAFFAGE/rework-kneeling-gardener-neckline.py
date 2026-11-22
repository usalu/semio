"""👔 kneeling-gardener: the shirt gets a real neckline. The top of the shirt was a level ring at the height of C7, 7 mm under the chin, so no neck showed from the front.

The upper shirt sections are tilted about the lateral axis, up to 30 degrees at the opening, and lowered by up to 12 mm: the neckline now lies 36 mm below C7 in front (above the jugular notch) and rises 18 mm above C7 at the nape; the collar band follows at 30 degrees. The neck is carried 40 mm further down into the shirt so the opening is filled. Elevation drawings of shirt and collar are redrawn from the same numbers; front and plan profiles are raised to the nape.

Usage: python3 rework-kneeling-gardener-neckline.py            candidate document
       python3 rework-kneeling-gardener-neckline.py --apply    catalog file (representations) and volumes source (publish with volumes-insert.py)
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "kneeling-gardener", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "kneeling-gardener.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-kneeling-gardener", "neckline", "🔣️.json")

TILT_FROM = 0.255
TILT_FULL = 0.306
TILT_DEG = 30.0
LOWER_FROM = 0.266
LOWER_OVER = 0.04
LOWER = 0.012
COLLAR_LOWER = 0.014
NECK_BASE = {"at": [-0.075, 0.02, 0], "rx": 0.058, "ry": 0.056}
THERMAL_NECK_BASE = {"at": [-0.012, 0.013, 0], "rx": 0.051, "ry": 0.051}
KEYS = ("representations",)


def r5(value):
    return round(value, 5)


def clamp(value):
    return min(1.0, max(0.0, value))


def tilted(quaternion, degrees):
    half = math.radians(-degrees) / 2
    ax, ay, az, aw = 0.0, 0.0, math.sin(half), math.cos(half)
    bx, by, bz, bw = quaternion
    return [
        r5(aw * bx + ax * bw + ay * bz - az * by),
        r5(aw * by - ax * bz + ay * bw + az * bx),
        r5(aw * bz + ax * by - ay * bx + az * bw),
        r5(aw * bw - ax * bx - ay * by - az * bz),
    ]


def neckline(solid):
    edges = []
    for section in solid["sections"]:
        u = section["at"][0]
        collar = solid["id"] == "shirt-collar"
        degrees = TILT_DEG if collar else TILT_DEG * clamp((u - TILT_FROM) / (TILT_FULL - TILT_FROM))
        lower = COLLAR_LOWER if collar else LOWER * clamp((u - LOWER_FROM) / LOWER_OVER)
        if degrees == 0:
            continue
        section["at"][0] = r5(u - lower)
        section["rotation"] = tilted(section["rotation"], degrees)
        t = math.radians(degrees)
        edges.append(([r5(section["at"][0] - section["ry"] * math.sin(t)), r5(section["at"][1] - section["ry"] * math.cos(t))],
                      [r5(section["at"][0] + section["ry"] * math.sin(t)), r5(section["at"][1] + section["ry"] * math.cos(t))]))
    return edges


def rebuild_volumes(volumes):
    edges = {}
    for scale, lists in volumes.items():
        for name in ("body", "thermal"):
            for solid in lists[name]:
                if solid.get("bone") == "torso" and solid.get("kind") == "loft" and solid["id"] in ("shirt", "shirt-collar", "th-shirt-torso-air"):
                    found = neckline(solid)
                    if name == "body":
                        edges[(scale, solid["id"])] = found
                if solid.get("bone") == "neck" and solid.get("kind") == "loft" and solid["id"] in ("neck", "th-neck"):
                    base = dict(NECK_BASE if solid["id"] == "neck" else THERMAL_NECK_BASE)
                    base["at"] = list(base["at"])
                    first = solid["sections"][0]
                    for key in ("exponent", "rotation"):
                        if key in first:
                            base[key] = first[key]
                    solid["sections"].insert(0, base)
    return edges


def rebuild_shapes(geometry, scale, edges):
    for shape in geometry["elevation"]:
        if shape.get("bone") != "torso":
            continue
        found = edges.get((scale, shape["id"]))
        if shape["id"] == "shirt" and found:
            low = [p for p in shape["points"] if p[0] <= TILT_FROM]
            front = sorted((p for p in low if p[1] < 0), key=lambda p: p[0])
            back = sorted((p for p in low if p[1] >= 0), key=lambda p: -p[0])
            shape["points"] = front + [e[0] for e in found] + [e[1] for e in reversed(found)] + back
        if shape["id"] == "shirt-collar" and found:
            shape["points"] = [found[0][0], found[1][0], found[1][1], found[0][1]]
            shape["z"] = 4.85
    for view in ("front", "plan"):
        for shape in geometry[view]:
            if shape.get("bone") != "torso":
                continue
            if shape["id"] == "shirt":
                shape["points"] = [[r5(p[0] + (0.018 if p[0] >= 0.302 else 0.012 if p[0] >= 0.295 else 0.0)), p[1]] for p in shape["points"]]
            if shape["id"] == "shirt-collar":
                shape["points"] = [[r5(p[0] + (0.015 if p[0] >= 0.31 else 0.0)), p[1]] for p in shape["points"]]


def rebuild(asset, volumes):
    shirt = next(s for s in volumes["50"]["body"] if s["id"] == "shirt")
    if any(abs(s["rotation"][2]) > 1e-6 for s in shirt["sections"]):
        raise SystemExit("the neckline is already tilted")
    edges = rebuild_volumes(volumes)
    for scale, representation in asset["representations"].items():
        if representation.get("geometry"):
            rebuild_shapes(representation["geometry"], scale, edges)


def main():
    original = open(CATALOG, encoding="utf-8").read()
    document = json.loads(original)
    source = json.load(open(SOURCE, encoding="utf-8"))
    asset = document["asset"]
    rebuild(asset, source["volumes"])
    if "--apply" not in sys.argv:
        asset["volumes"] = source["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    text = open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read()
    insert = {}
    exec(compile(text[: text.index("\ndef main(")], "volumes-insert.py", "exec"), insert)
    result = original
    for key in KEYS:
        start, end = insert["member_range"](result, "    ", key)
        result = result[:start] + '    "' + key + '": ' + insert["dump"](asset[key], 2) + result[end:]
    check = json.loads(result)["asset"]
    if any(check[key] != asset[key] for key in KEYS):
        raise SystemExit("splice did not round-trip")
    if open(CATALOG, encoding="utf-8").read() != original:
        raise SystemExit("the catalog file changed while converting: run again")
    open(CATALOG, "w", encoding="utf-8").write(result)
    json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf-8").write("\n")
    print("applied to", CATALOG, "and", SOURCE)


if __name__ == "__main__":
    main()
