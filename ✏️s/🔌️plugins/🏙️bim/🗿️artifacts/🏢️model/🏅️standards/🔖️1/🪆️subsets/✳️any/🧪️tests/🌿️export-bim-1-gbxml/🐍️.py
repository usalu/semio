#!/usr/bin/env python3
"""🌿️ Third-party ORACLE (lxml + shapely + numpy) for the gbXML 7.03 export of the BIM thermal envelope.

lxml (libxml2), shapely (GEOS) and numpy have never seen this repository's writer. They open the committed files `🧫️fixtures/🌿️gbxml/<case>/gbxml.xml` and:

* parse each as namespaced XML and require the gbXML root with version 7.03 and metric units; when the official schema `🧫️fixtures/🌿️gbxml/📜️schema/GreenBuildingXML_Ver7.03.xsd` is committed the document is
  validated with `lxml.etree.XMLSchema` (the structural audit below runs either way);
* check ids (`xs:ID` syntax, uniqueness) and that every `...IdRef` resolves, units and ranges (U-values positive, g-values in 0..1, azimuth in 0..360, tilt in 0..180 and consistent with the surface type,
  heating set point not above the cooling one, areas and volumes positive), one `AdjacentSpaceId` for an outer surface and two (different spaces of one building) for a shared partition, and that a partition is
  described once (no two surfaces of one pair of spaces on the same plane that overlap);
* recompute every surface from its `PlanarGeometry` with numpy (Newell normal, area, azimuth and tilt) and compare with the `RectangularGeometry` the file states, lay the polygon into its own plane with shapely,
  require that its openings lie inside it and do not overlap, and that the corner and size of the rectangle and the position of every opening in it follow from the polygons;
* close every space: the outward area vectors of its surfaces sum to zero and the divergence theorem gives back the `Volume`, and the floors add up to the `Area`;
* when the inference oracle committed the table of the model, regroup the file by space, kind, boundary, neighbour and compass sector and compare area (polygon minus openings) and heat loss (U x area) with it.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🌿️gbxml>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🌿️gbxml>     # rewrite the measured tables from the committed files

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🌿️gbxml/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import math
import re
import sys
from pathlib import Path

import numpy as np
import shapely
from lxml import etree
from shapely.geometry import Polygon

# endregion 🔖️Imports


# region 🔖️Vocabulary
GBXML = "http://www.gbxml.org/schema"
NS = {"g": GBXML}
XS_ID = re.compile(r"^[A-Za-z_][A-Za-z0-9_.\-]*$")
SURFACE_TYPES = {"InteriorWall", "ExteriorWall", "Roof", "InteriorFloor", "ExposedFloor", "UndergroundWall", "Ceiling", "SlabOnGrade", "UndergroundSlab", "Air"}
OUTER = {"ExteriorWall", "UndergroundWall", "Roof", "SlabOnGrade", "ExposedFloor"}
INTERIOR = {"InteriorWall", "InteriorFloor", "Ceiling"}
TILT = {"ExteriorWall": 90, "UndergroundWall": 90, "InteriorWall": 90, "Roof": 0, "Ceiling": 0, "SlabOnGrade": 180, "ExposedFloor": 180, "InteriorFloor": 180}
CONDITIONS = {"HeatedAndCooled", "HeatedOnly", "CooledOnly", "Unconditioned"}
OPENINGS = {"FixedWindow": "window", "OperableWindow": "window", "NonSlidingDoor": "door", "SlidingDoor": "door"}
IDREFS = [("AdjacentSpaceId", "spaceIdRef", "Space"), ("Space", "zoneIdRef", "Zone"), ("Space", "buildingStoreyIdRef", "BuildingStorey"), ("Surface", "constructionIdRef", "Construction"), ("Opening", "constructionIdRef", "Construction"),
          ("Opening", "windowTypeIdRef", "WindowType"), ("LayerId", "layerIdRef", "Layer"), ("MaterialId", "materialIdRef", "Material")]
TOLERANCE = 1e-6
GROUND, NEIGHBOUR = 0.6, 0.5


def q(name):
    """🔖️ The qualified name of a gbXML element."""
    return "{%s}%s" % (GBXML, name)


def text_of(element, name, default=None):
    """🔤️ The text of the first child `name`, or the default."""
    found = element.find(q(name))
    return default if found is None or found.text is None else found.text


def number_of(element, name, default=None):
    """🔢️ The number in the first child `name`, or the default."""
    value = text_of(element, name)
    return default if value is None else float(value)


def tag_name(element):
    return etree.QName(element).localname


# endregion 🔖️Vocabulary


# region 🔖️Geometry
def loop_of(geometry):
    """📍️ The vertices of the `PolyLoop` of a `PlanarGeometry` as an array."""
    return np.array([[float(c.text) for c in point.findall(q("Coordinate"))] for point in geometry.find(q("PolyLoop")).findall(q("CartesianPoint"))])


def area_vector(polygon):
    """🧮️ Twice the area vector of a planar polygon (the cross products of consecutive vertices about the first)."""
    base = polygon[0]
    return np.sum([np.cross(polygon[index] - base, polygon[index + 1] - base) for index in range(1, len(polygon) - 1)], axis=0)


def frame_of(normal):
    """🪟️ The frame of a surface seen from outside: up is the vertical in its plane (north for a horizontal one), right is up x normal."""
    vertical = np.array([0.0, 0.0, 1.0])
    up = vertical - normal * np.dot(vertical, normal)
    if np.linalg.norm(up) < 1e-9:
        up = np.array([0.0, 1.0, 0.0]) - normal * normal[1]
    up = up / np.linalg.norm(up)
    return np.cross(up, normal), up


def orientation(normal):
    """🧭️ Azimuth (clockwise from +Y, 0 for horizontal) and tilt (0 faces up) in degrees of a unit normal."""
    azimuth = 0.0 if math.hypot(normal[0], normal[1]) < 1e-9 else math.degrees(math.atan2(normal[0], normal[1])) % 360.0
    return azimuth, math.degrees(math.acos(max(-1.0, min(1.0, normal[2]))))


def local_polygon(polygon, origin, right, up):
    """📐️ A polygon laid into the plane of a surface: shapely coordinates along right and up from the origin."""
    return Polygon([(float(np.dot(point - origin, right)), float(np.dot(point - origin, up))) for point in polygon])


def sector(azimuth):
    return int(math.floor(((azimuth + 22.5) % 360.0) / 45.0)) % 8


# endregion 🔖️Geometry


# region 🔖️Reading
class Model:
    """📖️ A gbXML document read with lxml: its elements by id and the tables derived from them."""

    def __init__(self, document):
        self.root = etree.fromstring(document)
        self.problems = []
        self.by_id = {}
        for element in self.root.iter():
            if isinstance(element.tag, str) and element.get("id") is not None:
                identity = element.get("id")
                if not XS_ID.match(identity):
                    self.problems.append("%s: the id %r is no xs:ID" % (tag_name(element), identity))
                if identity in self.by_id:
                    self.problems.append("the id %r is used twice" % identity)
                self.by_id[identity] = element
        self.spaces = {e.get("id"): e for e in self.root.iter(q("Space"))}
        self.zones = {e.get("id"): e for e in self.root.findall(q("Zone"))}
        self.constructions = {e.get("id"): e for e in self.root.findall(q("Construction"))}
        self.layers = {e.get("id"): e for e in self.root.findall(q("Layer"))}
        self.materials = {e.get("id"): e for e in self.root.findall(q("Material"))}
        self.window_types = {e.get("id"): e for e in self.root.findall(q("WindowType"))}
        self.storeys = {e.get("id"): e for e in self.root.iter(q("BuildingStorey"))}
        self.surfaces = list(self.root.iter(q("Surface")))
        self.building_of = {}
        for building in self.root.iter(q("Building")):
            for space in building.findall(q("Space")):
                self.building_of[space.get("id")] = building.get("id")

    def cad(self, element):
        """🏷️ The `CADObjectId`s of an element."""
        return [item.text for item in element.findall(q("CADObjectId"))]

    def space_cad(self, identity):
        return self.cad(self.spaces[identity])[0]

    def construction_name_u(self, identity):
        if identity is None or identity not in self.constructions:
            return None, None
        construction = self.constructions[identity]
        return text_of(construction, "Name"), number_of(construction, "U-value")


# endregion 🔖️Reading


# region 🔖️Structure
def structure_problems(model):
    """🩺️ Root, ids, references, units, ranges and adjacency of the document."""
    problems = list(model.problems)
    root = model.root
    if root.tag != q("gbXML") or root.get("version") != "7.03":
        problems.append("the root is not a gbXML 7.03 document")
    for name, expected in [("temperatureUnit", "C"), ("lengthUnit", "Meters"), ("areaUnit", "SquareMeters"), ("volumeUnit", "CubicMeters"), ("useSIUnitsForResults", "true")]:
        if root.get(name) != expected:
            problems.append("%s is %r, not %r" % (name, root.get(name), expected))
    for owner, attribute, target in IDREFS:
        for element in root.iter(q(owner)):
            reference = element.get(attribute)
            if reference is not None and (reference not in model.by_id or tag_name(model.by_id[reference]) != target):
                problems.append("%s %s=%r does not resolve to a %s" % (owner, attribute, reference, target))
    for space_id, space in model.spaces.items():
        area, volume = number_of(space, "Area"), number_of(space, "Volume")
        if space.get("conditionType") not in CONDITIONS:
            problems.append("%s: conditionType %r" % (space_id, space.get("conditionType")))
        if not (area and area > 0 and volume and volume > 0):
            problems.append("%s: area %s and volume %s must be positive" % (space_id, area, volume))
        elif not 0.5 < volume / area < 20:
            problems.append("%s: the clear height %s is implausible" % (space_id, volume / area))
        for name in ("LightPowerPerArea", "EquipPowerPerArea", "OAFlowPerArea", "PeopleNumber"):
            value = number_of(space, name)
            if value is not None and value < 0:
                problems.append("%s: %s is negative" % (space_id, name))
    for zone_id, zone in model.zones.items():
        heating, cooling = number_of(zone, "DesignHeatT"), number_of(zone, "DesignCoolT")
        if heating is not None and cooling is not None and heating > cooling:
            problems.append("%s: heating set point %s above cooling %s" % (zone_id, heating, cooling))
        for value in (heating, cooling):
            if value is not None and not -30 < value < 50:
                problems.append("%s: set point %s is no room temperature in degrees Celsius" % (zone_id, value))
    for identity, construction in model.constructions.items():
        value = number_of(construction, "U-value")
        if value is None or not 0 < value < 20:
            problems.append("%s: U-value %s out of range" % (identity, value))
    for identity, window_type in model.window_types.items():
        u_value, g_value = number_of(window_type, "U-value"), number_of(window_type, "SolarHeatGainCoeff")
        if u_value is not None and not 0 < u_value < 20:
            problems.append("%s: U-value %s out of range" % (identity, u_value))
        if g_value is not None and not 0 <= g_value <= 1:
            problems.append("%s: g-value %s out of range" % (identity, g_value))
    for identity, material in model.materials.items():
        thickness, conductivity, resistance = number_of(material, "Thickness"), number_of(material, "Conductivity"), number_of(material, "R-value")
        if not (thickness and thickness > 0 and conductivity and conductivity > 0) or abs(resistance - thickness / conductivity) > 1e-8:
            problems.append("%s: thickness %s, conductivity %s and R-value %s disagree" % (identity, thickness, conductivity, resistance))
    for surface in model.surfaces:
        identity, kind = surface.get("id"), surface.get("surfaceType")
        if kind not in SURFACE_TYPES:
            problems.append("%s: surfaceType %r" % (identity, kind))
        spaces = [item.get("spaceIdRef") for item in surface.findall(q("AdjacentSpaceId"))]
        if kind in OUTER and len(spaces) != 1:
            problems.append("%s: an outer %s needs exactly one AdjacentSpaceId, has %d" % (identity, kind, len(spaces)))
        if kind in INTERIOR and len(spaces) not in (1, 2):
            problems.append("%s: an interior %s needs one or two AdjacentSpaceIds" % (identity, kind))
        if len(spaces) == 2 and (spaces[0] == spaces[1] or model.building_of.get(spaces[0]) != model.building_of.get(spaces[1])):
            problems.append("%s: its two spaces are the same or in two buildings" % identity)
        if len(spaces) == 1 and kind in INTERIOR:
            problems.append("%s: the %s has a space on one side only (a partition without its partner)" % (identity, kind))
        if (surface.get("exposedToSun") == "true") != (kind in {"ExteriorWall", "Roof", "ExposedFloor"}):
            problems.append("%s: exposedToSun disagrees with %s" % (identity, kind))
        if not model.cad(surface):
            problems.append("%s: no CADObjectId" % identity)
    return problems


# endregion 🔖️Structure


# region 🔖️Geometry audit
def surface_geometry(model, surface):
    """📍️ The polygon, unit normal and frame of a surface; the rectangle and openings the file states."""
    polygon = loop_of(surface.find(q("PlanarGeometry")))
    vector = area_vector(polygon)
    normal = vector / np.linalg.norm(vector)
    right, up = frame_of(normal)
    return polygon, vector, normal, right, up


def geometry_problems(model):
    """🩺️ Recompute normals, areas, orientation, rectangles and openings from the polygons."""
    problems = []
    for surface in model.surfaces:
        identity, kind = surface.get("id"), surface.get("surfaceType")
        polygon, vector, normal, right, up = surface_geometry(model, surface)
        area = np.linalg.norm(vector) / 2.0
        rectangle = surface.find(q("RectangularGeometry"))
        azimuth, tilt = orientation(normal)
        stated = (number_of(rectangle, "Azimuth"), number_of(rectangle, "Tilt"))
        if abs(stated[0] - azimuth) > 1e-5 and not (azimuth < 1e-5 and abs(stated[0] - 360.0) < 1e-5) or abs(stated[1] - tilt) > 1e-5:
            problems.append("%s: Azimuth/Tilt %s differ from the polygon's %s" % (identity, stated, (azimuth, tilt)))
        if kind in TILT and abs(tilt - TILT[kind]) > 1e-5:
            problems.append("%s: tilt %.4f does not fit %s" % (identity, tilt, kind))
        if not 0 <= stated[0] < 360 or not 0 <= stated[1] <= 180:
            problems.append("%s: orientation %s out of range" % (identity, stated))
        corner = np.array([float(c.text) for c in rectangle.find(q("CartesianPoint")).findall(q("Coordinate"))])
        local = local_polygon(polygon, corner, right, up)
        low_r, low_v, high_r, high_v = local.bounds
        width, height = number_of(rectangle, "Width"), number_of(rectangle, "Height")
        if not local.is_valid or abs(local.area - area) > TOLERANCE * max(1.0, area):
            problems.append("%s: the polygon laid into its plane is invalid or has area %.9f, not %.9f" % (identity, local.area, area))
        if abs(low_r) > TOLERANCE or abs(low_v) > TOLERANCE or abs(high_r - width) > TOLERANCE or abs(high_v - height) > TOLERANCE:
            problems.append("%s: the rectangle %s x %s at %s is not the bounding rectangle of the polygon (%s)" % (identity, width, height, corner, local.bounds))
        placed = []
        for opening in surface.findall(q("Opening")):
            name = opening.get("id")
            hole = loop_of(opening.find(q("PlanarGeometry")))
            shape = local_polygon(hole, corner, right, up)
            if not local.buffer(TOLERANCE).covers(shape):
                problems.append("%s: lies outside its surface %s" % (name, identity))
            for other, other_shape in placed:
                if shape.intersection(other_shape).area > TOLERANCE:
                    problems.append("%s overlaps %s in %s" % (name, other, identity))
            placed.append((name, shape))
            offset = [float(c.text) for c in opening.find(q("RectangularGeometry")).find(q("CartesianPoint")).findall(q("Coordinate"))]
            if abs(offset[0] - shape.bounds[0]) > TOLERANCE or abs(offset[1] - shape.bounds[1]) > TOLERANCE:
                problems.append("%s: its position %s in the surface is not %s" % (name, offset, shape.bounds[:2]))
            opening_rectangle = opening.find(q("RectangularGeometry"))
            if abs(number_of(opening_rectangle, "Width") - (shape.bounds[2] - shape.bounds[0])) > TOLERANCE or abs(number_of(opening_rectangle, "Height") - (shape.bounds[3] - shape.bounds[1])) > TOLERANCE:
                problems.append("%s: its width/height are not the extent of its polygon" % name)
            if opening.get("openingType") not in OPENINGS:
                problems.append("%s: openingType %r" % (name, opening.get("openingType")))
            if OPENINGS.get(opening.get("openingType")) == "window" and opening.get("windowTypeIdRef") is None:
                problems.append("%s: a window without a windowTypeIdRef" % name)
    return problems


def closure_problems(model):
    """🩺️ Every space is a closed polyhedron: outward area vectors sum to zero, the divergence theorem gives the volume, the floors give the area."""
    sums, volumes, floors = {}, {}, {}
    for surface in model.surfaces:
        polygon, vector, normal, _, _ = surface_geometry(model, surface)
        centre = polygon.mean(axis=0)
        spaces = [item.get("spaceIdRef") for item in surface.findall(q("AdjacentSpaceId"))]
        for index, space in enumerate(spaces):
            sign = 1.0 if index == 0 else -1.0
            sums[space] = sums.get(space, np.zeros(3)) + sign * vector / 2.0
            volumes[space] = volumes.get(space, 0.0) + sign * float(np.dot(centre, vector / 2.0)) / 3.0
            if sign * normal[2] < -0.99:
                floors[space] = floors.get(space, 0.0) + np.linalg.norm(vector) / 2.0
    problems = []
    for identity, space in model.spaces.items():
        if np.linalg.norm(sums.get(identity, np.zeros(3))) > 1e-5:
            problems.append("%s: the outward area vectors of its surfaces sum to %s, not zero (the envelope is not closed)" % (identity, sums.get(identity)))
            continue
        if abs(volumes.get(identity, 0.0) - number_of(space, "Volume")) > 1e-5 * number_of(space, "Volume"):
            problems.append("%s: the polygons enclose %.6f m3, the file says %.6f" % (identity, volumes.get(identity, 0.0), number_of(space, "Volume")))
        if abs(floors.get(identity, 0.0) - number_of(space, "Area")) > 1e-5 * number_of(space, "Area"):
            problems.append("%s: the floors add up to %.6f m2, the file says %.6f" % (identity, floors.get(identity, 0.0), number_of(space, "Area")))
    return problems


def duplicate_problems(model):
    """🩺️ A partition between two spaces is described once: two surfaces of one pair of spaces never overlap on one plane."""
    problems = []
    pairs = {}
    for surface in model.surfaces:
        spaces = tuple(sorted(item.get("spaceIdRef") for item in surface.findall(q("AdjacentSpaceId"))))
        if len(spaces) == 2:
            pairs.setdefault(spaces, []).append(surface)
    for spaces, surfaces in pairs.items():
        for index, one in enumerate(surfaces):
            for other in surfaces[index + 1:]:
                a, b = surface_geometry(model, one), surface_geometry(model, other)
                if abs(abs(np.dot(a[2], b[2])) - 1.0) > 1e-6:
                    continue
                shape = local_polygon(b[0], a[0][0], a[3], a[4])
                if local_polygon(a[0], a[0][0], a[3], a[4]).intersection(shape).area > 1e-6:
                    problems.append("%s and %s describe the same partition of %s twice" % (one.get("id"), other.get("id"), spaces))
    return problems


# endregion 🔖️Geometry audit


# region 🔖️Tables
def measure(model):
    """📊️ The table of a document (the same table the subject reports from its plan)."""
    spaces = {}
    for identity, space in model.spaces.items():
        zone = model.zones.get(space.get("zoneIdRef"))
        row = {
            "name": text_of(space, "Name"),
            "storey": text_of(model.storeys[space.get("buildingStoreyIdRef")], "Name"),
            "zone": "" if zone is None else text_of(zone, "Name"),
            "condition": space.get("conditionType"),
            "area": number_of(space, "Area"),
            "volume": number_of(space, "Volume"),
        }
        for key, name, owner in [("heating", "DesignHeatT", zone), ("cooling", "DesignCoolT", zone), ("people", "PeopleNumber", space), ("lighting", "LightPowerPerArea", space), ("equipment", "EquipPowerPerArea", space), ("outdoor_air", "OAFlowPerArea", space)]:
            value = None if owner is None else number_of(owner, name)
            if value is not None:
                row[key] = value
        spaces[model.cad(space)[0]] = row
    surfaces, by_type, opening_area = {}, {}, 0.0
    for surface in model.surfaces:
        polygon, vector, normal, right, up = surface_geometry(model, surface)
        area = float(np.linalg.norm(vector) / 2.0)
        rectangle = surface.find(q("RectangularGeometry"))
        name, u_value = model.construction_name_u(surface.get("constructionIdRef"))
        openings = {}
        for opening in surface.findall(q("Opening")):
            hole = loop_of(opening.find(q("PlanarGeometry")))
            hole_area = float(np.linalg.norm(area_vector(hole)) / 2.0)
            opening_rectangle = opening.find(q("RectangularGeometry"))
            row = {
                "kind": opening.get("openingType"),
                "area": hole_area,
                "width": number_of(opening_rectangle, "Width"),
                "height": number_of(opening_rectangle, "Height"),
                "offset": [float(c.text) for c in opening_rectangle.find(q("CartesianPoint")).findall(q("Coordinate"))],
            }
            if opening.get("windowTypeIdRef") is not None:
                row["window_type"] = text_of(model.window_types[opening.get("windowTypeIdRef")], "Name")
            if opening.get("constructionIdRef") is not None:
                row["construction"] = model.construction_name_u(opening.get("constructionIdRef"))[0]
            openings[model.cad(opening)[0]] = row
            opening_area += hole_area
        row = {
            "cad": model.cad(surface),
            "kind": surface.get("surfaceType"),
            "exposed": surface.get("exposedToSun") == "true",
            "spaces": [model.space_cad(item.get("spaceIdRef")) for item in surface.findall(q("AdjacentSpaceId"))],
            "azimuth": number_of(rectangle, "Azimuth"),
            "tilt": number_of(rectangle, "Tilt"),
            "width": number_of(rectangle, "Width"),
            "height": number_of(rectangle, "Height"),
            "area": area,
            "openings": openings,
        }
        if name is not None:
            row["construction"], row["u_value"] = name, u_value
        surfaces[row["cad"][0]] = row
        by_type[row["kind"]] = by_type.get(row["kind"], 0.0) + area
    constructions = {}
    for identity, construction in model.constructions.items():
        layers = []
        for item in construction.findall(q("LayerId")):
            material = model.layers[item.get("layerIdRef")].find(q("MaterialId")).get("materialIdRef")
            layers.append(text_of(model.materials[material], "Name"))
        constructions[text_of(construction, "Name")] = {"u_value": number_of(construction, "U-value"), "layers": layers}
    window_types = {}
    for identity, window_type in model.window_types.items():
        row = {"description": text_of(window_type, "Description", "")}
        for key, name in (("u_value", "U-value"), ("g_value", "SolarHeatGainCoeff")):
            value = number_of(window_type, name)
            if value is not None:
                row[key] = value
        window_types[text_of(window_type, "Name")] = row
    counts = {
        "campuses": len(model.root.findall(q("Campus"))),
        "buildings": len(list(model.root.iter(q("Building")))),
        "storeys": len(model.storeys),
        "spaces": len(model.spaces),
        "zones": len(model.zones),
        "surfaces": len(model.surfaces),
        "openings": sum(len(row["openings"]) for row in surfaces.values()),
        "constructions": len(model.constructions),
        "layers": len(model.layers),
        "materials": len(model.materials),
        "window_types": len(model.window_types),
    }
    return {"counts": counts, "spaces": spaces, "surfaces": surfaces, "constructions": constructions, "window_types": window_types, "totals": {"by_type": by_type, "opening_area": opening_area}}


# endregion 🔖️Tables


# region 🔖️Inference differential
def groups_from_file(model, snapshot):
    """🌡️ The groups `kind|boundary|neighbour|sector -> {area, loss}` of every space, regrouped from the file alone (the keys and the F-free loss `U x A` of the inference oracle)."""
    conditions = snapshot.get("space_conditions", {})

    def climate(space):
        row = conditions.get(space) or {}
        return row.get("heating_setpoint"), row.get("cooling_setpoint")

    groups = {}

    def add(space, key, area, u_value):
        group = groups.setdefault(space, {}).setdefault(key, {"area": 0.0, "loss": 0.0})
        group["area"] += area
        group["loss"] = None if group["loss"] is None or u_value is None else group["loss"] + u_value * area

    for surface in model.surfaces:
        kind = surface.get("surfaceType")
        polygon, vector, normal, _, _ = surface_geometry(model, surface)
        spaces = [model.space_cad(item.get("spaceIdRef")) for item in surface.findall(q("AdjacentSpaceId"))]
        _, u_value = model.construction_name_u(surface.get("constructionIdRef"))
        gross = float(np.linalg.norm(vector) / 2.0)
        azimuth, tilt = orientation(normal)
        for index, space in enumerate(spaces):
            other = spaces[1 - index] if len(spaces) == 2 else ""
            boundary = "exterior" if kind in ("ExteriorWall", "Roof", "ExposedFloor") else "ground" if kind in ("UndergroundWall", "SlabOnGrade") else ("adiabatic" if climate(space) == climate(other) else "adjacent")
            horizontal = tilt < 1e-5 or abs(tilt - 180.0) < 1e-5
            facing = (azimuth + (180.0 if index else 0.0)) % 360.0
            if horizontal:
                down = (abs(tilt - 180.0) < 1e-5) != bool(index)
                label, where = ("floor" if down else "ceiling"), None
            else:
                label, where = "wall", facing
            cut = sum(float(np.linalg.norm(area_vector(loop_of(opening.find(q("PlanarGeometry"))))) / 2.0) for opening in surface.findall(q("Opening")))
            net = gross if boundary == "ground" else max(gross - cut, 0.0)
            key = "%s|%s|%s|%s" % (label, boundary, other, "-" if where is None else sector(where))
            if net > 1e-12:
                add(space, key, net, u_value)
            for opening in surface.findall(q("Opening")):
                hole = float(np.linalg.norm(area_vector(loop_of(opening.find(q("PlanarGeometry"))))) / 2.0)
                kind_name = OPENINGS[opening.get("openingType")]
                if opening.get("windowTypeIdRef") is not None:
                    opening_u = number_of(model.window_types[opening.get("windowTypeIdRef")], "U-value")
                else:
                    opening_u = model.construction_name_u(opening.get("constructionIdRef"))[1]
                add(space, "%s|%s|%s|%s" % (kind_name, boundary, other, sector(facing)), hole, opening_u)
    return groups


def differential_problems(model, snapshot, expected):
    """🩺️ The regrouped file equals the committed inference table: every group's area and heat loss, and no group on either side alone."""
    problems = []
    groups = groups_from_file(model, snapshot)
    for space, row in expected["spaces"].items():
        mine = groups.get(space, {})
        for key in sorted(set(mine) | set(row["groups"])):
            want, have = row["groups"].get(key), mine.get(key)
            if want is None or have is None:
                problems.append("%s: group %s is only in the %s" % (space, key, "file" if want is None else "inference table"))
                continue
            if abs(want["area"] - have["area"]) > TOLERANCE:
                problems.append("%s %s: area %.9f in the file, %.9f in the inference table" % (space, key, have["area"], want["area"]))
            if "loss" in want and (have["loss"] is None or abs(want["loss"] - have["loss"]) > 1e-5):
                problems.append("%s %s: loss %s in the file, %s in the inference table" % (space, key, have["loss"], want["loss"]))
        written = {model.cad(element)[0]: element for element in model.spaces.values()}
        if space not in written or abs(row["floor_area"] - number_of(written[space], "Area")) > TOLERANCE or abs(row["volume"] - number_of(written[space], "Volume")) > TOLERANCE:
            problems.append("%s: floor area or volume differs from the inference table" % space)
    return problems


