"""🏷️ The classification and user property set audits: the classification parents, the type and element reference rows, the effective references and the written property values."""

from __future__ import annotations

# region 🔖️Imports
import ifcopenshell.util.classification
import ifcopenshell.util.element

from export_bim_ifc_kernel import TOLERANCE, identity
# endregion 🔖️Imports


# region 🔖️Measurement
PARENTS_SET = "Semio_ClassificationParents"
"""🔑️ The property set of the project that carries the parent column of the classification tables (IFC 2x3 has no slot for it)."""

TYPE_NAMES = {"Text": "IFCLABEL", "Real": "IFCREAL", "Integer": "IFCINTEGER", "Boolean": "IFCBOOLEAN", "Length": "IFCLENGTHMEASURE", "Area": "IFCAREAMEASURE", "Volume": "IFCVOLUMEMEASURE", "Angle": "IFCPLANEANGLEMEASURE"}
"""🏷️ The IFC measure type each kind of model property is written as."""

TYPE_COLLECTIONS = ["wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types"]
"""🏛️ The type records of the snapshot: their properties and classifications travel on the written type objects."""


def system_key(system):
    """🔑️ `name|edition` of an `IfcClassification`."""
    return "%s|%s" % (system.Name or "", system.Edition or "")


def item_reference(reference):
    """🔖️ The code of an `IfcClassificationReference` (`ItemReference` in IFC 2x3, `Identification` in IFC4)."""
    return getattr(reference, "ItemReference", None) or getattr(reference, "Identification", None) or ""


def parent_rows(model):
    """🌳️ The `Semio_ClassificationParents` set of the project: `name|edition|code` -> parent code."""
    rows = {}
    for project in model.by_type("IfcProject"):
        for relation in project.IsDefinedBy or []:
            definition = relation.RelatingPropertyDefinition if relation.is_a("IfcRelDefinesByProperties") else None
            if definition is not None and definition.is_a("IfcPropertySet") and definition.Name == PARENTS_SET:
                rows.update({row.Name: row.NominalValue.wrappedValue for row in definition.HasProperties})
    return rows


def classification_rows(model):
    """🗂️ The classification tables read with `ifcopenshell.util.classification`: every system by `name|edition` with its source and its references in file order (`code|title|parent`),
    and for every element or type (by its model id) the sorted `name|edition|code` cells it carries itself."""
    parents = parent_rows(model)
    systems = {system_key(system): {"source": system.Source or "", "entries": []} for system in model.by_type("IfcClassification")}
    for reference in model.by_type("IfcClassificationReference"):
        system = ifcopenshell.util.classification.get_classification(reference)
        if system is not None:
            key, code = system_key(system), item_reference(reference)
            systems[key]["entries"].append("%s|%s|%s" % (code, reference.Name or "", parents.get("%s|%s" % (key, code), "")))
    attached = {}
    for element in model.by_type("IfcRoot"):
        references = ifcopenshell.util.classification.get_references(element, should_inherit=False)
        holder = identity(element) if references else None
        if holder:
            attached.setdefault(holder, []).extend("%s|%s" % (system_key(ifcopenshell.util.classification.get_classification(reference)), item_reference(reference)) for reference in references)
    return {"systems": dict(sorted(systems.items())), "attached": {holder: sorted(cells) for holder, cells in sorted(attached.items())}}


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


INHERITING = {"walls": "wall_type", "slabs": "slab_type", "columns": "column_type", "beams": "beam_type", "ceilings": "ceiling_type"}
"""🏛️ The element collections the export types with `IfcRelDefinesByType`, and the field that names their type."""


def keyed(snapshot, assigned):
    """🔑️ The codes of one holder by `name|edition` of their system."""
    systems = snapshot.get("classification_systems", {})
    return {"%s|%s" % (systems[system]["name"], systems[system].get("edition", "")): code for system, code in assigned.items() if system in systems}


def classification_problems(model, snapshot, table):
    """🗂️ The classifications of the file against the snapshot: every element and type carries exactly its own (system, code) cells, each reference hangs directly below its `IfcClassification`, every table row of
    every system is in the file in table order with its parent, and `ifcopenshell.util.classification.get_references` returns, for an element with a type, the type's cells overridden per system by its own."""
    problems = []
    systems = snapshot.get("classification_systems", {})
    rows = table["classifications"]
    expected = {holder: sorted("%s|%s" % pair for pair in keyed(snapshot, assigned).items()) for holder, assigned in snapshot.get("classifications", {}).items() if assigned}
    for holder in sorted(set(expected) | set(rows["attached"])):
        if expected.get(holder, []) != rows["attached"].get(holder, []):
            problems.append("%s: classifications %s in the file, %s in the snapshot" % (holder, rows["attached"].get(holder, []), expected.get(holder, [])))
    if len(rows["systems"]) != len(systems):
        problems.append("%d classifications in the file, %d systems in the snapshot" % (len(rows["systems"]), len(systems)))
    for system in systems.values():
        key = "%s|%s" % (system["name"], system.get("edition", ""))
        row = rows["systems"].get(key)
        wanted = ["%s|%s|%s" % (entry["code"], entry["title"], entry.get("parent", "")) for entry in system["entries"]]
        if row is None:
            problems.append("%s: no IfcClassification in the file" % key)
        elif row["entries"] != wanted:
            problems.append("%s: the references %s differ from the table %s" % (key, row["entries"], wanted))
        elif row["source"] != (system.get("source") or ""):
            problems.append("%s: source %r in the file, %r in the snapshot" % (key, row["source"], system.get("source")))
    for reference in model.by_type("IfcClassificationReference"):
        if reference.ReferencedSource is None or not reference.ReferencedSource.is_a("IfcClassification"):
            problems.append("the reference %s does not hang below an IfcClassification" % item_reference(reference))
    by_tag = {element.Tag: element for element in model.by_type("IfcElement") if element.Tag}
    classes = snapshot.get("classifications", {})
    for collection, field in INHERITING.items():
        for tag, row in snapshot.get(collection, {}).items():
            element = by_tag.get(tag)
            if element is not None:
                wanted = {**keyed(snapshot, classes.get(row[field], {})), **keyed(snapshot, classes.get(tag, {}))}
                found = {system_key(ifcopenshell.util.classification.get_classification(reference)): item_reference(reference) for reference in ifcopenshell.util.classification.get_references(element)}
                if wanted != found:
                    problems.append("%s: effective classifications %s in the file, %s from its type and itself" % (tag, found, wanted))
    return problems


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


# endregion 🔖️Measurement
