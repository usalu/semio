#!/usr/bin/env python3
"""🧊️ Third-party ORACLE (IfcOpenShell geometry kernel) for the straight walls of `🧱️wall-layout`.

The sibling case `../🪜️infer-bim-1-levels-and-wall-heights` commits, per wall, a resolved `base_z`, `top_z`,
`height`, `thickness` and `length`. This file hands exactly those numbers to a solid modeller that has
never seen this repository and measures what it builds:

* every STRAIGHT wall becomes an `IfcWall` whose body is the committed join-trimmed `footprint` ring extruded by the
  committed `height` (`IfcArbitraryClosedProfileDef` + `add_profile_representation`), lifted to its committed `base_z`;
* the whole model is serialised to ISO 10303-21 text, re-opened (so the numbers come from bytes, not from
  the in-memory graph) and tessellated by IfcOpenShell 0.8.4.post1's own C++ kernel;
* the measured `z_min`, `z_max` and `volume` of the triangulation must equal the committed `base_z`,
  `top_z` and `volume` (footprint area times height). Openings are covered by the second half of this file (the `🧊️element-solids` cases).

Arc walls have no `IfcWall` standard-case form and are left to `shapely` in the sibling case.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../🪜️infer-bim-1-levels-and-wall-heights/🐍️.py — the table this file measures
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.api.aggregate
import ifcopenshell.api.context
import ifcopenshell.api.geometry
import ifcopenshell.api.project
import ifcopenshell.api.root
import ifcopenshell.api.spatial
import ifcopenshell.api.unit
import ifcopenshell.geom
import ifcopenshell.util.shape
import ifcopenshell.util.unit
import numpy

# endregion 🔖️Imports


# region 🔖️Kernel
TOLERANCE = 1e-9
"""⚖️ Absolute tolerance between the kernel's measurement and the committed table."""


def footprint_rings(snapshot, layouts):
    """📏️ The join-trimmed footprint ring `[(x, y), ...]` of every wall whose axis is a line and whose footprint edges are all straight, by wall id."""
    return {
        wall_id: [(vertex["point"]["x"], vertex["point"]["y"]) for vertex in layouts[wall_id]["footprint"]]
        for wall_id, wall in snapshot["walls"].items()
        if "Line" in wall["axis"] and len(layouts[wall_id].get("footprint", [])) >= 3 and all(abs(vertex["bulge"]) < 1e-12 for vertex in layouts[wall_id]["footprint"])
    }


def measure(snapshot, layouts):
    """🧊️ Extrudes, serialises, re-opens and tessellates the committed footprint of every straight wall: `{wall id: {base_z, top_z, volume}}`."""
    model = ifcopenshell.api.project.create_file(version="IFC4")
    project = ifcopenshell.api.root.create_entity(model, ifc_class="IfcProject", name=snapshot["project"]["name"])
    ifcopenshell.api.unit.assign_unit(model)
    scale = ifcopenshell.util.unit.calculate_unit_scale(model)
    context = ifcopenshell.api.context.add_context(model, context_type="Model")
    body = ifcopenshell.api.context.add_context(model, context_type="Model", context_identifier="Body", target_view="MODEL_VIEW", parent=context)
    site = ifcopenshell.api.root.create_entity(model, ifc_class="IfcSite", name="site")
    building = ifcopenshell.api.root.create_entity(model, ifc_class="IfcBuilding", name="building")
    storey = ifcopenshell.api.root.create_entity(model, ifc_class="IfcBuildingStorey", name="storey")
    ifcopenshell.api.aggregate.assign_object(model, products=[site], relating_object=project)
    ifcopenshell.api.aggregate.assign_object(model, products=[building], relating_object=site)
    ifcopenshell.api.aggregate.assign_object(model, products=[storey], relating_object=building)
    for wall_id, ring in footprint_rings(snapshot, layouts).items():
        layout = layouts[wall_id]
        points = [model.createIfcCartesianPoint((x / scale, y / scale)) for x, y in ring]
        points.append(points[0])
        profile = model.createIfcArbitraryClosedProfileDef("AREA", None, model.createIfcPolyline(points))
        element = ifcopenshell.api.root.create_entity(model, ifc_class="IfcWall", name=wall_id)
        representation = ifcopenshell.api.geometry.add_profile_representation(model, context=body, profile=profile, depth=layout["height"])
        ifcopenshell.api.geometry.assign_representation(model, product=element, representation=representation)
        ifcopenshell.api.geometry.edit_object_placement(model, product=element, matrix=numpy.array([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, layout["base_z"]], [0.0, 0.0, 0.0, 1.0]]))
        ifcopenshell.api.spatial.assign_container(model, products=[element], relating_structure=storey)
    reopened = ifcopenshell.file.from_string(model.to_string())
    settings = ifcopenshell.geom.settings()
    settings.set("use-world-coords", True)
    result = {}
    for element in reopened.by_type("IfcWall"):
        shape = ifcopenshell.geom.create_shape(settings, element)
        geometry = shape.geometry
        vertices = ifcopenshell.util.shape.get_vertices(geometry)
        result[element.Name] = {
            "base_z": round(float(vertices[:, 2].min()), 12),
            "top_z": round(float(vertices[:, 2].max()), 12),
            "volume": round(float(ifcopenshell.util.shape.get_volume(geometry)), 12),
        }
    return result


