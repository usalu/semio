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
import copy
import importlib.util
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
TOLERANCE = 1e-9
NEWLINE = chr(10)
"""⚖️ Absolute tolerance between the kernel's volume, the written quantity and the subject's report."""

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


def representation_items(element):
    """🧩️ `[(identifier, item)]` of every item of every representation of an element."""
    if element.Representation is None:
        return []
    return [(shape.RepresentationIdentifier, item) for shape in element.Representation.Representations for item in shape.Items]


def frame_problems(model, snapshot):
    """🏗️ Leaning columns, arc, inclined and joined beams and curtain walls against closed forms of their records."""
    problems = []
    kernel = settings()
    columns = {element.Tag: element for element in model.by_type("IfcColumn")}
    for tag, column in snapshot["columns"].items():
        tilt = column.get("tilt")
        if not tilt or tag not in columns:
            continue
        element = columns[tag]
        solids = [item for identifier, item in representation_items(element) if item.is_a("IfcExtrudedAreaSolid")]
        if len(solids) != 1:
            problems.append("%s: a leaning column is one extruded area solid" % tag)
            continue
        direction = solids[0].ExtrudedDirection.DirectionRatios
        wanted = (math.sin(tilt["angle"]) * math.cos(tilt["direction"]), math.sin(tilt["angle"]) * math.sin(tilt["direction"]), math.cos(tilt["angle"]))
        if any(abs(found - expected) > 1e-8 for found, expected in zip(direction, wanted)):
            problems.append("%s: extrusion direction %s, expected %s" % (tag, tuple(direction), wanted))
        quantities = ifcopenshell.util.element.get_psets(element, qtos_only=True).get("Qto_ColumnBaseQuantities", {})
        volume = kernel_volume(kernel, element)
        if volume is not None and abs(volume - quantities["CrossSectionArea"] * quantities["Length"]) > TOLERANCE * max(volume, 1.0) and "Circle" not in snapshot["column_types"][column["column_type"]]["profile"]:
            problems.append("%s: kernel volume %.12g, cross section times length %.12g" % (tag, volume, quantities["CrossSectionArea"] * quantities["Length"]))
        if abs(solids[0].Depth * math.cos(tilt["angle"]) - quantities["Length"] * math.cos(tilt["angle"])) > 1e-8:
            problems.append("%s: depth %.9g is not the length %.9g along the lean" % (tag, solids[0].Depth, quantities["Length"]))
        record = authored(element).get("Column")
        if record is None or json.loads(record).get("tilt") != tilt:
            problems.append("%s: the authored column record does not carry the lean" % tag)
    beams = {element.Tag: element for element in model.by_type("IfcBeam")}
    for tag, beam in snapshot["beams"].items():
        element = beams.get(tag)
        if element is None:
            continue
        (kind, body), = beam["axis"].items()
        swept = [item for identifier, item in representation_items(element) if item.is_a("IfcExtrudedAreaSolid")]
        breps = [item for identifier, item in representation_items(element) if item.is_a("IfcFacetedBrep")]
        curves = [item for identifier, item in representation_items(element) if identifier == "Axis"]
        quantities = ifcopenshell.util.element.get_psets(element, qtos_only=True).get("Qto_BeamBaseQuantities", {})
        if kind == "Arc" or beam.get("end_top_offset") is not None or (quantities and abs(quantities.get("NetVolume", 0.0) - quantities.get("GrossVolume", 0.0)) > 1e-9):
            if swept or len(breps) != 1 or len(curves) != 1:
                problems.append("%s: an arc, inclined or joined beam is one brep with one axis curve" % tag)
                continue
            if record_of(element, "Beam") is None:
                problems.append("%s: the authored beam record is missing" % tag)
        else:
            if len(swept) != 1 or breps or curves:
                problems.append("%s: a plain beam is one extruded area solid" % tag)
        if kind == "Arc" and curves:
            curve = curves[0]
            sweep = 4.0 * math.atan(body["bulge"])
            chord = math.dist((body["start"]["x"], body["start"]["y"]), (body["end"]["x"], body["end"]["y"]))
            radius = chord / (2.0 * abs(math.sin(sweep / 2.0)))
            if not curve.is_a("IfcTrimmedCurve") or not curve.BasisCurve.is_a("IfcCircle") or abs(curve.BasisCurve.Radius - radius) > 1e-8:
                problems.append("%s: the axis is not a trimmed circle of radius %.9g" % (tag, radius))
            else:
                ends = [tuple(trim[0].Coordinates) for trim in (curve.Trim1, curve.Trim2)]
                if any(abs(found - expected) > 1e-8 for end, point in zip(ends, (body["start"], body["end"])) for found, expected in zip(end, (point["x"], point["y"]))):
                    problems.append("%s: the trimmed curve ends at %s" % (tag, ends))
                if bool(curve.SenseAgreement) != (body["bulge"] > 0):
                    problems.append("%s: the sense of the trimmed curve disagrees with the bulge" % tag)
            arc_length = abs(sweep) * radius
            if abs(quantities.get("Length", 0.0) - math.hypot(arc_length, (beam["top_offset"] if beam.get("end_top_offset") is None else beam["end_top_offset"]) - beam["top_offset"])) > 1e-8:
                problems.append("%s: the written length %.9g is not the arc length %.9g" % (tag, quantities.get("Length", 0.0), arc_length))
    for tag, wall in snapshot["curtain_walls"].items():
        element = next((found for found in model.by_type("IfcCurtainWall") if found.Tag == tag), None)
        if element is None:
            continue
        kinds = sorted(part.is_a() for part in ifcopenshell.util.element.get_parts(element))
        overrides = {key: row for key, row in snapshot.get("curtain_panel_overrides", {}).items() if row["curtain"] == tag}
        doors = sum(1 for row in overrides.values() if isinstance(row["panel"], dict) and "Door" in row["panel"])
        windows = sum(1 for row in overrides.values() if isinstance(row["panel"], dict) and "Window" in row["panel"])
        if kinds.count("IfcMember") != 1 or kinds.count("IfcDoor") != doors or kinds.count("IfcWindow") != windows or kinds.count("IfcPlate") < 1:
            problems.append("%s: parts %s, expected one member, plates, %d doors and %d windows" % (tag, kinds, doors, windows))
        rows = authored(element)
        if json.loads(rows.get("CurtainWall", "null")) is None or rows.get("CurtainWallTypeId") != wall["curtain_wall_type"]:
            problems.append("%s: the authored curtain wall records are missing" % tag)
        elif json.loads(rows["CurtainWallType"]) != snapshot["curtain_wall_types"][wall["curtain_wall_type"]]:
            problems.append("%s: the authored type record differs from the snapshot" % tag)
        elif (json.loads(rows["CurtainPanelOverrides"]) if "CurtainPanelOverrides" in rows else {}) != overrides:
            problems.append("%s: the authored panel overrides differ from the snapshot" % tag)
    return problems


