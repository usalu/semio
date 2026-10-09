#!/usr/bin/env python3
"""🏠️ Third-party ORACLE for the `s.bim.model@1` inference `🏠️spaces`.

The subject (Rust, `semio-s-artifact-bim-model`) finds the room around a seed as a face of the storey's wall arrangement
(`semio-framework-2d` region booleans over the join-trimmed footprints) and measures it. This file reproduces the table from the SAME
committed snapshot with a library that has never seen this repository: `shapely` 2 (GEOS) unions the footprints of the sibling oracle
`../🧱️infer-bim-1-wall-joins`, `polygonize`s the boundary linework of the union into faces, keeps the faces that are not wall
material, and picks the face that contains the seed (a seed in wall material or in no bounded face is reported as the subject reports it).
Areas, perimeters, islands and contact lengths are GEOS measures; columns are cut out with `difference`; the clear height follows the
slab of the storey above that `contains` the room point. Explicit outlines take closed-form areas and perimeters (arcs are circular
segments, GEOS only samples them) which GEOS then audits on a sampled polygon within the sampled tolerance.

Audits beyond the table: every room is a valid polygon disjoint from the walls and from the other rooms, and the parametric law holds
(moving the partition by `delta` moves exactly `delta * clear width` of area from one room to the other).

The committed expectations under `🧫️fixtures/💡️inferences/🛋️spaces/<case>/💡️inference/🏠️spaces/🔣️.json` are WRITTEN by this
file (`write`), never by hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🛋️spaces>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🛋️spaces>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r2-design.md — snapshot and inference catalogue
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely import affinity
from shapely.geometry import LineString, Point, Polygon, box
from shapely.ops import polygonize, unary_union

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
SAMPLED = 1e-5
CONTACT = 1e-3
"""📏️ Shortest shared edge, in metres, that makes a wall a bounding wall of a room."""

GRID = 1e-9
"""📏️ Coordinate grid the footprints are snapped to before they are united, so that the corners of mitered walls (equal up to rounding) coincide for GEOS."""


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def joins():
    return load_sibling("🧱️infer-bim-1-wall-joins", "wall_joins")


def levels_oracle():
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels")


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


def xy(point):
    return (point["x"], point["y"])


# endregion 🔖️Vocabulary


# region 🔖️Loops
def vertices_of(loop):
    """🔷️ `[(point, bulge)]` of an authored loop."""
    return [(xy(vertex["point"]), vertex["bulge"]) for vertex in loop]


def loop_area(vertices):
    """📐️ Signed area of a bulged loop: shoelace plus circular-segment terms."""
    return joins().loop_area(vertices)


def loop_length(vertices):
    """📏️ Length of a bulged loop: chords and circular arcs in closed form."""
    total = 0.0
    for index, (point, bulge) in enumerate(vertices):
        following = vertices[(index + 1) % len(vertices)][0]
        chord = math.dist(point, following)
        total += chord if abs(bulge) < 1e-12 else chord / (2.0 * math.sin(abs(4.0 * math.atan(bulge)) / 2.0)) * abs(4.0 * math.atan(bulge))
    return total


def sampled(vertices):
    """〰️ A shapely polygon of a bulged loop with sampled arcs."""
    return joins().sampled_polygon(vertices)


def circle_vertices(profile):
    return [((profile["diameter"] / 2.0, 0.0), 1.0), ((-profile["diameter"] / 2.0, 0.0), 1.0)]


# endregion 🔖️Loops


# region 🔖️Arrangement
def footprints(snapshot, storey):
    """🧱️ The join-trimmed wall footprints and the curtain-wall bands of a storey, keyed by element id (walls from the sibling oracle)."""
    plan = joins().plan_geometry(snapshot)
    shapes = {wall_id: sampled([(xy(vertex["point"]), vertex["bulge"]) for vertex in row["footprint"]]) for wall_id, row in plan.items() if row["footprint"] and snapshot["walls"][wall_id]["storey"] == storey}
    for curtain_id, curtain in snapshot["curtain_walls"].items():
        if curtain["storey"] == storey:
            tag, body = variant(curtain["mullion"])
            depth = body["depth"] if tag in ("Rectangle", "IShape") else body["diameter"]
            curve = joins().axis_curve(curtain["axis"])
            shapes[curtain_id] = LineString(curve.sampled()).buffer(depth / 2.0, cap_style="flat")
    return {key: shapely.set_precision(shape, GRID) for key, shape in shapes.items()}


def free_faces(shapes):
    """🕳️ The bounded faces of the arrangement that are not wall material: `polygonize` of the union boundary, minus the faces that lie in the union."""
    if not shapes:
        return [], None
    union = unary_union(list(shapes.values()))
    boundary = union.boundary
    lines = list(boundary.geoms) if hasattr(boundary, "geoms") else [boundary]
    faces = [face for face in polygonize(lines) if not union.contains(face.representative_point())]
    return faces, union


def room_for(seed, faces, union):
    """🧭️ `(status, face)` of a seed: the free face containing it, else why there is none."""
    point = Point(seed)
    for face in faces:
        if face.contains(point):
            return "Inferred", face
    if union is not None and union.contains(point):
        return "SeedInsideWall", None
    return "NotEnclosed", None


# endregion 🔖️Arrangement


# region 🔖️Columns
def column_shapes(snapshot, storey):
    """🏛️ The plan outline of every column of a storey as a shapely polygon."""
    shapes = []
    for column in snapshot["columns"].values():
        kind = snapshot["column_types"].get(column["column_type"])
        if column["storey"] != storey or kind is None:
            continue
        tag, body = variant(kind["profile"])
        if tag == "Rectangle":
            outline = box(-body["width"] / 2.0, -body["depth"] / 2.0, body["width"] / 2.0, body["depth"] / 2.0)
        elif tag == "Circle":
            outline = sampled(circle_vertices(body))
        else:
            continue
        outline = affinity.rotate(outline, column["rotation"], origin=(0.0, 0.0), use_radians=True)
        shapes.append(affinity.translate(outline, column["position"]["x"], column["position"]["y"]))
    return shapes


# endregion 🔖️Columns


# region 🔖️Ceiling
def above_storey(snapshot, storey):
    """⬆️ The storey with the next higher level in the same building."""
    own = snapshot["storeys"][storey]
    higher = sorted((row["level"], storey_id) for storey_id, row in snapshot["storeys"].items() if row["building"] == own["building"] and row["level"] > own["level"])
    return higher[0][1] if higher else None


def ceiling(snapshot, storey, point):
    """🔝️ `(slab id, thickness, offset)` of the thickest slab of the storey above that covers the point, else `None`."""
    above = above_storey(snapshot, storey)
    candidates = []
    for slab_id, slab in snapshot["slabs"].items():
        if slab["storey"] != above:
            continue
        shape = Polygon([xy(vertex["point"]) for vertex in slab["boundary"]], [[xy(vertex["point"]) for vertex in hole] for hole in slab["holes"]])
        if shape.contains(Point(point)):
            thickness = math.fsum(layer["thickness"] for layer in snapshot["slab_types"].get(slab["slab_type"], {"layers": []})["layers"])
            candidates.append((-thickness, slab_id, thickness, slab["offset"]))
    if not candidates:
        return None
    _, slab_id, thickness, offset = sorted(candidates)[0]
    return slab_id, thickness, offset


# endregion 🔖️Ceiling


# region 🔖️Hung
def hung(snapshot, storey, point):
    """🔲️ `(ceiling id, drop of the underside below the storey top)` of the lowest ceiling of the storey that covers the point, else `None`.

    A ceiling hangs `offset` below the storey top and its layers reach down by the type thickness; a slope lowers the plane by `tan(angle)` per metre along the fall direction,
    measured from the uphill edge of the boundary (the point of the boundary with the smallest projection on the fall direction)."""
    candidates = []
    for ceiling_id, ceiling in snapshot.get("ceilings", {}).items():
        if ceiling["storey"] != storey:
            continue
        kind = snapshot.get("ceiling_types", {}).get(ceiling["ceiling_type"])
        thickness = math.fsum(max(layer["thickness"], 0.0) for layer in kind["layers"]) if kind else 0.0
        if thickness <= 1e-12 or len(ceiling["boundary"]) < 3:
            continue
        shape = Polygon([xy(vertex["point"]) for vertex in ceiling["boundary"]], [[xy(vertex["point"]) for vertex in hole] for hole in ceiling["holes"]])
        if not shape.contains(Point(point)):
            continue
        drop = ceiling["offset"] + thickness
        if ceiling.get("slope"):
            direction = ceiling["slope"]["direction"]
            along = lambda x, y: x * math.cos(direction) + y * math.sin(direction)
            uphill = min(along(x, y) for x, y in shape.exterior.coords)
            drop += math.tan(ceiling["slope"]["angle"]) * (along(point[0], point[1]) - uphill)
        candidates.append((-drop, ceiling_id, drop))
    if not candidates:
        return None
    _, ceiling_id, drop = sorted(candidates)[0]
    return ceiling_id, drop


# endregion 🔖️Hung


# region 🔖️Table
def refused(status):
    return {"status": status, "area": 0.0, "perimeter": 0.0, "net_floor_area": 0.0, "clear_height": 0.0, "volume": 0.0, "hole_count": 0, "ceiling_slab": "", "ceiling": "", "bounding_walls": []}


def row_of(snapshot, storey, status, area, perimeter, holes, shape, point, columns, shapes):
    """🏠️ One table row from the measured room."""
    height = snapshot["storeys"][storey]["height"]
    found = ceiling(snapshot, storey, point)
    clear = height if found is None else max(height + found[2] - found[1], 0.0)
    below = hung(snapshot, storey, point)
    if below is not None:
        clear = min(clear, max(height - below[1], 0.0))
    cut = shape.difference(unary_union(columns)).area if columns else shape.area
    net = max(area - (shape.area - cut), 0.0)
    walls = sorted(wall_id for wall_id, footprint in shapes.items() if shape.boundary.intersection(footprint.boundary).length > CONTACT)
    return {"status": status, "area": area, "perimeter": perimeter, "net_floor_area": net, "clear_height": clear, "volume": area * clear, "hole_count": holes, "ceiling_slab": "" if found is None else found[0], "ceiling": "" if below is None else below[0], "bounding_walls": walls}


def tables(snapshot):
    """🏠️ The `🏠️spaces` table of a snapshot."""
    result = {}
    cache = {}
    for space_id, space in snapshot["spaces"].items():
        storey = space["storey"]
        if storey not in cache:
            shapes = footprints(snapshot, storey)
            cache[storey] = (shapes, *free_faces(shapes), column_shapes(snapshot, storey))
        shapes, faces, union, columns = cache[storey]
        tag, body = variant(space["boundary"])
        if tag == "Explicit":
            vertices = vertices_of(body["outline"])
            area = abs(loop_area(vertices))
            if len(vertices) < 2 or area <= 1e-12:
                result[space_id] = refused("InvalidOutline")
                continue
            shape = sampled(vertices)
            result[space_id] = row_of(snapshot, storey, "Explicit", area, loop_length(vertices), 0, shape, shape.representative_point().coords[0], columns, shapes)
            continue
        status, face = room_for(xy(body["seed"]), faces, union)
        if face is None:
            result[space_id] = refused(status)
            continue
        result[space_id] = row_of(snapshot, storey, status, face.area, face.length, len(face.interiors), face, xy(body["seed"]), columns, shapes)
    return result


# endregion 🔖️Table


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Disagreements between the closed forms and what GEOS measures, plus the geometric and parametric invariants."""
    problems = []
    for storey in sorted({space["storey"] for space in snapshot["spaces"].values()}):
        shapes = footprints(snapshot, storey)
        faces, union = free_faces(shapes)
        found = []
        for space_id, space in snapshot["spaces"].items():
            tag, body = variant(space["boundary"])
            if space["storey"] != storey:
                continue
            if tag == "Explicit":
                vertices = vertices_of(body["outline"])
                if len(vertices) >= 2 and abs(loop_area(vertices)) > 1e-12:
                    shape = sampled(vertices)
                    if abs(shape.area - abs(loop_area(vertices))) > SAMPLED * max(shape.area, 1.0):
                        problems.append("%s: sampled area %.12g, closed form %.12g" % (space_id, shape.area, abs(loop_area(vertices))))
                    if abs(shape.length - loop_length(vertices)) > SAMPLED * max(shape.length, 1.0):
                        problems.append("%s: sampled perimeter %.12g, closed form %.12g" % (space_id, shape.length, loop_length(vertices)))
                continue
            status, face = room_for(xy(body["seed"]), faces, union)
            if face is None:
                continue
            found.append((space_id, face))
            if not face.is_valid:
                problems.append("%s: the room polygon is not valid" % space_id)
            if union is not None and face.intersection(union).area > EXACT:
                problems.append("%s: the room overlaps the walls by %.3g" % (space_id, face.intersection(union).area))
        for (first, left), (second, right) in zip(found, found[1:]):
            if left.intersection(right).area > EXACT and not left.equals(right):
                problems.append("%s and %s overlap" % (first, second))
    return problems + parametric_problems(snapshot) + ceiling_problems(snapshot)