def disagreements(measured, layouts):
    """⚖️ Every field where the kernel's measurement leaves the committed table."""
    problems = []
    for wall_id, row in measured.items():
        for field, actual in row.items():
            if abs(actual - layouts[wall_id][field]) > TOLERANCE * max(abs(layouts[wall_id][field]), 1.0):
                problems.append("%s.%s: ifcopenshell %.12g, committed %.12g" % (wall_id, field, actual, layouts[wall_id][field]))
    return problems


# endregion 🔖️Kernel


# region 🔖️Solids
SOLIDS_DIRECTORY = "🧊️element-solids"
"""📁️ The fixture folder (under `🧫️fixtures/💡️inferences`) that holds the `{snapshot, expected, meshes}` cases of `🧊️element-solids`."""

EPS = 1e-9
PANEL_THICKNESS = 0.024
GLASS_THICKNESS = 0.02
LEAF_THICKNESS = 0.04
KERNEL_TOLERANCE = 1e-7
"""⚖️ Relative tolerance between the kernel boolean result and the closed form."""


def level_of(snapshot, storey_id):
    """🪜️ `(elevation, top)` of a storey from the stacked heights of the storeys of its building at or above level 0."""
    storey = snapshot["storeys"][storey_id]
    below = [row for row in snapshot["storeys"].values() if row["building"] == storey["building"] and 0 <= row["level"] < storey["level"]]
    elevation = math.fsum(row["height"] for row in below)
    return elevation, elevation + storey["height"]


def vertical_of(snapshot, element):
    """🔝️ `(base_z, top_z)` of a wall or curtain wall from its storey, base offset and top constraint."""
    elevation, storey_top = level_of(snapshot, element["storey"])
    base = elevation + element["base_offset"]
    kind, body = next(iter(element["top"].items()))
    if kind == "Unconnected":
        return base, base + body["height"]
    if kind == "StoreyTop":
        return base, storey_top + body["offset"]
    return base, level_of(snapshot, body["storey"])[0] + body["offset"]


def faces_of(snapshot, wall):
    """↔️ `(left, right, thicknesses)`: face distances from the axis; layers run from the left face to the right face."""
    thicknesses = [layer["thickness"] for layer in snapshot["wall_types"][wall["wall_type"]]["layers"]]
    total = math.fsum(thicknesses)
    left = {"Center": total / 2.0, "Interior": 0.0, "Exterior": total}[wall["location"]]
    return left, total - left, thicknesses


