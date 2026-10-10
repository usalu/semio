#!/usr/bin/env python3
"""🏗️ Third-party ORACLE (IfcOpenShell) for the IFC4 export of the BIM model.

IfcOpenShell 0.8.4.post1 has never seen this repository's writer. It opens the committed IFC4 (ADD2 TC1) files of the fixtures `🧩️ifc4/<case>/<case>.ifc`, and:

* parses each, requires the schema `IFC4`, no `IfcOwnerHistory`, and validates every attribute type and every EXPRESS WHERE rule (`ifcopenshell.validate`);
* counts the entities of every class the export writes (the IFC4 classes: window, door and roof types, triangulated face sets, the project library and its templates);
* reads the spatial containment of every product, tessellates every straight wall, level slab, column, beam and covering with its C++ kernel and requires the volume to equal the written base quantity;
* requires the occurrence and the type of every door to carry the operation of the door type of the snapshot (leaves and swing) and every window type the partitioning of its panes;
* reads the project library (property set templates with their simple property templates, enumerations and `key=value;` descriptions, classification systems chained by `ReferencedSource` in `Sort` order, the
  declarations and the associations), the `IfcRelDefinesByTemplate` of every property set named like a template, and the classifications attached to every element and type;
* reads the user property sets of every type object and element and requires them to equal the snapshot.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures> [case...]    # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures> [case...]    # rewrite the measured table from the committed file

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.util.classification
import ifcopenshell.util.element
import ifcopenshell.util.shape
import ifcopenshell.validate

# endregion 🔖️Imports


# region 🔖️Measurement
PSETS = "psets"
TOLERANCE = 1e-9


NEWLINE = chr(10)


MEASURED = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab", "IfcCovering", "IfcFurnishingElement", "IfcFlowTerminal", "IfcBuildingElementProxy", "IfcFlowSegment"]


def identity(element):
    """🔖️ The model id the export carries in `Tag` (elements and types), `Name` (annotations) or `ObjectType` (spatial elements)."""
    if element.is_a("IfcAnnotation"):
        return element.Name
    return element.Tag if element.is_a("IfcElement") or element.is_a("IfcTypeProduct") else element.ObjectType


def settings():
    """⚙️ Kernel settings: world coordinates, openings subtracted."""
    kernel = ifcopenshell.geom.settings()
    kernel.set("use-world-coords", True)
    return kernel


def kernel_volume(kernel, element):
    """🧊️ The volume of the tessellated element (`None` when it has no swept or faceted body)."""
    try:
        shape = ifcopenshell.geom.create_shape(kernel, element)
    except RuntimeError:
        return None
    geometry = shape.geometry
    return float(ifcopenshell.util.shape.get_volume(geometry))


def written_volume(element):
    """📏️ The volume written in the element's base quantities (`NetVolume`, else `GrossVolume`; a covering has none and writes `NetArea` and `Width`, whose product is the volume of its sweep)."""
    for name, values in ifcopenshell.util.element.get_psets(element, qtos_only=True).items():
        if name.endswith("BaseQuantities"):
            for key in ("NetVolume", "GrossVolume"):
                if key in values:
                    return float(values[key])
            if "NetArea" in values and "Width" in values:
                return float(values["NetArea"]) * float(values["Width"])
    return None


def straight(element, snapshot):
    """➖️ Whether the element is measured exactly by the kernel: not a curved wall, a sloped slab or a round profile (the kernel tessellates circles)."""
    tag = element.Tag
    if element.is_a("IfcWall") and not element.is_a("IfcWallStandardCase"):
        wall = snapshot["walls"][tag]
        return "Line" in wall["axis"] and not wall.get("base_slab") and next(iter(wall["top"])) not in ("Roof", "Slab", "Ceiling")
    if element.is_a("IfcSlab"):
        slab = snapshot["slabs"].get(tag)
        return slab is not None and not slab.get("slope") and all(vertex["bulge"] == 0 for loop in [slab["boundary"], *slab["holes"]] for vertex in loop)
    if element.is_a("IfcCovering"):
        ceiling = snapshot["ceilings"].get(tag)
        return ceiling is not None and not ceiling.get("slope") and all(vertex["bulge"] == 0 for loop in [ceiling["boundary"], *ceiling["holes"]] for vertex in loop)
    if element.is_a("IfcColumn"):
        return "Circle" not in snapshot["column_types"][snapshot["columns"][tag]["column_type"]]["profile"]
    if element.is_a("IfcBeam"):
        return "Circle" not in snapshot["beam_types"][snapshot["beams"][tag]["beam_type"]]["profile"]
    if element.is_a("IfcFlowSegment"):
        return "Pipe" not in snapshot.get("mep_elements", {}).get(tag, {}).get("shape", {})
    if element.is_a("IfcFurnishingElement") or element.is_a("IfcFlowTerminal") or element.is_a("IfcBuildingElementProxy"):
        return tag in snapshot.get("components", {})
    return True


