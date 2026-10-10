#!/usr/bin/env python3
"""🛠️ Third-party ORACLE for the `s.bim.model@1` authoring tools (`✏️editor/🧵️gestures`).

The subject (Rust, `semio-s-artifact-bim-model`) turns pointer events into model mutations. The numbers behind those mutations are plain
plane geometry: the bulge of the arc through three clicks, the arc length of the foot of a pointer on a wall axis and the side it is on,
the rectangle of two opposite corners, a ring made counter-clockwise, the centre offset of an opening kept inside its host, the storey
height a dragged top line sets and the turn between two directions, the point a typed line names (keyboard entry: absolute, relative, polar, bare length) and the point the arrow-key cursor stands on after a run of fine and coarse steps. This file recomputes each of them from the SAME committed cases with
libraries that have never seen this repository (`numpy` for the circumcircle and angles, `shapely` 2 / GEOS for projection, boxes,
orientation and areas) and either writes the expectations (`write`) or audits the committed ones (`check`). The Rust subject replays the
cases through its tools and compares against the same JSON, so subject and oracle meet on one fixture.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🛠️gestures/🔣️.json>
    python 🐍️.py write <path to 🧫️fixtures/🛠️gestures/🔣️.json>

@see ../../🧫️fixtures/🛠️gestures/🔣️.json — the cases and their expectations
@see https://shapely.readthedocs.io/en/stable/manual.html#shapely.LineString.project
"""

import json
import math
import re
import sys
from pathlib import Path

import numpy
from shapely.geometry import LineString, Point, Polygon, box
from shapely.geometry.polygon import orient

TOLERANCE = 1e-9
"""⚖️ The subject computes in f64 with closed forms; everything here is exact up to rounding."""

HEIGHT_STEP = 0.05
HEIGHT_MINIMUM = 0.5


def bulge_through(start, through, end):
    """🌙️ tan(sweep / 4) of the arc from start to end through `through`, sweep positive counter-clockwise (circumcircle by linear algebra)."""
    a, b, c = (numpy.array(p, dtype=float) for p in (start, through, end))
    matrix = 2 * numpy.array([b - a, c - a])
    right = numpy.array([b @ b - a @ a, c @ c - a @ a])
    centre = numpy.linalg.solve(matrix, right)
    turn = (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])
    direction = 1.0 if turn > 0 else -1.0
    first = math.atan2(a[1] - centre[1], a[0] - centre[0])
    last = math.atan2(c[1] - centre[1], c[0] - centre[0])
    sweep = ((last - first) * direction) % (2 * math.pi) * direction
    return math.tan(sweep / 4)


def projection(axis, point):
    """📐️ Arc length of the foot of `point` on the axis, its distance and its side (positive left of travel), measured by GEOS."""
    line = LineString([axis["start"], axis["end"]])
    spot = Point(point)
    foot = line.interpolate(line.project(spot))
    tangent = (axis["end"][0] - axis["start"][0], axis["end"][1] - axis["start"][1])
    cross = tangent[0] * (point[1] - foot.y) - tangent[1] * (point[0] - foot.x)
    return line.project(spot), line.distance(spot), (cross > 0) - (cross < 0)


def rectangle(a, b):
    """🔲️ The counter-clockwise ring and area of the box spanned by two opposite corners."""
    shape = box(min(a[0], b[0]), min(a[1], b[1]), max(a[0], b[0]), max(a[1], b[1]))
    ring = [list(p) for p in shape.exterior.coords][:-1]
    return ring, shape.area


def counter_clockwise(ring):
    """🧭️ The ring running counter-clockwise (reversed when it ran clockwise) and its area."""
    shape = orient(Polygon(ring), 1.0)
    return [list(p) for p in shape.exterior.coords][:-1], shape.area


def window_offset(length, width, foot):
    """🪟️ The centre offset of an opening of `width` kept inside a host of `length`; none when the host is too short."""
    if length < width:
        return None
    return min(max(foot, width / 2), length - width / 2)


