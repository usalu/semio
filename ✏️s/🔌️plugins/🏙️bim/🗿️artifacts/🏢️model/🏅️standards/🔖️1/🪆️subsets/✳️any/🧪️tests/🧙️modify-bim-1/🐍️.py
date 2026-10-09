#!/usr/bin/env python3
"""🧙️ Third-party ORACLE for the `s.bim.model@1` modify kernel (`🧬️schema/🧬️mutations/🧙️modify`) and for the wall end joins of `🧱️wall-layout`.

The subject (Rust, `semio-s-artifact-bim-model`) copies, mirrors, arrays, offsets, trims, extends and cuts authored geometry, and joins
wall ends as the authored preference says. The numbers behind those mutations are plane geometry. This file recomputes each of them from the
SAME committed cases with libraries that have never seen this repository (`shapely` 2 / GEOS for affine maps, parallel curves, line
intersections, polygon halves and the footprints of joined walls, `numpy` for circles) and either writes the expectations (`write`) or
audits the committed ones (`check`). The Rust subject replays the cases through its own kernel and compares against the same JSON, so subject
and oracle meet on one fixture.

* `maps`: the image of points and directions under a translation, a rotation about a pivot and a reflection about a line
  (`shapely.affinity.translate`, `rotate`, `affine_transform` with the reflection matrix);
* `loops`: the mirror image of a closed bulged loop keeps its area and centroid mirrored and runs counter-clockwise (arcs sampled into 4096 chords);
* `offsets`: the parallel curve of a line (`LineString.offset_curve`) and of an arc (concentric circle);
* `trims`: the end of a wall moved onto the carrier of another axis (infinite line or full circle intersections);
* `splits`: the two halves a straight cut leaves of a polygon (`Polygon.intersection` with a half plane): areas and centroids;
* `arrays`: the images of a point under the k-th turn of a radial array;
* `joins`: the footprints of two walls meeting at a node, with the authored join preferences, as GEOS intersections of the offset carriers of
  their faces (mitre without a limit, butt against the near face with the through wall running on to the far face, free ends).

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🧙️modify/🔣️.json>
    python 🐍️.py write <path to 🧫️fixtures/🧙️modify/🔣️.json>

@see ../../🧫️fixtures/🧙️modify/🔣️.json — the cases and their expectations
@see https://shapely.readthedocs.io/en/stable/manual.html#shapely.affinity.affine_transform
"""

import json
import math
import sys
from pathlib import Path

import numpy
from shapely import affinity
from shapely.geometry import LineString, Point, Polygon
from shapely.geometry.polygon import orient

TOLERANCE = 1e-9
SAMPLED = 1e-5
ARC_CHORDS = 4096
REACH = 1.0e4


# region 🔖️Maps
def reflection(line):
    """🪞️ The affine matrix `[a, b, d, e, xoff, yoff]` of the reflection about the line through two points."""
    (x0, y0), (x1, y1) = line
    phi = math.atan2(y1 - y0, x1 - x0)
    c, s = math.cos(2 * phi), math.sin(2 * phi)
    matrix = numpy.array([[c, s], [s, -c]])
    shift = numpy.array([x0, y0]) - matrix @ numpy.array([x0, y0])
    return [matrix[0][0], matrix[0][1], matrix[1][0], matrix[1][1], shift[0], shift[1]]


def image(kind, case, point):
    """📍️ The image of one point under the map a case describes, measured by shapely."""
    geometry = Point(point)
    if kind == "translate":
        moved = affinity.translate(geometry, *case["vector"])
    elif kind == "rotate":
        moved = affinity.rotate(geometry, case["angle"], origin=tuple(case["pivot"]), use_radians=True)
    else:
        moved = affinity.affine_transform(geometry, reflection(case["line"]))
    return [moved.x, moved.y]


def direction(kind, case, theta):
    """🧭️ The image of a direction angle, measured on the unit vector `(cos theta, sin theta)` carried through the same map as a point."""
    origin, tip = image(kind, case, (0.0, 0.0)), image(kind, case, (math.cos(theta), math.sin(theta)))
    return math.atan2(tip[1] - origin[1], tip[0] - origin[0])


def maps(cases):
    """🗺️ Images of the points and directions of every map case."""
    out = {}
    for kind, rows in cases.items():
        out[kind] = [{**case, "images": [image(kind, case, point) for point in case["points"]], "turned": [direction(kind, case, theta) for theta in case["directions"]]} for case in rows]
    return out


# endregion 🔖️Maps