def annotation_rows(model):
    """🪧️ Every `IfcAnnotation` by name: kind, printed text, curve count, sorted text literals and the written total of a dimension."""
    rows = {}
    for element in model.by_type("IfcAnnotation"):
        items = [item for representation in (element.Representation.Representations if element.Representation else []) for item in representation.Items]
        quantities = ifcopenshell.util.element.get_psets(element, qtos_only=True).get("Semio_DimensionValue", {})
        rows[element.Name] = {
            "kind": element.ObjectType,
            "printed": element.Description or "",
            "curves": sum(item.is_a("IfcPolyline") or item.is_a("IfcCircle") for item in items),
            "literals": sorted(item.Literal for item in items if item.is_a("IfcTextLiteralWithExtent")),
            "total": float(quantities["Total"]) if "Total" in quantities else None,
        }
    return dict(sorted(rows.items()))


TYPE_NAMES = {"Text": "IFCLABEL", "Real": "IFCREAL", "Integer": "IFCINTEGER", "Boolean": "IFCBOOLEAN", "Length": "IFCLENGTHMEASURE", "Area": "IFCAREAMEASURE", "Volume": "IFCVOLUMEMEASURE", "Angle": "IFCPLANEANGLEMEASURE"}


TYPE_COLLECTIONS = ["wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types"]


def system_key(system):
    """🔑️ `name|edition` of an `IfcClassification`."""
    return "%s|%s" % (system.Name or "", system.Edition or "")


def item_reference(reference):
    """🔖️ The code of an `IfcClassificationReference` (`ItemReference` in IFC 2x3, `Identification` in IFC4)."""
    return getattr(reference, "ItemReference", None) or getattr(reference, "Identification", None) or ""


def type_property_rows(model):
    """🏷️ The user property sets of every type object by type id, set name and property name: the written IFC value type and the value (`Semio_Authoring` is bookkeeping)."""
    rows = {}
    for kind in model.by_type("IfcTypeProduct"):
        sets = {}
        for definition in kind.HasPropertySets or []:
            if definition.is_a("IfcPropertySet") and definition.Name != "Semio_Authoring":
                sets[definition.Name] = {row.Name: [row.NominalValue.is_a().upper(), row.NominalValue.wrappedValue] for row in definition.HasProperties}
        if sets and kind.Tag:
            rows[kind.Tag] = dict(sorted(sets.items()))
    return dict(sorted(rows.items()))


def kernel_bounds(kernel, element):
    """📦️ The world-space `(min, max)` corners of the tessellated element, or `None`."""
    try:
        shape = ifcopenshell.geom.create_shape(kernel, element)
    except RuntimeError:
        return None
    geometry = shape.geometry
    vertices = ifcopenshell.util.shape.get_vertices(geometry)
    return vertices.min(axis=0), vertices.max(axis=0)


def placement_problems(model):
    """📍️ Every window or door whose kernel geometry leaves the bounding box of the wall it fills (on a curved wall the flat frame
    protrudes by the sagitta of its width, so only its centre must lie inside)."""
    kernel = settings()
    problems = []
    for relation in model.by_type("IfcRelFillsElement"):
        filling = relation.RelatedBuildingElement
        host = relation.RelatingOpeningElement.VoidsElements[0].RelatingBuildingElement
        inner, outer = kernel_bounds(kernel, filling), kernel_bounds(kernel, host)
        if inner is None or outer is None:
            problems.append("%s or its host %s has no geometry" % (filling.Tag, host.Tag))
        elif host.is_a("IfcWallStandardCase") and ((inner[0] < outer[0] - 1e-6).any() or (inner[1] > outer[1] + 1e-6).any()):
            problems.append("%s leaves the bounds of its host %s" % (filling.Tag, host.Tag))
        elif not host.is_a("IfcWallStandardCase") and (((inner[0] + inner[1]) / 2 < outer[0]).any() or ((inner[0] + inner[1]) / 2 > outer[1]).any()):
            problems.append("%s leaves the bounds of its host %s" % (filling.Tag, host.Tag))
    return problems


