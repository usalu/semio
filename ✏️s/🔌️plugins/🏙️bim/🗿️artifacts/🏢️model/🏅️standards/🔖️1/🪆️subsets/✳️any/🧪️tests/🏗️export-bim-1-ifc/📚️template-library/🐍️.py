"""📚️ The IFC4 template library table and its audit: the property set templates, the classification chain and the declarations."""

from __future__ import annotations

# region 🔖️Imports
import ifcopenshell.util.classification

from export_bim_ifc_classification import system_key, item_reference, written_value, same_value
# endregion 🔖️Imports


# region 🔖️Measurement
ENTITIES = {
    "Site": "IfcSite", "Building": "IfcBuilding", "Storey": "IfcBuildingStorey", "Wall": "IfcWall", "CurtainWall": "IfcCurtainWall", "Column": "IfcColumn", "Beam": "IfcBeam", "Slab": "IfcSlab",
    "Ceiling": "IfcCovering", "Roof": "IfcRoof", "Window": "IfcWindow", "Door": "IfcDoor", "Void": "IfcOpeningElement", "Stair": "IfcStair", "Ramp": "IfcRamp", "Railing": "IfcRailing",
    "Space": "IfcSpace", "Zone": "IfcZone", "WallType": "IfcWallType", "SlabType": "IfcSlabType", "CeilingType": "IfcCoveringType", "RoofType": "IfcRoofType", "ColumnType": "IfcColumnType",
    "BeamType": "IfcBeamType", "WindowType": "IfcWindowType", "DoorType": "IfcDoorType",
}
"""🎯️ The IFC4 entity each template target of the model names (`ApplicableEntity` of an `IfcPropertySetTemplate`)."""

MEASURES = {"Text": "IfcLabel", "Real": "IfcReal", "Integer": "IfcInteger", "Boolean": "IfcBoolean", "Length": "IfcLengthMeasure", "Area": "IfcAreaMeasure", "Volume": "IfcVolumeMeasure", "Angle": "IfcPlaneAngleMeasure"}
"""📏️ The IFC4 measure type each property kind of the model is declared as (`PrimaryMeasureType`)."""

DEFINITION_KEYS = ["description", "unit", "required", "default", "minimum", "maximum"]
"""🔑️ The keys of a definition list, in the order the export writes them."""

LIBRARY_COUNTED = ["IfcProject", "IfcProjectLibrary", "IfcRelDeclares", "IfcPropertySetTemplate", "IfcSimplePropertyTemplate", "IfcPropertyEnumeration", "IfcClassification", "IfcClassificationReference", "IfcRelAssociatesClassification"]
"""📊️ The classes the library table counts."""


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
    associated = sorted("%s>%s" % (named(entity), system_key(relation.RelatingClassification)) for relation in model.by_type("IfcRelAssociatesClassification") for entity in relation.RelatedObjects)
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


# endregion 🔖️Measurement
