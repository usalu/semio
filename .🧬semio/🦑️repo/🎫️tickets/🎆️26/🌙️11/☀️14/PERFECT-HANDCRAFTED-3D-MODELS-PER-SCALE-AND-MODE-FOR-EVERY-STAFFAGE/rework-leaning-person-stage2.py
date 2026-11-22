"""🧥 Stage 2 of humans/leaning-person: the coarse hoodies get the rounded trunk and sloping shoulders of the 1:50 garment in eleven hand-set slices.

Slices are (height along the thorax bone, half breadth, half depth, superellipse exponent); hem and neckline stay as they were.
The hood, a part of its own at 1:50 and 1:100, is merged into the upper back here: HOOD moves the back of five slices out by twice
its value (half depth and centre both), so the garment still bears on the wall. Loaded by rework-leaning-person-build.py after stage 1.
"""
import copy

SLICES = [(-0.25, 0.177, 0.126, 2.5), (-0.1, 0.167, 0.122, 2.5), (0.02, 0.1695, 0.128, 2.5), (0.13, 0.18, 0.129, 2.4), (0.19, 0.19, 0.121, 2.35), (0.235, 0.193, 0.108, 2.3), (0.265, 0.189, 0.094, 2.25), (0.29, 0.163, 0.079, 2.12), (0.312, 0.117, 0.0675, 2.03)]


HOOD = {0.13: 0.004, 0.19: 0.009, 0.235: 0.011, 0.265: 0.011, 0.29: 0.007}


def apply(a, m):
    fine = next(s for s in a["volumes"]["50"]["body"] if s["id"] == "hoodie")["sections"]
    for scale in ("200", "500"):
        coarse = next(s for s in a["volumes"][scale]["body"] if s["id"] == "hoodie")
        middle = []
        for x, rx, ry, exponent in SLICES:
            twin = copy.deepcopy(min(fine, key=lambda c: abs(c["at"][0] - x)))
            twin["at"][0] = x
            twin.update({"rx": rx, "ry": round(ry + HOOD.get(x, 0), 5), "exponent": exponent})
            twin["at"][1] = round(twin["at"][1] + HOOD.get(x, 0), 5)
            middle.append(twin)
        coarse["sections"] = [coarse["sections"][0]] + middle + [coarse["sections"][-1]]