def authored(element):
    """🏷️ The `Semio_Authoring` rows of an element."""
    return ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {})


def keyed(snapshot, assigned):
    """🔑️ The codes of one holder by `name|edition` of their system."""
    systems = snapshot.get("classification_systems", {})
    return {"%s|%s" % (systems[system]["name"], systems[system].get("edition", "")): code for system, code in assigned.items() if system in systems}


def written_value(value):
    """🏷️ `[IFC value type, value]` of a model property value such as `{"Length": {"value": 1.8}}`."""
    kind, body = next(iter(value.items()))
    return [TYPE_NAMES[kind], body["value"]]


def same_value(left, right):
    """⚖️ Equality of two decoded values: numbers within a relative 1e-9, a boolean is never a number."""
    if isinstance(left, bool) or isinstance(right, bool):
        return type(left) is type(right) and left == right
    if isinstance(left, (int, float)) and isinstance(right, (int, float)):
        return abs(left - right) <= TOLERANCE * max(abs(left), abs(right), 1.0)
    if isinstance(left, (list, tuple)) and isinstance(right, (list, tuple)):
        return len(left) == len(right) and all(same_value(a, b) for a, b in zip(left, right))
    if isinstance(left, dict) and isinstance(right, dict):
        return left.keys() == right.keys() and all(same_value(left[key], right[key]) for key in left)
    return left == right


def property_problems(model, snapshot):
    """🏷️ The authored property sets against the snapshot: a type object carries the sets authored on its type record in `HasPropertySets` (IFC value type and value), an element carries only its own
    (`get_psets(..., should_inherit=False)`), so nothing a type or a template default only implies is written."""
    problems = []
    properties = snapshot.get("properties", {})
    types = {kind for collection in TYPE_COLLECTIONS for kind in snapshot.get(collection, {})}
    found = type_property_rows(model)
    for kind in sorted(types | set(found)):
        wanted = {name: {prop: written_value(value) for prop, value in rows.items()} for name, rows in properties.get(kind, {}).items()} if kind in types else {}
        if not same_value(found.get(kind, {}), wanted):
            problems.append("%s: type property sets %s in the file, %s in the snapshot" % (kind, found.get(kind, {}), wanted))
    own = {}
    for element in model.by_type("IfcElement"):
        if element.Tag and ":" not in element.Tag:
            for name, rows in ifcopenshell.util.element.get_psets(element, psets_only=True, should_inherit=False).items():
                if not name.startswith("Semio_"):
                    own.setdefault(element.Tag, {}).setdefault(name, {}).update({key: value for key, value in rows.items() if key != "id"})
    for tag in sorted((set(own) | {tag for tag in properties if tag not in types}) & {element.Tag for element in model.by_type("IfcElement")}):
        wanted = {name: {prop: written_value(value)[1] for prop, value in rows.items()} for name, rows in properties.get(tag, {}).items()}
        if not same_value(own.get(tag, {}), wanted):
            problems.append("%s: own property sets %s in the file, %s in the snapshot" % (tag, own.get(tag, {}), wanted))
    return problems


EMPTY_COLLECTIONS = [
    "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces", "openings", "ceilings", "ramps", "wall_sweeps", "properties", "classifications",
    "wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types",
    "components", "component_overrides", "mep_elements", "families", "family_parameters", "family_solids",
]


def quantities_of(element, name):
    """🧮️ One base-quantity set of an element, or an empty dict."""
    return ifcopenshell.util.element.get_psets(element, qtos_only=True).get(name, {})


def close(left, right, relative=1e-9):
    """⚖️ Equality within a relative tolerance (absolute below one)."""
    return abs(left - right) <= relative * max(abs(right), 1.0)


ENTITIES = {
    "Site": "IfcSite", "Building": "IfcBuilding", "Storey": "IfcBuildingStorey", "Wall": "IfcWall", "CurtainWall": "IfcCurtainWall", "Column": "IfcColumn", "Beam": "IfcBeam", "Slab": "IfcSlab",
    "Ceiling": "IfcCovering", "Roof": "IfcRoof", "Window": "IfcWindow", "Door": "IfcDoor", "Void": "IfcOpeningElement", "Stair": "IfcStair", "Ramp": "IfcRamp", "Railing": "IfcRailing",
    "Space": "IfcSpace", "Zone": "IfcZone", "WallType": "IfcWallType", "SlabType": "IfcSlabType", "CeilingType": "IfcCoveringType", "RoofType": "IfcRoofType", "ColumnType": "IfcColumnType",
    "BeamType": "IfcBeamType", "WindowType": "IfcWindowType", "DoorType": "IfcDoorType",
}


