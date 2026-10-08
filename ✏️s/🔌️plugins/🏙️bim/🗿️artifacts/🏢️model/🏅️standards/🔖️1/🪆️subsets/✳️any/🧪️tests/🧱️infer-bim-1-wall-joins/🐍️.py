#!/usr/bin/env python3
"""🧱️ Third-party ORACLE (shapely) for the plan geometry of `🧱️wall-layout`: location lines, layer offsets, wall joins and footprints.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per wall, the face curves offset from the axis by the location line and
the layers of the wall type, and trims them at the joins to its neighbours on the same storey. This file reproduces that table from the
SAME committed snapshot in two independent routes and compares them:

* an analytic route in plain Python (`math` only): offset curves of lines and arcs, infinite-carrier intersections, the mitre ring around
  an axis-end node, T butts against the near face, X crossings, exact loop areas with circular-segment terms;
* a `shapely` 2 (GEOS) route that never sees the analytic code: every straight-wall corner is the intersection of two `offset_curve`
  carriers computed by GEOS, each footprint is a real `Polygon`, and the areas, lengths, validity and the join invariants below are
  measured by GEOS. Arcs are sampled into 4096 chords (GEOS has no true arcs) and compared at a relative 1e-5.

Join invariants measured with shapely (they hold for ANY correct implementation, they are not copied from the subject):

* the footprints of two walls joined by a Miter or a Butt never overlap (intersection area 0), and a Cross overlaps by exactly the
  `overlap_area` the table commits;
* a node of two walls with the same thickness and location Center has the union area of `LineString(chain).buffer(t / 2,
  join_style="mitre")`;
* a stem butting a wall equals its untrimmed band cut down by the other wall's body (`difference`).

Conventions (see `🧱️wall-layout/🦀️.rs`): layers run from the left (interior) face to the right (exterior) face along the axis; `Interior`
puts the axis on the left face, `Exterior` on the right face, `CoreCenter` on the centre of the Core (else Structure) layers, `Center` on
the mid plane. Ends within 1e-6 m form a node mitered pairwise around it, an end on another axis butts against the near face, two axes
crossing in both interiors form a Cross that trims nothing.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../🪜️infer-bim-1-levels-and-wall-heights/🐍️.py — levels, heights and the table writer this file feeds
"""

# region 🔖️Imports
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import LineString, Point, Polygon
from shapely.ops import unary_union

# endregion 🔖️Imports


# region 🔖️Vocabulary
JOIN_TOLERANCE = 1e-6
"""📏️ Distance within which two axis ends coincide, or an axis end lies on another axis."""

MITER_LIMIT = 4.0
"""📐️ A corner whose mitre point lies farther than this many thicknesses from the node is cut square."""

LENGTH_EPS = 1e-9
"""📏️ Tangency and zero-length tolerance of the planar kernel."""

ARC_SEGMENTS = 4096
"""📐️ Chords an arc is sampled into before shapely measures it."""

EXACT = 1e-9
SAMPLED = 1e-5
"""⚖️ Absolute tolerance of exact arithmetic and relative tolerance where shapely samples an arc."""

PLAN_FIELDS = ("thickness", "length", "offset_left", "offset_right", "left_length", "right_length", "footprint_area")
"""🧱️ The scalar plan fields of a wall layout that do not depend on the height."""


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


def xy(point):
    """📍️ A `Point2` as an `(x, y)` pair."""
    return (point["x"], point["y"])


def mark(point):
    """📍️ An `(x, y)` pair as a `Point2`."""
    return {"x": point[0], "y": point[1]}


# endregion 🔖️Vocabulary


