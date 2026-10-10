#!/usr/bin/env python3
"""Prototype of the gbXML export for the `box` case of the energy-envelope fixtures, written with lxml and numpy only (no repository code).

It builds the envelope of the one-room box by hand (inner room 3.7 x 2.7 x 3 m, a window in the south wall, a door in the north wall, no roof construction), writes it the way the Rust writer is
designed to (Campus/Building/Space, Surface with RectangularGeometry/PlanarGeometry, Opening, Construction/Layer/Material, WindowType, Zone, polygons turned by true north minus the rotation of the
building) and runs the oracle of the draft (`gbxml_oracle.py`) on it: this proves the audit, the table and the regrouping against the committed inference table before the Rust writer exists.

    python r12-w2-wp20-energy-io-prototype.py <fixtures root 🧫️fixtures> <oracle file> <out dir>
"""
import importlib.util
import json
import math
import sys
from pathlib import Path

import numpy as np
from lxml import etree

NS = "http://www.gbxml.org/schema"


def el(parent, name, text=None, **attrs):
    node = etree.SubElement(parent, "{%s}%s" % (NS, name), {key: str(value) for key, value in attrs.items()})
    if text is not None:
        node.text = text if isinstance(text, str) else repr(round(float(text), 9))
    return node


def turned(point, bearing):
    s, c = math.sin(bearing), math.cos(bearing)
    return [round(point[0] * c + point[1] * s, 9), round(-point[0] * s + point[1] * c, 9), round(point[2], 9)]


def newell(polygon):
    base = np.array(polygon[0])
    return np.sum([np.cross(np.array(polygon[i]) - base, np.array(polygon[i + 1]) - base) for i in range(1, len(polygon) - 1)], axis=0)


def frame(normal):
    up = np.array([0, 0, 1.0]) - normal * normal[2]
    if np.linalg.norm(up) < 1e-9:
        up = np.array([0, 1.0, 0]) - normal * normal[1]
    up /= np.linalg.norm(up)
    return np.cross(up, normal), up


def rect(polygon, right, up):
    base = np.array(polygon[0])
    local = [(np.dot(np.array(p) - base, right), np.dot(np.array(p) - base, up)) for p in polygon]
    lo = [min(l[i] for l in local) for i in (0, 1)]
    hi = [max(l[i] for l in local) for i in (0, 1)]
    corner = base + right * lo[0] + up * lo[1]
    return [round(float(x), 9) for x in corner], round(hi[0] - lo[0], 9), round(hi[1] - lo[1], 9), lo, hi


def polyloop(parent, polygon):
    geometry = el(parent, "PlanarGeometry")
    loop = el(geometry, "PolyLoop")
    for point in polygon:
        cartesian = el(loop, "CartesianPoint")
        for value in point:
            el(cartesian, "Coordinate", value)


