"""Generates the language-agnostic geometry fixtures with closed-form (analytic) expected values.

Run: python r3-g-geometry-fixtures.py  (writes 🧫️fixtures/<domain>/🔣️.json under the geometry module).
Nothing here calls the Rust implementation; every expected number is a textbook formula.
"""
import json
import math
import os

ROOT = r"C:\git\semio\🧰️framework\🔨️modules\📐️geometry\🧫️fixtures"
PI = math.pi
TAU = 2 * PI


def bulge(sweep):
    return math.tan(sweep / 4)


def arc_case(name, start, end, sweep, center, radius):
    mid_angle = math.atan2(start[1] - center[1], start[0] - center[0]) + sweep / 2
    return {
        "name": name,
        "start": start,
        "end": end,
        "bulge": bulge(sweep),
        "expected": {
            "sweep": sweep,
            "radius": radius,
            "center": center,
            "length": radius * abs(sweep),
            "mid": [center[0] + radius * math.cos(mid_angle), center[1] + radius * math.sin(mid_angle)],
            "segment_area": radius * radius / 2 * (sweep - math.sin(sweep)),
        },
    }


def circle_pt(c, r, a):
    return [c[0] + r * math.cos(a), c[1] + r * math.sin(a)]


def write(domain, data):
    folder = os.path.join(ROOT, domain)
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8", newline="\n") as handle:
        json.dump(data, handle, indent=1, ensure_ascii=False)
        handle.write("\n")


def seg(start, end, b=0.0):
    return {"start": start, "end": end, "bulge": b}


s2 = math.sqrt(0.5)
quarter = seg([1, 0], [0, 1], bulge(PI / 2))
line_arcs = [
    arc_case("semicircle below chord", [0, 0], [2, 0], PI, [1, 0], 1),
    arc_case("quarter circle ccw", [1, 0], [0, 1], PI / 2, [0, 0], 1),
    arc_case("quarter circle cw", [0, 1], [1, 0], -PI / 2, [0, 0], 1),
    arc_case("major arc 270 degrees", [1, 0], [0, -1], 3 * PI / 2, [0, 0], 1),
    arc_case("offset centre radius 5", circle_pt([2, 3], 5, PI / 6), circle_pt([2, 3], 5, PI / 6 + 1.0), 1.0, [2, 3], 5),
]
bounds = {
    "semicircle below chord": [0, -1, 2, 0],
    "quarter circle ccw": [0, 0, 1, 1],
    "quarter circle cw": [0, 0, 1, 1],
    "major arc 270 degrees": [-1, -1, 1, 1],
}
for case in line_arcs:
    if case["name"] in bounds:
        case["expected"]["bounds"] = bounds[case["name"]]
line_case = {
    "name": "straight line 3-4-5",
    "start": [0, 0],
    "end": [3, 4],
    "bulge": 0,
    "expected": {"sweep": 0, "radius": None, "center": None, "length": 5, "mid": [1.5, 2], "segment_area": 0, "bounds": [0, 0, 3, 4]},
}
line_arcs.append(line_case)

offsets = [
    {"name": "line left", "seg": seg([0, 0], [4, 0]), "distance": 1, "expected": seg([0, 1], [4, 1])},
    {"name": "line right", "seg": seg([0, 0], [4, 0]), "distance": -1, "expected": seg([0, -1], [4, -1])},
    {"name": "ccw arc left shrinks", "seg": quarter, "distance": 0.25, "expected": seg([0.75, 0], [0, 0.75], bulge(PI / 2))},
    {"name": "ccw arc right grows", "seg": quarter, "distance": -0.25, "expected": seg([1.25, 0], [0, 1.25], bulge(PI / 2))},
    {"name": "cw arc left grows", "seg": seg([0, 1], [1, 0], bulge(-PI / 2)), "distance": 0.25, "expected": seg([0, 1.25], [1.25, 0], bulge(-PI / 2))},
    {"name": "arc collapses", "seg": quarter, "distance": 1, "expected": None},
    {"name": "zero length line", "seg": seg([1, 1], [1, 1]), "distance": 1, "expected": None},
]