# region 🔖️Curves
class Curve:
    """〰️ A line (`bulge == 0`) or a circular arc from `start` to `end` with `bulge = tan(sweep / 4)`; positive bulge is counter-clockwise."""

    def __init__(self, start, end, bulge):
        self.start, self.end, self.bulge = start, end, bulge

    @property
    def is_line(self):
        return abs(self.bulge) <= 1e-12

    @property
    def sweep(self):
        return 4.0 * math.atan(self.bulge)

    @property
    def chord(self):
        return math.dist(self.start, self.end)

    @property
    def radius(self):
        return math.inf if self.is_line else self.chord / (2.0 * math.sin(abs(self.sweep) / 2.0))

    @property
    def center(self):
        if self.is_line or self.chord <= LENGTH_EPS:
            return None
        chord = self.chord
        direction = ((self.end[0] - self.start[0]) / chord, (self.end[1] - self.start[1]) / chord)
        offset = chord / 2.0 * (1.0 - self.bulge ** 2) / (2.0 * self.bulge)
        return ((self.start[0] + self.end[0]) / 2.0 - direction[1] * offset, (self.start[1] + self.end[1]) / 2.0 + direction[0] * offset)

    @property
    def length(self):
        return self.chord if self.is_line else self.radius * abs(self.sweep)

    def start_angle(self):
        center = self.center
        return math.atan2(self.start[1] - center[1], self.start[0] - center[0])

    def point_at(self, t):
        center = self.center
        if center is None:
            return (self.start[0] + (self.end[0] - self.start[0]) * t, self.start[1] + (self.end[1] - self.start[1]) * t)
        angle = self.start_angle() + self.sweep * t
        return (center[0] + self.radius * math.cos(angle), center[1] + self.radius * math.sin(angle))

    def tangent_at(self, t):
        center = self.center
        if center is None:
            length = self.chord
            return ((self.end[0] - self.start[0]) / length, (self.end[1] - self.start[1]) / length)
        angle = self.start_angle() + self.sweep * t
        direction = math.copysign(1.0, self.sweep)
        return (-math.sin(angle) * direction, math.cos(angle) * direction)

    def param_of(self, point):
        center = self.center
        if center is None:
            d = (self.end[0] - self.start[0], self.end[1] - self.start[1])
            return ((point[0] - self.start[0]) * d[0] + (point[1] - self.start[1]) * d[1]) / (d[0] * d[0] + d[1] * d[1])
        first = (self.start[0] - center[0], self.start[1] - center[1])
        second = (point[0] - center[0], point[1] - center[1])
        angle = math.atan2(first[0] * second[1] - first[1] * second[0], first[0] * second[0] + first[1] * second[1])
        angle = (angle * math.copysign(1.0, self.sweep)) % (2.0 * math.pi)
        if angle > 2.0 * math.pi - 1e-9:
            angle -= 2.0 * math.pi
        return angle / abs(self.sweep)

    def closest_distance(self, point):
        """🎯️ Distance from `point` to the segment, endpoints included."""
        center = self.center
        if center is None:
            t = min(max(self.param_of(point), 0.0), 1.0)
            return math.dist(point, self.point_at(t))
        t = self.param_of(point)
        v = (point[0] - center[0], point[1] - center[1])
        if math.hypot(*v) > LENGTH_EPS and t <= 1.0:
            return abs(math.hypot(*v) - self.radius)
        return min(math.dist(point, self.start), math.dist(point, self.end))

    def closest_param(self, point):
        center = self.center
        if center is None:
            return min(max(self.param_of(point), 0.0), 1.0)
        t = self.param_of(point)
        v = (point[0] - center[0], point[1] - center[1])
        if math.hypot(*v) > LENGTH_EPS and t <= 1.0:
            return max(t, 0.0)
        return 0.0 if math.dist(point, self.start) <= math.dist(point, self.end) else 1.0

    def offset(self, distance):
        """↔️ Parallel curve at `distance` to the left of the direction of travel; `None` when an arc collapses."""
        center = self.center
        if center is None:
            length = self.chord
            if length <= LENGTH_EPS:
                return None
            shift = (-(self.end[1] - self.start[1]) / length * distance, (self.end[0] - self.start[0]) / length * distance)
            return Curve((self.start[0] + shift[0], self.start[1] + shift[1]), (self.end[0] + shift[0], self.end[1] + shift[1]), self.bulge)
        radius = self.radius - distance * math.copysign(1.0, self.sweep)
        if radius <= LENGTH_EPS:
            return None
        scale = radius / self.radius
        scaled = lambda p: (center[0] + (p[0] - center[0]) * scale, center[1] + (p[1] - center[1]) * scale)
        return Curve(scaled(self.start), scaled(self.end), self.bulge)

    def retarget(self, start, end):
        """🧷️ The same carrier with new end points; the sweep is re-measured in the direction of travel."""
        center = self.center
        if center is None:
            return Curve(start, end, 0.0)
        direction = math.copysign(1.0, self.sweep)
        first = (start[0] - center[0], start[1] - center[1])
        second = (end[0] - center[0], end[1] - center[1])
        angle = (math.atan2(first[0] * second[1] - first[1] * second[0], first[0] * second[0] + first[1] * second[1]) * direction) % (2.0 * math.pi)
        if angle <= 1e-12 and math.dist(start, end) > LENGTH_EPS:
            angle = 2.0 * math.pi
        return Curve(start, end, math.tan(angle * direction / 4.0))

    def reversed(self):
        return Curve(self.end, self.start, -self.bulge)

    def sampled(self, chords=ARC_SEGMENTS):
        """〰️ The curve as a polyline (a line is its two end points)."""
        count = 1 if self.is_line else chords
        return [self.point_at(step / count) for step in range(count + 1)]


