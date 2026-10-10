#!/usr/bin/env python3
"""🔋️ Third-party ORACLE (jsonschema + shapely + numpy) for the energy model BIM writes.

The subject (Rust, `semio-s-artifact-bim-model`) turns the thermal envelope it inferred (`energy-envelope`) into the snapshot of `s.energy.model@1`. This file never sees that writer. It opens the
committed export `🧫️fixtures/🚪️energy/<case>/🔋️model.json` and

* validates it with `jsonschema` (draft 7) against the energy artifact's own snapshot facet and against the strict schema of the export (`🚪️io/📤️export/🔋️energy/🔣️.json`, the field layout of the
  energy engine's `Model`);
* audits the model as the engine's `Model::validate` does (unique ids, references, partner surfaces that point back, positive thicknesses and conductivities, windows in the plane and inside their host);
* recomputes from the polygons alone (numpy: Newell normals and areas; shapely: footprints) the floor area and, by the divergence theorem, the volume of every zone, the net area of every surface by
  kind, boundary, neighbour and compass sector (azimuth = bearing of the outward normal + the north axis), and from the exported layers the ISO 6946 transmittance
  (`R_T = R_si + sum(d / lambda) + R_se`, surface resistances by the direction of the heat flow); it adds the conditioned zones of every scope up to A, A/V, H_T, H'_T and the sector areas;
* compares the table with the one the inference oracle `../🌡️infer-bim-1-energy` committed for the same snapshot (the openings of a partition are merged into it, the export has no
  window or door behind an interior boundary).

    python 🐍️.py check <path to 🧫️fixtures/🚪️energy>      # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🚪️energy>      # rewrite the measured table from the committed exports

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🔋️energy/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import importlib.util
import json
import math
import re
import sys
from pathlib import Path

import jsonschema
import numpy as np
from shapely.geometry import Polygon

# endregion 🔖️Imports


# region 🔖️Constants
HERE = Path(__file__).resolve().parent
SUBSET = HERE.parents[1]
ENERGY_FACET = SUBSET.parents[6] / "🔋️energy" / "🗿️artifacts" / "🔋️model" / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🧬️schema" / "📸️snapshot" / "🔣️.json"
EXPORT_SCHEMA = SUBSET / "🚪️io" / "📤️export" / "🔋️energy" / "🔣️.json"
HOUSE = SUBSET / "🖼️assets" / "🏡️house" / "📸️snapshot.json"
RSI = {"up": 0.10, "horizontal": 0.13, "down": 0.17}
RSE = 0.04
GROUND = 0.6
NEIGHBOUR = 0.5
BRIDGE = 0.05
NO_HEATING = -100.0
TOLERANCE = 1e-9
PLANE = 1e-3
KINDS = ("wall", "curtain-wall", "window", "door", "floor", "ceiling")

# endregion 🔖️Constants


# region 🔖️Geometry
def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, SUBSET / "🧪️tests" / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def newell(vertices):
    """📐️ The area vector of a polygon (half the sum of the cross products of its edges)."""
    points = np.asarray(vertices, dtype=float)
    return np.cross(points, np.roll(points, -1, axis=0)).sum(axis=0) / 2.0


def sector(azimuth):
    return int(math.floor(((azimuth + 22.5) % 360.0) / 45.0)) % 8


def tetra_volume(polygons):
    """🧊️ The volume a closed set of outward-facing polygons encloses: the sum of the signed tetrahedra of a fan of every polygon."""
    total = 0.0
    for vertices in polygons:
        points = np.asarray(vertices, dtype=float)
        for index in range(1, len(points) - 1):
            total += float(np.dot(points[0], np.cross(points[index], points[index + 1]))) / 6.0
    return total


def transmittance(layers, flow, far):
    """🧱️ ISO 6946: `1 / (R_si + sum(d / lambda) + R_se)`, the far side being the outdoor air, the ground (no resistance) or another room."""
    resistance = sum(thickness / conductivity for thickness, conductivity in layers)
    outside = {"outdoor": RSE, "ground": 0.0, "room": RSI[flow]}[far]
    return 1.0 / (RSI[flow] + resistance + outside)


# endregion 🔖️Geometry


# region 🔖️Reading
def space_of(zone):
    """🏷️ The id of the space a zone was written for: the first word of its name."""
    return zone["name"].split(" ", 1)[0]


class Export:
    """📖️ The committed export and what can be read of it by id."""

    def __init__(self, document):
        self.document = document
        self.model = document["model"]
        self.zones = {zone["id"]: zone for zone in self.model["zones"]}
        self.spaces = {space["zone_id"]: space for space in self.model["spaces"]}
        self.surfaces = {surface["id"]: surface for surface in self.model["surfaces"]}
        self.materials = {material["id"]: material for material in self.model["materials"]}
        self.constructions = {construction["id"]: construction for construction in self.model["constructions"]}
        self.constants = {schedule["id"]: schedule["value"] for schedule in self.model["schedules"]["constants"]}
        self.windows = {}
        for fenestration in self.model["fenestrations"]:
            self.windows.setdefault(fenestration["surface_id"], []).append(fenestration)
        self.partner = {}
        for pair in self.model["adjacency_pairs"]:
            self.partner[pair["surface_a_id"]] = pair["surface_b_id"]
            self.partner[pair["surface_b_id"]] = pair["surface_a_id"]
        self.thermostat = {thermostat["zone_id"]: thermostat for thermostat in self.model["thermostats"]}
        self.north = self.model["site"]["north_axis_deg"]

    def space_id(self, zone_id):
        return space_of(self.zones[zone_id])

    def conditioned(self, zone_id):
        return zone_id in self.thermostat

    def heated(self, zone_id):
        return zone_id in self.thermostat and self.constants[self.thermostat[zone_id]["heating_setpoint_schedule_id"]] > NO_HEATING

    def layers(self, construction_id):
        return [(self.materials[m]["thickness_m"], self.materials[m]["conductivity_w_m_k"]) for m in self.constructions[construction_id]["layer_material_ids"]]


def validate(document):
    """🧪️ Schema problems of the export against the facet of the energy artifact and against the strict schema of the export."""
    problems = []
    for name, path in (("energy facet", ENERGY_FACET), ("export schema", EXPORT_SCHEMA)):
        schema = json.loads(path.read_text(encoding="utf-8"))
        jsonschema.Draft7Validator.check_schema(schema)
        for error in jsonschema.Draft7Validator(schema).iter_errors(document):
            problems.append("%s: %s: %s" % (name, "/".join(str(part) for part in error.path), error.message[:160]))
    return problems


def integrity(export):
    """🩺️ The checks of `Model::validate` and the reciprocity of the partner surfaces."""
    model, problems = export.model, []
    for family in ("zones", "surfaces", "fenestrations", "materials", "constructions", "people", "lighting", "equipment", "thermostats", "ideal_loads", "thermal_enclosures"):
        ids = [row["id"] for row in model[family]]
        if len(set(ids)) != len(ids):
            problems.append("%s: ids are not unique" % family)
    names = [zone["name"] for zone in model["zones"]]
    if len(set(names)) != len(names):
        problems.append("zones: names are not unique")
    schedule_ids = {row["id"] for kind in ("constants", "daily", "weekly") for row in model["schedules"][kind]}
    for surface in model["surfaces"]:
        if surface["zone_id"] not in export.zones:
            problems.append("%s: unknown zone" % surface["name"])
        if surface["construction_id"] not in export.constructions:
            problems.append("%s: unknown construction" % surface["name"])
        boundary = surface["outside_boundary_condition"]
        if isinstance(boundary, dict):
            other = export.surfaces.get(boundary["Interzone"])
            if other is None or other["outside_boundary_condition"] != {"Interzone": surface["id"]}:
                problems.append("%s: its Interzone partner does not point back" % surface["name"])
            elif other["zone_id"] == surface["zone_id"]:
                problems.append("%s: its partner is in the same zone" % surface["name"])
    for construction in model["constructions"]:
        for material in construction["layer_material_ids"]:
            if material not in export.materials:
                problems.append("%s: unknown material" % construction["name"])
    for fenestration in model["fenestrations"]:
        host = export.surfaces.get(fenestration["surface_id"])
        if host is None:
            problems.append("%s: unknown host" % fenestration["name"])
            continue
        if host["outside_boundary_condition"] != "OutdoorAir":
            problems.append("%s: its host is not an exterior surface" % fenestration["name"])
        normal = newell(host["vertices_m"])
        unit = normal / np.linalg.norm(normal)
        origin = np.asarray(host["vertices_m"][0], dtype=float)
        if any(abs(float(np.dot(np.asarray(vertex) - origin, unit))) > PLANE for vertex in fenestration["vertices_m"]):
            problems.append("%s: not in the plane of its host" % fenestration["name"])
    for kind in ("people", "lighting", "equipment"):
        for row in model[kind]:
            if row["schedule_id"] not in schedule_ids:
                problems.append("%s %s: unknown schedule" % (kind, row["id"]))
    for thermostat in model["thermostats"]:
        for key in ("heating_setpoint_schedule_id", "cooling_setpoint_schedule_id"):
            if thermostat[key] not in schedule_ids:
                problems.append("thermostat %s: unknown schedule" % thermostat["id"])
    for material in model["materials"]:
        if material["thickness_m"] <= 0 or material["conductivity_w_m_k"] <= 0:
            problems.append("%s: invalid thermal properties" % material["name"])
    return problems


# endregion 🔖️Reading


# region 🔖️Table
def add(groups, key, area, u_value, solar=0.0):
    group = groups.setdefault(key, {"area": 0.0, "loss": 0.0, "solar": 0.0})
    group["area"] += area
    group["solar"] += solar
    group["loss"] += u_value * area


def surface_rows(export):
    """🧱️ Per zone the groups of its surfaces: kind, boundary, neighbour and sector, with area, loss `sum(U * A)` and solar aperture."""
    per_zone = {zone_id: {} for zone_id in export.zones}
    for surface in export.model["surfaces"]:
        vector = newell(surface["vertices_m"])
        gross = float(np.linalg.norm(vector))
        normal = vector / gross
        boundary = surface["outside_boundary_condition"]
        if boundary == "OutdoorAir":
            name, adjacent, far = "exterior", "", "outdoor"
        elif boundary == "Ground":
            name, adjacent, far = "ground", "", "ground"
        elif boundary == "Adiabatic":
            name, far = "adiabatic", "room"
            adjacent = export.space_id(export.surfaces[export.partner[surface["id"]]]["zone_id"]) if surface["id"] in export.partner else ""
        else:
            name, far = "adjacent", "room"
            adjacent = export.space_id(export.surfaces[boundary["Interzone"]]["zone_id"])
        if normal[2] > 0.5:
            kind, flow, where = "ceiling", "up", "-"
        elif normal[2] < -0.5:
            kind, flow, where = "floor", "down", "-"
        else:
            kind, flow = "wall", "horizontal"
            where = str(sector((math.degrees(math.atan2(normal[0], normal[1])) + export.north) % 360.0))
        u_value = transmittance(export.layers(surface["construction_id"]), flow, far)
        cut = 0.0
        for fenestration in export.windows.get(surface["id"], []):
            area = float(np.linalg.norm(newell(fenestration["vertices_m"])))
            cut += area
            label = fenestration["name"].split(" ", 1)[0]
            label = label if label in ("door", "curtain-wall") else "window"
            add(per_zone[surface["zone_id"]], "|".join((label, name, adjacent, where)), area, fenestration["u_value_w_m2k"], fenestration["shgc"] * area if label == "window" else 0.0)
        net = (gross - cut) if name != "ground" else gross
        if net > 1e-9:
            add(per_zone[surface["zone_id"]], "|".join((kind, name, adjacent, where)), net, u_value)
    return per_zone


def totals(rows, climate, member):
    """📊️ A, A/V, H_T, H'_T and the areas by sector of the conditioned spaces of a scope (the rules of DIN V 4108-6 the inference states)."""
    included = [s for s in sorted(rows) if member(s) and rows[s]["conditioned"]]
    area = loss = floor_area = volume = glazing = roof = floor_envelope = solar = 0.0
    opaque, windows, aperture = np.zeros(8), np.zeros(8), np.zeros(8)
    u_opaque = a_opaque = u_window = a_window = 0.0
    for space in included:
        floor_area += rows[space]["floor_area"]
        volume += rows[space]["volume"]
        for key, group in rows[space]["groups"].items():
            kind, boundary, adjacent, where = key.split("|")
            if boundary == "adiabatic":
                continue
            if boundary == "adjacent" and climate["conditioned"].get(adjacent, False) and member(adjacent):
                continue
            factor = {"exterior": 1.0, "ground": GROUND}.get(boundary, NEIGHBOUR)
            area += group["area"]
            loss += factor * group["loss"]
            if kind == "window":
                windows[int(where)] += group["area"]
                aperture[int(where)] += group["solar"]
                glazing += group["area"]
                u_window += group["loss"]
                a_window += group["area"]
            elif kind in ("wall", "door", "curtain-wall"):
                opaque[int(where)] += group["area"]
                u_opaque += group["loss"]
                a_opaque += group["area"]
            elif kind == "ceiling":
                roof += group["area"]
                u_opaque += group["loss"]
                a_opaque += group["area"]
            else:
                floor_envelope += group["area"]
                u_opaque += group["loss"]
                a_opaque += group["area"]
    facade = float(opaque.sum()) + glazing
    return {
        "spaces": included,
        "floor_area": floor_area,
        "volume": volume,
        "envelope_area": area,
        "a_over_v": area / volume if volume > 0 else 0.0,
        "transmission": loss,
        "h_t_prime": (loss + BRIDGE * area) / area if area > 0 else 0.0,
        "opaque_by_sector": opaque.tolist(),
        "window_by_sector": windows.tolist(),
        "solar_by_sector": aperture.tolist(),
        "roof_area": roof,
        "floor_envelope_area": floor_envelope,
        "glazing_area": glazing,
        "glazing_ratio": glazing / facade if facade > 0 else 0.0,
        "solar_aperture": float(aperture.sum()),
        "mean_u_opaque": u_opaque / a_opaque if a_opaque > 0 else 0.0,
        "mean_u_window": u_window / a_window if a_window > 0 else 0.0,
        "missing": 0,
    }


