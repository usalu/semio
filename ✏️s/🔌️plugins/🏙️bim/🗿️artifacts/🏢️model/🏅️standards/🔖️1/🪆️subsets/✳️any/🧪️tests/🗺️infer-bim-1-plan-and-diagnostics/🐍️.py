#!/usr/bin/env python3
"""🗺️ Third-party ORACLE for the `s.bim.model@1` inference `🗺️plan-linework`.

The subject (Rust, `semio-s-artifact-bim-model`) cuts every storey at the cut height it authors (1.2 m above its elevation by default) and draws the architectural plan as typed
primitives. This file re-derives, from the SAME committed snapshots and without sharing a line of code with the subject, the measures a
geometry library can adjudicate and compares them with the subject's `plan-metrics` table:

* `WallCut.area`: `shapely` builds every cut wall as a real polygon (the band between its two faces, extended by the half thickness of the wall
  it is mitered with, trimmed against the near face of the wall it butts against), removes the opening gaps (boxes across the band) and
  `unary_union` measures the poche. Arc walls are sampled (4096 chords) and audited against the closed form of the annular sector.
* `ColumnCut.area`, `CurtainMullion.area`: shapely polygons (rotated rectangles, sampled circles, I shapes, mullion boxes).
* `SlabEdge.length`, `BeamOutline.length`, `RailingPath.length`, `GridLine.length`: shapely `length` of the boundary rings, buffered beams and polylines;
  `SpaceOutline.length`: the explicit outlines plus the faces of `extent - union(wall footprints)` that hold a bounded seed (`polygonize`-style room finding).

The committed expectations under `🧫️fixtures/💡️inferences/🗺️plan-linework/<case>/💡️inference/🗺️plan-metrics/🔣️.json` are WRITTEN by this file
(`write`), never by hand; where a library can only approximate (arcs, circles) the closed form goes into the table and the library audits it.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences>

The same file answers for `⚠️diagnostics`: `shapely` rebuilds walls (mitered and butted bands), columns (rotated profiles) and beams (flat-capped
buffers of their axes) as polygons, intersects every pair in a building and reports the overlap areas of the pairs of kinds that clash (a beam
that ends on its partner rests on it, crossing walls are joined); missing wall types and hosts, zero-length axes, openings beyond their host,
storeys that share or skip a level and spaces that share a number are decided by plain set arithmetic. The table is `"<slug>|<ids>" -> measure`.

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
"""

# region 🔖️Imports
import csv
import io
import json
import math
import sys
from pathlib import Path

import shapely
from shapely import affinity
from shapely.geometry import LineString, Point, Polygon, box
from shapely.ops import unary_union

# endregion 🔖️Imports


# region 🔖️Vocabulary
DEFAULT_CUT_HEIGHT = 1.2
"""✂️ The plan convention: a storey that authors no `cut_height` is cut 1.2 m above its elevation."""

ARC_SEGMENTS = 4096
"""📐️ Chords an arc or circle is sampled into before shapely measures it."""

EPS = 1e-9
SNAP = 1e-9
"""📐️ Grid (metres) the footprints are snapped to, so that pieces that share an edge dissolve in a union."""

SAMPLED_TOLERANCE = 1e-5
"""⚖️ Tolerance where shapely measures a sampled arc against the closed form."""

COMPARE_TOLERANCE = 1e-9
"""⚖️ Tolerance between two committed tables."""

MEASURES = ("WallCut.area", "ColumnCut.area", "CurtainMullion.area", "SlabEdge.length", "BeamOutline.length", "RailingPath.length", "GridLine.length", "SpaceOutline.length")
"""📖️ The measures of the table, in the order the subject lists them."""

# endregion 🔖️Vocabulary


# region 🔖️Snapshot
def xy(point):
    """📍️ A `Point2` as an `(x, y)` pair."""
    return (point["x"], point["y"])


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


def load_snapshot(path):
    """📸️ Reads one committed snapshot fixture."""
    return json.loads(Path(path).read_text(encoding="utf-8"))


def storey_levels(snapshot):
    """🪜️ `{storey: (elevation, top)}`: level 0 is the building datum, levels >= 0 stack upward, levels < 0 hang downward (exactly rounded sums)."""
    result = {}
    for building_id in snapshot["buildings"]:
        rows = sorted((storey["level"], storey_id) for storey_id, storey in snapshot["storeys"].items() if storey["building"] == building_id)
        stacked, hung = [], []
        for level, storey_id in rows:
            if level >= 0:
                low = math.fsum(stacked)
                stacked.append(snapshot["storeys"][storey_id]["height"])
                result[storey_id] = (low, math.fsum(stacked))
        for level, storey_id in reversed(rows):
            if level < 0:
                high = -math.fsum(hung)
                hung.append(snapshot["storeys"][storey_id]["height"])
                result[storey_id] = (-math.fsum(hung), high)
    return result