def carrier_intersections(a, b):
    """✂️ Points where the infinite carriers (full line or full circle) of two curves meet; parallel and concentric carriers give none."""
    ca, cb = a.center, b.center
    if ca is None and cb is None:
        da = (a.end[0] - a.start[0], a.end[1] - a.start[1])
        db = (b.end[0] - b.start[0], b.end[1] - b.start[1])
        denominator = da[0] * db[1] - da[1] * db[0]
        if abs(denominator) <= 1e-12 * math.hypot(*da) * math.hypot(*db):
            return []
        t = ((b.start[0] - a.start[0]) * db[1] - (b.start[1] - a.start[1]) * db[0]) / denominator
        return [(a.start[0] + da[0] * t, a.start[1] + da[1] * t)]
    if ca is None or cb is None:
        line, circle = (a, b) if ca is None else (b, a)
        center, radius = circle.center, circle.radius
        d = (line.end[0] - line.start[0], line.end[1] - line.start[1])
        l2 = d[0] * d[0] + d[1] * d[1]
        t0 = ((center[0] - line.start[0]) * d[0] + (center[1] - line.start[1]) * d[1]) / l2
        foot = (line.start[0] + d[0] * t0, line.start[1] + d[1] * t0)
        distance = math.dist(foot, center)
        if distance > radius + LENGTH_EPS:
            return []
        if distance >= radius - LENGTH_EPS:
            return [foot]
        half = math.sqrt(radius * radius - distance * distance) / math.sqrt(l2)
        return [(foot[0] - d[0] * half, foot[1] - d[1] * half), (foot[0] + d[0] * half, foot[1] + d[1] * half)]
    ra, rb = a.radius, b.radius
    d = math.dist(ca, cb)
    if d <= LENGTH_EPS or d > ra + rb + LENGTH_EPS or d < abs(ra - rb) - LENGTH_EPS:
        return []
    along = (ra * ra - rb * rb + d * d) / (2.0 * d)
    h2 = ra * ra - along * along
    axis = ((cb[0] - ca[0]) / d, (cb[1] - ca[1]) / d)
    base = (ca[0] + axis[0] * along, ca[1] + axis[1] * along)
    if math.sqrt(max(h2, 0.0)) <= LENGTH_EPS:
        return [base]
    h = math.sqrt(h2)
    return [(base[0] - axis[1] * h, base[1] + axis[0] * h), (base[0] + axis[1] * h, base[1] - axis[0] * h)]


def nearest(points, near):
    """🎯️ The point closest to `near`, or `None`."""
    return min(points, key=lambda p: math.dist(p, near)) if points else None


def bounded_hits(a, b):
    """✂️ Intersections of two segments (not carriers) with the parameters on each, sorted along `a`."""
    slack = lambda c: LENGTH_EPS / max(c.length, LENGTH_EPS)
    hits = []
    for point in carrier_intersections(a, b):
        ta, tb = a.param_of(point), b.param_of(point)
        if -slack(a) <= ta <= 1.0 + slack(a) and -slack(b) <= tb <= 1.0 + slack(b):
            hits.append((ta, tb, point))
    return sorted(hits, key=lambda hit: hit[0])


# endregion 🔖️Curves


# region 🔖️Bands
def axis_curve(axis):
    """〰️ The authored axis as a curve."""
    tag, body = variant(axis)
    return Curve(xy(body["start"]), xy(body["end"]), body["bulge"] if tag == "Arc" else 0.0)


