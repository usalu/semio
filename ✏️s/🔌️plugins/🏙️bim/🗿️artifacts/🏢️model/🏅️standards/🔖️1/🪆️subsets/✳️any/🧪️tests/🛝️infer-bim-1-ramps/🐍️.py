#!/usr/bin/env python3
"""🛝️ Third-party ORACLE for the `s.bim.model@1` inference `🛝️ramp-runs`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per ramp, the rise from the storey levels and the top constraint, the arc length of the centre-line
path (vertices with bulges), the flat landings (foot, head and a centred one on every corner), the sloped flights between them, the slope `|rise| / run_length`
and the code flags. No third-party engine owns ramp arithmetic, so this file is a second, independently written implementation of the same rules (arc length
from the bulge by the circular-segment formula, corners from tangent angles, landings merged on sorted intervals) and then AUDITS the result with `shapely` 2
(GEOS), a library that has never seen this repository: the path becomes a LineString (every arc sampled into 4096 chords) whose GEOS length must measure the
table's `length`; `shapely.ops.substring` cuts it into the flights and landings, which must tile it exactly and measure their own lengths; the flat-capped,
mitred buffer of the path must have the area `width * length` of the strip; and the heights must be continuous across every flight and landing. The parametric
law is a metamorphic property: raising the height of a storey by `delta` changes the rise of exactly the ramps that follow a `Storey` top by `delta` times the
difference of "target above the raised storey" and "own storey above it", and leaves every `Unconnected` ramp alone.

The committed expectations under `🧫️fixtures/💡️inferences/🛝️ramp-runs/<case>/💡️inference/🛝️ramp-runs/🔣️.json` are WRITTEN by this file (`write`), never by
hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🛝️ramp-runs>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🛝️ramp-runs>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see https://shapely.readthedocs.io/en/stable/manual.html#shapely.ops.substring — the cut of a line at arc lengths
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import LineString
from shapely.ops import substring

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
TURN_EPS = 1e-3
CHORDS = 4096
SAMPLED = 1e-6


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def levels_oracle():
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels")


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


# endregion 🔖️Vocabulary


# region 🔖️Path
def segments(ramp):
    """〰️ The segments of the path as `(start, end, bulge)`: a vertex that repeats the previous one adds nothing."""
    path = ramp["path"]
    rows = []
    for first, second in zip(path, path[1:]):
        start, end = (first["point"]["x"], first["point"]["y"]), (second["point"]["x"], second["point"]["y"])
        if math.dist(start, end) > EXACT:
            rows.append((start, end, first["bulge"]))
    return rows


def sweep(bulge):
    """🌀️ The signed angle a bulge spans: `4 * atan(bulge)`."""
    return 4.0 * math.atan(bulge)


def seg_length(segment):
    """📏️ Chord for a line, `radius * |sweep|` for an arc."""
    start, end, bulge = segment
    chord = math.dist(start, end)
    if abs(bulge) < 1e-12:
        return chord
    angle = abs(sweep(bulge))
    return chord * angle / (2.0 * math.sin(angle / 2.0))


def tangents(segment):
    """🧭️ The headings at the start and the end of a segment: the chord heading turned by half the sweep."""
    start, end, bulge = segment
    chord = math.atan2(end[1] - start[1], end[0] - start[0])
    half = sweep(bulge) / 2.0
    return chord - half, chord + half


def wrapped(angle):
    """🔄️ An angle folded into `(-pi, pi]`."""
    return math.atan2(math.sin(angle), math.cos(angle))


def starts_of(rows):
    """📏️ The arc length at which every segment starts, and the whole length (exactly rounded)."""
    starts = [math.fsum(seg_length(row) for row in rows[:index]) for index in range(len(rows))]
    return starts, math.fsum(seg_length(row) for row in rows)


def corners(rows, starts):
    """🔄️ The arc lengths of the vertices where the heading changes by more than the turn tolerance."""
    return [starts[index] for index in range(1, len(rows)) if abs(wrapped(tangents(rows[index])[0] - tangents(rows[index - 1])[1])) > TURN_EPS]


def flats(ramp, length, turns):
    """🟫️ The merged flat stretches: the foot, the head and one centred on every corner, clipped to the path."""
    raw = [(0.0, ramp["landing_start"]), (length - ramp["landing_end"], length)] + [(corner - ramp["landing_turn"] / 2.0, corner + ramp["landing_turn"] / 2.0) for corner in turns]
    clipped = sorted((max(low, 0.0), min(high, length)) for low, high in raw if min(high, length) - max(low, 0.0) > EXACT)
    merged = []
    for low, high in clipped:
        if merged and low <= merged[-1][1] + EXACT:
            merged[-1] = (merged[-1][0], max(merged[-1][1], high))
        else:
            merged.append((low, high))
    return merged


def sloped(landed, length):
    """⛰️ The stretches of the path between the flat ones."""
    rows, at = [], 0.0
    for low, high in landed + [(length, length)]:
        if low - at > EXACT:
            rows.append((at, low))
        at = max(at, high)
    return rows


# endregion 🔖️Path


# region 🔖️Rules
def vertical(ramp, levels):
    """🔝️ `(base, top)` of a ramp from its storey, base offset and top constraint."""
    own = levels[ramp["storey"]]
    base = own["elevation"] + ramp["base_offset"]
    tag, body = variant(ramp["top"])
    if tag == "Unconnected":
        return base, base + body["height"]
    if tag == "StoreyTop":
        return base, own["top_elevation"] + body["offset"]
    return base, levels.get(body["storey"], own)["elevation"] + body["offset"]


def run_of(ramp, levels):
    """🛝️ The run of one ramp."""
    base, top = vertical(ramp, levels)
    rise = top - base
    rows = segments(ramp)
    starts, length = starts_of(rows)
    landed = flats(ramp, length, corners(rows, starts))
    pieces = sloped(landed, length)
    run_length = math.fsum(high - low for low, high in pieces)
    slope = abs(rise) / run_length if run_length > EXACT else 0.0

    def height(passed):
        return base + (rise * passed / run_length if run_length > EXACT else 0.0)

    flights, passed = [], 0.0
    for low, high in pieces:
        flights.append({"from": low, "to": high, "length": high - low, "z_from": height(passed), "z_to": height(passed + high - low)})
        passed += high - low

    def z_at(arc):
        if run_length <= EXACT:
            return base + (rise * min(max(arc / length, 0.0), 1.0) if length > EXACT else 0.0)
        return height(math.fsum(max(min(arc, high) - low, 0.0) for low, high in pieces))

    landings = [{"from": low, "to": high, "length": high - low, "z": z_at(low)} for low, high in landed]
    run_ok = abs(rise) <= EXACT or run_length > EXACT
    slope_ok = run_ok and slope <= ramp["max_slope"] + EXACT
    return {
        "base_z": base, "top_z": top, "rise": rise, "length": length, "run_length": run_length, "slope": slope, "angle": math.atan(slope), "width": ramp["width"],
        "flights": flights, "landings": landings, "compliance": {"run_ok": run_ok, "slope_ok": slope_ok, "compliant": slope_ok},
    }


def table(snapshot):
    """🛝️ The `🛝️ramp-runs` table: the run of every ramp whose storey exists."""
    levels = levels_oracle().storey_levels(snapshot)
    return {ramp_id: run_of(ramp, levels) for ramp_id, ramp in snapshot["ramps"].items() if ramp["storey"] in levels}


# endregion 🔖️Rules


# region 🔖️Audit
def sampled_line(ramp):
    """〰️ The path as a shapely LineString, every arc cut into 4096 chords."""
    points = []
    for start, end, bulge in segments(ramp):
        if abs(bulge) < 1e-12:
            chain = [start, end]
        else:
            angle = sweep(bulge)
            chord = math.dist(start, end)
            radius = chord / (2.0 * math.sin(angle / 2.0))
            heading = math.atan2(end[1] - start[1], end[0] - start[0])
            centre = (start[0] + radius * math.cos(heading + math.pi / 2.0 - angle / 2.0), start[1] + radius * math.sin(heading + math.pi / 2.0 - angle / 2.0))
            first = math.atan2(start[1] - centre[1], start[0] - centre[0])
            chain = [(centre[0] + abs(radius) * math.cos(first + angle * index / CHORDS), centre[1] + abs(radius) * math.sin(first + angle * index / CHORDS)) for index in range(CHORDS + 1)]
        points.extend(chain if not points else chain[1:])
    return LineString(points) if len(points) > 1 else None


def audit(ramp_id, ramp, run):
    """🩺️ GEOS measures of one run: path length, tiling pieces, strip area, continuous heights, the slope identity."""
    problems = []
    line = sampled_line(ramp)
    if line is None:
        return problems if run["length"] <= EXACT else ["%s: no GEOS path but the table says %.12g" % (ramp_id, run["length"])]
    tolerance = SAMPLED * max(run["length"], 1.0) if any(abs(row[2]) >= 1e-12 for row in segments(ramp)) else EXACT
    if abs(line.length - run["length"]) > tolerance:
        problems.append("%s: GEOS measures the path %.12g, the table says %.12g" % (ramp_id, line.length, run["length"]))
    cut = sorted([(row["from"], row["to"], "flight") for row in run["flights"]] + [(row["from"], row["to"], "landing") for row in run["landings"]])
    for index, (low, high, kind) in enumerate(cut):
        if index and abs(low - cut[index - 1][1]) > EXACT:
            problems.append("%s: %s at %.9g does not start where the previous piece ends (%.9g)" % (ramp_id, kind, low, cut[index - 1][1]))
        if abs(substring(line, low, high).length - (high - low)) > tolerance:
            problems.append("%s: GEOS cuts the %s [%.6g, %.6g] to %.12g" % (ramp_id, kind, low, high, substring(line, low, high).length))
    if cut and (abs(cut[0][0]) > EXACT or abs(cut[-1][1] - run["length"]) > EXACT):
        problems.append("%s: the pieces cover [%.9g, %.9g] of a path of %.9g" % (ramp_id, cut[0][0], cut[-1][1], run["length"]))
    if not cut and run["length"] > EXACT:
        problems.append("%s: a path of %.9g metres has no piece" % (ramp_id, run["length"]))
    strip = line.buffer(ramp["width"] / 2.0, cap_style="flat", join_style="mitre")
    if abs(strip.area - ramp["width"] * line.length) > tolerance * ramp["width"] * 4.0:
        problems.append("%s: the GEOS strip measures %.9g, width x length is %.9g" % (ramp_id, strip.area, ramp["width"] * line.length))
    for first, second in zip(run["flights"], run["flights"][1:]):
        if abs(second["z_from"] - first["z_to"]) > EXACT:
            problems.append("%s: the height jumps by %.3g between two flights" % (ramp_id, second["z_from"] - first["z_to"]))
    if run["flights"] and (abs(run["flights"][0]["z_from"] - run["base_z"]) > EXACT or abs(run["flights"][-1]["z_to"] - run["top_z"]) > EXACT):
        problems.append("%s: the flights do not climb from the foot to the head" % ramp_id)
    if run["run_length"] > EXACT and abs(run["slope"] * run["run_length"] - abs(run["rise"])) > EXACT:
        problems.append("%s: slope x run %.12g is not the rise %.12g" % (ramp_id, run["slope"] * run["run_length"], abs(run["rise"])))
    for flight in run["flights"]:
        if abs(abs(flight["z_to"] - flight["z_from"]) - run["slope"] * flight["length"]) > EXACT:
            problems.append("%s: a flight climbs by %.12g, the slope allows %.12g" % (ramp_id, abs(flight["z_to"] - flight["z_from"]), run["slope"] * flight["length"]))
    return problems


def parametric_problems(snapshot):
    """🧪️ Raising a storey by `delta` moves the rise of a `Storey`-topped ramp by `delta * ([target above] - [own above])` and no other."""
    problems = []
    before, delta = table(snapshot), 0.4
    levels = {storey_id: storey["level"] for storey_id, storey in snapshot["storeys"].items()}
    for storey_id in sorted(snapshot["storeys"]):
        raised = copy.deepcopy(snapshot)
        raised["storeys"][storey_id]["height"] += delta
        after = table(raised)
        for ramp_id, ramp in snapshot["ramps"].items():
            if ramp_id not in before:
                continue
            tag, body = variant(ramp["top"])
            expected = 0.0
            if tag == "Storey":
                target = levels.get(body["storey"], levels[ramp["storey"]])
                expected = delta * ((target > levels[storey_id]) - (levels[ramp["storey"]] > levels[storey_id]))
            elif tag == "StoreyTop":
                expected = delta * (levels[ramp["storey"]] == levels[storey_id])
            moved = after[ramp_id]["rise"] - before[ramp_id]["rise"]
            if abs(moved - expected) > EXACT:
                problems.append("%s: raising %s by %.1f moved the rise by %.12g, expected %.12g" % (ramp_id, storey_id, delta, moved, expected))
            if abs(after[ramp_id]["length"] - before[ramp_id]["length"]) > EXACT:
                problems.append("%s: raising %s changed the path length" % (ramp_id, storey_id))
    return problems


def problems_of(snapshot):
    """🩺️ Every audit problem of a snapshot."""
    problems = []
    for ramp_id, run in table(snapshot).items():
        problems += audit(ramp_id, snapshot["ramps"][ramp_id], run)
    return problems + parametric_problems(snapshot)


# endregion 🔖️Audit


# region 🔖️Handlers
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return levels_oracle().compare(expected, actual, path)


def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def ramp_runs_handler(ctx):
    """🛝️ Oracle answer for `🛝️ramp-runs`, after GEOS and the parametric law agreed with it."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = table(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("ramp-runs-ramps", ramp_runs_handler)


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
        computed = table(snapshot)
        target = case / "💡️inference" / "🛝️ramp-runs" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "ramp-runs")]
        print("%s: shapely %s, %d ramps" % (case.name, shapely.__version__, len(computed)))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
