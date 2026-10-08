#!/usr/bin/env python3
"""🧊️ Third-party ORACLE (shapely 2 over GEOS, numpy) for the column, beam, slab, roof, stair and railing solids of `🧊️element-solids`.

Each case under `🧫️fixtures/💡️inferences/🧊️element-solids/` named in `CASES` holds an authored `snapshot` and an `expected` table written by this file from the
snapshot alone. The oracle never sees the subject's meshes: it re-derives every volume and every axis-aligned bound from closed forms and from `shapely`
(`Polygon.area`, `bounds`, `buffer` with mitre joins, `affinity`, `convex_hull`, `LineString.length`), so the subject's tessellation is measured against a library that has
never seen this repository.

* columns: `area(profile) * height`, bounds of the rotated, translated profile polygon between the resolved base and top;
* beams: `area(profile) * length`, bounds of the swept section rectangle with the profile dropped below the beam top;
* slabs: `area(boundary - holes) * sum(layers)`, bounds of the boundary and the top plane range of a slope;
* roofs: `area(eave) * sum(layers)` with the eave from a mitred `buffer`, the rise from closed forms audited by `buffer(-d)` bisection (hip), the extent (gable, shed), and the
  fallback decision from `convex_hull`;
* stairs: the riser law and the prisms re-derived from the documented `stair-runs` contract, areas and bounds from `shapely`;
* railings: posts and mitred rail from `LineString` measures.

An element whose geometry is degenerate is `null` in `expected` (the subject must not emit a solid for it). `volume_tolerance` and `bounds_tolerance` are the exact tolerance
the tessellated arcs allow: `1e-9` for planar parts, otherwise the arc length times the sagitta `CHORD_TOLERANCE`.

Standalone use (no test host needed):

    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧊️element-solids>
    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧊️element-solids>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🧬️schema/💡️inferences/🧊️element-solids — the subject
"""

# region 🔖️Imports
import itertools
import json
import math
import sys
from pathlib import Path

import numpy
import shapely
from shapely import affinity
from shapely.geometry import LineString, Polygon, box
from shapely.geometry.polygon import orient
from shapely.ops import polylabel

# endregion 🔖️Imports

# region 🔖️Constants
CHORD_TOLERANCE = 1e-4
"""📏️ Sagitta of every tessellated arc of the subject (`element_solids::CHORD_TOLERANCE`)."""

EXACT = 1e-9
"""⚖️ Tolerance of planar parts."""

POST_SIZE, RAIL_WIDTH, RAIL_DEPTH = 0.05, 0.06, 0.04
"""🛤️ The railing section constants of the subject (`railings::{POST_SIZE, RAIL_WIDTH, RAIL_DEPTH}`)."""

ARC_SAMPLES = 4096
"""🌙️ Chords per arc when the oracle samples a bulged edge for `shapely`."""

CASES = {
    "columns-profiles": "columns",
    "beams-profiles": "beams",
    "slabs-holes-slope": "slabs",
    "roofs-shapes": "roofs",
    "stairs-flights": "stairs",
    "railings-posts": "railings",
}
"""🗂️ The fixture cases of these families and the collection each one measures."""
# endregion 🔖️Constants


# region 🔖️Levels
def storey_levels(snapshot):
    """🪜️ `{storey id: (elevation, top)}` building-relative, level 0 at the building datum, levels stacking up and down."""
    levels = {}
    by_building = {}
    for storey_id, storey in snapshot["storeys"].items():
        by_building.setdefault(storey["building"], []).append((storey["level"], storey_id, storey["height"]))
    for rows in by_building.values():
        heights = []
        for level, storey_id, height in sorted(row for row in rows if row[0] >= 0):
            elevation = math.fsum(heights)
            heights.append(height)
            levels[storey_id] = (elevation, math.fsum(heights))
        heights = []
        for level, storey_id, height in sorted((row for row in rows if row[0] < 0), reverse=True):
            top = -math.fsum(heights)
            heights.append(height)
            levels[storey_id] = (-math.fsum(heights), top)
    return levels


def resolved_top(top, base_z, own, target):
    """🔝️ The top `z` of a `TopConstraint` (`StoreyTop` = own top, `Storey` = elevation of the constrained storey)."""
    (kind, body), = top.items()
    if kind == "Unconnected":
        return base_z + body["height"]
    if kind == "StoreyTop":
        return own[1] + body["offset"]
    return (target or own)[0] + body["offset"]