def opening_cut(snapshot, opening):
    """✂️ `(s0, s1, z0, z1)` of an opening in its host development, from the override-else-type rule (the sill override replaces the type sill)."""
    kind, body = next(iter(opening["kind"].items()))
    if kind == "Window":
        window = snapshot["window_types"][body["window_type"]]
        width, high, sill = opening.get("width", window["width"]), opening.get("height", window["height"]), opening.get("sill_override", window["sill"])
    elif kind == "Door":
        door = snapshot["door_types"][body["door_type"]]
        width, high, sill = opening.get("width", door["width"]), opening.get("height", door["height"]), opening.get("sill_override", 0.0)
    else:
        width, high, sill = opening.get("width", body["width"]), opening.get("height", body["height"]), opening.get("sill_override", 0.0)
    return opening["offset"] - width / 2.0, opening["offset"] + width / 2.0, sill, sill + high


def valid_cuts(snapshot, host_id, length, height):
    """✂️ `{opening id: cut}` of the openings of a host that sit inside it and overlap no sibling."""
    cuts = {oid: opening_cut(snapshot, opening) for oid, opening in snapshot.get("openings", {}).items() if opening["host"] == host_id}
    inside = {oid: cut for oid, cut in cuts.items() if cut[0] >= -EPS and cut[1] <= length + EPS and cut[2] >= -EPS and cut[3] <= height + EPS and cut[1] - cut[0] > EPS and cut[3] - cut[2] > EPS}

    def overlap(a, b):
        return min(a[1], b[1]) - max(a[0], b[0]) > EPS and min(a[3], b[3]) - max(a[2], b[2]) > EPS

    return {oid: cut for oid, cut in inside.items() if not any(overlap(cut, other) for other_id, other in inside.items() if other_id != oid)}


def usable_cuts(cuts, low, high, height):
    """✂️ `(notches, holes)`: the cuts strictly inside `[low, high]` and below the top; a cut that starts on the base is a bottom notch."""
    fitting = [cut for cut in cuts.values() if cut[0] > low + EPS and cut[1] < high - EPS and cut[3] < height - EPS]
    return [cut for cut in fitting if cut[2] <= EPS], [cut for cut in fitting if cut[2] > EPS]


def box_measures(box):
    """📦️ `(volume, area, corners)` of an axis-aligned box `((x0, x1), (y0, y1), (z0, z1))`."""
    dx, dy, dz = box[0][1] - box[0][0], box[1][1] - box[1][0], box[2][1] - box[2][0]
    return dx * dy * dz, 2.0 * (dx * dy + dy * dz + dz * dx), [(x, y, z) for x in box[0] for y in box[1] for z in box[2]]


def bounds_of(points):
    """🧮️ `{min, max}` of 3D points, rounded to 12 digits."""
    return {"min": [round(min(p[i] for p in points), 12) for i in range(3)], "max": [round(max(p[i] for p in points), 12) for i in range(3)]}


def tips_of(wall):
    """📍️ The two axis end points of a straight wall."""
    line = wall["axis"]["Line"]
    return [(line[name]["x"], line[name]["y"]) for name in ("start", "end")]


def joins_of(snapshot, wall_id, wall):
    """🔗️ `(joined, shifts)`: whether an end meets another wall and the length each end gains (`0` mitered node, minus half the through thickness for a T butt)."""
    others = [(oid, other) for oid, other in snapshot["walls"].items() if oid != wall_id and other["storey"] == wall["storey"] and "Line" in other["axis"]]
    shifts, joined = [], False
    for point in tips_of(wall):
        shift = 0.0
        if any(math.dist(point, tip) < 1e-6 for _, other in others for tip in tips_of(other)):
            joined = True
        else:
            for _, other in others:
                a, b = tips_of(other)
                t = ((point[0] - a[0]) * (b[0] - a[0]) + (point[1] - a[1]) * (b[1] - a[1])) / ((b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2)
                foot = (a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]))
                if 0 < t < 1 and math.dist(point, foot) < 1e-6:
                    joined, shift = True, -math.fsum(layer["thickness"] for layer in snapshot["wall_types"][other["wall_type"]]["layers"]) / 2.0
        shifts.append(shift)
    return joined, shifts


