"""🪢 Playground swing: suspension points spread to EN 1176-2 lateral stability (G >= S + 0.05 L).

Hand-chosen numbers: seat fixings S = 0.40 m (eye bolts at z = +-0.20), chain L = 1.9 m, so the hanger pins must be
at least 0.40 + 0.095 = 0.495 m apart; chosen 0.50 m (z = +-0.25). Each chain therefore leans inward by
asin(0.05 / 1.9) = 1.508 degrees about the bone's v axis through its pin. Hangers on the beam move 50 mm outward.

Usage: python3 rework-15b-playground-swing-spread.py candidate|catalog
  candidate: writes the source `volumes/objects/playground-swing.json` and the candidate document
             `🗑️generated/r3-playground-swing/🔣️.json` (catalog + new volumes + new 2D).
  catalog:   re-reads the catalog file and writes the 2D changes only (volumes go in through volumes-insert.py).
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/playground-swing/🔣️.json"
SOURCE = os.path.join(T, "volumes/objects/playground-swing.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-playground-swing/🔣️.json")

SEAT_Z = 0.20
PIN_Z = 0.25
SHIFT = PIN_Z - SEAT_Z
LENGTH = 1.9
PHI = math.asin(SHIFT / LENGTH)
PIVOT_Y = 2.45 - 0.085
BEAM_X = 1.0


def r6(value):
    return round(value, 6)


def turn(point, angle):
    x, y, z = point
    c, s = math.cos(angle), math.sin(angle)
    return [x * c + z * s, y, -x * s + z * c]


def lean(point, side):
    """🪢 A point of a chain that hung at z = side * SEAT_Z, leaned about the pin at z = side * PIN_Z."""
    x, y, z = turn([point[0], point[1], point[2] - side * SEAT_Z], side * PHI)
    return [r6(x), r6(y), r6(z + side * PIN_Z)]


def multiply(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
        aw * bw - ax * bx - ay * by - az * bz,
    ]


def lean_solid(solid, side):
    if solid["kind"] == "capsule":
        solid["p0"] = lean(solid["p0"], side)
        solid["p1"] = lean(solid["p1"], side)
        return
    angle = side * PHI
    quaternion = [0.0, math.sin(angle / 2), 0.0, math.cos(angle / 2)]
    transform = solid.get("transform", {})
    rotation = multiply(quaternion, transform.get("rotation", [0.0, 0.0, 0.0, 1.0]))
    translation = lean(transform.get("translation", [0.0, 0.0, 0.0]), side)
    solid["transform"] = {
        **({"scale": transform["scale"]} if "scale" in transform else {}),
        "rotation": [round(v, 8) for v in rotation],
        "translation": translation,
    }
    if "repeat" in solid and "translation" in solid["repeat"]:
        solid["repeat"]["translation"] = [r6(v) for v in turn(solid["repeat"]["translation"], angle)]


def move_hanger(solid, side):
    if "origin" in solid:
        solid["origin"][2] = r6(solid["origin"][2] + side * SHIFT)
    else:
        solid["transform"]["translation"][2] = r6(solid["transform"]["translation"][2] + side * SHIFT)


def side_of(identifier):
    return 1 if "left" in identifier else -1


def edit_volumes(volumes):
    for scale in volumes.values():
        for name in ("body", "thermal", "structure"):
            for solid in scale[name]:
                if "of" in solid:
                    continue
                if solid.get("bone") in ("chain-left", "chain-right"):
                    lean_solid(solid, 1 if solid["bone"] == "chain-left" else -1)
                elif solid.get("bone") == "beam" and "hanger" in solid["id"]:
                    move_hanger(solid, side_of(solid["id"]))


def chain_front(side, top, bottom, radius):
    """✏️ Front elevation of a leaning chain: x lateral, y height of the rest pose, from the pin down to the seat eye."""
    upper = side * (PIN_Z - top * SHIFT / LENGTH)
    lower = side * (PIN_Z - bottom * SHIFT / LENGTH)
    return [
        [r6(upper - radius), r6(PIVOT_Y - top)],
        [r6(upper + radius), r6(PIVOT_Y - top)],
        [r6(lower + radius), r6(PIVOT_Y - bottom)],
        [r6(lower - radius), r6(PIVOT_Y - bottom)],
    ]


def chain_plan(side, radius):
    inner, outer = sorted([side * (SEAT_Z - radius), side * (PIN_Z + radius)])
    return [[r6(BEAM_X - radius), r6(inner)], [r6(BEAM_X + radius), r6(inner)], [r6(BEAM_X + radius), r6(outer)], [r6(BEAM_X - radius), r6(outer)]]


def redraw(shape, points):
    top, bottom, radius = shape["x0"], shape["x1"], shape["r0"]
    for key in ("solid", "x0", "y0", "x1", "y1", "r0", "r1"):
        shape.pop(key, None)
    shape["kind"] = "poly"
    shape["points"] = points(top, bottom, radius)
    shape["closed"] = True


def edit_2d(asset):
    for representation in asset["representations"].values():
        geometry = representation.get("geometry", {})
        for view in ("front", "plan"):
            for shape in geometry.get(view, []):
                identifier = shape.get("id", "")
                if shape.get("bone") == "beam" and identifier.startswith("hanger"):
                    key = "x" if view == "front" else "y"
                    shape[key] = r6(shape[key] + side_of(identifier) * SHIFT)
                elif identifier in ("chain-left", "chain-right") and shape.get("kind") == "capsule":
                    side = side_of(identifier)
                    if view == "front":
                        redraw(shape, lambda top, bottom, radius: chain_front(side, top, bottom, radius))
                    else:
                        redraw(shape, lambda top, bottom, radius: chain_plan(side, radius))


def block(shape):
    return json.dumps(shape, ensure_ascii=False, indent=2).replace("\n", "\n" + " " * 12)


def swap(text, old, new):
    """✂️ Replaces the first occurrence of one shape in the catalog text, every other byte stays as it is."""
    if block(old) not in text:
        raise SystemExit("shape text not found: " + old["id"])
    return text.replace(block(old), block(new), 1)


def main():
    mode = sys.argv[1]
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    document = json.loads(text)
    if mode == "candidate":
        with open(SOURCE, encoding="utf-8") as handle:
            source = json.load(handle)
        edit_volumes(source["volumes"])
        with open(SOURCE, "w", encoding="utf-8") as handle:
            json.dump(source, handle, ensure_ascii=False, indent=1)
        document["asset"]["volumes"] = source["volumes"]
        edit_2d(document["asset"])
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        with open(CANDIDATE, "w", encoding="utf-8") as handle:
            json.dump(document, handle, ensure_ascii=False, indent=1)
    elif mode == "catalog":
        before = json.loads(text)
        edit_2d(document["asset"])
        for scale, representation in document["asset"]["representations"].items():
            for view in ("front", "plan"):
                olds = before["asset"]["representations"][scale].get("geometry", {}).get(view, [])
                for old, new in zip(olds, representation.get("geometry", {}).get(view, [])):
                    if old != new:
                        text = swap(text, old, new)
        with open(CATALOG, "w", encoding="utf-8") as handle:
            handle.write(text)
    else:
        raise SystemExit("usage: candidate|catalog")


main()
