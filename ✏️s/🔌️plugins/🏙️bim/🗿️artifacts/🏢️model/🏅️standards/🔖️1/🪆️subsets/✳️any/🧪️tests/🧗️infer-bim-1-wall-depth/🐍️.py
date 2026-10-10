#!/usr/bin/env python3
"""🧗️ Third-party ORACLE (shapely 2 over GEOS) for the wall depth of `s.bim.model@1`: walls attached to roofs and slabs, wall sweeps and authored opening reveals.

The subject (Rust, `semio-s-artifact-bim-model`) infers, for every wall whose top or base is attached, the elevation edges along its axis (`base_z`, `top_z`, the area of the elevation
`side_area` and, for a free wall of constant thickness, the `volume`); for every wall sweep its path `length` less the stretches the openings of its host interrupt, the area of its
cross-section, its `gross_volume` and the visible surface; for every opening with an authored reveal the area of the reveal and the lateral planes of its frame. This file reproduces the
table from the SAME committed snapshot alone with a library that has never seen this repository:

* the height of a roof at a plan point is the geometry of its pitch: a hip roof rises `tan(pitch)` per metre from the nearest edge of its footprint (`Polygon.exterior.distance`), a gable
  roof from the nearest eave edge (the edges across the ridge are vertical gable ends), a sloped slab falls `tan(angle)` per metre along its fall direction from its uphill edge;
* the elevation edge of a wall is that height sampled along the axis and integrated EXACTLY: the integrand is piecewise linear, so the integral is refined at every kink (bisection on the
  second difference) and summed trapezoid by trapezoid; `top_z` and `base_z` are the extremes at the nodes;
* a sweep run is `LineString.difference` of the face stretch and the cut rectangles of the openings that overlap its height range, the cross-section is a `Polygon`, the visible surface is the
  boundary of the section clipped at the inset, less the stretch on the inset line;
* the area of a reveal is the perimeter of the cut rectangle (`Polygon.length`) times the reveal depth.

The committed expectation is written by this file, never by hand.

Standalone use (no test host needed):

    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧗️wall-depth>
    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧗️wall-depth>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🧬️schema/💡️inferences/🧱️wall-layout/🧲️attach/🦀️.rs — the attach
@see ../../🧬️schema/💡️inferences/🧊️element-solids/🧷️wall-sweeps/🦀️.rs — the sweeps
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import LineString, MultiLineString, Point, Polygon, box

# endregion 🔖️Imports


# region 🔖️Vocabulary
def canonical(value):
    """🧹️ The JSON form committed to the fixture: floats rounded to 12 decimals so reruns are byte stable."""
    if isinstance(value, float):
        return round(value, 12) + 0.0
    if isinstance(value, dict):
        return {key: canonical(item) for key, item in sorted(value.items())}
    if isinstance(value, list):
        return [canonical(item) for item in value]
    return value


def levels(snapshot):
    """🪜️ `{storey id: (elevation, top elevation)}`: the stacking of the storeys of each building from level 0 upward."""
    out = {}
    for building in snapshot["buildings"]:
        rows = sorted(((storey["level"], key) for key, storey in snapshot["storeys"].items() if storey["building"] == building))
        elevation = 0.0
        for _, key in [row for row in rows if row[0] >= 0]:
            top = elevation + snapshot["storeys"][key]["height"]
            out[key] = (elevation, top)
            elevation = top
    return out


def line_of(axis):
    """〰️ The two ends of a straight wall axis; arcs are outside this oracle."""
    assert "Line" in axis, "this oracle audits straight walls"
    line = axis["Line"]
    return (line["start"]["x"], line["start"]["y"]), (line["end"]["x"], line["end"]["y"])


def thickness_of(snapshot, wall):
    return math.fsum(layer["thickness"] for layer in snapshot["wall_types"][wall["wall_type"]]["layers"])


# endregion 🔖️Vocabulary


# region 🔖️Surfaces
def footprint_of(loop):
    return Polygon([(vertex["point"]["x"], vertex["point"]["y"]) for vertex in loop])


def roof_height(snapshot, roof, point):
    """🏠️ The height of the underside of a roof above its eave plane at `point`, `None` outside the footprint."""
    polygon = footprint_of(roof["footprint"])
    assert roof["overhang"] == 0, "this oracle audits roofs without an overhang"
    if not polygon.buffer(1e-9).contains(Point(point)):
        return None
    shape = roof["shape"]
    if "Hip" in shape:
        return math.tan(shape["Hip"]["pitch"]) * polygon.exterior.distance(Point(point))
    if "Gable" in shape:
        pitch, ridge = shape["Gable"]["pitch"], shape["Gable"]["ridge_direction"]
        ring = list(polygon.exterior.coords)
        eaves = [LineString([a, b]) for a, b in zip(ring, ring[1:]) if abs((b[0] - a[0]) * math.cos(ridge) + (b[1] - a[1]) * math.sin(ridge)) > 0.02 * math.hypot(b[0] - a[0], b[1] - a[1])]
        return math.tan(pitch) * min(eave.distance(Point(point)) for eave in eaves)
    if "Flat" in shape or shape == "Flat":
        return 0.0
    raise AssertionError("shape %r is outside this oracle" % (shape,))


def slab_top(snapshot, slab, point, levels_):
    """⬜️ The z of the top plane of a slab at `point`, `None` outside the boundary."""
    polygon = footprint_of(slab["boundary"])
    if not polygon.buffer(1e-9).contains(Point(point)):
        return None
    reference = levels_[slab["storey"]][0] + slab["offset"]
    slope = slab.get("slope")
    if not slope or abs(slope["angle"]) < 1e-12:
        return reference
    along = lambda x, y: x * math.cos(slope["direction"]) + y * math.sin(slope["direction"])
    uphill = min(along(x, y) for x, y in polygon.exterior.coords)
    return reference - math.tan(slope["angle"]) * (along(*point) - uphill)


def top_at(snapshot, wall, point, levels_):
    """🔝️ The z of the top of a wall at a plan point of its axis."""
    top = wall["top"]
    if "Roof" in top:
        roof = snapshot["roofs"][top["Roof"]["roof"]]
        height = roof_height(snapshot, roof, point)
        return levels_[roof["storey"]][1] + roof["base_offset"] + height + top["Roof"]["offset"]
    if "Slab" in top:
        slab = snapshot["slabs"][top["Slab"]["slab"]]
        return slab_top(snapshot, slab, point, levels_) - math.fsum(layer["thickness"] for layer in snapshot["slab_types"][slab["slab_type"]]["layers"]) + top["Slab"]["offset"]
    if "StoreyTop" in top:
        return levels_[wall["storey"]][1] + top["StoreyTop"]["offset"]
    raise AssertionError("top %r is outside this oracle" % (top,))


def base_at(snapshot, wall, point, levels_):
    """⬇️ The z of the base of a wall at a plan point of its axis."""
    if wall.get("base_slab"):
        return slab_top(snapshot, snapshot["slabs"][wall["base_slab"]], point, levels_) + wall["base_offset"]
    return levels_[wall["storey"]][0] + wall["base_offset"]


# endregion 🔖️Surfaces


# region 🔖️Integration
def refine(function, a, b, fa, fb, nodes, depth=0):
    """📈️ Splits `[a, b]` until the function is linear on every piece (the midpoint lies on the chord within 1e-13), collecting the nodes."""
    mid = (a + b) / 2.0
    fm = function(mid)
    if depth > 60 or abs(fm - (fa + fb) / 2.0) <= 1e-13 or (b - a) < 1e-10:
        return
    refine(function, a, mid, fa, fm, nodes, depth + 1)
    nodes.append((mid, fm))
    refine(function, mid, b, fm, fb, nodes, depth + 1)


def profile(function, length, parts=64):
    """📈️ The nodes `(s, f(s))` of a piecewise linear function on `[0, length]`: an even grid refined at every kink."""
    grid = [length * k / parts for k in range(parts + 1)]
    nodes = [(grid[0], function(grid[0]))]
    for a, b in zip(grid, grid[1:]):
        fa, fb = function(a), function(b)
        refine(function, a, b, fa, fb, nodes)
        nodes.append((b, fb))
    return sorted(set(nodes))


def integral(nodes):
    return math.fsum((b[0] - a[0]) * (a[1] + b[1]) / 2.0 for a, b in zip(nodes, nodes[1:]))


# endregion 🔖️Integration


# region 🔖️Measure
def wall_row(snapshot, levels_, wall_id):
    """🧱️ The row of one wall with an attached top or base: its extremes, the area of its elevation and, for a free wall without openings, its volume."""
    wall = snapshot["walls"][wall_id]
    start, end = line_of(wall["axis"])
    length = math.hypot(end[0] - start[0], end[1] - start[1])
    at = lambda s: (start[0] + (end[0] - start[0]) * s / length, start[1] + (end[1] - start[1]) * s / length)
    nodes = profile(lambda s: top_at(snapshot, wall, at(s), levels_) - base_at(snapshot, wall, at(s), levels_), length)
    tops = [top_at(snapshot, wall, at(s), levels_) for s, _ in nodes]
    bases = [base_at(snapshot, wall, at(s), levels_) for s, _ in nodes]
    area = integral(nodes)
    ends = line_of(wall["axis"])
    free = all(math.dist(a, b) > 1e-6 for key, other in snapshot["walls"].items() if key != wall_id and other["storey"] == wall["storey"] for a in line_of(other["axis"]) for b in ends)
    hosted = any(opening["host"] == wall_id for opening in snapshot.get("openings", {}).values())
    row = {"base_z": min(bases), "top_z": max(tops), "side_area": area}
    if free and not hosted:
        row["volume"] = thickness_of(snapshot, wall) * area
    return row


def sweep_row(snapshot, levels_, sweep_id):
    """🧷️ The row of one wall sweep on a free straight wall."""
    sweep = snapshot["wall_sweeps"][sweep_id]
    wall = snapshot["walls"][sweep["host"]]
    start, end = line_of(wall["axis"])
    length = math.hypot(end[0] - start[0], end[1] - start[1])
    profile_ = sweep["profile"]["Rectangle"]
    out, rise = profile_["width"], profile_["depth"]
    at = lambda s: (start[0] + (end[0] - start[0]) * s / length, start[1] + (end[1] - start[1]) * s / length)
    blocked = []
    for opening in snapshot.get("openings", {}).values():
        if opening["host"] != sweep["host"]:
            continue
        kind = opening["kind"]
        if "Window" in kind:
            size = snapshot["window_types"][kind["Window"]["window_type"]]
            sill = opening.get("sill_override", size["sill"])
        else:
            size = snapshot["door_types"][kind["Door"]["door_type"]]
            sill = opening.get("sill_override", 0.0)
        width, height = opening.get("width", size["width"]), opening.get("height", size["height"])
        base = base_at(snapshot, wall, at(opening["offset"]), levels_)
        low, high = base + sweep["height"], base + sweep["height"] + rise
        if base + sill < high - 1e-9 and base + sill + height > low + 1e-9:
            blocked.append((opening["offset"] - width / 2.0, opening["offset"] + width / 2.0))
    path = LineString([(0.0, 0.0), (length, 0.0)])
    kept = path.difference(MultiLineString([[(low, 0.0), (high, 0.0)] for low, high in blocked])) if blocked else path
    section = box(0.0, 0.0, out, rise)
    shown = section.intersection(box(sweep["inset"], -1.0, out + 1.0, rise + 1.0))
    visible = shown.boundary.difference(LineString([(sweep["inset"], -1.0), (sweep["inset"], rise + 1.0)])).length
    return {"length": kept.length, "section_area": section.area, "gross_volume": section.area * kept.length, "surface_area": visible * kept.length}


def reveal_row(snapshot, opening_id):
    """🪟️ The row of one opening with an authored reveal on a straight wall along +x at y = 0."""
    opening = snapshot["openings"][opening_id]
    wall = snapshot["walls"][opening["host"]]
    start, end = line_of(wall["axis"])
    assert start[1] == end[1] == 0.0 and end[0] > start[0] and wall["location"] == "Center", "this oracle audits centred walls along +x on y = 0"
    thickness = thickness_of(snapshot, wall)
    kind = opening["kind"]["Window"] if "Window" in opening["kind"] else opening["kind"]["Door"]
    size = snapshot["window_types"][kind["window_type"]] if "window_type" in kind else snapshot["door_types"][kind["door_type"]]
    width, height = opening.get("width", size["width"]), opening.get("height", size["height"])
    depth = min(opening["reveal_depth"], thickness)
    front = thickness / 2.0 - depth if not opening["flip_facing"] else -thickness / 2.0 + depth
    other = front - size["frame_depth"] if not opening["flip_facing"] else front + size["frame_depth"]
    return {"jamb_area": Polygon([(0, 0), (width, 0), (width, height), (0, height)]).length * depth, "frame_front_y": front, "frame_back_y": other}


def tables(snapshot):
    """🧗️ The table of a snapshot: attached walls, sweeps and reveals."""
    levels_ = levels(snapshot)
    walls = sorted(key for key, wall in snapshot["walls"].items() if "Roof" in wall["top"] or "Slab" in wall["top"] or "Ceiling" in wall["top"] or wall.get("base_slab"))
    return {
        "walls": {key: wall_row(snapshot, levels_, key) for key in walls},
        "sweeps": {key: sweep_row(snapshot, levels_, key) for key in sorted(snapshot.get("wall_sweeps", {}))},
        "reveals": {key: reveal_row(snapshot, key) for key, opening in sorted(snapshot.get("openings", {}).items()) if opening.get("reveal_depth") is not None},
    }


# endregion 🔖️Measure


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def wall_depth_handler(ctx):
    """🧗️ Oracle answer for the attached walls, the sweeps and the reveals."""
    from semio_repo_test import Outcome

    payload = canonical(tables(case_snapshot(ctx)))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("wall-depth", wall_depth_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        table = canonical(tables(json.loads(snapshot_path.read_text(encoding="utf-8"))))
        target = case / "💡️inference" / "🧗️wall-depth" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        elif json.loads(target.read_text(encoding="utf-8")) != table:
            failures.append("%s: the committed table differs from the oracle" % case.name)
        print("%s: shapely %s, %d attached walls, %d sweeps, %d reveals" % (case.name, shapely.__version__, len(table["walls"]), len(table["sweeps"]), len(table["reveals"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
