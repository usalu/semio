#!/usr/bin/env python3
"""🎨️ Third-party ORACLE for the `s.bim.model@1` inference `🎨️finishes`.

A space authors only the materials that finish its floor, walls and ceiling; the areas are derived. The subject (Rust, `semio-s-artifact-bim-model`)
takes them from the resolved room and the opening frames of the model graph. This file reproduces the table from the SAME committed snapshot with
a library that has never seen this repository: `shapely` 2 (GEOS) gives the rooms through the sibling oracle `../🏠️infer-bim-1-spaces` (unary union of the
wall footprints, `polygonize`, `difference` of the columns) and then decides, for every opening of a wall of the storey, on which side of the host the room
lies: the middle of each face of the host is a shapely `Point`, and it counts when `Polygon.boundary.distance(point)` is zero within 0.1 mm. The opening
rectangle (width times the part of its height between the floor and the ceiling of the room) is subtracted from `perimeter * clear height`; the floor is
the net floor area (`Polygon.area` minus the columns). The ceiling is the part of the room under its hung ceiling (`Polygon.intersection` with the ceiling boundary
minus its holes, `difference` of the columns) over the cosine of that ceiling slope plus the rest of the net floor over the cosine of the slope of the slab that closes the room.

Audits beyond the table: the finish areas follow the closed forms (`wall = perimeter * clear height - openings`, `ceiling >= floor`), every opening
counted on a room lies on its boundary on at least one face, and a door between two rooms is subtracted from both.

The committed expectations under `🧫️fixtures/💡️inferences/🎨️finishes/<case>/💡️inference/🎨️finishes/🔣️.json` are WRITTEN by this file (`write`), never by
hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🎨️finishes>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🎨️finishes>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r9-audit-completeness.md — work package 07
"""

# region 🔖️Imports
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import Point
from shapely.ops import unary_union

# endregion 🔖️Imports


# region 🔖️Vocabulary
FACE = 1e-4
"""📏️ How far, in metres, the middle of a host face may lie from the room boundary to count as on it."""


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def spaces():
    return load_sibling("🏠️infer-bim-1-spaces", "spaces")


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


# endregion 🔖️Vocabulary


# region 🔖️Rooms
def rooms(snapshot):
    """🏠️ `{space id: (table row, shapely polygon of the room)}` of every resolved space, from the spaces oracle."""
    oracle = spaces()
    table = oracle.tables(snapshot)
    cache = {}
    found = {}
    for space_id, space in snapshot["spaces"].items():
        row = table[space_id]
        if row["status"] not in ("Inferred", "Explicit"):
            continue
        tag, body = variant(space["boundary"])
        if tag == "Explicit":
            found[space_id] = (row, oracle.sampled(oracle.vertices_of(body["outline"])))
            continue
        storey = space["storey"]
        if storey not in cache:
            shapes = oracle.footprints(snapshot, storey)
            cache[storey] = oracle.free_faces(shapes)
        faces, union = cache[storey]
        _, face = oracle.room_for(oracle.xy(body["seed"]), faces, union)
        found[space_id] = (row, face)
    return found


def elevation(snapshot, storey):
    """⬆️ The elevation of a storey above the building datum: the heights of the storeys below it in its building."""
    own = snapshot["storeys"][storey]
    return math.fsum(row["height"] for row in snapshot["storeys"].values() if row["building"] == own["building"] and row["level"] < own["level"])


# endregion 🔖️Rooms


# region 🔖️Openings
def rectangle(snapshot, opening):
    """🪟️ `(width, height, sill)` of an opening after the override-else-type rule."""
    tag, body = variant(opening["kind"])
    if tag == "Window":
        kind = snapshot["window_types"][body["window_type"]]
        width, height, sill = kind["width"], kind["height"], kind["sill"]
    elif tag == "Door":
        kind = snapshot["door_types"][body["door_type"]]
        width, height, sill = kind["width"], kind["height"], 0.0
    else:
        width, height, sill = body["width"], body["height"], 0.0
    return (opening.get("width") or width, opening.get("height") or height, opening["sill_override"] if opening.get("sill_override") is not None else sill)


def faces_of(snapshot, opening):
    """🧱️ The middle of both faces of the host of an opening and the opening rectangle, or `None` when the host is not a straight centred wall or the opening is not cut."""
    wall = snapshot["walls"].get(opening["host"])
    if wall is None:
        return None
    tag, axis = variant(wall["axis"])
    if tag != "Line" or wall["location"] != "Center":
        return None
    start, end = (axis["start"]["x"], axis["start"]["y"]), (axis["end"]["x"], axis["end"]["y"])
    length = math.dist(start, end)
    thickness = math.fsum(layer["thickness"] for layer in snapshot["wall_types"][wall["wall_type"]]["layers"])
    width, height, sill = rectangle(snapshot, opening)
    low, high = opening["offset"] - width / 2.0, opening["offset"] + width / 2.0
    storey = snapshot["storeys"][wall["storey"]]
    if low <= thickness / 2.0 or high >= length - thickness / 2.0 or sill < 0.0 or sill + height >= storey["height"] - wall["base_offset"]:
        return None
    direction = ((end[0] - start[0]) / length, (end[1] - start[1]) / length)
    normal = (-direction[1], direction[0])
    centre = (start[0] + direction[0] * opening["offset"], start[1] + direction[1] * opening["offset"])
    faces = [(centre[0] + normal[0] * side * thickness / 2.0, centre[1] + normal[1] * side * thickness / 2.0) for side in (1.0, -1.0)]
    bottom = elevation(snapshot, wall["storey"]) + wall["base_offset"] + sill
    return wall["storey"], faces, width, bottom, bottom + height


