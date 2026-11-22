"""🦵 Chisel of the goat author: ham and gaskin of `animals/goat` re-cut by hand at every scale.

Rows are horizontal world sections of the rest pose, relative to the hip joint:
(y, rear edge x, front edge x, lateral centre w, lateral half, exponent); near side, the far twin mirrors w.
The rear line runs unbroken from the pin bone over the ham and the Achilles tendon to the point of the hock.
"""
import json
import math
import sys

SOURCE = "volumes/animals/goat.json"
THIGH = ("thigh", (0.0, 0.0, -56.5412), [
    (0.074, 0.010, 0.070, 0.012, 0.008, 2.0),
    (0.068, -0.022, 0.090, 0.011, 0.018, 2.0),
    (0.056, -0.052, 0.104, 0.008, 0.027, 2.0),
    (0.040, -0.074, 0.113, 0.005, 0.034, 2.0),
    (0.018, -0.090, 0.122, 0.002, 0.041, 2.1),
    (-0.020, -0.098, 0.136, -0.004, 0.047, 2.2),
    (-0.070, -0.094, 0.146, -0.008, 0.048, 2.2),
    (-0.120, -0.082, 0.150, -0.012, 0.045, 2.2),
    (-0.165, -0.064, 0.148, -0.015, 0.040, 2.2),
    (-0.200, -0.046, 0.142, -0.017, 0.034, 2.1),
    (-0.228, -0.022, 0.128, -0.018, 0.026, 2.0),
    (-0.244, 0.015, 0.108, -0.019, 0.015, 2.0),
])
GASKIN = ("gaskin", (0.118, -0.178, -116.1), [
    (-0.135, -0.036, 0.126, -0.016, 0.018, 2.0),
    (-0.160, -0.058, 0.142, -0.017, 0.031, 2.0),
    (-0.200, -0.042, 0.140, -0.018, 0.033, 2.0),
    (-0.240, -0.024, 0.116, -0.016, 0.030, 2.0),
    (-0.280, -0.014, 0.092, -0.012, 0.026, 2.0),
    (-0.320, -0.014, 0.070, -0.007, 0.022, 2.0),
    (-0.355, -0.019, 0.052, -0.003, 0.019, 2.0),
    (-0.385, -0.020, 0.040, -0.001, 0.018, 2.0),
    (-0.402, -0.006, 0.032, 0.000, 0.012, 2.0),
])
PICK = {"50": (range(12), range(9)), "100": ([0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], [0, 1, 2, 3, 4, 6, 7, 8]), "200": ([1, 2, 4, 6, 8, 9, 10], [1, 2, 4, 6, 7]), "500": ([1, 2, 5, 7, 9, 10], [1, 3, 5, 7])}
FLOOR = {"50": 0.0, "100": 0.0, "200": 0.016, "500": 0.038}


def sections(frame, rows, mirror, floor):
    jx, jy, angle = frame
    a = math.radians(angle)
    theta = -(a + math.pi / 2)
    s, c = math.sin(theta / 2), math.cos(theta / 2)
    rotation = [round(0.5 * (c - s), 8), round(0.5 * (c + s), 8), round(0.5 * (c + s), 8), round(0.5 * (c - s), 8)]
    out = []
    for y, rear, front, w, lateral, exponent in rows:
        dx, dy = (rear + front) / 2 - jx, y - jy
        u = dx * math.cos(a) + dy * math.sin(a)
        v = -dx * math.sin(a) + dy * math.cos(a)
        out.append({"at": [round(u, 5), round(v, 5), -w if mirror else w], "rx": max(round((front - rear) / 2, 5), floor), "ry": max(lateral, floor), "exponent": exponent, "rotation": rotation})
    return out


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, picks in PICK.items():
        body = document["volumes"][scale]["body"]
        for (name, frame, table), pick in zip((THIGH, GASKIN), picks):
            rows = [table[i] for i in pick]
            for side, mirror in (("n", False), ("f", True)):
                entry = next(e for e in body if e.get("id") == name + "-" + side)
                entry["sections"] = sections(frame, rows, mirror, FLOOR[scale])
                entry["capStart"] = True
                entry["capEnd"] = True
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] hind legs written", file=sys.stderr)


main()
