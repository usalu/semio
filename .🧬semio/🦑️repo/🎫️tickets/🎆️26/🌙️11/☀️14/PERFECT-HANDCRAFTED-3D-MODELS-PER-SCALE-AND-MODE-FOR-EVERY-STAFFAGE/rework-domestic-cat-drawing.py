"""✏️ Side elevation of the domestic cat: the trunk shapes end in the same rounded noses as the trunk lofts, so a bent spine draws no flat cap."""
import json, math, sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/domestic-cat/🔣️.json"
NOSES = {
    "50": {"rump": [("end", 0.0474, 0.058)], "hindbody": [("start", 0.0, 0.055), ("end", 0.1331, 0.060)], "midbody": [("start", 0.0, 0.060), ("end", 0.0677, 0.058)], "chest": [("start", 0.0, 0.056)]},
    "100": {"rump": [("end", 0.0474, 0.058)], "hindbody": [("start", 0.0, 0.055), ("end", 0.1331, 0.060)], "midbody": [("start", 0.0, 0.060), ("end", 0.0677, 0.058)], "chest": [("start", 0.0025, 0.058)]},
    "200": {"hindbody": [("end", 0.147, 0.060)], "chest": [("start", -0.030, 0.062)]},
}
STATIONS = {"50": (0.4, 0.7, 0.9), "100": (0.45, 0.85), "200": (0.6,)}


def nose(points, side, x0, length, stations):
    """👃 Replace the run of outline points beyond the joint station by a half ellipse between the two crossings."""
    beyond = [(p[0] > x0) if side == "end" else (p[0] < x0) for p in points]
    count = len(points)
    first = next(i for i in range(count) if beyond[i] and not beyond[i - 1])
    points, beyond = points[first:] + points[:first], beyond[first:] + beyond[:first]
    run = beyond.index(False)
    if any(beyond[run:]):
        raise SystemExit("outline crosses the joint station more than twice")
    def crossing(a, b):
        u = (x0 - a[0]) / (b[0] - a[0])
        return [x0, a[1] + u * (b[1] - a[1])]
    enter, leave = crossing(points[-1], points[0]), crossing(points[run - 1], points[run])
    centre, half, sign = (enter[1] + leave[1]) / 2, (enter[1] - leave[1]) / 2, 1 if side == "end" else -1
    shape = lambda t: math.sqrt(1 - t * t) * (1 - 0.12 * t)
    arc = [[x0 + sign * length * t, centre + half * shape(t)] for t in stations] + [[x0 + sign * length, centre]] + [[x0 + sign * length * t, centre - half * shape(t)] for t in reversed(stations)]
    return [[round(c, 5) for c in p] for p in [enter] + arc + [leave]] + points[run:]


text = open(CATALOG).read()
document = json.loads(text)
for scale, shapes in NOSES.items():
    elevation = {shape["id"]: shape for shape in document["asset"]["representations"][scale]["geometry"]["elevation"]}
    for identifier, ends in shapes.items():
        old = '"points": ' + json.dumps(elevation[identifier]["points"])
        for side, x0, length in ends:
            elevation[identifier]["points"] = nose(elevation[identifier]["points"], side, x0, length, STATIONS[scale])
        if text.count(old) != 1:
            raise SystemExit(f"{scale} {identifier}: outline text found {text.count(old)} times")
        text = text.replace(old, '"points": ' + json.dumps(elevation[identifier]["points"]))
        print(scale, identifier, len(elevation[identifier]["points"]), "points")
if json.loads(text) != document:
    raise SystemExit("spliced text differs from the edited document")
open(sys.argv[1] if len(sys.argv) > 1 else CATALOG, "w").write(text)