MEASURES = {"Text": "IfcLabel", "Real": "IfcReal", "Integer": "IfcInteger", "Boolean": "IfcBoolean", "Length": "IfcLengthMeasure", "Area": "IfcAreaMeasure", "Volume": "IfcVolumeMeasure", "Angle": "IfcPlaneAngleMeasure"}


DEFINITION_KEYS = ["description", "unit", "required", "default", "minimum", "maximum"]


LIBRARY_COUNTED = ["IfcProject", "IfcProjectLibrary", "IfcRelDeclares", "IfcPropertySetTemplate", "IfcSimplePropertyTemplate", "IfcPropertyEnumeration", "IfcClassification", "IfcClassificationReference", "IfcRelAssociatesClassification"]


def unescape(text):
    """🔤️ Reverses the percent-escaping of `%`, `;`, `=` and line breaks in a definition list."""
    for code, plain in (("%3B", ";"), ("%3D", "="), ("%0D", chr(13)), ("%0A", chr(10)), ("%25", "%")):
        text = text.replace(code, plain)
    return text


def definition_fields(description):
    """🧾️ The `key=value;` list of a simple property template description as a dict, in order."""
    fields = {}
    for part in (description or "").split(";"):
        if part:
            key, _, value = part.partition("=")
            fields[key] = unescape(value)
    return fields


def library_table(model):
    """📚️ The oracle table of an IFC4 library file: counts per class, every property set template with its simple property templates in order (type, measure type, description, enumerated values), every
    classification system by `name|edition` with its references sorted by `Sort` (code, title, parent code), the declarations and the classification associations."""
    counts = {name: len(model.by_type(name, include_subtypes=False)) for name in LIBRARY_COUNTED}
    templates = {}
    for template in model.by_type("IfcPropertySetTemplate"):
        rows = []
        for row in template.HasPropertyTemplates:
            values = [[value.is_a().upper(), value.wrappedValue] for value in row.Enumerators.EnumerationValues] if row.Enumerators else []
            rows.append({"name": row.Name, "type": row.TemplateType, "measure": row.PrimaryMeasureType, "description": row.Description or "", "values": values})
        templates[template.Name] = {"type": template.TemplateType, "applicable": template.ApplicableEntity or "", "properties": rows}
    systems = {}
    for system in model.by_type("IfcClassification"):
        rows = []
        for reference in model.by_type("IfcClassificationReference"):
            if ifcopenshell.util.classification.get_classification(reference) == system:
                above = reference.ReferencedSource
                rows.append((reference.Sort or "", item_reference(reference), reference.Name or "", item_reference(above) if above.is_a("IfcClassificationReference") else ""))
        systems[system_key(system)] = {"source": system.Source or "", "entries": [{"code": code, "title": title, "parent": parent, "sort": sort} for sort, code, title, parent in sorted(rows)]}
    named = lambda entity: "%s:%s" % (entity.is_a().upper(), entity.Name or "")
    declares = sorted("%s>%s" % (named(relation.RelatingContext), ",".join(sorted(named(entity) for entity in relation.RelatedDefinitions))) for relation in model.by_type("IfcRelDeclares"))
    associated = sorted("%s>%s" % (named(entity), system_key(relation.RelatingClassification)) for relation in model.by_type("IfcRelAssociatesClassification") if relation.RelatingClassification.is_a("IfcClassification") for entity in relation.RelatedObjects)
    return {"schema": model.schema, "counts": counts, "templates": dict(sorted(templates.items())), "systems": dict(sorted(systems.items())), "declares": declares, "associated": associated}


def template_kind(targets):
    """🧭️ `IfcPropertySetTemplateTypeEnum` of a template: type kinds only are type driven, element kinds only occurrence driven, both type driven with override, none not defined."""
    types = [target for target in targets if target.endswith("Type")]
    if not targets:
        return "NOTDEFINED"
    return "PSET_TYPEDRIVENONLY" if len(types) == len(targets) else "PSET_OCCURRENCEDRIVEN" if not types else "PSET_TYPEDRIVENOVERRIDE"


def format_number(value):
    """🔢️ A bound as the export prints it: nanometre-rounded, no trailing zeros."""
    return ("%.9f" % value).rstrip("0").rstrip(".") if value != int(value) else str(int(value))


