"""🧪 Stand-in for the model functions of T/api.md section 2 until the model agent's module lands; installs only the names the real module lacks."""
from __future__ import annotations

import math

L, W, H = 21.5, 19.0, 9.4
GX = [1.25, 6.0, 10.75, 15.5, 20.25]
GY = [0.0, 4.75, 9.5, 14.25, 19.0]
Z1, Z2 = 3.5, 6.7


def _arc(cx, cy, r, a0, a1, n=24):
    return [[cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))] for i in range(n + 1)]


def load_model():
    return {"revision": 1, "car": {"length": 4.7, "width": 1.75, "turning_radius": 6.0},
            "roof": {"waterproofing": 0.02, "screed": 0.06, "soil": 0.30, "concrete": 0.25}}


def _geometry(model=None):
    model = model or load_model()
    pilotis = [{"tag": "ABCDE"[j] + str(i + 1), "guid": "G%d%d" % (j, i), "x": x, "y": y, "radius": 0.15, "base": 0.0, "top": Z1 - 0.25}
               for j, y in enumerate(GY) for i, x in enumerate(GX)]
    box = [[0, 0], [L, 0], [L, W], [0, W]]
    def wall(tag, a, b, s, e):
        return {"tag": tag, "guid": "G" + tag, "start": a, "end": b, "thickness": 0.3, "base": Z1, "top": Z2,
                "openings": [{"tag": "RW" + tag[1:], "guid": "GO" + tag, "window_guid": "GW" + tag, "start": s, "end": e, "sill": Z1 + 0.9, "head": Z1 + 2.1}]}
    walls = [wall("W-S", [0, 0], [L, 0], 1.0, 20.5), wall("W-E", [L, 0], [L, W], 1.0, 18.0),
             wall("W-N", [L, W], [0, W], 1.0, 20.5), wall("W-W", [0, W], [0, 0], 1.0, 18.0)]
    curtain = _arc(10.75, 7.0, 4.0, 180, 0)
    path = [[6.75, 2.5]] + curtain + [[14.75, 2.5]]
    screens = [{"tag": "S1", "guid": "GS1", "path": _arc(17.5, 4.0, 3.0, 180, 360, 12)[:9], "base": Z2, "top": H},
               {"tag": "S2", "guid": "GS2", "path": [[6.0, 10.0], [8.0, 10.0], [8.0, 15.0]], "base": Z2, "top": H}]
    roof = [[0, 9.5], [L, 9.5], [L, W], [0, W]]
    return {
        "box": {"length": L, "width": W, "height": H},
        "storeys": {"ground": 0.0, "first": Z1, "roof": Z2},
        "grid": {"x": GX, "y": GY, "labels_x": list("12345"), "labels_y": list("ABCDE")},
        "pilotis": pilotis,
        "slabs": [{"tag": "SL-1", "guid": "GSL1", "kind": "FLOOR", "outline": box, "voids": [[[9.5, 3.0], [12.0, 3.0], [12.0, 10.0], [9.5, 10.0]]], "top": Z1, "thickness": 0.25},
                  {"tag": "SL-R", "guid": "GSLR", "kind": "ROOF", "outline": roof, "voids": [], "top": Z2, "thickness": 0.25}],
        "walls": walls,
        "curtain_wall": {"tag": "CW-1", "guid": "GCW", "path": path, "base": 0.0, "top": Z1 - 0.25, "radius": 4.0, "centre": [10.75, 7.0]},
        "ramp": {"tag": "RA-1", "guid": "GRA", "flights": [{"start": [10.75, 3.0], "end": [10.75, 8.5], "width": 1.5, "z0": 0.0, "z1": Z1},
                                                          {"start": [10.75, 9.2], "end": [10.75, 5.0], "width": 1.5, "z0": Z1, "z1": Z2}]},
        "screens": screens,
        "spaces": [{"id": "hall", "name": {"en": "Hall", "de": "Halle"}, "outline": path, "storey": "ground", "area": 60.0},
                   {"id": "garage", "name": {"en": "Garage", "de": "Garage"}, "outline": [[1.25, 7], [6.75, 7], [6.75, 17.75], [1.25, 17.75]], "storey": "ground", "area": 59.0},
                   {"id": "service", "name": {"en": "Service", "de": "Dienst"}, "outline": [[14.75, 7], [20.25, 7], [20.25, 17.75], [14.75, 17.75]], "storey": "ground", "area": 59.0},
                   {"id": "salon", "name": {"en": "Salon", "de": "Salon"}, "outline": [[0.3, 0.3], [10.4, 0.3], [10.4, 9.2], [0.3, 9.2]], "storey": "first", "area": 90.0},
                   {"id": "terrace", "name": {"en": "Hanging garden", "de": "Hängender Garten"}, "outline": [[11.1, 0.3], [21.2, 0.3], [21.2, 9.2], [11.1, 9.2]], "storey": "first", "area": 92.0},
                   {"id": "solarium", "name": {"en": "Solarium", "de": "Solarium"}, "outline": [[11.0, 1.0], [20.0, 1.0], [20.0, 8.0], [11.0, 8.0]], "storey": "roof", "area": 70.0}],
        "services": [{"tag": "BO-1", "guid": "GBO", "class": "IfcBoiler", "kind": "boiler", "points": [[3.0, 3.0, 0.0], [4.0, 3.0, 1.2]]},
                     {"tag": "RD-1", "guid": "GRD", "class": "IfcSpaceHeater", "kind": "radiator", "points": [[4.0, 0.5, Z1 + 0.15], [7.0, 0.5, Z1 + 0.75]]},
                     {"tag": "DR-1", "guid": "GDR", "class": "IfcWasteTerminal", "kind": "drain", "points": [[15.0, 7.0, Z1]]},
                     {"tag": "PP-1", "guid": "GPP", "class": "IfcPipeSegment", "kind": "pipe", "points": [[15.0, 7.0, Z1], [15.0, 7.0, 0.0]]}],
    }


