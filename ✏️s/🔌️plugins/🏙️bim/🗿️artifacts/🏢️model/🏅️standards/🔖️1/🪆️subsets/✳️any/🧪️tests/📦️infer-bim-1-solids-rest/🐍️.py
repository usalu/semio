#!/usr/bin/env python3
"""🧊️ Third-party ORACLE (shapely 2 over GEOS, numpy) for the column, beam, slab, ceiling, roof, stair and railing solids of `🧊️element-solids`.

Each case under `🧫️fixtures/💡️inferences/🧊️element-solids/` named in `CASES` holds an authored `snapshot` and an `expected` table written by this file from the
snapshot alone. The oracle never sees the subject's meshes: it re-derives every volume and every axis-aligned bound from closed forms and from `shapely`
(`Polygon.area`, `bounds`, `buffer` with mitre joins, `affinity`, `convex_hull`, `LineString.length`), so the subject's tessellation is measured against a library that has
never seen this repository.

* columns: `area(profile) * height`, bounds of the rotated, translated profile polygon between the resolved base and top;
* beams: `area(profile) * length`, bounds of the swept section rectangle with the profile dropped below the beam top;
* slabs: `area(boundary - holes) * sum(layers)`, bounds of the boundary and the top plane range of a slope;
* ceilings: the same prism, hung `offset` below the top of the storey instead of standing on its floor;
* roofs: `area(eave) * sum(layers)` with the eave from a mitred `buffer`, the rise from closed forms audited by `buffer(-d)` bisection (hip and mansard over any simple straight footprint,
  concave L, T and U included), the extent (gable, shed), and the fallback decision (curved footprint, invalid pitch, gable ends next to a reflex corner);
* stairs: the tread slabs, riser boards, stringer bands (clipped with `shapely`) and landing slabs of the runs of the sibling stair-runs oracle, volumes, per-part volumes and bounds from `shapely`;
* railings: the authored rail and post sections, posts, balusters and infill slabs from `LineString` measures and `shapely` footprints.

An element whose geometry is degenerate is `null` in `expected` (the subject must not emit a solid for it). `volume_tolerance` and `bounds_tolerance` are the exact tolerance
the tessellated arcs allow: `1e-9` for planar parts, otherwise the arc length times the sagitta `CHORD_TOLERANCE`.

Standalone use (no test host needed):

    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧊️element-solids>
    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧊️element-solids>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🧬️schema/💡️inferences/🧊️element-solids — the subject
"""

# region 🔖️Imports
import importlib.util
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

ARC_SAMPLES = 4096
"""🌙️ Chords per arc when the oracle samples a bulged edge for `shapely`."""

CASES = {
    "columns-profiles": "columns",
    "beams-profiles": "beams",
    "frame-tilt-joins": "frame",
    "slabs-holes-slope": "slabs",
    "ceilings-holes-slope": "ceilings",
    "ceilings-meshes": "ceilings",
    "roofs-shapes": "roofs",
    "stairs-flights": "stairs",
    "railings-posts": "railings",
}
"""🗂️ The fixture cases of these families and the collection each one measures."""
# endregion 🔖️Constants