h = math.sqrt(0.75)
quarter2 = seg([1, 1], [0, 0], bulge(PI / 2))
intersections = [
    {"name": "crossing lines", "a": seg([0, 0], [4, 4]), "b": seg([0, 4], [4, 0]), "extent": "bounded", "expected": [[2, 2]]},
    {"name": "lines miss bounded", "a": seg([0, 0], [1, 0]), "b": seg([2, -1], [2, 1]), "extent": "bounded", "expected": []},
    {"name": "lines meet unbounded", "a": seg([0, 0], [1, 0]), "b": seg([2, -1], [2, 1]), "extent": "unbounded", "expected": [[2, 0]]},
    {"name": "parallel lines", "a": seg([0, 0], [4, 0]), "b": seg([0, 1], [4, 1]), "extent": "unbounded", "expected": []},
    {"name": "line through quarter arc", "a": seg([0, 0], [2, 2]), "b": quarter, "extent": "bounded", "expected": [[s2, s2]]},
    {"name": "chord hits arc once bounded", "a": seg([-2, 0.5], [2, 0.5]), "b": quarter, "extent": "bounded", "expected": [[h, 0.5]]},
    {"name": "chord hits circle twice unbounded", "a": seg([-2, 0.5], [2, 0.5]), "b": quarter, "extent": "unbounded", "expected": [[-h, 0.5], [h, 0.5]]},
    {"name": "tangent at arc end", "a": seg([-1, 1], [1, 1]), "b": quarter, "extent": "bounded", "expected": [[0, 1]]},
    {"name": "arc arc bounded", "a": quarter, "b": seg([1, 1], [0, 0], bulge(PI / 2)), "extent": "bounded", "expected": [[0.5, h]]},
    {"name": "circle circle unbounded", "a": quarter, "b": seg([1, 1], [0, 0], bulge(PI / 2)), "extent": "unbounded", "expected": [[0.5, -h], [0.5, h]]},
    {"name": "disjoint circles", "a": quarter, "b": seg([11, 0], [10, 1], bulge(PI / 2)), "extent": "unbounded", "expected": []},
]
for case in intersections:
    case["expected"].sort(key=lambda p: (p[0], p[1]))

closest = [
    {"name": "line projects inside", "seg": seg([0, 0], [4, 0]), "point": [2, 3], "expected": {"t": 0.5, "point": [2, 0], "distance": 3}},
    {"name": "line clamps to start", "seg": seg([0, 0], [4, 0]), "point": [-1, 1], "expected": {"t": 0, "point": [0, 0], "distance": math.sqrt(2)}},
    {"name": "arc radial projection", "seg": quarter, "point": [2, 2], "expected": {"t": 0.5, "point": [s2, s2], "distance": 2 * math.sqrt(2) - 1}},
    {"name": "arc clamps to end", "seg": quarter, "point": [-1, -0.5], "expected": {"t": 1, "point": [0, 1], "distance": math.hypot(1, 1.5)}},
    {"name": "arc inside circle", "seg": quarter, "point": [0.2, 0.2], "expected": {"t": 0.5, "point": [s2, s2], "distance": 1 - math.hypot(0.2, 0.2)}},
]

three_points = [
    {"name": "semicircle through top", "a": [1, 0], "through": [0, 1], "b": [-1, 0], "expected": {"center": [0, 0], "radius": 1, "sweep": PI}},
    {"name": "clockwise quarter", "a": [0, 1], "through": [s2, s2], "b": [1, 0], "expected": {"center": [0, 0], "radius": 1, "sweep": -PI / 2}},
    {"name": "collinear", "a": [0, 0], "through": [1, 0], "b": [2, 0], "expected": None},
]

