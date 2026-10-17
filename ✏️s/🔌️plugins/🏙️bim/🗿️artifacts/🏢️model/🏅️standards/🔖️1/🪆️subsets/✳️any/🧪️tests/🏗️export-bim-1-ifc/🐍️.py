#!/usr/bin/env python3
"""🏗️ Third-party ORACLE (IfcOpenShell) for the IFC 2x3 export of the BIM house.

IfcOpenShell 0.8.4.post1 has never seen this repository's writer. It opens the committed file
`🧫️fixtures/🏗️ifc/🏠️house/🏠️house.ifc` and `🧫️fixtures/🏗️ifc/🪧️notated/🪧️notated.ifc`, and:

* parses it, requires the schema `IFC2X3` and validates every attribute type and every EXPRESS WHERE rule (`ifcopenshell.validate`);
* counts the entities of every class the export writes;
* reads the spatial containment of every product (`ifcopenshell.util.element.get_container`);
* tessellates every straight wall, level slab, column and beam with its C++ geometry kernel (openings are subtracted
  by the kernel through `IfcRelVoidsElement`) and measures the volume of the triangulation;
* reads every `IfcAnnotation` (dimension, tag, text note, leader) of the notated room: its kind, its printed text, the curves and the sorted text literals of its
  `Annotation2D` representation and the `Total` of a dimension, and requires each total to equal the distance the shapely-adjudicated inference oracle committed;
* requires every window and door to lie inside the bounding box of the wall it fills;
* reads the attic of the wall-depth case: the `Semio_WallAttach`, `Semio_WallSweep` and `Semio_OpeningReveal` sets and the `IfcRelConnectsElements` of attached walls, the kernel volume and
  vertical extent of every tessellated wall against the shapely-adjudicated `wall-layout` table, the lengths, cross-sections and volumes of the runs of every sweep against its `sweeps` table, and the
  lateral range of the window of a reveal against its `reveals` table;
* reads every `IfcRamp` of the ramps case with its `IfcRampFlight` and `LANDING` slab parts (`ifcopenshell.util.element.get_parts`) and requires their counts, base quantities, ramp type, storey, railing and authored
  record to equal the committed, shapely-adjudicated `ramp-runs` table, and the kernel volumes of the parts to add up to the written gross volume;
* reads the frame case: the extrusion direction and depth of every leaning column against `(sin a cos d, sin a sin d, cos a)`, the kernel volume of its sheared prism against the cross-section area times the length
  along the lean, the `IfcTrimmedCurve` axis of every arc beam (radius, end points, sense) against the closed form of its bulge, the brep body and axis of arc, inclined and joined beams, the parts of every curtain
  wall (members, plates, one door or window per filled cell) against its panel overrides, and the authored records of columns, beams and curtain walls in `Semio_Authoring`;
* reads the base quantities back (`ifcopenshell.util.element.get_psets(..., qtos_only=True)`) and requires the kernel
  volume to equal the written `NetVolume`/`GrossVolume` within 1e-9;
* reads the classifications of the psets fixture with `ifcopenshell.util.classification` (`get_references`, `get_classification`): every element and type carries exactly the (system, code) pairs of its
  snapshot record (many systems per holder), every table row of every system is an `IfcClassificationReference` in table order whose parent comes from the project's `Semio_ClassificationParents`
  set, and the effective references of an element equal its type's overridden per system by its own;
* reads the user property sets of every type object (`HasPropertySets`, IFC value type and value) and of every element (`get_psets(..., should_inherit=False)`), and requires them to equal the snapshot, so
  nothing a type only implies is written onto its instances;
* opens the committed IFC4 template library `📚️library.ifc` of the psets fixture, validates it (attribute types and EXPRESS rules) and requires one `IfcPropertySetTemplate` per template (template type and
  `ApplicableEntity` from the targets, one `IfcSimplePropertyTemplate` per definition with its measure type, `IfcPropertyEnumeration` and the `key=value;` description), one `IfcClassification` per system
  with its references chained to their parents in table order, and the declarations of the project and the library.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🏗️ifc> [case...]   # rewrite the measured table from the committed file (of the named cases only)

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import os
import json
import sys
import ifcopenshell.util.element
import ifcopenshell.validate
import importlib.util
from pathlib import Path


def _load(alias, folder):
    """🧩️ Loads one child module of this oracle from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module