def straight_wall(snapshot, wall_id, wall):
    """🧱️ Closed-form `volume` of a straight wall; `area` and `bounds` too when no end is joined."""
    start, end = tips_of(wall)
    length = math.dist(start, end)
    base, top = vertical_of(snapshot, wall)
    height = top - base
    left, right, thicknesses = faces_of(snapshot, wall)
    joined, shifts = joins_of(snapshot, wall_id, wall)
    notches, holes = usable_cuts(valid_cuts(snapshot, wall_id, length, height), 0.0, length, height)
    net_face = (length + math.fsum(shifts)) * height - math.fsum((c[1] - c[0]) * (c[3] - c[2]) for c in notches + holes)
    row = {"volume": round(math.fsum(t * net_face for t in thicknesses), 12)}
    if joined:
        return row
    area = 0.0
    for t in thicknesses:
        area += 2.0 * net_face + 2.0 * t * height + 2.0 * length * t
        area -= math.fsum((c[1] - c[0]) * t for c in notches)
        area += math.fsum(2.0 * ((c[1] - c[0]) + (c[3] - c[2])) * t for c in holes)
        area += math.fsum((2.0 * c[3] + (c[1] - c[0])) * t for c in notches)
    d = ((end[0] - start[0]) / length, (end[1] - start[1]) / length)
    n = (-d[1], d[0])
    corners = [(p[0] + n[0] * off, p[1] + n[1] * off, z) for p in (start, end) for off in (left, -right) for z in (base, top)]
    row.update({"area": round(area, 12), "bounds": bounds_of(corners)})
    return row


def arc_geometry(arc):
    """🌙️ `(centre, radius, start angle, sweep)` of an `Arc` axis (counter-clockwise for a positive bulge)."""
    start, end = (arc["start"]["x"], arc["start"]["y"]), (arc["end"]["x"], arc["end"]["y"])
    sweep = 4.0 * math.atan(arc["bulge"])
    chord = math.dist(start, end)
    radius = chord / (2.0 * math.sin(abs(sweep) / 2.0))
    mid = ((start[0] + end[0]) / 2.0, (start[1] + end[1]) / 2.0)
    normal = (-(end[1] - start[1]) / chord, (end[0] - start[0]) / chord)
    shift = radius * math.cos(sweep / 2.0)
    centre = (mid[0] + normal[0] * shift, mid[1] + normal[1] * shift)
    return centre, radius, math.atan2(start[1] - centre[1], start[0] - centre[0]), sweep


def arc_wall(snapshot, wall_id, wall):
    """🌙️ Closed-form measures of an unjoined arc wall with holes: exact for the true annular sector, met by the mesh within the chord tolerance."""
    centre, radius, first, sweep = arc_geometry(wall["axis"]["Arc"])
    length = radius * abs(sweep)
    base, top = vertical_of(snapshot, wall)
    height = top - base
    left, right, thicknesses = faces_of(snapshot, wall)
    notches, holes = usable_cuts(valid_cuts(snapshot, wall_id, length, height), 0.0, length, height)
    assert not notches, "arc walls with bottom notches are not part of this oracle"
    volume, area, offset = 0.0, 0.0, left
    for t in thicknesses:
        outer, inner = radius - (offset - t), radius - offset
        mid = (outer + inner) / 2.0
        volume += height * abs(sweep) / 2.0 * (outer**2 - inner**2)
        area += height * abs(sweep) * (outer + inner) + 2.0 * t * height + 2.0 * abs(sweep) * t * mid
        for c in holes:
            fraction, hole = (c[1] - c[0]) / radius, c[3] - c[2]
            volume -= fraction / 2.0 * (outer**2 - inner**2) * hole
            area += -fraction * (outer + inner) * hole + 2.0 * hole * t + 2.0 * fraction * t * mid
        offset -= t
    points = []
    for step in range(0, 20001):
        angle = first + sweep * step / 20000.0
        for side in (radius + left, radius - right):
            for z in (base, top):
                points.append((centre[0] + side * math.cos(angle), centre[1] + side * math.sin(angle), z))
    return {"volume": round(volume, 12), "area": round(area, 12), "bounds": bounds_of(points)}