bands = [
    {
        "name": "straight band",
        "axis": seg([0, 0], [4, 0]),
        "left": 0.1,
        "right": 0.2,
        "expected": [[0, -0.2, 0], [4, -0.2, 0], [4, 0.1, 0], [0, 0.1, 0]],
        "area": 4 * 0.3,
    },
    {
        "name": "ccw arc band",
        "axis": seg([2, 0], [0, 2], bulge(PI / 2)),
        "left": 0.1,
        "right": 0.1,
        "expected": [[2.1, 0, bulge(PI / 2)], [0, 2.1, 0], [0, 1.9, -bulge(PI / 2)], [1.9, 0, 0]],
        "area": (2.1**2 - 1.9**2) * PI / 4,
    },
]

corners = [
    {
        "name": "L corner equal widths",
        "prev": seg([0, 0], [4, 0]),
        "prev_widths": [0.1, 0.1],
        "next": seg([4, 0], [4, 4]),
        "next_widths": [0.1, 0.1],
        "expected": {"left": [3.9, 0.1], "right": [4.1, -0.1]},
    },
    {
        "name": "L corner different widths",
        "prev": seg([0, 0], [4, 0]),
        "prev_widths": [0.1, 0.2],
        "next": seg([4, 0], [4, 4]),
        "next_widths": [0.3, 0.1],
        "expected": {"left": [3.7, 0.1], "right": [4.1, -0.2]},
    },
    {
        "name": "collinear continuation",
        "prev": seg([0, 0], [4, 0]),
        "prev_widths": [0.1, 0.1],
        "next": seg([4, 0], [8, 0]),
        "next_widths": [0.1, 0.1],
        "expected": {"left": None, "right": None},
    },
]

write("🌙️bulge", {"arcs": line_arcs, "offsets": offsets, "intersections": intersections, "closest": closest, "three_points": three_points, "bands": bands, "corners": corners})

# ---------------------------------------------------------------- loops
def loop_case(name, vertices, area, perimeter, centroid, bounds, contains=None, ccw=None):
    return {
        "name": name,
        "vertices": vertices,
        "expected": {
            "signed_area": area if ccw is None else (area if ccw else -area),
            "area": abs(area),
            "perimeter": perimeter,
            "centroid": centroid,
            "bounds": bounds,
            "ccw": True if ccw is None else ccw,
        },
        "contains": contains or [],
    }


def v(x, y, b=0.0):
    return [x, y, b]