def record_of(element, name):
    """🏷️ The authored record row `name` of an element, `None` when absent."""
    return authored(element).get(name)


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


PHASED = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces"]
"""🕰️ The collections whose elements carry an authored construction phase."""


def phase_problems(model, snapshot):
    """🕰️ The construction phase of every product: read back from the `Semio_Authoring` property set with IfcOpenShell and compared with the
    authored phase of the committed snapshot. New work writes no row, a window or door carries the phase of the wall that hosts it."""
    expected = {tag: row["phase"] for key in PHASED for tag, row in snapshot[key].items()}
    hosts = {**snapshot["walls"], **snapshot["curtain_walls"]}
    for tag, opening in snapshot["openings"].items():
        if "Void" not in opening["kind"]:
            expected[tag] = hosts[opening["host"]]["phase"]
    found = {}
    for element in model.by_type("IfcProduct"):
        tag = identity(element)
        if tag in expected:
            found[tag] = ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {}).get("Phase", "New")
    problems = ["%s: no product in the file" % tag for tag in expected if tag not in found]
    problems += ["%s: phase %s in the file, %s in the snapshot" % (tag, found[tag], phase) for tag, phase in expected.items() if tag in found and found[tag] != phase]
    if snapshot["walls"].keys() >= {"w-south", "w-arc"} and len({phase for phase in expected.values()}) < 4:
        problems.append("the committed house exercises %d of the 4 phases" % len(set(expected.values())))
    return problems


def annotation_problems(model, snapshot, table, committed):
    """🪧️ The annotations of the file against the snapshot: one `IfcAnnotation` per authored annotation, each contained in a storey, each dimension total equal to the distance
    the committed (shapely-adjudicated) inference table prints, each text literal in a `bottom-middle` box with a height."""
    authored = {**snapshot.get("dimensions", {}), **snapshot.get("tags", {}), **snapshot.get("text_notes", {}), **snapshot.get("leaders", {})}
    problems = []
    if table["counts"]["IfcAnnotation"] != len(authored):
        problems.append("IfcAnnotation: %d in the file, %d in the snapshot" % (table["counts"]["IfcAnnotation"], len(authored)))
    for element in model.by_type("IfcAnnotation"):
        container = ifcopenshell.util.element.get_container(element)
        if container is None or not container.is_a("IfcBuildingStorey"):
            problems.append("%s is not contained in a storey" % element.Name)
        identifier = ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {}).get("Id")
        if identifier not in authored:
            problems.append("%s: the authoring Id %s is not an annotation of the snapshot" % (element.Name, identifier))
        for representation in (element.Representation.Representations if element.Representation else []):
            if (representation.RepresentationIdentifier, representation.RepresentationType) != ("Annotation", "Annotation2D"):
                problems.append("%s: representation %s/%s" % (element.Name, representation.RepresentationIdentifier, representation.RepresentationType))
            for item in representation.Items:
                if item.is_a("IfcTextLiteralWithExtent") and (item.BoxAlignment != "bottom-middle" or item.Extent.SizeInY <= 0):
                    problems.append("%s: the text %s has no box" % (element.Name, item.Literal))
    for storey in (committed or {}).values():
        for identifier, row in storey["dimensions"].items():
            written = [value["total"] for value in table["annotations"].values() if value["kind"] == "Dimension" and value["total"] is not None]
            if not any(abs(total - row["total"]) <= TOLERANCE * max(row["total"], 1.0) for total in written):
                problems.append("%s: no dimension total in the file equals the committed distance %.12g" % (identifier, row["total"]))
    return problems


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


# region 🔖️WallDepth
ATTACH_SET = "Semio_WallAttach"
REVEAL_SET = "Semio_OpeningReveal"
SWEEP_SET = "Semio_WallSweep"
ATTACHED_TOPS = {"Roof": "roof", "Slab": "slab", "Ceiling": "ceiling"}
"""🧗️ The top constraints that follow a roof, a slab or a ceiling, with the field that names the target."""

EMPTY_COLLECTIONS = [
    "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces", "openings", "ceilings", "ramps", "wall_sweeps", "properties", "classifications",
    "wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types",
    "components", "component_overrides", "mep_elements", "families", "family_parameters", "family_solids",
]
"""📚️ The collections a case may leave out of its snapshot: the audit reads each of them as empty."""