def offsets(snapshot, wall):
    """↔️ `(left, right, interfaces)`: distances from the axis to the faces and the signed layer interface offsets, left positive."""
    layers = snapshot["wall_types"].get(wall["wall_type"], {"layers": []})["layers"]
    thickness = math.fsum(layer["thickness"] for layer in layers)
    location = wall["location"]
    if location == "Center":
        left = thickness / 2.0
    elif location == "Interior":
        left = 0.0
    elif location == "Exterior":
        left = thickness
    else:
        chosen = [index for index, layer in enumerate(layers) if layer["function"] == "Core"] or [index for index, layer in enumerate(layers) if layer["function"] == "Structure"]
        left = (math.fsum(layer["thickness"] for layer in layers[: chosen[0]]) + math.fsum(layer["thickness"] for layer in layers[: chosen[-1] + 1])) / 2.0 if chosen else thickness / 2.0
    interfaces = [left]
    cumulative = 0.0
    for layer in layers:
        cumulative += layer["thickness"]
        interfaces.append(left - cumulative)
    return left, thickness - left, interfaces


class Band:
    """🧱️ A wall in plan: its axis and the distances to its left and right face."""

    def __init__(self, axis, left, right):
        self.axis, self.left, self.right = axis, left, right

    @property
    def thickness(self):
        return self.left + self.right

    def face(self, side):
        return self.axis.offset(self.left if side == "left" else -self.right)

    def tip(self, tip):
        return self.axis.start if tip == 0 else self.axis.end

    def outward(self, tip):
        x, y = self.axis.tangent_at(0.0 if tip == 0 else 1.0)
        return (x, y) if tip == 0 else (-x, -y)

    def square(self, tip):
        pick = lambda side: (lambda face: (face.start if tip == 0 else face.end) if face else self.tip(tip))(self.face(side))
        return pick("left"), pick("right"), None

    def interior(self, t):
        return t * self.axis.length > JOIN_TOLERANCE and (1.0 - t) * self.axis.length > JOIN_TOLERANCE


def storey_bands(snapshot, storey):
    """🧱️ The bands of the walls of one storey whose axis has a length."""
    bands = {}
    for wall_id, wall in snapshot["walls"].items():
        if wall["storey"] != storey:
            continue
        axis = axis_curve(wall["axis"])
        if axis.length > JOIN_TOLERANCE:
            left, right, _ = offsets(snapshot, wall)
            bands[wall_id] = Band(axis, left, right)
    return bands


# endregion 🔖️Bands


# region 🔖️Joins
def contact(bands, wall_id, tip):
    """🔗️ What the axis end `tip` (0 start, 1 end) of a wall touches: `("free",)`, `("node", [(id, tip)...])` or `("butt", through id, hit point, tangent)`."""
    point = bands[wall_id].tip(tip)
    node = [(other, end) for other, band in sorted(bands.items()) if other != wall_id for end in (0, 1) if math.dist(band.tip(end), point) <= JOIN_TOLERANCE]
    if node:
        return ("node", node)
    for other, band in sorted(bands.items()):
        if other != wall_id and band.axis.closest_distance(point) <= JOIN_TOLERANCE:
            t = band.axis.closest_param(point)
            return ("butt", other, band.axis.point_at(t), band.axis.tangent_at(t))
    return ("free",)


def miter(ours, our_side, theirs, their_side, node):
    """📐️ Where one face of `ours` meets one face of `theirs` nearest the node, unless the corner is parallel or too far."""
    a, b = ours.face(our_side), theirs.face(their_side)
    if a is None or b is None:
        return None
    point = nearest(carrier_intersections(a, b), node)
    if point is None or math.dist(point, node) > MITER_LIMIT * max(ours.thickness, theirs.thickness) + JOIN_TOLERANCE:
        return None
    return point


def ccw_side(tip):
    """↔️ The face on the counter-clockwise side of the outward direction of an end."""
    return "left" if tip == 0 else "right"


def other_side(side):
    return "right" if side == "left" else "left"


