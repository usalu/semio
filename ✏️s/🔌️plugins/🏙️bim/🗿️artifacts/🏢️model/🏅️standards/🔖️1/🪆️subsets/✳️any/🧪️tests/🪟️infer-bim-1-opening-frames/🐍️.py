#!/usr/bin/env python3
"""🪟️ Third-party ORACLE for the `s.bim.model@1` inference `🪟️opening-frames`.

The subject (Rust, `semio-s-artifact-bim-model`) resolves every window, door and void of a snapshot: size and sill, the point and
tangent on the host axis at arc length `offset`, the local and world frames, the cut rectangle in the host's `(s, z)`
development, the reveal depth, the door swing and window glazing as plan strokes and the validity of the placement. This file
reproduces the whole table from the SAME committed snapshot and audits it with libraries that have never seen this repository:

* `shapely` 2 (GEOS) walks a sampled axis (`LineString`, 4096 chords for an arc) with `interpolate` and must meet the closed-form
  point and tangent; the cut is a real `box`, its `area` must be `width * height`, `covers` against the host rectangle
  `box(0, 0, length, height)` must agree with the validity flags, and `intersection(...).area` between sibling cut boxes decides
  the overlaps. `shapely.affinity.rotate` and `translate` build the world points of the frame.
* `numpy` carries the frame algebra (right-handedness by `cross`, rotation matrices, the swing arcs).

The committed expectations under `🧫️fixtures/💡️inferences/🪟️opening-frames/<case>/💡️inference/🪟️opening-frames/🔣️.json` are WRITTEN by this
file (`write`), never by hand, and the Rust subject is compared against them.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🪟️opening-frames>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🪟️opening-frames>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r4-api-opening-frames.md — the value type and its conventions
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import numpy
import shapely
from shapely import affinity
from shapely.geometry import LineString, Point, box

# endregion 🔖️Imports


# region 🔖️Vocabulary
ARC_SEGMENTS = 4096
"""📐️ Chords an arc axis is sampled into before shapely walks it; the sampling error stays below 1e-6."""

EPS = 1e-9
"""⚖️ Coincidence tolerance of lengths (one nanometre), the same policy the subject documents."""

SAMPLED_TOLERANCE = 1e-5
"""⚖️ Tolerance where shapely walks a sampled arc against the closed form."""

COMPARE_TOLERANCE = 1e-9
"""⚖️ Tolerance between two committed tables."""

ISSUES = ("HostMissing", "TypeMissing", "NonPositiveSize", "OutsideHostExtent", "BelowHostBase", "AboveHostTop", "OverlapsSibling")
"""🩺️ The validity vocabulary, in the order the subject reports it."""

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


# endregion 🔖️Snapshot


# region 🔖️Hosts
def storey_levels(snapshot):
    """🪜️ Level 0 is the building datum: levels >= 0 stack upward with exactly rounded sums, levels < 0 hang downward."""
    result = {}
    for building_id, building in snapshot["buildings"].items():
        rows = sorted((storey["level"], storey_id) for storey_id, storey in snapshot["storeys"].items() if storey["building"] == building_id)
        datum = snapshot["sites"][building["site"]]["elevation"] + building["elevation"]
        stacked, hung = [], []
        for level, storey_id in rows:
            if level >= 0:
                low = math.fsum(stacked)
                stacked.append(snapshot["storeys"][storey_id]["height"])
                result[storey_id] = (low, math.fsum(stacked), datum)
        for level, storey_id in reversed(rows):
            if level < 0:
                high = -math.fsum(hung)
                hung.append(snapshot["storeys"][storey_id]["height"])
                result[storey_id] = (-math.fsum(hung), high, datum)
    return result


def axis_geometry(axis):
    """〰️ Closed form of a wall axis: its length and, for an arc, centre, radius, start angle and signed sweep (`bulge = tan(sweep / 4)`)."""
    tag, body = variant(axis)
    start, end = numpy.array(xy(body["start"])), numpy.array(xy(body["end"]))
    chord = float(numpy.linalg.norm(end - start))
    if tag == "Line":
        return {"kind": "line", "start": start, "end": end, "length": chord, "direction": (end - start) / chord if chord > 0 else numpy.array([1.0, 0.0])}
    bulge = body["bulge"]
    sweep = 4.0 * math.atan(bulge)
    direction = (end - start) / chord
    centre = (start + end) / 2.0 + numpy.array([-direction[1], direction[0]]) * chord * (1.0 - bulge * bulge) / (4.0 * bulge)
    radius = float(numpy.linalg.norm(start - centre))
    first = math.atan2(start[1] - centre[1], start[0] - centre[0])
    return {"kind": "arc", "start": start, "end": end, "centre": centre, "radius": radius, "first": first, "sweep": sweep, "length": radius * abs(sweep)}


def point_and_tangent(geometry, s):
    """📍️ The point and unit tangent of travel at arc length `s` from the start (not clamped)."""
    if geometry["kind"] == "line":
        return geometry["start"] + geometry["direction"] * s, geometry["direction"]
    angle = geometry["first"] + math.copysign(s / geometry["radius"], geometry["sweep"])
    point = geometry["centre"] + geometry["radius"] * numpy.array([math.cos(angle), math.sin(angle)])
    return point, math.copysign(1.0, geometry["sweep"]) * numpy.array([-math.sin(angle), math.cos(angle)])


def sampled_line(geometry):
    """〰️ The axis as a shapely `LineString` (an arc is sampled into chords)."""
    if geometry["kind"] == "line":
        return LineString([tuple(geometry["start"]), tuple(geometry["end"])])
    return LineString([tuple(geometry["centre"] + geometry["radius"] * numpy.array([math.cos(geometry["first"] + geometry["sweep"] * step / ARC_SEGMENTS), math.sin(geometry["first"] + geometry["sweep"] * step / ARC_SEGMENTS)])) for step in range(ARC_SEGMENTS + 1)])


def top_z(snapshot, levels, record, base_z):
    """🔝️ The resolved top of a wall or curtain wall from its `TopConstraint`."""
    tag, body = variant(record["top"])
    if tag == "Unconnected":
        return base_z + body["height"]
    if tag == "StoreyTop":
        return levels[record["storey"]][1] + body["offset"]
    return levels[body["storey"]][0] + body["offset"] if body["storey"] in levels else levels[record["storey"]][0] + body["offset"]


def mullion_depth(profile):
    """▭️ The depth of a mullion profile."""
    tag, body = variant(profile)
    if tag == "Circle":
        return body["diameter"]
    if tag == "Custom":
        ys = [vertex["point"]["y"] for vertex in body["outline"]]
        return max(ys) - min(ys)
    return body["depth"]


def left_face_distance(location, layers, thickness):
    """↔️ Distance from the axis to the left face: `Interior` names the left face as the axis, `Exterior` the right face, `Center` the mid plane, `CoreCenter` the centre of the `Core` (else `Structure`) layers."""
    if location == "Interior":
        return 0.0
    if location == "Exterior":
        return thickness
    if location == "CoreCenter":
        for function in ("Core", "Structure"):
            indices = [index for index, layer in enumerate(layers) if layer["function"] == function]
            if indices:
                return (math.fsum(layer["thickness"] for layer in layers[:indices[0]]) + math.fsum(layer["thickness"] for layer in layers[:indices[-1] + 1])) / 2.0
    return thickness / 2.0


def host_extent(snapshot, levels, host_id):
    """🏗️ The axis, base, height, thickness, length, datum and building placement of a wall (which wins) or curtain wall; `None` when it does not resolve."""
    if host_id in snapshot["walls"]:
        record = snapshot["walls"][host_id]
        layers = snapshot["wall_types"][record["wall_type"]]["layers"]
        thickness = math.fsum(layer["thickness"] for layer in layers)
        face_left = left_face_distance(record["location"], layers, thickness)
    elif host_id in snapshot.get("curtain_walls", {}):
        record = snapshot["curtain_walls"][host_id]
        thickness = mullion_depth(record["mullion"])
        face_left = thickness / 2.0
    else:
        return None
    if record["storey"] not in levels:
        return None
    base = levels[record["storey"]][0] + record["base_offset"]
    top = top_z(snapshot, levels, record, base)
    building = snapshot["buildings"][snapshot["storeys"][record["storey"]]["building"]]
    geometry = axis_geometry(record["axis"])
    return {"geometry": geometry, "base_z": base, "height": top - base, "thickness": thickness, "face_left": face_left, "face_right": thickness - face_left, "length": geometry["length"], "datum": levels[record["storey"]][2], "origin": xy(building["origin"]), "rotation": building["rotation"]}


# endregion 🔖️Hosts


# region 🔖️Openings
def resolved_size(snapshot, opening):
    """📐️ Width, height and sill: an explicit width or height wins over the type; the sill override replaces the sill of the type (a window defaults to its type sill, a door or void to zero)."""
    tag, body = variant(opening["kind"])
    if tag == "Window":
        kind = snapshot["window_types"].get(body["window_type"])
        defaults = (kind["width"], kind["height"], kind["sill"]) if kind else (0.0, 0.0, 0.0)
    elif tag == "Door":
        kind = snapshot["door_types"].get(body["door_type"])
        defaults = (kind["width"], kind["height"], 0.0) if kind else (0.0, 0.0, 0.0)
    else:
        kind, defaults = True, (body["width"], body["height"], 0.0)
    return opening.get("width", defaults[0]), opening.get("height", defaults[1]), opening.get("sill_override", defaults[2]), kind is not None and kind is not False


def cut_box(snapshot, opening):
    """✂️ The cut rectangle as a shapely `box(s_min, z_min, s_max, z_max)`."""
    width, height, sill, _ = resolved_size(snapshot, opening)
    return box(opening["offset"] - width / 2.0, sill, opening["offset"] + width / 2.0, sill + height)


def vec3(origin):
    """🧭️ A numpy triple as the `{x, y, z}` object of the table."""
    return {"x": float(origin[0]), "y": float(origin[1]), "z": float(origin[2])}


def point2(origin):
    """📍️ A numpy pair as the `{x, y}` object of the table."""
    return {"x": float(origin[0]), "y": float(origin[1])}


def empty_frame():
    """🧭️ The default frame an unresolvable opening carries."""
    zero = {"x": 0.0, "y": 0.0, "z": 0.0}
    return {"origin": dict(zero), "x_axis": dict(zero), "y_axis": dict(zero), "z_axis": dict(zero)}


def leaf_strokes(hinge, side, length, x, y):
    """🚪️ The open leaf line and the swing arc of one leaf: hinge on the jamb, the leaf opens towards `+y` through a quarter turn."""
    free = hinge - x * side * length
    closed = free - hinge
    sweep = math.pi / 2.0 if closed[0] * y[1] - closed[1] * y[0] > 0 else -math.pi / 2.0
    return [
        {"role": "Leaf", "shape": {"Line": {"from": point2(hinge), "to": point2(hinge + y * length)}}},
        {"role": "Swing", "shape": {"Arc": {"centre": point2(hinge), "radius": length, "start_angle": math.atan2(closed[1], closed[0]), "sweep": sweep}}},
    ]


def plan_strokes(snapshot, opening, hand, centre, x, y, width, face):
    """🖊️ Glazing line of a window on the location line, leaves and swings of a door with the hinge on the jamb at the +y face (Left hand hinges at +x seen from +y), nothing for a void."""
    tag, body = variant(opening["kind"])
    if tag == "Window":
        return [{"role": "Glazing", "shape": {"Line": {"from": point2(centre - x * width / 2.0), "to": point2(centre + x * width / 2.0)}}}]
    if tag == "Door" and hand is not None:
        side = 1.0 if hand == "Left" else -1.0
        centre = centre + y * face
        if snapshot["door_types"][body["door_type"]]["leaves"] == "Single":
            return leaf_strokes(centre + x * side * width / 2.0, side, width, x, y)
        return [stroke for sign in (1.0, -1.0) for stroke in leaf_strokes(centre + x * sign * width / 2.0, sign, width / 2.0, x, y)]
    return []


def opening_frame(snapshot, levels, opening_id):
    """🪟️ The full table row of one opening."""
    opening = snapshot["openings"][opening_id]
    width, height, sill, type_found = resolved_size(snapshot, opening)
    cut = cut_box(snapshot, opening)
    s_min, z_min, s_max, z_max = cut.bounds
    issues = []
    host = host_extent(snapshot, levels, opening["host"])
    if host is None:
        issues.append("HostMissing")
    if not type_found:
        issues.append("TypeMissing")
    if width <= EPS or height <= EPS:
        issues.append("NonPositiveSize")
    row = {
        "width": width, "height": height, "sill": sill, "offset": opening["offset"],
        "cut": {"s_min": opening["offset"] - width / 2.0, "s_max": opening["offset"] + width / 2.0, "z_min": sill, "z_max": sill + height},
        "reveal_depth": 0.0, "face_front": 0.0, "face_back": 0.0, "host_length": 0.0, "host_height": 0.0, "point": {"x": 0.0, "y": 0.0},
        "local": empty_frame(), "world": empty_frame(), "plan": [], "overlaps": [],
    }
    if host is None:
        return {**row, "issues": issues, "valid": False}
    if s_min < -EPS or s_max > host["length"] + EPS:
        issues.append("OutsideHostExtent")
    if z_min < -EPS:
        issues.append("BelowHostBase")
    if z_max > host["height"] + EPS:
        issues.append("AboveHostTop")
    overlaps = [other_id for other_id, other in snapshot["openings"].items() if other["host"] == opening["host"] and other_id != opening_id and extents_overlap(cut, cut_box(snapshot, other))]
    if overlaps:
        issues.append("OverlapsSibling")
    point, tangent = point_and_tangent(host["geometry"], opening["offset"])
    tangent = tangent if numpy.linalg.norm(tangent) > 0.5 else numpy.array([1.0, 0.0])
    facing = -1.0 if opening["flip_facing"] else 1.0
    x, y = tangent * facing, numpy.array([-tangent[1], tangent[0]]) * facing
    face_front, face_back = (host["face_right"], host["face_left"]) if opening["flip_facing"] else (host["face_left"], host["face_right"])
    local = {"origin": numpy.array([point[0], point[1], host["base_z"] + sill]), "x_axis": numpy.array([x[0], x[1], 0.0]), "y_axis": numpy.array([y[0], y[1], 0.0]), "z_axis": numpy.array([0.0, 0.0, 1.0])}
    rotation = host["rotation"]
    placed = affinity.translate(affinity.rotate(Point(local["origin"][0], local["origin"][1]), rotation, origin=(0, 0), use_radians=True), host["origin"][0], host["origin"][1])
    world = {"origin": numpy.array([placed.x, placed.y, local["origin"][2] + host["datum"]])}
    for name in ("x_axis", "y_axis"):
        turned = affinity.rotate(Point(local[name][0], local[name][1]), rotation, origin=(0, 0), use_radians=True)
        world[name] = numpy.array([turned.x, turned.y, 0.0])
    world["z_axis"] = local["z_axis"]
    hand = None
    tag, body = variant(opening["kind"])
    if tag == "Door" and body["door_type"] in snapshot["door_types"]:
        swing = snapshot["door_types"][body["door_type"]]["swing"]
        hand = swing if not opening["flip_hand"] else ("Right" if swing == "Left" else "Left")
    row.update({
        "reveal_depth": host["thickness"], "face_front": face_front, "face_back": face_back, "host_length": host["length"], "host_height": host["height"], "point": point2(point),
        "local": {name: vec3(value) for name, value in local.items()}, "world": {name: vec3(value) for name, value in world.items()},
        "plan": plan_strokes(snapshot, opening, hand, point, x, y, width, face_front), "overlaps": sorted(overlaps), "issues": issues, "valid": not issues,
    })
    if hand is not None:
        row["hand"] = hand
    return row


def extents_overlap(first, second):
    """✂️ Whether two cut boxes share a positive extent in both `s` and `z` (shapely `intersection` with a nanometre threshold on each side)."""
    shared = first.intersection(second)
    if shared.is_empty:
        return False
    s_min, z_min, s_max, z_max = shared.bounds
    return s_max - s_min > EPS and z_max - z_min > EPS


def opening_frames(snapshot):
    """🪟️ The `🪟️opening-frames` table: one row per opening, in id order."""
    levels = storey_levels(snapshot)
    return {opening_id: opening_frame(snapshot, levels, opening_id) for opening_id in sorted(snapshot["openings"])}


# endregion 🔖️Openings


# region 🔖️Audit
def shapely_problems(snapshot, table):
    """〰️ Every disagreement between the closed form and what shapely measures: walked axis points and tangents, cut areas, containment in the host rectangle."""
    problems = []
    levels = storey_levels(snapshot)
    for opening_id, opening in snapshot["openings"].items():
        row = table[opening_id]
        cut = cut_box(snapshot, opening)
        if abs(cut.area - row["width"] * row["height"]) > EPS * max(1.0, row["width"] * row["height"]):
            problems.append("%s: shapely cut area %.12g != %.12g" % (opening_id, cut.area, row["width"] * row["height"]))
        host = host_extent(snapshot, levels, opening["host"])
        if host is None:
            continue
        line = sampled_line(host["geometry"])
        if abs(line.length - host["length"]) > SAMPLED_TOLERANCE * max(1.0, host["length"]):
            problems.append("%s: shapely host length %.12g != %.12g" % (opening_id, line.length, host["length"]))
        walked = line.interpolate(min(max(opening["offset"], 0.0), line.length))
        inside_axis = 0.0 <= opening["offset"] <= host["length"]
        if inside_axis and math.hypot(walked.x - row["point"]["x"], walked.y - row["point"]["y"]) > SAMPLED_TOLERANCE * max(1.0, host["length"]):
            problems.append("%s: shapely point (%.9g, %.9g) != (%.9g, %.9g)" % (opening_id, walked.x, walked.y, row["point"]["x"], row["point"]["y"]))
        if inside_axis:
            step = 1e-3
            ahead, behind = line.interpolate(min(opening["offset"] + step, line.length)), line.interpolate(max(opening["offset"] - step, 0.0))
            span = math.hypot(ahead.x - behind.x, ahead.y - behind.y)
            direction = ((ahead.x - behind.x) / span, (ahead.y - behind.y) / span)
            facing = -1.0 if opening["flip_facing"] else 1.0
            if math.hypot(direction[0] * facing - row["local"]["x_axis"]["x"], direction[1] * facing - row["local"]["x_axis"]["y"]) > 1e-4:
                problems.append("%s: shapely tangent %s != (%.9g, %.9g)" % (opening_id, direction, row["local"]["x_axis"]["x"], row["local"]["x_axis"]["y"]))
        rectangle = box(0.0, 0.0, host["length"], host["height"])
        fits = not ({"OutsideHostExtent", "BelowHostBase", "AboveHostTop"} & set(row["issues"]))
        if rectangle.buffer(EPS).covers(cut) != fits:
            problems.append("%s: shapely covers=%s but issues are %s" % (opening_id, not fits, row["issues"]))
        for frame_name in ("local", "world"):
            axes = row[frame_name]
            x_axis, y_axis, z_axis = (numpy.array([axes[name]["x"], axes[name]["y"], axes[name]["z"]]) for name in ("x_axis", "y_axis", "z_axis"))
            if "HostMissing" not in row["issues"]:
                if not numpy.allclose(numpy.cross(x_axis, y_axis), z_axis, atol=1e-12):
                    problems.append("%s: %s frame is not right-handed" % (opening_id, frame_name))
    return problems


# endregion 🔖️Audit


# region 🔖️Projection
def rounded(value):
    """🔢️ Rounds every number to 12 decimals so the committed JSON is stable across platforms."""
    if isinstance(value, dict):
        return {key: rounded(item) for key, item in value.items()}
    if isinstance(value, list):
        return [rounded(item) for item in value]
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
    if isinstance(committed, list) and isinstance(computed, list):
        if len(committed) != len(computed):
            return ["%s: length %d != %d" % (path, len(committed), len(computed))]
        return [problem for index, (left, right) in enumerate(zip(committed, computed)) for problem in differences("%s[%d]" % (path, index), left, right)]
    if isinstance(committed, (int, float)) and isinstance(computed, (int, float)) and not isinstance(committed, bool):
        return [] if abs(committed - computed) <= COMPARE_TOLERANCE * max(1.0, abs(committed)) else ["%s: committed %.12g, computed %.12g" % (path, committed, computed)]
    return [] if committed == computed else ["%s: committed %r, computed %r" % (path, committed, computed)]


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def opening_frames_handler(ctx):
    """🪟️ Oracle answer for `🪟️opening-frames`, after shapely and numpy agreed with the closed form."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    table = rounded(opening_frames(snapshot))
    problems = shapely_problems(snapshot, table)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("opening-frames-placed", opening_frames_handler).oracle("opening-frames-invalid", opening_frames_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates them."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = load_snapshot(snapshot_path)
        table = rounded(opening_frames(snapshot))
        failures += ["%s: %s" % (case.name, problem) for problem in shapely_problems(snapshot, table)]
        target = case / "💡️inference" / "🪟️opening-frames" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in differences("frames", json.loads(target.read_text(encoding="utf-8")), table)]
        print("%s: shapely %s, %d openings" % (case.name, shapely.__version__, len(snapshot["openings"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