def build(rows, snapshot):
    """📏️ The table of rows (`conditioned`, `heated`, `floor_area`, `volume` and groups with area, loss and solar aperture): the rows without the solar apertures and the totals of every scope (membership from the snapshot)."""
    climate = {"conditioned": {s: row["conditioned"] for s, row in rows.items()}}
    scopes = [("zone:" + zone, lambda s, zone=zone: snapshot["spaces"][s].get("zone") == zone) for zone in sorted(snapshot.get("zones", {}))]
    scopes += [("building:" + building, lambda s, building=building: snapshot["storeys"][snapshot["spaces"][s]["storey"]]["building"] == building) for building in sorted(snapshot["buildings"])]
    scopes.append(("project", lambda s: True))
    result = {"spaces": {}, "totals": {}}
    for name, member in scopes:
        result["totals"][name] = totals(rows, climate, member)
    for space, row in sorted(rows.items()):
        result["spaces"][space] = dict(row, groups={key: {k: v for k, v in group.items() if k != "solar"} for key, group in sorted(row["groups"].items())})
    return result


def measure(export, snapshot):
    """📏️ The table of the export."""
    per_zone = surface_rows(export)
    rows = {}
    for zone_id, zone in export.zones.items():
        rows[space_of(zone)] = {"conditioned": export.conditioned(zone_id), "heated": export.heated(zone_id), "floor_area": export.spaces[zone_id]["floor_area_m2"], "volume": zone["volume_m3"], "groups": per_zone[zone_id]}
    return build(rows, snapshot)