def node_trim(bands, wall_id, tip, members):
    """🔗️ Trim of an end that is part of a node: mitre with the counter-clockwise and the clockwise neighbour around the node."""
    me, node = bands[wall_id], bands[wall_id].tip(tip)
    ring = sorted(((math.atan2(bands[other].outward(end)[1], bands[other].outward(end)[0]), other, end) for other, end in members + [(wall_id, tip)]))
    index = next(i for i, (_, other, end) in enumerate(ring) if other == wall_id and end == tip)
    ccw, cw = ring[(index + 1) % len(ring)], ring[(index - 1) % len(ring)]
    at_ccw = miter(me, ccw_side(tip), bands[ccw[1]], other_side(ccw_side(ccw[2])), node)
    at_cw = miter(me, other_side(ccw_side(tip)), bands[cw[1]], ccw_side(cw[2]), node)
    left, right = (at_ccw, at_cw) if tip == 0 else (at_cw, at_ccw)
    square_left, square_right, _ = me.square(tip)
    return (left or square_left, right or square_right, None)


def butt_trim(bands, wall_id, tip, through, hit, tangent):
    """🔗️ Trim of an end that butts against the near face of another wall."""
    me, wall = bands[wall_id], bands[through]
    square = me.square(tip)
    outward = me.outward(tip)
    orientation = tangent[0] * outward[1] - tangent[1] * outward[0]
    if abs(orientation) <= 1e-9:
        return square
    near = wall.face("left" if orientation > 0 else "right")

    def cut(side, fallback):
        face = me.face(side)
        if face is None or near is None:
            return fallback
        point = nearest(carrier_intersections(face, near), hit)
        if point is None or math.dist(point, hit) > MITER_LIMIT * max(me.thickness, wall.thickness) + JOIN_TOLERANCE:
            return fallback
        return point

    left, right = cut("left", None), cut("right", None)
    if left is None or right is None:
        return (left or square[0], right or square[1], None)
    return left, right, near


END = {0: "Start", 1: "End"}


def join_wall(bands, wall_id):
    """🔗️ `(start trim, end trim, joins)` of one wall among the bands of its storey, in the documented order."""
    me = bands[wall_id]
    joins, trims = [], []
    for tip in (0, 1):
        point = me.tip(tip)
        found = contact(bands, wall_id, tip)
        if found[0] == "free":
            trims.append(me.square(tip))
        elif found[0] == "node":
            joins += [{"kind": "Miter", "end": END[tip], "other": other, "other_end": END[end], "point": mark(point), "overlap_area": 0.0} for other, end in found[1]]
            trims.append(node_trim(bands, wall_id, tip, found[1]))
        else:
            joins.append({"kind": "Butt", "end": END[tip], "other": found[1], "other_end": "Along", "point": mark(found[2]), "overlap_area": 0.0})
            trims.append(butt_trim(bands, wall_id, tip, found[1], found[2], found[3]))
    for other, band in sorted(bands.items()):
        if other == wall_id:
            continue
        for tip in (0, 1):
            if me.axis.closest_distance(band.tip(tip)) <= JOIN_TOLERANCE:
                found = contact(bands, other, tip)
                if found[0] == "butt" and found[1] == wall_id:
                    joins.append({"kind": "Through", "end": "Along", "other": other, "other_end": END[tip], "point": mark(found[2]), "overlap_area": 0.0})
        for ta, tb, point in bounded_hits(me.axis, band.axis):
            if me.interior(ta) and band.interior(tb):
                joins.append({"kind": "Cross", "end": "Along", "other": other, "other_end": "Along", "point": mark(point), "overlap_area": cross_overlap(me, band)})
    return trims[0], trims[1], joins


# endregion 🔖️Joins


# region 🔖️Footprints
def loop_area(vertices):
    """📐️ Exact area of a closed loop of `(point, bulge)` vertices: the shoelace sum plus the signed circular-segment terms."""
    total = 0.0
    for index, (point, bulge) in enumerate(vertices):
        following = vertices[(index + 1) % len(vertices)][0]
        total += (point[0] * following[1] - following[0] * point[1]) / 2.0
        if abs(bulge) > 1e-12:
            sweep = abs(4.0 * math.atan(bulge))
            radius = math.dist(point, following) / (2.0 * math.sin(sweep / 2.0))
            total += math.copysign(1.0, bulge) * radius * radius / 2.0 * (sweep - math.sin(sweep))
    return total


def cut_bulge(trim, source, target):
    """🌙️ Bulge of an end edge that follows the face of the wall it butts against: the short arc of that face's circle from `source` to `target`, 0 for a straight cut."""
    cut = trim[2]
    center = cut.center if cut is not None else None
    if center is None:
        return 0.0
    first = (source[0] - center[0], source[1] - center[1])
    second = (target[0] - center[0], target[1] - center[1])
    return math.tan(math.atan2(first[0] * second[1] - first[1] * second[0], first[0] * second[0] + first[1] * second[1]) / 4.0)