# region 🔖️Siblings
def load_sibling(name):
    """🧭️ Imports the oracle module of the sibling case whose folder ends in `infer-bim-1-<name>` by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        folder = next(Path(__file__).resolve().parents[1].glob("*infer-bim-1-" + name))
        spec = importlib.util.spec_from_file_location(key, folder / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


# endregion 🔖️Siblings


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
def lean_of(column):
    """📐️ `((ux, uy), stretch, slope)` of a leaning column (the unit direction in plan, `1 / cos(angle)`, `tan(angle)`), `None` for a plumb one."""
    tilt = column.get("tilt")
    if not tilt or not math.isfinite(tilt["angle"]) or abs(tilt["angle"]) <= 1e-12 or abs(tilt["angle"]) >= math.pi / 2.0:
        return None
    return (math.cos(tilt["direction"]), math.sin(tilt["direction"])), 1.0 / math.cos(tilt["angle"]), math.tan(tilt["angle"])


def column_footprint(column, polygon, base, z):
    """🔷️ The horizontal section of a column at the height `z`: the profile rotated and placed; a leaning column stretches it by `1 / cos(angle)` along the lean and moves it by `(z - base) * tan(angle)` along it (a shear, `shapely.affinity`)."""
    position = (column["position"]["x"], column["position"]["y"])
    placed = affinity.translate(affinity.rotate(polygon, column["rotation"], origin=(0, 0), use_radians=True), *position)
    lean = lean_of(column)
    if lean is None:
        return placed
    (ux, uy), stretch, slope = lean
    k = stretch - 1.0
    sheared = affinity.affine_transform(affinity.translate(placed, -position[0], -position[1]), [1.0 + k * ux * ux, k * ux * uy, k * ux * uy, 1.0 + k * uy * uy, 0.0, 0.0])
    return affinity.translate(sheared, position[0] + (z - base) * slope * ux, position[1] + (z - base) * slope * uy)


def column_rows(snapshot, levels):
    """🏛️ Column volume and bounds; a leaning column is the sheared prism between its stretched base section and the same section moved along the lean (volume `area * rise / cos(angle)`)."""
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
        lean = lean_of(column)
        low, high = column_footprint(column, polygon, base, base), column_footprint(column, polygon, base, top)
        min_x, min_y, max_x, max_y = low.union(high).bounds if lean else xy_bounds(low)
        volume = area * (top - base) * (lean[1] if lean else 1.0)
        rows[column_id] = row("Column", volume, (min_x, min_y, base, max_x, max_y, top), arcs, top - base, arcs > 0, height=top - base, base_z=base, top_z=top)
    return rows


# endregion 🔖️Columns


# region 🔖️Beams
def axis_stations(axis):
    """〰️ `(points, length)` of a beam axis: the two ends of a line, or the arc cut into `ARC_SAMPLES` chords (`shapely` measures the polyline)."""
    (kind, body), = axis.items()
    start, end = (body["start"]["x"], body["start"]["y"]), (body["end"]["x"], body["end"]["y"])
    if kind == "Line":
        return [start, end], math.dist(start, end)
    centre, radius, sweep = arc(start, end, body["bulge"])
    first = math.atan2(start[1] - centre[1], start[0] - centre[0])
    points = [(centre[0] + radius * math.cos(first + sweep * i / ARC_SAMPLES), centre[1] + radius * math.sin(first + sweep * i / ARC_SAMPLES)) for i in range(ARC_SAMPLES + 1)]
    points[0], points[-1] = start, end
    return points, abs(sweep) * radius


def beam_cuts(snapshot, levels, beam, points, depth, tops):
    """🔗️ How far the axis is cut back at each end by the columns of its storey whose vertical extent holds the mid-depth of the beam there: the piece of the axis inside the column sections (`shapely` intersection) that starts at the end."""
    from shapely.geometry import Point
    from shapely.ops import unary_union

    cuts = []
    for index, line in enumerate((LineString(points), LineString(points[::-1]))):
        middle = tops[index] - depth / 2.0
        rings = []
        for column in snapshot.get("columns", {}).values():
            kind = snapshot.get("column_types", {}).get(column["column_type"])
            section = profile_region(kind["profile"]) if kind and column["storey"] == beam["storey"] else None
            if section is None:
                continue
            own = levels[column["storey"]]
            base = own[0] + column["base_offset"]
            top = resolved_top(column["top"], base, own, target_level(column["top"], levels))
            if base - EXACT <= middle <= top + EXACT:
                rings.append(column_footprint(column, section[0], base, middle))
        piece = 0.0
        if rings:
            found = line.intersection(unary_union(rings))
            for part in getattr(found, "geoms", [found]):
                if part.geom_type == "LineString" and part.distance(Point(line.coords[0])) < EXACT:
                    piece = max(piece, part.length)
        cuts.append(piece)
    return cuts


def beam_rows(snapshot, levels):
    """➖️ Beam volume and bounds: the profile hangs from the reference line and is swept along the axis (a line or an arc), perpendicular to the climbing axis of an inclined beam, cut back to the faces of the columns it joins; the volume is the area of the profile times the axis length between the cuts (Pappus), the bounds those of the swept corners of the hull of the profile."""
    rows = {}
    for beam_id, beam in snapshot.get("beams", {}).items():
        rows[beam_id] = None
        kind = snapshot.get("beam_types", {}).get(beam["beam_type"])
        section = profile_region(kind["profile"]) if kind else None
        points, length = axis_stations(beam["axis"])
        if section is None or length <= EXACT:
            continue
        polygon, area, arcs = section
        max_v = polygon.bounds[3]
        depth = max_v - polygon.bounds[1]
        storey_top = levels[beam["storey"]][1]
        top = storey_top + beam["top_offset"]
        end_top = storey_top + (beam["top_offset"] if beam.get("end_top_offset") is None else beam["end_top_offset"])
        cuts = beam_cuts(snapshot, levels, beam, points, depth, (top, end_top))
        if cuts[0] + cuts[1] >= length - 1e-6:
            cuts = [0.0, 0.0]
        rise = end_top - top
        slope = math.atan2(rise, length)
        sine, cosine = math.sin(slope), math.cos(slope)
        cloud = numpy.array(points)
        steps = numpy.diff(cloud, axis=0)
        sizes = numpy.hypot(steps[:, 0], steps[:, 1])
        walked = numpy.concatenate([[0.0], numpy.cumsum(sizes)])
        curved_axis = len(points) > 2
        distances = [cuts[0] + (length - cuts[0] - cuts[1]) * step / ARC_SAMPLES for step in range(ARC_SAMPLES + 1)] if curved_axis else [cuts[0], length - cuts[1]]
        corners = list(polygon.convex_hull.simplify(1e-12).exterior.coords)
        lows, highs = [math.inf] * 3, [-math.inf] * 3
        for distance in distances:
            index = min(max(int(numpy.searchsorted(walked, min(distance, walked[-1]), side="right")) - 1, 0), len(steps) - 1)
            fraction = (min(distance, walked[-1]) - walked[index]) / sizes[index]
            station = cloud[index] + steps[index] * fraction
            tangent = steps[index] / sizes[index]
            lateral = (-tangent[1], tangent[0])
            for u, v in corners:
                v -= max_v
                values = (station[0] + lateral[0] * u - tangent[0] * v * sine, station[1] + lateral[1] * u - tangent[1] * v * sine, top + rise * distance / length + v * cosine)
                for axis_index, value in enumerate(values):
                    lows[axis_index], highs[axis_index] = min(lows[axis_index], value), max(highs[axis_index], value)
        solid_length = (length - cuts[0] - cuts[1]) / math.cos(slope)
        made = row("Beam", area * solid_length, (lows[0], lows[1], lows[2], highs[0], highs[1], highs[2]), arcs, solid_length, curved_axis or arcs > 0, bounds_scale=3.0 if curved_axis else 2.0, length=length, top_z=top)
        if curved_axis:
            radius = arc(points[0], points[-1], next(iter(beam["axis"].values()))["bulge"])[1]
            made["volume_tolerance"] = max(made["volume_tolerance"], 3.0 * area * solid_length * CHORD_TOLERANCE / radius)
            made["curved"] = True
        rows[beam_id] = made
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


# region 🔖️Ceilings
def ceiling_rows(snapshot, levels):
    """🔲️ Ceiling volume and bounds: the layers hang from `storey top - offset`, a slope tilts the plane about the uphill edge."""
    rows = {}
    for ceiling_id, ceiling in snapshot.get("ceilings", {}).items():
        rows[ceiling_id] = None
        kind = snapshot.get("ceiling_types", {}).get(ceiling["ceiling_type"])
        thickness = math.fsum(max(layer["thickness"], 0.0) for layer in kind["layers"]) if kind else 0.0
        if thickness <= 1e-12 or len(ceiling["boundary"]) < 3:
            continue
        polygon, area, arcs = region(ceiling["boundary"], ceiling.get("holes", []))
        top = levels[ceiling["storey"]][1] - ceiling["offset"]
        fall = 0.0
        if ceiling.get("slope"):
            direction = ceiling["slope"]["direction"]
            projections = [x * math.cos(direction) + y * math.sin(direction) for x, y in polygon.exterior.coords]
            fall = math.tan(ceiling["slope"]["angle"]) * (max(projections) - min(projections))
        min_x, min_y, max_x, max_y = xy_bounds(polygon)
        rows[ceiling_id] = row("Ceiling", area * thickness, (min_x, min_y, top - fall - thickness, max_x, max_y, top), arcs, thickness, arcs > 0, thickness=thickness, top_z=top, fall=fall)
    return rows


# endregion 🔖️Ceilings


# region 🔖️Roofs
SKELETON_TOLERANCE = 1e-7
"""⚖️ Tolerance of the corners of a skeleton roof: the framework merges event points within 1e-9 of the extent, which moves areas and heights by about 1e-8."""

GABLE_END_TOLERANCE = 0.02
"""📐️ Largest sine of the angle between an edge and the perpendicular of the ridge for which the edge is a gable end (`roof::GABLE_END_TOLERANCE`)."""


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


def inset_depth(polygon):
    """🧭️ The largest mitred inward offset `d` that leaves anything of a simple straight polygon: the time the straight skeleton needs to collapse it, which is the latest node time of the independent `py_straight_skeleton` library; audited by bisection on `shapely`'s mitred negative buffer (which drops slivers thinner than about 1e-3) and by the linear program on convex polygons."""
    from py_straight_skeleton import compute_skeleton

    peak = max(node.time for node in compute_skeleton(exterior=list(orient(polygon, 1.0).exterior.coords[:-1]), holes=[]).nodes)
    low, high = 0.0, max(polygon.bounds[2] - polygon.bounds[0], polygon.bounds[3] - polygon.bounds[1])
    for _ in range(60):
        middle = (low + high) / 2.0
        if polygon.buffer(-middle, join_style="mitre", mitre_limit=100.0).is_empty:
            high = middle
        else:
            low = middle
    assert 0.0 <= peak - low < 1e-3, "the skeleton collapses at %r, the mitred buffer at %r" % (peak, low)
    if abs(polygon.convex_hull.area - polygon.area) < 1e-9:
        assert abs(peak - inscribed_distance(polygon)) < 1e-9, "the skeleton %r vs the linear program %r" % (peak, inscribed_distance(polygon))
    return peak