# endregion 🔖️Inference differential


# region 🔖️Schema
def schema_problems(root, document):
    """📜️ Validation with the official XSD when it is committed under `🧫️fixtures/🌿️gbxml/📜️schema`."""
    schema_path = root / "📜️schema" / "GreenBuildingXML_Ver7.03.xsd"
    if not schema_path.exists():
        return None
    schema = etree.XMLSchema(etree.parse(str(schema_path)))
    parsed = etree.fromstring(document)
    return [] if schema.validate(parsed) else [str(error) for error in schema.error_log]


# endregion 🔖️Schema


# region 🔖️Audit
def audit(model, snapshot=None, expected=None):
    """🩺️ Every problem of one document."""
    problems = structure_problems(model)
    later = [geometry_problems, duplicate_problems, closure_problems]
    if snapshot is not None and expected is not None:
        later.append(lambda found: differential_problems(found, snapshot, expected))
    for check in later:
        try:
            problems += check(model)
        except (KeyError, AttributeError, IndexError, TypeError, ValueError) as error:
            problems.append("%s could not read the document: %r" % (getattr(check, "__name__", "the differential"), error))
    return problems


# endregion 🔖️Audit


# region 🔖️Handlers
def export_handler(ctx):
    """🌿️ Oracle answer: the table of the committed file, after the audit."""
    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    document = ctx.input_bytes(next(uri for uri in uris if uri.endswith(".xml")))
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "📸️snapshot" in uri)).decode("utf-8"))
    expected = next((json.loads(ctx.input_bytes(uri).decode("utf-8")) for uri in uris if "💡️inference" in uri), None)
    model = Model(document)
    problems = audit(model, snapshot, expected)
    if problems:
        raise AssertionError("; ".join(problems))
    table = measure(model)
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    built = Adapter("python")
    for identity in ("export-gbxml-box", "export-gbxml-zoning", "export-gbxml-stack", "export-gbxml-house"):
        built = built.oracle(identity, export_handler)
    return built


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed files against the committed tables; `write` rewrites the measured tables from the files."""
    command, root = arguments[0], Path(arguments[1])
    problems = []
    schema_seen = False
    for case in sorted(path for path in root.iterdir() if path.is_dir() and (path / "gbxml.xml").exists()):
        document = (case / "gbxml.xml").read_bytes()
        own = case / "📸️snapshot" / "🔣️.json"
        shared = root.parent / "💡️inferences" / "🌡️energy-envelope" / case.name / "📸️snapshot" / "🔣️.json"
        snapshot = json.loads((own if own.exists() else shared).read_text(encoding="utf-8"))
        table_path = root.parent / "💡️inferences" / "🌡️energy-envelope" / case.name / "💡️inference" / "🌡️energy-envelope" / "🔣️.json"
        expected = json.loads(table_path.read_text(encoding="utf-8")) if table_path.exists() else None
        model = Model(document)
        found = audit(model, snapshot, expected)
        schema = schema_problems(root, document)
        schema_seen = schema_seen or schema is not None
        found += schema or []
        problems += ["%s: %s" % (case.name, item) for item in found]
        table = measure(model)
        path = case / "🔬️measure" / "🔣️.json"
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote the table of %d spaces and %d surfaces%s" % (case.name, len(table["spaces"]), len(table["surfaces"]), " (differential against the inference table)" if expected else ""))
        elif json.loads(path.read_text(encoding="utf-8")) != json.loads(json.dumps(table)):
            problems.append("%s: the committed table differs from the measurement of the committed file" % case.name)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (lxml %s, shapely %s, numpy %s, XSD %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", etree.LXML_VERSION, shapely.__version__, np.__version__, "validated" if schema_seen else "not committed, structural audit only"))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
