#!/usr/bin/env python3
"""🌡️ Third-party ORACLE for the `s.bim.model@1` inference `🌡️energy-envelope`.

The subject (Rust, `semio-s-artifact-bim-model`) classifies every edge of a room against the wall it lies on and the room behind it, splits floors and ceilings by the rooms
above and below, takes U-values from the layer stacks (ISO 6946) and adds the conditioned spaces up to A, A/V, H_T and H'_T. This file reproduces the table from the SAME committed
snapshot with libraries that have never seen this repository: `shapely` 2 (GEOS) gives the rooms (the arrangement of the sibling oracle `../🏠️infer-bim-1-spaces`), `contains_xy` samples
every edge every centimetre to find the wall and the room behind it, `intersection` splits floors and ceilings by the rooms below and above, and `numpy` evaluates the ISO 6946
resistances (`R_T = R_si + sum(d / lambda) + R_se`) and the sums. The rules are restated here from the authored records alone: azimuth is the bearing of the outward normal clockwise
from true north (`atan2(nx, ny) + true_north - rotation`), tilt 0 up and 180 down, a surface behind a wall with no room is exterior (ground below the datum), a room behind a wall with
the same set points is adiabatic, a floor lies on the ground when it stands on the datum with no storey below, `F_x` is 1.0 / 0.6 / 0.5, `dU_WB` is 0.05.

Audits beyond the table: the closed form of the first case (a box with a window and a door), the net wall area plus the openings equals the gross area, every surface area is positive, the
areas of a case sum to the edge lengths times the clear heights plus twice the floor, and the metamorphic laws that thicker insulation never raises a U-value and that turning the building by
a quarter turn moves every sector by two.

The committed expectations under `🧫️fixtures/💡️inferences/🌡️energy-envelope/<case>/💡️inference/🌡️energy-envelope/🔣️.json` are WRITTEN by this file (`write`), never by hand, and
the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🌡️energy-envelope>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🌡️energy-envelope>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r9-audit-completeness.md — work package 20
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import numpy as np
import shapely
from shapely.geometry import LineString, Point, Polygon
from shapely.geometry.polygon import orient

# endregion 🔖️Imports

STEP = 0.01
PROBE = 0.002
BRIDGE = 0.05
GROUND = 0.6
NEIGHBOUR = 0.5
RSI = {"up": 0.10, "horizontal": 0.13, "down": 0.17}
RSE = 0.04


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


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


def xy(point):
    return (point["x"], point["y"])


# endregion 🔖️Vocabulary


# region 🔖️Thermal
def resistance(snapshot, layers):
    """🍰️ `sum(d / lambda)` of a layer stack, `None` when a layer has no thermal data."""
    total = 0.0
    if not layers:
        return None
    for layer in layers:
        material = snapshot["materials"].get(layer["material"])
        if material is None or not layer["thickness"] > 0 or not material["conductivity"] > 0:
            return None
        total += layer["thickness"] / material["conductivity"]
    return total


def transmittance(snapshot, layers, flow, far):
    """🧱️ ISO 6946: `U = 1 / (R_si + R + R_se)` with the far side outdoor (R_se), ground (none) or a room (R_si)."""
    inner = resistance(snapshot, layers)
    if inner is None:
        return None
    outer = {"exterior": RSE, "ground": 0.0}.get(far, RSI[flow])
    return 1.0 / (RSI[flow] + inner + outer)


def layers_of(snapshot, library, kind_id):
    kind = snapshot[library].get(kind_id)
    return None if kind is None else kind["layers"]


# endregion 🔖️Thermal


# region 🔖️Storeys
def elevation(snapshot, storey_id):
    """🪜️ Building-relative elevation of a storey: level 0 on the datum, upward and downward stacks of the storey heights."""
    row = snapshot["storeys"][storey_id]
    mates = {other: value for other, value in snapshot["storeys"].items() if value["building"] == row["building"]}
    if row["level"] >= 0:
        return math.fsum(value["height"] for value in mates.values() if 0 <= value["level"] < row["level"])
    return -math.fsum(value["height"] for value in mates.values() if row["level"] <= value["level"] < 0)


def neighbour(snapshot, storey_id, up):
    row = snapshot["storeys"][storey_id]
    levels = sorted((value["level"], other) for other, value in snapshot["storeys"].items() if value["building"] == row["building"] and ((value["level"] > row["level"]) if up else (value["level"] < row["level"])))
    if not levels:
        return None
    return levels[0][1] if up else levels[-1][1]


# endregion 🔖️Storeys


# region 🔖️Rooms
class Model:
    """🏠️ The rooms of a snapshot as shapely polygons plus the rows of the sibling spaces oracle (clear heights)."""

    def __init__(self, snapshot):
        self.snapshot = snapshot
        self.rows = spaces().tables(snapshot)
        self.shapes = {}
        self.faces = {}
        self.room = {}
        for storey in snapshot["storeys"]:
            shapes = spaces().footprints(snapshot, storey)
            self.shapes[storey] = shapes
        for space_id, space in snapshot["spaces"].items():
            storey = space["storey"]
            if storey not in self.faces:
                self.faces[storey] = spaces().free_faces(self.shapes[storey])
            faces, union = self.faces[storey]
            tag, body = variant(space["boundary"])
            status, face = spaces().room_for(xy(body["seed"]), faces, union)
            self.room[space_id] = orient(face, 1.0) if face is not None else None
        building = next(iter(snapshot["buildings"].values()))
        site = snapshot["sites"][building["site"]]
        self.bearing = site["true_north"] - building["rotation"]

    def thickness(self, wall_id):
        wall = self.snapshot["walls"][wall_id]
        return math.fsum(layer["thickness"] for layer in self.snapshot["wall_types"][wall["wall_type"]]["layers"])

    def conditions(self, space_id):
        return self.snapshot.get("space_conditions", {}).get(space_id)

    def climate_equal(self, one, other):
        a, b = self.conditions(one) or {}, self.conditions(other) or {}
        return a.get("heating_setpoint") == b.get("heating_setpoint") and a.get("cooling_setpoint") == b.get("cooling_setpoint")

    def conditioned(self, space_id):
        row = self.conditions(space_id) or {}
        return "heating_setpoint" in row or "cooling_setpoint" in row

    def azimuth(self, normal):
        return math.degrees(math.atan2(normal[0], normal[1]) + self.bearing) % 360.0


# endregion 🔖️Rooms


# region 🔖️Envelope
def sector(azimuth):
    return int(math.floor(((azimuth + 22.5) % 360.0) / 45.0)) % 8


def key_of(kind, boundary, adjacent, azimuth):
    return "%s|%s|%s|%s" % (kind, boundary, adjacent, "-" if azimuth is None else sector(azimuth))


def add(groups, key, area, u_value, solar=0.0):
    group = groups.setdefault(key, {"area": 0.0, "loss": 0.0, "solar": 0.0})
    group["area"] += area
    group["solar"] += solar
    group["loss"] = None if group["loss"] is None or u_value is None else group["loss"] + u_value * area


def wall_groups(model, space_id, groups):
    """🧱️ The wall, window and door surfaces of a room: every edge sampled every centimetre, the wall under it and the room behind it found with `contains_xy`."""
    snapshot = model.snapshot
    space = snapshot["spaces"][space_id]
    storey = space["storey"]
    face = model.room[space_id]
    height = model.rows[space_id]["clear_height"]
    floor_z = elevation(snapshot, storey)
    others = [(other, model.room[other]) for other, row in snapshot["spaces"].items() if row["storey"] == storey and model.room[other] is not None]
    open_length = 0.0
    rings = [face.exterior] + list(face.interiors)
    edges = {}
    for ring in rings:
        coordinates = list(ring.coords)
        for a, b in zip(coordinates[:-1], coordinates[1:]):
            length = math.dist(a, b)
            if length < 1e-6:
                continue
            normal = ((b[1] - a[1]) / length, -(b[0] - a[0]) / length)
            count = max(1, int(round(length / STEP)))
            for sample in range(count):
                t = (sample + 0.5) / count
                point = (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
                near = Point(point[0] + normal[0] * PROBE, point[1] + normal[1] * PROBE)
                wall = next((wall_id for wall_id, shape in sorted(model.shapes[storey].items()) if shape.contains(near)), None)
                if wall is None:
                    open_length += length / count
                    continue
                depth = model.thickness(wall) + PROBE
                far = Point(point[0] + normal[0] * depth, point[1] + normal[1] * depth)
                behind = next((other for other, shape in others if shape.contains(far)), None)
                if behind == space_id:
                    continue
                entry = edges.setdefault((wall, behind, normal), {"length": 0.0, "a": a, "b": b})
                entry["length"] += length / count
    grade = min(max(-floor_z, 0.0), height)
    for (wall, behind, normal), entry in sorted(edges.items(), key=lambda item: (item[0][0], item[0][1] or "", item[0][2])):
        azimuth = model.azimuth(normal)
        if behind is None:
            boundary, adjacent = "exterior", ""
        else:
            boundary, adjacent = ("adiabatic" if model.climate_equal(space_id, behind) else "adjacent"), behind
        far = "room" if behind is not None else "exterior"
        openings = [(opening_id, opening) for opening_id, opening in snapshot["openings"].items() if opening["host"] == wall and near_edge(model, opening, entry, wall)]
        cut = 0.0
        for opening_id, opening in openings:
            tag, body = variant(opening["kind"])
            if tag == "Window":
                kind = snapshot["window_types"][body["window_type"]]
                area = (opening.get("width") or kind["width"]) * (opening.get("height") or kind["height"])
                add(groups, key_of("window", boundary, adjacent, azimuth), area, kind.get("u_value"), (kind.get("g_value") or 0.0) * (1.0 - (kind.get("frame_fraction") or 0.0)) * area)
            elif tag == "Door":
                kind = snapshot["door_types"][body["door_type"]]
                area = (opening.get("width") or kind["width"]) * (opening.get("height") or kind["height"])
                add(groups, key_of("door", boundary, adjacent, azimuth), area, kind.get("u_value"))
            else:
                area = (opening.get("width") or 0.0) * (opening.get("height") or 0.0)
            cut += area
        layers = layers_of(snapshot, "wall_types", snapshot["walls"][wall]["wall_type"])
        if behind is None and grade > 1e-9:
            add(groups, key_of("wall", "ground", "", azimuth), entry["length"] * grade, transmittance(snapshot, layers, "horizontal", "ground"))
            if height - grade > 1e-9:
                add(groups, key_of("wall", "exterior", "", azimuth), entry["length"] * (height - grade) - cut, transmittance(snapshot, layers, "horizontal", "exterior"))
        else:
            add(groups, key_of("wall", boundary, adjacent, azimuth), max(entry["length"] * height - cut, 0.0), transmittance(snapshot, layers, "horizontal", far))
    return open_length


def near_edge(model, opening, entry, wall):
    """🪟️ Whether the centre of an opening lies within a wall thickness of the edge run (and between its ends)."""
    host = model.snapshot["walls"][wall]
    tag, body = variant(host["axis"])
    start, end = xy(body["start"]), xy(body["end"])
    length = math.dist(start, end)
    centre = (start[0] + (end[0] - start[0]) * opening["offset"] / length, start[1] + (end[1] - start[1]) * opening["offset"] / length)
    a, b = entry["a"], entry["b"]
    return LineString([a, b]).distance(Point(centre)) <= model.thickness(wall) + 0.01 and LineString([a, b]).project(Point(centre)) <= math.dist(a, b) + 1e-9


def floor_slab(snapshot, storey, point):
    best = None
    for slab_id, slab in snapshot["slabs"].items():
        if slab["storey"] == storey and Polygon([xy(v["point"]) for v in slab["boundary"]]).contains(Point(point)):
            layers = layers_of(snapshot, "slab_types", slab["slab_type"])
            if layers is not None:
                thickness = math.fsum(layer["thickness"] for layer in layers)
                if best is None or thickness > best[0]:
                    best = (thickness, layers)
    return None if best is None else best[1]


def roof_layers(snapshot, storey, point):
    for roof_id, roof in sorted(snapshot.get("roofs", {}).items()):
        if roof["storey"] == storey and Polygon([xy(v["point"]) for v in roof["footprint"]]).contains(Point(point)):
            return layers_of(snapshot, "roof_types", roof["roof_type"])
    return None


def horizontal_groups(model, space_id, groups, up):
    """⬆️ The floor (or ceiling) of a room split by the rooms below (above) it with `intersection`."""
    snapshot = model.snapshot
    space = snapshot["spaces"][space_id]
    storey = space["storey"]
    face = model.room[space_id]
    other_storey = neighbour(snapshot, storey, up)
    parts = []
    if other_storey is not None:
        for other, row in snapshot["spaces"].items():
            if row["storey"] == other_storey and model.room[other] is not None:
                piece = face.intersection(model.room[other])
                if piece.area > 1e-4:
                    parts.append((other, piece.area, piece.representative_point().coords[0]))
    rest = face.area - math.fsum(part[1] for part in parts)
    if rest > 1e-4 or not parts:
        parts.append((None, max(rest, 0.0), tuple(face.representative_point().coords[0])))
    on_datum = elevation(snapshot, storey) <= 1e-9
    for other, area, point in parts:
        if other is None:
            boundary, adjacent = ("ground" if (not up and other_storey is None and on_datum) else "exterior"), ""
        else:
            boundary, adjacent = ("adiabatic" if model.climate_equal(space_id, other) else "adjacent"), other
        far = "room" if other is not None else boundary
        if up:
            layers = floor_slab(snapshot, other_storey, point) if other_storey is not None else None
            layers = layers or roof_layers(snapshot, storey, point)
        else:
            layers = floor_slab(snapshot, storey, point)
        add(groups, key_of("ceiling" if up else "floor", boundary, adjacent, None), area, transmittance(snapshot, layers, "up" if up else "down", far))


def table_of(model):
    """🌡️ The oracle table: the surfaces of every space merged by (kind, boundary, neighbour, sector) and the totals of every scope."""
    snapshot = model.snapshot
    result = {"spaces": {}, "totals": {}}
    internal = {}
    if not snapshot.get("space_conditions"):
        return result
    for space_id in sorted(snapshot["spaces"]):
        face = model.room[space_id]
        if face is None:
            continue
        groups = {}
        open_length = wall_groups(model, space_id, groups)
        horizontal_groups(model, space_id, groups, False)
        horizontal_groups(model, space_id, groups, True)
        row = model.rows[space_id]
        conditions = model.conditions(space_id) or {}
        internal[space_id] = groups
        result["spaces"][space_id] = {
            "conditioned": model.conditioned(space_id),
            "heated": "heating_setpoint" in conditions,
            "floor_area": face.area,
            "volume": face.area * row["clear_height"],
            "height": row["clear_height"],
            "open_length": open_length,
            "groups": {key: {k: v for k, v in value.items() if v is not None and k != "solar"} for key, value in sorted(groups.items())},
        }
    scopes = [("zone:" + zone, lambda s, zone=zone: snapshot["spaces"][s].get("zone") == zone) for zone in sorted(snapshot.get("zones", {}))]
    scopes += [("building:" + building, lambda s, building=building: snapshot["storeys"][snapshot["spaces"][s]["storey"]]["building"] == building) for building in sorted(snapshot["buildings"])]
    scopes.append(("project", lambda s: True))
    for name, member in scopes:
        result["totals"][name] = totals_of(model, result["spaces"], internal, member)
    return result


def totals_of(model, rows, internal, member):
    """📊️ H_T, H'_T, A, A/V and the areas by sector of the conditioned spaces of a scope, restated from the groups."""
    spaces_in = [s for s in sorted(rows) if member(s) and rows[s]["conditioned"]]
    area = loss = floor_area = volume = glazing = roof = floor_env = 0.0
    missing = 0
    opaque = np.zeros(8)
    windows = np.zeros(8)
    solar_by = np.zeros(8)
    u_opaque = a_opaque = u_window = a_window = 0.0
    for s in spaces_in:
        floor_area += rows[s]["floor_area"]
        volume += rows[s]["volume"]
        for key, group in internal[s].items():
            kind, boundary, adjacent, where = key.split("|")
            if boundary == "adiabatic":
                continue
            if boundary == "adjacent" and model.conditioned(adjacent) and member(adjacent):
                continue
            factor = {"exterior": 1.0, "ground": GROUND}.get(boundary, NEIGHBOUR)
            area += group["area"]
            if group["loss"] is not None:
                loss += factor * group["loss"]
            else:
                missing += 1
            if kind == "window":
                windows[int(where)] += group["area"]
                solar_by[int(where)] += group["solar"]
                glazing += group["area"]
                if group["loss"] is not None:
                    u_window += group["loss"]
                    a_window += group["area"]
            elif kind in ("wall", "door", "curtain-wall"):
                opaque[int(where)] += group["area"]
                if group["loss"] is not None:
                    u_opaque += group["loss"]
                    a_opaque += group["area"]
            elif kind == "ceiling":
                roof += group["area"]
                if group["loss"] is not None:
                    u_opaque += group["loss"]
                    a_opaque += group["area"]
            else:
                floor_env += group["area"]
                if group["loss"] is not None:
                    u_opaque += group["loss"]
                    a_opaque += group["area"]
    facade = float(opaque.sum()) + glazing
    return {
        "spaces": spaces_in,
        "floor_area": floor_area,
        "volume": volume,
        "envelope_area": area,
        "a_over_v": area / volume if volume > 0 else 0.0,
        "transmission": loss,
        "h_t_prime": (loss + BRIDGE * area) / area if area > 0 else 0.0,
        "opaque_by_sector": opaque.tolist(),
        "window_by_sector": windows.tolist(),
        "solar_by_sector": solar_by.tolist(),
        "roof_area": roof,
        "floor_envelope_area": floor_env,
        "glazing_area": glazing,
        "glazing_ratio": glazing / facade if facade > 0 else 0.0,
        "solar_aperture": float(solar_by.sum()),
        "mean_u_opaque": u_opaque / a_opaque if a_opaque > 0 else 0.0,
        "mean_u_window": u_window / a_window if a_window > 0 else 0.0,
        "missing": missing,
    }