def reflex_gable_end(polygon, ridge_direction):
    """⚠️ Whether a gable end (an edge across the ridge within the tolerance) has a reflex corner at either end, which would make the skeleton surface step."""
    ring = list(orient(polygon, 1.0).exterior.coords[:-1])
    count = len(ring)
    sin, cos = math.sin(ridge_direction), math.cos(ridge_direction)

    def reflex(index):
        a, b, c = ring[index - 1], ring[index], ring[(index + 1) % count]
        return (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]) < -1e-9

    for index in range(count):
        a, b = ring[index], ring[(index + 1) % count]
        length = math.hypot(b[0] - a[0], b[1] - a[1])
        along = ((b[0] - a[0]) * cos + (b[1] - a[1]) * sin) / length
        if abs(along) <= GABLE_END_TOLERANCE and (reflex(index) or reflex((index + 1) % count)):
            return True
    return False


def roof_rows(snapshot, levels):
    """🏠️ Roof volume, bounds, fallback, rise and ridge length. A pitched roof over a straight simple footprint is the weighted straight skeleton read as heights: its rise is `tan(pitch)` times the mitred collapse depth (piecewise for a mansard), its volume the eave area times the layers."""
    rows = {}
    for roof_id, roof in snapshot.get("roofs", {}).items():
        rows[roof_id] = None
        kind = snapshot.get("roof_types", {}).get(roof["roof_type"])
        thickness = math.fsum(max(layer["thickness"], 0.0) for layer in kind["layers"]) if kind else 0.0
        footprint = roof["footprint"]
        if thickness <= 1e-12 or len(footprint) < 3:
            continue
        curved = any(abs(vertex["bulge"]) > 1e-12 for vertex in footprint)
        if not curved and not Polygon([(vertex["point"]["x"], vertex["point"]["y"]) for vertex in footprint]).is_valid:
            continue
        polygon, area, arcs = region(footprint)
        overhang = roof["overhang"]
        if abs(overhang) > 1e-12:
            assert not curved, "the oracle grows straight footprints only"
            polygon = polygon.buffer(overhang, join_style="mitre", mitre_limit=4.0)
            area = polygon.area
        z0 = levels[roof["storey"]][1] + roof["base_offset"]
        shape = roof["shape"]
        (name, body), = shape.items() if isinstance(shape, dict) else ((shape, {}),)
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
        elif name == "Mansard" and not (body["lower_pitch"] > 0 and body["upper_pitch"] > 0 and body["break_height"] > 0):
            fallback = "roof.fallback-flat.invalid-pitch"
        elif name in ("Gable", "Hip", "Mansard") and curved:
            fallback = "roof.fallback-flat.curved-footprint"
        elif name in ("Gable", "Hip", "Mansard") and not polygon.is_valid:
            continue
        elif name in ("Gable", "Hip") and body["pitch"] == 0.0:
            rise = 0.0
        elif name == "Gable" and not reflex_gable_end(polygon, body["ridge_direction"]):
            direction = body["ridge_direction"]
            projections = [-x * math.sin(direction) + y * math.cos(direction) for x, y in polygon.exterior.coords]
            rise = math.tan(body["pitch"]) * (max(projections) - min(projections)) / 2.0
            middle = (max(projections) + min(projections)) / 2.0
            far = 1e4
            ridge = LineString([(-far * math.cos(direction) - middle * math.sin(direction), -far * math.sin(direction) + middle * math.cos(direction)), (far * math.cos(direction) - middle * math.sin(direction), far * math.sin(direction) + middle * math.cos(direction))])
            ridge_length = polygon.intersection(ridge).length
        elif name in ("Gable", "Hip"):
            if name == "Gable":
                fallback = "roof.gable-ends-adjust-to-hip"
            depth = inset_depth(polygon)
            rise = math.tan(body["pitch"]) * depth
            if abs(polygon.convex_hull.area - polygon.area) < 1e-9:
                slack = 0.01
                thin = polygon.buffer(-(depth - slack), join_style="mitre", mitre_limit=100.0).minimum_rotated_rectangle
                sides = sorted(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(thin.exterior.coords, thin.exterior.coords[1:]))
                ridge_length = sides[-1] - 2.0 * slack
        elif name == "Mansard":
            depth = inset_depth(polygon)
            lower, upper = math.tan(body["lower_pitch"]), math.tan(body["upper_pitch"])
            rise = min(lower * depth, body["break_height"] + upper * (depth - body["break_height"] / lower))
        min_x, min_y, max_x, max_y = xy_bounds(polygon)
        extra = {"fallback": fallback, "eave_z": z0}
        if name in ("Hip", "Gable") and rise > 0 and fallback != "roof.fallback-flat.invalid-pitch":
            extra["slope_area"] = area / math.cos(body["pitch"])
        if ridge_length is not None:
            extra["ridge_length"] = ridge_length
            extra["ridge_length_tolerance"] = 1e-5
        rows[roof_id] = row("Roof", area * thickness, (min_x, min_y, z0, max_x, max_y, z0 + rise + thickness), arcs, thickness, arcs > 0, thickness=thickness, rise=rise, **extra)
        if name in ("Gable", "Hip", "Mansard") and rise > 0:
            rows[roof_id]["volume_tolerance"] = max(rows[roof_id]["volume_tolerance"], SKELETON_TOLERANCE)
            rows[roof_id]["bounds_tolerance"] = max(rows[roof_id]["bounds_tolerance"], SKELETON_TOLERANCE)
    return rows