def resolved_range(record, levels, base_offset_key="base_offset"):
    """🔝️ `(base, top)` of an element from its base offset and `TopConstraint`."""
    elevation, top_elevation = levels[record["storey"]]
    base = elevation + record.get(base_offset_key, 0.0)
    tag, body = variant(record["top"])
    if tag == "Unconnected":
        return base, base + body["height"]
    if tag == "StoreyTop":
        return base, top_elevation + body["offset"]
    return base, (levels[body["storey"]][0] if body["storey"] in levels else elevation) + body["offset"]


# endregion 🔖️Snapshot


# region 🔖️Walls
def face_offsets(wall, snapshot):
    """↔️ `(left, right)` distances from the axis to the faces, left being the interior face: `Interior` puts the axis on it, `Exterior` on the other, `Center` mid, `CoreCenter` on the core (else structure) layers."""
    layers = snapshot["wall_types"].get(wall["wall_type"], {"layers": []})["layers"]
    thickness = math.fsum(layer["thickness"] for layer in layers)
    if wall["location"] == "Interior":
        left = 0.0
    elif wall["location"] == "Exterior":
        left = thickness
    elif wall["location"] == "CoreCenter":
        left = thickness / 2.0
        for function in ("Core", "Structure"):
            rows = [index for index, layer in enumerate(layers) if layer["function"] == function]
            if rows:
                left = (math.fsum(layer["thickness"] for layer in layers[: rows[0]]) + math.fsum(layer["thickness"] for layer in layers[: rows[-1] + 1])) / 2.0
                break
    else:
        left = thickness / 2.0
    return left, thickness - left


def line_band(start, end, left, right, extend_start=0.0, extend_end=0.0):
    """▭️ The band around a straight axis between its faces, optionally lengthened at its ends."""
    dx, dy = end[0] - start[0], end[1] - start[1]
    length = math.hypot(dx, dy)
    ux, uy = dx / length, dy / length
    nx, ny = -uy, ux
    a = (start[0] - ux * extend_start, start[1] - uy * extend_start)
    b = (end[0] + ux * extend_end, end[1] + uy * extend_end)
    return Polygon([(a[0] - nx * right, a[1] - ny * right), (b[0] - nx * right, b[1] - ny * right), (b[0] + nx * left, b[1] + ny * left), (a[0] + nx * left, a[1] + ny * left)])


def arc_geometry(body):
    """〰️ Centre, radius, first angle and signed sweep of an arc axis (`bulge = tan(sweep / 4)`)."""
    start, end, bulge = xy(body["start"]), xy(body["end"]), body["bulge"]
    sweep = 4.0 * math.atan(bulge)
    chord = math.hypot(end[0] - start[0], end[1] - start[1])
    ux, uy = (end[0] - start[0]) / chord, (end[1] - start[1]) / chord
    offset = chord * (1.0 - bulge * bulge) / (4.0 * bulge)
    centre = ((start[0] + end[0]) / 2.0 - uy * offset, (start[1] + end[1]) / 2.0 + ux * offset)
    radius = math.hypot(start[0] - centre[0], start[1] - centre[1])
    return centre, radius, math.atan2(start[1] - centre[1], start[0] - centre[0]), sweep


def sector(centre, inner, outer, first, sweep):
    """🍰️ The annular sector as a sampled shapely polygon."""
    angles = [first + sweep * step / ARC_SEGMENTS for step in range(ARC_SEGMENTS + 1)]
    outer_ring = [(centre[0] + outer * math.cos(a), centre[1] + outer * math.sin(a)) for a in angles]
    inner_ring = [(centre[0] + inner * math.cos(a), centre[1] + inner * math.sin(a)) for a in reversed(angles)]
    return Polygon(outer_ring + inner_ring)


def opening_gap(snapshot, opening, host, levels, cut):
    """✂️ `(s_min, s_max)` of the part of an opening that cuts the plane, or `None`."""
    tag, body = variant(opening["kind"])
    if tag == "Window":
        kind = snapshot["window_types"][body["window_type"]]
        width, height, sill = kind["width"], kind["height"], opening.get("sill_override", kind["sill"])
    elif tag == "Door":
        kind = snapshot["door_types"][body["door_type"]]
        width, height, sill = kind["width"], kind["height"], opening.get("sill_override", 0.0)
    else:
        width, height, sill = body["width"], body["height"], opening.get("sill_override", 0.0)
    width, height = opening.get("width", width), opening.get("height", height)
    base, _ = resolved_range(host, levels)
    if not (base + sill <= cut < base + sill + height):
        return None
    return opening["offset"] - width / 2.0, opening["offset"] + width / 2.0