def tables(snapshot):
    """🌡️ The `🌡️energy-envelope` table of a snapshot."""
    return table_of(Model(snapshot))


# endregion 🔖️Envelope


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Closed forms and metamorphic laws that must hold on every case."""
    problems = []
    model = Model(snapshot)
    table = table_of(model)
    for space_id, row in table["spaces"].items():
        face = model.room[space_id]
        height = row["height"]
        walls = math.fsum(group["area"] for key, group in row["groups"].items() if key.split("|")[0] in ("wall", "window", "door"))
        rings = [face.exterior] + list(face.interiors)
        perimeter = math.fsum(ring.length for ring in rings) - row["open_length"]
        if abs(walls - perimeter * height) > 1e-6 * max(walls, 1.0):
            problems.append("%s: wall, window and door areas %.6f differ from perimeter times height %.6f" % (space_id, walls, perimeter * height))
        horizontal = math.fsum(group["area"] for key, group in row["groups"].items() if key.split("|")[0] in ("floor", "ceiling"))
        if abs(horizontal - 2.0 * row["floor_area"]) > 1e-6:
            problems.append("%s: floor and ceiling areas %.6f differ from twice the floor %.6f" % (space_id, horizontal, 2.0 * row["floor_area"]))
        if any(group["area"] <= 0 for group in row["groups"].values()):
            problems.append("%s: a surface group has no area" % space_id)
    thicker = copy.deepcopy(snapshot)
    for kind in thicker["wall_types"].values():
        for layer in kind["layers"]:
            if thicker["materials"][layer["material"]]["category"] == "Insulation":
                layer["thickness"] *= 2.0
    if thicker != snapshot and table["totals"]:
        before, after = table["totals"]["project"], tables(thicker)["totals"]["project"]
        if after["spaces"] and before["envelope_area"] > 0 and after["transmission"] > before["transmission"] + 1e-9:
            problems.append("thicker insulation raised the heat loss (%s -> %s)" % (before["transmission"], after["transmission"]))
    turned = copy.deepcopy(snapshot)
    for building in turned["buildings"].values():
        building["rotation"] += math.pi / 2.0
    if table["totals"]:
        before, after = table["totals"]["project"], tables(turned)["totals"]["project"]
        if not np.allclose(np.roll(before["window_by_sector"], -2), after["window_by_sector"], atol=1e-9) and not np.allclose(np.roll(before["window_by_sector"], 2), after["window_by_sector"], atol=1e-9):
            problems.append("turning the building by a quarter turn did not move the sectors by two")
    return problems


# endregion 🔖️Audit


# region 🔖️Projection
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (relative tolerance 1e-9)."""
    if isinstance(expected, dict) and isinstance(actual, dict):
        found = []
        for key in sorted(set(expected) | set(actual)):
            if key not in expected or key not in actual:
                found.append("%s/%s: present on one side only" % (path, key))
            else:
                found += compare(expected[key], actual[key], "%s/%s" % (path, key))
        return found
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return ["%s: length %d against %d" % (path, len(expected), len(actual))]
        return [problem for index, (left, right) in enumerate(zip(expected, actual)) for problem in compare(left, right, "%s[%d]" % (path, index))]
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)) and not isinstance(expected, bool):
        return [] if abs(expected - actual) <= 1e-9 * max(abs(expected), abs(actual), 1.0) else ["%s: %r against %r" % (path, expected, actual)]
    return [] if expected == actual else ["%s: %r against %r" % (path, expected, actual)]


# endregion 🔖️Projection


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def energy_handler(ctx):
    """🌡️ Oracle answer for `🌡️energy-envelope`, after the closed forms and the metamorphic laws agreed with it."""
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

    return Adapter("python").oracle("energy-box", energy_handler).oracle("energy-zoning", energy_handler).oracle("energy-stack", energy_handler)


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
        target = case / "💡️inference" / "🌡️energy-envelope" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "energy")]
        print("%s: shapely %s, numpy %s, %d spaces" % (case.name, shapely.__version__, np.__version__, len(snapshot["spaces"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
# endregion 🔖️Standalone