def audit(export):
    """🔬️ Closed forms: the volume the polygons of a zone enclose, the footprint of its floors."""
    problems = []
    for zone_id, zone in export.zones.items():
        polygons = [surface["vertices_m"] for surface in export.model["surfaces"] if surface["zone_id"] == zone_id]
        volume = tetra_volume(polygons)
        if abs(volume - zone["volume_m3"]) > 1e-6 * max(1.0, zone["volume_m3"]):
            problems.append("%s: the polygons enclose %.9f m3, the zone says %.9f m3" % (zone["name"], volume, zone["volume_m3"]))
        floors = [Polygon([(x, y) for x, y, _ in surface["vertices_m"]]).area for surface in export.model["surfaces"] if surface["zone_id"] == zone_id and newell(surface["vertices_m"])[2] < -1e-9]
        if abs(sum(floors) - export.spaces[zone_id]["floor_area_m2"]) > 1e-6:
            problems.append("%s: the floors measure %.9f m2, the space says %.9f m2" % (zone["name"], sum(floors), export.spaces[zone_id]["floor_area_m2"]))
    return problems


# endregion 🔖️Table


# region 🔖️Compare
def compare(expected, actual, path=""):
    """⚖️ The differences of two JSON values, numbers within a relative and absolute 1e-9."""
    if isinstance(expected, dict) and isinstance(actual, dict):
        problems = ["%s: missing key %s" % (path, key) for key in expected if key not in actual] + ["%s: unexpected key %s" % (path, key) for key in actual if key not in expected]
        for key in expected:
            if key in actual:
                problems += compare(expected[key], actual[key], "%s/%s" % (path, key))
        return problems
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return ["%s: %d items against %d" % (path, len(expected), len(actual))]
        return [problem for index, (left, right) in enumerate(zip(expected, actual)) for problem in compare(left, right, "%s[%d]" % (path, index))]
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)) and not isinstance(expected, bool):
        return [] if abs(expected - actual) <= TOLERANCE * max(1.0, abs(expected)) else ["%s: %r against %r" % (path, expected, actual)]
    return [] if expected == actual else ["%s: %r against %r" % (path, expected, actual)]


