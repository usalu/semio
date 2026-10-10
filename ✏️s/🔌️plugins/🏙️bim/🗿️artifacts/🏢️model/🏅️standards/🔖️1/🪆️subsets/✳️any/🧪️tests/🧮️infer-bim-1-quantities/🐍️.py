#!/usr/bin/env python3
"""🧮️ Third-party ORACLE for the `s.bim.model@1` inference `🧮️quantities`.

The subject (Rust, `semio-s-artifact-bim-model`) takes off the quantities of every element from the inferred layouts and the authored
types: wall lengths, side areas, openings, volumes and layer volumes, slab and column and beam volumes, space areas, masses, and the
totals per kind, type, material, storey, building and project. This file reproduces the table of the elements whose measures follow in
closed form (walls, slabs, columns, beams, spaces) from the SAME committed snapshot, measuring every area with `shapely` 2 (GEOS), a
library that has never seen this repository:

* wall footprints come from the sibling oracle `../🧱️infer-bim-1-wall-joins` (analytic joins audited by GEOS); each layer area is the GEOS
  `intersection` of that footprint with the strip between the two layer interface curves, each opening area the GEOS `intersection` of the cut
  rectangle with the wall development box, wall lengths are GEOS `LineString.length`;
* slab areas are GEOS polygons with holes (arcs are circular segments in closed form, GEOS audits them on a sampled polygon);
* column and beam profile areas and perimeters are GEOS polygons, beam lengths GEOS `LineString.length`;
* space rows come from the sibling oracle `../🏠️infer-bim-1-spaces` (a `polygonize` of the wall arrangement);
* totals are re-summed independently over the closed-form elements.

The parametric law is a metamorphic property: raising one storey height by `delta` adds `delta * length` to the gross side area of every
`StoreyTop` wall on it, `delta * footprint area` to its volume, `delta` to the height of its columns, and moves nothing else.

The authoring law is a second one. A case directory that carries a `🦠️mutation/🔣️.json` (`setElementStorey` or `setElementPhase`) is the model BEFORE the
mutation: the oracle applies the one authored field itself (`apply_mutation`, never the subject's code), requires that the element and the
openings it hosts take the new storey or phase, that every other row of the take-off is untouched, and that the totals per storey (per phase) move
exactly those rows. The committed table of such a case is the take-off of the model AFTER the mutation.

The committed expectations under `🧫️fixtures/💡️inferences/🧮️quantities/<case>/💡️inference/🧮️quantities/🔣️.json` are WRITTEN by this
file (`write`), never by hand, and the Rust subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧮️quantities>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧮️quantities>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r2-design.md — snapshot and inference catalogue
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import LineString, Polygon, box

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
SAMPLED = 1e-5
REACH = 1e3
KEYS = {"Wall": "wall", "Slab": "slab", "Column": "column", "Beam": "beam", "Space": "space"}


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def joins():
    return load_sibling("🧱️infer-bim-1-wall-joins", "wall_joins")


def levels_oracle():
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels")


def frame_oracle():
    """➖️ The sibling shapely oracle of the column and beam solids: leaning columns, arc and inclined beams cut back to the column faces."""
    return load_sibling("📦️infer-bim-1-solids-rest", "frame")


def spaces_oracle():
    return load_sibling("🏠️infer-bim-1-spaces", "spaces")


def variant(value):
    """🏷️ Splits an externally tagged enum into `(tag, body)`; a unit variant is a bare string."""
    if isinstance(value, str):
        return value, {}
    (tag, body), = value.items()
    return tag, body


def xy(point):
    return (point["x"], point["y"])


def density(snapshot, material):
    return snapshot["materials"].get(material, {"density": 0.0})["density"]


# endregion 🔖️Vocabulary


# region 🔖️Rows
def empty(kind, storey, type_id="", phase="New"):
    """🧮️ A row with every measure zero."""
    return {"kind": kind, "storey": storey, "phase": phase, "type_id": type_id, "count": 1, "length": 0.0, "width": 0.0, "height": 0.0, "perimeter": 0.0, "gross_side_area": 0.0, "opening_area": 0.0, "net_side_area": 0.0, "gross_area": 0.0, "net_area": 0.0, "surface_area": 0.0, "gross_volume": 0.0, "net_volume": 0.0, "mass": 0.0, "risers": 0, "layers": [], "finishes": []}


def layer_row(snapshot, material, thickness, area, volume):
    return {"material": material, "thickness": thickness, "area": area, "volume": volume, "mass": volume * density(snapshot, material)}


def scale_at(curve, offset):
    """⭕️ How much longer a curve at `offset` (left positive) is than the axis per unit of axis length: 1 on a line, `1 - sign * offset / radius` on an arc."""
    return 1.0 if curve.is_line else 1.0 - math.copysign(1.0, curve.bulge) * offset / curve.radius


def strip_of(curve, upper, lower):
    """▭️ The shapely strip between the curves at offsets `upper` and `lower` (left positive); a straight axis is extended so the strip passes the miters."""
    if curve.is_line:
        direction = ((curve.end[0] - curve.start[0]) / curve.length, (curve.end[1] - curve.start[1]) / curve.length)
        curve = joins().Curve((curve.start[0] - direction[0] * REACH, curve.start[1] - direction[1] * REACH), (curve.end[0] + direction[0] * REACH, curve.end[1] + direction[1] * REACH), 0.0)
    first, second = curve.offset(upper), curve.offset(lower)
    return Polygon(first.sampled() + list(reversed(second.sampled())))


def layer_area(curve, layout, footprint, upper, lower):
    """🍰️ Plan area of the layer between two interface offsets: the closed-form annular sector of an arc with no joins, else the GEOS `intersection` of the footprint with the layer strip."""
    if footprint is None:
        return 0.0
    if not curve.is_line and not layout["joins"]:
        radius = lambda offset: curve.radius - offset * math.copysign(1.0, curve.bulge)
        return abs(curve.sweep) / 2.0 * abs(radius(upper) ** 2 - radius(lower) ** 2)
    return footprint.intersection(strip_of(curve, upper, lower)).area


def wall_row(snapshot, levels, layouts, wall_id):
    """🧱️ The quantities of one wall."""
    wall, layout = snapshot["walls"][wall_id], layouts[wall_id]
    row = empty("Wall", wall["storey"], wall["wall_type"], wall["phase"])
    curve = joins().axis_curve(wall["axis"])
    footprint = joins().sampled_polygon([(xy(vertex["point"]), vertex["bulge"]) for vertex in layout["footprint"]]) if layout["footprint"] else None
    height, length = layout["height"], curve.length
    opening_area = 0.0
    for opening in snapshot["openings"].values():
        if opening["host"] != wall_id:
            continue
        width, tall, sill = resolve(snapshot, opening)
        if width is None or width <= EXACT or tall <= EXACT:
            continue
        cut = box(opening["offset"] - width / 2.0, sill, opening["offset"] + width / 2.0, sill + tall)
        opening_area += cut.intersection(box(0.0, 0.0, curve.length, height)).area
    layers = snapshot["wall_types"].get(wall["wall_type"], {"layers": []})["layers"]
    interfaces = layout["layer_offsets"]
    rows = []
    for index, layer in enumerate(layers):
        area = layer_area(curve, layout, footprint, interfaces[index], interfaces[index + 1])
        middle = (interfaces[index] + interfaces[index + 1]) / 2.0
        volume = max(area * height - opening_area * layer["thickness"] * scale_at(curve, middle), 0.0)
        rows.append(layer_row(snapshot, layer["material"], layer["thickness"], area, volume))
    footprint_area = layout["footprint_area"]
    gross_volume = footprint_area * height
    net_volume = sum(entry["volume"] for entry in rows) if rows else max(gross_volume - opening_area * layout["thickness"], 0.0)
    row.update(length=length, width=layout["thickness"], height=height, perimeter=spaces_oracle().loop_length([(xy(vertex["point"]), vertex["bulge"]) for vertex in layout["footprint"]]) if layout["footprint"] else 0.0, gross_side_area=length * height, opening_area=opening_area, net_side_area=max(length * height - opening_area, 0.0), gross_area=footprint_area, net_area=footprint_area, gross_volume=gross_volume, net_volume=net_volume, mass=sum(entry["mass"] for entry in rows), layers=rows)
    return row


def resolve(snapshot, opening):
    """📐️ `(width, height, sill)` of an opening: an explicit size wins over the type; the sill override replaces the type sill; a missing type gives `None`."""
    tag, body = variant(opening["kind"])
    if tag == "Window":
        kind = snapshot["window_types"].get(body["window_type"])
        base = (kind["width"], kind["height"], kind["sill"]) if kind else None
    elif tag == "Door":
        kind = snapshot["door_types"].get(body["door_type"])
        base = (kind["width"], kind["height"], 0.0) if kind else None
    else:
        base = (body["width"], body["height"], 0.0)
    if base is None:
        return None, 0.0, 0.0
    width = opening["width"] if opening.get("width") is not None else base[0]
    tall = opening["height"] if opening.get("height") is not None else base[1]
    return width, tall, opening.get("sill_override", base[2])


def slab_row(snapshot, slab_id):
    """⬜️ The quantities of one slab."""
    slab = snapshot["slabs"][slab_id]
    row = empty("Slab", slab["storey"], slab["slab_type"], slab["phase"])
    spaces = spaces_oracle()
    outer = [(xy(vertex["point"]), vertex["bulge"]) for vertex in slab["boundary"]]
    holes = [[(xy(vertex["point"]), vertex["bulge"]) for vertex in hole] for hole in slab["holes"]]
    gross = abs(spaces.loop_area(outer))
    net = max(gross - sum(abs(spaces.loop_area(hole)) for hole in holes), 0.0)
    layers = snapshot["slab_types"].get(slab["slab_type"], {"layers": []})["layers"]
    thickness = math.fsum(max(layer["thickness"], 0.0) for layer in layers)
    slope = abs(math.cos(slab["slope"]["angle"])) if slab.get("slope") else 1.0
    rows = [layer_row(snapshot, layer["material"], layer["thickness"], net, net * max(layer["thickness"], 0.0)) for layer in layers]
    row.update(width=thickness, perimeter=spaces.loop_length(outer) + sum(spaces.loop_length(hole) for hole in holes), gross_area=gross, net_area=net, surface_area=net / max(slope, 1e-9), gross_volume=gross * thickness, net_volume=net * thickness, mass=sum(entry["mass"] for entry in rows), layers=rows)
    return row


def profile(kind):
    """▭️ `(area, perimeter)` of a profile in closed form."""
    tag, body = variant(kind["profile"])
    if tag == "Rectangle":
        return body["width"] * body["depth"], 2.0 * (body["width"] + body["depth"])
    if tag == "Circle":
        return math.pi * body["diameter"] ** 2 / 4.0, math.pi * body["diameter"]
    if tag == "IShape":
        w, d, t, f = body["width"], body["depth"], body["web"], body["flange"]
        outline = Polygon([(-w / 2, -d / 2), (w / 2, -d / 2), (w / 2, -d / 2 + f), (t / 2, -d / 2 + f), (t / 2, d / 2 - f), (w / 2, d / 2 - f), (w / 2, d / 2), (-w / 2, d / 2), (-w / 2, d / 2 - f), (-t / 2, d / 2 - f), (-t / 2, -d / 2 + f), (-w / 2, -d / 2 + f)])
        return outline.area, outline.length
    raise AssertionError("custom profiles are not part of this fixture")


def column_row(snapshot, levels, column_id):
    """🏛️ The quantities of one column."""
    column = snapshot["columns"][column_id]
    kind = snapshot["column_types"][column["column_type"]]
    own = levels[column["storey"]]
    base = own["elevation"] + column["base_offset"]
    tag, body = variant(column["top"])
    top = base + body["height"] if tag == "Unconnected" else own["top_elevation"] + body["offset"] if tag == "StoreyTop" else levels.get(body["storey"], own)["elevation"] + body["offset"]
    height = max(top - base, 0.0)
    area, perimeter = profile(kind)
    tilt = column.get("tilt")
    stretch = 1.0 / math.cos(tilt["angle"]) if tilt and abs(tilt["angle"]) > 1e-12 else 1.0
    volume = area * height * stretch
    row = empty("Column", column["storey"], column["column_type"], column["phase"])
    row.update(length=height * stretch, height=height, perimeter=perimeter, gross_area=area, net_area=area, gross_volume=volume, net_volume=volume, mass=volume * density(snapshot, kind["material"]), layers=[layer_row(snapshot, kind["material"], 0.0, area, volume)])
    return row


CUT = {}
"""🧮️ The cut-back beam rows of the snapshot last asked about (the sibling oracle measures every beam at once)."""


def beam_row(snapshot, beam_id):
    """➖️ The quantities of one beam."""
    beam = snapshot["beams"][beam_id]
    kind = snapshot["beam_types"][beam["beam_type"]]
    frame = frame_oracle()
    axis_tag, axis = variant(beam["axis"])
    plan = LineString([xy(axis["start"]), xy(axis["end"])]).length if axis_tag == "Line" else abs(4.0 * math.atan(axis["bulge"])) * math.dist(xy(axis["start"]), xy(axis["end"])) / (2.0 * abs(math.sin(2.0 * math.atan(axis["bulge"]))))
    rise = 0.0 if beam.get("end_top_offset") is None else beam["end_top_offset"] - beam["top_offset"]
    length = math.hypot(plan, rise)
    area, perimeter = profile(kind)
    gross = area * length
    if CUT.get("snapshot") is not snapshot:
        CUT.update(snapshot=snapshot, rows=frame.beam_rows(snapshot, frame.storey_levels(snapshot)))
    cut = CUT["rows"][beam_id]
    volume = cut["volume"] if cut is not None else gross
    row = empty("Beam", beam["storey"], beam["beam_type"], beam["phase"])
    row.update(length=length, perimeter=perimeter, gross_area=area, net_area=area, gross_volume=gross, net_volume=volume, mass=volume * density(snapshot, kind["material"]), layers=[layer_row(snapshot, kind["material"], 0.0, area, volume)])
    return row


def space_rows(snapshot):
    """🏠️ The space rows, from the sibling spaces oracle (only resolved rooms have quantities)."""
    rows = {}
    for space_id, found in spaces_oracle().tables(snapshot).items():
        if found["status"] not in ("Inferred", "Explicit"):
            continue
        row = empty("Space", snapshot["spaces"][space_id]["storey"], phase=snapshot["spaces"][space_id]["phase"])
        row.update(height=found["clear_height"], perimeter=found["perimeter"], gross_area=found["area"], net_area=found["net_floor_area"], gross_volume=found["volume"], net_volume=found["volume"])
        rows[space_id] = row
    return rows


# endregion 🔖️Rows


# region 🔖️Totals
def area_of(row):
    return row["net_side_area"] if row["kind"] in ("Wall", "CurtainWall") else row["net_area"]


def add_element(totals, row):
    totals["count"] += row["count"]
    totals["length"] += row["length"]
    totals["area"] += area_of(row)
    totals["volume"] += row["net_volume"]
    totals["mass"] += row["mass"]


def add_layer(totals, entry):
    totals["count"] += 1
    totals["area"] += entry["area"]
    totals["volume"] += entry["volume"]
    totals["mass"] += entry["mass"]


def zero():
    return {"count": 0, "length": 0.0, "area": 0.0, "volume": 0.0, "mass": 0.0}


def add(total, row):
    key = KEYS[row["kind"]]
    add_element(total["kinds"].setdefault(key, zero()), row)
    if row["type_id"]:
        add_element(total["types"].setdefault("%s:%s" % (key, row["type_id"]), zero()), row)
    for entry in row["layers"]:
        if entry["material"]:
            add_layer(total["materials"].setdefault(entry["material"], zero()), entry)
    phase = row["phase"].lower()
    add_element(total["phases"].setdefault(phase, zero()), row)
    add_element(total["phase_kinds"].setdefault("%s:%s" % (phase, key), zero()), row)


def summarise(snapshot, elements):
    """➕️ The totals per storey, building and project over a set of rows (ids in order)."""
    fresh = lambda: {"kinds": {}, "types": {}, "materials": {}, "finishes": {}, "phases": {}, "phase_kinds": {}}
    result = {"elements": elements, "storeys": {}, "buildings": {}, "project": fresh()}
    for element_id in sorted(elements):
        row = elements[element_id]
        add(result["project"], row)
        storey = snapshot["storeys"].get(row["storey"])
        if storey is not None:
            add(result["storeys"].setdefault(row["storey"], fresh()), row)
            add(result["buildings"].setdefault(storey["building"], fresh()), row)
    return result


def table(snapshot):
    """🧮️ The `🧮️quantities` table of the closed-form elements of a snapshot."""
    levels = levels_oracle().storey_levels(snapshot)
    layouts = levels_oracle().wall_layouts(snapshot, levels)
    elements = {wall_id: wall_row(snapshot, levels, layouts, wall_id) for wall_id in snapshot["walls"]}
    elements.update({slab_id: slab_row(snapshot, slab_id) for slab_id in snapshot["slabs"]})
    elements.update({column_id: column_row(snapshot, levels, column_id) for column_id in snapshot["columns"]})
    elements.update({beam_id: beam_row(snapshot, beam_id) for beam_id in snapshot["beams"]})
    elements.update(space_rows(snapshot))
    return summarise(snapshot, elements)


# endregion 🔖️Totals


# region 🔖️Audit
def problems_of(snapshot):
    """🩺️ Disagreements between the closed forms and GEOS, plus the parametric law."""
    problems = []
    spaces = spaces_oracle()
    for slab_id, slab in snapshot["slabs"].items():
        outer = [(xy(vertex["point"]), vertex["bulge"]) for vertex in slab["boundary"]]
        holes = [[(xy(vertex["point"]), vertex["bulge"]) for vertex in hole] for hole in slab["holes"]]
        shape = Polygon(joins().sampled_polygon(outer).exterior.coords, [joins().sampled_polygon(hole).exterior.coords for hole in holes])
        expected = abs(spaces.loop_area(outer)) - sum(abs(spaces.loop_area(hole)) for hole in holes)
        if abs(shape.area - expected) > SAMPLED * max(expected, 1.0):
            problems.append("%s: GEOS slab area %.12g, closed form %.12g" % (slab_id, shape.area, expected))
    for column_id, column in snapshot["columns"].items():
        area, perimeter = profile(snapshot["column_types"][column["column_type"]])
        tag, body = variant(snapshot["column_types"][column["column_type"]]["profile"])
        if tag == "Circle":
            circle = joins().sampled_polygon([((body["diameter"] / 2.0, 0.0), 1.0), ((-body["diameter"] / 2.0, 0.0), 1.0)])
            if abs(circle.area - area) > SAMPLED * area or abs(circle.length - perimeter) > SAMPLED * perimeter:
                problems.append("%s: GEOS circle %.12g / %.12g, closed form %.12g / %.12g" % (column_id, circle.area, circle.length, area, perimeter))
    layouts = levels_oracle().wall_layouts(snapshot, levels_oracle().storey_levels(snapshot))
    for wall_id, wall in snapshot["walls"].items():
        curve, layout = joins().axis_curve(wall["axis"]), layouts[wall_id]
        if curve.is_line or layout["joins"] or not layout["footprint"]:
            continue
        footprint = joins().sampled_polygon([(xy(vertex["point"]), vertex["bulge"]) for vertex in layout["footprint"]])
        interfaces = layout["layer_offsets"]
        for index in range(len(interfaces) - 1):
            closed = layer_area(curve, layout, footprint, interfaces[index], interfaces[index + 1])
            measured = footprint.intersection(strip_of(curve, interfaces[index], interfaces[index + 1])).area
            if abs(measured - closed) > SAMPLED * max(closed, 1.0):
                problems.append("%s layer %d: GEOS %.12g, closed-form annular sector %.12g" % (wall_id, index, measured, closed))
    elements = table(snapshot)["elements"]
    for wall_id, row in elements.items():
        if row["kind"] != "Wall":
            continue
        if abs(sum(entry["area"] for entry in row["layers"]) - row["gross_area"]) > EXACT * max(row["gross_area"], 1.0):
            problems.append("%s: the layer areas %.12g do not add up to the footprint %.12g" % (wall_id, sum(entry["area"] for entry in row["layers"]), row["gross_area"]))
    return problems + parametric_problems(snapshot)


def parametric_problems(snapshot):
    """🧪️ Raising a storey by `delta` moves exactly the walls and columns that follow it."""
    problems = []
    before = table(snapshot)["elements"]
    for storey_id in sorted(snapshot["storeys"]):
        raised = copy.deepcopy(snapshot)
        raised["storeys"][storey_id]["height"] += 0.4
        after = table(raised)["elements"]
        for element_id, row in before.items():
            if row["kind"] == "Wall" and row["storey"] == storey_id and variant(snapshot["walls"][element_id]["top"])[0] == "StoreyTop":
                if abs(after[element_id]["gross_side_area"] - row["gross_side_area"] - 0.4 * row["length"]) > EXACT * 10:
                    problems.append("%s: raising %s by 0.4 changed the gross side area by %.12g" % (element_id, storey_id, after[element_id]["gross_side_area"] - row["gross_side_area"]))
            if row["kind"] == "Column" and row["storey"] == storey_id and variant(snapshot["columns"][element_id]["top"])[0] == "StoreyTop":
                if abs(after[element_id]["height"] - row["height"] - 0.4) > EXACT * 10:
                    problems.append("%s: raising %s by 0.4 changed the column height by %.12g" % (element_id, storey_id, after[element_id]["height"] - row["height"]))
            if row["kind"] == "Slab" and after[element_id] != row:
                problems.append("%s: a slab followed a storey height" % element_id)
    return problems


STOREYED = ["walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "stairs", "railings", "ramps", "spaces"]
PHASED = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces"]


def apply_mutation(snapshot, mutation):
    """🦠️ The model after a `setElementStorey` or `setElementPhase` mutation: the one authored field of the one element, set in whichever collection holds the id.
    An independent re-implementation of the rule (no refusal logic): the committed cases only carry mutations the subject accepts."""
    field, collections = {"setElementStorey": ("storey", STOREYED), "setElementPhase": ("phase", PHASED)}[mutation["mutation"]]
    changed = copy.deepcopy(snapshot)
    for key in collections:
        row = changed.get(key, {}).get(mutation["id"])
        if row is not None:
            row[field] = mutation[field]
            return changed
    raise AssertionError("%s names no element that carries a %s" % (mutation["id"], field))


def mutation_problems(snapshot, mutation):
    """🧪️ The law of moving an element: the mutated element takes the new storey (or phase) and keeps every opening it hosts (the hosted openings follow, so its opening area
    is unchanged), every other row of the take-off that does not depend on the moved geometry is unchanged, and the totals move exactly the row: a storey (or phase)
    loses what the other gains, the project counts nothing new."""
    after, problems = apply_mutation(snapshot, mutation), []
    one, two = table(snapshot), table(after)
    moved = [mutation["id"]]
    field = "storey" if mutation["mutation"] == "setElementStorey" else "phase"
    for element_id in moved:
        row, was = two["elements"].get(element_id), one["elements"].get(element_id)
        if row is None:
            problems.append("%s has no row after the mutation" % element_id)
        elif row[field] != mutation[field]:
            problems.append("%s: %s is %s after the mutation, not %s" % (element_id, field, row[field], mutation[field]))
        elif was is not None and abs(row["opening_area"] - was["opening_area"]) > EXACT * 10:
            problems.append("%s: its openings %s the host (opening area %.12g, was %.12g)" % (element_id, "did not follow" if row["opening_area"] < was["opening_area"] else "overshoot", row["opening_area"], was["opening_area"]))
    for element_id, row in one["elements"].items():
        if element_id not in moved and row["kind"] != "Wall" and row != two["elements"].get(element_id):
            problems.append("%s changed with the mutation of %s" % (element_id, mutation["id"]))
    rows = [one["elements"][element_id] for element_id in moved if element_id in one["elements"]]
    source = rows[0][field] if rows else None
    kinds = {}
    for row in rows:
        kinds[KEYS[row["kind"]]] = kinds.get(KEYS[row["kind"]], 0) + row["count"]
    for kind, count in kinds.items():
        if field == "storey":
            had = lambda total, key: total["storeys"].get(key, {}).get("kinds", {}).get(kind, {}).get("count", 0)
        else:
            had = lambda total, key: total["project"]["phase_kinds"].get("%s:%s" % (key.lower(), kind), {}).get("count", 0)
        if had(two, source) != had(one, source) - count:
            problems.append("%s: %d %s row(s) left %s, which holds %d afterwards instead of %d" % (mutation["id"], count, kind, source, had(two, source), had(one, source) - count))
        if had(two, mutation[field]) != had(one, mutation[field]) + count:
            problems.append("%s: %d %s row(s) joined %s, which holds %d afterwards instead of %d" % (mutation["id"], count, kind, mutation[field], had(two, mutation[field]), had(one, mutation[field]) + count))
        if two["project"]["kinds"].get(kind, {}).get("count", 0) != one["project"]["kinds"].get(kind, {}).get("count", 0):
            problems.append("%s: the project count of %s changed" % (mutation["id"], kind))
    return problems


# endregion 🔖️Audit


# region 🔖️Handlers
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return levels_oracle().compare(expected, actual, path)


def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def case_mutation(ctx):
    """🦠️ The mutation payload a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "🦠️mutation" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def storey_move_handler(ctx):
    """🛗️ Oracle answer for the take-off of a model after `setElementStorey`: the independent move, the law of moving, GEOS and the parametric law agree."""
    from semio_repo_test import Outcome

    snapshot, mutation = case_snapshot(ctx), case_mutation(ctx)
    moved = apply_mutation(snapshot, mutation)
    problems = mutation_problems(snapshot, mutation) + problems_of(moved)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = table(moved)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def quantities_handler(ctx):
    """🧮️ Oracle answer for `🧮️quantities`, after GEOS and the parametric law agreed with it."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = table(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("quantities-building", quantities_handler).oracle("quantities-storey-move", storey_move_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        mutation_path = case / "🦠️mutation" / "🔣️.json"
        if mutation_path.exists():
            mutation = json.loads(mutation_path.read_text(encoding="utf-8"))
            failures += ["%s: %s" % (case.name, problem) for problem in mutation_problems(snapshot, mutation)]
            snapshot = apply_mutation(snapshot, mutation)
        failures += ["%s: %s" % (case.name, problem) for problem in problems_of(snapshot)]
        computed = table(snapshot)
        target = case / "💡️inference" / "🧮️quantities" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "quantities")]
        print("%s: shapely %s, %d elements" % (case.name, shapely.__version__, len(computed["elements"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