# endregion 🔖️Roofs


# region 🔖️Stairs
HUGE = 1e3
"""♾️ Half-extent of the clip boxes of the stringer bands, in metres."""


def sector(centre, inner, outer, start, stop):
    """🌙️ The polygon of an annulus sector sampled with `ARC_SAMPLES` chords."""
    angles = numpy.linspace(start, stop, ARC_SAMPLES + 1)
    outside = [(centre[0] + outer * math.cos(angle), centre[1] + outer * math.sin(angle)) for angle in angles]
    inside = [(centre[0] + inner * math.cos(angle), centre[1] + inner * math.sin(angle)) for angle in angles[::-1]] if inner > 1e-9 else [centre]
    return Polygon(outside + inside)


def placed_polygon(polygon, origin, angle):
    """🧭️ A polygon of a flight frame (`u` along the travel, `w` across) rotated by the travel angle and moved to the foot of the flight."""
    return affinity.translate(affinity.rotate(polygon, angle, origin=(0, 0), use_radians=True), origin[0], origin[1])


def stringer_side(kind, going, rise, thickness, nosing, treads, closed, depth):
    """🪵️ The side profile `(u, z)` of one stringer board as a shapely polygon, or `None`: the band of vertical `depth` under the pitch line through the noses (closed, cut by the floor and the arrival level), under the saw tooth of the tread undersides (open) or under the line through the back lower corners of the treads (mono), all cut by the floor."""
    if kind == "None" or treads <= 0:
        return None
    slope, end, level = rise / going, treads * going, (treads + 1) * rise
    floor = box(-HUGE, 0.0, HUGE, HUGE)

    def banded(start, line):
        return Polygon([(start, line(start)), (end, line(end)), (end, line(end) - depth), (start, line(start) - depth)])

    if kind == "Closed":
        polygon = banded(-nosing, lambda u: rise + (u + nosing) * slope).intersection(floor).intersection(box(-HUGE, -HUGE, HUGE, level))
    elif kind == "Mono":
        polygon = banded(0.0, lambda u: u * slope - thickness).intersection(floor)
    else:
        lead = thickness if closed else 0.0
        outline = []
        for tread in range(treads):
            z = (tread + 1) * rise - thickness
            outline += [(tread * going + lead, z), (min((tread + 1) * going + lead, end), z)]
        outline += [(end, (end - lead) * slope - thickness - depth), (lead, -thickness - depth)]
        polygon = Polygon(outline).intersection(floor)
    return polygon if polygon.area > EXACT else None