def measured(boxes, origin=(0.0, 0.0), x_axis=(1.0, 0.0), y_axis=(0.0, 1.0)):
    """📦️ `{volume, area, bounds}` of disjoint local boxes under a rigid plan placement (`z` is absolute in the boxes)."""
    points = []
    for box in boxes:
        for x, y, z in box_measures(box)[2]:
            points.append((origin[0] + x_axis[0] * x + y_axis[0] * y, origin[1] + x_axis[1] * x + y_axis[1] * y, z))
    return {"volume": round(math.fsum(box_measures(box)[0] for box in boxes), 12), "area": round(math.fsum(box_measures(box)[1] for box in boxes), 12), "bounds": bounds_of(points)}


def window_boxes(window, width, height, centre, sill):
    """🪟️ The frame, muntin and glass boxes of a window in its local frame (`z` from the sill)."""
    frame = min(max(window["frame_width"], 0.0), width / 2.0, height / 2.0)
    half, depth = width / 2.0, max(window["frame_depth"], 0.0)
    lateral = (centre - depth / 2.0, centre + depth / 2.0)
    rise = (sill + frame, sill + height - frame)
    boxes = [((-half, half), lateral, (sill, sill + frame)), ((-half, half), lateral, (sill + height - frame, sill + height)), ((-half, -half + frame), lateral, rise), ((half - frame, half), lateral, rise)]
    inner = width - 2.0 * frame
    if window["panes"] > 0 and inner > EPS and height - 2.0 * frame > EPS:
        panes = window["panes"]
        muntin = min(frame / 2.0, inner / panes / 2.0)
        pane = (inner - (panes - 1) * muntin) / panes
        glass = min(GLASS_THICKNESS, depth)
        for k in range(panes):
            start = -half + frame + k * (pane + muntin)
            boxes.append(((start, start + pane), (centre - glass / 2.0, centre + glass / 2.0), rise))
            if k + 1 < panes:
                boxes.append(((start + pane, start + pane + muntin), lateral, rise))
    return boxes


def door_boxes(door, width, height, centre, sill):
    """🚪️ The frame and leaf boxes of a door in its local frame (`z` from the sill)."""
    frame = min(max(door["frame_width"], 0.0), width / 2.0, height / 2.0)
    half, depth = width / 2.0, max(door["frame_depth"], 0.0)
    lateral = (centre - depth / 2.0, centre + depth / 2.0)
    boxes = [((-half, half), lateral, (sill + height - frame, sill + height)), ((-half, -half + frame), lateral, (sill, sill + height - frame)), ((half - frame, half), lateral, (sill, sill + height - frame))]
    inner, count, leaf = width - 2.0 * frame, 2 if door["leaves"] == "Double" else 1, min(LEAF_THICKNESS, depth)
    for k in range(count):
        start = -half + frame + k * inner / count
        boxes.append(((start, start + inner / count), (centre - leaf / 2.0, centre + leaf / 2.0), (sill, sill + height - frame)))
    return boxes


def filler(snapshot, opening_id, opening):
    """🚪️ Closed-form measures of the window or door filler of an opening on a straight wall; `None` when there is none."""
    kind = next(iter(opening["kind"]))
    host = snapshot["walls"].get(opening["host"])
    if kind == "Void" or host is None or "Line" not in host["axis"]:
        return None
    start, end = tips_of(host)
    length = math.dist(start, end)
    base, top = vertical_of(snapshot, host)
    if opening_id not in valid_cuts(snapshot, opening["host"], length, top - base):
        return None
    s0, s1, z0, z1 = opening_cut(snapshot, opening)
    left, right, _ = faces_of(snapshot, host)
    facing = -1.0 if opening["flip_facing"] else 1.0
    d = ((end[0] - start[0]) / length, (end[1] - start[1]) / length)
    x_axis, y_axis = (d[0] * facing, d[1] * facing), (-d[1] * facing, d[0] * facing)
    origin = (start[0] + d[0] * opening["offset"], start[1] + d[1] * opening["offset"])
    centre = (left - right) / 2.0 * facing
    if kind == "Window":
        boxes = window_boxes(snapshot["window_types"][opening["kind"]["Window"]["window_type"]], s1 - s0, z1 - z0, centre, base + z0)
    else:
        boxes = door_boxes(snapshot["door_types"][opening["kind"]["Door"]["door_type"]], s1 - s0, z1 - z0, centre, base + z0)
    return measured(boxes, origin, x_axis, y_axis)