_load("export_bim_ifc_kernel", "📐️kernel-helpers")
_load("export_bim_ifc_classification", "🏷️classification-psets")
_load("export_bim_ifc_elements", "🧱️element-checks")
_load("export_bim_ifc_walldepth", "🧗️wall-depth")
_load("export_bim_ifc_ramps", "🛝️ramps")
_load("export_bim_ifc_components", "🪑️components")
_load("export_bim_ifc_library", "📚️template-library")

from export_bim_ifc_kernel import TOLERANCE, NEWLINE, identity, settings, kernel_volume
from export_bim_ifc_classification import classification_rows, type_property_rows, classification_problems, property_problems
from export_bim_ifc_elements import placement_problems, frame_problems, phase_problems, annotation_problems
from export_bim_ifc_walldepth import EMPTY_COLLECTIONS, wall_depth_problems
from export_bim_ifc_ramps import exported_ramps, ramp_problems
from export_bim_ifc_components import component_problems
from export_bim_ifc_library import library_table, library_problems
# endregion 🔖️Imports


# region 🔖️Measurement
COUNTED = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor",
    "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial",
    "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcBuildingElementProxyType", "IfcColumnType",
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight", "IfcRelConnectsElements",
    "IfcFurnishingElement", "IfcFurnitureType", "IfcFlowTerminal", "IfcFlowTerminalType", "IfcSanitaryTerminalType", "IfcLightFixtureType", "IfcBuildingElementProxy", "IfcFlowSegment", "IfcDuctSegmentType", "IfcPipeSegmentType",
    "IfcCableCarrierSegmentType", "IfcSystem", "IfcRelAssignsToGroup",
]
"""📊️ The classes whose instances are counted."""

MEASURED = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab", "IfcCovering", "IfcFurnishingElement", "IfcFlowTerminal", "IfcBuildingElementProxy", "IfcFlowSegment"]
"""📐️ The classes whose volume the kernel measures."""


PSETS = "\U0001F3F7️psets"
LIBRARY = "\U0001F4DA️library.ifc"
"""🏷️ The case of the classification and property set fixture and the file name of its IFC4 template library."""

DEPTH_CASE = "🧗️wall-depth"
"""🧗️ The case of the attached walls, wall sweeps and reveals, written from the attic of the wall-depth inference fixture."""

CASES = {"🏠️house": "🏠️house.ifc", "🪧️notated": "🪧️notated.ifc", "🔲️ceilings": "🔲️ceilings.ifc", "🛝️ramps": "🛝️ramps.ifc", PSETS: PSETS + ".ifc"}
SNAPSHOTS = {PSETS: ("🏗️ifc", PSETS, "📸️snapshot"), "🏠️house": ("🏗️ifc", "🏠️house", "📸️snapshot"),"🔲️ceilings": ("🏗️ifc", "🔲️ceilings", "📸️snapshot"), "🪧️notated": ("💡️inferences", "🪧️annotation-layout", "🏠️room", "📸️snapshot"), "🛝️ramps": ("💡️inferences", "🛝️ramp-runs", "🏞️ramps", "📸️snapshot")}
INFERRED = {"🪧️notated": ("annotations", "📏️annotations"), "🛝️ramps": ("ramp-runs", "🛝️ramp-runs")}
"""🧮️ The committed inference table a case is audited against: its key in `committed` and its folder next to the snapshot."""
FRAME_CASE = "🏗️frame"
CASES[FRAME_CASE] = FRAME_CASE + ".ifc"
SNAPSHOTS[FRAME_CASE] = ("🏗️ifc", FRAME_CASE, "📸️snapshot")
CASES[DEPTH_CASE] = DEPTH_CASE + ".ifc"
SNAPSHOTS[DEPTH_CASE] = ("💡️inferences", DEPTH_CASE, "🏠️attic", "📸️snapshot")
INFERRED[DEPTH_CASE] = ("wall-depth", DEPTH_CASE)
COMPONENT_CASE = "🪑️components"
CASES[COMPONENT_CASE] = COMPONENT_CASE + ".ifc"
SNAPSHOTS[COMPONENT_CASE] = ("🏗️ifc", COMPONENT_CASE, "📸️snapshot")


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
        return tag in snapshot["slabs"] and not snapshot["slabs"][tag].get("slope")
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