def rows_of(element, name):
    """🏷️ The rows of one property set of an element (empty when it has none)."""
    return ifcopenshell.util.element.get_psets(element).get(name, {})


def top_of(wall):
    """🔝️ `(kind, body)` of the authored top of a snapshot wall: `("Roof", {"roof": "r-1", "offset": 0})`."""
    return next(iter(wall["top"].items()))


def attach_problems(model, snapshot, table):
    """🧗️ The attach of every wall: the `Semio_WallAttach` set carries the authored top, target, offsets and base slab, a free wall carries none, and the `IfcRelConnectsElements` of the
    file are exactly the top and base attaches of the snapshot, from the wall to its roof, slab or ceiling (read back with `RelatingElement`, `RelatedElement` and `Description`)."""
    problems = []
    walls = {element.Tag: element for element in model.by_type("IfcWall")}
    expected = []
    for tag, wall in snapshot["walls"].items():
        kind, body = top_of(wall)
        slab = wall.get("base_slab") or ""
        rows = rows_of(walls[tag], ATTACH_SET) if tag in walls else {}
        if kind not in ATTACHED_TOPS and not slab:
            if rows:
                problems.append("%s: a free wall carries the set %s" % (tag, ATTACH_SET))
            continue
        target = body.get(ATTACHED_TOPS.get(kind, "storey"), "")
        if kind in ATTACHED_TOPS:
            expected.append(("TopAttach", tag, target))
        if slab:
            expected.append(("BaseAttach", tag, slab))
        want = {"TopKind": kind, "TopTarget": target, "BaseSlab": slab}
        problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value) for key, value in want.items() if rows.get(key) != value]
        for key, value in (("TopOffset", body.get("offset", 0.0)), ("BaseOffset", wall["base_offset"])):
            if not close(rows.get(key, float("nan")), value):
                problems.append("%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value))
    found = sorted((relation.Description, relation.RelatingElement.Tag, relation.RelatedElement.Tag) for relation in model.by_type("IfcRelConnectsElements"))
    if found != sorted(expected):
        problems.append("IfcRelConnectsElements: %s in the file, %s attached by the snapshot" % (found, sorted(expected)))
    if table["counts"]["IfcRelConnectsElements"] != len(expected):
        problems.append("IfcRelConnectsElements: %d counted, %d expected" % (table["counts"]["IfcRelConnectsElements"], len(expected)))
    return problems


def envelope_problems(model, snapshot, committed):
    """🧊️ The tessellated body of an attached wall against the shapely-adjudicated `wall-layout` table: the kernel volume equals the table volume within 1e-6 (relative) and the vertical extent of the
    kernel mesh equals the lowest base and the highest top of the table, on top of the elevation of the building."""
    problems = []
    kernel = settings()
    walls = {element.Tag: element for element in model.by_type("IfcWall")}
    for tag, row in (committed or {}).get("walls", {}).items():
        element = walls.get(tag)
        if element is None or "base_z" not in row:
            continue
        storey = snapshot["storeys"][snapshot["walls"][tag]["storey"]]
        datum = snapshot["buildings"][storey["building"]]["elevation"]
        bounds = kernel_bounds(kernel, element)
        if bounds is None:
            problems.append("%s: the kernel has no geometry" % tag)
            continue
        if not (close(float(bounds[0][2]), row["base_z"] + datum, 1e-6) and close(float(bounds[1][2]), row["top_z"] + datum, 1e-6)):
            problems.append("%s: kernel z %.9f..%.9f, table %.9f..%.9f" % (tag, bounds[0][2], bounds[1][2], row["base_z"] + datum, row["top_z"] + datum))
        volume = kernel_volume(kernel, element)
        if "volume" in row and not close(volume, row["volume"], 1e-6):
            problems.append("%s: kernel volume %.9f, table %.9f" % (tag, volume, row["volume"]))
    return problems


def sweep_problems(model, snapshot, committed):
    """🧷️ The runs of every sweep: one `IfcMember` per run, all with the authored record in `Semio_WallSweep` (host, side, height, inset, material, profile JSON), `Run` counting from zero; the lengths, the
    cross-section and the gross volume of the runs add up to the shapely-adjudicated `sweeps` table, and the kernel volumes of the runs add up to its gross volume within 1e-6 (relative)."""
    problems = []
    kernel = settings()
    members = {}
    for element in model.by_type("IfcMember"):
        if element.ObjectType == "WallSweep":
            members.setdefault(element.Tag, []).append(element)
    if members.keys() != snapshot["wall_sweeps"].keys():
        problems.append("IfcMember sweeps %s in the file, %s in the snapshot" % (sorted(members), sorted(snapshot["wall_sweeps"])))
    for tag, sweep in snapshot["wall_sweeps"].items():
        runs = members.get(tag, [])
        written = sorted(int(rows_of(member, SWEEP_SET).get("Run", -1)) for member in runs)
        if written != list(range(len(runs))):
            problems.append("%s: runs %s are not numbered from zero" % (tag, written))
        for member in runs:
            rows = rows_of(member, SWEEP_SET)
            want = {"SweepId": tag, "Host": sweep["host"], "Side": sweep["side"], "Material": sweep["material"]}
            problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value) for key, value in want.items() if rows.get(key) != value]
            problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), sweep[field]) for key, field in (("Height", "height"), ("Inset", "inset")) if not close(rows.get(key, float("nan")), sweep[field])]
            if json.loads(rows.get("Profile", "null")) != sweep["profile"]:
                problems.append("%s: the profile in the set differs from the snapshot" % tag)
        row = (committed or {}).get("sweeps", {}).get(tag)
        if row is None or not runs:
            continue
        quantities = [quantities_of(member, "Qto_MemberBaseQuantities") for member in runs]
        length, volume = sum(q.get("Length", 0.0) for q in quantities), sum(q.get("GrossVolume", 0.0) for q in quantities)
        if not (close(length, row["length"]) and close(volume, row["gross_volume"])):
            problems.append("%s: runs add up to length %.12g and gross volume %.12g, the table says %.12g and %.12g" % (tag, length, volume, row["length"], row["gross_volume"]))
        if any(not close(q.get("CrossSectionArea", float("nan")), row["section_area"]) for q in quantities):
            problems.append("%s: a cross-section differs from the table value %.12g" % (tag, row["section_area"]))
        kernel_total = sum(kernel_volume(kernel, member) or 0.0 for member in runs)
        if not close(kernel_total, row["gross_volume"], 1e-6):
            problems.append("%s: the kernel volume of the runs is %.12g, the table says %.12g" % (tag, kernel_total, row["gross_volume"]))
    return problems