stadium_area = 2 + PI / 2
stadium_cy = (1 + PI / 2 * (1 + 4 / (3 * PI))) / stadium_area
notch_area = 4 - PI / 2
notch_cy = (4 - PI / 2 * 4 / (3 * PI)) / notch_area
sector_d = 2 * math.sin(3 * PI / 4) / (3 * 3 * PI / 4)
loops = [
    loop_case("unit square ccw", [v(0, 0), v(1, 0), v(1, 1), v(0, 1)], 1, 4, [0.5, 0.5], [0, 0, 1, 1], [{"point": [0.5, 0.5], "at": "inside"}, {"point": [1.5, 0.5], "at": "outside"}, {"point": [1, 0.5], "at": "boundary"}]),
    loop_case("unit square cw", [v(0, 0), v(0, 1), v(1, 1), v(1, 0)], 1, 4, [0.5, 0.5], [0, 0, 1, 1], [{"point": [0.5, 0.5], "at": "inside"}], ccw=False),
    loop_case(
        "rectangle with semicircular top",
        [v(0, 0), v(2, 0), v(2, 1, 1), v(0, 1)],
        stadium_area,
        4 + PI,
        [1, stadium_cy],
        [0, 0, 2, 2],
        [{"point": [1, 1.5], "at": "inside"}, {"point": [1, 2.5], "at": "outside"}, {"point": [1.9, 1.9], "at": "outside"}, {"point": [0.5, 1.5], "at": "inside"}, {"point": [1, 2], "at": "boundary"}],
    ),
    loop_case("circle from two semicircles", [v(1, 0, 1), v(-1, 0, 1)], PI, 2 * PI, [0, 0], [-1, -1, 1, 1], [{"point": [0, 0], "at": "inside"}, {"point": [0.99, 0], "at": "inside"}, {"point": [1.01, 0], "at": "outside"}]),
    loop_case(
        "square with semicircular notch",
        [v(0, 0, -1), v(2, 0), v(2, 2), v(0, 2)],
        notch_area,
        6 + PI,
        [1, notch_cy],
        [0, 0, 2, 2],
        [{"point": [1, 0.5], "at": "outside"}, {"point": [1, 1.5], "at": "inside"}, {"point": [1, 1], "at": "boundary"}, {"point": [1, 1.2], "at": "inside"}],
    ),
    loop_case(
        "270 degree pie",
        [v(1, 0, bulge(3 * PI / 2)), v(0, -1), v(0, 0)],
        3 * PI / 4,
        2 + 3 * PI / 2,
        [-s2 * sector_d, s2 * sector_d],
        [-1, -1, 1, 1],
        [{"point": [-0.5, 0.5], "at": "inside"}, {"point": [0.3, -0.3], "at": "outside"}, {"point": [0.5, -0.5], "at": "outside"}, {"point": [0.5, 0.5], "at": "inside"}, {"point": [-0.9, -0.9], "at": "outside"}],
    ),
    loop_case("L shape", [v(0, 0), v(2, 0), v(2, 1), v(1, 1), v(1, 2), v(0, 2)], 3, 8, [5 / 6, 5 / 6], [0, 0, 2, 2], [{"point": [1.5, 0.5], "at": "inside"}, {"point": [1.5, 1.5], "at": "outside"}, {"point": [0.5, 1.5], "at": "inside"}]),
]

loop_offsets = [
    {"name": "square grows", "vertices": [v(0, 0), v(2, 0), v(2, 2), v(0, 2)], "distance": 0.5, "miter_limit": 4, "expected_area": 9, "expected_perimeter": 12},
    {"name": "square shrinks", "vertices": [v(0, 0), v(2, 0), v(2, 2), v(0, 2)], "distance": -0.5, "miter_limit": 4, "expected_area": 1, "expected_perimeter": 4},
    {"name": "clockwise square grows too", "vertices": [v(0, 0), v(0, 2), v(2, 2), v(2, 0)], "distance": 0.5, "miter_limit": 4, "expected_area": 9, "expected_perimeter": 12},
    {"name": "circle grows", "vertices": [v(1, 0, 1), v(-1, 0, 1)], "distance": 0.5, "miter_limit": 4, "expected_area": PI * 1.5**2, "expected_perimeter": TAU * 1.5},
    {"name": "circle shrinks", "vertices": [v(1, 0, 1), v(-1, 0, 1)], "distance": -0.5, "miter_limit": 4, "expected_area": PI * 0.25, "expected_perimeter": TAU * 0.5},
    {"name": "circle collapses", "vertices": [v(1, 0, 1), v(-1, 0, 1)], "distance": -1, "miter_limit": 4, "expected_area": None, "expected_perimeter": None},
    {"name": "L shape grows with miters", "vertices": [v(0, 0), v(2, 0), v(2, 1), v(1, 1), v(1, 2), v(0, 2)], "distance": 0.25, "miter_limit": 4, "expected_area": 3 + 8 * 0.25 + 4 * 0.25 * 0.25, "expected_perimeter": 8 + 8 * 0.25},
]
write("➰️loops", {"loops": loops, "offsets": loop_offsets})

# ---------------------------------------------------------------- triangulation
def poly(*pts):
    return [list(map(float, p)) for p in pts]


def rect(x0, y0, x1, y1):
    return poly((x0, y0), (x1, y0), (x1, y1), (x0, y1))