def stringer_boards(kind, stringer_width, width):
    """🪵️ The across ranges `(from, to)` of the boards of a stringer."""
    half = width / 2.0
    return {"None": [], "Closed": [(half, half + stringer_width), (-half - stringer_width, -half)], "Open": [(half - stringer_width, half), (-half, -half + stringer_width)], "Mono": [(-stringer_width / 2.0, stringer_width / 2.0)]}[kind]


def straight_parts(stair, run, flight, landed):
    """🪜️ `[(part, plan polygon, z from, z to, volume)]` of one straight flight: tread slabs, riser boards (closed risers) and stringer boards."""
    going, rise, thickness, nosing = flight["tread"], run["riser_height"], stair["tread_thickness"], stair["nosing"]
    treads, width, base = flight["treads"], run["width"], flight["base_z"]
    origin, angle, half = (flight["start"]["x"], flight["start"]["y"]), flight["direction"], run["width"] / 2.0
    parts = []

    def block(name, u, z):
        polygon = placed_polygon(box(u[0], -half, u[1], half), origin, angle)
        parts.append((name, polygon, base + z[0], base + z[1], (u[1] - u[0]) * width * (z[1] - z[0])))

    for tread in range(treads):
        top = (tread + 1) * rise
        block("step", (tread * going - nosing, (tread + 1) * going), (top - thickness, top))
    if stair["riser"] == "Closed":
        for riser in range(treads + (1 if landed else 0)):
            block("riser", (riser * going, riser * going + thickness), (riser * rise, (riser + 1) * rise - thickness))
    side = stringer_side(stair["stringer"]["kind"], going, rise, thickness, nosing, treads, stair["riser"] == "Closed", stair["stringer"]["depth"])
    if side is not None:
        low_u, low_z, high_u, high_z = side.bounds
        for across in stringer_boards(stair["stringer"]["kind"], stair["stringer"]["width"], width):
            polygon = placed_polygon(box(low_u, across[0], high_u, across[1]), origin, angle)
            parts.append(("stringer", polygon, base + low_z, base + high_z, side.area * (across[1] - across[0])))
    return parts