def clearance(model=None):
    return {"spacing": 4.75, "initial_spacing": 5.0, "diameter": 0.3, "clear_width": 4.45, "car_width": 1.75, "margin": 2.7, "turning_radius": 6.0, "curve_radius": 4.0, "fits": True}


def class_chain(ifc_class):
    return [{"class": "IfcRoot", "adds": ["GlobalId", "OwnerHistory", "Name", "Description"]},
            {"class": "IfcObjectDefinition", "adds": []}, {"class": "IfcObject", "adds": ["ObjectType"]},
            {"class": "IfcProduct", "adds": ["ObjectPlacement", "Representation"]},
            {"class": "IfcElement", "adds": ["Tag"]}, {"class": "IfcBuiltElement", "adds": []},
            {"class": ifc_class, "adds": ["PredefinedType"]}]


def quantities(key, model=None):
    return {"cross_section_area": math.pi * 0.15 ** 2, "height": 3.25, "volume": math.pi * 0.15 ** 2 * 3.25, "count": 25, "total": 25 * math.pi * 0.15 ** 2 * 3.25}


def step_excerpt(key, model=None):
    lines = ["ISO-10303-21;", "#41=IFCCOLUMN('2O2Fr$t4X7Zf8NOew3FLOH',#5,'A1',$,'Piloti',#60,#70,$,.COLUMN.);",
             "#43=IFCPROPERTYSINGLEVALUE('LoadBearing',$,IFCBOOLEAN(.T.),$);"]
    spans = []
    for line, text, role in ((1, "IFCCOLUMN", "class"), (1, "2O2Fr$t4X7Zf8NOew3FLOH", "guid"), (1, "'A1'", "tag"), (1, ".COLUMN.", "predefined"),
                             (2, "IFCPROPERTYSINGLEVALUE", "property"), (2, "'LoadBearing'", "property")):
        start = lines[line].index(text)
        spans.append({"line": line, "start": start, "end": start + len(text), "role": role})
    return {"lines": lines, "spans": spans}