def target_level(top, levels):
    """🎯️ The level of the storey a `Storey` constraint names, if any."""
    (kind, body), = top.items()
    return levels.get(body["storey"]) if kind == "Storey" else None


# endregion 🔖️Levels


# region 🔖️Loops
def arc(start, end, bulge):
    """🌙️ `(centre, radius, sweep)` of the arc of a bulged edge; the sweep is signed, counter-clockwise positive."""
    sweep = 4.0 * math.atan(bulge)
    chord = (end[0] - start[0], end[1] - start[1])
    length = math.hypot(*chord)
    distance = (length / 2.0) / math.tan(sweep / 2.0) if abs(math.tan(sweep / 2.0)) > 1e-300 else 0.0
    normal = (-chord[1] / length, chord[0] / length)
    centre = ((start[0] + end[0]) / 2.0 + normal[0] * distance, (start[1] + end[1]) / 2.0 + normal[1] * distance)
    return centre, abs(length / (2.0 * math.sin(sweep / 2.0))), sweep


def sampled(vertices, samples=ARC_SAMPLES):
    """🔷️ A bulged loop as plain points, every arc cut into `samples` chords."""
    points = []
    count = len(vertices)
    for index, vertex in enumerate(vertices):
        start = (vertex["point"]["x"], vertex["point"]["y"])
        nxt = vertices[(index + 1) % count]["point"]
        end = (nxt["x"], nxt["y"])
        points.append(start)
        if abs(vertex["bulge"]) > 1e-12:
            centre, radius, sweep = arc(start, end, vertex["bulge"])
            first = math.atan2(start[1] - centre[1], start[0] - centre[0])
            for step in range(1, samples):
                angle = first + sweep * step / samples
                points.append((centre[0] + radius * math.cos(angle), centre[1] + radius * math.sin(angle)))
    return points


def closed_area(vertices):
    """📐️ Exact area of a bulged loop: shoelace plus the circular segments."""
    count = len(vertices)
    shoelace = 0.0
    segments = 0.0
    for index, vertex in enumerate(vertices):
        a = (vertex["point"]["x"], vertex["point"]["y"])
        nxt = vertices[(index + 1) % count]["point"]
        b = (nxt["x"], nxt["y"])
        shoelace += a[0] * b[1] - b[0] * a[1]
        if abs(vertex["bulge"]) > 1e-12:
            _, radius, sweep = arc(a, b, vertex["bulge"])
            segments += radius * radius / 2.0 * (sweep - math.sin(sweep))
    return abs(shoelace / 2.0 + segments)


def arc_length(vertices):
    """🌙️ Total length of the arcs of a loop."""
    count = len(vertices)
    total = 0.0
    for index, vertex in enumerate(vertices):
        if abs(vertex["bulge"]) > 1e-12:
            nxt = vertices[(index + 1) % count]["point"]
            _, radius, sweep = arc((vertex["point"]["x"], vertex["point"]["y"]), (nxt["x"], nxt["y"]), vertex["bulge"])
            total += radius * abs(sweep)
    return total


def region(shell, holes=()):
    """🧩️ `(polygon, exact area, arc length)` of a bulged loop with bulged holes; the sampled polygon is audited against the exact area."""
    polygon = Polygon(sampled(shell), [sampled(hole) for hole in holes])
    assert polygon.is_valid, "the sampled region is not a valid polygon"
    exact = closed_area(shell) - math.fsum(closed_area(hole) for hole in holes)
    assert abs(polygon.area - exact) <= 1e-6 * max(exact, 1.0), "shapely area %r vs closed form %r" % (polygon.area, exact)
    return polygon, exact, arc_length(shell) + math.fsum(arc_length(hole) for hole in holes)


def corners(*pairs):
    """🔷️ Straight loop vertices from points."""
    return [{"point": {"x": x, "y": y}, "bulge": 0.0} for x, y in pairs]