def winding_parts(stair, run, flight):
    """🌀️ `([(part, plan polygon, z from, z to, volume)], arc weight)` of a winding flight: one wedge slab per tread; the stringer and the risers are left out."""
    winder, treads = flight["winder"], flight["treads"]
    centre = (winder["centre"]["x"], winder["centre"]["y"])
    parts, arcs = [], 0.0
    for index in range(treads):
        delta = winder["sweep"] / max(treads, 1)
        wedge = sector(centre, winder["inner_radius"], winder["outer_radius"], winder["start_angle"] + delta * index, winder["start_angle"] + delta * (index + 1))
        top = flight["base_z"] + (index + 1) * run["riser_height"]
        parts.append(("step", wedge, top - stair["tread_thickness"], top, wedge.area * stair["tread_thickness"]))
        arcs += (winder["outer_radius"] + winder["inner_radius"]) * abs(delta)
    return parts, arcs


def landing_part(stair, landing):
    """🟫️ The slab of a landing: `depth` along its direction, `width` across, as thick as a tread, its top at the landing height."""
    polygon = placed_polygon(box(-landing["depth"] / 2.0, -landing["width"] / 2.0, landing["depth"] / 2.0, landing["width"] / 2.0), (landing["centre"]["x"], landing["centre"]["y"]), landing["direction"])
    return ("landing", polygon, landing["z"] - stair["tread_thickness"], landing["z"], landing["depth"] * landing["width"] * stair["tread_thickness"])


