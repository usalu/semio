#!/usr/bin/env python3
"""⚖️ Third-party ORACLE for the `s.bim.model@1` inference `⚖️rule-results`.

The subject (Rust, `semio-s-artifact-bim-model`) checks the authored rules (a limit, a direction, a severity and a scope) against the measures of the model and lists the members that break them. This
file re-derives the table from the SAME committed inputs without sharing a line of code with the subject: the authored snapshot (the rules and their scopes) and the committed measures of the model
(`📏️measures`: riser height, tread, width and counts of the stairs, the frame width of the doors, slope and length of the ramps, the net floor area of the zones and the resolved rooms with their
outlines, which the stair, opening, ramp, zone and space oracles audit separately). `shapely` 2 (GEOS), a library that has never seen this repository, measures the rooms: the area of an outline
polygon with its holes and the length of all its rings give the corridor width as the short side of the rectangle with that area and perimeter.

The committed expectation under `🧫️fixtures/💡️inferences/⚖️rule-results/<case>/💡️inference/⚖️rules/🔣️.json` is WRITTEN by this file (`write`), never by hand.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/⚖️rule-results>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/⚖️rule-results>
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import Polygon

# endregion 🔖️Imports

EXACT = 1e-9
MAXIMUM = {"MaxRiser", "MaxRampSlope", "MaxCompartmentArea"}


# region 🔖️Elements
def phase_of(snapshot, element):
    """🕰️ The effective phase of an element: its own, New for a kind without a phase."""
    for collection in ("walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces"):
        row = snapshot.get(collection, {}).get(element)
        if row:
            return row.get("phase", "New")
    host = snapshot.get("openings", {}).get(element, {}).get("host")
    for collection in ("walls", "curtain_walls"):
        if host in snapshot.get(collection, {}):
            return snapshot[collection][host].get("phase", "New")
    return "New"


def storey_of(snapshot, element):
    row = snapshot.get("stairs", {}).get(element) or snapshot.get("ramps", {}).get(element) or snapshot.get("spaces", {}).get(element)
    if row:
        return row["storey"]
    host = snapshot.get("openings", {}).get(element, {}).get("host")
    for collection in ("walls", "curtain_walls"):
        if host in snapshot.get(collection, {}):
            return snapshot[collection][host]["storey"]
    return None


def covers(snapshot, scope, element, storey):
    """🎯️ Whether the scope covers an element standing on `storey` (a zone stands on none and is covered by every storey restriction)."""
    return (
        (not scope["ids"] or element in scope["ids"])
        and (not scope["storeys"] or storey is None or storey in scope["storeys"])
        and (not scope["phases"] or phase_of(snapshot, element) in scope["phases"])
    )


def members(snapshot, rule):
    """🧩️ The (id, storey) members a rule measures, in id order."""
    kind, scope = rule["kind"], rule["scope"]
    wanted = scope["filter"].strip().lower()
    rows = []
    if kind in ("MaxRiser", "MinTread", "MinStairWidth"):
        rows = [(i, r["storey"]) for i, r in snapshot.get("stairs", {}).items() if covers(snapshot, scope, i, r["storey"])]
    elif kind == "MaxRampSlope":
        rows = [(i, r["storey"]) for i, r in snapshot.get("ramps", {}).items() if covers(snapshot, scope, i, r["storey"])]
    elif kind == "MinDoorWidth":
        rows = [(i, storey_of(snapshot, i)) for i, r in snapshot.get("openings", {}).items() if "Door" in r["kind"] and storey_of(snapshot, i) is not None and covers(snapshot, scope, i, storey_of(snapshot, i))]
    elif kind in ("MinClearHeight", "MinCorridorWidth"):
        rows = [(i, r["storey"]) for i, r in snapshot.get("spaces", {}).items() if (not wanted or r["usage"].strip().lower() == wanted) and covers(snapshot, scope, i, r["storey"])]
    elif kind == "MaxCompartmentArea":
        rows = [(i, None) for i, r in snapshot.get("zones", {}).items() if (not wanted or r["category"].strip().lower() == wanted) and covers(snapshot, scope, i, None)]
    return sorted(rows)


# endregion 🔖️Elements


# region 🔖️Measures
def equivalent_width(room):
    """📐️ The short side of the rectangle with the area and the perimeter of the room outline (shapely measures both)."""
    outline = [(v["point"]["x"], v["point"]["y"]) for v in room["outline"]]
    holes = [[(v["point"]["x"], v["point"]["y"]) for v in hole] for hole in room["holes"]]
    polygon = Polygon(outline, holes)
    area, perimeter = polygon.area, polygon.exterior.length + sum(ring.length for ring in polygon.interiors)
    return max((perimeter - math.sqrt(max(perimeter * perimeter - 16 * area, 0.0))) / 4.0, 0.0)


def measure(rule, element, measures):
    kind = rule["kind"]
    stair, ramp, zone, room = measures["stairs"].get(element), measures["ramps"].get(element), measures["zones"].get(element), measures["rooms"].get(element)
    if kind == "MaxRiser":
        return stair["riser_height"] if stair and stair["riser_count"] > 0 else None
    if kind == "MinTread":
        return stair["tread"] if stair and stair["tread_count"] > 0 else None
    if kind == "MinStairWidth":
        return stair["width"] if stair and stair["riser_count"] > 0 else None
    if kind == "MinDoorWidth":
        return measures["frames"].get(element)
    if kind == "MaxRampSlope":
        return ramp["slope"] if ramp and ramp["length"] > 0 else None
    if kind == "MaxCompartmentArea":
        return zone["net_area"] if zone and zone["resolved"] > 0 else None
    if room is None or room["status"] not in ("Inferred", "Explicit"):
        return None
    return room["clear_height"] if kind == "MinClearHeight" else equivalent_width(room)


def table(snapshot, measures):
    """⚖️ The table of every rule of the snapshot over the committed measures."""
    out = {}
    for rule_id, rule in sorted(snapshot.get("rules", {}).items()):
        checked, violations = 0, []
        for element, storey in members(snapshot, rule):
            value = measure(rule, element, measures)
            if value is None:
                continue
            checked += 1
            broken = value > rule["limit"] + EXACT if rule["kind"] in MAXIMUM else value < rule["limit"] - EXACT
            if broken:
                violations.append({"element": element, "measured": value, "limit": rule["limit"], "storey": storey or ""})
        out[rule_id] = {"checked": checked, "violations": violations}
    return out


def audit(snapshot, measures):
    """🔎️ Independent checks: the limits are positive, every violation is on the wrong side of its limit and the rooms are valid polygons."""
    problems = []
    for rule_id, rule in snapshot.get("rules", {}).items():
        if not (rule["limit"] > 0 and math.isfinite(rule["limit"])):
            problems.append("%s: the limit must be positive" % rule_id)
    for element, room in measures["rooms"].items():
        outline = [(v["point"]["x"], v["point"]["y"]) for v in room["outline"]]
        if room["status"] in ("Inferred", "Explicit") and not Polygon(outline).is_valid:
            problems.append("%s: the outline is not a valid polygon" % element)
    return problems


def compare(expected, got):
    """⚖️ The differences between two tables: structure exactly, numbers within a nanometre."""
    if sorted(expected) != sorted(got):
        return ["rules differ: %s vs %s" % (sorted(expected), sorted(got))]
    problems = []
    for rule_id in expected:
        a, b = expected[rule_id], got[rule_id]
        if a["checked"] != b["checked"]:
            problems.append("%s.checked: %s vs %s" % (rule_id, a["checked"], b["checked"]))
        if [v["element"] for v in a["violations"]] != [v["element"] for v in b["violations"]]:
            problems.append("%s: violations %s vs %s" % (rule_id, [v["element"] for v in a["violations"]], [v["element"] for v in b["violations"]]))
            continue
        for x, y in zip(a["violations"], b["violations"]):
            if abs(x["measured"] - y["measured"]) > EXACT or abs(x["limit"] - y["limit"]) > EXACT or x["storey"] != y["storey"]:
                problems.append("%s %s: %s vs %s" % (rule_id, x["element"], x, y))
    return problems


# endregion 🔖️Measures


# region 🔖️Handlers
def rules_handler(ctx):
    """🧭️ Oracle handler: the table of the committed inputs of a scenario (registered in the ORACLE role only)."""
    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "snapshot" in uri)))
    measures = json.loads(ctx.input_bytes(next(uri for uri in uris if "measures" in uri)))
    return Outcome(table(snapshot, measures))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("rules-limits", rules_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        measures = json.loads((case / "📏️measures" / "🔣️.json").read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in audit(snapshot, measures)]
        computed = table(snapshot, measures)
        target = case / "💡️inference" / "⚖️rules" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed)]
        print("%s: shapely %s, %d rules, %d violations" % (case.name, shapely.__version__, len(computed), sum(len(v["violations"]) for v in computed.values())))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