def profile_region(profile):
    """▭️ `(polygon, exact area, arc length)` of a centred profile, or `None` when the profile is invalid."""
    (kind, body), = profile.items()
    if kind == "Rectangle" and body["width"] > 0 and body["depth"] > 0:
        w, d = body["width"] / 2.0, body["depth"] / 2.0
        return box(-w, -d, w, d), body["width"] * body["depth"], 0.0
    if kind == "Circle" and body["diameter"] > 0:
        r = body["diameter"] / 2.0
        loop = [{"point": {"x": r, "y": 0.0}, "bulge": 1.0}, {"point": {"x": -r, "y": 0.0}, "bulge": 1.0}]
        return region(loop)
    if kind == "IShape" and min(body["width"], body["depth"], body["web"], body["flange"]) > 0 and body["web"] < body["width"] and 2 * body["flange"] < body["depth"]:
        x, y, w, f = body["width"] / 2.0, body["depth"] / 2.0, body["web"] / 2.0, body["flange"]
        points = [(-x, -y), (x, -y), (x, -y + f), (w, -y + f), (w, y - f), (x, y - f), (x, y), (-x, y), (-x, y - f), (-w, y - f), (-w, -y + f), (-x, -y + f)]
        polygon = Polygon(points)
        return polygon, 2 * body["width"] * f + body["web"] * (body["depth"] - 2 * f), 0.0
    if kind == "Custom" and len(body["outline"]) >= 3:
        return region(body["outline"])
    return None


# endregion 🔖️Loops


# region 🔖️Rows
def row(family, volume, bounds, arcs, extent, curved_bounds, bounds_scale=1.0, **more):
    """📋️ One expected row; the volume tolerance is the arc length times the sagitta times the extrusion extent, the bounds tolerance one sagitta (two where a round profile is dropped from its top)."""
    (min_x, min_y, min_z, max_x, max_y, max_z) = bounds
    tolerance = max(EXACT, arcs * CHORD_TOLERANCE * extent * 1.05)
    return {
        "family": family,
        "volume": volume,
        "volume_tolerance": tolerance,
        "min": [min_x, min_y, min_z],
        "max": [max_x, max_y, max_z],
        "bounds_tolerance": CHORD_TOLERANCE * 1.05 * bounds_scale if curved_bounds else EXACT,
        "curved": arcs > 0,
        **more,
    }


def xy_bounds(polygon):
    """📦️ `(min x, min y, max x, max y)` of a polygon."""
    return polygon.bounds


# endregion 🔖️Rows


# region 🔖️Columns
def column_rows(snapshot, levels):
    """🏛️ Column volume and bounds."""
    rows = {}
    for column_id, column in snapshot.get("columns", {}).items():
        rows[column_id] = None
        kind = snapshot.get("column_types", {}).get(column["column_type"])
        section = profile_region(kind["profile"]) if kind else None
        own = levels[column["storey"]]
        base = own[0] + column["base_offset"]
        top = resolved_top(column["top"], base, own, target_level(column["top"], levels))
        if section is None or top - base <= EXACT:
            continue
        polygon, area, arcs = section
        placed = affinity.translate(affinity.rotate(polygon, column["rotation"], origin=(0, 0), use_radians=True), column["position"]["x"], column["position"]["y"])
        min_x, min_y, max_x, max_y = xy_bounds(placed)
        rows[column_id] = row("Column", area * (top - base), (min_x, min_y, base, max_x, max_y, top), arcs, top - base, arcs > 0, height=top - base, base_z=base, top_z=top)
    return rows


# endregion 🔖️Columns


# region 🔖️Beams
def beam_rows(snapshot, levels):
    """➖️ Beam volume and bounds: the profile hangs from the reference line, the section rectangle is swept along the axis."""
    rows = {}
    for beam_id, beam in snapshot.get("beams", {}).items():
        rows[beam_id] = None
        kind = snapshot.get("beam_types", {}).get(beam["beam_type"])
        section = profile_region(kind["profile"]) if kind else None
        start, end = (beam["start"]["x"], beam["start"]["y"]), (beam["end"]["x"], beam["end"]["y"])
        length = LineString([start, end]).length
        if section is None or length <= EXACT:
            continue
        polygon, area, arcs = section
        min_u, min_v, max_u, max_v = polygon.bounds
        top = levels[beam["storey"]][1] + beam["top_offset"]
        normal = ((start[1] - end[1]) / length, (end[0] - start[0]) / length)
        sweep = Polygon([(start[0] + normal[0] * u, start[1] + normal[1] * u) for u in (min_u, max_u)] + [(end[0] + normal[0] * u, end[1] + normal[1] * u) for u in (max_u, min_u)])
        min_x, min_y, max_x, max_y = xy_bounds(sweep)
        rows[beam_id] = row("Beam", area * length, (min_x, min_y, top - (max_v - min_v), max_x, max_y, top), arcs, length, arcs > 0, bounds_scale=2.0, length=length, top_z=top)
    return rows