def joined_band(snapshot, walls, wid):
    """🔗️ The footprint of a straight wall: its band lengthened at a mitered end by the half thickness of the wall it meets and cut along the miter line (the bisector of the two walls, equal thicknesses), trimmed against the near face of the wall it butts against."""
    wall = walls[wid]
    tag, body = variant(wall["axis"])
    start, end = xy(body["start"]), xy(body["end"])
    left, right = face_offsets(wall, snapshot)
    extend, butts, cuts = [], [], []
    for point, far in ((start, end), (end, start)):
        reach = 0.0
        partners = []
        for other_id, other in walls.items():
            other_tag, other_body = variant(other["axis"])
            if other_id == wid or other_tag != "Line":
                continue
            os, oe = xy(other_body["start"]), xy(other_body["end"])
            other_left, other_right = face_offsets(other, snapshot)
            if any(math.hypot(point[0] - q[0], point[1] - q[1]) < 1e-6 for q in (os, oe)):
                reach = max(reach, (other_left + other_right) / 2.0)
                away = oe if math.hypot(point[0] - os[0], point[1] - os[1]) < 1e-6 else os
                partners.append((away, other_left + other_right))
            else:
                through = LineString([os, oe])
                if through.distance(Point(point)) < 1e-6 and 1e-6 < through.project(Point(point)) < through.length - 1e-6:
                    butts.append(line_band(os, oe, other_left, other_right))
        extend.append(reach)
        if len(partners) == 1 and abs(partners[0][1] - (left + right)) < 1e-9:
            mine = (far[0] - point[0], far[1] - point[1])
            theirs = (partners[0][0][0] - point[0], partners[0][0][1] - point[1])
            mine = (mine[0] / math.hypot(*mine), mine[1] / math.hypot(*mine))
            theirs = (theirs[0] / math.hypot(*theirs), theirs[1] / math.hypot(*theirs))
            if abs(mine[0] * theirs[1] - mine[1] * theirs[0]) > 1e-9:
                bisector = (mine[0] + theirs[0], mine[1] + theirs[1])
                norm = math.hypot(*bisector)
                bisector = (bisector[0] / norm, bisector[1] / norm)
                side = (-bisector[1], bisector[0])
                if side[0] * mine[0] + side[1] * mine[1] < 0:
                    side = (-side[0], -side[1])
                big = 1e3
                cuts.append(Polygon([(point[0] - bisector[0] * big, point[1] - bisector[1] * big), (point[0] + bisector[0] * big, point[1] + bisector[1] * big), (point[0] + bisector[0] * big + side[0] * big, point[1] + bisector[1] * big + side[1] * big), (point[0] - bisector[0] * big + side[0] * big, point[1] - bisector[1] * big + side[1] * big)]))
    shape = line_band(start, end, left, right, extend[0], extend[1])
    for half in cuts:
        shape = shape.intersection(half)
    for through in butts:
        shape = shape.difference(through)
    return shapely.set_precision(shape, SNAP)


def wall_cut_area(snapshot, storey, levels, cut):
    """🧱️ The poche area of the storey: union of the cut walls minus the opening gaps, with the closed form for arc walls. Returns `(area, audit)` where `audit` is the shapely area of the sampled arcs for the closed-form part."""
    walls = {wid: w for wid, w in snapshot["walls"].items() if w["storey"] == storey}
    polygons, closed_form, audit = [], 0.0, 0.0
    for wid, wall in walls.items():
        base, top = resolved_range(wall, levels)
        if not (base <= cut < top) or wall["phase"] == "Demolished":
            continue
        tag, body = variant(wall["axis"])
        left, right = face_offsets(wall, snapshot)
        gaps = sorted(g for g in (opening_gap(snapshot, o, wall, levels, cut) for o in snapshot["openings"].values() if o["host"] == wid) if g)
        if tag == "Arc":
            centre, radius, first, sweep = arc_geometry(body)
            sign = 1.0 if sweep > 0 else -1.0
            inner, outer = (radius - left, radius + right) if sign > 0 else (radius + left, radius - right)
            length = radius * abs(sweep)
            span = [(max(0.0, a), min(length, b)) for a, b in gaps]
            area = 0.5 * (outer * outer - inner * inner) * abs(sweep) - sum(0.5 * (outer * outer - inner * inner) * (b - a) / radius for a, b in span if b > a)
            shape = sector(centre, inner, outer, first, sweep)
            for a, b in span:
                shape = shape.difference(sector(centre, inner - 1.0, outer + 1.0, first + sign * a / radius, sign * (b - a) / radius))
            closed_form += area
            audit += shape.area
            continue
        start, end = xy(body["start"]), xy(body["end"])
        shape = joined_band(snapshot, walls, wid)
        for a, b in gaps:
            shape = shape.difference(gap_box(start, end, a, b, max(left, right) + 1.0))
        polygons.append(shape)
    union = unary_union(polygons).area if polygons else 0.0
    return union + closed_form, (union + audit) if audit else None


def gap_box(start, end, s_min, s_max, half):
    """✂️ The strip across a straight wall between arc lengths `s_min` and `s_max`."""
    length = math.hypot(end[0] - start[0], end[1] - start[1])
    ux, uy = (end[0] - start[0]) / length, (end[1] - start[1]) / length
    nx, ny = -uy, ux
    a = (start[0] + ux * s_min, start[1] + uy * s_min)
    b = (start[0] + ux * s_max, start[1] + uy * s_max)
    return Polygon([(a[0] - nx * half, a[1] - ny * half), (b[0] - nx * half, b[1] - ny * half), (b[0] + nx * half, b[1] + ny * half), (a[0] + nx * half, a[1] + ny * half)])


# endregion 🔖️Walls


