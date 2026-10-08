#!/usr/bin/env python3
"""🪜️ Third-party ORACLE for the `s.bim.model@1` inferences `🪜️storey-levels` and `🧱️wall-layout`.

The subject (Rust, `semio-s-artifact-bim-model`) derives every storey elevation and, per wall, the resolved base and top, height,
thickness, centreline length, location-line face offsets, join-trimmed face curves and footprint loop, joins, side areas and volume
from a snapshot of authored parameters. This file reproduces those numbers from the SAME committed snapshot and audits them with a
library that has never seen this repository:

* the plan geometry (offsets, joins, footprints) is the oracle of the sibling case `../🧱️infer-bim-1-wall-joins`, where `shapely` 2
  (GEOS) re-measures every footprint, face length and join invariant (this file imports it and adds the heights);
* the parametric law is a metamorphic property: raising one storey height by `delta` moves exactly the storeys above it, grows
  exactly the walls that end at its top, and leaves everything else untouched.

Elevations are a running sum and no third-party engine owns that arithmetic, so they are computed with
`math.fsum` (exactly rounded) against the subject's plain f64 sum, held by the parametric law, and then
re-measured as `z` extents by the sibling case `../🔳️infer-bim-1-wall-solids` with `ifcopenshell`.

The committed expectations under `🧫️fixtures/💡️inferences/<case>/💡️inference/<inference>/🔣️.json` are
WRITTEN by this file (`write`), never by hand, and the Rust subject is compared against them.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r2-design.md — snapshot and inference catalogue
"""

# region 🔖️Imports
import importlib.util
import itertools
import json
import math
import sys
from pathlib import Path

import shapely

# endregion 🔖️Imports


# region 🔖️Vocabulary
ABSOLUTE_TOLERANCE = 1e-9
"""⚖️ Exact arithmetic (elevations, heights, thickness, line length, volumes of straight walls)."""

SAMPLED_TOLERANCE = 1e-5
"""⚖️ Relative tolerance where shapely measures a sampled arc against its closed form."""

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


# region 🔖️Levels
def storey_levels(snapshot):
    """🪜️ Level 0 is the building datum at elevation 0. A storey with a level >= 0 sits on the exactly rounded sum of the heights of
    the storeys with 0 <= level below it; a storey with a level < 0 hangs below the datum by the exactly rounded sum of the
    heights of the storeys with its level <= level < 0. Absolute values add the site and building elevations."""
    result = {}
    for building_id, building in snapshot["buildings"].items():
        rows = sorted(((storey["level"], storey_id) for storey_id, storey in snapshot["storeys"].items() if storey["building"] == building_id))
        datum = snapshot["sites"][building["site"]]["elevation"] + building["elevation"]
        height = lambda storey_id: snapshot["storeys"][storey_id]["height"]
        spans = {}
        stacked = []
        for level, storey_id in rows:
            if level >= 0:
                low = math.fsum(stacked)
                stacked.append(height(storey_id))
                spans[storey_id] = (low, math.fsum(stacked))
        hung = []
        for level, storey_id in reversed(rows):
            if level < 0:
                high = -math.fsum(hung)
                hung.append(height(storey_id))
                spans[storey_id] = (-math.fsum(hung), high)
        for storey_id, (low, high) in spans.items():
            result[storey_id] = {"elevation": low, "top_elevation": high, "absolute_elevation": datum + low, "absolute_top_elevation": datum + high}
    return result


# endregion 🔖️Levels


# region 🔖️Walls
def top_z(snapshot, levels, wall, base_z):
    """🔝️ The resolved top of a wall from its `TopConstraint`."""
    tag, body = variant(wall["top"])
    storey = wall["storey"]
    if tag == "Unconnected":
        return base_z + body["height"]
    if tag == "StoreyTop":
        return levels[storey]["top_elevation"] + body["offset"]
    if tag == "Storey":
        if snapshot["storeys"][body["storey"]]["building"] != snapshot["storeys"][storey]["building"]:
            raise AssertionError("wall constrained to a storey of another building")
        return levels[body["storey"]]["elevation"] + body["offset"]
    raise AssertionError("unknown top constraint %r" % tag)


def geometry():
    """🧭️ The plan-geometry oracle of the sibling case `../🧱️infer-bim-1-wall-joins`, imported by path (the cases are folders, not packages)."""
    key = "bim_oracle_wall_joins"
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / "🧱️infer-bim-1-wall-joins" / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def wall_layouts(snapshot, levels):
    """🧱️ Per wall: the resolved `base_z`/`top_z`/`height`, the plan geometry of the sibling oracle (thickness, centreline `length`, face
    offsets, layer interfaces, join-trimmed faces, footprint loop, joins) and the quantities that follow from the height: `side_area`
    (length times height), `left_area`/`right_area` (face length times height) and `volume` (footprint area times height)."""
    plan = geometry().plan_geometry(snapshot)
    result = {}
    for wall_id, wall in snapshot["walls"].items():
        base = levels[wall["storey"]]["elevation"] + wall["base_offset"]
        top = top_z(snapshot, levels, wall, base)
        height = top - base
        row = plan[wall_id]
        result[wall_id] = {
            **row,
            "base_z": base,
            "top_z": top,
            "height": height,
            "side_area": row["length"] * height,
            "left_area": row["left_length"] * height,
            "right_area": row["right_length"] * height,
            "volume": row["footprint_area"] * height,
        }
    return result


# endregion 🔖️Walls


