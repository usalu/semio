#!/usr/bin/env python3
"""🔥️ Third-party ORACLE (IfcOpenShell + numpy) for the thermal data of the BIM model in IFC 2x3 and IFC4 files.

IfcOpenShell has never seen this repository's writer. It opens the committed files `🧫️fixtures/🏗️ifc/🔥️energy/<schema>.ifc` (the house with its conditions, window and door thermal data, an authored
`ThermalTransmittance` on a wall and on a window) and:

* reads the thermal property sets of every product (`Pset_SpaceThermalRequirements`, `Pset_WallCommon`, `Pset_SlabCommon`, `Pset_RoofCommon`, `Pset_WindowCommon`, `Pset_DoorCommon`,
  `Pset_DoorWindowGlazingType`) with their IFC value types, the `Conditions` and `DerivedRows` bookkeeping rows and the `UValue`/`GValue`/`FrameFraction` rows of the window and door types;
* checks every written property against the official property set templates shipped with IfcOpenShell (`Pset_IFC2X3.ifc`, `Pset_IFC4_ADD2.ifc`): the set exists, the property is part of it and the
  value type is the template's primary measure type;
* restates the rules from the snapshot alone: set points in kelvin (`SpaceTemperatureMin` and `WinterMin` = heating, `Max` and `SummerMax` = cooling), air conditioning when a cooling set point is stated, the
  outdoor air flow as air changes per hour (`rate * 3.6 / clear height`, the height from the space's `Qto_SpaceBaseQuantities`), the type values of windows and doors (`ThermalTransmittance`,
  `GlazingAreaFraction` = 1 - frame fraction, `SolarHeatGainTransmittance`), the ISO 6946 bounds of the U-value of a wall, slab or roof (it lies between the values the same layer stack has against the outdoors, the ground and a room,
  for the heat flow directions it can see), that an authored property wins and is not listed as derived, that a derived property is listed, and that IFC 2x3 has no `ThermalTransmittance` in `Pset_RoofCommon`;
* runs the EXPRESS rules of the schema over the file.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc/🔥️energy>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🏗️ifc/🔥️energy>     # rewrite the measured table from the committed files

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🔥️energy/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.util.element as element_util
import numpy as np

# endregion 🔖️Imports


# region 🔖️Vocabulary
SETS = ["Pset_SpaceThermalRequirements", "Pset_WallCommon", "Pset_CurtainWallCommon", "Pset_SlabCommon", "Pset_RoofCommon", "Pset_WindowCommon", "Pset_DoorCommon", "Pset_DoorWindowGlazingType"]
KELVIN = 273.15
RSI = {"up": 0.10, "horizontal": 0.13, "down": 0.17}
RSE = 0.04
TOLERANCE = 1e-9


def identity(product):
    """🔑️ The model id of a product: `Tag` of a building element, `ObjectType` of a space."""
    if product.is_a("IfcSpace"):
        return product.ObjectType
    return getattr(product, "Tag", None)


def cell(value):
    """🏷️ `[TYPE, value]` of a property value: the IFC type name in upper case and the Python value."""
    if value is None:
        return ["", None]
    return [value.is_a().upper(), value.wrappedValue]


# endregion 🔖️Vocabulary


# region 🔖️Reading
def property_sets(product):
    """📋️ The own property sets of a product: `set -> property -> [type, value]` (no inherited sets)."""
    found = {}
    for relation in getattr(product, "IsDefinedBy", None) or []:
        if relation.is_a("IfcRelDefinesByProperties") and relation.RelatingPropertyDefinition.is_a("IfcPropertySet"):
            found.update(read_set(relation.RelatingPropertyDefinition))
    for definition in getattr(product, "HasPropertySets", None) or []:
        if definition.is_a("IfcPropertySet"):
            found.update(read_set(definition))
    return found


def read_set(definition):
    return {definition.Name: {prop.Name: cell(prop.NominalValue) for prop in definition.HasProperties or [] if prop.is_a("IfcPropertySingleValue")}}


def measure(path):
    """📊️ The thermal table of one file (the same table the subject reports from its document)."""
    model = ifcopenshell.open(str(path))
    elements, derived, conditions, types = {}, {}, {}, {}
    for product in model.by_type("IfcProduct"):
        key = identity(product)
        if not key:
            continue
        sets = property_sets(product)
        rows = {name: dict(sorted(sets[name].items())) for name in SETS if name in sets}
        if rows:
            elements[key] = rows
        authoring = sets.get("Semio_Authoring", {})
        if "DerivedRows" in authoring:
            derived[key] = authoring["DerivedRows"][1]
        if "Conditions" in authoring:
            conditions[key] = authoring["Conditions"][1]
    for entity in ("IfcWindowStyle", "IfcWindowType", "IfcDoorStyle", "IfcDoorType"):
        try:
            found = model.by_type(entity)
        except RuntimeError:
            continue
        for kind in found:
            authoring = property_sets(kind).get("Semio_Authoring", {})
            rows = {name: authoring[name][1] for name in ("UValue", "GValue", "FrameFraction") if name in authoring}
            if rows:
                types[kind.Tag or kind.Name] = rows
    return model, {"schema": model.schema, "elements": elements, "derived": derived, "conditions": conditions, "types": types}


# endregion 🔖️Reading


# region 🔖️Templates
def templates_of(schema):
    """📚️ The official templates of a schema: `set -> property -> primary measure type (upper case)`."""
    name = {"IFC2X3": "Pset_IFC2X3.ifc", "IFC4": "Pset_IFC4_ADD2.ifc"}[schema]
    library = ifcopenshell.open(str(Path(ifcopenshell.__file__).parent / "util" / "schema" / name))
    found = {}
    for template in library.by_type("IfcPropertySetTemplate"):
        if template.Name in SETS:
            found[template.Name] = {prop.Name: (prop.PrimaryMeasureType or "").upper() for prop in template.HasPropertyTemplates if prop.is_a("IfcSimplePropertyTemplate")}
    return found


def template_problems(table):
    """🩺️ Every property the export derived exists in its standard set with the standard measure type (authored properties are the author's)."""
    problems = []
    official = templates_of(table["schema"])
    for key, sets in table["elements"].items():
        mine = set(table["derived"].get(key, "").split(","))
        for name, rows in sets.items():
            if name not in official:
                problems.append("%s: the standard set %s has no template" % (key, name))
                continue
            for prop, (kind, _) in rows.items():
                if "%s.%s" % (name, prop) not in mine:
                    continue
                if prop not in official[name]:
                    problems.append("%s: %s.%s is no property of the standard set" % (key, name, prop))
                elif official[name][prop] != kind:
                    problems.append("%s: %s.%s is %s, the template says %s" % (key, name, prop, kind, official[name][prop]))
    return problems


# endregion 🔖️Templates


# region 🔖️Rules
def layer_resistance(snapshot, layers):
    total = 0.0
    for layer in layers:
        material = snapshot["materials"][layer["material"]]
        total += layer["thickness"] / material["conductivity"]
    return total


def bounds(snapshot, layers, flows):
    """🧱️ The smallest and largest ISO 6946 U-value of a layer stack for the heat flow directions and far sides a surface can see."""
    inner = layer_resistance(snapshot, layers)
    values = []
    for flow in flows:
        for far in ("exterior", "ground", "room"):
            outer = {"exterior": RSE, "ground": 0.0, "room": RSI[flow]}[far]
            values.append(1.0 / (RSI[flow] + inner + outer))
    return min(values), max(values)


def authored(snapshot, holders, set_name, prop):
    for holder in holders:
        row = snapshot.get("properties", {}).get(holder, {}).get(set_name, {}).get(prop)
        if row is not None:
            return next(iter(row.values()))["value"]
    return None


def rules_problems(snapshot, table, model):
    """🩺️ The rules of the thermal sets restated from the snapshot alone."""
    problems = []
    elements, derived = table["elements"], table["derived"]
    v4 = table["schema"] == "IFC4"

    def listed(key, set_name, prop):
        return "%s.%s" % (set_name, prop) in derived.get(key, "").split(",")

    def close(a, b, tolerance=TOLERANCE):
        return abs(a - b) <= tolerance * max(1.0, abs(b))

    for space_id, conditions in snapshot.get("space_conditions", {}).items():
        rows = elements.get(space_id, {}).get("Pset_SpaceThermalRequirements")
        if rows is None:
            problems.append("%s: no Pset_SpaceThermalRequirements" % space_id)
            continue
        for prop, key in (("SpaceTemperatureMin", "heating_setpoint"), ("SpaceTemperatureWinterMin", "heating_setpoint"), ("SpaceTemperatureMax", "cooling_setpoint"), ("SpaceTemperatureSummerMax", "cooling_setpoint")):
            if key in conditions:
                if prop not in rows or not close(rows[prop][1], conditions[key] + KELVIN):
                    problems.append("%s: %s is %s, not %s K" % (space_id, prop, rows.get(prop), conditions[key] + KELVIN))
            elif prop in rows:
                problems.append("%s: %s written without a set point" % (space_id, prop))
        if rows.get("AirConditioning", [None, None])[1] != ("cooling_setpoint" in conditions):
            problems.append("%s: AirConditioning disagrees with the cooling set point" % space_id)
        space = next((entity for entity in model.by_type("IfcSpace") if entity.ObjectType == space_id), None)
        height = (element_util.get_psets(space, qtos_only=True).get("Qto_SpaceBaseQuantities") or {}).get("Height") if space is not None else None
        conditioned = "heating_setpoint" in conditions or "cooling_setpoint" in conditions
        name = "MechanicalVentilationRate" if conditioned else "NaturalVentilationRate"
        if "ventilation_rate" in conditions and height:
            if name not in rows or not close(rows[name][1], conditions["ventilation_rate"] * 3.6 / height, 1e-6):
                problems.append("%s: %s is %s, not %s" % (space_id, name, rows.get(name), conditions["ventilation_rate"] * 3.6 / height))
        elif "MechanicalVentilationRate" in rows or "NaturalVentilationRate" in rows:
            problems.append("%s: a ventilation rate without an outdoor air flow" % space_id)
        record = table["conditions"].get(space_id)
        if record is None or json.loads(record) != conditions:
            problems.append("%s: the Conditions row %r is not the snapshot's record" % (space_id, record))
        for prop in rows:
            if not listed(space_id, "Pset_SpaceThermalRequirements", prop):
                problems.append("%s: the derived row Pset_SpaceThermalRequirements.%s is not listed in DerivedRows" % (space_id, prop))
    for space_id in table["conditions"]:
        if space_id not in snapshot.get("space_conditions", {}):
            problems.append("%s: a Conditions row without conditions in the snapshot" % space_id)
    for opening_id, opening in snapshot["openings"].items():
        kind = opening["kind"]
        if "Window" in kind:
            type_id, record, common = kind["Window"]["window_type"], snapshot["window_types"][kind["Window"]["window_type"]], "Pset_WindowCommon"
        elif "Door" in kind:
            type_id, record, common = kind["Door"]["door_type"], snapshot["door_types"][kind["Door"]["door_type"]], "Pset_DoorCommon"
        else:
            continue
        rows = elements.get(opening_id, {}).get(common, {})
        holders = [opening_id, type_id]
        wanted = [("ThermalTransmittance", record.get("u_value"))]
        if common == "Pset_WindowCommon":
            wanted.append(("GlazingAreaFraction", None if record.get("frame_fraction") is None else 1.0 - record["frame_fraction"]))
        for prop, value in wanted:
            override = authored(snapshot, holders, common, prop)
            if override is not None:
                if prop in rows and listed(opening_id, common, prop):
                    problems.append("%s: the authored %s.%s is listed as derived" % (opening_id, common, prop))
                if opening_id in snapshot.get("properties", {}) and prop in snapshot["properties"][opening_id].get(common, {}) and (prop not in rows or not close(rows[prop][1], override)):
                    problems.append("%s: the authored %s.%s = %s was not kept (%s)" % (opening_id, common, prop, override, rows.get(prop)))
            elif value is not None:
                if prop not in rows or not close(rows[prop][1], value):
                    problems.append("%s: %s.%s is %s, not %s" % (opening_id, common, prop, rows.get(prop), value))
                elif not listed(opening_id, common, prop):
                    problems.append("%s: the derived %s.%s is not listed in DerivedRows" % (opening_id, common, prop))
            elif prop in rows:
                problems.append("%s: %s.%s written without data" % (opening_id, common, prop))
        if common == "Pset_WindowCommon" and record.get("g_value") is not None and authored(snapshot, holders, "Pset_DoorWindowGlazingType", "SolarHeatGainTransmittance") is None:
            glazing = elements.get(opening_id, {}).get("Pset_DoorWindowGlazingType", {}).get("SolarHeatGainTransmittance")
            if glazing is None or not close(glazing[1], record["g_value"]):
                problems.append("%s: SolarHeatGainTransmittance is %s, not %s" % (opening_id, glazing, record["g_value"]))
    for type_id, record in {**snapshot["window_types"], **snapshot["door_types"]}.items():
        written = table["types"].get(type_id, {})
        for prop, key in (("UValue", "u_value"), ("GValue", "g_value"), ("FrameFraction", "frame_fraction")):
            if key in record and (prop not in written or not close(written[prop], record[key])):
                problems.append("%s: the type row %s is %s, not %s" % (type_id, prop, written.get(prop), record[key]))
            if key not in record and prop in written:
                problems.append("%s: the type row %s has no data in the snapshot" % (type_id, prop))
    for wall_id, wall in snapshot["walls"].items():
        rows = elements.get(wall_id, {}).get("Pset_WallCommon", {})
        override = authored(snapshot, [wall_id], "Pset_WallCommon", "ThermalTransmittance")
        if override is not None:
            if "ThermalTransmittance" not in rows or not close(rows["ThermalTransmittance"][1], override):
                problems.append("%s: the authored ThermalTransmittance %s was not kept" % (wall_id, override))
            if listed(wall_id, "Pset_WallCommon", "ThermalTransmittance"):
                problems.append("%s: the authored ThermalTransmittance is listed as derived" % wall_id)
        elif "ThermalTransmittance" in rows:
            low, high = bounds(snapshot, snapshot["wall_types"][wall["wall_type"]]["layers"], ["horizontal"])
            if not low - 1e-9 <= rows["ThermalTransmittance"][1] <= high + 1e-9:
                problems.append("%s: U %s is outside the ISO 6946 bounds %s..%s of its layers" % (wall_id, rows["ThermalTransmittance"][1], low, high))
            if not listed(wall_id, "Pset_WallCommon", "ThermalTransmittance"):
                problems.append("%s: the derived ThermalTransmittance is not listed in DerivedRows" % wall_id)
    for wall_id, wall in snapshot.get("curtain_walls", {}).items():
        rows = elements.get(wall_id, {}).get("Pset_CurtainWallCommon", {})
        override = authored(snapshot, [wall_id], "Pset_CurtainWallCommon", "ThermalTransmittance")
        u_value = snapshot["curtain_wall_types"][wall["curtain_wall_type"]].get("u_value")
        if override is not None:
            if "ThermalTransmittance" not in rows or not close(rows["ThermalTransmittance"][1], override):
                problems.append("%s: the authored ThermalTransmittance %s was not kept" % (wall_id, override))
        elif "ThermalTransmittance" in rows:
            if u_value is None or not close(rows["ThermalTransmittance"][1], u_value):
                problems.append("%s: the curtain wall U %s is not the type's %s" % (wall_id, rows["ThermalTransmittance"], u_value))
    for collection, set_name, types, kind_key in (("slabs", "Pset_SlabCommon", "slab_types", "slab_type"), ("roofs", "Pset_RoofCommon", "roof_types", "roof_type")):
        for identity_, row in snapshot[collection].items():
            rows = elements.get(identity_, {}).get(set_name, {})
            if set_name == "Pset_RoofCommon" and not v4 and "ThermalTransmittance" in rows:
                problems.append("%s: IFC 2x3 has no ThermalTransmittance in Pset_RoofCommon" % identity_)
            override = authored(snapshot, [identity_], set_name, "ThermalTransmittance")
            if override is not None:
                continue
            if "ThermalTransmittance" in rows:
                low, high = bounds(snapshot, snapshot[types][row[kind_key]]["layers"], ["up", "down"])
                if not low - 1e-9 <= rows["ThermalTransmittance"][1] <= high + 1e-9:
                    problems.append("%s: U %s is outside the ISO 6946 bounds %s..%s of its layers" % (identity_, rows["ThermalTransmittance"][1], low, high))
    return problems


def express_problems(model):
    """🩺️ The EXPRESS rules of the schema over the file."""
    try:
        import ifcopenshell.validate as validate
    except ImportError:
        return []

    class Collector:
        def __init__(self):
            self.rows = []

        def set_state(self, *_):
            pass

        def error(self, message, *_):
            self.rows.append(message)

        def warning(self, message, *_):
            pass

        def info(self, message, *_):
            pass

        def debug(self, message, *_):
            pass

        def log(self, *_):
            pass

        def critical(self, message, *_):
            self.rows.append(message)

    collector = Collector()
    try:
        validate.validate(model, collector, express_rules=True)
    except Exception as error:  # the validator is a third-party tool; report, never crash the oracle
        return ["the validator failed: %r" % (error,)]
    return ["%s" % row for row in collector.rows]


# endregion 🔖️Rules


# region 🔖️Handlers
def audit(path, snapshot):
    model, table = measure(path)
    problems = template_problems(table) + rules_problems(snapshot, table, model) + express_problems(model)
    return table, problems


def export_handler(ctx):
    """🔥️ Oracle answer: the table of the committed file, after the audit."""
    import tempfile

    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "📸️snapshot" in uri)).decode("utf-8"))
    document = ctx.input_bytes(next(uri for uri in uris if uri.endswith(".ifc")))
    with tempfile.TemporaryDirectory() as folder:
        path = Path(folder) / "file.ifc"
        path.write_bytes(document)
        table, problems = audit(path, snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-ifc-energy-2x3", export_handler).oracle("export-ifc-energy-4", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed files against the committed tables; `write` rewrites the tables from the files."""
    command, root = arguments[0], Path(arguments[1])
    snapshot = json.loads((root / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    problems = []
    for name in ("energy-2x3", "energy-4"):
        path = root / (name + ".ifc")
        table, found = audit(path, snapshot)
        problems += ["%s: %s" % (name, item) for item in found]
        target = root / "🔬️measure" / (name + ".json")
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote the table of %d elements, %d derived lists, %d conditions" % (name, len(table["elements"]), len(table["derived"]), len(table["conditions"])))
        elif json.loads(target.read_text(encoding="utf-8")) != json.loads(json.dumps(table)):
            problems.append("%s: the committed table differs from the measurement of the committed file" % name)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s, numpy %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version, np.__version__))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