# endregion 🔖️Beams


# region 🔖️Slabs
def slab_rows(snapshot, levels):
    """⬜️ Slab volume and bounds, with the top plane range of a slope."""
    rows = {}
    for slab_id, slab in snapshot.get("slabs", {}).items():
        rows[slab_id] = None
        kind = snapshot.get("slab_types", {}).get(slab["slab_type"])
        thickness = math.fsum(max(layer["thickness"], 0.0) for layer in kind["layers"]) if kind else 0.0
        if thickness <= 1e-12 or len(slab["boundary"]) < 3:
            continue
        polygon, area, arcs = region(slab["boundary"], slab.get("holes", []))
        top = levels[slab["storey"]][0] + slab["offset"]
        fall = 0.0
        if slab.get("slope"):
            direction = slab["slope"]["direction"]
            projections = [x * math.cos(direction) + y * math.sin(direction) for x, y in polygon.exterior.coords]
            fall = math.tan(slab["slope"]["angle"]) * (max(projections) - min(projections))
        min_x, min_y, max_x, max_y = xy_bounds(polygon)
        rows[slab_id] = row("Slab", area * thickness, (min_x, min_y, top - fall - thickness, max_x, max_y, top), arcs, thickness, arcs > 0, thickness=thickness, top_z=top, fall=fall)
    return rows


# endregion 🔖️Slabs


# region 🔖️Roofs
def inscribed_distance(polygon):
    """🧭️ The largest `d` with a point at least `d` inside every edge line of a convex polygon: the height of a hip ridge over the pitch slope.

    A small linear program `maximise d subject to n_i . p - d >= n_i . a_i` in `(x, y, d)`, solved exactly by `numpy` over the vertices of its feasible region (triples of
    active edges), and audited against the pole of inaccessibility `shapely.ops.polylabel` finds by quadtree search.
    """
    ring = orient(polygon, 1.0).exterior.coords[:-1]
    rows = []
    for index, a in enumerate(ring):
        b = ring[(index + 1) % len(ring)]
        length = math.hypot(b[0] - a[0], b[1] - a[1])
        normal = (-(b[1] - a[1]) / length, (b[0] - a[0]) / length)
        rows.append((normal[0], normal[1], -1.0, normal[0] * a[0] + normal[1] * a[1]))
    best = None
    for triple in itertools.combinations(rows, 3):
        matrix = numpy.array([row[:3] for row in triple])
        if abs(numpy.linalg.det(matrix)) < 1e-12:
            continue
        x, y, d = numpy.linalg.solve(matrix, numpy.array([row[3] for row in triple]))
        if all(row[0] * x + row[1] * y - d >= row[3] - 1e-9 for row in rows) and (best is None or d > best):
            best = d
    pole = polylabel(polygon, tolerance=1e-4)
    assert abs(polygon.exterior.distance(pole) - best) < 1e-3, "polylabel %r vs the linear program %r" % (polygon.exterior.distance(pole), best)
    return float(best)


