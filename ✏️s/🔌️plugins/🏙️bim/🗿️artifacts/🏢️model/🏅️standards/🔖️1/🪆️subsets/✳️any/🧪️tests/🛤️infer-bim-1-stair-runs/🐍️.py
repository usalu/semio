#!/usr/bin/env python3
"""🪜️ Third-party ORACLE for the `s.bim.model@1` inference `🪜️stair-runs`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per stair, the rise from the storey levels and the top constraint, the equal
riser height from the riser limit, the tread from Blondel's rule `2R + T` and the minimum tread, the flights and landings of the four
flight kinds and the code flags. No third-party engine owns stair arithmetic, so this file is a second, independently written
implementation of the same rules on exactly rounded sums (`math.fsum` levels from the sibling levels oracle) and then AUDITS the
geometry with `shapely` 2 (GEOS), a library that has never seen this repository: every flight is a flat-capped strip around its walking
line and every landing a rotated box; the walking lines must have the table's `run_length`, the strips must be disjoint, a landing must
touch the strips it joins, and a winder strip must cover the annulus sector it claims. The parametric law is a metamorphic property:
raising the height of a storey by `delta` raises the rise of exactly the `StoreyTop` stairs on it, leaves the `Unconnected` ones alone
and never lowers a riser count.

The committed expectations under `🧫️fixtures/💡️inferences/🪜️stair-runs/<case>/💡️inference/🪜️stair-runs/🔣️.json` are WRITTEN by this
file (`write`), never by hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🪜️stair-runs>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🪜️stair-runs>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r4-api-stair-runs.md — the placement contract
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

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
BLONDEL_MIN, BLONDEL_MAX, BLONDEL_TARGET = 0.59, 0.65, 0.62
MAX_RISERS = 512


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


def mark(x, y):
    return {"x": x, "y": y}


# endregion 🔖️Vocabulary


# region 🔖️Rules
def riser_count(rise, max_riser):
    """🔢️ `(count, within the cap)`: the fewest equal risers within the limit."""
    if not (rise > EXACT and math.isfinite(max_riser) and max_riser > EXACT):
        return 0, False
    wanted = max(math.ceil(rise / max_riser - EXACT), 1)
    return (MAX_RISERS, False) if wanted > MAX_RISERS else (wanted, True)


def heading(angle):
    """🧭️ The unit vector of an angle."""
    return (math.cos(angle), math.sin(angle))


def along(origin, angle, distance):
    return (origin[0] + distance * math.cos(angle), origin[1] + distance * math.sin(angle))


def flight(first, risers, start, angle, tread, riser, base):
    treads = max(risers - 1, 0)
    return {"first_riser": first, "risers": risers, "treads": treads, "start": mark(*start), "direction": angle, "tread": tread, "base_z": base + (first - 1) * riser, "length": treads * tread}


def layout(stair, count, riser, tread, base):
    """🪜️ `(flights, landings)` of a stair by flight kind."""
    start, angle, width = (stair["start"]["x"], stair["start"]["y"]), stair["direction"], stair["width"]
    tag, body = variant(stair["flight"])
    single = ([flight(1, count, start, angle, tread, riser, base)], []) if count else ([], [])
    if count == 0:
        return [], []
    if tag == "Straight" or (tag in ("LTurn", "UTurn") and count < 2):
        return single
    if tag == "LTurn":
        split = body["split"] if math.isfinite(body["split"]) else 0.5
        first = int(min(max(math.floor(split * count + 0.5), 1), count - 1))
        sign = 1.0 if body["turn"] == "Left" else -1.0
        foot = along(start, angle, (first - 1) * tread)
        centre = along(foot, angle, width / 2.0)
        turned = angle + sign * math.pi / 2.0
        landing = {"after_flight": 0, "z": base + first * riser, "centre": mark(*centre), "direction": angle, "width": width, "depth": width}
        return [flight(1, first, start, angle, tread, riser, base), flight(first + 1, count - first, along(centre, turned, width / 2.0), turned, tread, riser, base)], [landing]
    if tag == "UTurn":
        gap = max(body["gap"], 0.0) if math.isfinite(body["gap"]) else 0.0
        first = -(-count // 2)
        foot = along(start, angle, (first - 1) * tread)
        second = along(foot, angle + math.pi / 2.0, width + gap)
        centre = along(along(foot, angle, width / 2.0), angle + math.pi / 2.0, (width + gap) / 2.0)
        landing = {"after_flight": 0, "z": base + first * riser, "centre": mark(*centre), "direction": angle, "width": 2.0 * width + gap, "depth": width}
        return [flight(1, first, start, angle, tread, riser, base), flight(first + 1, count - first, second, angle + math.pi, tread, riser, base)], [landing]
    if tag == "Spiral":
        outer = max(body["radius"], width)
        walking = outer - width / 2.0
        sign = -1.0 if body["sweep"] < 0 else 1.0
        centre = along(start, angle + math.pi / 2.0, sign * walking)
        treads = count - 1
        arc = abs(body["sweep"]) * walking / treads if treads else 0.0
        run = flight(1, count, start, angle, arc, riser, base)
        run["winder"] = {"centre": mark(*centre), "inner_radius": max(outer - width, 0.0), "outer_radius": outer, "start_angle": angle - sign * math.pi / 2.0, "sweep": body["sweep"]}
        return [run], []
    raise AssertionError("unknown flight %r" % tag)


def run_of(snapshot, levels, stair):
    """🪜️ The run of one stair."""
    storey = stair["storey"]
    base = levels[storey]["elevation"]
    tag, body = variant(stair["top"])
    if tag == "Unconnected":
        top = base + body["height"]
    elif tag == "StoreyTop":
        top = levels[storey]["top_elevation"] + body["offset"]
    else:
        top = levels.get(body["storey"], levels[storey])["elevation"] + body["offset"]
    rise = top - base
    count, within = riser_count(rise, stair["max_riser"])
    riser = rise / count if count else 0.0
    designed = max(BLONDEL_TARGET - 2.0 * riser, stair["min_tread"], 0.0)
    flights, landings = layout(stair, count, riser, designed, base)
    tread = flights[0]["tread"] if variant(stair["flight"])[0] == "Spiral" and flights else designed
    stride = 2.0 * riser + tread
    positive = rise > EXACT
    riser_ok = positive and within and riser <= stair["max_riser"] + EXACT
    tread_ok = count <= 1 or tread >= stair["min_tread"] - EXACT
    blondel_ok = count <= 1 or BLONDEL_MIN - EXACT <= stride <= BLONDEL_MAX + EXACT
    return {
        "base_z": base, "top_z": top, "rise": rise, "riser_count": count, "riser_height": riser, "tread_count": max(count - 1, 0), "tread": tread, "stride": stride, "width": stair["width"],
        "run_length": math.fsum(run["length"] for run in flights), "flights": flights, "landings": landings,
        "compliance": {"rise_positive": positive, "riser_ok": riser_ok, "tread_ok": tread_ok, "blondel_ok": blondel_ok, "compliant": positive and riser_ok and tread_ok and blondel_ok},
    }


def table(snapshot):
    """🪜️ The `🪜️stair-runs` table: the run of every stair whose storey exists."""
    levels = levels_oracle().storey_levels(snapshot)
    return {stair_id: run_of(snapshot, levels, stair) for stair_id, stair in snapshot["stairs"].items() if stair["storey"] in levels}


# endregion 🔖️Rules


# region 🔖️Audit
def strip(run, row):
    """▭️ A flat-capped strip around the straight walking line of a flight."""
    start = (row["start"]["x"], row["start"]["y"])
    end = along(start, row["direction"], row["length"])
    return LineString([start, end]).buffer(run["width"] / 2.0, cap_style="flat") if row["length"] > 0 else Point(start).buffer(1e-9)


def landing_box(row):
    """▭️ The landing rectangle: `depth` along its direction, `width` across, centred."""
    shape = box(-row["depth"] / 2.0, -row["width"] / 2.0, row["depth"] / 2.0, row["width"] / 2.0)
    return affinity.translate(affinity.rotate(shape, row["direction"], origin=(0.0, 0.0), use_radians=True), row["centre"]["x"], row["centre"]["y"])


def winder_sector(row):
    """◔️ The annulus sector a winder flight claims, built by shapely from the sampled arcs."""
    winder = row["winder"]
    steps = 2048
    angles = [winder["start_angle"] + winder["sweep"] * index / steps for index in range(steps + 1)]
    outer = [(winder["centre"]["x"] + winder["outer_radius"] * math.cos(a), winder["centre"]["y"] + winder["outer_radius"] * math.sin(a)) for a in angles]
    inner = [(winder["centre"]["x"] + winder["inner_radius"] * math.cos(a), winder["centre"]["y"] + winder["inner_radius"] * math.sin(a)) for a in reversed(angles)]
    return Polygon(outer + inner)


def audit(stair_id, stair, run):
    """🩺️ GEOS measures of one run: walking-line length, disjoint strips, touching landings, winder area, riser sum."""
    problems = []
    if run["riser_count"] and abs(run["riser_height"] * run["riser_count"] - run["rise"]) > EXACT:
        problems.append("%s: %d risers of %.12g do not add up to the rise %.12g" % (stair_id, run["riser_count"], run["riser_height"], run["rise"]))
    if sum(row["risers"] for row in run["flights"]) != run["riser_count"]:
        problems.append("%s: the flights carry %d risers, the stair has %d" % (stair_id, sum(row["risers"] for row in run["flights"]), run["riser_count"]))
    lines = [LineString([(row["start"]["x"], row["start"]["y"]), along((row["start"]["x"], row["start"]["y"]), row["direction"], row["length"])]) for row in run["flights"] if "winder" not in row and row["length"] > 0]
    if "winder" not in (run["flights"][0] if run["flights"] else {}) and abs(sum(line.length for line in lines) - run["run_length"]) > EXACT:
        problems.append("%s: GEOS walking lines measure %.12g, the table says %.12g" % (stair_id, sum(line.length for line in lines), run["run_length"]))
    strips = [strip(run, row) for row in run["flights"] if "winder" not in row]
    landings = [landing_box(row) for row in run["landings"]]
    for left in range(len(strips)):
        for right in range(left + 1, len(strips)):
            if strips[left].intersection(strips[right]).area > EXACT:
                problems.append("%s: flights %d and %d overlap by %.3g" % (stair_id, left, right, strips[left].intersection(strips[right]).area))
    for index, shape in enumerate(landings):
        row = run["landings"][index]
        if abs(shape.area - row["width"] * row["depth"]) > EXACT:
            problems.append("%s: landing %d box measures %.12g, expected %.12g" % (stair_id, index, shape.area, row["width"] * row["depth"]))
        if not shape.buffer(1e-6).intersects(strips[row["after_flight"]]) or not shape.buffer(1e-6).intersects(strips[row["after_flight"] + 1]):
            problems.append("%s: landing %d does not join its flights" % (stair_id, index))
        for flight_index, flight_strip in enumerate(strips):
            if shape.intersection(flight_strip).area > EXACT * 10:
                problems.append("%s: landing %d overlaps flight %d by %.3g" % (stair_id, index, flight_index, shape.intersection(flight_strip).area))
    for row in run["flights"]:
        if "winder" in row:
            sector = winder_sector(row)
            winder = row["winder"]
            expected = 0.5 * abs(winder["sweep"]) * (winder["outer_radius"] ** 2 - winder["inner_radius"] ** 2)
            if abs(sector.area - expected) > 1e-5 * max(expected, 1.0):
                problems.append("%s: winder sector measures %.9g, closed form %.9g" % (stair_id, sector.area, expected))
            if abs(row["length"] - abs(winder["sweep"]) * (winder["outer_radius"] + winder["inner_radius"]) / 2.0 * row["treads"] / max(row["treads"], 1)) > 1e-9 * max(row["length"], 1.0):
                problems.append("%s: winder length %.12g is not the arc on the walking line" % (stair_id, row["length"]))
    return problems


def parametric_problems(snapshot):
    """🧪️ Raising a storey by `delta` raises exactly the rise of the `StoreyTop` stairs on it, keeps `Unconnected` ones, never lowers a count."""
    problems = []
    before = table(snapshot)
    for storey_id in sorted({stair["storey"] for stair in snapshot["stairs"].values()}):
        raised = copy.deepcopy(snapshot)
        raised["storeys"][storey_id]["height"] += 0.4
        after = table(raised)
        for stair_id, stair in snapshot["stairs"].items():
            if stair["storey"] != storey_id:
                continue
            tag, _ = variant(stair["top"])
            moved = after[stair_id]["rise"] - before[stair_id]["rise"]
            if tag == "StoreyTop" and abs(moved - 0.4) > EXACT:
                problems.append("%s: raising %s by 0.4 moved the rise by %.12g" % (stair_id, storey_id, moved))
            if tag == "Unconnected" and abs(moved) > EXACT:
                problems.append("%s: an unconnected stair followed its storey by %.12g" % (stair_id, moved))
            if after[stair_id]["riser_count"] < before[stair_id]["riser_count"]:
                problems.append("%s: raising the storey lowered the riser count" % stair_id)
    return problems


def problems_of(snapshot):
    """🩺️ Every audit problem of a snapshot."""
    problems = []
    for stair_id, run in table(snapshot).items():
        problems += audit(stair_id, snapshot["stairs"][stair_id], run)
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


def stair_runs_handler(ctx):
    """🪜️ Oracle answer for `🪜️stair-runs`, after GEOS and the parametric law agreed with it."""
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

    return Adapter("python").oracle("stair-runs-flights", stair_runs_handler)


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
        target = case / "💡️inference" / "🪜️stair-runs" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "stair-runs")]
        print("%s: shapely %s, %d stairs" % (case.name, shapely.__version__, len(snapshot["stairs"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
