"""💪 Chisel of the goat author: shoulder and upper arm of `animals/goat` re-cut by hand at every scale.

Rows are horizontal world sections of the rest pose, relative to the hip joint:
(y, rear edge x, front edge x, lateral centre beside the plane, lateral half, exponent); the far twin mirrors z.
The shoulder is the flat muscle sheet over the blade, lying on the ribs from the withers to the point of the
shoulder; the arm is the triceps wedge from the rear border of the blade to the olecranon with the humerus in front.
"""
import json
import math
import sys

SOURCE = "volumes/animals/goat.json"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json"
SHOULDER = [
    (0.108, 0.600, 0.660, 0.032, 0.009, 2.0),
    (0.098, 0.586, 0.673, 0.040, 0.016, 2.0),
    (0.080, 0.579, 0.686, 0.052, 0.022, 2.0),
    (0.055, 0.580, 0.700, 0.064, 0.025, 2.1),
    (0.025, 0.592, 0.716, 0.073, 0.027, 2.1),
    (-0.005, 0.608, 0.733, 0.077, 0.028, 2.1),
    (-0.030, 0.628, 0.749, 0.078, 0.029, 2.0),
    (-0.050, 0.648, 0.757, 0.077, 0.029, 2.0),
    (-0.068, 0.664, 0.755, 0.074, 0.026, 2.0),
    (-0.082, 0.678, 0.746, 0.071, 0.019, 2.0),
    (-0.091, 0.692, 0.734, 0.069, 0.010, 2.0),
]
ARM = [
    (-0.008, 0.644, 0.698, 0.070, 0.011, 2.0),
    (-0.024, 0.626, 0.726, 0.073, 0.022, 2.0),
    (-0.048, 0.614, 0.746, 0.076, 0.029, 2.0),
    (-0.078, 0.607, 0.747, 0.076, 0.032, 2.1),
    (-0.110, 0.603, 0.735, 0.076, 0.032, 2.1),
    (-0.140, 0.600, 0.720, 0.076, 0.031, 2.1),
    (-0.168, 0.600, 0.705, 0.076, 0.029, 2.0),
    (-0.192, 0.604, 0.694, 0.075, 0.026, 2.0),
    (-0.212, 0.613, 0.688, 0.075, 0.023, 2.0),
    (-0.228, 0.626, 0.685, 0.075, 0.020, 2.0),
    (-0.240, 0.642, 0.680, 0.075, 0.013, 2.0),
]
PICK = {"50": (range(11), range(11)), "100": ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), "200": ([1, 2, 4, 6, 7, 8, 9], [1, 2, 4, 6, 8, 9]), "500": ([2, 4, 6, 8], [2, 4, 6, 8])}
FLOOR = {"50": 0.0, "100": 0.0085, "200": 0.016, "500": 0.038}
PLANE = [[1, 0, 0], [0, 0, -1], [0, 1, 0]]


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rw(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rv(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def ru(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def frames(bones):
    by, out = {b["id"]: b for b in bones}, {}

    def frame(ident):
        if ident not in out:
            bone = by[ident]
            rest = mul(mul(rw(bone["angleDeg"]), rv(bone.get("abductionDeg", 0))), ru(bone.get("twistDeg", 0)))
            if bone["parent"] is None:
                out[ident] = ([bone["x"], bone["y"], bone.get("z", 0)], rest)
            else:
                origin, basis = frame(bone["parent"])
                moved = app(basis, [bone["x"], bone["y"], bone.get("z", 0)])
                out[ident] = ([origin[i] + moved[i] for i in range(3)], mul(basis, rest))
        return out[ident]

    for bone in bones:
        frame(bone["id"])
    return out


def quat(m):
    w = math.sqrt(max(0, 1 + m[0][0] + m[1][1] + m[2][2])) / 2
    if w > 1e-6:
        return [(m[2][1] - m[1][2]) / (4 * w), (m[0][2] - m[2][0]) / (4 * w), (m[1][0] - m[0][1]) / (4 * w), w]
    x = math.sqrt(max(0, 1 + m[0][0] - m[1][1] - m[2][2])) / 2
    y = math.sqrt(max(0, 1 - m[0][0] + m[1][1] - m[2][2])) / 2
    z = math.sqrt(max(0, 1 - m[0][0] - m[1][1] + m[2][2])) / 2
    return [x, math.copysign(y, m[0][1] + m[1][0]), math.copysign(z, m[0][2] + m[2][0]), 0]


def sections(frame, rows, side, floor):
    origin, basis = frame
    rotation = [round(q, 8) for q in quat(mul(tr(basis), PLANE))]
    out = []
    for y, rear, front, centre, lateral, exponent in rows:
        world = [(rear + front) / 2, y, side * centre]
        local = app(tr(basis), [world[i] - origin[i] for i in range(3)])
        out.append({"at": [round(v, 5) for v in local], "rx": max(round((front - rear) / 2, 5), floor), "ry": max(lateral, floor), "exponent": exponent, "rotation": rotation})
    return out


def main():
    with open(CATALOG, encoding="utf-8") as handle:
        catalog = json.load(handle)
    rig = catalog.get("asset", catalog)["rig"]
    world = frames(rig["bones"])
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, picks in PICK.items():
        body = document["volumes"][scale]["body"]
        for (name, bone, table), pick in zip((("shoulder", "scapula", SHOULDER), ("arm", "humerus", ARM)), picks):
            rows = [table[i] for i in pick]
            for suffix, side in (("n", -1), ("f", 1)):
                entry = next((e for e in body if e.get("id") == name + "-" + suffix), None)
                if entry is None:
                    print("[DEBUG] no", name, suffix, "at", scale, file=sys.stderr)
                    continue
                entry["bone"] = bone + "-f" + suffix
                entry["sections"] = sections(world[bone + "-f" + suffix], rows, side, FLOOR[scale])
                entry["capStart"] = True
                entry["capEnd"] = True
    for entry in document["volumes"]["500"]["structure"]:
        if entry.get("id") in ("scapula-n", "scapula-f"):
            for section in entry["sections"]:
                section["rx"] = section["ry"] = 0.038
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] shoulders written", file=sys.stderr)


main()