def roof_rows(snapshot, levels):
    """🏠️ Roof volume, bounds, fallback and ridge length."""
    rows = {}
    for roof_id, roof in snapshot.get("roofs", {}).items():
        rows[roof_id] = None
        kind = snapshot.get("roof_types", {}).get(roof["roof_type"])
        thickness = math.fsum(max(layer["thickness"], 0.0) for layer in kind["layers"]) if kind else 0.0
        footprint = roof["footprint"]
        if thickness <= 1e-12 or len(footprint) < 3:
            continue
        curved = any(abs(vertex["bulge"]) > 1e-12 for vertex in footprint)
        polygon, area, arcs = region(footprint)
        overhang = roof["overhang"]
        if abs(overhang) > 1e-12:
            assert not curved, "the oracle grows straight footprints only"
            polygon = polygon.buffer(overhang, join_style="mitre", mitre_limit=4.0)
            area = polygon.area
        z0 = levels[roof["storey"]][1] + roof["base_offset"]
        shape = roof["shape"]
        (name, body), = shape.items() if isinstance(shape, dict) else ((shape, {}),)
        convex = not curved and polygon.convex_hull.area - polygon.area < 1e-9
        fallback = None
        rise = 0.0
        ridge_length = None
        pitches = [body[key] for key in ("pitch", "lower_pitch", "upper_pitch") if key in body]
        if name in ("Shed", "Gable", "Hip", "Mansard") and not all(0.0 <= pitch < math.pi / 2 - 1e-6 for pitch in pitches):
            fallback = "roof.fallback-flat.invalid-pitch"
        elif name == "Shed":
            direction = body["direction"]
            projections = [x * math.cos(direction) + y * math.sin(direction) for x, y in polygon.exterior.coords]
            rise = math.tan(body["pitch"]) * (max(projections) - min(projections))
        elif name in ("Gable", "Hip", "Mansard") and not convex:
            fallback = "roof.fallback-flat.curved-footprint" if curved else "roof.fallback-flat.non-convex-footprint"
        elif name == "Gable":
            direction = body["ridge_direction"]
            projections = [-x * math.sin(direction) + y * math.cos(direction) for x, y in polygon.exterior.coords]
            rise = math.tan(body["pitch"]) * (max(projections) - min(projections)) / 2.0
            middle = (max(projections) + min(projections)) / 2.0
            far = 1e4
            ridge = LineString([(-far * math.cos(direction) - middle * math.sin(direction), -far * math.sin(direction) + middle * math.cos(direction)), (far * math.cos(direction) - middle * math.sin(direction), far * math.sin(direction) + middle * math.cos(direction))])
            ridge_length = polygon.intersection(ridge).length
        elif name == "Hip":
            depth = inscribed_distance(polygon)
            rise = math.tan(body["pitch"]) * depth
            slack = 0.01
            thin = polygon.buffer(-(depth - slack), join_style="mitre", mitre_limit=100.0).minimum_rotated_rectangle
            sides = sorted(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(thin.exterior.coords, thin.exterior.coords[1:]))
            ridge_length = sides[-1] - 2.0 * slack
        elif name == "Mansard":
            depth = inscribed_distance(polygon)
            lower, upper = math.tan(body["lower_pitch"]), math.tan(body["upper_pitch"])
            rise = min(lower * depth, body["break_height"] + upper * (depth - body["break_height"] / lower))
        min_x, min_y, max_x, max_y = xy_bounds(polygon)
        extra = {"fallback": fallback, "eave_z": z0}
        if ridge_length is not None:
            extra["ridge_length"] = ridge_length
            extra["ridge_length_tolerance"] = 1e-5
        rows[roof_id] = row("Roof", area * thickness, (min_x, min_y, z0, max_x, max_y, z0 + rise + thickness), arcs, thickness, arcs > 0, thickness=thickness, rise=rise, **extra)
    return rows


# endregion 🔖️Roofs


# region 🔖️Stairs
def sector(centre, inner, outer, start, stop):
    """🌙️ The polygon of an annulus sector sampled with `ARC_SAMPLES` chords."""
    angles = numpy.linspace(start, stop, ARC_SAMPLES + 1)
    outside = [(centre[0] + outer * math.cos(angle), centre[1] + outer * math.sin(angle)) for angle in angles]
    inside = [(centre[0] + inner * math.cos(angle), centre[1] + inner * math.sin(angle)) for angle in angles[::-1]] if inner > 1e-9 else [centre]
    return Polygon(outside + inside)


def oriented_rectangle(origin, angle, along, across):
    """▭️ A rectangle of `(from, to)` along and across a direction, as a shapely polygon."""
    rectangle = box(along[0], across[0], along[1], across[1])
    return affinity.translate(affinity.rotate(rectangle, angle, origin=(0, 0), use_radians=True), origin[0], origin[1])


