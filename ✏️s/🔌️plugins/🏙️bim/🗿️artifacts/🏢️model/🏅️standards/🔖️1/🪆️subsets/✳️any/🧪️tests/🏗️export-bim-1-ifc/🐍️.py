#!/usr/bin/env python3
"""🏗️ Third-party ORACLE (IfcOpenShell) for the IFC 2x3 export of the BIM house.

IfcOpenShell 0.8.4.post1 has never seen this repository's writer. It opens the committed file
`🧫️fixtures/🏗️ifc/🏠️house/🏠️house.ifc`, and:

* parses it, requires the schema `IFC2X3` and validates every attribute type and every EXPRESS WHERE rule (`ifcopenshell.validate`);
* counts the entities of every class the export writes;
* reads the spatial containment of every product (`ifcopenshell.util.element.get_container`);
* tessellates every straight wall, level slab, column and beam with its C++ geometry kernel (openings are subtracted
  by the kernel through `IfcRelVoidsElement`) and measures the volume of the triangulation;
* requires every window and door to lie inside the bounding box of the wall it fills;
* reads the base quantities back (`ifcopenshell.util.element.get_psets(..., qtos_only=True)`) and requires the kernel
  volume to equal the written `NetVolume`/`GrossVolume` within 1e-9.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🏗️ifc>     # rewrite the measured table from the committed file

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
"""⚖️ Absolute tolerance between the kernel's volume, the written quantity and the subject's report."""

COUNTED = [
    "IfcProject", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcWallStandardCase", "IfcWall", "IfcOpeningElement", "IfcWindow", "IfcDoor",
    "IfcSlab", "IfcRoof", "IfcColumn", "IfcBeam", "IfcStair", "IfcStairFlight", "IfcRailing", "IfcCurtainWall", "IfcMember", "IfcPlate", "IfcSpace", "IfcGrid",
    "IfcRelVoidsElement", "IfcRelFillsElement", "IfcRelAggregates", "IfcRelContainedInSpatialStructure", "IfcRelDefinesByType", "IfcRelAssociatesMaterial",
    "IfcRelAssociatesClassification", "IfcMaterialLayerSet", "IfcMaterialLayerSetUsage", "IfcWallType", "IfcSlabType", "IfcBuildingElementProxyType", "IfcColumnType",
    "IfcBeamType", "IfcWindowStyle", "IfcDoorStyle", "IfcFacetedBrep", "IfcClassification", "IfcClassificationReference",
]
"""📊️ The classes whose instances are counted."""

MEASURED = ["IfcWallStandardCase", "IfcWall", "IfcColumn", "IfcBeam", "IfcSlab"]
"""📐️ The classes whose volume the kernel measures."""


def identity(element):
    """🔖️ The model id the export carries in `Tag` (elements) or `ObjectType` (spatial elements)."""
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
    """📏️ The volume written in the element's base quantities (`NetVolume`, else `GrossVolume`)."""
    for name, values in ifcopenshell.util.element.get_psets(element, qtos_only=True).items():
        if name.endswith("BaseQuantities"):
            for key in ("NetVolume", "GrossVolume"):
                if key in values:
                    return float(values[key])
    return None


def straight(element, snapshot):
    """➖️ Whether the element is measured exactly by the kernel: not a curved wall, a sloped slab or a round profile (the kernel tessellates circles)."""
    tag = element.Tag
    if element.is_a("IfcWall") and not element.is_a("IfcWallStandardCase"):
        return "Line" in snapshot["walls"][tag]["axis"]
    if element.is_a("IfcSlab"):
        return tag in snapshot["slabs"] and not snapshot["slabs"][tag].get("slope")
    if element.is_a("IfcColumn"):
        return "Circle" not in snapshot["column_types"][snapshot["columns"][tag]["column_type"]]["profile"]
    if element.is_a("IfcBeam"):
        return "Circle" not in snapshot["beam_types"][snapshot["beams"][tag]["beam_type"]]["profile"]
    return True


def measure(model, snapshot):
    """🧊️ The oracle table: counts per class, containment per storey and the kernel volume of every measurable element."""
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
    return {"schema": model.schema, "counts": counts, "containment": containment, "volumes": volumes}


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


def audit(model, snapshot, table):
    """⚖️ Every place where the file, the written quantities or the snapshot leave the kernel's table."""
    problems = []
    if table["schema"] != "IFC2X3":
        problems.append("schema is %s, not IFC2X3" % table["schema"])
    collections = {
        "IfcBuildingStorey": "storeys", "IfcColumn": "columns", "IfcBeam": "beams", "IfcSpace": "spaces", "IfcCurtainWall": "curtain_walls", "IfcStair": "stairs", "IfcRailing": "railings",
        "IfcOpeningElement": "openings", "IfcRoof": "roofs",
    }
    for cls, key in collections.items():
        if table["counts"][cls] != len(snapshot[key]):
            problems.append("%s: %d in the file, %d in the snapshot" % (cls, table["counts"][cls], len(snapshot[key])))
    walls = table["counts"]["IfcWall"] + table["counts"]["IfcWallStandardCase"]
    if walls != len(snapshot["walls"]):
        problems.append("walls: %d in the file, %d in the snapshot" % (walls, len(snapshot["walls"])))
    openings = [opening for opening in snapshot["openings"].values() if "Void" not in opening["kind"]]
    if table["counts"]["IfcWindow"] + table["counts"]["IfcDoor"] != len(openings):
        problems.append("fillings: %d in the file, %d in the snapshot" % (table["counts"]["IfcWindow"] + table["counts"]["IfcDoor"], len(openings)))
    for storey, tags in table["containment"].items():
        for tag in tags:
            if tag not in {**snapshot["walls"], **snapshot["columns"], **snapshot["beams"], **snapshot["slabs"], **snapshot["roofs"], **snapshot["stairs"], **snapshot["railings"], **snapshot["curtain_walls"], **snapshot["openings"]}:
                problems.append("%s contains the unknown element %s" % (storey, tag))
    by_tag = {element.Tag: element for name in MEASURED for element in model.by_type(name, include_subtypes=False)}
    for tag, volume in table["volumes"].items():
        quantity = written_volume(by_tag[tag])
        if quantity is None:
            problems.append("%s has no written volume" % tag)
        elif abs(quantity - volume) > TOLERANCE * max(abs(volume), 1.0):
            problems.append("%s: kernel %.12g, written %.12g" % (tag, volume, quantity))
    problems += placement_problems(model)
    return problems


def open_case(root):
    """📂️ The committed snapshot, the committed file and the committed table of the house case."""
    case = Path(root) / "🏠️house"
    snapshot = json.loads((case / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    model = ifcopenshell.open(str(case / "🏠️house.ifc"))
    return case, snapshot, model


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

    return Adapter("python").oracle("export-ifc-house", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], arguments[1]
    case, snapshot, model = open_case(root)
    table = measure(model, snapshot)
    problems = audit(model, snapshot, table)
    log = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, log, express_rules=True)
    problems += ["validate: %s" % entry["message"] for entry in log.statements if str(entry.get("level")).lower() == "error"]
    path = case / "🔬️measure" / "🔣️.json"
    if command == "write":
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        print("wrote %s: %d classes, %d measured elements" % (path, len(table["counts"]), len(table["volumes"])))
    else:
        committed = json.loads(path.read_text(encoding="utf-8"))
        if committed != table:
            problems.append("the committed table differs from the measurement of the committed file")
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (ifcopenshell %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", ifcopenshell.version))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