def curtain_wall(snapshot, curtain):
    """🪞️ Closed-form measures of a straight curtain wall: a mullion on every grid line plus a thin panel per cell."""
    start, end = tips_of(curtain)
    length = math.dist(start, end)
    base, top = vertical_of(snapshot, curtain)
    height = top - base
    across, depth = curtain["mullion"]["Rectangle"]["width"], curtain["mullion"]["Rectangle"]["depth"]

    def cells(extent, spacing):
        return 1 if spacing <= EPS else max(1, math.ceil(extent / spacing - EPS))

    def member(extent, count, k):
        centre = min(max(extent * k / count, across / 2.0), max(extent - across / 2.0, across / 2.0))
        return centre - across / 2.0, centre + across / 2.0

    columns, rows = cells(length, curtain["u_spacing"]), cells(height, curtain["v_spacing"])
    verticals = [member(length, columns, k) for k in range(columns + 1)]
    horizontals = [member(height, rows, k) for k in range(rows + 1)]
    lateral = (-depth / 2.0, depth / 2.0)
    boxes = [((a, b), lateral, (base, base + height)) for a, b in verticals]
    for a, b in horizontals:
        boxes += [((verticals[i][1], verticals[i + 1][0]), lateral, (base + a, base + b)) for i in range(columns) if verticals[i + 1][0] - verticals[i][1] > EPS]
    thickness = min(PANEL_THICKNESS, depth)
    for i in range(columns):
        for j in range(rows):
            x, z = (verticals[i][1], verticals[i + 1][0]), (base + horizontals[j][1], base + horizontals[j + 1][0])
            if x[1] - x[0] > EPS and z[1] - z[0] > EPS:
                boxes.append((x, (-thickness / 2.0, thickness / 2.0), z))
    d = ((end[0] - start[0]) / length, (end[1] - start[1]) / length)
    return measured(boxes, start, d, (-d[1], d[0]))


def expected_table(snapshot):
    """🧊️ `{element id: {volume, area?, bounds?}}` for every element of a case that is stated in closed form."""
    table = {}
    for wall_id, wall in sorted(snapshot.get("walls", {}).items()):
        table[wall_id] = straight_wall(snapshot, wall_id, wall) if "Line" in wall["axis"] else arc_wall(snapshot, wall_id, wall)
    for curtain_id, curtain in sorted(snapshot.get("curtain_walls", {}).items()):
        table[curtain_id] = curtain_wall(snapshot, curtain)
    for opening_id, opening in sorted(snapshot.get("openings", {}).items()):
        row = filler(snapshot, opening_id, opening)
        if row is not None:
            table[opening_id] = row
    return table


def union_audit(snapshot, table):
    """⚖️ `shapely` audit of a joined room: the unioned square-capped bands times the height equal the summed wall volumes."""
    from shapely.geometry import LineString
    from shapely.ops import unary_union

    bands = []
    for wall in snapshot["walls"].values():
        left, right, _ = faces_of(snapshot, wall)
        bands.append(LineString(tips_of(wall)).buffer((left + right) / 2.0, cap_style="square"))
    base, top = vertical_of(snapshot, next(iter(snapshot["walls"].values())))
    return unary_union(bands).area * (top - base), math.fsum(table[wall_id]["volume"] for wall_id in snapshot["walls"])