def reveal_problems(model, snapshot, committed):
    """🪟️ The reveal of every opening: `Semio_OpeningReveal` on the opening element carries exactly the authored depth and material (an opening without a reveal carries none), and the kernel mesh of the
    window of an authored reveal spans the lateral range the shapely-adjudicated table gives its frame (`frame_back_y` .. `frame_front_y`, for a host axis along x)."""
    problems = []
    kernel = settings()
    voids = {element.Tag: element for element in model.by_type("IfcOpeningElement")}
    windows = {element.Tag: element for element in model.by_type("IfcWindow")}
    for tag, opening in snapshot["openings"].items():
        rows = rows_of(voids[tag], REVEAL_SET)
        depth, material = opening.get("reveal_depth"), opening.get("reveal_material")
        if depth is None and material is None:
            if rows:
                problems.append("%s: an opening without a reveal carries %s" % (tag, REVEAL_SET))
            continue
        if (depth is None) != ("RevealDepth" not in rows) or (depth is not None and not close(rows["RevealDepth"], depth)):
            problems.append("%s: RevealDepth %r, the snapshot says %r" % (tag, rows.get("RevealDepth"), depth))
        if rows.get("RevealMaterial") != material:
            problems.append("%s: RevealMaterial %r, the snapshot says %r" % (tag, rows.get("RevealMaterial"), material))
    for tag, row in (committed or {}).get("reveals", {}).items():
        bounds = kernel_bounds(kernel, windows[tag]) if tag in windows else None
        host = snapshot["walls"][snapshot["openings"][tag]["host"]]["axis"]["Line"]
        if bounds is None or host["start"]["y"] != host["end"]["y"]:
            problems.append("%s: no window geometry or a host axis that does not run along x" % tag)
            continue
        if not (close(float(bounds[0][1]), row["frame_back_y"] + host["start"]["y"], 1e-6) and close(float(bounds[1][1]), row["frame_front_y"] + host["start"]["y"], 1e-6)):
            problems.append("%s: the window spans y %.9f..%.9f, the table says %.9f..%.9f" % (tag, bounds[0][1], bounds[1][1], row["frame_back_y"], row["frame_front_y"]))
    return problems


def wall_depth_problems(model, snapshot, table, committed):
    """🧗️ Every place where the attached walls, sweeps and reveals of the file leave the snapshot or the committed wall-depth table."""
    return attach_problems(model, snapshot, table) + envelope_problems(model, snapshot, committed) + sweep_problems(model, snapshot, committed) + reveal_problems(model, snapshot, committed)


# endregion 🔖️WallDepth


def exported_ramps(snapshot, runs):
    """🛝️ The ramps the export writes: those the committed `ramp-runs` table resolves (their storey exists) with a path of positive length."""
    return {ramp_id: run for ramp_id, run in (runs or {}).items() if ramp_id in snapshot.get("ramps", {}) and run["length"] > TOLERANCE}


def ramp_type(ramp, run):
    """🛝️ `IfcRampTypeEnum` of a ramp: a curved path is a spiral, up to two landings make one straight run, three landings two straight runs, more is user defined."""
    if any(vertex["bulge"] != 0 for vertex in ramp["path"][:-1]):
        return "SPIRAL_RAMP"
    landings = len(run["landings"])
    return "STRAIGHT_RUN_RAMP" if landings <= 2 else "TWO_STRAIGHT_RUN_RAMP" if landings == 3 else "USERDEFINED"


def quantities_of(element, name):
    """🧮️ One base-quantity set of an element, or an empty dict."""
    return ifcopenshell.util.element.get_psets(element, qtos_only=True).get(name, {})


def close(left, right, relative=1e-9):
    """⚖️ Equality within a relative tolerance (absolute below one)."""
    return abs(left - right) <= relative * max(abs(right), 1.0)