def build(snapshot):
    bearing = snapshot["sites"]["site"]["true_north"] - snapshot["buildings"]["bldg"]["rotation"]
    x0, x1, y0, y1, h = 0.15, 3.85, 0.15, 2.85, 3.0
    walls = {
        "sp/w1": ("ExteriorWall", [(x0, y0, 0), (x1, y0, 0), (x1, y0, h), (x0, y0, h)]),
        "sp/w2": ("ExteriorWall", [(x1, y0, 0), (x1, y1, 0), (x1, y1, h), (x1, y0, h)]),
        "sp/w3": ("ExteriorWall", [(x1, y1, 0), (x0, y1, 0), (x0, y1, h), (x1, y1, h)]),
        "sp/w4": ("ExteriorWall", [(x0, y1, 0), (x0, y0, 0), (x0, y0, h), (x0, y1, h)]),
        "sp/f1": ("SlabOnGrade", [(x0, y0, 0), (x0, y1, 0), (x1, y1, 0), (x1, y0, 0)]),
        "sp/c1": ("Roof", [(x0, y0, h), (x1, y0, h), (x1, y1, h), (x0, y1, h)]),
    }
    openings = {"sp/w1": [("o-win", "FixedWindow", [(1.4, y0, 0.9), (2.6, y0, 0.9), (2.6, y0, 2.1), (1.4, y0, 2.1)])], "sp/w3": [("o-door", "NonSlidingDoor", [(3.5, y1, 0), (2.5, y1, 0), (2.5, y1, 2.1), (3.5, y1, 2.1)])]}
    root = etree.Element("{%s}gbXML" % NS, nsmap={None: NS}, version="7.03", useSIUnitsForResults="true", temperatureUnit="C", lengthUnit="Meters", areaUnit="SquareMeters", volumeUnit="CubicMeters")
    campus = el(root, "Campus", id="campus-site")
    el(el(campus, "Location"), "Name", "Plot")
    building = el(campus, "Building", id="building-bldg")
    el(building, "Name", "Building")
    el(building, "Area", 9.99)
    el(el(building, "BuildingStorey", id="storey-st-0"), "Name", "Ground")
    space = el(building, "Space", id="space-sp", zoneIdRef="zone-sp", conditionType="HeatedAndCooled", buildingStoreyIdRef="storey-st-0")
    el(space, "Name", "0.01 Room")
    el(space, "Area", 9.99)
    el(space, "Volume", 29.97)
    el(space, "PeopleNumber", 0.999, unit="NumberOfPeople")
    el(space, "LightPowerPerArea", 8, unit="WattPerSquareMeter")
    el(space, "EquipPowerPerArea", 12, unit="WattPerSquareMeter")
    el(space, "OAFlowPerArea", 1.5, unit="LPerSecPerSquareM")
    el(space, "CADObjectId", "sp")
    for cad, (kind, polygon) in walls.items():
        rotated = [turned(p, bearing) for p in polygon]
        vector = newell(rotated)
        normal = vector / np.linalg.norm(vector)
        right, up = frame(normal)
        azimuth = 0.0 if math.hypot(normal[0], normal[1]) < 1e-9 else math.degrees(math.atan2(normal[0], normal[1])) % 360
        tilt = math.degrees(math.acos(max(-1, min(1, normal[2]))))
        corner, width, height, lo, hi = rect(rotated, right, up)
        surface = el(campus, "Surface", id="surface-" + cad.replace("/", "_"), surfaceType=kind, exposedToSun=str(kind in ("ExteriorWall", "Roof")).lower())
        if kind == "ExteriorWall":
            surface.set("constructionIdRef", "construction-wt")
        if kind == "SlabOnGrade":
            surface.set("constructionIdRef", "construction-slab")
        el(surface, "Name", cad)
        el(surface, "AdjacentSpaceId", spaceIdRef="space-sp")
        geometry = el(surface, "RectangularGeometry")
        el(geometry, "Azimuth", round(azimuth, 9))
        corner_node = el(geometry, "CartesianPoint")
        for value in corner:
            el(corner_node, "Coordinate", value)
        el(geometry, "Tilt", round(tilt, 9))
        el(geometry, "Height", height)
        el(geometry, "Width", width)
        polyloop(surface, rotated)
        for name, opening_type, hole in openings.get(cad, []):
            hole_rotated = [turned(p, bearing) for p in hole]
            _, w2, h2, lo2, hi2 = rect(hole_rotated, right, up)
            local = [(np.dot(np.array(p) - np.array(corner), right), np.dot(np.array(p) - np.array(corner), up)) for p in hole_rotated]
            opening = el(surface, "Opening", id="opening-" + name, openingType=opening_type)
            if opening_type == "FixedWindow":
                opening.set("windowTypeIdRef", "windowtype-win")
            else:
                opening.set("constructionIdRef", "construction-door-dr")
            el(opening, "Name", name)
            og = el(opening, "RectangularGeometry")
            el(og, "Azimuth", round(azimuth, 9))
            pc = el(og, "CartesianPoint")
            el(pc, "Coordinate", round(min(l[0] for l in local), 9))
            el(pc, "Coordinate", round(min(l[1] for l in local), 9))
            el(og, "Tilt", 90)
            el(og, "Height", round(max(l[1] for l in local) - min(l[1] for l in local), 9))
            el(og, "Width", round(max(l[0] for l in local) - min(l[0] for l in local), 9))
            polyloop(opening, hole_rotated)
            el(opening, "CADObjectId", name)
        el(surface, "CADObjectId", cad)
    wt = el(root, "WindowType", id="windowtype-win")
    el(wt, "Name", "Window")
    el(wt, "Description", "frame fraction 0.25")
    el(wt, "U-value", 1.1, unit="WPerSquareMeterK")
    el(wt, "SolarHeatGainCoeff", 0.5, unit="Fraction", solarIncidentAngle="0")
    layers, thickness = [], {"m": 0.2, "ins": 0.1}
    construction = el(root, "Construction", id="construction-wt")
    u_wall = 1.0 / (0.13 + 0.2 / 0.8 + 0.1 / 0.04 + 0.04)
    el(construction, "Name", "Insulated brick U=%s" % round(u_wall, 9))
    el(construction, "U-value", round(u_wall, 9), unit="WPerSquareMeterK")
    for material in ("m", "ins"):
        el(construction, "LayerId", layerIdRef="layer-mat-%s" % material)
    slab = el(root, "Construction", id="construction-slab")
    u_slab = 1.0 / (0.17 + 0.25 / 0.8)
    el(slab, "Name", "Slab 250 U=%s" % round(u_slab, 9))
    el(slab, "U-value", round(u_slab, 9), unit="WPerSquareMeterK")
    el(slab, "LayerId", layerIdRef="layer-mat-m")
    door = el(root, "Construction", id="construction-door-dr")
    el(door, "Name", "Door U=1.8")
    el(door, "U-value", 1.8, unit="WPerSquareMeterK")
    for material in ("m", "ins"):
        node = snapshot["materials"][material]
        layer = el(root, "Layer", id="layer-mat-%s" % material)
        el(layer, "MaterialId", materialIdRef="mat-%s" % material)
    for material in ("m", "ins"):
        node = snapshot["materials"][material]
        mat = el(root, "Material", id="mat-%s" % material)
        el(mat, "Name", "%s %s mm" % (node["name"], thickness[material] * 1000))
        el(mat, "R-value", round(thickness[material] / node["conductivity"], 9), unit="SquareMeterKPerW")
        el(mat, "Thickness", thickness[material], unit="Meters")
        el(mat, "Conductivity", node["conductivity"], unit="WPerMeterK")
        el(mat, "Density", node["density"], unit="KgPerCubicM")
        el(mat, "SpecificHeat", node["specific_heat"], unit="JPerKgK")
    zone = el(root, "Zone", id="zone-sp")
    el(zone, "Name", "0.01 Room")
    el(zone, "DesignHeatT", 20, unit="C")
    el(zone, "DesignCoolT", 26, unit="C")
    el(zone, "CADObjectId", "sp")
    return etree.tostring(root, xml_declaration=True, encoding="UTF-8", pretty_print=True)


def main(arguments):
    fixtures, oracle_path, out = Path(arguments[0]), Path(arguments[1]), Path(arguments[2])
    spec = importlib.util.spec_from_file_location("gbxml_oracle", oracle_path)
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    case = fixtures / "💡️inferences" / "🌡️energy-envelope" / "🏠️box"
    snapshot = json.loads((case / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    expected = json.loads((case / "💡️inference" / "🌡️energy-envelope" / "🔣️.json").read_text(encoding="utf-8"))
    document = build(snapshot)
    out.mkdir(parents=True, exist_ok=True)
    (out / "box.xml").write_bytes(document)
    model = oracle.Model(document)
    problems = oracle.audit(model, snapshot, expected)
    for problem in problems:
        print("[FAIL] %s" % problem)
    table = oracle.measure(model)
    print("prototype box: %d problem(s); %d surfaces, %d openings" % (len(problems), table["counts"]["surfaces"], table["counts"]["openings"]))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
