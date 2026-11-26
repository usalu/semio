"""🕳️ Repair of the filled paths whose subpaths overlap (`🧪️tests/🕳️fill-rule`, catalog case): under the even-odd rule the doubly covered area is painted as a hole. Each path is rewritten in place of its `d` string and nothing else of the document changes.

- `park-bench` 1:200 and 1:500 plan `three-back-slats`: the three reclined slats cover each other seen from above (0.1815..0.2273, 0.2044..0.2502, 0.22..0.2658 m); the path becomes their outline, one ring 0.1815..0.2658 m.
- `seated-office-worker` 1:200 elevation `chair-tyre-front`: the inner ring of the tyre was simplified apart from the outer one and crosses it; it becomes the outer pentagon shrunk to 0.85 about its centroid (the 3.75 mm tyre wall of the 25 mm castor, as the octagon ring of 1:100), wound the other way.
- `tram` 1:50 and 1:100 elevation `bogie-legs-f`, `bogie-legs-r`: the two legs and the cross bar become one U-shaped ring, their outline.

Idempotent: a document that already holds the new string is left alone. Usage: fill-rule.py <asset id>… [--trial <dir>] (with --trial the repaired copy is written to <dir>/<id>.json and the catalog is not touched)."""
import glob
import json
import sys

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def ring(points):
    return "M " + " L ".join(f"{x:g} {y:g}" for x, y in points) + " Z"


def area(points):
    return abs(sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points)))) / 2


def bogie(leg_in, leg_out, top, bar_top):
    points = [(-leg_out, top), (-leg_out, -0.18), (-0.46, -0.18), (-0.46, -0.26), (0.46, -0.26), (0.46, -0.18), (leg_out, -0.18), (leg_out, top), (leg_in, top), (leg_in, bar_top), (-leg_in, bar_top), (-leg_in, top)]
    legs = 2 * (leg_out - leg_in) * (top + 0.18)
    bar = 0.92 * (bar_top + 0.26)
    overlap = 2 * (0.46 - leg_in) * (bar_top + 0.18)
    assert abs(area(points) - (legs + bar - overlap)) < 1e-12, (area(points), legs, bar, overlap)
    return ring(points)


TYRE_OUTER = [(-0.0077, -0.02489), (0.02418, -0.00989), (0.0223, 0.01261), (0.00355, 0.02573), (-0.02457, 0.00698)]
TYRE_CENTRE = (sum(p[0] for p in TYRE_OUTER) / 5, sum(p[1] for p in TYRE_OUTER) / 5)
TYRE_INNER = [(round(TYRE_CENTRE[0] + 0.85 * (x - TYRE_CENTRE[0]), 5), round(TYRE_CENTRE[1] + 0.85 * (y - TYRE_CENTRE[1]), 5)) for x, y in reversed(TYRE_OUTER)]

REPAIRS = {
    "park-bench": [(
        "M 0.0 0.1815 L 1.8 0.1815 L 1.8 0.2273 L 0.0 0.2273 Z M 0.0 0.2044 L 1.8 0.2044 L 1.8 0.2502 L 0.0 0.2502 Z M 0.0 0.22 L 1.8 0.22 L 1.8 0.2658 L 0.0 0.2658 Z",
        "M 0.0 0.1815 L 1.8 0.1815 L 1.8 0.2658 L 0.0 0.2658 Z", 2)],
    "seated-office-worker": [(
        "M -0.0077 -0.02489 L 0.02418 -0.00989 L 0.0223 0.01261 L 0.00355 0.02573 L -0.02457 0.00698 Z M -0.00395 -0.02114 L -0.01895 -0.01177 L -0.02082 0.00698 L 0.00918 0.02011 L 0.02043 -0.00802 Z",
        "M -0.0077 -0.02489 L 0.02418 -0.00989 L 0.0223 0.01261 L 0.00355 0.02573 L -0.02457 0.00698 Z " + ring(TYRE_INNER), 1)],
    "tram": [
        ("M -0.55 -0.18 L -0.45 -0.18 L -0.45 0.21 L -0.55 0.21 Z M 0.45 -0.18 L 0.55 -0.18 L 0.55 0.21 L 0.45 0.21 Z M -0.46 -0.26 L 0.46 -0.26 L 0.46 -0.16 L -0.46 -0.16 Z", bogie(0.45, 0.55, 0.21, -0.16), 2),
        ("M -0.56 -0.18 L -0.44 -0.18 L -0.44 0.19 L -0.56 0.19 Z M 0.44 -0.18 L 0.56 -0.18 L 0.56 0.19 L 0.44 0.19 Z M -0.46 -0.26 L 0.46 -0.26 L 0.46 -0.14 L -0.46 -0.14 Z", bogie(0.44, 0.56, 0.19, -0.14), 2)],
}

args = sys.argv[1:]
trial = args[args.index("--trial") + 1] if "--trial" in args else None
ids = [a for i, a in enumerate(args) if a != "--trial" and (i == 0 or args[i - 1] != "--trial")]
for asset in ids:
    path = glob.glob(f"{S}/*/{asset}/🔣️.json")[0]
    text = open(path, encoding="utf-8").read()
    changed = 0
    for old, new, count in REPAIRS[asset]:
        found = text.count(json.dumps(old))
        if found == 0 and text.count(json.dumps(new)) == count:
            continue
        assert found == count, f"{asset}: {found} occurrences of the old path, expected {count}"
        text = text.replace(json.dumps(old), json.dumps(new))
        changed += found
    json.loads(text)
    if changed == 0:
        print(f"{asset}: unchanged")
        continue
    target = f"{trial}/{asset}.json" if trial else path
    if not trial:
        assert open(path, encoding="utf-8").read().count(json.dumps(REPAIRS[asset][0][0])) == REPAIRS[asset][0][2], f"{asset}: the document changed while it was repaired"
    open(target, "w", encoding="utf-8").write(text)
    print(f"{asset}: {changed} paths rewritten -> {target}")