def measure(model, snapshot):
    """🧊️ The oracle table: counts per class, containment per storey, the kernel volume of every measurable element and the annotations."""
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


def audit(model, snapshot, table, committed=None):
    """⚖️ Every place where the file, the written quantities or the snapshot leave the kernel's table."""
    snapshot = {**{key: {} for key in EMPTY_COLLECTIONS}, **snapshot}
    problems = []
    if table["schema"] != "IFC2X3":
        problems.append("schema is %s, not IFC2X3" % table["schema"])
    committed = committed or {}
    collections = {
        "IfcBuildingStorey": "storeys", "IfcColumn": "columns", "IfcBeam": "beams", "IfcSpace": "spaces", "IfcCurtainWall": "curtain_walls", "IfcStair": "stairs",
        "IfcOpeningElement": "openings", "IfcRoof": "roofs",
    }
    sided = [ramp_id for ramp_id in exported_ramps(snapshot, committed.get("ramp-runs")) if snapshot["ramps"][ramp_id]["railing_left"] or snapshot["ramps"][ramp_id]["railing_right"]]
    if table["counts"]["IfcRailing"] != len(snapshot.get("railings", {})) + len(sided):
        problems.append("IfcRailing: %d in the file, %d in the snapshot and %d on the sides of ramps" % (table["counts"]["IfcRailing"], len(snapshot.get("railings", {})), len(sided)))
    collections["IfcCovering"] = "ceilings"
    for cls, key in collections.items():
        if table["counts"][cls] != len(snapshot.get(key, {})):
            problems.append("%s: %d in the file, %d in the snapshot" % (cls, table["counts"][cls], len(snapshot.get(key, {}))))
    if table["counts"]["IfcCoveringType"] != len(snapshot.get("ceiling_types", {})):
        problems.append("IfcCoveringType: %d in the file, %d in the snapshot" % (table["counts"]["IfcCoveringType"], len(snapshot.get("ceiling_types", {}))))
    walls = table["counts"]["IfcWall"] + table["counts"]["IfcWallStandardCase"]
    if walls != len(snapshot["walls"]):
        problems.append("walls: %d in the file, %d in the snapshot" % (walls, len(snapshot["walls"])))
    openings = [opening for opening in snapshot["openings"].values() if "Void" not in opening["kind"]]
    if table["counts"]["IfcWindow"] + table["counts"]["IfcDoor"] != len(openings):
        problems.append("fillings: %d in the file, %d in the snapshot" % (table["counts"]["IfcWindow"] + table["counts"]["IfcDoor"], len(openings)))
    for storey, tags in table["containment"].items():
        for tag in tags:
            if tag not in {**snapshot["walls"], **snapshot["columns"], **snapshot["beams"], **snapshot["slabs"], **snapshot.get("ceilings", {}), **snapshot["roofs"], **snapshot["stairs"], **snapshot["railings"], **snapshot.get("ramps", {}), **snapshot["curtain_walls"], **snapshot["openings"], **snapshot["wall_sweeps"], **snapshot["components"], **snapshot["mep_elements"]}:
                problems.append("%s contains the unknown element %s" % (storey, tag))
    by_tag = {element.Tag: element for name in MEASURED for element in model.by_type(name, include_subtypes=False)}
    for tag, volume in table["volumes"].items():
        quantity = written_volume(by_tag[tag])
        if quantity is None:
            problems.append("%s has no written volume" % tag)
        elif abs(quantity - volume) > TOLERANCE * max(abs(volume), 1.0):
            problems.append("%s: kernel %.12g, written %.12g" % (tag, volume, quantity))
    problems += placement_problems(model)
    problems += frame_problems(model, snapshot)
    problems += phase_problems(model, snapshot)
    problems += annotation_problems(model, snapshot, table, committed.get("annotations"))
    problems += classification_problems(model, snapshot, table)
    problems += property_problems(model, snapshot)
    problems += ramp_problems(model, snapshot, committed.get("ramp-runs"))
    problems += wall_depth_problems(model, snapshot, table, committed.get("wall-depth"))
    problems += component_problems(model, snapshot)
    return problems