def kernel_walls(snapshot):
    """🧊️ IfcOpenShell kernel `{wall id: {volume, z_min, z_max}}` of every straight wall, each opening subtracted by an `IfcOpeningElement`."""
    import ifcopenshell.api.feature

    model = ifcopenshell.api.project.create_file(version="IFC4")
    project = ifcopenshell.api.root.create_entity(model, ifc_class="IfcProject", name=snapshot["project"]["name"])
    ifcopenshell.api.unit.assign_unit(model)
    context = ifcopenshell.api.context.add_context(model, context_type="Model")
    body = ifcopenshell.api.context.add_context(model, context_type="Model", context_identifier="Body", target_view="MODEL_VIEW", parent=context)
    site = ifcopenshell.api.root.create_entity(model, ifc_class="IfcSite", name="site")
    building = ifcopenshell.api.root.create_entity(model, ifc_class="IfcBuilding", name="building")
    storey = ifcopenshell.api.root.create_entity(model, ifc_class="IfcBuildingStorey", name="storey")
    ifcopenshell.api.aggregate.assign_object(model, products=[site], relating_object=project)
    ifcopenshell.api.aggregate.assign_object(model, products=[building], relating_object=site)
    ifcopenshell.api.aggregate.assign_object(model, products=[storey], relating_object=building)

    def placement(origin, angle, z):
        return numpy.array([[math.cos(angle), -math.sin(angle), 0.0, origin[0]], [math.sin(angle), math.cos(angle), 0.0, origin[1]], [0.0, 0.0, 1.0, z], [0.0, 0.0, 0.0, 1.0]])

    for wall_id, wall in snapshot["walls"].items():
        if "Line" not in wall["axis"]:
            continue
        start, end = tips_of(wall)
        length = math.dist(start, end)
        base, top = vertical_of(snapshot, wall)
        total = math.fsum(faces_of(snapshot, wall)[2])
        angle = math.atan2(end[1] - start[1], end[0] - start[0])
        element = ifcopenshell.api.root.create_entity(model, ifc_class="IfcWall", name=wall_id)
        ifcopenshell.api.geometry.assign_representation(model, product=element, representation=ifcopenshell.api.geometry.add_wall_representation(model, context=body, length=length, height=top - base, thickness=total))
        ifcopenshell.api.geometry.edit_object_placement(model, product=element, matrix=placement(start, angle, base))
        ifcopenshell.api.spatial.assign_container(model, products=[element], relating_structure=storey)
        notches, holes = usable_cuts(valid_cuts(snapshot, wall_id, length, top - base), 0.0, length, top - base)
        for index, (s0, s1, z0, z1) in enumerate(notches + holes):
            low = z0 if z0 > EPS else -0.1
            opening = ifcopenshell.api.root.create_entity(model, ifc_class="IfcOpeningElement", name="%s-%d" % (wall_id, index))
            ifcopenshell.api.geometry.assign_representation(model, product=opening, representation=ifcopenshell.api.geometry.add_wall_representation(model, context=body, length=s1 - s0, height=z1 - low, thickness=2.0 * (total + 1.0)))
            cos, sin = math.cos(angle), math.sin(angle)
            origin = (start[0] + cos * s0 + sin * (total + 1.0), start[1] + sin * s0 - cos * (total + 1.0))
            ifcopenshell.api.geometry.edit_object_placement(model, product=opening, matrix=placement(origin, angle, base + low))
            ifcopenshell.api.feature.add_feature(model, feature=opening, element=element)
    reopened = ifcopenshell.file.from_string(model.to_string())
    settings = ifcopenshell.geom.settings()
    settings.set("use-world-coords", True)
    result = {}
    for element in reopened.by_type("IfcWall"):
        shape = ifcopenshell.geom.create_shape(settings, element)
        geometry = shape.geometry
        vertices = ifcopenshell.util.shape.get_vertices(geometry)
        result[element.Name] = {"volume": round(float(ifcopenshell.util.shape.get_volume(geometry)), 12), "z_min": round(float(vertices[:, 2].min()), 12), "z_max": round(float(vertices[:, 2].max()), 12)}
    return result