triangulation = [
    {"name": "unit square", "outer": rect(0, 0, 1, 1), "holes": [], "area": 1, "triangles": 2},
    {"name": "clockwise square", "outer": rect(0, 0, 1, 1)[::-1], "holes": [], "area": 1, "triangles": 2},
    {"name": "L shape", "outer": poly((0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2)), "holes": [], "area": 3, "triangles": 4},
    {"name": "U shape", "outer": poly((0, 0), (3, 0), (3, 2), (2, 2), (2, 1), (1, 1), (1, 2), (0, 2)), "holes": [], "area": 5, "triangles": 6},
    {"name": "square with square hole", "outer": rect(0, 0, 4, 4), "holes": [rect(1, 1, 3, 3)], "area": 12, "triangles": 8},
    {"name": "square with clockwise hole", "outer": rect(0, 0, 4, 4), "holes": [rect(1, 1, 3, 3)[::-1]], "area": 12, "triangles": 8},
    {"name": "two holes", "outer": rect(0, 0, 10, 6), "holes": [rect(1, 1, 3, 3), rect(5, 2, 8, 5)], "area": 47, "triangles": 14},
    {
        "name": "L shape with two holes",
        "outer": poly((0, 0), (4, 0), (4, 2), (2, 2), (2, 4), (0, 4)),
        "holes": [rect(2.5, 0.5, 3.5, 1.5), rect(0.5, 2.5, 1.5, 3.5)],
        "area": 10,
        "triangles": 16,
    },
    {"name": "hexagon", "outer": [[math.cos(k * TAU / 6), math.sin(k * TAU / 6)] for k in range(6)], "holes": [], "area": 3 * math.sqrt(3) / 2, "triangles": 4},
    {"name": "annulus-like octagons", "outer": [[3 * math.cos(k * TAU / 8), 3 * math.sin(k * TAU / 8)] for k in range(8)], "holes": [[[math.cos(k * TAU / 8), math.sin(k * TAU / 8)] for k in range(8)]], "area": 2 * math.sqrt(2) * (9 - 1), "triangles": 16},
]
write("🔺️triangulation", triangulation)

# ---------------------------------------------------------------- mesh
def box_case(name, outer, holes, bottom, top, volume, area, bounds, centroid=None):
    return {"name": name, "outer": outer, "holes": holes, "bottom": bottom, "top": top, "expected": {"volume": volume, "area": area, "bounds": bounds, "centroid": centroid, "watertight": True}}