def library_problems(model, snapshot, table):
    """📚️ The IFC4 library against the snapshot: the schema is IFC4, a project declares a project library that declares one property set template per template of the snapshot (type driven, occurrence driven or both,
    applicable to the IFC entities of its targets), whose simple property templates carry the measure type, the enumeration of the allowed values and the required flag, unit, default, range and help text of
    the definition in their description; every classification system is an `IfcClassification` the library is associated with, and its references chain to their parent references in table order."""
    problems = []
    if table["schema"] != "IFC4":
        problems.append("schema is %s, not IFC4" % table["schema"])
    templates = {template["name"]: template for template in snapshot.get("property_templates", {}).values()}
    if set(table["templates"]) != set(templates):
        problems.append("templates %s in the file, %s in the snapshot" % (sorted(table["templates"]), sorted(templates)))
    for name, template in templates.items():
        row = table["templates"].get(name)
        if row is None:
            continue
        if (row["type"], row["applicable"]) != (template_kind(template["applies_to"]), ",".join(ENTITIES[target] for target in template["applies_to"])):
            problems.append("%s: template type %s on %s, expected %s on %s" % (name, row["type"], row["applicable"], template_kind(template["applies_to"]), ",".join(ENTITIES[t] for t in template["applies_to"])))
        if [prop["name"] for prop in row["properties"]] != [definition["name"] for definition in template["properties"]]:
            problems.append("%s: properties %s differ from %s" % (name, [prop["name"] for prop in row["properties"]], [definition["name"] for definition in template["properties"]]))
            continue
        for prop, definition in zip(row["properties"], template["properties"]):
            fields = definition_fields(prop["description"])
            wanted = {"required": str(definition["required"]).lower()}
            for key, field in (("description", "description"), ("unit", "unit")):
                if definition.get(field) is not None:
                    wanted[key] = definition[field]
            if definition.get("default_value") is not None:
                value = written_value(definition["default_value"])[1]
                wanted["default"] = str(value).lower() if isinstance(value, bool) else format_number(value) if isinstance(value, (int, float)) else value
            for key in ("minimum", "maximum"):
                if definition.get(key) is not None:
                    wanted[key] = format_number(definition[key])
            kind = "P_ENUMERATEDVALUE" if definition["allowed"] else "P_SINGLEVALUE"
            if fields != wanted or list(fields) != [key for key in DEFINITION_KEYS if key in wanted] or prop["type"] != kind or prop["measure"] != MEASURES[definition["kind"]] or not same_value(prop["values"], [written_value(value) for value in definition["allowed"]]):
                problems.append("%s.%s: %s differs from the definition %s" % (name, definition["name"], prop, definition))
    for system in snapshot.get("classification_systems", {}).values():
        key = "%s|%s" % (system["name"], system.get("edition", ""))
        row = table["systems"].get(key)
        wanted = [{"code": entry["code"], "title": entry["title"], "parent": entry.get("parent", ""), "sort": "%06d" % index} for index, entry in enumerate(system["entries"])]
        if row is None or row["entries"] != wanted or row["source"] != (system.get("source") or ""):
            problems.append("%s: %s in the file, %s in the snapshot" % (key, row, wanted))
        if not any(entry.endswith(">" + key) for entry in table["associated"]):
            problems.append("%s: the library is not associated with the classification" % key)
    library = [entry for entry in table["declares"] if entry.startswith("IFCPROJECT:")]
    if len(library) != 1 or not library[0].split(">")[1].startswith("IFCPROJECTLIBRARY:"):
        problems.append("the project declares %s, not one project library" % library)
    return problems


COUNTED = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor", "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial", "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcRoofType", "IfcColumnType",
    "IfcBeamType", "IfcWindowType", "IfcDoorType", "IfcTriangulatedFaceSet", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight", "IfcRelConnectsElements",
    "IfcProjectLibrary", "IfcPropertySetTemplate", "IfcRelDeclares", "IfcRelDefinesByTemplate",
]
"""📊️ The classes whose instances are counted (the subject counts the same list)."""