def audit_case(name, document):
    """⚖️ Every disagreement between the closed forms, the kernel and the shapely union for one case document, with the closed-form table."""
    snapshot, table = document["snapshot"], expected_table(document["snapshot"])
    problems = []
    if name.startswith("straight"):
        for wall_id, row in kernel_walls(snapshot).items():
            base, top = vertical_of(snapshot, snapshot["walls"][wall_id])
            for field, wanted in (("volume", table[wall_id]["volume"]), ("z_min", base), ("z_max", top)):
                if abs(row[field] - wanted) > KERNEL_TOLERANCE * max(abs(wanted), 1.0):
                    problems.append("%s/%s.%s: ifcopenshell %.12g, closed form %.12g" % (name, wall_id, field, row[field], wanted))
    if name.startswith("room"):
        union, summed = union_audit(snapshot, table)
        if abs(union - summed) > 1e-9:
            problems.append("%s: shapely union volume %.12g, summed walls %.12g" % (name, union, summed))
    return problems, table


def solid_cases(root):
    """📁️ `{case name: path}` of the `🧊️element-solids` fixtures under a `💡️inferences` root."""
    return {path.parent.name: path for path in sorted((Path(root) / SOLIDS_DIRECTORY).glob("*/🔣️.json"))}


def kernel_projection(document):
    """📤️ The projection of the oracle scenario: the kernel `{volume, z_min, z_max}` of the straight walls of one case."""
    return kernel_walls(document["snapshot"])


# endregion 🔖️Solids


# region 🔖️Handlers
def case_inputs(ctx):
    """📸️ The snapshot and the committed `🧱️wall-layout` table a scenario names, resolved through the host."""
    uris = ctx.step_input_uris()
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "📸️snapshot" in uri)).decode("utf-8"))
    layouts = json.loads(ctx.input_bytes(next(uri for uri in uris if "🧱️wall-layout" in uri)).decode("utf-8"))
    return snapshot, layouts


def wall_solids_handler(ctx):
    """🧊️ Oracle answer: the kernel's `z` extent and volume of every straight wall."""
    from semio_repo_test import Outcome

    payload = measure(*case_inputs(ctx))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def wall_solids_openings_handler(ctx):
    """🧊️ Oracle answer: the kernel's `volume` and `z` extent of every straight wall of the committed case, openings subtracted."""
    from semio_repo_test import Outcome

    uri = next(uri for uri in ctx.step_input_uris() if SOLIDS_DIRECTORY in uri)
    payload = kernel_projection(json.loads(ctx.input_bytes(uri).decode("utf-8")))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("wall-solids", wall_solids_handler).oracle("wall-solids-openings", wall_solids_openings_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def solids_main(command, root):
    """🧊️ `write` rewrites the `expected` table of every `🧊️element-solids` case from the closed forms; both commands audit them with the kernel and shapely."""
    failures = []
    for name, path in solid_cases(root).items():
        document = json.loads(path.read_text(encoding="utf-8"))
        if not (document["snapshot"].get("walls") or document["snapshot"].get("curtain_walls")):
            continue
        problems, table = audit_case(name, document)
        failures += problems
        if command == "write":
            head = json.dumps({"snapshot": document["snapshot"], "expected": table}, indent=2, ensure_ascii=False)[:-2]
            path.write_text("%s,\n  \"meshes\": %s\n}\n" % (head, json.dumps(document.get("meshes", {}), separators=(",", ":"), ensure_ascii=False)), encoding="utf-8")
        elif document["expected"] != table:
            failures.append("%s: the committed expected table differs from the closed forms (run write)" % name)
        print("%s: %d elements %s" % (name, len(table), "written" if command == "write" else "checked"))
    return failures


def main(arguments):
    """🏃️ `check` measures every case under a fixtures root against its committed tables; `write` regenerates the `🧊️element-solids` expectations first."""
    root = Path(arguments[1])
    failures = solids_main(arguments[0], root)
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        if not (case / "💡️inference" / "🧱️wall-layout" / "🔣️.json").exists():
            continue
        layouts = json.loads((case / "💡️inference" / "🧱️wall-layout" / "🔣️.json").read_text(encoding="utf-8"))
        measured = measure(snapshot, layouts)
        failures += ["%s: %s" % (case.name, problem) for problem in disagreements(measured, layouts)]
        print("%s: ifcopenshell %s measured %d of %d walls (arcs excluded)" % (case.name, ifcopenshell.version, len(measured), len(snapshot["walls"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (arguments[0], "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