flat = lambda z: [0, 0, z]
slope_area_factor = math.sqrt(1 + 0.25)
extrusions = [
    box_case("unit square 2 high", rect(0, 0, 1, 1), [], flat(0), flat(2), 2, 10, [[0, 0, 0], [1, 1, 2]], [0.5, 0.5, 1]),
    box_case("L shape 3 high", poly((0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2)), [], flat(0), flat(3), 9, 30, [[0, 0, 0], [2, 2, 3]], [5 / 6, 5 / 6, 1.5]),
    box_case("slab with opening", rect(0, 0, 4, 4), [rect(1, 1, 3, 3)], flat(3), flat(3.2), 2.4, 28.8, [[0, 0, 3], [4, 4, 3.2]], [2, 2, 3.1]),
    box_case("clockwise input same solid", rect(0, 0, 1, 1)[::-1], [], flat(0), flat(2), 2, 10, [[0, 0, 0], [1, 1, 2]], [0.5, 0.5, 1]),
    box_case("sloped slab", rect(0, 0, 2, 2), [], [0.5, 0, 0], [0.5, 0, 0.2], 0.8, 2 * 4 * slope_area_factor + 1.6, [[0, 0, 0], [2, 2, 1.2]]),
]
extrusions[-1]["expected"]["centroid"] = [1, 1, 0.5 + 0.1]
loop_extrusions = [
    {"name": "cylinder from circle loop", "outer": [v(1, 0, 1), v(-1, 0, 1)], "holes": [], "tolerance": 1e-4, "bottom": flat(0), "top": flat(1), "volume": [PI - 1e-3, PI], "area": [4 * PI - 5e-3, 4 * PI], "watertight": True},
]
prisms = [
    {"name": "unit cube as prism", "lower": [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], "upper": [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]], "volume": 1, "area": 6},
    {"name": "frustum", "lower": [[0, 0, 0], [2, 0, 0], [2, 2, 0], [0, 2, 0]], "upper": [[0.5, 0.5, 3], [1.5, 0.5, 3], [1.5, 1.5, 3], [0.5, 1.5, 3]], "volume": 7, "area": 5 + 4 * 1.5 * math.sqrt(9.25)},
    {"name": "clockwise lower ring", "lower": [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]], "upper": [[0, 0, 1], [0, 1, 1], [1, 1, 1], [1, 0, 1]], "volume": 1, "area": 6},
]
profile = rect(-0.1, 0, 0.1, 0.4)
sweeps = [
    {"name": "beam along a line", "profile": profile, "path": [seg([0, 0], [5, 0])], "base_z": 0, "tolerance": 1e-3, "volume": [0.4, 0.4], "area": [6.16, 6.16], "bounds": [[0, -0.1, 0], [5, 0.1, 0.4]]},
    {"name": "beam raised on a line", "profile": profile, "path": [seg([0, 0], [5, 0])], "base_z": 2.5, "tolerance": 1e-3, "volume": [0.4, 0.4], "area": [6.16, 6.16], "bounds": [[0, -0.1, 2.5], [5, 0.1, 2.9]]},
    {"name": "beam along a quarter arc", "profile": profile, "path": [seg([2, 0], [0, 2], bulge(PI / 2))], "base_z": 0, "tolerance": 1e-5, "volume": [0.08 * PI * (1 - 1e-4), 0.08 * PI], "area": None, "bounds": [[0, 0, 0], [2.1, 2.1, 0.4]]},
    {"name": "mitered corner", "profile": rect(-0.1, 0, 0.1, 0.2), "path": [seg([0, 0], [2, 0]), seg([2, 0], [2, 2])], "base_z": 0, "tolerance": 1e-3, "volume": [0.16, 0.16], "area": None, "bounds": [[0, -0.1, 0], [2.1, 2, 0.2]]},
    {"name": "hollow tube", "profile": rect(-0.2, 0, 0.2, 0.4), "holes": [rect(-0.1, 0.1, 0.1, 0.3)], "path": [seg([0, 0], [3, 0])], "base_z": 0, "tolerance": 1e-3, "volume": [3 * (0.16 - 0.04), 3 * (0.16 - 0.04)], "area": None, "bounds": [[0, -0.2, 0], [3, 0.2, 0.4]]},
]
door_outline = poly((0, 0), (1, 0), (1, 2.1), (1.9, 2.1), (1.9, 0), (5, 0), (5, 2.5), (0, 2.5))
window = poly((3, 1), (3, 2), (4.2, 2), (4.2, 1))
arc_len = 5 * PI / 2
arc_outline = poly((0, 0), (arc_len, 0), (arc_len, 3), (0, 3))
arc_window = poly((3, 1), (3, 2), (4, 2), (4, 1))
arc_volume = 1.5 * PI - 0.2
walls = [
    {"name": "straight wall with door and window", "axis": seg([0, 0], [5, 0]), "left": 0.1, "right": 0.1, "axis_length": 5, "base_z": 0, "tolerance": 1e-3, "left_face": {"outer": door_outline, "holes": [window]}, "right_face": {"outer": door_outline, "holes": [window]}, "volume": [1.882, 1.882], "area": [23.54, 23.54], "watertight": True},
    {"name": "wall with slanted ends", "axis": seg([0, 0], [5, 0]), "left": 0.1, "right": 0.1, "axis_length": 5, "base_z": 0, "tolerance": 1e-3, "left_face": {"outer": rect(0, 0, 5, 2), "holes": []}, "right_face": {"outer": poly((0.2, 0), (4.8, 0), (4.8, 2), (0.2, 2)), "holes": []}, "volume": [1.92, 1.92], "area": None, "watertight": True},
    {"name": "curved wall with window", "axis": seg([5, 0], [0, 5], bulge(PI / 2)), "left": 0.1, "right": 0.1, "axis_length": arc_len, "base_z": 0, "tolerance": 1e-5, "left_face": {"outer": arc_outline, "holes": [arc_window]}, "right_face": {"outer": arc_outline, "holes": [arc_window]}, "volume": [arc_volume * (1 - 2e-4), arc_volume], "area": None, "watertight": True},
]
write("🕸️mesh", {"extrusions": extrusions, "loop_extrusions": loop_extrusions, "prisms": prisms, "sweeps": sweeps, "walls": walls})