def expected_table(snapshot):
    """🎯️ The table the inference oracle (shapely rooms, centimetre sampling) gives for the snapshot, with the windows and doors behind an interior boundary merged into their wall: the export has no opening behind a partition."""
    sibling = load_sibling("🌡️infer-bim-1-energy", "energy")
    model = sibling.Model(snapshot)
    rows = {}
    for space_id in sorted(snapshot["spaces"]):
        face = model.room[space_id]
        if face is None:
            continue
        groups = {}
        sibling.wall_groups(model, space_id, groups)
        sibling.horizontal_groups(model, space_id, groups, False)
        sibling.horizontal_groups(model, space_id, groups, True)
        for key in sorted(groups):
            kind, boundary, adjacent, where = key.split("|")
            if kind in ("window", "door") and boundary in ("adjacent", "adiabatic"):
                host = groups["|".join(("wall", boundary, adjacent, where))]
                opening = groups.pop(key)
                host["loss"] += host["loss"] / host["area"] * opening["area"]
                host["area"] += opening["area"]
        conditions = model.conditions(space_id) or {}
        rows[space_id] = {"conditioned": model.conditioned(space_id), "heated": "heating_setpoint" in conditions, "floor_area": face.area, "volume": face.area * model.rows[space_id]["clear_height"], "groups": groups}
    return build(rows, snapshot)


