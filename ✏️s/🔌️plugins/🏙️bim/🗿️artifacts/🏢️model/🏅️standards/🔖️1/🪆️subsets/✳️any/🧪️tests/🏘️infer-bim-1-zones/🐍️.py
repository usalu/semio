#!/usr/bin/env python3
"""🏘️ Third-party ORACLE for the `s.bim.model@1` inference `🏘️zones`.

Zones and area schemes only add up what the spaces already measure. The subject (Rust, `semio-s-artifact-bim-model`) sums the take-off rows of the
spaces a zone or a scheme counts. This file reproduces the table from the SAME committed snapshot with a library that has never seen this
repository: the rooms come from `shapely` 2 (GEOS) through the sibling oracles `../🏠️infer-bim-1-spaces` (areas, net floor areas, volumes) and
`../🪣️infer-bim-1-finishes` (finish areas), and the sums are `math.fsum`, which is exactly rounded, not the subject's running f64 sum.

The rules are restated here from the authored records alone: a space belongs to the zone it names; an area scheme counts a space when its usage list
is empty or names the usage of the space and its zone list is empty or names the zone of the space; the area of a scheme is the gross area or the net floor
area of the counted rooms, its occupancy the density of the zone of each counted space times its net floor area.

Audits beyond the table: every zone total is the sum of the schemes that count exactly its spaces (a scheme with that zone list and an empty usage list
equals the zone), the net area never exceeds the gross area, and a space in no zone adds no occupancy.

The committed expectations under `🧫️fixtures/💡️inferences/🏘️zones/<case>/💡️inference/🏘️zones/🔣️.json` are WRITTEN by this file (`write`), never by hand,
and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🏘️zones>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🏘️zones>

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

# endregion 🔖️Imports


# region 🔖️Vocabulary
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


def finishes():
    return load_sibling("🪣️infer-bim-1-finishes", "finishes")


# endregion 🔖️Vocabulary


# region 🔖️Rules
def counts(scheme, space):
    """🗃️ Whether the rule of a scheme counts a space."""
    return (not scheme["usages"] or space["usage"] in scheme["usages"]) and (not scheme["zones"] or space.get("zone") in scheme["zones"])


def densities(snapshot):
    return {zone_id: zone["occupancy_density"] for zone_id, zone in snapshot.get("zones", {}).items()}


# endregion 🔖️Rules


# region 🔖️Table
def measured(snapshot):
    """🏠️ `{space id: (room row, finish row or None)}`: the room of every space and the finish areas of the resolved ones."""
    rooms = spaces().tables(snapshot)
    finished = finishes().tables(snapshot)
    return {space_id: (rooms[space_id], finished.get(space_id)) for space_id in snapshot["spaces"]}


def zone_row(snapshot, zone_id, rows):
    members = [space_id for space_id, space in snapshot["spaces"].items() if space.get("zone") == zone_id]
    resolved = [space_id for space_id in members if rows[space_id][1] is not None]
    density = densities(snapshot)[zone_id]
    return {
        "spaces": len(members),
        "resolved": len(resolved),
        "area": math.fsum(rows[space_id][0]["area"] for space_id in resolved),
        "net_area": math.fsum(rows[space_id][0]["net_floor_area"] for space_id in resolved),
        "volume": math.fsum(rows[space_id][0]["volume"] for space_id in resolved),
        "occupancy": math.fsum(density * rows[space_id][0]["net_floor_area"] for space_id in resolved),
        "floor_finish_area": math.fsum(rows[space_id][1]["floor_area"] for space_id in resolved),
        "wall_finish_area": math.fsum(rows[space_id][1]["wall_area"] for space_id in resolved),
        "ceiling_finish_area": math.fsum(rows[space_id][1]["ceiling_area"] for space_id in resolved),
    }


def scheme_row(snapshot, scheme, rows):
    counted = [space_id for space_id, space in snapshot["spaces"].items() if counts(scheme, space)]
    resolved = [space_id for space_id in counted if rows[space_id][1] is not None]
    density = densities(snapshot)
    measure = "area" if scheme["measure"] == "Gross" else "net_floor_area"
    return {
        "spaces": len(counted),
        "resolved": len(resolved),
        "area": math.fsum(rows[space_id][0][measure] for space_id in resolved),
        "volume": math.fsum(rows[space_id][0]["volume"] for space_id in resolved),
        "occupancy": math.fsum(density.get(snapshot["spaces"][space_id].get("zone"), 0.0) * rows[space_id][0]["net_floor_area"] for space_id in resolved),
    }


def tables(snapshot):
    """🏘️ The `🏘️zones` table of a snapshot: the totals of every zone and every area scheme."""
    rows = measured(snapshot)
    return {
        "zones": {zone_id: zone_row(snapshot, zone_id, rows) for zone_id in snapshot.get("zones", {})},
        "schemes": {scheme_id: scheme_row(snapshot, scheme, rows) for scheme_id, scheme in snapshot.get("area_schemes", {}).items()},
    }


# endregion 🔖️Table


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Disagreements between the zones and the schemes that count the same spaces, plus the invariants of the sums."""
    problems = []
    result = tables(snapshot)
    for zone_id, row in result["zones"].items():
        if row["net_area"] > row["area"] + 1e-9:
            problems.append("%s: the net area exceeds the gross area" % zone_id)
        twin = {"measure": "Net", "usages": [], "zones": [zone_id]}
        counted = scheme_row(snapshot, twin, measured(snapshot))
        for key in ("spaces", "resolved", "volume", "occupancy"):
            if abs(counted[key] - row[key]) > 1e-9 * max(abs(row[key]), 1.0):
                problems.append("%s: a scheme that counts exactly the zone disagrees on %s (%s against %s)" % (zone_id, key, counted[key], row[key]))
        if abs(counted["area"] - row["net_area"]) > 1e-9 * max(row["net_area"], 1.0):
            problems.append("%s: a net scheme that counts exactly the zone disagrees on the area" % zone_id)
    for scheme_id, row in result["schemes"].items():
        if row["resolved"] > row["spaces"]:
            problems.append("%s: more resolved spaces than counted spaces" % scheme_id)
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


def zones_handler(ctx):
    """🏘️ Oracle answer for `🏘️zones`, after the audits agreed with it."""
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

    return Adapter("python").oracle("zones-zoning", zones_handler)


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
        target = case / "💡️inference" / "🏘️zones" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "zones")]
        print("%s: shapely %s, %d zones, %d schemes" % (case.name, shapely.__version__, len(table["zones"]), len(table["schemes"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