# region 🔖️Loops
def arc_points(start, end, bulge, chords):
    """〰️ Points along the bulged segment from `start` to `end`, the end point excluded."""
    if abs(bulge) < 1e-12:
        return [start]
    sweep = 4 * math.atan(bulge)
    chord = math.dist(start, end)
    radius = chord / (2 * math.sin(abs(sweep) / 2))
    mid = ((start[0] + end[0]) / 2, (start[1] + end[1]) / 2)
    offset = chord / 2 * (1 - bulge**2) / (2 * bulge)
    centre = (mid[0] - (end[1] - start[1]) / chord * offset, mid[1] + (end[0] - start[0]) / chord * offset)
    first = math.atan2(start[1] - centre[1], start[0] - centre[0])
    return [(centre[0] + radius * math.cos(first + sweep * k / chords), centre[1] + radius * math.sin(first + sweep * k / chords)) for k in range(chords)]


def flatten(loop):
    """🔷️ A polygon whose chords approximate the arcs of a bulged loop."""
    points = []
    for index, (x, y, bulge) in enumerate(loop):
        following = loop[(index + 1) % len(loop)]
        points.extend(arc_points((x, y), (following[0], following[1]), bulge, ARC_CHORDS))
    return Polygon(points)


def exact_area(loop):
    """📐️ Signed area of a bulged loop: the shoelace sum plus the signed circular segments."""
    total = 0.0
    for index, (x, y, bulge) in enumerate(loop):
        nx, ny, _ = loop[(index + 1) % len(loop)]
        total += 0.5 * (x * ny - nx * y)
        if abs(bulge) > 1e-12:
            sweep = 4 * math.atan(bulge)
            radius = math.dist((x, y), (nx, ny)) / (2 * math.sin(abs(sweep) / 2))
            total += 0.5 * radius**2 * (sweep - math.sin(sweep))
    return total


def loops(cases):
    """➰️ Area, centroid and orientation of a loop and of its mirror image."""
    out = []
    for case in cases:
        polygon = flatten([tuple(v) for v in case["loop"]])
        mirrored = affinity.affine_transform(polygon, reflection(case["line"]))
        out.append({**case, "area": exact_area([tuple(v) for v in case["loop"]]), "image_area": mirrored.area, "centroid": list(polygon.centroid.coords[0]), "image_centroid": list(mirrored.centroid.coords[0]), "image_counter_clockwise": orient(mirrored, 1.0).exterior.is_ccw})
    return out


# endregion 🔖️Loops


# region 🔖️Curves
def circle(axis):
    """⭕️ Centre, radius and signed sweep of an arc axis (numpy)."""
    start, end, bulge = numpy.array(axis["start"], float), numpy.array(axis["end"], float), axis["bulge"]
    sweep = 4 * math.atan(bulge)
    chord = numpy.linalg.norm(end - start)
    radius = chord / (2 * math.sin(abs(sweep) / 2))
    direction = (end - start) / chord
    offset = chord / 2 * (1 - bulge**2) / (2 * bulge)
    centre = (start + end) / 2 + numpy.array([-direction[1], direction[0]]) * offset
    return centre, radius, sweep


def carrier(axis):
    """📏️ The infinite carrier of a line axis as a long shapely line."""
    start, end = numpy.array(axis["start"], float), numpy.array(axis["end"], float)
    direction = (end - start) / numpy.linalg.norm(end - start)
    return LineString([tuple(start - direction * REACH), tuple(end + direction * REACH)])


def offset_axis(axis, distance):
    """↔️ The parallel axis at `distance` to the left of the direction of travel (shapely offset curve, concentric circle for an arc)."""
    if abs(axis["bulge"]) < 1e-12:
        line = LineString([tuple(axis["start"]), tuple(axis["end"])]).offset_curve(distance)
        points = list(line.coords)
        if math.dist(points[0], axis["start"]) > math.dist(points[-1], axis["start"]):
            points.reverse()
        return {"start": list(points[0]), "end": list(points[-1]), "bulge": 0.0}
    centre, radius, sweep = circle(axis)
    shrunk = radius - distance * (1 if sweep > 0 else -1)
    scale = shrunk / radius
    start = centre + (numpy.array(axis["start"], float) - centre) * scale
    end = centre + (numpy.array(axis["end"], float) - centre) * scale
    return {"start": list(start), "end": list(end), "bulge": axis["bulge"]}


def offsets(cases):
    return [{**case, "image": offset_axis(case["axis"], case["distance"])} for case in cases]