def stair_rows(snapshot, levels):
    """🪜️ Stair volume, bounds and per-part volumes, re-derived from the stair-runs oracle's runs and the documented construction of treads, risers, stringers and landings."""
    rows = {}
    runs = load_sibling("stair-runs").table(snapshot)
    for stair_id, stair in snapshot.get("stairs", {}).items():
        rows[stair_id] = None
        run = runs.get(stair_id)
        if run is None or not (run["rise"] > 1e-9 and stair["max_riser"] > 1e-9):
            continue
        parts, arcs = [], 0.0
        for index, flight in enumerate(run["flights"]):
            if "winder" in flight:
                found, weight = winding_parts(stair, run, flight)
                parts += found
                arcs += weight
            else:
                parts += straight_parts(stair, run, flight, index + 1 < len(run["flights"]))
        parts += [landing_part(stair, landing) for landing in run["landings"]]
        if not parts:
            continue
        union = shapely.union_all([polygon for _, polygon, _, _, _ in parts])
        min_x, min_y, max_x, max_y = xy_bounds(union)
        low, high = min(part[2] for part in parts), max(part[3] for part in parts)
        volumes = {}
        for name, _, _, _, volume in parts:
            volumes[name] = volumes.get(name, 0.0) + volume
        treads = [part for part in parts if part[0] in ("step", "landing")]
        spiral = any("winder" in flight for flight in run["flights"])
        ignored = spiral and stair["stringer"]["kind"] != "None"
        rows[stair_id] = row(
            "Stair", math.fsum(volumes.values()), (min_x, min_y, low, max_x, max_y, high), arcs, stair["tread_thickness"], arcs > 0,
            riser_count=run["riser_count"], riser_height=run["riser_height"], tread=run["tread"], steps=len(treads), plan_area=math.fsum(part[1].area for part in treads),
            parts=volumes, diagnostic="stair.stringer-ignored-on-spiral" if ignored else None,
        )
    return rows


# endregion 🔖️Stairs


# region 🔖️Railings
INFILL_FOOT = 0.05
"""⬆️ Height of the underside of an infill above the base of the railing (`railings::INFILL_FOOT`)."""


def divisions(length, spacing):
    """🔢️ The equal parts a length is divided into: the fewest no longer than the spacing, at least one; no spacing leaves one."""
    return max(1, math.ceil(length / spacing - 1e-9)) if spacing > 1e-9 else 1


def rotated_at(polygon, at, angle):
    """🧭️ A centred profile polygon whose `x` runs along `angle`, moved to `at`."""
    return affinity.translate(affinity.rotate(polygon, angle, origin=(0, 0), use_radians=True), at.x, at.y)


