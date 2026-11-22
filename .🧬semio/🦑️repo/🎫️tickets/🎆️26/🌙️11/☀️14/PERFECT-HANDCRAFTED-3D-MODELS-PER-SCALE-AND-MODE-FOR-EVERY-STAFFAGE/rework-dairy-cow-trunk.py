"""🐮 Dairy cow trunk chisel: hand-cut top and bottom lines of neck, withers, fore chest, muzzle and cheek for every scale.

Usage: python3 rework-dairy-cow-trunk.py <asset document> (in place)
A line row is (u, top, bottom, half breadth) in the bone frame (u along the bone, top and bottom along its normal); a section row is (u, centre, half depth, half breadth).
Reference (Holstein-Friesian cow, 1.45 m): neck 0.55 m deep and 0.30 m broad at the shoulder, 0.29 x 0.17 m at the throat, top line dipping 0.08 m behind the poll; withers 0.03 m above the back, no hump; fore chest ending 0.10 m ahead of the point of the shoulder; muzzle 0.17 m broad; cheek 0.19 m deep over the masseter.
"""
import json
import sys

LIMB = [0.5, 0.5, 0.5, 0.5]

NECK = {
    "50": [(-0.10, 0.160, -0.420, 0.150), (0.05, 0.118, -0.395, 0.132), (0.18, 0.082, -0.345, 0.114), (0.32, 0.060, -0.300, 0.098), (0.46, 0.052, -0.265, 0.089), (0.58, 0.055, -0.240, 0.085), (0.68, 0.062, -0.222, 0.084)],
    "100": [(-0.10, 0.160, -0.420, 0.150), (0.08, 0.110, -0.385, 0.128), (0.28, 0.066, -0.312, 0.102), (0.50, 0.052, -0.257, 0.088), (0.68, 0.062, -0.222, 0.084)],
    "200": [(-0.10, 0.160, -0.420, 0.150), (0.12, 0.100, -0.370, 0.122), (0.40, 0.055, -0.280, 0.092), (0.68, 0.062, -0.222, 0.084)],
    "500": [(-0.10, 0.160, -0.420, 0.150), (0.28, 0.066, -0.315, 0.105), (0.68, 0.065, -0.225, 0.088)],
}
CREST = [(0.0, 0.092, 0.040, 0.050), (0.15, 0.052, 0.040, 0.046), (0.35, 0.020, 0.038, 0.044), (0.55, 0.016, 0.038, 0.046), (0.67, 0.024, 0.038, 0.052)]
BACK = {
    "50": (0.70, [(0.78, -0.091, 0.125, 0.140), (0.88, -0.082, 0.130, 0.122), (0.97, -0.077, 0.135, 0.106), (1.06, -0.079, 0.135, 0.094), (1.14, -0.085, 0.125, 0.085), (1.21, -0.103, 0.115, 0.078)]),
    "100": (0.70, [(0.78, -0.091, 0.125, 0.140), (0.92, -0.079, 0.132, 0.115), (1.04, -0.077, 0.135, 0.096), (1.19, -0.098, 0.118, 0.080)]),
}
BARREL = {
    "50": (1.20, [(1.21, -0.355, 0.295, 0.212), (1.27, -0.365, 0.250, 0.175), (1.315, -0.375, 0.180, 0.125)]),
    "100": (1.26, [(1.27, -0.365, 0.250, 0.175), (1.32, -0.375, 0.165, 0.115)]),
}
HEAD = {"50": {0.41: 0.074, 0.47: 0.082, 0.505: 0.086}, "100": {0.42: 0.074, 0.505: 0.086}, "200": {0.52: 0.086}, "500": {0.52: 0.084}}
JAW = {"50": {0.01: (0.095, 0.094), 0.1: (0.082, 0.084)}, "100": {0.02: (0.095, 0.094)}, "200": {0.02: (0.095, 0.094)}}
MUZZLE = [0.028, 0.052, 0.078]


def section(u, centre, depth, breadth, exponent, rotation=None):
    row = {"at": [round(u, 4), round(centre, 4), 0], "rx": round(depth, 4), "ry": round(breadth, 4), "rotation": list(rotation or LIMB)}
    if exponent is not None:
        row["exponent"] = exponent
    return row


def domed(rows, exponent, start=True, end=True):
    first, last = rows[0], rows[-1]
    head = [(first[0] - d * 0.05, first[1], first[2] * k, first[3] * k) for d, k in ((0.6, 0.34), (0.44, 0.66), (0.24, 0.9))] if start else []
    tail = [(last[0] + d * 0.05, last[1], last[2] * k, last[3] * k) for d, k in ((0.24, 0.9), (0.44, 0.66), (0.6, 0.34))] if end else []
    return [section(u, c, a, b, exponent) for u, c, a, b in head + list(rows) + tail]


def lines(rows):
    return [(u, (top + bottom) / 2, (top - bottom) / 2, breadth) for u, top, bottom, breadth in rows]


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    volumes = document["asset"]["volumes"]
    for scale in ("50", "100", "200", "500"):
        listing = {entry["id"]: entry for entry in volumes[scale]["body"]}
        listing["neck"]["sections"] = domed(lines(NECK[scale]), 2.3)
        if "neck-crest" in listing:
            listing["neck-crest"]["sections"] = domed(CREST, None)
        if "dewlap" in listing:
            for row in listing["dewlap"]["sections"]:
                row["at"][1] = round(row["at"][1] - 0.035 * max(0.0, row["at"][0] - 0.129) / 0.476, 4)
        for name, table in (("back", BACK), ("barrel", BARREL)):
            if scale in table and name in listing:
                cut, rows = table[scale]
                entry = listing[name]
                exponent = entry["sections"][-1].get("exponent")
                entry["sections"] = [row for row in entry["sections"] if row["at"][0] < cut] + domed(rows, exponent, start=False)
        if scale == "200":
            for row in listing["trunk"]["sections"]:
                if abs(row["at"][0] - 0.98) < 1e-6:
                    row["at"][1], row["rx"] = -0.325, 0.383
        head = listing["head"]
        last = None
        for row in head["sections"]:
            for station, breadth in HEAD[scale].items():
                if abs(row["at"][0] - station) < 1e-6:
                    row["ry"], last = breadth, breadth
        if last is not None:
            for row, k in zip(head["sections"][-3:], (0.9, 0.66, 0.34)):
                row["ry"] = round(last * k, 5)
        if "muzzle" in listing and listing["muzzle"].get("kind") == "ellipsoid":
            listing["muzzle"]["radii"] = list(MUZZLE)
        if "jaw" in listing:
            for row in listing["jaw"]["sections"]:
                for station, (depth, breadth) in JAW.get(scale, {}).items():
                    if abs(row["at"][0] - station) < 1e-6:
                        row["rx"], row["ry"] = depth, breadth
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


main()