def meet(axis, target, near):
    """🎯️ The intersection of the carriers of two axes nearest to `near` (lines: GEOS; arcs: numpy circle formulas)."""
    if abs(axis["bulge"]) < 1e-12 and abs(target["bulge"]) < 1e-12:
        hit = carrier(axis).intersection(carrier(target))
        return None if hit.is_empty or hit.geom_type != "Point" else [hit.x, hit.y]
    if abs(axis["bulge"]) < 1e-12:
        line, arc = axis, target
    else:
        line, arc = target, axis
    centre, radius, _ = circle(arc)
    start, end = numpy.array(line["start"], float), numpy.array(line["end"], float)
    direction = (end - start) / numpy.linalg.norm(end - start)
    to_centre = start - centre
    b = 2 * direction @ to_centre
    c = to_centre @ to_centre - radius**2
    discriminant = b * b - 4 * c
    if discriminant < 0:
        return None
    roots = [(-b + sign * math.sqrt(discriminant)) / 2 for sign in (-1, 1)]
    points = [start + direction * root for root in roots]
    return list(min(points, key=lambda point: math.dist(point, near)))


def trims(cases):
    out = []
    for case in cases:
        axis = case["axis"]
        near = axis["start"] if case["end"] == "Start" else axis["end"]
        out.append({**case, "point": meet(axis, case["target"], near)})
    return out


# endregion 🔖️Curves


# region 🔖️Splits
def splits(cases):
    """✂️ The halves of a polygon on the two sides of the infinite line through two points, areas and centroids."""
    out = []
    for case in cases:
        polygon = flatten([tuple(v) for v in case["loop"]])
        (x0, y0), (x1, y1) = case["line"]
        length = math.hypot(x1 - x0, y1 - y0)
        ux, uy = (x1 - x0) / length, (y1 - y0) / length
        far = REACH
        left = Polygon([(x0 - ux * far, y0 - uy * far), (x0 + ux * far, y0 + uy * far), (x0 + ux * far - uy * far, y0 + uy * far + ux * far), (x0 - ux * far - uy * far, y0 - uy * far + ux * far)])
        half, rest = polygon.intersection(left), polygon.difference(left)
        out.append({**case, "left_area": half.area, "right_area": rest.area, "left_centroid": list(half.centroid.coords[0]), "right_centroid": list(rest.centroid.coords[0])})
    return out


# endregion 🔖️Splits


# region 🔖️Arrays
def arrays(cases):
    return [{**case, "images": [list(affinity.rotate(Point(case["point"]), case["step"] * k, origin=tuple(case["center"]), use_radians=True).coords[0]) for k in range(1, case["count"] + 1)]} for case in cases]


# endregion 🔖️Arrays


# region 🔖️Joins
def band(wall):
    """🧱️ The unit direction, left normal and half thickness of a straight wall of location line Center."""
    start, end = numpy.array(wall["start"], float), numpy.array(wall["end"], float)
    direction = (end - start) / numpy.linalg.norm(end - start)
    return start, end, direction, numpy.array([-direction[1], direction[0]]), wall["thickness"] / 2


def face(wall, side):
    """↔️ The carrier of the left (+1) or right (-1) face of a wall."""
    start, end, direction, normal, half = band(wall)
    shift = normal * half * side
    return LineString([tuple(start + shift - direction * REACH), tuple(end + shift + direction * REACH)])


def cross(a, b, near):
    hit = a.intersection(b)
    if hit.is_empty or hit.geom_type != "Point":
        return None
    return [hit.x, hit.y]


def square_ends(wall):
    """⬜️ The four face end points of a free wall: `(left.start, left.end, right.start, right.end)`."""
    start, end, direction, normal, half = band(wall)
    return {"left": {"Start": list(start + normal * half), "End": list(end + normal * half)}, "right": {"Start": list(start - normal * half), "End": list(end - normal * half)}}


def other_end(wall, tip):
    return "End" if tip == "Start" else "Start"


def node_point(wall, tip):
    return wall["start"] if tip == "Start" else wall["end"]


def footprint(wall, ends):
    """🧱️ The footprint `[right.start, right.end, left.end, left.start]` of a wall from its four trimmed face ends."""
    return [ends["right"]["Start"], ends["right"]["End"], ends["left"]["End"], ends["left"]["Start"]]