def opening_area(snapshot, space, row, room):
    """✂️ The area the openings cut out of the walls of a room: width times the part of the height between floor and ceiling, once per host face on the room boundary."""
    floor = elevation(snapshot, space["storey"])
    ceiling = floor + row["clear_height"]
    total = 0.0
    on_boundary = 0
    for opening in snapshot["openings"].values():
        found = faces_of(snapshot, opening)
        if found is None or found[0] != space["storey"]:
            continue
        _, faces, width, bottom, top = found
        height = max(min(top, ceiling) - max(bottom, floor), 0.0)
        for face in faces:
            if room.boundary.distance(Point(face)) <= FACE:
                total += width * height
                on_boundary += 1
    return total, on_boundary


# endregion 🔖️Openings


# region 🔖️Ceiling
def hung_shape(snapshot, ceiling):
    """🔲️ The shapely polygon of a hung ceiling: its boundary (arcs sampled) without its holes."""
    oracle = spaces()
    shell = oracle.sampled(oracle.vertices_of(ceiling["boundary"]))
    return shell.difference(unary_union([oracle.sampled(oracle.vertices_of(hole)) for hole in ceiling["holes"]])) if ceiling["holes"] else shell


def tilt(slope):
    """📐️ The area factor of a tilted plane: the cosine of its fall angle."""
    return abs(math.cos(slope["angle"])) if slope else 1.0


def ceiling_area(snapshot, space, row, room):
    """🔲️ The ceiling surface of a room: the net floor under the hung ceiling over the cosine of its slope, the rest over the cosine of the slope of the slab that closes the room."""
    slab = snapshot["slabs"].get(row["ceiling_slab"]) if row["ceiling_slab"] else None
    soffit = tilt((slab or {}).get("slope"))
    hung = snapshot.get("ceilings", {}).get(row["ceiling"]) if row["ceiling"] else None
    if hung is None:
        return row["net_floor_area"] / soffit
    columns = spaces().column_shapes(snapshot, space["storey"])
    covered = room.intersection(hung_shape(snapshot, hung))
    covered = min((covered.difference(unary_union(columns)) if columns else covered).area, row["net_floor_area"])
    return covered / tilt(hung.get("slope")) + (row["net_floor_area"] - covered) / soffit


# endregion 🔖️Ceiling


# region 🔖️Table
def tables(snapshot):
    """🎨️ The `🎨️finishes` table of a snapshot: floor, wall (gross, openings, net) and ceiling areas of every resolved space."""
    oracle = spaces()
    result = {}
    for space_id, (row, room) in rooms(snapshot).items():
        space = snapshot["spaces"][space_id]
        gross = row["perimeter"] * row["clear_height"]
        openings, _ = opening_area(snapshot, space, row, room)
        result[space_id] = {
            "floor_area": row["net_floor_area"],
            "wall_gross_area": gross,
            "wall_opening_area": openings,
            "wall_area": max(gross - openings, 0.0),
            "ceiling_area": ceiling_area(snapshot, space, row, room),
        }
    return result


# endregion 🔖️Table


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Disagreements between the closed forms and the geometry, plus the invariants of the finish areas."""
    problems = []
    resolved = rooms(snapshot)
    for space_id, (row, room) in resolved.items():
        space = snapshot["spaces"][space_id]
        openings, _ = opening_area(snapshot, space, row, room)
        if openings > row["perimeter"] * row["clear_height"] + 1e-9:
            problems.append("%s: the openings (%.6g) exceed the wall area" % (space_id, openings))
        if ceiling_area(snapshot, space, row, room) < row["net_floor_area"] - 1e-9:
            problems.append("%s: the ceiling surface is smaller than the net floor" % space_id)
        if row["net_floor_area"] > row["area"] + 1e-9:
            problems.append("%s: the net floor area exceeds the room" % space_id)
        if abs(room.area - row["area"]) > 1e-5 * max(row["area"], 1.0):
            problems.append("%s: GEOS area %.9g, table area %.9g" % (space_id, room.area, row["area"]))
    for opening_id, opening in snapshot["openings"].items():
        found = faces_of(snapshot, opening)
        if found is None:
            continue
        sides = sum(1 for space_id, (_, room) in resolved.items() for face in found[1] if snapshot["spaces"][space_id]["storey"] == found[0] and room.boundary.distance(Point(face)) <= FACE)
        if sides == 0 and any(snapshot["spaces"][space_id]["storey"] == found[0] for space_id in resolved):
            problems.append("%s: an opening of a storey with rooms lies on the boundary of none of them" % opening_id)
    return problems


# endregion 🔖️Audit


# region 🔖️Projection
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels").compare(expected, actual, path)


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def finishes_handler(ctx):
    """🎨️ Oracle answer for `🎨️finishes`, after the closed forms agreed with the geometry."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = tables(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("finishes-rooms", finishes_handler).oracle("finishes-one-room", finishes_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in problems_of(snapshot)]
        table = tables(snapshot)
        target = case / "💡️inference" / "🎨️finishes" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "finishes")]
        print("%s: shapely %s, %d finished spaces" % (case.name, shapely.__version__, len(table)))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