def stair_rows(snapshot, levels):
    """🪜️ Stair riser law, prism volumes and bounds, re-derived from the documented `stair-runs` contract."""
    rows = {}
    for stair_id, stair in snapshot.get("stairs", {}).items():
        rows[stair_id] = None
        own = levels[stair["storey"]]
        base = own[0]
        top = resolved_top(stair["top"], base, own, target_level(stair["top"], levels))
        rise = top - base
        if not (rise > 1e-9 and stair["max_riser"] > 1e-9):
            continue
        count = max(1, math.ceil(rise / stair["max_riser"] - 1e-9))
        riser = rise / count
        tread = max(0.62 - 2.0 * riser, stair["min_tread"], 0.0)
        width, start, direction = stair["width"], (stair["start"]["x"], stair["start"]["y"]), stair["direction"]
        flight = stair["flight"]
        kind, body = next(iter(flight.items())) if isinstance(flight, dict) else (flight, {})
        pieces = []
        arcs = 0.0

        def along(origin, angle, distance):
            return (origin[0] + distance * math.cos(angle), origin[1] + distance * math.sin(angle))

        def run(first_riser, risers, origin, angle):
            floor = base + (first_riser - 1) * riser
            for index in range(risers - 1):
                pieces.append((oriented_rectangle(origin, angle, (index * tread, (index + 1) * tread), (-width / 2.0, width / 2.0)), floor + (index + 1) * riser))

        def landing(after_risers, centre, angle, wide, depth):
            pieces.append((oriented_rectangle(centre, angle, (-depth / 2.0, depth / 2.0), (-wide / 2.0, wide / 2.0)), base + after_risers * riser))

        if kind == "Spiral":
            outer = max(body["radius"], width)
            walking = outer - width / 2.0
            sign = -1.0 if body["sweep"] < 0 else 1.0
            centre = along(start, direction + math.pi / 2.0, sign * walking)
            start_angle = direction - sign * math.pi / 2.0
            treads = count - 1
            tread = abs(body["sweep"]) * walking / treads if treads else 0.0
            for index in range(treads):
                delta = body["sweep"] / treads
                pieces.append((sector(centre, max(outer - width, 0.0), outer, start_angle + delta * index, start_angle + delta * (index + 1)), base + (index + 1) * riser))
                arcs += (outer + max(outer - width, 0.0)) * abs(delta)
        elif kind == "LTurn" and count >= 2:
            first = min(max(math.floor(body["split"] * count + 0.5), 1), count - 1)
            sign = 1.0 if body["turn"] == "Left" else -1.0
            foot = along(start, direction, (first - 1) * tread)
            centre = along(foot, direction, width / 2.0)
            turned = direction + sign * math.pi / 2.0
            run(1, first, start, direction)
            landing(first, centre, direction, width, width)
            run(first + 1, count - first, along(centre, turned, width / 2.0), turned)
        elif kind == "UTurn" and count >= 2:
            gap = max(body["gap"], 0.0)
            first = -(-count // 2)
            foot = along(start, direction, (first - 1) * tread)
            second = along(foot, direction + math.pi / 2.0, width + gap)
            centre = along(along(foot, direction, width / 2.0), direction + math.pi / 2.0, (width + gap) / 2.0)
            run(1, first, start, direction)
            landing(first, centre, direction, 2.0 * width + gap, width)
            run(first + 1, count - first, second, direction + math.pi)
        else:
            run(1, count, start, direction)
        if not pieces:
            continue
        volume = math.fsum(piece.area * (height - base) for piece, height in pieces)
        union = shapely.union_all([piece for piece, _ in pieces])
        min_x, min_y, max_x, max_y = xy_bounds(union)
        highest = max(height for _, height in pieces)
        flat_area = math.fsum(piece.area for piece, _ in pieces)
        rows[stair_id] = row("Stair", volume, (min_x, min_y, base, max_x, max_y, highest), arcs, highest - base, arcs > 0, riser_count=count, riser_height=riser, tread=tread, steps=len(pieces), plan_area=flat_area)
    return rows


# endregion 🔖️Stairs


# region 🔖️Railings
def railing_rows(snapshot, levels):
    """🛤️ Railing post count, volume and bounds from `LineString` measures."""
    rows = {}
    for railing_id, railing in snapshot.get("railings", {}).items():
        rows[railing_id] = None
        points = []
        for point in railing["path"]:
            xy = (point["x"], point["y"])
            if not points or math.hypot(xy[0] - points[-1][0], xy[1] - points[-1][1]) > 1e-9:
                points.append(xy)
        if len(points) < 2 or railing["height"] < 1e-9:
            continue
        base = levels[railing["storey"]][0] + railing["base_offset"]
        top = base + railing["height"]
        path = LineString(points)
        spacing = railing["post_spacing"]
        posts = []
        for index in range(len(points) - 1):
            segment = LineString([points[index], points[index + 1]])
            divisions = max(1, math.ceil(segment.length / spacing - 1e-9)) if spacing > 1e-9 else 1
            angle = math.atan2(points[index + 1][1] - points[index][1], points[index + 1][0] - points[index][0])
            for step in range(0 if index == 0 else 1, divisions + 1):
                at = segment.interpolate(segment.length * step / divisions)
                posts.append(affinity.translate(affinity.rotate(box(-POST_SIZE / 2, -POST_SIZE / 2, POST_SIZE / 2, POST_SIZE / 2), angle, origin=(0, 0), use_radians=True), at.x, at.y))
        post_height = railing["height"] - RAIL_DEPTH
        post_volume = len(posts) * POST_SIZE * POST_SIZE * post_height if post_height > 1e-9 else 0.0
        rail_volume = RAIL_WIDTH * RAIL_DEPTH * path.length
        rail_footprint = path.buffer(RAIL_WIDTH / 2.0, cap_style="flat", join_style="mitre", mitre_limit=10.0)
        footprint = shapely.union_all([rail_footprint] + (posts if post_height > 1e-9 else []))
        min_x, min_y, max_x, max_y = xy_bounds(footprint)
        low = base if post_height > 1e-9 else top - RAIL_DEPTH
        rows[railing_id] = row("Railing", post_volume + rail_volume, (min_x, min_y, low, max_x, max_y, top), 0.0, railing["height"], False, posts=len(posts) if post_height > 1e-9 else 0, length=path.length)
    return rows


# endregion 🔖️Railings


# region 🔖️Tables
FAMILIES = {"columns": column_rows, "beams": beam_rows, "slabs": slab_rows, "roofs": roof_rows, "stairs": stair_rows, "railings": railing_rows}


def expected_of(case_name, snapshot):
    """🧊️ The expected table of one case, computed from its snapshot alone."""
    return FAMILIES[CASES[case_name]](snapshot, storey_levels(snapshot))


def canonical(value):
    """🧹️ The JSON form committed to the fixture: floats rounded to 12 decimals so reruns are byte stable."""
    if isinstance(value, float):
        return round(value, 12)
    if isinstance(value, dict):
        return {key: canonical(item) for key, item in value.items()}
    if isinstance(value, list):
        return [canonical(item) for item in value]
    return value


# endregion 🔖️Tables


# region 🔖️Handlers
def planar_projection(expected):
    """📋️ `{element id: {volume, min, max}}` of the planar parts: the projection both roles of the platform case compare."""
    return {key: {"volume": row["volume"], "min": row["min"], "max": row["max"]} for key, row in expected.items() if row is not None and not row["curved"]}


def solids_handler(ctx):
    """🧊️ Oracle answer: the planar table of every case named by the scenario's fixtures."""
    from semio_repo_test import Outcome

    payload = {}
    for uri in ctx.step_input_uris():
        case = next((name for name in CASES if "/%s/" % name in uri), None)
        if case:
            document = json.loads(ctx.input_bytes(uri).decode("utf-8"))
            payload[case] = planar_projection(expected_of(case, document["snapshot"]))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("solids-rest", solids_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `write` rewrites the `expected` member of every case; `check` fails on any drift from the committed table."""
    mode, root = arguments[0], Path(arguments[1])
    failures = []
    for name in CASES:
        file = root / name / "🔣️.json"
        document = json.loads(file.read_text(encoding="utf-8"))
        table = canonical(expected_of(name, document["snapshot"]))
        if mode == "write":
            document["expected"] = table
            file.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
            print("%s: wrote %d elements (%d absent)" % (name, len(table), sum(1 for row in table.values() if row is None)))
        elif document["expected"] != table:
            failures.append(name)
            print("[FAIL] %s: committed expected differs from the oracle (shapely %s, numpy %s)" % (name, shapely.__version__, numpy.__version__))
        else:
            print("%s: shapely %s agrees on %d elements" % (name, shapely.__version__, len(table)))
    print("%s: %s" % (mode, "%d problem(s)" % len(failures) if failures else "ok"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