def ramp_problems(model, snapshot, runs):
    """🛝️ Every `IfcRamp` of the file against the committed `ramp-runs` table (shapely-adjudicated): one ramp per resolved ramp in its storey, one `IfcRampFlight` per sloped flight and one `LANDING` slab per
    landing, all aggregated by the ramp with `IfcRelAggregates`; the ramp's base quantities (length, width, rise, plan area) and every flight and landing quantity equal the run; one `IfcRailing` per ramp with a side
    railing; the authored record travels back unchanged; and the kernel volumes of the aggregated parts add up to the written gross volume."""
    problems = []
    expected = exported_ramps(snapshot, runs)
    ramps = {element.Tag: element for element in model.by_type("IfcRamp", include_subtypes=False)}
    if ramps.keys() != expected.keys():
        problems.append("IfcRamp: %s in the file, %s resolved by the ramp-runs table" % (sorted(ramps), sorted(expected)))
    kernel = settings()
    flights_total = landings_total = 0
    for ramp_id, run in expected.items():
        authored, element = snapshot["ramps"][ramp_id], ramps.get(ramp_id)
        if element is None:
            continue
        if element.ShapeType != ramp_type(authored, run):
            problems.append("%s: ramp type %s, expected %s" % (ramp_id, element.ShapeType, ramp_type(authored, run)))
        container = ifcopenshell.util.element.get_container(element)
        if container is None or identity(container) != authored["storey"]:
            problems.append("%s is not contained in its storey %s" % (ramp_id, authored["storey"]))
        parts = ifcopenshell.util.element.get_parts(element)
        flights = sorted((part for part in parts if part.is_a("IfcRampFlight")), key=lambda part: part.Tag)
        landings = sorted((part for part in parts if part.is_a("IfcSlab") and part.PredefinedType == "LANDING"), key=lambda part: part.Tag)
        rails = [part for part in parts if part.is_a("IfcRailing")]
        flights_total, landings_total = flights_total + len(flights), landings_total + len(landings)
        if len(flights) != len(run["flights"]) or len(landings) != len(run["landings"]):
            problems.append("%s: %d flights and %d landings in the file, %d and %d in the run" % (ramp_id, len(flights), len(landings), len(run["flights"]), len(run["landings"])))
        if len(parts) != len(flights) + len(landings) + len(rails):
            problems.append("%s: %d aggregated parts are neither flights, landings nor railings" % (ramp_id, len(parts) - len(flights) - len(landings) - len(rails)))
        if len(rails) != (1 if authored["railing_left"] or authored["railing_right"] else 0):
            problems.append("%s: %d railings in the file for the sides asked for" % (ramp_id, len(rails)))
        for part, row in zip(flights, run["flights"]):
            values = quantities_of(part, "Qto_RampFlightBaseQuantities")
            if not (close(values.get("Length", -1), row["length"]) and close(values.get("Width", -1), run["width"])):
                problems.append("%s: flight %s quantities %s, run %.12g x %.12g" % (ramp_id, part.Tag, values, row["length"], run["width"]))
        for part, row in zip(landings, run["landings"]):
            values = quantities_of(part, "Qto_SlabBaseQuantities")
            if not (close(values.get("Length", -1), row["length"]) and close(values.get("Width", -1), run["width"])):
                problems.append("%s: landing %s quantities %s, run %.12g x %.12g" % (ramp_id, part.Tag, values, row["length"], run["width"]))
        values = quantities_of(element, "Qto_RampBaseQuantities")
        for name, want in (("Length", run["length"]), ("Width", run["width"]), ("Height", run["rise"]), ("GrossArea", run["width"] * run["length"])):
            if name not in values or not close(values[name], want, 1e-9 if name != "GrossArea" else 1e-6):
                problems.append("%s: Qto_RampBaseQuantities.%s is %s, the run says %.12g" % (ramp_id, name, values.get(name), want))
        volumes = [kernel_volume(kernel, part) for part in flights + landings]
        if any(volume is None for volume in volumes):
            problems.append("%s: the kernel has no geometry for a flight or a landing" % ramp_id)
        elif "GrossVolume" in values and not close(sum(volumes), values["GrossVolume"], 1e-6):
            problems.append("%s: the kernel volume of the parts is %.12g, the written GrossVolume %.12g" % (ramp_id, sum(volumes), values["GrossVolume"]))
        record = ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {}).get("Ramp")
        if record is None or json.loads(record) != authored:
            problems.append("%s: the authored ramp record in Semio_Authoring differs from the snapshot" % ramp_id)
    landing_slabs = sum(1 for slab in model.by_type("IfcSlab", include_subtypes=False) if slab.PredefinedType == "LANDING")
    if (flights_total, landings_total) != (len(model.by_type("IfcRampFlight", include_subtypes=False)), landing_slabs):
        problems.append("the file holds flights or landing slabs that no ramp aggregates")
    return problems


# region 🔖️Components
def families_oracle():
    """🧬️ The independent family evaluator of the families inference oracle (the F1 expression interpreter, numpy and shapely), loaded by path."""
    if "families_oracle" not in sys.modules:
        folder = next(path for path in sorted(Path(__file__).resolve().parents[1].iterdir()) if path.name.endswith("infer-bim-1-families"))
        spec = importlib.util.spec_from_file_location("families_oracle", next(path for path in sorted(folder.iterdir()) if path.name.endswith(".py")))
        loaded = importlib.util.module_from_spec(spec)
        sys.modules["families_oracle"] = loaded
        spec.loader.exec_module(loaded)
    return sys.modules["families_oracle"]


def component_class(category, system):
    """🪑️ The IFC 2x3 class of a component: furniture is an `IfcFurnishingElement`, plumbing and lighting fixtures and every Mechanical or Electrical family with a system are `IfcFlowTerminal`s, the rest `IfcBuildingElementProxy`."""
    if category == "Furniture":
        return "IfcFurnishingElement"
    if category in ("Plumbing", "Lighting") or (category in ("Mechanical", "Electrical") and system):
        return "IfcFlowTerminal"
    return "IfcBuildingElementProxy"