def railing_rows(snapshot, levels):
    """🛤️ Railing volume, bounds and per-part volumes from `LineString` measures: the authored rail swept along the path with its highest point on the top, posts at every vertex and equal subdivisions, balusters strictly inside every bay and an infill slab per bay."""
    rows = {}
    for railing_id, railing in snapshot.get("railings", {}).items():
        rows[railing_id] = None
        points = []
        for point in railing["path"]:
            xy = (point["x"], point["y"])
            if not points or math.hypot(xy[0] - points[-1][0], xy[1] - points[-1][1]) > 1e-9:
                points.append(xy)
        rail = profile_region(railing["profile"])
        post = profile_region(railing["post_profile"])
        if len(points) < 2 or railing["height"] < 1e-9 or rail is None or post is None:
            continue
        base = levels[railing["storey"]][0] + railing["base_offset"]
        top = base + railing["height"]
        path = LineString(points)
        rail_polygon, rail_area, rail_arcs = rail
        rail_across = max(abs(rail_polygon.bounds[0]), abs(rail_polygon.bounds[2]))
        rail_height = rail_polygon.bounds[3] - rail_polygon.bounds[1]
        underside = top - rail_height
        stand = underside - base > 1e-9
        bays, positions = [], []
        for index in range(len(points) - 1):
            segment = LineString([points[index], points[index + 1]])
            count = divisions(segment.length, railing["post_spacing"])
            angle = math.atan2(points[index + 1][1] - points[index][1], points[index + 1][0] - points[index][0])
            stations = [segment.interpolate(segment.length * step / count) for step in range(count + 1)]
            positions += [(at, angle) for at in stations[(0 if index == 0 else 1):]]
            bays += [(LineString([stations[step], stations[step + 1]]), angle) for step in range(count)]
        footprints = [path.buffer(rail_across, cap_style="flat", join_style="mitre", mitre_limit=10.0)]
        volumes = {"rail": rail_area * path.length, "post": 0.0, "baluster": 0.0, "infill": 0.0}
        weights = rail_arcs * path.length
        balusters, infill_area = 0, 0.0
        if stand:
            post_polygon, post_area, post_arcs = post
            footprints += [rotated_at(post_polygon, at, angle) for at, angle in positions]
            volumes["post"] = len(positions) * post_area * (underside - base)
            weights += len(positions) * post_arcs * (underside - base)
            if railing.get("baluster"):
                profile = profile_region(railing["baluster"]["profile"])
                for bay, angle in bays:
                    count = divisions(bay.length, railing["baluster"]["spacing"])
                    for step in range(1, count):
                        footprints.append(rotated_at(profile[0], bay.interpolate(bay.length * step / count), angle))
                        balusters += 1
                volumes["baluster"] = balusters * profile[1] * (underside - base)
                weights += balusters * profile[2] * (underside - base)
            infill = railing["infill"]
            kind, body = next(iter(infill.items())) if isinstance(infill, dict) else (infill, {})
            if kind in ("Glass", "Panel"):
                inset, foot = post_polygon.bounds[2] - post_polygon.bounds[0], base + INFILL_FOOT
                inset /= 2.0
                for bay, angle in bays:
                    reach = bay.length - 2.0 * inset
                    if reach > 1e-9 and underside - foot > 1e-9:
                        slab = box(-reach / 2.0, -body["thickness"] / 2.0, reach / 2.0, body["thickness"] / 2.0)
                        footprints.append(rotated_at(slab, bay.interpolate(bay.length / 2.0), angle))
                        infill_area += reach * (underside - foot)
                volumes["infill"] = infill_area * body["thickness"]
        union = shapely.union_all(footprints)
        min_x, min_y, max_x, max_y = xy_bounds(union)
        low = base if stand else underside
        curved = weights > 0
        rows[railing_id] = row(
            "Railing", math.fsum(volumes.values()), (min_x, min_y, low, max_x, max_y, top), weights, 1.0, curved, bounds_scale=2.0,
            posts=len(positions) if stand else 0, balusters=balusters, infill_area=infill_area, length=path.length, parts=volumes,
        )
    return rows


# endregion 🔖️Railings


# region 🔖️Tables
FAMILIES = {"columns": column_rows, "beams": beam_rows, "frame": lambda snapshot, levels: {**column_rows(snapshot, levels), **beam_rows(snapshot, levels)}, "slabs": slab_rows, "ceilings": ceiling_rows, "roofs": roof_rows, "stairs": stair_rows, "railings": railing_rows}


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
        case = next((name for name in CASES if uri.rstrip("/").split("/")[-2].endswith(name)), None)
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
        file = next(root.glob("*%s" % name)) / "🔣️.json"
        document = json.loads(file.read_text(encoding="utf-8"))
        table = canonical(expected_of(name, document["snapshot"]))
        if mode == "write":
            document["expected"] = table
            file.unlink()
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