# ---------------------------------------------------------------- section
sections = [
    {"name": "unit square column mid height", "outer": rect(0, 0, 1, 1), "holes": [], "bottom": 0, "top": 2, "z": 1, "expected": [{"closed": True, "area": 1, "points": 4}]},
    {"name": "cut above the solid", "outer": rect(0, 0, 1, 1), "holes": [], "bottom": 0, "top": 2, "z": 3, "expected": []},
    {"name": "slab with opening", "outer": rect(0, 0, 4, 4), "holes": [rect(1, 1, 3, 3)], "bottom": 0, "top": 0.2, "z": 0.1, "expected": [{"closed": True, "area": 16, "points": 4}, {"closed": True, "area": -4, "points": 4}]},
    {"name": "L shape wall", "outer": poly((0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2)), "holes": [], "bottom": 0, "top": 3, "z": 1.5, "expected": [{"closed": True, "area": 3, "points": 6}]},
    {"name": "cut exactly at the top face", "outer": rect(0, 0, 1, 1), "holes": [], "bottom": 0, "top": 2, "z": 2, "expected": [{"closed": True, "area": 1, "points": 4}]},
]
write("🔪️section", sections)

# ---------------------------------------------------------------- placement
c90, s90 = 0, 1
placements = [
    {"name": "rotate quarter turn", "ops": [{"rotation_z": PI / 2}], "point": [1, 0, 0], "expected": [0, 1, 0]},
    {"name": "translate then rotate", "ops": [{"translation": [1, 0, 0]}, {"rotation_z": PI / 2}], "point": [0, 0, 0], "expected": [0, 1, 0]},
    {"name": "rotate then translate", "ops": [{"rotation_z": PI / 2}, {"translation": [1, 0, 0]}], "point": [1, 0, 0], "expected": [1, 1, 0]},
    {"name": "axis rotation about x", "ops": [{"rotation_axis": [1, 0, 0], "angle": PI / 2}], "point": [0, 1, 0], "expected": [0, 0, 1]},
    {"name": "mirror x", "ops": [{"scaling": [-1, 1, 1]}], "point": [2, 3, 4], "expected": [-2, 3, 4]},
]
write("🧭️placement", {"placements": placements, "z_planes": [{"name": "sloped plane", "anchor": [1, 1], "z": 3, "direction": 0, "slope": 0.5, "queries": [{"point": [1, 1], "z": 3}, {"point": [3, 1], "z": 4}, {"point": [1, 5], "z": 3}]}]})