# endregion 🔖️Compare


# region 🔖️Handlers
def snapshot_path(root, case):
    return Path(HOUSE) if case.endswith("house") else root / case / "📸️snapshot" / "🔣️.json"


def measured(root, case):
    """📏️ Everything the oracle says of one case: problems, the measured table and the snapshot."""
    export = Export(json.loads((root / case / "🔋️model.json").read_text(encoding="utf-8")))
    snapshot = json.loads(snapshot_path(root, case).read_text(encoding="utf-8"))
    return validate(export.document) + integrity(export) + audit(export), measure(export, snapshot), snapshot


def export_handler(ctx):
    """🔋️ Oracle answer: the measured table of the committed export, after the audit."""
    from semio_repo_test import Outcome

    export = Export(json.loads(ctx.input_bytes(next(u for u in ctx.step_input_uris() if u.endswith("🔋️model.json"))).decode("utf-8")))
    snapshot = json.loads(ctx.input_bytes(next(u for u in ctx.step_input_uris() if "📸️snapshot" in u)).decode("utf-8"))
    problems = validate(export.document) + integrity(export) + audit(export)
    if problems:
        raise AssertionError("; ".join(problems))
    table = measure(export, snapshot)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("energy-export-room", export_handler).oracle("energy-export-pair", export_handler).oracle("energy-export-stack", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares the measured table of every committed export with the inference oracle's; `write` rewrites the measured tables."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for case in sorted(path.name for path in root.iterdir() if path.is_dir()):
        problems, table, snapshot = measured(root, case)
        failures += ["%s: %s" % (case, problem) for problem in problems]
        target = root / case / "🔬️measure-energy" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        else:
            failures += ["%s: %s" % (case, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), table, "measure")]
        failures += ["%s: %s" % (case, problem) for problem in compare(expected_table(snapshot), table, "export")]
        print("%s: %d zones, %d scopes" % (case, len(table["spaces"]), len(table["totals"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    from importlib.metadata import version

    print("%s: %s (jsonschema %s, numpy %s)" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees", version("jsonschema"), np.__version__))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