FIXTURES = "ifc4"
CASES = {
    "house": ["ifc", "house", "snapshot"],
    "psets": ["ifc", "psets", "snapshot"],
    "ceilings": ["ifc", "ceilings", "snapshot"],
    "notated": ["inferences", "annotation-layout", "room", "snapshot"],
    "ramps": ["inferences", "ramp-runs", "ramps", "snapshot"],
    "wall-depth": ["inferences", "wall-depth", "attic", "snapshot"],
}
EXAMPLES = {"example-house": ["assets", "house", "snapshot.json"], "example-office": ["assets", "office", "snapshot.json"]}
"""🏡️ The shipped examples: their snapshot lies in the assets next to the fixtures (the harness reads fixtures only, so these cases run standalone and in the Rust drift test)."""
"""🗂️ The committed snapshot of each case below the fixture root, by the names of its folders without their emoji."""

MEASURE_FOLDER = "\U0001F52C️measure"
JSON_FILE = "\U0001F523️.json"
"""🔬️ Where the measured table of a case is written."""

DOOR_OPERATION = {("Single", "Left"): "SINGLE_SWING_LEFT", ("Single", "Right"): "SINGLE_SWING_RIGHT", ("Double", "Left"): "DOUBLE_DOOR_SINGLE_SWING", ("Double", "Right"): "DOUBLE_DOOR_SINGLE_SWING"}
WINDOW_PARTITIONING = {1: "SINGLE_PANEL", 2: "DOUBLE_PANEL_VERTICAL", 3: "TRIPLE_PANEL_VERTICAL"}
"""🚪️ The operation of a door type and the partitioning of a window type as IFC4 names them."""


def plain(name):
    """🔤️ A folder or file name without its emoji."""
    return "".join(char for char in name if ord(char) < 128)


def child(parent, name):
    """🔎️ The entry of `parent` whose name is `name` once the emoji are removed."""
    for entry in Path(parent).iterdir():
        if plain(entry.name) == name:
            return entry
    raise FileNotFoundError("%s has no entry named %s" % (parent, name))


def snapshot_of(root, case):
    """📸️ The committed snapshot of a case."""
    folder = Path(root) if case in CASES else Path(root).parent
    for name in CASES.get(case) or EXAMPLES[case]:
        folder = child(folder, name)
    return json.loads((folder if folder.is_file() else child(folder, ".json")).read_text(encoding="utf-8"))


def case_folder(root, case):
    """📂️ The folder of the IFC4 fixture of a case."""
    return child(child(root, FIXTURES), case)


def classification_rows(model):
    """🗂️ The classification tables read with `ifcopenshell.util.classification`: every system by `name|edition` with its source and its references in `Sort` order (`code|title|parent`, the parent being the
    reference it hangs below), and for every element or type (by its model id) the sorted `name|edition|code` cells it carries itself."""
    systems = {system_key(system): {"source": system.Source or "", "entries": []} for system in model.by_type("IfcClassification")}
    rows = []
    for reference in model.by_type("IfcClassificationReference"):
        system = ifcopenshell.util.classification.get_classification(reference)
        if system is not None:
            above = reference.ReferencedSource
            parent = item_reference(above) if above is not None and above.is_a("IfcClassificationReference") else ""
            rows.append((reference.Sort or "", system_key(system), "%s|%s|%s" % (item_reference(reference), reference.Name or "", parent)))
    for _, key, entry in sorted(rows, key=lambda row: row[0]):
        systems[key]["entries"].append(entry)
    attached = {}
    for element in model.by_type("IfcRoot"):
        references = ifcopenshell.util.classification.get_references(element, should_inherit=False)
        holder = identity(element) if references else None
        if holder:
            attached.setdefault(holder, []).extend("%s|%s" % (system_key(ifcopenshell.util.classification.get_classification(reference)), item_reference(reference)) for reference in references)
    return {"systems": dict(sorted(systems.items())), "attached": {holder: sorted(cells) for holder, cells in sorted(attached.items())}}


def measure(model, snapshot):
    """🧊️ The oracle table: counts per class, containment per storey, the kernel volume of every measurable element, the annotations, the classification tables and the type property sets."""
    kernel = settings()
    counts = {name: len(model.by_type(name, include_subtypes=False)) for name in COUNTED}
    containment = {}
    for storey in model.by_type("IfcBuildingStorey"):
        contained = []
        for relation in model.by_type("IfcRelContainedInSpatialStructure"):
            if relation.RelatingStructure == storey:
                contained += [identity(element) for element in relation.RelatedElements]
        containment[identity(storey)] = sorted(contained)
    volumes = {}
    for name in MEASURED:
        for element in model.by_type(name, include_subtypes=False):
            if element.Tag and ":" not in element.Tag and straight(element, snapshot):
                volume = kernel_volume(kernel, element)
                if volume is not None:
                    volumes[element.Tag] = round(volume, 12)
    return {"schema": model.schema, "counts": counts, "containment": containment, "volumes": volumes, "annotations": annotation_rows(model), "classifications": classification_rows(model), "type_properties": type_property_rows(model)}