def open_case(root, name):
    """📂️ The committed snapshot, the committed file, the committed inference table of the annotations (when the case has some) and the directory of the case."""
    case = Path(root) / name
    snapshot = json.loads(Path(root).parent.joinpath(*SNAPSHOTS[name], "🔣️.json").read_text(encoding="utf-8"))
    model = ifcopenshell.open(str(case / CASES[name]))
    committed = {}
    if name in INFERRED:
        key, slug = INFERRED[name]
        inferred = Path(root).parent.joinpath(*SNAPSHOTS[name][:-1], "💡️inference", slug, "🔣️.json")
        if inferred.exists():
            committed[key] = json.loads(inferred.read_text(encoding="utf-8"))
    return case, snapshot, model, committed


# endregion 🔖️Measurement
# region 🔖️Handlers
def export_handler(ctx):
    """🧊️ Oracle answer: the kernel's table of the committed file."""
    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    ifc_path = next(uri for uri in uris if uri.endswith(".ifc"))
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "📸️snapshot" in uri)).decode("utf-8"))
    model = ifcopenshell.file.from_string(ctx.input_bytes(ifc_path).decode("utf-8"))
    table = measure(model, snapshot)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def library_handler(ctx):
    """📚️ Oracle answer: the table of the IFC4 template library, read from the committed file."""
    from semio_repo_test import Outcome

    ifc_path = next(uri for uri in ctx.step_input_uris() if uri.endswith(".ifc"))
    table = library_table(ifcopenshell.file.from_string(ctx.input_bytes(ifc_path).decode("utf-8")))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return (
        Adapter("python")
        .oracle("export-ifc-house", export_handler)
        .oracle("export-ifc-frame", export_handler)
        .oracle("export-ifc-notated", export_handler)
        .oracle("export-ifc-ceilings", export_handler)
        .oracle("export-ifc-ramps", export_handler)
        .oracle("export-ifc-stepped", export_handler).oracle("export-ifc-wall-depth", export_handler)
        .oracle("export-ifc-psets", export_handler)
        .oracle("export-ifc-components", export_handler)
        .oracle("export-ifc-library", library_handler)
    )


# endregion 🔖️Handlers
# region 🔖️Standalone
def library_case(command, case, snapshot):
    """📚️ Opens the committed IFC4 library of a case, validates it (every attribute type and EXPRESS rule), audits it against the snapshot and writes or compares its table `📚️measure/🔣️.json`."""
    problems = []
    model = ifcopenshell.open(str(case / LIBRARY))
    table = library_table(model)
    problems += ["library: %s" % problem for problem in library_problems(model, snapshot, table)]
    log = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, log, express_rules=True)
    problems += ["library validate: %s" % entry["message"] for entry in log.statements if str(entry.get("level")).lower() == "error"]
    path = case / "\U0001F4DA️measure" / "\U0001F523️.json"
    if command == "write":
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + NEWLINE, encoding="utf-8", newline=NEWLINE)
        print("wrote the library table: %d templates, %d systems" % (len(table["templates"]), len(table["systems"])))
    elif json.loads(path.read_text(encoding="utf-8")) != table:
        problems.append("library: the committed table differs from the measurement of the committed file")
    return problems


def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], arguments[1]
    problems = []
    for name in arguments[2:] or CASES:
        case, snapshot, model, committed = open_case(root, name)
        table = measure(model, snapshot)
        problems += ["%s: %s" % (name, problem) for problem in audit(model, snapshot, table, committed)]
        log = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, log, express_rules=True)
        problems += ["%s validate: %s" % (name, entry["message"]) for entry in log.statements if str(entry.get("level")).lower() == "error"]
        path = case / "🔬️measure" / "🔣️.json"
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + NEWLINE, encoding="utf-8", newline=NEWLINE)
            print("wrote %s: %d classes, %d measured elements, %d annotations" % (path, len(table["counts"]), len(table["volumes"]), len(table["annotations"])))
        elif json.loads(path.read_text(encoding="utf-8")) != table:
            problems.append("%s: the committed table differs from the measurement of the committed file" % name)
        if name == PSETS:
            problems += library_case(command, case, snapshot)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))


# endregion 🔖️Standalone