# ---------------------------------------------------------------- 2d regions (semio-framework-2d)
ROOT2D = r"C:\git\semio\🧰️framework\🔨️modules\◻️2d\🧱️regions\🧫️fixtures"
os.makedirs(ROOT2D, exist_ok=True)
sq4 = rect(0, 0, 4, 4)
sq6 = rect(0, 0, 6, 6)
ell = poly((0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2))
regions2d = {
    "offsets": [
        {"name": "square grows with miter", "outer": sq4, "holes": [], "distance": 1, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 36, "tolerance": 1e-9}},
        {"name": "square shrinks with miter", "outer": sq4, "holes": [], "distance": -1, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 4, "tolerance": 1e-9}},
        {"name": "square grows with bevel", "outer": sq4, "holes": [], "distance": 1, "join": {"kind": "bevel"}, "expected": {"regions": 1, "holes": 0, "area": 34, "tolerance": 1e-9}},
        {"name": "square grows with round corners", "outer": sq4, "holes": [], "distance": 1, "join": {"kind": "round", "tolerance": 1e-4}, "expected": {"regions": 1, "holes": 0, "area": 32 + PI, "tolerance": 2e-3}},
        {"name": "square collapses", "outer": sq4, "holes": [], "distance": -2, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 0, "holes": 0, "area": 0, "tolerance": 1e-9}},
        {"name": "square vanishes", "outer": sq4, "holes": [], "distance": -3, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 0, "holes": 0, "area": 0, "tolerance": 1e-9}},
        {"name": "hole shrinks when the region grows", "outer": sq6, "holes": [rect(2, 2, 4, 4)], "distance": 0.5, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 1, "area": 48, "tolerance": 1e-9}},
        {"name": "hole grows when the region shrinks", "outer": sq6, "holes": [rect(2, 2, 4, 4)], "distance": -0.5, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 1, "area": 16, "tolerance": 1e-9}},
        {"name": "hole closes when the region grows enough", "outer": sq6, "holes": [rect(2, 2, 4, 4)], "distance": 1, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 64, "tolerance": 1e-9}},
        {"name": "L shape grows", "outer": ell, "holes": [], "distance": 0.25, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 5.25, "tolerance": 1e-9}},
        {"name": "L shape shrinks", "outer": ell, "holes": [], "distance": -0.25, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 1.25, "tolerance": 1e-9}},
        {"name": "clockwise input is normalised", "outer": sq4[::-1], "holes": [], "distance": 1, "join": {"kind": "miter", "limit": 4}, "expected": {"regions": 1, "holes": 0, "area": 36, "tolerance": 1e-9}},
    ],
    "booleans": [
        {"name": "overlapping squares union", "operation": "union", "subject": [rect(0, 0, 2, 2)], "clip": [rect(1, 1, 3, 3)], "expected": {"regions": 1, "holes": 0, "area": 7}},
        {"name": "overlapping squares intersection", "operation": "intersection", "subject": [rect(0, 0, 2, 2)], "clip": [rect(1, 1, 3, 3)], "expected": {"regions": 1, "holes": 0, "area": 1}},
        {"name": "overlapping squares difference", "operation": "difference", "subject": [rect(0, 0, 2, 2)], "clip": [rect(1, 1, 3, 3)], "expected": {"regions": 1, "holes": 0, "area": 3}},
        {"name": "overlapping squares xor", "operation": "xor", "subject": [rect(0, 0, 2, 2)], "clip": [rect(1, 1, 3, 3)], "expected": {"regions": 2, "holes": 0, "area": 6}},
        {"name": "contained clip punches a hole", "operation": "difference", "subject": [sq4], "clip": [rect(1, 1, 3, 3)], "expected": {"regions": 1, "holes": 1, "area": 12}},
        {"name": "disjoint union keeps both", "operation": "union", "subject": [rect(0, 0, 1, 1)], "clip": [rect(5, 5, 6, 6)], "expected": {"regions": 2, "holes": 0, "area": 2}},
        {"name": "disjoint intersection is empty", "operation": "intersection", "subject": [rect(0, 0, 1, 1)], "clip": [rect(5, 5, 6, 6)], "expected": {"regions": 0, "holes": 0, "area": 0}},
        {"name": "wall footprints merge at a T", "operation": "union", "subject": [rect(0, 0, 5, 0.2), rect(2.4, 0.2, 2.6, 3)], "clip": [], "expected": {"regions": 1, "holes": 0, "area": 1 + 0.2 * 2.8}},
    ],
}
with open(os.path.join(ROOT2D, "🔣️.json"), "w", encoding="utf-8", newline="\n") as handle:
    json.dump(regions2d, handle, indent=1, ensure_ascii=False)
    handle.write("\n")
print("fixtures written")