def component_type_class(category, entity):
    """🏷️ The type class that goes with a family category and the class of its occurrences."""
    return {"Furniture": "IfcFurnitureType", "Plumbing": "IfcSanitaryTerminalType", "Lighting": "IfcLightFixtureType"}.get(category) or ("IfcFlowTerminalType" if entity == "IfcFlowTerminal" else "IfcBuildingElementProxyType")


SEGMENTS = {"Duct": ("IfcDuctSegmentType", "Qto_DuctSegmentBaseQuantities"), "Pipe": ("IfcPipeSegmentType", "Qto_PipeSegmentBaseQuantities"), "Tray": ("IfcCableCarrierSegmentType", "Qto_CableCarrierSegmentBaseQuantities")}
"""🌀️ The type class and the base quantity set of the IFC 2x3 type objects of each MEP shape (the occurrences are `IfcFlowSegment`s)."""

COMPONENT_SET = "Qto_ComponentBaseQuantities"
OVERRIDES_SET = "Semio_ComponentOverrides"
COORDINATE_TOLERANCE = 1e-6
"""📏️ The sets of a component and the tolerance of the kernel's coordinates (the exporter rounds to a nanometre)."""


def storey_base(snapshot, storey_id):
    """🪜️ Building origin and absolute elevation of a storey (the sum of the heights of the storeys below it plus the building elevation; the case has an unrotated building)."""
    storey = snapshot["storeys"][storey_id]
    building = snapshot["buildings"][storey["building"]]
    below = sorted((row["level"], sid) for sid, row in snapshot["storeys"].items() if row["building"] == storey["building"])
    z = building["elevation"]
    for level, sid in below:
        if sid == storey_id:
            break
        z += snapshot["storeys"][sid]["height"]
    return (building["origin"]["x"], building["origin"]["y"]), z


def wall_thickness(snapshot, wall):
    """📏️ The total thickness of a wall: the sum of its layers."""
    return sum(layer["thickness"] for layer in snapshot["wall_types"][wall["wall_type"]]["layers"])


def instance_frame(snapshot, component):
    """📐️ `(origin x, origin y, yaw)` of a component in its storey frame, from the documented semantics: a free component turns by its rotation about its position; a hosted one projects its position on the wall
    axis (shapely `project`), sits on the face of the wall on the side where the position lies and turns so that its depth axis points away from the wall, plus its rotation."""
    from shapely.geometry import LineString, Point

    position = component["position"]
    if component.get("host") is None:
        return position["x"], position["y"], component["rotation"]
    wall = snapshot["walls"][component["host"]]
    body = wall["axis"]["Line"]
    start, end = (body["start"]["x"], body["start"]["y"]), (body["end"]["x"], body["end"]["y"])
    length = math.dist(start, end)
    direction = ((end[0] - start[0]) / length, (end[1] - start[1]) / length)
    along = LineString([start, end]).project(Point(position["x"], position["y"]))
    left = (-direction[1], direction[0])
    side = (position["x"] - start[0]) * left[0] + (position["y"] - start[1]) * left[1]
    normal = left if side >= 0 else (-left[0], -left[1])
    half = wall_thickness(snapshot, wall) / 2
    origin = (start[0] + direction[0] * along + normal[0] * half, start[1] + direction[1] * along + normal[1] * half)
    return origin[0], origin[1], math.atan2(-normal[0], normal[1]) + component["rotation"]


def family_solids(snapshot, component_id):
    """🧬️ The visible solids of a component's family under its overrides, as `(min, max, volume)` boxes in the family frame: the family evaluator re-run on a copy of the snapshot whose parameter formulas are replaced."""
    component = snapshot["components"][component_id]
    adjusted = copy.deepcopy(snapshot)
    for row in snapshot.get("component_overrides", {}).values():
        if row["component"] == component_id:
            adjusted["family_parameters"]["%s.%s" % (component["family"], row["name"])]["value"] = row["value"]
    family = families_oracle().family_row(adjusted, component["family"])
    return [(solid["min"], solid["max"], solid["volume"]) for solid in family["solids"].values() if solid["visible"] and solid["volume"] > 0], family["issues"]


def world_bounds(snapshot, component_id):
    """📦️ The world-space `(min, max)` corners of the instance: every family box transformed by the mirror (negated x), the yaw and the origin of the instance frame, on top of the storey elevation."""
    component = snapshot["components"][component_id]
    boxes, _ = family_solids(snapshot, component_id)
    (bx, by), floor = storey_base(snapshot, component["storey"])
    ox, oy, yaw = instance_frame(snapshot, component)
    sine, cosine = math.sin(yaw), math.cos(yaw)
    points = []
    for low, high, _ in boxes:
        for x in (low[0], high[0]):
            for y in (low[1], high[1]):
                for z in (low[2], high[2]):
                    local = (-x if component["mirrored"] else x, y)
                    points.append((bx + ox + local[0] * cosine - local[1] * sine, by + oy + local[0] * sine + local[1] * cosine, floor + component["elevation"] + z))
    return [min(point[axis] for point in points) for axis in range(3)], [max(point[axis] for point in points) for axis in range(3)]


def footprint_area(snapshot, component_id):
    """👣️ The plan area of the rectangle that bounds the visible solids in the instance frame."""
    boxes, _ = family_solids(snapshot, component_id)
    return (max(high[0] for _, high, _ in boxes) - min(low[0] for low, _, _ in boxes)) * (max(high[1] for _, high, _ in boxes) - min(low[1] for low, _, _ in boxes))