def storey_height(elevation, pointer):
    """📏️ The storey height a dragged top line sets: pointer snapped to the step, never below the minimum."""
    return max(round((pointer - elevation) / HEIGHT_STEP) * HEIGHT_STEP, HEIGHT_MINIMUM)


def rotation(pivot, reference, target):
    """🔄️ The turn between two directions seen from the pivot, in [-pi, pi)."""
    a = math.atan2(reference[1] - pivot[1], reference[0] - pivot[0])
    b = math.atan2(target[1] - pivot[1], target[0] - pivot[0])
    return (b - a + math.pi) % (2 * math.pi) - math.pi


def typed_point(entry, anchor, toward):
    """⌨️ The point a typed line names: `x, y` absolute, `@dx, dy` and `length<angle` from the anchor (the origin without one), a bare length towards `toward` (+X without one); numpy does the trigonometry."""
    origin = numpy.array(anchor if anchor is not None else [0.0, 0.0], dtype=float)
    text = entry.strip()
    relative = text.startswith("@")
    body = text[1:].strip() if relative else text
    if "<" in body:
        length, degrees = (float(part.strip().rstrip("°").rstrip("m")) for part in body.split("<", 1))
        radians = numpy.radians(degrees)
        point = origin + length * numpy.array([numpy.cos(radians), numpy.sin(radians)])
    else:
        parts = re.split(r"[,;]", body) if re.search(r"[,;]", body) else body.split()
        if len(parts) == 2:
            offset = numpy.array([float(part) for part in parts])
            point = origin + offset if relative else offset
        else:
            length = float(body.rstrip("m"))
            direction = numpy.array(toward, dtype=float) - origin if toward is not None else numpy.array([1.0, 0.0])
            if numpy.linalg.norm(direction) <= 1e-6:
                direction = numpy.array([1.0, 0.0])
            point = origin + length * direction / numpy.linalg.norm(direction)
    return [float(round(value, 9)) + 0.0 for value in point]


CURSOR_STEPS = {"left": (-1.0, 0.0), "right": (1.0, 0.0), "up": (0.0, 1.0), "down": (0.0, -1.0)}
"""⌨️ The arrow keys of the keyboard cursor as unit directions: a fine step is 0.1 m, a `_far` step 1 m."""


def cursor_point(start, steps):
    """⌨️ Where the keyboard cursor stands after the arrow keys `steps` from `start`: numpy sums the steps (0.1 m, or 1 m for `<direction>_far`) onto the start."""
    point = numpy.array(start, dtype=float)
    for step in steps:
        name, _, far = step.partition("_")
        point = point + (1.0 if far == "far" else 0.1) * numpy.array(CURSOR_STEPS[name])
    return [float(round(value, 9)) + 0.0 for value in point]


def component_fit(axis, thickness, point):
    """🪑️ Where a component mounted on the wall `axis` of `thickness` stands for the plan point `point`: GEOS projects the point onto the axis, numpy puts the origin on the face of the side the point lies on
    (the left face for a point on the axis) and gives the unit vector of the local depth axis, which points away from the wall along the outward normal."""
    line = LineString([axis["start"], axis["end"]])
    spot = Point(point)
    foot = numpy.array(line.interpolate(line.project(spot)).coords[0])
    tangent = numpy.array(axis["end"], dtype=float) - numpy.array(axis["start"], dtype=float)
    tangent = tangent / numpy.linalg.norm(tangent)
    left = numpy.array([-tangent[1], tangent[0]])
    offset = numpy.array(point, dtype=float) - foot
    side = float(numpy.sign(tangent[0] * offset[1] - tangent[1] * offset[0])) or 1.0
    depth = left * side
    origin = foot + depth * thickness / 2
    return [float(round(value, 9)) + 0.0 for value in origin], [float(round(value, 9)) + 0.0 for value in depth]