# region 🔖️Members
def profile_polygon(profile):
    """▭️ A profile as a shapely polygon around the origin (`x` across, `y` deep) and its closed-form area."""
    tag, body = variant(profile)
    if tag == "Rectangle":
        w, d = body["width"], body["depth"]
        return box(-w / 2.0, -d / 2.0, w / 2.0, d / 2.0), w * d
    if tag == "Circle":
        r = body["diameter"] / 2.0
        return Point(0.0, 0.0).buffer(r, ARC_SEGMENTS // 4), math.pi * r * r
    if tag == "IShape":
        w, d, t, f = body["width"], body["depth"], body["web"], body["flange"]
        shape = unary_union([box(-w / 2.0, -d / 2.0, w / 2.0, -d / 2.0 + f), box(-w / 2.0, d / 2.0 - f, w / 2.0, d / 2.0), box(-t / 2.0, -d / 2.0, t / 2.0, d / 2.0)])
        return shape, 2.0 * w * f + t * (d - 2.0 * f)
    shape = Polygon([xy(vertex["point"]) for vertex in body["outline"]])
    return shape, shape.area


def column_cut_area(snapshot, storey, levels, cut):
    """🏛️ Area of the columns the plane cuts: the closed form, and the shapely area as audit."""
    total, audit = 0.0, 0.0
    for column in snapshot["columns"].values():
        if column["storey"] != storey:
            continue
        base, top = resolved_range(column, levels)
        if not (base <= cut < top):
            continue
        shape, area = profile_polygon(snapshot["column_types"][column["column_type"]]["profile"])
        placed = affinity.translate(affinity.rotate(shape, column["rotation"], origin=(0, 0), use_radians=True), *xy(column["position"]))
        total += area
        audit += placed.area
    return total, audit


def grid_edges(extent, rule):
    """🕸️ The cell edges `0 ..= extent` of one direction under a grid rule: equal cells of about a spacing, or the explicit lines strictly inside the extent."""
    if rule is None:
        return [0.0, extent]
    kind, body = next(iter(rule.items()))
    if kind == "Spacing":
        count = 1 if body["spacing"] <= 1e-9 or extent <= 1e-9 else max(1, math.ceil(extent / body["spacing"] - 1e-9))
        return [extent * k / count for k in range(count + 1)]
    inside = sorted(line for line in body["positions"] if 1e-9 < line < extent - 1e-9)
    return [0.0, *[line for index, line in enumerate(inside) if index == 0 or line - inside[index - 1] > 1e-9], extent]


def mullion_area(snapshot, storey, levels, cut):
    """🪞️ Area of the mullions of the curtain walls the plane cuts: one section on every vertical grid edge, the border section on the first and the last, the interior section on the others (the grid rule of the wall over the one of its type)."""
    total = 0.0
    for curtain in snapshot["curtain_walls"].values():
        if curtain["storey"] != storey:
            continue
        base, top = resolved_range(curtain, levels)
        if not (base <= cut < top):
            continue
        kind = snapshot["curtain_wall_types"][curtain["curtain_wall_type"]]
        tag, body = variant(curtain["axis"])
        length = LineString([xy(body["start"]), xy(body["end"])]).length
        edges = grid_edges(length, curtain.get("u_grid") or kind["u_grid"])
        for k in range(len(edges)):
            shape, _ = profile_polygon(kind["border_mullion"] if k in (0, len(edges) - 1) else kind["interior_mullion"])
            total += shape.area
    return total


def ring(vertices):
    """🔷️ The straight ring of a loop (the fixtures carry no bulges in loops)."""
    assert all(vertex["bulge"] == 0 for vertex in vertices), "loops with arcs are measured by the Rust tests, not here"
    return Polygon([xy(vertex["point"]) for vertex in vertices])


def slab_edge_length(snapshot, storey, levels, cut):
    """⬜️ Perimeter of the slabs the plane does not cut (the outer ring only)."""
    total = 0.0
    for slab in snapshot["slabs"].values():
        if slab["storey"] != storey:
            continue
        thickness = math.fsum(layer["thickness"] for layer in snapshot["slab_types"][slab["slab_type"]]["layers"])
        top = levels[storey][0] + slab["offset"]
        if not (top - thickness <= cut < top):
            total += ring(slab["boundary"]).exterior.length
    return total


def beam_outline_length(snapshot, storey):
    """➖️ Perimeter of the flat-capped buffer of every beam axis."""
    total = 0.0
    for beam in snapshot["beams"].values():
        if beam["storey"] != storey:
            continue
        width = variant(snapshot["beam_types"][beam["beam_type"]]["profile"])[1].get("width", 0.0)
        kind, body = variant(beam["axis"])
        if kind == "Arc":
            sweep, chord = 4.0 * math.atan(body["bulge"]), math.dist(xy(body["start"]), xy(body["end"]))
            radius = chord / (2.0 * abs(math.sin(sweep / 2.0)))
            total += 2.0 * abs(sweep) * radius + 2.0 * width
        else:
            total += LineString([xy(body["start"]), xy(body["end"])]).buffer(width / 2.0, cap_style="flat").exterior.length
    return total


def railing_length(snapshot, storey):
    """🛤️ Length of the railing paths."""
    return float(sum(LineString([xy(p) for p in railing["path"]]).length for railing in snapshot["railings"].values() if railing["storey"] == storey and len(railing["path"]) > 1))


def grid_length(snapshot, storey):
    """📏️ Length of the grid lines of the building of the storey."""
    building = snapshot["storeys"][storey]["building"]
    return sum(LineString([xy(g["start"]), xy(g["end"])]).length for g in snapshot["grids"].values() if g["building"] == building)


def room_perimeter(snapshot, storey):
    """🏠️ Perimeter (outline plus islands) of the explicit space outlines and of the rooms around the bounded seeds: the face of `extent - union(wall footprints)` that holds the seed, ignored where it reaches the extent (open) or the seed lies in a wall."""
    total = sum(ring(space["boundary"]["Explicit"]["outline"]).exterior.length for space in snapshot["spaces"].values() if space["storey"] == storey and "Explicit" in space["boundary"])
    seeds = [Point(xy(space["boundary"]["Bounded"]["seed"])) for space in snapshot["spaces"].values() if space["storey"] == storey and "Bounded" in space["boundary"]]
    if not seeds:
        return total
    walls = {wid: w for wid, w in snapshot["walls"].items() if w["storey"] == storey}
    shapes = [joined_band(snapshot, walls, wid) for wid, wall in walls.items() if variant(wall["axis"])[0] == "Line" and sum(face_offsets(wall, snapshot)) > 1e-9 and LineString([xy(variant(wall["axis"])[1]["start"]), xy(variant(wall["axis"])[1]["end"])]).length > 1e-6]
    for curtain in snapshot["curtain_walls"].values():
        if curtain["storey"] == storey:
            _, body = variant(curtain["axis"])
            depth = variant(snapshot["curtain_wall_types"][curtain["curtain_wall_type"]]["interior_mullion"])[1].get("depth", 0.0)
            shapes.append(line_band(xy(body["start"]), xy(body["end"]), depth / 2.0, depth / 2.0))
    obstacles = unary_union(shapes)
    minx, miny, maxx, maxy = unary_union([obstacles] + seeds).bounds
    extent = box(minx - 1.0, miny - 1.0, maxx + 1.0, maxy + 1.0)
    faces = extent.difference(obstacles)
    for seed in seeds:
        for face in getattr(faces, "geoms", [faces]):
            if face.contains(seed) and not face.exterior.intersects(extent.exterior):
                total += face.exterior.length + sum(hole.length for hole in face.interiors)
    return total


# endregion 🔖️Members


# region 🔖️Diagnostics
CLASH_AREA = 1e-3
"""⚖️ Footprint overlaps below 10 cm2 are not clashes."""

CLASH_HEIGHT = 1e-6
"""⚖️ Height overlaps below a micrometre are not clashes."""

CLASH_SLUGS = {("wall", "wall"): "clash.wall-wall", ("column", "wall"): "clash.wall-column", ("column", "column"): "clash.column-column", ("beam", "wall"): "clash.wall-beam", ("beam", "column"): "clash.beam-column", ("beam", "beam"): "clash.beam-beam"}
"""📖️ The clashes a prismatic overlap test adjudicates, by the sorted pair of kinds."""


def bodies(snapshot, levels):
    """📦️ Walls, columns and beams as `{id, kind, storey, shape, low, high, axis, ends}`: shapely polygons between two heights."""
    rows = []
    by_storey = {}
    for wid, wall in snapshot["walls"].items():
        by_storey.setdefault(wall["storey"], {})[wid] = wall
    for wid, wall in snapshot["walls"].items():
        if wall["storey"] not in levels or wall["wall_type"] not in snapshot["wall_types"]:
            continue
        tag, body = variant(wall["axis"])
        start, end = xy(body["start"]), xy(body["end"])
        left, right = face_offsets(wall, snapshot)
        if tag != "Line" or math.hypot(end[0] - start[0], end[1] - start[1]) <= 1e-6 or left + right <= 1e-9:
            continue
        base, top = resolved_range(wall, levels)
        rows.append({"id": wid, "kind": "wall", "storey": wall["storey"], "shape": joined_band(snapshot, by_storey[wall["storey"]], wid), "low": base, "high": top, "axis": LineString([start, end]), "ends": None})
    for cid, column in snapshot["columns"].items():
        kind = snapshot["column_types"].get(column["column_type"])
        if kind is None or column["storey"] not in levels:
            continue
        shape, _ = profile_polygon(kind["profile"])
        placed = affinity.translate(affinity.rotate(shape, column["rotation"], origin=(0, 0), use_radians=True), *xy(column["position"]))
        base, top = resolved_range(column, levels)
        rows.append({"id": cid, "kind": "column", "storey": column["storey"], "shape": placed, "low": base, "high": top, "axis": None, "ends": None})
    for bid, beam in snapshot["beams"].items():
        kind = snapshot["beam_types"].get(beam["beam_type"])
        axis_kind, axis_body = variant(beam["axis"])
        start, end = xy(axis_body["start"]), xy(axis_body["end"])
        if kind is None or beam["storey"] not in levels or host_length(beam) <= 1e-6:
            continue
        width = variant(kind["profile"])[1].get("width", 0.0)
        depth = variant(kind["profile"])[1].get("depth", 0.0)
        top = levels[beam["storey"]][1] + beam["top_offset"]
        end_top = levels[beam["storey"]][1] + (beam["top_offset"] if beam.get("end_top_offset") is None else beam["end_top_offset"])
        if axis_kind == "Arc":
            centre, radius, first, sweep = arc_geometry(axis_body)
            path = LineString([(centre[0] + radius * math.cos(first + sweep * i / 4096), centre[1] + radius * math.sin(first + sweep * i / 4096)) for i in range(4097)])
        else:
            path = LineString([start, end])
        rows.append({"id": bid, "kind": "beam", "storey": beam["storey"], "shape": path.buffer(width / 2.0, cap_style="flat"), "low": min(top, end_top) - depth, "high": max(top, end_top), "axis": None, "ends": (Point(start), Point(end))})
    return rows


def clashes(snapshot, levels):
    """💥️ `{key: overlap area}` of the clashing pairs of one model, within each building."""
    rows = bodies(snapshot, levels)
    found = {}
    for index, first in enumerate(rows):
        for second in rows[index + 1:]:
            if snapshot["storeys"][first["storey"]]["building"] != snapshot["storeys"][second["storey"]]["building"]:
                continue
            slug = CLASH_SLUGS.get(tuple(sorted((first["kind"], second["kind"]))))
            if slug is None or min(first["high"], second["high"]) - max(first["low"], second["low"]) <= CLASH_HEIGHT:
                continue
            if first["axis"] is not None and second["axis"] is not None and first["axis"].crosses(second["axis"]):
                continue
            beams = [(a, b) for a, b in ((first, second), (second, first)) if a["kind"] == "beam"]
            if any(b["shape"].buffer(1e-6).intersects(end) for a, b in beams for end in a["ends"]):
                continue
            area = first["shape"].intersection(second["shape"]).area
            if area > CLASH_AREA:
                found["%s|%s" % (slug, "+".join(sorted((first["id"], second["id"]))))] = area
    return found


def host_length(host):
    """📏️ The length of a wall or curtain wall axis."""
    tag, body = variant(host["axis"])
    if tag == "Line":
        return LineString([xy(body["start"]), xy(body["end"])]).length
    _, radius, _, sweep = arc_geometry(body)
    return radius * abs(sweep)


def opening_width(snapshot, opening):
    """📐️ The width of an opening: an explicit width wins over its type; `None` when its type is missing."""
    tag, body = variant(opening["kind"])
    kind = snapshot["window_types"].get(body.get("window_type")) if tag == "Window" else snapshot["door_types"].get(body.get("door_type")) if tag == "Door" else body
    return opening.get("width", kind["width"]) if kind else None


def references_and_extents(snapshot):
    """🔗️ Missing wall types and hosts, zero-length axes and openings that reach beyond their host."""
    found = {}
    for wid, wall in snapshot["walls"].items():
        if wall["wall_type"] not in snapshot["wall_types"]:
            found["reference.wall-type|%s" % wid] = 0.0
    for oid, opening in snapshot["openings"].items():
        host = snapshot["walls"].get(opening["host"]) or snapshot["curtain_walls"].get(opening["host"])
        if host is None:
            found["reference.opening-host|%s" % oid] = 0.0
            continue
        width = opening_width(snapshot, opening)
        if width is not None and (opening["offset"] - width / 2.0 < -1e-9 or opening["offset"] + width / 2.0 > host_length(host) + 1e-9) and host["storey"] in snapshot["storeys"]:
            found["opening.outside-host|%s" % oid] = 0.0
    for collection in ("walls", "curtain_walls"):
        for eid, element in snapshot[collection].items():
            if host_length(element) <= 1e-6:
                found["degenerate.axis-length|%s" % eid] = 0.0
    for bid, beam in snapshot["beams"].items():
        if host_length(beam) <= 1e-6:
            found["degenerate.axis-length|%s" % bid] = 0.0
    return found


def levels_and_numbers(snapshot):
    """🪜️ Storeys that share a level or skip one per building, and spaces of a building that share a number."""
    found = {}
    by_building = {}
    for sid, storey in snapshot["storeys"].items():
        by_building.setdefault(storey["building"], {}).setdefault(storey["level"], []).append(sid)
    for building, rows in by_building.items():
        for level, ids in rows.items():
            if len(ids) > 1:
                found["storey.level-duplicate|%s" % "+".join(sorted(ids))] = 0.0
        ordered = sorted(rows)
        for below, above in zip(ordered, ordered[1:]):
            if above - below > 1:
                found["storey.level-gap|%s+%s" % (sorted(rows[below])[0], sorted(rows[above])[0])] = float(above - below)
        numbers = {}
        for space_id, space in snapshot["spaces"].items():
            if space["storey"] in snapshot["storeys"] and snapshot["storeys"][space["storey"]]["building"] == building and space["number"]:
                numbers.setdefault(space["number"], []).append(space_id)
        for ids in numbers.values():
            if len(ids) > 1:
                found["space.duplicate-number|%s" % "+".join(sorted(ids))] = 0.0
    return found


def diagnostics_table(snapshot):
    """⚠️ The table `"<slug>|<ids>" -> {measure}` of every finding a third-party library can adjudicate."""
    found = {}
    found.update(clashes(snapshot, storey_levels(snapshot)))
    found.update(references_and_extents(snapshot))
    found.update(levels_and_numbers(snapshot))
    return {key: {"measure": float(value)} for key, value in sorted(found.items())}


# endregion 🔖️Diagnostics


# region 🔖️Projection
def plan_metrics(snapshot):
    """🗺️ The table `storey -> measure -> value` and the list of audit problems where shapely disagrees with a closed form."""
    levels = storey_levels(snapshot)
    table, problems = {}, []
    for storey in sorted(snapshot["storeys"]):
        cut = levels[storey][0] + snapshot["storeys"][storey].get("cut_height", DEFAULT_CUT_HEIGHT)
        walls, wall_audit = wall_cut_area(snapshot, storey, levels, cut)
        columns, column_audit = column_cut_area(snapshot, storey, levels, cut)
        table[storey] = {
            "WallCut.area": walls,
            "ColumnCut.area": columns,
            "CurtainMullion.area": mullion_area(snapshot, storey, levels, cut),
            "SlabEdge.length": slab_edge_length(snapshot, storey, levels, cut),
            "BeamOutline.length": beam_outline_length(snapshot, storey),
            "RailingPath.length": railing_length(snapshot, storey),
            "GridLine.length": grid_length(snapshot, storey),
            "SpaceOutline.length": room_perimeter(snapshot, storey),
        }
        table[storey] = {name: float(value) for name, value in table[storey].items()}
        if wall_audit is not None and abs(wall_audit - walls) > SAMPLED_TOLERANCE * max(1.0, walls):
            problems.append("%s: sampled wall poche %.9f vs closed form %.9f" % (storey, wall_audit, walls))
        if abs(column_audit - columns) > SAMPLED_TOLERANCE * max(1.0, columns):
            problems.append("%s: sampled columns %.9f vs closed form %.9f" % (storey, column_audit, columns))
    return table, problems


def rounded(value):
    """🔢️ Rounds every number to 12 decimals so the committed JSON is stable across platforms."""
    if isinstance(value, dict):
        return {key: rounded(item) for key, item in value.items()}
    if isinstance(value, float):
        return 0.0 if round(value, 12) == 0 else round(value, 12)
    return value


def differences(path, committed, computed):
    """⚖️ Differences between a committed table and a computed one, recursively."""
    if isinstance(committed, dict) and isinstance(computed, dict):
        found = []
        if sorted(committed) != sorted(computed):
            found.append("%s: keys differ: committed %s, computed %s" % (path, sorted(committed), sorted(computed)))
        for key in sorted(set(committed) & set(computed)):
            found += differences("%s.%s" % (path, key), committed[key], computed[key])
        return found
    if isinstance(committed, (int, float)) and isinstance(computed, (int, float)):
        return [] if abs(committed - computed) <= COMPARE_TOLERANCE * max(1.0, abs(committed)) else ["%s: committed %.12g, computed %.12g" % (path, committed, computed)]
    return [] if committed == computed else ["%s: committed %r, computed %r" % (path, committed, computed)]


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def plan_metrics_handler(ctx):
    """🗺️ Oracle answer for the plan metrics, after shapely agreed with every closed form."""
    from semio_repo_test import Outcome

    table, problems = plan_metrics(case_snapshot(ctx))
    if problems:
        raise AssertionError("; ".join(problems))
    table = rounded(table)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def diagnostics_handler(ctx):
    """⚠️ Oracle answer for the adjudicated diagnostics: clash areas by shapely, ids and levels by set arithmetic."""
    from semio_repo_test import Outcome

    table = rounded(diagnostics_table(case_snapshot(ctx)))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


ADJUDICATED_CODES = ("clash.wall-wall", "clash.wall-column", "clash.column-column", "clash.wall-beam", "clash.beam-column", "clash.beam-beam", "reference.wall-type", "reference.opening-host", "opening.outside-host", "degenerate.axis-length", "storey.level-duplicate", "storey.level-gap", "space.duplicate-number")
"""⚖️ The codes of the diagnostics a library can adjudicate on its own, by their slugs."""

CSV_HEADER = ["severity", "code", "storey", "elements", "missing", "message_en", "message_de"]
"""📊️ The columns of the CSV export of the findings."""


def export_uri(ctx, name):
    """📤️ The URI of the committed export file a scenario names."""
    return next(candidate for candidate in ctx.step_input_uris() if "📤️export" in candidate and candidate.endswith(name))


def export_measure(finding):
    """📏️ The measure of an exported finding: the overlap area of a clash, the levels a gap skips, else zero."""
    values = finding["values"]
    if finding["code"] == "storey.level-gap":
        return float(values["to"]) - float(values["from"])
    return float(values.get("overlap_area", 0.0))


def export_json_table(document):
    """🧾️ The adjudicated table of the JSON export, read with the json module: `"<slug>|<ids>" -> {measure}`."""
    return {"%s|%s" % (finding["code"], "+".join(finding["elements"])): {"measure": export_measure(finding)} for finding in document["findings"] if finding["code"] in ADJUDICATED_CODES}


def export_json_handler(ctx):
    """🧾️ Oracle answer for the JSON export: python's json module reads the committed file and shapely's table must agree with it."""
    from semio_repo_test import Outcome

    document = json.loads(ctx.input_bytes(export_uri(ctx, "⚠️diagnostics.json")).decode("utf-8"))
    table = rounded(export_json_table(document))
    problems = differences("table", rounded(diagnostics_table(case_snapshot(ctx))), table)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


PANEL_SEVERITIES = (("error", "error"), ("warning", "warning"), ("info", "note"))
"""🚨️ The severities of the exported findings and the keys of the panel groups, most severe first."""


def panel_group_table(document, snapshot):
    """🚨️ The groups of the diagnostics panel recomputed from the exported findings: per severity, the storeys by level (the storeys the model lacks after the known ones, the whole model last) with the findings counted per kind (the domain of the code)."""
    levels = {identifier: storey["level"] for identifier, storey in snapshot["storeys"].items()}
    table = {}
    for token, key in PANEL_SEVERITIES:
        groups = {}
        for finding in document["findings"]:
            if finding["severity"] != token:
                continue
            storey = finding["storey"]
            rank = (0, levels[storey], storey) if storey in levels else ((1, 0, storey) if storey is not None else (2, 0, ""))
            kinds = groups.setdefault(rank, {})
            kind = finding["code"].split(".", 1)[0]
            kinds[kind] = kinds.get(kind, 0) + 1
        table[key] = [{"storey": rank[2] if rank[0] < 2 else None, "kinds": {kind: float(count) for kind, count in kinds.items()}} for rank, kinds in sorted(groups.items())]
    return table


def panel_groups_handler(ctx):
    """🚨️ Oracle answer for the panel groups: the exported JSON is read with the json module and grouped with plain dictionaries."""
    from semio_repo_test import Outcome

    document = json.loads(ctx.input_bytes(export_uri(ctx, "⚠️diagnostics.json")).decode("utf-8"))
    table = panel_group_table(document, case_snapshot(ctx))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def export_csv_table(text):
    """📊️ The CSV export read with the csv module: `"<position>|<slug>|<ids>" -> {severity, storey, missing, message_en, message_de}`; a record that breaks RFC 4180 or the header fails."""
    rows = list(csv.reader(io.StringIO(text, newline=""), strict=True))
    if rows[0] != CSV_HEADER:
        raise AssertionError("header %r" % rows[0])
    table = {}
    for position, row in enumerate(rows[1:]):
        if len(row) != len(CSV_HEADER):
            raise AssertionError("record %d has %d fields" % (position, len(row)))
        record = dict(zip(CSV_HEADER, row))
        table["%04d|%s|%s" % (position, record["code"], record["elements"])] = {"severity": record["severity"], "storey": record["storey"], "missing": record["missing"], "message_en": record["message_en"], "message_de": record["message_de"]}
    return table


def export_csv_handler(ctx):
    """📊️ Oracle answer for the CSV export: python's csv module reads the committed file; every finding shapely adjudicates must be one of its records."""
    from semio_repo_test import Outcome

    table = export_csv_table(ctx.input_bytes(export_uri(ctx, "⚠️diagnostics.csv")).decode("utf-8"))
    present = {key.split("|", 1)[1] for key in table}
    missing = [key for key in diagnostics_table(case_snapshot(ctx)) if key not in present]
    if missing:
        raise AssertionError("the export lacks findings shapely adjudicates: %s" % missing)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("plan-metrics-house", plan_metrics_handler).oracle("plan-metrics-curved", plan_metrics_handler).oracle("plan-metrics-cut-heights", plan_metrics_handler).oracle("diagnostics-clean", diagnostics_handler).oracle("diagnostics-defects", diagnostics_handler).oracle("diagnostics-export-json", export_json_handler).oracle("diagnostics-export-csv", export_csv_handler).oracle("diagnostics-panel-groups", panel_groups_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under the fixtures root with its committed expectation; `write` regenerates them."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    jobs = [("🗺️plan-linework", "🗺️plan-metrics", plan_metrics), ("⚠️diagnostics", "⚠️diagnostics", lambda snapshot: (diagnostics_table(snapshot), []))]
    for family, table_name, compute in jobs:
        for snapshot_path in sorted((root / family).glob("*/📸️snapshot/🔣️.json")):
            case = snapshot_path.parents[1]
            table, problems = compute(load_snapshot(snapshot_path))
            table = rounded(table)
            failures += ["%s/%s: %s" % (family, case.name, problem) for problem in problems]
            target = case / "💡️inference" / table_name / "🔣️.json"
            if command == "write":
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
            else:
                failures += ["%s/%s: %s" % (family, case.name, problem) for problem in differences("table", json.loads(target.read_text(encoding="utf-8")), table)]
            print("%s/%s: shapely %s, %d row(s)" % (family, case.name, shapely.__version__, len(table)))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
