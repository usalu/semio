"""Builds, with ifcopenshell only, a small IFC 2x3 file and an IFC4 library in the structure the Rust export writes for the psets fixture, and runs the oracle's audits on them.

It exercises the oracle code (tables and audits) before the Rust crate compiles; it is not a substitute for the committed files the Rust export writes.

Usage: python -X utf8 -I r12-w2-wp18-psets-io-prototype.py <oracle 🐍️.py> <psets snapshot json> <output directory>
"""

import importlib.util
import json
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.validate
import ifcopenshell.guid


def load(path):
    spec = importlib.util.spec_from_file_location("oracle", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def guid(key):
    return ifcopenshell.guid.compress(__import__("uuid").uuid5(__import__("uuid").NAMESPACE_URL, key).hex)


def owner_history(f):
    c = f.create_entity
    person = c("IfcPerson", None, "a", None, None, None, None, None, None)
    org = c("IfcOrganization", None, "o", None, None)
    pao = c("IfcPersonAndOrganization", person, org, None)
    app = c("IfcApplication", org, "1", "semio BIM", "semio.bim")
    return c("IfcOwnerHistory", pao, app, None, "ADDED", None, None, None, 0)


def nominal(f, value):
    kind, body = next(iter(value.items()))
    wrapped = {"Text": "IfcLabel", "Real": "IfcReal", "Integer": "IfcInteger", "Boolean": "IfcBoolean", "Length": "IfcLengthMeasure", "Area": "IfcAreaMeasure", "Volume": "IfcVolumeMeasure", "Angle": "IfcPlaneAngleMeasure"}[kind]
    return f.create_entity(wrapped, body["value"])


def property_set(f, owner, key, name, rows):
    props = [f.create_entity("IfcPropertySingleValue", prop, None, nominal(f, value), None) for prop, value in rows.items()]
    return f.create_entity("IfcPropertySet", guid(key), owner, name, None, props)


def classification_2x3(f, owner, snapshot, project, holders):
    c = f.create_entity
    systems, references = {}, {}
    parents = {}
    for system_id, system in snapshot["classification_systems"].items():
        source = c("IfcClassification", system.get("source") or "", system.get("edition", ""), None, system["name"])
        systems[system_id] = source
        for entry in system["entries"]:
            references[(system_id, entry["code"])] = c("IfcClassificationReference", None, entry["code"], entry["title"], source)
            if entry.get("parent"):
                parents["%s|%s|%s" % (system["name"], system.get("edition", ""), entry["code"])] = entry["parent"]
    if parents:
        rows = {key: {"Text": {"value": parent}} for key, parent in parents.items()}
        pset = property_set(f, owner, "project:classification-parents", "Semio_ClassificationParents", rows)
        c("IfcRelDefinesByProperties", guid("project:parents"), owner, None, None, [project], pset)
    attached = {}
    for holder, assigned in snapshot["classifications"].items():
        for system_id, code in assigned.items():
            attached.setdefault(references[(system_id, code)], []).append(holders[holder])
    for reference, entities in attached.items():
        c("IfcRelAssociatesClassification", guid("assoc:%d" % reference.id()), owner, None, None, entities, reference)


def build_2x3(oracle, snapshot, path):
    f = ifcopenshell.file(schema="IFC2X3")
    c = f.create_entity
    owner = owner_history(f)
    origin = c("IfcAxis2Placement3D", c("IfcCartesianPoint", (0.0, 0.0, 0.0)), None, None)
    context = c("IfcGeometricRepresentationContext", None, "Model", 3, 1e-6, origin, c("IfcDirection", (0.0, 1.0)))
    units = c("IfcUnitAssignment", [c("IfcSIUnit", None, "LENGTHUNIT", None, "METRE")])
    project = c("IfcProject", guid("project"), owner, "IFC Psets", None, None, None, None, [context], units)
    holders = {}
    kinds = [("wall_types", "IfcWallType", "STANDARD"), ("door_types", "IfcDoorStyle", None), ("window_types", "IfcWindowStyle", None), ("column_types", "IfcColumnType", "COLUMN")]
    types = {}
    for collection, entity, predefined in kinds:
        for type_id, row in snapshot[collection].items():
            sets = [property_set(f, owner, "%s:%s:authoring" % (collection, type_id), "Semio_Authoring", {"Width": {"Real": {"value": 1.0}}})]
            sets += [property_set(f, owner, "%s:%s" % (type_id, name), name, rows) for name, rows in snapshot["properties"].get(type_id, {}).items()]
            if entity == "IfcWallType":
                holders[type_id] = c(entity, guid("type:" + type_id), owner, row["name"], None, None, sets, None, type_id, None, predefined)
            elif entity == "IfcColumnType":
                holders[type_id] = c(entity, guid("type:" + type_id), owner, row["name"], None, None, sets, None, type_id, None, predefined)
            elif entity == "IfcDoorStyle":
                holders[type_id] = c(entity, guid("type:" + type_id), owner, row["name"], None, None, sets, None, type_id, "SINGLE_SWING_LEFT", "NOTDEFINED", False, True)
            else:
                holders[type_id] = c(entity, guid("type:" + type_id), owner, row["name"], None, None, sets, None, type_id, "NOTDEFINED", "NOTDEFINED", False, True)
            types[type_id] = holders[type_id]
    for collection, entity, field in [("walls", "IfcWall", "wall_type"), ("columns", "IfcColumn", "column_type")]:
        for element_id, row in snapshot[collection].items():
            element = c(entity, guid("element:" + element_id), owner, element_id, None, None, None, None, element_id)
            holders[element_id] = element
            if row[field] in types:
                c("IfcRelDefinesByType", guid("typed:" + element_id), owner, None, None, [element], types[row[field]])
            for name, rows in snapshot["properties"].get(element_id, {}).items():
                c("IfcRelDefinesByProperties", guid("define:%s:%s" % (element_id, name)), owner, None, None, [element], property_set(f, owner, "%s:%s" % (element_id, name), name, rows))
    for storey_id, row in snapshot["storeys"].items():
        holders[storey_id] = c("IfcBuildingStorey", guid("storey:" + storey_id), owner, row["name"], None, storey_id, None, None, None, "ELEMENT", 0.0)
    classification_2x3(f, owner, snapshot, project, holders)
    path.write_text(f.to_string(), encoding="utf-8", newline="\n")
    return f


def build_library(oracle, snapshot, path):
    f = ifcopenshell.file(schema="IFC4")
    c = f.create_entity
    owner = owner_history(f)
    owner.ChangeAction = "NOCHANGE"
    origin = c("IfcAxis2Placement3D", c("IfcCartesianPoint", (0.0, 0.0, 0.0)), None, None)
    context = c("IfcGeometricRepresentationContext", None, "Model", 3, 1e-6, origin, c("IfcDirection", (0.0, 1.0)))
    units = c("IfcUnitAssignment", [c("IfcSIUnit", None, "LENGTHUNIT", None, "METRE")])
    project = c("IfcProject", guid("lib-project"), owner, "IFC Psets", None, None, None, None, [context], units)
    library = c("IfcProjectLibrary", guid("lib"), owner, "IFC Psets Library", None, None, None, None, [context], units)
    c("IfcRelDeclares", guid("lib-declares"), owner, None, None, project, [library])
    templates = []
    for template_id, template in snapshot["property_templates"].items():
        rows = []
        for definition in template["properties"]:
            parts = []
            if definition.get("description") is not None:
                parts.append(("description", definition["description"]))
            if definition.get("unit") is not None:
                parts.append(("unit", definition["unit"]))
            parts.append(("required", str(definition["required"]).lower()))
            if definition.get("default_value") is not None:
                value = oracle.written_value(definition["default_value"])[1]
                parts.append(("default", str(value).lower() if isinstance(value, bool) else oracle.format_number(value) if isinstance(value, (int, float)) else value))
            for key in ("minimum", "maximum"):
                if definition.get(key) is not None:
                    parts.append((key, oracle.format_number(definition[key])))
            text = "".join("%s=%s;" % (key, value.replace("%", "%25").replace(";", "%3B").replace("=", "%3D")) for key, value in parts)
            enumeration = None
            if definition["allowed"]:
                enumeration = c("IfcPropertyEnumeration", definition["name"], [nominal(f, value) for value in definition["allowed"]], None)
            rows.append(c("IfcSimplePropertyTemplate", guid("simple:%s:%s" % (template_id, definition["name"])), owner, definition["name"], text, "P_ENUMERATEDVALUE" if enumeration else "P_SINGLEVALUE", oracle.MEASURES[definition["kind"]], None, enumeration, None, None, None, None))
        targets = template["applies_to"]
        templates.append(c("IfcPropertySetTemplate", guid("template:" + template_id), owner, template["name"], None, oracle.template_kind(targets), ",".join(oracle.ENTITIES[t] for t in targets), rows))
    c("IfcRelDeclares", guid("lib-templates"), owner, None, None, library, templates)
    for system_id, system in snapshot["classification_systems"].items():
        source = c("IfcClassification", system.get("source"), system.get("edition") or None, None, system["name"], None, None, None)
        written = {}
        pending = list(enumerate(system["entries"]))
        while pending:
            rest = []
            for index, entry in pending:
                parent = source if not entry.get("parent") else written.get(entry["parent"])
                if parent is None:
                    rest.append((index, entry))
                else:
                    written[entry["code"]] = c("IfcClassificationReference", None, entry["code"], entry["title"], parent, None, "%06d" % index)
            pending = rest
        c("IfcRelAssociatesClassification", guid("lib-assoc:" + system_id), owner, None, None, [library], source)
    path.write_text(f.to_string(), encoding="utf-8", newline="\n")
    return f


def validate(model):
    log = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, log, express_rules=True)
    return [entry["message"].splitlines()[0] for entry in log.statements if str(entry.get("level")).lower() == "error"]


def main(oracle_path, snapshot_path, out):
    oracle = load(oracle_path)
    snapshot = json.loads(Path(snapshot_path).read_text(encoding="utf-8"))
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    problems = []
    model = build_2x3(oracle, snapshot, out / "proto-2x3.ifc")
    problems += ["2x3 validate: " + message for message in validate(model)]
    table = {"classifications": oracle.classification_rows(model), "type_properties": oracle.type_property_rows(model)}
    problems += ["2x3 classification: " + problem for problem in oracle.classification_problems(model, snapshot, table)]
    problems += ["2x3 properties: " + problem for problem in oracle.property_problems(model, snapshot)]
    print(json.dumps(table["classifications"], indent=1)[:1500])
    print(json.dumps(table["type_properties"])[:900])
    library = build_library(oracle, snapshot, out / "proto-library.ifc")
    problems += ["IFC4 validate: " + message for message in validate(library)]
    library_table = oracle.library_table(library)
    problems += ["IFC4 audit: " + problem for problem in oracle.library_problems(library, snapshot, library_table)]
    print(json.dumps(library_table)[:1800])
    for problem in problems:
        print("[FAIL]", problem)
    print("prototype:", "%d problem(s)" % len(problems) if problems else "audits agree")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:4]))
