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
* reads every `IfcRamp` of the ramps case with its `IfcRampFlight` and `LANDING` slab parts (`ifcopenshell.util.element.get_parts`) and requires their counts, base quantities, ramp type, storey, railing and authored
  record to equal the committed, shapely-adjudicated `ramp-runs` table, and the kernel volumes of the parts to add up to the written gross volume;
* reads the base quantities back (`ifcopenshell.util.element.get_psets(..., qtos_only=True)`) and requires the kernel
  volume to equal the written `NetVolume`/`GrossVolume` within 1e-9.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🏗️ifc> [case...]   # rewrite the measured table from the committed file (of the named cases only)

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.geom
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
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference", "IfcAnnotation", "IfcTextLiteralWithExtent", "IfcPolyline", "IfcPlanarExtent", "IfcCovering", "IfcCoveringType", "IfcRamp", "IfcRampFlight",
]
"""📊️ The classes whose instances are counted."""

MEASURED = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab", "IfcCovering"]
"""📐️ The classes whose volume the kernel measures."""


CASES = {"🏠️house": "🏠️house.ifc", "🪧️notated": "🪧️notated.ifc", "🔲️ceilings": "🔲️ceilings.ifc", "🛝️ramps": "🛝️ramps.ifc"}
SNAPSHOTS = {"🏠️house": ("🏗️ifc", "🏠️house", "📸️snapshot"), "🔲️ceilings": ("🏗️ifc", "🔲️ceilings", "📸️snapshot"), "🪧️notated": ("💡️inferences", "🪧️annotation-layout", "🏠️room", "📸️snapshot"), "🛝️ramps": ("💡️inferences", "🛝️ramp-runs", "🏞️ramps", "📸️snapshot")}
INFERRED = {"🪧️notated": ("annotations", "📏️annotations"), "🛝️ramps": ("ramp-runs", "🛝️ramp-runs")}
"""🧮️ The committed inference table a case is audited against: its key in `committed` and its folder next to the snapshot."""


def identity(element):
    """🔖️ The model id the export carries in `Tag` (elements), `Name` (annotations) or `ObjectType` (spatial elements)."""
    if element.is_a("IfcAnnotation"):
        return element.Name
    return element.Tag if element.is_a("IfcElement") else element.ObjectType


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
        return "Line" in snapshot["walls"][tag]["axis"]
    if element.is_a("IfcSlab"):
        return tag in snapshot["slabs"] and not snapshot["slabs"][tag].get("slope")
    if element.is_a("IfcCovering"):
        ceiling = snapshot["ceilings"].get(tag)
        return ceiling is not None and not ceiling.get("slope") and all(vertex["bulge"] == 0 for loop in [ceiling["boundary"], *ceiling["holes"]] for vertex in loop)
    if element.is_a("IfcColumn"):
        return "Circle" not in snapshot["column_types"][snapshot["columns"][tag]["column_type"]]["profile"]
    if element.is_a("IfcBeam"):
        return "Circle" not in snapshot["beam_types"][snapshot["beams"][tag]["beam_type"]]["profile"]
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
    return {"schema": model.schema, "counts": counts, "containment": containment, "volumes": volumes, "annotations": annotation_rows(model)}


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


def audit(model, snapshot, table, committed=None):
    """⚖️ Every place where the file, the written quantities or the snapshot leave the kernel's table."""
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
            if tag not in {**snapshot["walls"], **snapshot["columns"], **snapshot["beams"], **snapshot["slabs"], **snapshot.get("ceilings", {}), **snapshot["roofs"], **snapshot["stairs"], **snapshot["railings"], **snapshot.get("ramps", {}), **snapshot["curtain_walls"], **snapshot["openings"]}:
                problems.append("%s contains the unknown element %s" % (storey, tag))
    by_tag = {element.Tag: element for name in MEASURED for element in model.by_type(name, include_subtypes=False)}
    for tag, volume in table["volumes"].items():
        quantity = written_volume(by_tag[tag])
        if quantity is None:
            problems.append("%s has no written volume" % tag)
        elif abs(quantity - volume) > TOLERANCE * max(abs(volume), 1.0):
            problems.append("%s: kernel %.12g, written %.12g" % (tag, volume, quantity))
    problems += placement_problems(model)
    problems += phase_problems(model, snapshot)
    problems += annotation_problems(model, snapshot, table, committed.get("annotations"))
    problems += ramp_problems(model, snapshot, committed.get("ramp-runs"))
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


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-ifc-house", export_handler).oracle("export-ifc-notated", export_handler).oracle("export-ifc-ceilings", export_handler).oracle("export-ifc-ramps", export_handler).oracle("export-ifc-stepped", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
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
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