def footprint_of(band, start, end):
    """🧱️ `(left face, right face, vertices)`: the faces retargeted to the end trims and the footprint loop `[right.start, right.end, left.end, left.start]`; a butt end edge follows the face it butts against."""
    left, right = band.face("left"), band.face("right")
    if left is None or right is None:
        return None
    left, right = left.retarget(start[0], end[0]), right.retarget(start[1], end[1])
    return left, right, [(right.start, right.bulge), (right.end, cut_bulge(end, right.end, left.end)), (left.end, -left.bulge), (left.start, cut_bulge(start, left.start, right.start))]


def sampled_polygon(vertices):
    """〰️ A shapely polygon of a loop; arc edges are sampled."""
    ring = []
    for index, (point, bulge) in enumerate(vertices):
        following = vertices[(index + 1) % len(vertices)][0]
        ring += Curve(point, following, bulge).sampled()[:-1]
    return Polygon(ring)


def band_polygon(band, start=None, end=None):
    """🧱️ The (optionally trimmed) footprint of a band as a shapely polygon."""
    start = start or (band.face("left").start, band.face("right").start, None)
    end = end or (band.face("left").end, band.face("right").end, None)
    return sampled_polygon(footprint_of(band, start, end)[2])


def cross_overlap(first, second):
    """🔀️ Area both untrimmed bodies cover at a crossing, measured by shapely."""
    return band_polygon(first).intersection(band_polygon(second)).area


def plan_geometry(snapshot):
    """🧱️ Per wall: thickness, centreline length, face offsets, layer interfaces, joined faces and footprint, areas per unit of height and the joins."""
    result = {}
    storeys = {}
    for wall in snapshot["walls"].values():
        storeys.setdefault(wall["storey"], None)
    banded = {storey: storey_bands(snapshot, storey) for storey in storeys}
    for wall_id, wall in snapshot["walls"].items():
        left, right, interfaces = offsets(snapshot, wall)
        axis = axis_curve(wall["axis"])
        row = {
            "thickness": math.fsum(layer["thickness"] for layer in snapshot["wall_types"].get(wall["wall_type"], {"layers": []})["layers"]),
            "length": axis.length,
            "offset_left": left,
            "offset_right": right,
            "layer_offsets": interfaces,
            "left_face": {"start": mark((0.0, 0.0)), "end": mark((0.0, 0.0)), "bulge": 0.0},
            "right_face": {"start": mark((0.0, 0.0)), "end": mark((0.0, 0.0)), "bulge": 0.0},
            "left_length": 0.0,
            "right_length": 0.0,
            "footprint": [],
            "footprint_area": 0.0,
            "joins": [],
        }
        bands = banded[wall["storey"]]
        if wall_id in bands:
            start, end, joins = join_wall(bands, wall_id)
            row["joins"] = joins
            placed = footprint_of(bands[wall_id], start, end)
            if placed:
                face_left, face_right, vertices = placed
                curve = lambda face: {"start": mark(face.start), "end": mark(face.end), "bulge": face.bulge}
                row.update(left_face=curve(face_left), right_face=curve(face_right), left_length=face_left.length, right_length=face_right.length, footprint=[{"point": mark(p), "bulge": b} for p, b in vertices], footprint_area=loop_area(vertices))
        result[wall_id] = row
    return result


# endregion 🔖️Footprints


# region 🔖️Audit
def close(actual, expected, relative):
    """⚖️ Whether two numbers agree within the absolute or relative tolerance."""
    return abs(actual - expected) <= (SAMPLED if relative else EXACT) * max(abs(expected), 1.0)


def long_line(curve, reach=1e4):
    """〰️ The infinite carrier of a straight curve as a very long shapely line."""
    d = (curve.end[0] - curve.start[0], curve.end[1] - curve.start[1])
    length = math.hypot(*d)
    unit = (d[0] / length, d[1] / length)
    return LineString([(curve.start[0] - unit[0] * reach, curve.start[1] - unit[1] * reach), (curve.start[0] + unit[0] * reach, curve.start[1] + unit[1] * reach)])