def component_frame(origin, yaw_degrees, mirrored, local):
    """🧭️ The plan points of family-frame points: numpy mirrors local x, turns by the yaw with a rotation matrix and adds the origin."""
    radians = numpy.radians(yaw_degrees)
    rotation_matrix = numpy.array([[numpy.cos(radians), -numpy.sin(radians)], [numpy.sin(radians), numpy.cos(radians)]])
    mirror = numpy.diag([-1.0 if mirrored else 1.0, 1.0])
    return [[float(round(value, 9)) + 0.0 for value in rotation_matrix @ mirror @ numpy.array(p, dtype=float) + numpy.array(origin, dtype=float)] for p in local]


def route_length(points):
    """🌀️ The length of a polyline in space: numpy sums the norms of its segments."""
    array = numpy.array(points, dtype=float)
    return float(round(numpy.linalg.norm(numpy.diff(array, axis=0), axis=1).sum(), 9))


def turned(steps):
    """🔄️ The rotation in degrees in [0, 360) after a run of turn keys (degrees counter-clockwise, negative clockwise)."""
    return float(round(numpy.mod(numpy.sum(numpy.array(steps, dtype=float)), 360.0), 9)) + 0.0


def expectations(cases):
    """🧮️ The committed cases with every expectation recomputed."""
    out = dict(cases)
    out["component_fits"] = [{**c, **dict(zip(("origin", "depth"), component_fit(c["axis"], c["thickness"], c["point"])))} for c in cases["component_fits"]]
    out["component_frames"] = [{**c, "world": component_frame(c["origin"], c["yaw"], c["mirrored"], c["local"])} for c in cases["component_frames"]]
    out["routes"] = [{**c, "length": route_length(c["points"])} for c in cases["routes"]]
    out["turns"] = [{**c, "degrees": turned(c["steps"])} for c in cases["turns"]]
    out["arcs"] = [{**c, "bulge": bulge_through(c["start"], c["through"], c["end"])} for c in cases["arcs"]]
    out["projections"] = [
        {**c, **dict(zip(("offset", "distance", "side"), projection(c["axis"], c["point"])))} for c in cases["projections"]
    ]
    out["rectangles"] = [{**c, **dict(zip(("ring", "area"), rectangle(c["a"], c["b"])))} for c in cases["rectangles"]]
    out["counter_clockwise"] = [{**c, **dict(zip(("expected", "area"), counter_clockwise(c["ring"])))} for c in cases["counter_clockwise"]]
    out["windows"] = [{**c, "offset": window_offset(c["length"], c["width"], c["foot"])} for c in cases["windows"]]
    out["heights"] = [{**c, "height": storey_height(c["elevation"], c["pointer"])} for c in cases["heights"]]
    out["rotations"] = [{**c, "angle": rotation(c["pivot"], c["reference"], c["target"])} for c in cases["rotations"]]
    out["typed"] = [{**c, "point": typed_point(c["entry"], c.get("anchor"), c.get("toward"))} for c in cases["typed"]]
    out["cursor"] = [{**c, "point": cursor_point(c["start"], c["steps"])} for c in cases["cursor"]]
    return out


def close(left, right):
    """⚖️ Structural equality with the oracle tolerance on numbers."""
    if isinstance(left, (int, float)) and isinstance(right, (int, float)) and not isinstance(left, bool):
        return abs(left - right) <= TOLERANCE * max(1.0, abs(right))
    if isinstance(left, list) and isinstance(right, list):
        return len(left) == len(right) and all(close(x, y) for x, y in zip(left, right))
    if isinstance(left, dict) and isinstance(right, dict):
        return left.keys() == right.keys() and all(close(left[k], right[k]) for k in left)
    return left == right


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
    print(f"ok: {sum(len(v) for v in committed.values() if isinstance(v, list))} cases agree with numpy {numpy.__version__} and shapely")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