def operation_problems(model, snapshot):
    """🚪️ Every door type and door carries the operation of its snapshot record, every window type the partitioning of its panes, and every occurrence repeats its type."""
    problems = []
    for kind in model.by_type("IfcDoorType"):
        record = snapshot["door_types"].get(kind.Tag)
        if record is None or kind.OperationType != DOOR_OPERATION[(record["leaves"], record["swing"])] or kind.PredefinedType != "DOOR":
            problems.append("%s: door type %s/%s, snapshot %s" % (kind.Tag, kind.PredefinedType, kind.OperationType, record and (record["leaves"], record["swing"])))
    for kind in model.by_type("IfcWindowType"):
        record = snapshot["window_types"].get(kind.Tag)
        if record is None or kind.PartitioningType != WINDOW_PARTITIONING.get(record["panes"], "NOTDEFINED") or kind.PredefinedType != "WINDOW":
            problems.append("%s: window type %s/%s, snapshot %s" % (kind.Tag, kind.PredefinedType, kind.PartitioningType, record and record["panes"]))
    for door in model.by_type("IfcDoor"):
        kind = ifcopenshell.util.element.get_type(door)
        if kind is not None and (door.OperationType != kind.OperationType or door.PredefinedType != "DOOR"):
            problems.append("%s: the door says %s, its type %s" % (door.Tag, door.OperationType, kind.OperationType))
    for window in model.by_type("IfcWindow"):
        kind = ifcopenshell.util.element.get_type(window)
        if kind is not None and (window.PartitioningType != kind.PartitioningType or window.PredefinedType != "WINDOW"):
            problems.append("%s: the window says %s, its type %s" % (window.Tag, window.PartitioningType, kind.PartitioningType))
    return problems


def schema_problems(model):
    """🔖️ The file is IFC4 and uses none of the IFC 2x3 constructs the IFC4 schema replaced."""
    problems = []
    if model.schema != "IFC4":
        problems.append("schema is %s, not IFC4" % model.schema)
    if model.by_type("IfcOwnerHistory"):
        problems.append("an IFC4 file written without dates carries no owner history")
    if model.by_type("IfcFacetedBrep"):
        problems.append("IFC4 bodies are triangulated face sets, not faceted breps")
    for relation in model.by_type("IfcRelDefinesByTemplate"):
        for definition in relation.RelatedPropertySets:
            if definition.Name != relation.RelatingTemplate.Name:
                problems.append("a property set %s is related to the template %s" % (definition.Name, relation.RelatingTemplate.Name))
    return problems


def count_problems(model, snapshot, table):
    """🔢️ The counts of the file against the collections of the snapshot."""
    problems = []
    counts = table["counts"]
    collections = {"IfcColumn": "columns", "IfcBeam": "beams", "IfcSpace": "spaces", "IfcCurtainWall": "curtain_walls", "IfcStair": "stairs", "IfcOpeningElement": "openings", "IfcRoof": "roofs", "IfcCovering": "ceilings"}
    hosts = set(snapshot["walls"])
    for cls, key in collections.items():
        wanted = len([row for row in snapshot.get(key, {}).values() if row["host"] in hosts]) if key == "openings" else len(snapshot.get(key, {}))
        if counts[cls] != wanted:
            problems.append("%s: %d in the file, %d in the snapshot" % (cls, counts[cls], wanted))
    if counts["IfcWall"] + counts["IfcWallStandardCase"] != len(snapshot["walls"]):
        problems.append("walls: %d in the file, %d in the snapshot" % (counts["IfcWall"] + counts["IfcWallStandardCase"], len(snapshot["walls"])))
    fillings = [opening for opening in snapshot["openings"].values() if "Void" not in opening["kind"]]
    if counts["IfcWindow"] + counts["IfcDoor"] < len(fillings):
        problems.append("fillings: %d in the file, %d in the snapshot" % (counts["IfcWindow"] + counts["IfcDoor"], len(fillings)))
    if counts["IfcRailing"] < len(snapshot.get("railings", {})):
        problems.append("railings: %d in the file, %d in the snapshot" % (counts["IfcRailing"], len(snapshot.get("railings", {}))))
    types = {"IfcWallType": "wall_types", "IfcSlabType": "slab_types", "IfcRoofType": "roof_types", "IfcColumnType": "column_types", "IfcBeamType": "beam_types", "IfcWindowType": "window_types", "IfcDoorType": "door_types", "IfcCoveringType": "ceiling_types"}
    for cls, key in types.items():
        if counts[cls] != len(snapshot.get(key, {})):
            problems.append("%s: %d in the file, %d in the snapshot" % (cls, counts[cls], len(snapshot.get(key, {}))))
    return problems