def mesh(key, segments=16, model=None):
    r, h, n = 0.15, 3.25, segments
    verts = [[r * math.cos(2 * math.pi * i / n), r * math.sin(2 * math.pi * i / n), z] for z in (0, h) for i in range(n)]
    verts += [[0, 0, 0], [0, 0, h]]
    faces = []
    for i in range(n):
        j = (i + 1) % n
        faces += [[i, j, n + j], [i, n + j, n + i], [2 * n, j, i], [2 * n + 1, n + i, n + j]]
    return {"vertices": verts, "faces": faces, "vertex_count": len(verts), "triangle_count": len(faces), "kept": ["geometry"], "lost": ["class", "properties"]}


def partial_model(model_id, model=None):
    return {"id": model_id, "schema_id": "IFC4X3_ADD2", "author": {"architecture": "Le Corbusier", "structure": "Structural engineer", "services": "MEP engineer"}[model_id],
            "revision": 3, "element_guids": [], "counts_by_class": {"architecture": {"IfcWall": 4, "IfcWindow": 4, "IfcSlab": 2}, "structure": {"IfcColumn": 25, "IfcSlab": 2}, "services": {"IfcBoiler": 1, "IfcSpaceHeater": 3}}[model_id]}


def translate_structure(model=None):
    xs = [GX[0] + (GX[-1] - GX[0]) * i / 8 for i in range(9)]
    ys = [GY[0] + (GY[-1] - GY[0]) * j / 8 for j in range(9)]
    nodes = [{"id": "N%d-%d" % (j, i), "position": [x, y, Z1 - 0.125]} for j, y in enumerate(ys) for i, x in enumerate(xs)]
    shells = [{"id": "S%d-%d" % (j, i), "nodes": ["N%d-%d" % (j, i), "N%d-%d" % (j, i + 1), "N%d-%d" % (j + 1, i + 1), "N%d-%d" % (j + 1, i)], "thickness": 0.25, "source_guid": "GSL1"} for j in range(8) for i in range(8)]
    supports = [{"node": "N%d-%d" % (j * 2, i * 2), "condition": "pinned"} for j in range(5) for i in range(5)]
    members = [{"id": "M%d" % k, "kind": "curve", "ifc_class": "IfcColumn", "source_guid": "G%d%d" % (k // 5, k % 5)} for k in range(25)]
    return {"source_revision": 1, "members": members, "nodes": nodes, "shells": shells, "supports": supports, "loads": {"area": 6.2},
            "assumptions": [{"en": "Slab as mid-surface", "de": "Decke als Mittelfläche"}], "losses": [{"source_tag": "A1", "property": "LoadBearing", "en": "Material grade", "de": "Materialgüte"}]}


def tributary(model=None):
    cells = []
    for j, y in enumerate(GY):
        for i, x in enumerate(GX):
            x0 = 0 if i == 0 else (GX[i - 1] + x) / 2
            x1 = L if i == 4 else (GX[i + 1] + x) / 2
            y0 = 0 if j == 0 else (GY[j - 1] + y) / 2
            y1 = W if j == 4 else (GY[j + 1] + y) / 2
            cells.append({"tag": "ABCDE"[j] + str(i + 1), "area": (x1 - x0) * (y1 - y0), "reaction": 6.2 * (x1 - x0) * (y1 - y0), "cells": [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]})
    return {"area_load": 6.2, "pilotis": cells}


def install(module, force=False):
    for name, fn in (("load_model", load_model), ("building_geometry", _geometry), ("clearance", clearance), ("class_chain", class_chain),
                     ("quantities", quantities), ("step_excerpt", step_excerpt), ("mesh", mesh), ("partial_model", partial_model),
                     ("translate_structure", translate_structure), ("tributary", tributary)):
        if force or not hasattr(module, name):
            setattr(module, name, fn)