def shapely_corner(a, b, near):
    """📐️ Intersection of two straight carriers computed by GEOS from their `offset_curve`s: the independent mitre point."""
    point = long_line(a).intersection(long_line(b))
    return (point.x, point.y) if point.geom_type == "Point" and math.dist((point.x, point.y), near) <= MITER_LIMIT * 10 else None


def shapely_offset(curve, distance):
    """↔️ A straight face as GEOS sees it: the `offset_curve` of the axis (left positive), as a curve with the axis direction."""
    line = LineString([curve.start, curve.end]).offset_curve(distance)
    points = list(line.coords)
    return Curve(points[0], points[-1], 0.0)


def straight_problems(snapshot, plan):
    """🧪️ Every disagreement between the analytic mitre/butt corners of two straight walls and the GEOS construction of the same corners."""
    problems = []
    for storey in {wall["storey"] for wall in snapshot["walls"].values()}:
        bands = storey_bands(snapshot, storey)
        for wall_id, band in bands.items():
            if not band.axis.is_line:
                continue
            row = plan[wall_id]
            for join in row["joins"]:
                if join["kind"] not in ("Miter", "Butt") or not bands[join["other"]].axis.is_line:
                    continue
                other = bands[join["other"]]
                tip = 0 if join["end"] == "Start" else 1
                node = band.tip(tip)
                for side in ("left", "right"):
                    mine = shapely_offset(band.axis, band.left if side == "left" else -band.right)
                    if join["kind"] == "Miter":
                        other_tip = 0 if join["other_end"] == "Start" else 1
                        counter = ccw_side(tip) == side
                        their_side = other_side(ccw_side(other_tip)) if counter else ccw_side(other_tip)
                        if len([j for j in row["joins"] if j["end"] == join["end"] and j["kind"] == "Miter"]) != 1:
                            continue
                        theirs = shapely_offset(other.axis, other.left if their_side == "left" else -other.right)
                    else:
                        outward = band.outward(tip)
                        tangent = other.axis.tangent_at(other.axis.closest_param(node))
                        theirs = shapely_offset(other.axis, other.left if tangent[0] * outward[1] - tangent[1] * outward[0] > 0 else -other.right)
                    corner = shapely_corner(mine, theirs, node)
                    committed = row["left_face" if side == "left" else "right_face"]["start" if tip == 0 else "end"]
                    if corner is not None and not (close(corner[0], committed["x"], False) and close(corner[1], committed["y"], False)):
                        problems.append("%s %s face at the %s join with %s: GEOS corner %s, committed %s" % (wall_id, side, join["kind"], join["other"], corner, committed))
    return problems


def polygons(snapshot, plan):
    """🧱️ The committed footprint of every wall as a shapely polygon, by id."""
    return {wall_id: sampled_polygon([(xy(v["point"]), v["bulge"]) for v in row["footprint"]]) for wall_id, row in plan.items() if row["footprint"]}


def shapely_problems(snapshot, plan):
    """🩺️ Every disagreement between the analytic table and what shapely measures on the footprints, plus the join invariants."""
    problems = []
    shapes = polygons(snapshot, plan)
    for wall_id, polygon in shapes.items():
        row = plan[wall_id]
        sampled = variant(snapshot["walls"][wall_id]["axis"])[0] == "Arc"
        if not polygon.is_valid:
            problems.append("%s: footprint is not a valid polygon" % wall_id)
        if not close(polygon.area, row["footprint_area"], sampled):
            problems.append("%s: shapely area %.12g, committed %.12g" % (wall_id, polygon.area, row["footprint_area"]))
        for name, face in (("left_length", "left_face"), ("right_length", "right_face")):
            curve = Curve(xy(row[face]["start"]), xy(row[face]["end"]), row[face]["bulge"])
            if not close(LineString(curve.sampled()).length, row[name], sampled):
                problems.append("%s: shapely %s %.12g, committed %.12g" % (wall_id, name, LineString(curve.sampled()).length, row[name]))
        axis = axis_curve(snapshot["walls"][wall_id]["axis"])
        if not close(LineString(axis.sampled()).length, row["length"], sampled):
            problems.append("%s: shapely axis length %.12g, committed %.12g" % (wall_id, LineString(axis.sampled()).length, row["length"]))
    for wall_id, row in plan.items():
        for join in row["joins"]:
            other = join["other"]
            if other < wall_id or wall_id not in shapes or other not in shapes:
                continue
            overlap = shapes[wall_id].intersection(shapes[other]).area
            expected = join["overlap_area"] if join["kind"] == "Cross" else 0.0
            curved = any(variant(snapshot["walls"][i]["axis"])[0] == "Arc" for i in (wall_id, other))
            if abs(overlap - expected) > (1e-6 if curved else EXACT):
                problems.append("%s/%s (%s): shapely overlap %.12g, expected %.12g" % (wall_id, other, join["kind"], overlap, expected))
    problems += buffer_problems(snapshot, plan, shapes) + difference_problems(snapshot, plan, shapes) + straight_problems(snapshot, plan)
    return problems


