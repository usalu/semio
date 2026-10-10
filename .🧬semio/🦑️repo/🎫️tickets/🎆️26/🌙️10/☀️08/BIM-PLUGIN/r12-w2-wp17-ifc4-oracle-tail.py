

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
    for cls, key in collections.items():
        if counts[cls] != len(snapshot.get(key, {})):
            problems.append("%s: %d in the file, %d in the snapshot" % (cls, counts[cls], len(snapshot.get(key, {}))))
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


def volume_problems(model, table):
    """🧮️ The kernel volume of every measurable element equals the volume the file writes in its base quantities."""
    problems = []
    by_tag = {element.Tag: element for name in MEASURED for element in model.by_type(name, include_subtypes=False)}
    for tag, volume in table["volumes"].items():
        quantity = written_volume(by_tag[tag])
        if quantity is None:
            problems.append("%s has no written volume" % tag)
        elif abs(quantity - volume) > TOLERANCE * max(abs(volume), 1.0):
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