def ceiling_problems(snapshot):
    """🧪️ Lowering a flat ceiling that governs a room by `delta` lowers the clear height of that room by exactly `delta`, and the rooms it does not govern keep theirs."""
    delta = 0.05
    before = tables(snapshot)
    problems = []
    for ceiling_id, ceiling in snapshot.get("ceilings", {}).items():
        if ceiling.get("slope"):
            continue
        lowered = copy.deepcopy(snapshot)
        lowered["ceilings"][ceiling_id]["offset"] += delta
        after = tables(lowered)
        for space_id, row in before.items():
            governed = row["ceiling"] == ceiling_id and row["clear_height"] > 0 and row["clear_height"] == max(snapshot["storeys"][snapshot["spaces"][space_id]["storey"]]["height"] - hung(snapshot, snapshot["spaces"][space_id]["storey"], room_point(snapshot, space_id))[1], 0.0)
            expected = row["clear_height"] - delta if governed else row["clear_height"]
            if abs(after[space_id]["clear_height"] - expected) > EXACT * 10 and row["ceiling"] != ceiling_id:
                problems.append("%s: lowering %s changed a room it does not hang over (%.12g to %.12g)" % (space_id, ceiling_id, row["clear_height"], after[space_id]["clear_height"]))
            elif governed and abs(after[space_id]["clear_height"] - expected) > EXACT * 10:
                problems.append("%s: lowering %s by %g changed the clear height from %.12g to %.12g" % (space_id, ceiling_id, delta, row["clear_height"], after[space_id]["clear_height"]))
    return problems


