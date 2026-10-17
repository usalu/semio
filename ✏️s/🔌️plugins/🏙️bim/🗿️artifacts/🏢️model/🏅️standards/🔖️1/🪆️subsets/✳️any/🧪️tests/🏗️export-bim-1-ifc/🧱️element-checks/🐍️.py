"""🧱️ The element audits of the house, frame and annotation cases: the placement of products, the frame members, the authored phases and the annotations."""

from __future__ import annotations

# region 🔖️Imports
import json
import math
import ifcopenshell.util.element

from export_bim_ifc_kernel import (
    TOLERANCE,
    identity,
    settings,
    kernel_volume,
    kernel_bounds,
    authored,
    representation_items,
    record_of,
)
# endregion 🔖️Imports


# region 🔖️Measurement
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


# endregion 🔖️Measurement