def section_area(shape):
    """▭️ The closed-form cross-section area of an MEP shape: width x height, or pi r^2 (the quantities are written in closed form, the solid of a pipe is its inscribed polygon)."""
    kind, body = next(iter(shape.items()))
    if kind == "Pipe":
        return math.pi * body["diameter"] ** 2 / 4
    return body["width"] * body["height"]


def path_length(path):
    """📏️ The length of a polyline in space."""
    return sum(math.dist((a["x"], a["y"], a["z"]), (b["x"], b["y"], b["z"])) for a, b in zip(path, path[1:]))


def shape_key(shape):
    """🔑️ `(kind, dimensions)` of an MEP shape: elements of one shape share one type object."""
    kind, body = next(iter(shape.items()))
    return kind, tuple(sorted(body.items()))


def clean(rows):
    """🧹️ A property set as a dict without the id ifcopenshell adds."""
    return {key: value for key, value in rows.items() if key != "id"}


def component_problems(model, snapshot):
    """🪑️ Components and MEP elements of the file against the snapshot and the independent family evaluator: one product of the documented class per component and per MEP element, contained in its storey, typed by an
    object that carries the authored family (parameters and solids) or the shape, the authored record and the overrides as property sets, the base quantities, the kernel volume equal to the written one and to the sum
    of the family solids (components) or the section times the centre line (MEP), the kernel bounds equal to the instance frame (host fit, yaw, mirror) applied to the family boxes (components) or around the centre line
    (MEP), and one `IfcSystem` per service that groups its elements and terminals."""
    components, mep = snapshot.get("components", {}), snapshot.get("mep_elements", {})
    if not components and not mep:
        return []
    problems = []
    kernel = settings()
    classes = {"IfcFurnishingElement", "IfcFlowTerminal", "IfcBuildingElementProxy", "IfcFlowSegment"}
    products = {element.Tag: element for name in classes for element in model.by_type(name, include_subtypes=False) if element.Tag and ":" not in element.Tag}
    expected_ids = set(components) | set(mep)
    if set(products) != expected_ids:
        problems.append("products %s in the file, %s in the snapshot" % (sorted(set(products) - expected_ids), sorted(expected_ids - set(products))))
    types = {}
    for tag, component in components.items():
        element = products.get(tag)
        if element is None:
            continue
        category = snapshot["families"][component["family"]]["category"]
        entity = component_class(category, component.get("system"))
        if element.is_a() != entity:
            problems.append("%s: class %s, expected %s" % (tag, element.is_a(), entity))
        container = ifcopenshell.util.element.get_container(element)
        if container is None or identity(container) != component["storey"]:
            problems.append("%s is not contained in its storey %s" % (tag, component["storey"]))
        kind = ifcopenshell.util.element.get_type(element)
        wanted = component_type_class(category, entity)
        if kind is None or kind.is_a() != wanted or kind.Tag != component["family"]:
            problems.append("%s: type %s, expected %s of family %s" % (tag, kind and (kind.is_a(), kind.Tag), wanted, component["family"]))
        elif kind.Name != snapshot["families"][component["family"]]["name"]:
            problems.append("%s: type name %s, family %s" % (tag, kind.Name, snapshot["families"][component["family"]]["name"]))
        else:
            types[(component["family"], wanted)] = kind
        record = record_of(element, "Component")
        if record is None or json.loads(record) != component:
            problems.append("%s: the authored component record in Semio_Authoring differs from the snapshot" % tag)
        overrides = {row["name"]: row["value"] for row in snapshot.get("component_overrides", {}).values() if row["component"] == tag}
        found = clean(ifcopenshell.util.element.get_psets(element).get(OVERRIDES_SET, {}))
        if found != overrides:
            problems.append("%s: overrides %s in %s, %s in the snapshot" % (tag, found, OVERRIDES_SET, overrides))
        boxes, issues = family_solids(snapshot, tag)
        if issues or not boxes:
            problems.append("%s: the family evaluates with issues %s" % (tag, issues))
            continue
        volume = sum(box[2] for box in boxes)
        values = quantities_of(element, COMPONENT_SET)
        for name, want in (("GrossVolume", volume), ("NetVolume", volume), ("GrossFootprintArea", footprint_area(snapshot, tag))):
            if name not in values or not close(values[name], want, 1e-9):
                problems.append("%s: %s.%s is %s, the independent family evaluation gives %.12g" % (tag, COMPONENT_SET, name, values.get(name), want))
        measured = kernel_volume(kernel, element)
        if measured is None or not close(measured, volume, 1e-9):
            problems.append("%s: kernel volume %s, the independent family evaluation gives %.12g" % (tag, measured, volume))
        bounds = kernel_bounds(kernel, element)
        low, high = world_bounds(snapshot, tag)
        if bounds is None or any(abs(float(found) - want) > COORDINATE_TOLERANCE for found, want in zip(list(bounds[0]) + list(bounds[1]), low + high)):
            problems.append("%s: kernel bounds %s, the instance frame gives %s" % (tag, bounds and (bounds[0].tolist(), bounds[1].tolist()), (low, high)))
    for (family_id, wanted), kind in types.items():
        rows = clean(ifcopenshell.util.element.get_psets(kind).get("Semio_Authoring", {}))
        parameters = {row["name"]: row for row in snapshot["family_parameters"].values() if row["family"] == family_id}
        solids = {key: row for key, row in snapshot["family_solids"].items() if row["family"] == family_id}
        if json.loads(rows.get("Family", "null")) != snapshot["families"][family_id] or json.loads(rows.get("Parameters", "null")) != parameters or json.loads(rows.get("Solids", "null")) != solids:
            problems.append("%s: the authored family records in the type differ from the snapshot" % family_id)
    shapes = {}
    for tag, row in mep.items():
        element = products.get(tag)
        if element is None:
            continue
        kind, body = next(iter(row["shape"].items()))
        type_class, quantity_set = SEGMENTS[kind]
        if element.is_a() != "IfcFlowSegment":
            problems.append("%s: class %s, expected IfcFlowSegment" % (tag, element.is_a()))
        container = ifcopenshell.util.element.get_container(element)
        if container is None or identity(container) != row["storey"]:
            problems.append("%s is not contained in its storey %s" % (tag, row["storey"]))
        owner = ifcopenshell.util.element.get_type(element)
        if owner is None or owner.is_a() != type_class:
            problems.append("%s: type %s, expected %s" % (tag, owner and owner.is_a(), type_class))
        elif shapes.setdefault(shape_key(row["shape"]), owner) is not owner:
            problems.append("%s: elements of one shape %s do not share one type object" % (tag, shape_key(row["shape"])))
        record = record_of(element, "MepElement")
        if record is None or json.loads(record) != row:
            problems.append("%s: the authored MEP record in Semio_Authoring differs from the snapshot" % tag)
        axis = [item for identifier, item in representation_items(element) if identifier == "Axis"]
        points = [tuple(point.Coordinates) for point in axis[0].Points] if len(axis) == 1 and axis[0].is_a("IfcPolyline") else None
        wanted = [(point["x"], point["y"], point["z"]) for point in row["path"]]
        if points is None or len(points) != len(wanted) or any(abs(a - b) > 1e-9 for found, want in zip(points, wanted) for a, b in zip(found, want)):
            problems.append("%s: the Axis polyline %s is not the path of the snapshot" % (tag, points))
        length, area = path_length(row["path"]), section_area(row["shape"])
        values = quantities_of(element, quantity_set)
        for name, want in (("Length", length), ("CrossSectionArea", area), ("GrossVolume", area * length), ("NetVolume", area * length)):
            if name not in values or not close(values[name], want, 1e-9):
                problems.append("%s: %s.%s is %s, the section times the centre line gives %.12g" % (tag, quantity_set, name, values.get(name), want))
        measured = kernel_volume(kernel, element)
        lowest = 0.98 if kind == "Pipe" else 1 - 1e-6
        if measured is None or measured > area * length * (1 + 1e-9) or measured < area * length * lowest:
            problems.append("%s: kernel volume %s, the mitred prism gives %.12g" % (tag, measured, area * length))
        bounds = kernel_bounds(kernel, element)
        reach = max(body.values()) / 2
        lows = [min(point[axis] for point in [(p["x"], p["y"], p["z"]) for p in row["path"]]) for axis in range(3)]
        highs = [max(point[axis] for point in [(p["x"], p["y"], p["z"]) for p in row["path"]]) for axis in range(3)]
        (bx, by), floor = storey_base(snapshot, row["storey"])
        shift = [bx, by, floor]
        if bounds is None or any(found > shift[axis] + lows[axis] + 1e-6 or found < shift[axis] + lows[axis] - reach - 1e-6 for axis, found in enumerate(bounds[0])) or any(found < shift[axis] + highs[axis] - 1e-6 or found > shift[axis] + highs[axis] + reach + 1e-6 for axis, found in enumerate(bounds[1])):
            problems.append("%s: kernel bounds %s do not enclose the centre line %s..%s within %.3f" % (tag, bounds and (bounds[0].tolist(), bounds[1].tolist()), lows, highs, reach))
    groups = {}
    for relation in model.by_type("IfcRelAssignsToGroup"):
        if relation.RelatingGroup.is_a("IfcSystem"):
            groups.setdefault(relation.RelatingGroup.Name, []).extend(identity(member) for member in relation.RelatedObjects)
    expected = {}
    for tag, row in mep.items():
        expected.setdefault(row["system"], []).append(tag)
    for tag, component in components.items():
        if component.get("system"):
            expected.setdefault(component["system"], []).append(tag)
    if {name: sorted(tags) for name, tags in groups.items()} != {name: sorted(tags) for name, tags in expected.items()}:
        problems.append("systems %s in the file, %s in the snapshot" % ({name: sorted(tags) for name, tags in groups.items()}, {name: sorted(tags) for name, tags in expected.items()}))
    if len(model.by_type("IfcSystem", include_subtypes=False)) != len(expected):
        problems.append("IfcSystem: %d in the file, one per service (%d) expected" % (len(model.by_type("IfcSystem", include_subtypes=False)), len(expected)))
    wanted_types = len({(component["family"], component_type_class(snapshot["families"][component["family"]]["category"], component_class(snapshot["families"][component["family"]]["category"], component.get("system")))) for component in components.values()}) + len({shape_key(row["shape"]) for row in mep.values()})
    found_types = sum(len(model.by_type(name, include_subtypes=False)) for name in ("IfcFurnitureType", "IfcFlowTerminalType", "IfcSanitaryTerminalType", "IfcLightFixtureType", "IfcDuctSegmentType", "IfcPipeSegmentType", "IfcCableCarrierSegmentType")) + len([kind for kind in model.by_type("IfcBuildingElementProxyType", include_subtypes=False) if kind.Tag in snapshot.get("families", {})])
    if found_types != wanted_types:
        problems.append("%d type objects of components and MEP elements in the file, %d expected" % (found_types, wanted_types))
    return problems


# endregion 🔖️Components


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
