#!/usr/bin/env python3
"""🔲️ Third-party ORACLE (shapely 2 over GEOS) for the ceilings of `s.bim.model@1`: their take-off and their vertical span.

The subject (Rust, `semio-s-artifact-bim-model`) derives, for every ceiling, the areas, the perimeter, the layer volumes and mass of the `🧮️quantities`
inference and the span of the solid plus the z of its underside at a plan point (the number the `🏠️spaces` inference takes the clear height from). This file
reproduces the table from the SAME committed snapshot alone with a library that has never seen this repository: `shapely` builds the region of the boundary with
its holes (every bulged edge sampled into 4096 chords), audits the exact areas and the perimeter against `Polygon.area` and `Polygon.length`, and answers the
probe with `Polygon.contains`.

* `gross_area` the boundary, `net_area` the boundary less its holes, `surface_area` the sloped underside (`net_area / cos(angle)`), `perimeter` the boundary and every hole;
* `width` the sum of the layer thicknesses, `gross_volume` and `net_volume` the areas times it, `mass` the layer volumes times the density of their materials;
* `top_z` the storey top less the `offset`, `bottom_z` the lowest point of the underside: the top less the fall (the extent of the boundary along the fall direction times
  `tan(angle)`) less the width;
* `underside_z` the z of the visible underside at the probe, `null` where the probe lies in a hole or outside: the top plane keeps its drop at the uphill edge of the boundary (the
  point of the boundary with the smallest projection on the fall direction) and falls by `tan(angle)` per metre along it. The probe is the first vertex moved 5 % towards the mean of
  the vertices of the boundary.

A ceiling whose type is unknown or has no layers, or whose boundary has fewer than three vertices, has no row (`null`): the subject emits no solid for it.

Standalone use (no test host needed):

    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🔲️ceilings>
    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🔲️ceilings>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🧬️schema/💡️inferences/🧊️element-solids/🔲️ceilings/🦀️.rs — the solid and the underside
@see ../../🧬️schema/💡️inferences/🧮️quantities/🦀️.rs — the take-off
"""

# region 🔖️Imports
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import Point

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
"""⚖️ Tolerance between the closed forms and the table the subject reports."""

SAMPLED = 1e-5
"""🌙️ Relative tolerance of the areas and the perimeter that shapely measures on the sampled arcs."""

PROBE = 0.05
"""📍️ Fraction of the way from the first vertex to the mean of the vertices where the underside is read."""


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


def rest():
    return load_sibling("solids-rest")


# endregion 🔖️Vocabulary


# region 🔖️Measure
def perimeter(loop):
    """📏️ Exact length of a bulged loop: the chords of the straight edges and the arcs of the bulged ones."""
    total = 0.0
    for index, vertex in enumerate(loop):
        a = (vertex["point"]["x"], vertex["point"]["y"])
        following = loop[(index + 1) % len(loop)]["point"]
        b = (following["x"], following["y"])
        if abs(vertex["bulge"]) > 1e-12:
            _, radius, sweep = rest().arc(a, b, vertex["bulge"])
            total += radius * abs(sweep)
        else:
            total += math.hypot(b[0] - a[0], b[1] - a[1])
    return total


def probe_of(boundary):
    """📍️ The plan point the underside is read at."""
    points = [(vertex["point"]["x"], vertex["point"]["y"]) for vertex in boundary]
    mean = (math.fsum(x for x, _ in points) / len(points), math.fsum(y for _, y in points) / len(points))
    return (points[0][0] + PROBE * (mean[0] - points[0][0]), points[0][1] + PROBE * (mean[1] - points[0][1]))


def row_of(snapshot, levels, ceiling_id):
    """🔲️ The row of one ceiling, `None` when it has no extent."""
    ceiling = snapshot["ceilings"][ceiling_id]
    kind = snapshot.get("ceiling_types", {}).get(ceiling["ceiling_type"])
    layers = [layer for layer in kind["layers"]] if kind else []
    width = math.fsum(max(layer["thickness"], 0.0) for layer in layers)
    if width <= 1e-12 or len(ceiling["boundary"]) < 3:
        return None
    holes = ceiling.get("holes", [])
    polygon, net, _ = rest().region(ceiling["boundary"], holes)
    gross = rest().closed_area(ceiling["boundary"])
    around = perimeter(ceiling["boundary"]) + math.fsum(perimeter(hole) for hole in holes)
    assert abs(polygon.length - around) <= SAMPLED * max(around, 1.0), "shapely perimeter %r vs closed form %r" % (polygon.length, around)
    assert abs(rest().Polygon(rest().sampled(ceiling["boundary"])).area - gross) <= SAMPLED * max(gross, 1.0)
    slope = ceiling.get("slope")
    angle = slope["angle"] if slope else 0.0
    direction = slope["direction"] if slope else 0.0
    along = lambda x, y: x * math.cos(direction) + y * math.sin(direction)
    boundary = rest().Polygon(rest().sampled(ceiling["boundary"]))
    projections = [along(x, y) for x, y in boundary.exterior.coords]
    fall = math.tan(angle) * (max(projections) - min(projections)) if slope else 0.0
    top = levels[ceiling["storey"]][1] - ceiling["offset"]
    point = probe_of(ceiling["boundary"])
    drop = math.tan(angle) * (along(*point) - min(projections)) if slope else 0.0
    under = top - drop - width if polygon.contains(Point(point)) else None
    mass = math.fsum(net * max(layer["thickness"], 0.0) * snapshot["materials"][layer["material"]]["density"] for layer in layers)
    return {
        "gross_area": gross,
        "net_area": net,
        "surface_area": net / max(abs(math.cos(angle)), 1e-9),
        "perimeter": around,
        "width": width,
        "gross_volume": gross * width,
        "net_volume": net * width,
        "mass": mass,
        "top_z": top,
        "bottom_z": top - abs(fall) - width,
        "underside_z": under,
    }


def tables(snapshot):
    """🔲️ The table of a snapshot: one row per ceiling, `None` for a ceiling without extent."""
    levels = rest().storey_levels(snapshot)
    return {ceiling_id: row_of(snapshot, levels, ceiling_id) for ceiling_id in sorted(snapshot.get("ceilings", {}))}


def canonical(value):
    """🧹️ The JSON form committed to the fixture: floats rounded to 12 decimals so reruns are byte stable."""
    return rest().canonical(value)


# endregion 🔖️Measure


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def ceilings_handler(ctx):
    """🔲️ Oracle answer for the ceiling take-off and span."""
    from semio_repo_test import Outcome

    payload = canonical(tables(case_snapshot(ctx)))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("ceilings-takeoff", ceilings_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        table = canonical(tables(json.loads(snapshot_path.read_text(encoding="utf-8"))))
        target = case / "💡️inference" / "🔲️ceilings" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        elif json.loads(target.read_text(encoding="utf-8")) != table:
            failures.append("%s: the committed table differs from the oracle" % case.name)
        print("%s: shapely %s, %d ceilings (%d without extent)" % (case.name, shapely.__version__, len(table), sum(1 for row in table.values() if row is None)))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