def joined(case):
    """🔗️ The footprints of the walls of a case: a node of two ends, each end with an authored preference."""
    walls = {wall["id"]: wall for wall in case["walls"]}
    ends = {wall_id: square_ends(wall) for wall_id, wall in walls.items()}
    (first_id, first_tip), (second_id, second_tip) = [(end["wall"], end["tip"]) for end in case["node"]]
    preference = lambda wall_id, tip: walls[wall_id].get("start_join" if tip == "Start" else "end_join")
    prefs = {first_id: preference(first_id, first_tip), second_id: preference(second_id, second_tip)}
    if "None" in prefs.values():
        return {wall_id: footprint(walls[wall_id], ends[wall_id]) for wall_id in walls}
    butts = [wall_id for wall_id, join in prefs.items() if join == "Butt"]
    tips = {first_id: first_tip, second_id: second_tip}
    if butts:
        through = min([wall_id for wall_id in prefs if wall_id not in butts] or sorted(prefs))
        butting = [wall_id for wall_id in prefs if wall_id != through]
        for wall_id in butting:
            wall, anchor = walls[wall_id], walls[through]
            node = numpy.array(node_point(wall, tips[wall_id]))
            _, _, direction_t, normal_t, half_t = band(anchor)
            _, _, direction_b, _, _ = band(wall)
            outward_b = direction_b if tips[wall_id] == "Start" else -direction_b
            near_side = 1 if float(numpy.dot(outward_b, normal_t)) > 0 else -1
            near = face(anchor, near_side)
            for side in (1, -1):
                key = "left" if side == 1 else "right"
                ends[wall_id][key][tips[wall_id]] = cross(face(wall, side), near, node)
        wall, node = walls[through], numpy.array(node_point(walls[through], tips[through]))
        _, _, direction_t, _, _ = band(wall)
        back = -(direction_t if tips[through] == "Start" else -direction_t)
        for side in (1, -1):
            key = "left" if side == 1 else "right"
            base = numpy.array(ends[through][key][tips[through]])
            candidates = []
            for wall_id in butting:
                for far_side in (1, -1):
                    point = cross(face(wall, side), face(walls[wall_id], far_side), node)
                    if point is not None and float(numpy.dot(numpy.array(point) - base, back)) > TOLERANCE and math.dist(point, node) <= 4.0 * wall["thickness"] + 1e-6:
                        candidates.append(point)
            if candidates:
                ends[through][key][tips[through]] = max(candidates, key=lambda point: float(numpy.dot(numpy.array(point) - base, back)))
        return {wall_id: footprint(walls[wall_id], ends[wall_id]) for wall_id in walls}
    forced = "Miter" in prefs.values()
    for wall_id, tip in tips.items():
        other_id = second_id if wall_id == first_id else first_id
        wall, neighbour = walls[wall_id], walls[other_id]
        node = numpy.array(node_point(wall, tip))
        mine, theirs = (1 if tip == "Start" else -1), (1 if tips[other_id] == "Start" else -1)
        limit = 4.0 * max(wall["thickness"], neighbour["thickness"])
        for side, far in ((mine, -theirs), (-mine, theirs)):
            point = cross(face(wall, side), face(neighbour, far), node)
            if point is not None and (forced or math.dist(point, node) <= limit + 1e-6):
                ends[wall_id]["left" if side == 1 else "right"][tip] = point
    return {wall_id: footprint(walls[wall_id], ends[wall_id]) for wall_id in walls}


def joins(cases):
    return [{**case, "footprints": joined(case)} for case in cases]


# endregion 🔖️Joins


# region 🔖️Fixture
def expectations(committed):
    return {
        "description": committed["description"],
        "maps": maps(committed["maps"]),
        "loops": loops(committed["loops"]),
        "offsets": offsets(committed["offsets"]),
        "trims": trims(committed["trims"]),
        "splits": splits(committed["splits"]),
        "arrays": arrays(committed["arrays"]),
        "joins": joins(committed["joins"]),
    }


def close(a, b):
    """⚖️ Equality of two JSON values with a relative tolerance on numbers (arcs are sampled, so their numbers meet at 1e-5)."""
    if isinstance(a, dict) and isinstance(b, dict):
        return a.keys() == b.keys() and all(close(a[key], b[key]) for key in a)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, (int, float)) and isinstance(b, (int, float)) and not isinstance(a, bool):
        return abs(a - b) <= SAMPLED * max(1.0, abs(a), abs(b))
    return a == b


def main(argv):
    sys.stdout.reconfigure(encoding="utf-8")
    if len(argv) != 3 or argv[1] not in ("check", "write"):
        print(__doc__)
        return 2
    path = Path(argv[2])
    committed = json.loads(path.read_text(encoding="utf-8"))
    computed = expectations(committed)
    if argv[1] == "write":
        path.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"wrote {path}")
        return 0
    wrong = [key for key in computed if not close(computed[key], committed[key])]
    if wrong:
        print(f"the committed expectations disagree with the oracle: {', '.join(wrong)}")
        return 1
    print(f"ok: {sum(len(v) if isinstance(v, (list, dict)) else 0 for k, v in committed.items() if k != 'description')} cases agree with numpy {numpy.__version__} and shapely")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
# endregion 🔖️Fixture
