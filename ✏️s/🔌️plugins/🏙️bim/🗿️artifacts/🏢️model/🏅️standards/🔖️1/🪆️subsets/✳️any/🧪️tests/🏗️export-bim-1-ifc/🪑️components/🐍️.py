"""🪑️ The component audit: the family solids, the instance frames, the bounds and sections of components and their base quantities and overrides."""

from __future__ import annotations

# region 🔖️Imports
import copy
import json
import math
import sys
import ifcopenshell.util.element
import importlib.util
from pathlib import Path

from export_bim_ifc_kernel import (
    identity,
    settings,
    kernel_volume,
    kernel_bounds,
    representation_items,
    record_of,
    quantities_of,
    close,
)
# endregion 🔖️Imports


# region 🔖️Measurement
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
# endregion 🔖️Measurement