def buffer_problems(snapshot, plan, shapes):
    """🧪️ A two-wall node of equal thickness and location Center covers the area of `LineString(chain).buffer(t / 2, mitre)`."""
    problems = []
    for wall_id, row in plan.items():
        walls = snapshot["walls"]
        for join in row["joins"]:
            other = join["other"]
            if join["kind"] != "Miter" or other < wall_id or wall_id not in shapes or other not in shapes:
                continue
            alone = all(len(plan[i]["joins"]) == 1 and plan[i]["joins"][0]["kind"] == "Miter" for i in (wall_id, other))
            centred = all(walls[i]["location"] == "Center" for i in (wall_id, other))
            if not (alone and centred and abs(row["thickness"] - plan[other]["thickness"]) < 1e-12):
                continue
            first, second = axis_curve(walls[wall_id]["axis"]), axis_curve(walls[other]["axis"])
            if join["end"] == "Start":
                first = first.reversed()
            if join["other_end"] == "End":
                second = second.reversed()
            chain = LineString(first.sampled()[:-1] + second.sampled())
            expected = chain.buffer(row["thickness"] / 2.0, cap_style="flat", join_style="mitre", mitre_limit=10.0).area
            joined = unary_union([shapes[wall_id], shapes[other]]).area
            curved = not (first.is_line and second.is_line)
            if abs(joined - expected) > (SAMPLED if curved else EXACT) * max(expected, 1.0):
                problems.append("%s/%s: union %.12g, shapely mitre buffer %.12g" % (wall_id, other, joined, expected))
    return problems


def difference_problems(snapshot, plan, shapes):
    """🧪️ A stem butting a wall equals its untrimmed band minus the body of that wall."""
    problems = []
    for wall_id, row in plan.items():
        for join in row["joins"]:
            if join["kind"] != "Butt" or wall_id not in shapes or join["other"] not in shapes:
                continue
            bands = storey_bands(snapshot, snapshot["walls"][wall_id]["storey"])
            expected = band_polygon(bands[wall_id]).difference(band_polygon(bands[join["other"]]))
            curved = not (bands[wall_id].axis.is_line and bands[join["other"]].axis.is_line)
            other_end = shapes[wall_id].symmetric_difference(expected).area
            if other_end > (1e-5 if curved else EXACT):
                problems.append("%s: footprint differs from the untrimmed band minus %s by %.12g" % (wall_id, join["other"], other_end))
    return problems


# endregion 🔖️Audit


# region 🔖️Standalone
def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key in sys.modules:
        return sys.modules[key]
    spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules[key] = module
    spec.loader.exec_module(module)
    return module


def layouts_of(snapshot):
    """🧱️ The full `🧱️wall-layout` table: heights from the levels oracle, plan geometry from this file."""
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels").projections(snapshot)["🧱️wall-layout"]


def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def wall_joins_handler(ctx):
    """🧱️ Oracle answer for the wall layout of the joins snapshot, after shapely agreed with the analytic route."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = shapely_problems(snapshot, plan_geometry(snapshot))
    if problems:
        raise AssertionError("; ".join(problems))
    payload = layouts_of(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("wall-joins", wall_joins_handler)


def main(arguments):
    """🏃️ `check` audits every case under a fixtures root with shapely; the tables are written and compared by the levels oracle."""
    root = Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        problems = shapely_problems(snapshot, plan_geometry(snapshot))
        failures += ["%s: %s" % (snapshot_path.parents[1].name, problem) for problem in problems]
        print("%s: shapely %s audited %d walls" % (snapshot_path.parents[1].name, shapely.__version__, len(snapshot["walls"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (arguments[0], "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
