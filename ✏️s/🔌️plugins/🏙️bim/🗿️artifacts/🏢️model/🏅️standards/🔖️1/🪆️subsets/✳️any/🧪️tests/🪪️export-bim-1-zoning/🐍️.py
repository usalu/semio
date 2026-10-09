#!/usr/bin/env python3
"""🏘️ Third-party ORACLE (IfcOpenShell) for the zoning part of the IFC 2x3 export of the BIM house.

IfcOpenShell 0.8.4.post1 has never seen this repository's writer. It opens the committed file `🧫️fixtures/🏗️ifc/🏘️zoned/🏘️zoned.ifc` and:

* parses it, requires the schema `IFC2X3` and validates every attribute type and every EXPRESS WHERE rule (`ifcopenshell.validate`);
* reads every `IfcZone` (name, `ObjectType` = category, `Pset_ZoneCommon.Reference`, the authored density of `Semio_Authoring`, the spaces its `IfcRelAssignsToGroup`
  assigns and the six totals of the element quantity `Semio_ZoneTotals`) with `ifcopenshell.util.element.get_psets`;
* reads every `IfcGroup` of `ObjectType` `AreaScheme` (name, measure, counted usages and zones from `Semio_Authoring`);
* reads `Pset_SpaceCoveringRequirements` of every `IfcSpace`;
* audits all of it against the committed snapshot with plain Python: the members of a zone are exactly the spaces that name it, an area scheme groups nothing,
  every reference equals its authoring id, a space sits in at most one zone, and every covering names the material the snapshot names.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🏗️ifc>     # rewrite the measured table from the committed file

@see ../../🔮️oracles/🔣️.json — the registration of the oracles this file answers beside
@see ../../🚪️io/📤️export/🏗️ifc/🏘️zoning/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.util.element
import ifcopenshell.validate

# endregion 🔖️Imports


# region 🔖️Reading
NEWLINE = chr(10)
TOTALS = ["GrossFloorArea", "NetFloorArea", "GrossVolume", "FloorFinishArea", "WallFinishArea", "CeilingFinishArea"]
"""📊️ The element quantities the export writes for a zone in `Semio_ZoneTotals`."""

CASE = ("🏘️zoned", "🏘️zoned.ifc")
SNAPSHOT = ("🏗️ifc", "🏘️zoned", "📸️snapshot", "🔣️.json")


def without_id(rows):
    """🧹️ A property set as IfcOpenShell returns it, without its `id` member."""
    return {name: value for name, value in rows.items() if name != "id"}


def members_of(group):
    """👥️ The `ObjectType` (the model id) of every space assigned to a group, sorted."""
    return sorted(obj.ObjectType for relation in group.IsGroupedBy or [] for obj in relation.RelatedObjects)


def zone_rows(model):
    """🏘️ Every `IfcZone` by its authoring id: name, category, reference, density, members and totals."""
    rows = {}
    for zone in model.by_type("IfcZone"):
        psets = ifcopenshell.util.element.get_psets(zone)
        authoring = without_id(psets.get("Semio_Authoring", {}))
        totals = without_id(ifcopenshell.util.element.get_psets(zone, qtos_only=True).get("Semio_ZoneTotals", {}))
        rows[authoring["Id"]] = {
            "name": zone.Name,
            "category": zone.ObjectType or "",
            "reference": psets.get("Pset_ZoneCommon", {}).get("Reference"),
            "occupancy_density": float(authoring["OccupancyDensity"]),
            "members": members_of(zone),
            "totals": {name: float(value) for name, value in totals.items()},
        }
    return dict(sorted(rows.items()))


def scheme_rows(model):
    """🗃️ Every `IfcGroup` of type `AreaScheme` by its authoring id: name, measure, counted usages and zones."""
    rows = {}
    for group in model.by_type("IfcGroup", include_subtypes=False):
        if group.ObjectType != "AreaScheme":
            continue
        authoring = without_id(ifcopenshell.util.element.get_psets(group).get("Semio_Authoring", {}))
        rows[authoring["Id"]] = {"name": group.Name, "measure": authoring["Measure"], "usages": json.loads(authoring["Usages"]), "zones": json.loads(authoring["Zones"])}
    return dict(sorted(rows.items()))


def covering_rows(model):
    """🎨️ The `Pset_SpaceCoveringRequirements` of every space that has one, by space id."""
    rows = {}
    for space in model.by_type("IfcSpace"):
        covering = without_id(ifcopenshell.util.element.get_psets(space).get("Pset_SpaceCoveringRequirements", {}))
        if covering:
            rows[space.ObjectType] = dict(sorted(covering.items()))
    return dict(sorted(rows.items()))


def measure(model):
    """📊️ The oracle table: zones, area schemes and coverings as IfcOpenShell reads them."""
    return {"zones": zone_rows(model), "schemes": scheme_rows(model), "coverings": covering_rows(model)}


# endregion 🔖️Reading


# region 🔖️Audit
def audit(model, snapshot, table):
    """⚖️ Every place where the file leaves the committed snapshot or itself."""
    problems = []
    if model.schema != "IFC2X3":
        problems.append("schema is %s, not IFC2X3" % model.schema)
    if set(table["zones"]) != set(snapshot["zones"]):
        problems.append("zones: %s in the file, %s in the snapshot" % (sorted(table["zones"]), sorted(snapshot["zones"])))
    if set(table["schemes"]) != set(snapshot["area_schemes"]):
        problems.append("area schemes: %s in the file, %s in the snapshot" % (sorted(table["schemes"]), sorted(snapshot["area_schemes"])))
    seen = {}
    for zone_id, row in table["zones"].items():
        source = snapshot["zones"].get(zone_id)
        if source is None:
            continue
        expected = sorted(space_id for space_id, space in snapshot["spaces"].items() if space.get("zone") == zone_id)
        if row["members"] != expected:
            problems.append("%s: members %s in the file, %s in the snapshot" % (zone_id, row["members"], expected))
        if (row["name"], row["category"], row["occupancy_density"]) != (source["name"], source["category"], source["occupancy_density"]):
            problems.append("%s: name, category or density differ from the snapshot" % zone_id)
        if row["reference"] != zone_id:
            problems.append("%s: Pset_ZoneCommon.Reference is %s" % (zone_id, row["reference"]))
        if set(row["totals"]) != set(TOTALS):
            problems.append("%s: totals %s" % (zone_id, sorted(row["totals"])))
        for member in row["members"]:
            if member in seen:
                problems.append("%s is a member of %s and %s" % (member, seen[member], zone_id))
            seen[member] = zone_id
    for scheme_id, row in table["schemes"].items():
        source = snapshot["area_schemes"].get(scheme_id)
        if source is not None and (row["name"], row["measure"], row["usages"], row["zones"]) != (source["name"], source["measure"], source["usages"], source["zones"]):
            problems.append("%s: the rule in the file differs from the snapshot" % scheme_id)
    for group in model.by_type("IfcGroup", include_subtypes=False):
        if group.ObjectType == "AreaScheme" and members_of(group):
            problems.append("%s: an area scheme must group no space" % group.Name)
    for space_id, space in snapshot["spaces"].items():
        expected = {row: snapshot["materials"][space[field]]["name"] for row, field in (("FloorCovering", "floor_finish"), ("WallCovering", "wall_finish"), ("CeilingCovering", "ceiling_finish")) if space.get(field)}
        if table["coverings"].get(space_id, {}) != dict(sorted(expected.items())):
            problems.append("%s: covering %s in the file, %s in the snapshot" % (space_id, table["coverings"].get(space_id, {}), expected))
    return problems


def open_case(root):
    """📂️ The committed snapshot, the IFC model and the directory of the case."""
    case = Path(root) / CASE[0]
    snapshot = json.loads(Path(root).joinpath(*SNAPSHOT[1:]).read_text(encoding="utf-8"))
    return case, snapshot, ifcopenshell.open(str(case / CASE[1]))


# endregion 🔖️Audit


# region 🔖️Handlers
def export_handler(ctx):
    """🏘️ Oracle answer: IfcOpenShell's table of the committed file."""
    from semio_repo_test import Outcome

    ifc_uri = next(uri for uri in ctx.step_input_uris() if uri.endswith(".ifc"))
    model = ifcopenshell.file.from_string(ctx.input_bytes(ifc_uri).decode("utf-8"))
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in ctx.step_input_uris() if "📸️snapshot" in uri)).decode("utf-8"))
    table = measure(model)
    problems = audit(model, snapshot, table)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-zoning-zoned", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], arguments[1]
    case, snapshot, model = open_case(root)
    table = measure(model)
    problems = ["%s" % problem for problem in audit(model, snapshot, table)]
    log = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, log, express_rules=True)
    problems += ["validate: %s" % entry["message"] for entry in log.statements if str(entry.get("level")).lower() == "error"]
    path = case / "🔬️measure" / "🔣️.json"
    if command == "write":
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + NEWLINE, encoding="utf-8", newline=NEWLINE)
        print("wrote %s: %d zones, %d area schemes, %d covered spaces" % (path, len(table["zones"]), len(table["schemes"]), len(table["coverings"])))
    elif json.loads(path.read_text(encoding="utf-8")) != table:
        problems.append("the committed table differs from the measurement of the committed file")
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