WRITTEN_TOLERANCE = 1e-8
"""⚖️ Relative tolerance between the kernel volume and the written quantity: coordinates and directions are written to a nanometre, which moves the volume of a slanted or meshed body by a few parts in 1e9."""


def volume_problems(model, table):
    """🧮️ The kernel volume of every measurable element equals the volume the file writes in its base quantities."""
    problems = []
    by_tag = {element.Tag: element for name in MEASURED for element in model.by_type(name, include_subtypes=False)}
    for tag, volume in table["volumes"].items():
        quantity = written_volume(by_tag[tag])
        if quantity is None:
            problems.append("%s has no written volume" % tag)
        elif abs(quantity - volume) > WRITTEN_TOLERANCE * max(abs(volume), 1.0):
            problems.append("%s: kernel %.12g, written %.12g" % (tag, volume, quantity))
    return problems


def attached_problems(snapshot, table):
    """🗂️ Every holder carries exactly the (system, code) cells of its record."""
    problems = []
    expected = {holder: sorted("%s|%s" % pair for pair in keyed(snapshot, assigned).items()) for holder, assigned in snapshot.get("classifications", {}).items() if assigned}
    found = table["classifications"]["attached"]
    for holder in sorted(set(expected) | set(found)):
        if expected.get(holder, []) != found.get(holder, []):
            problems.append("%s: classifications %s in the file, %s in the snapshot" % (holder, found.get(holder, []), expected.get(holder, [])))
    return problems


def audit(model, snapshot, table):
    """⚖️ Every place where the file leaves the snapshot, the schema or the kernel."""
    snapshot = {**{key: {} for key in EMPTY_COLLECTIONS}, **snapshot}
    problems = schema_problems(model) + count_problems(model, snapshot, table) + volume_problems(model, table) + operation_problems(model, snapshot) + placement_problems(model) + attached_problems(snapshot, table) + property_problems(model, snapshot)
    if snapshot.get("property_templates") or snapshot.get("classification_systems"):
        problems += library_problems(model, snapshot, library_table(model))
    log = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, log, express_rules=True)
    problems += ["validate: %s" % entry["message"] for entry in log.statements if str(entry.get("level")).lower() == "error"]
    return problems


# endregion 🔖️Measurement


# region 🔖️Handlers
def export_handler(ctx):
    """🧊️ Oracle answer: the kernel's table of the committed IFC4 file."""
    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    ifc_path = next(uri for uri in uris if uri.endswith(".ifc"))
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "snapshot" in uri)).decode("utf-8"))
    model = ifcopenshell.file.from_string(ctx.input_bytes(ifc_path).decode("utf-8"))
    table = measure(model, snapshot)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    built = Adapter("python")
    for scenario in ("export-ifc4-house", "export-ifc4-psets", "export-ifc4-ceilings", "export-ifc4-notated", "export-ifc4-ramps", "export-ifc4-wall-depth", "roundtrip-ifc4-house", "roundtrip-ifc4-psets", "stepped-ifc4-house"):
        built = built.oracle(scenario, export_handler)
    return built


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed files against the committed tables; `write` rewrites the tables from the files."""
    command, root = arguments[0], arguments[1]
    problems = []
    for case in arguments[2:] or [*CASES, *EXAMPLES]:
        folder = case_folder(root, case)
        snapshot = snapshot_of(root, case)
        model = ifcopenshell.open(str(child(folder, case + ".ifc")))
        table = measure(model, snapshot)
        problems += ["%s: %s" % (case, problem) for problem in audit(model, snapshot, table)]
        path = folder / MEASURE_FOLDER / JSON_FILE
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + NEWLINE, encoding="utf-8", newline=NEWLINE)
            print("wrote %s: %d classes, %d measured elements" % (case, len(table["counts"]), len(table["volumes"])))
        elif json.loads(path.read_text(encoding="utf-8")) != table:
            problems.append("%s: the committed table differs from the measurement of the committed file" % case)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