# region 🔖️Audit
def close(actual, expected, relative):
    """⚖️ Whether two numbers agree within the absolute or relative tolerance."""
    return abs(actual - expected) <= (SAMPLED_TOLERANCE * max(abs(expected), 1.0) if relative else ABSOLUTE_TOLERANCE * max(abs(expected), 1.0))


def shapely_problems(snapshot, layouts):
    """〰️ Every disagreement between the analytic plan geometry and what shapely measures on it (see the sibling case)."""
    return geometry().shapely_problems(snapshot, geometry().plan_geometry(snapshot))


# endregion 🔖️Audit


# region 🔖️Parametricity
def parametric_problems(snapshot):
    """♻️ Raising one storey's height by `delta` moves exactly the storeys above it in its building by `delta`, grows exactly
    the walls that end at that storey's top, and leaves every other wall and storey unchanged."""
    edited_id, delta = "st-ground", 0.4
    before_levels = storey_levels(snapshot)
    before = wall_layouts(snapshot, before_levels)
    edited = json.loads(json.dumps(snapshot))
    edited["storeys"][edited_id]["height"] += delta
    after_levels = storey_levels(edited)
    after = wall_layouts(edited, after_levels)
    pivot = snapshot["storeys"][edited_id]
    shift = {storey_id: delta if storey["building"] == pivot["building"] and storey["level"] > pivot["level"] else 0.0 for storey_id, storey in snapshot["storeys"].items()}
    top_shift = {storey_id: shift[storey_id] + (delta if storey_id == edited_id else 0.0) for storey_id in shift}
    problems = []
    for storey_id in snapshot["storeys"]:
        for field, expected in (("elevation", shift[storey_id]), ("top_elevation", top_shift[storey_id])):
            moved = after_levels[storey_id][field] - before_levels[storey_id][field]
            if not close(moved, expected, False):
                problems.append("%s: %s moved by %.12g, expected %.12g" % (storey_id, field, moved, expected))
    for wall_id, wall in snapshot["walls"].items():
        tag, body = variant(wall["top"])
        expected_top = {"Unconnected": shift[wall["storey"]], "StoreyTop": top_shift[wall["storey"]], "Storey": shift.get(body.get("storey", ""), 0.0)}[tag]
        for field, expected in (("base_z", shift[wall["storey"]]), ("top_z", expected_top)):
            moved = after[wall_id][field] - before[wall_id][field]
            if not close(moved, expected, False):
                problems.append("%s: %s moved by %.12g, expected %.12g" % (wall_id, field, moved, expected))
    return problems


# endregion 🔖️Parametricity


# region 🔖️Projection
def rounded(value):
    """🔢️ Rounds every number to 12 decimals (recursively) so the committed JSON is stable across platforms."""
    if isinstance(value, dict):
        return {key: rounded(item) for key, item in value.items()}
    if isinstance(value, list):
        return [rounded(item) for item in value]
    return round(value, 12) if isinstance(value, float) else value


def projections(snapshot):
    """🎯️ The two inference outputs this oracle answers for, keyed by inference slug."""
    levels = storey_levels(snapshot)
    return {"🪜️storey-levels": rounded(levels), "🧱️wall-layout": rounded(wall_layouts(snapshot, levels))}


def problems_of(snapshot):
    """🩺️ Every disagreement the two independent routes (shapely, the parametric law) found."""
    levels = storey_levels(snapshot)
    layouts = wall_layouts(snapshot, levels)
    return shapely_problems(snapshot, layouts) + parametric_problems(snapshot)


def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table, recursively: numbers within the tolerance, arrays in order."""
    if isinstance(expected, dict) and isinstance(actual, dict):
        problems = ["%s: members differ: committed %s, computed %s" % (path, sorted(expected), sorted(actual))] if sorted(expected) != sorted(actual) else []
        for key in sorted(set(expected) & set(actual)):
            problems += compare(expected[key], actual[key], path + "." + key if path else key)
        return problems
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return ["%s: %d committed items, %d computed" % (path, len(expected), len(actual))]
        return [problem for index, (left, right) in enumerate(zip(expected, actual)) for problem in compare(left, right, "%s[%d]" % (path, index))]
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)) and not isinstance(expected, bool):
        return [] if close(actual, expected, False) else ["%s: committed %.12g, computed %.12g" % (path, expected, actual)]
    return [] if expected == actual else ["%s: committed %r, computed %r" % (path, expected, actual)]


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def storey_levels_handler(ctx):
    """🪜️ Oracle answer for `🪜️storey-levels`."""
    from semio_repo_test import Outcome

    payload = projections(case_snapshot(ctx))["🪜️storey-levels"]
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def wall_layout_handler(ctx):
    """🧱️ Oracle answer for `🧱️wall-layout`, after shapely and the parametric law agreed with it."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = projections(snapshot)["🧱️wall-layout"]
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("storey-levels", storey_levels_handler).oracle("wall-layout", wall_layout_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectations; `write` regenerates them."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = load_snapshot(snapshot_path)
        failures += ["%s: %s" % (case.name, problem) for problem in problems_of(snapshot)]
        for slug, table in projections(snapshot).items():
            target = case / "💡️inference" / slug / "🔣️.json"
            if command == "write":
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
                continue
            committed = json.loads(target.read_text(encoding="utf-8"))
            failures += ["%s/%s: %s" % (case.name, slug, problem) for problem in compare(committed, table, slug)]
        print("%s: shapely %s, %d storeys, %d walls" % (case.name, shapely.__version__, len(snapshot["storeys"]), len(snapshot["walls"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