def room_point(snapshot, space_id):
    """📍️ The point the clear height of a space is measured at: its seed, or the representative point of its outline."""
    tag, body = variant(snapshot["spaces"][space_id]["boundary"])
    if tag == "Explicit":
        return sampled(vertices_of(body["outline"])).representative_point().coords[0]
    return xy(body["seed"])


def parametric_problems(snapshot):
    """🧪️ Moving the ground partition by `delta` moves `delta * clear width` of area from the kitchen to the living room (when the snapshot has both)."""
    wall = snapshot["walls"].get("ground-partition")
    if wall is None or "sp-living" not in snapshot["spaces"] or "sp-kitchen" not in snapshot["spaces"]:
        return []
    moved = copy.deepcopy(snapshot)
    tag, body = variant(moved["walls"]["ground-partition"]["axis"])
    for end in ("start", "end"):
        body[end]["x"] += 0.5
    before, after = tables(snapshot), tables(moved)
    clear_width = 6.0 - 0.2
    expected = {"sp-living": 0.5 * clear_width, "sp-kitchen": -0.5 * clear_width}
    problems = []
    for space_id, delta in expected.items():
        if abs(after[space_id]["area"] - before[space_id]["area"] - delta) > EXACT * 10:
            problems.append("%s: moving the partition by 0.5 changed the area by %.12g, expected %.12g" % (space_id, after[space_id]["area"] - before[space_id]["area"], delta))
    return problems


# endregion 🔖️Audit


# region 🔖️Projection
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return levels_oracle().compare(expected, actual, path)


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def spaces_handler(ctx):
    """🏠️ Oracle answer for `🏠️spaces`, after GEOS and the parametric law agreed with it."""
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

    return Adapter("python").oracle("spaces-rooms", spaces_handler).oracle("spaces-hung-edit", spaces_handler)


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
        target = case / "💡️inference" / "🏠️spaces" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "spaces")]
        print("%s: shapely %s, %d spaces" % (case.name, shapely.__version__, len(snapshot["spaces"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
