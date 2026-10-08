#!/usr/bin/env python3
"""Writes the IFC export fixture model `S/🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json` (label x-ifc).

One site, one building, four storeys, joined and free-standing straight walls, a curved wall, openings of every kind,
columns, beams, flat and sloped slabs with holes, a gable roof, two stairs, a railing, two spaces, three grid lines,
property sets and classifications. Usage: python r4-x-ifc-gen-house.py <output json path>
"""

import json
import sys


def p(x, y):
    return {"x": x, "y": y}


def v(x, y, bulge=0.0):
    return {"point": p(x, y), "bulge": bulge}


def line(a, b):
    return {"Line": {"start": p(*a), "end": p(*b)}}


def arc(a, b, bulge):
    return {"Arc": {"start": p(*a), "end": p(*b), "bulge": bulge}}


def top_storey(offset=0.0):
    return {"StoreyTop": {"offset": offset}}


def top_free(height):
    return {"Unconnected": {"height": height}}


def wall(storey, kind, axis, location="Center", top=None, base_offset=0.0, phase="New", name=""):
    return {"storey": storey, "wall_type": kind, "axis": axis, "location": location, "base_offset": base_offset, "top": top or top_storey(), "phase": phase, "name": name}


def opening(host, kind, offset, sill=0.0, width=None, height=None, flip_hand=False, flip_facing=False, name=""):
    row = {"host": host, "kind": kind, "offset": offset, "sill": sill, "flip_hand": flip_hand, "flip_facing": flip_facing, "name": name}
    if width is not None:
        row["width"] = width
    if height is not None:
        row["height"] = height
    return row


def text(value):
    return {"Text": {"value": value}}


def number(kind, value):
    return {kind: {"value": value}}


def flag(value):
    return {"Boolean": {"value": value}}


def rectangle(x0, y0, x1, y1):
    return [v(x0, y0), v(x1, y0), v(x1, y1), v(x0, y1)]


model = {
    "schema": "s.bim.model@1",
    "project": {"name": "IFC House", "description": "Export fixture", "author": "A. Architect", "organization": "Semio", "phase_names": ["Existing", "New"]},
    "materials": {
        "m-brick": {"name": "Brick", "category": "Masonry", "color": {"r": 0.7, "g": 0.35, "b": 0.25}, "density": 1800.0, "conductivity": 0.8, "specific_heat": 900.0},
        "m-wool": {"name": "Mineral Wool", "category": "Insulation", "color": {"r": 0.9, "g": 0.85, "b": 0.3}, "density": 40.0, "conductivity": 0.035, "specific_heat": 1030.0},
        "m-conc": {"name": "Concrete", "category": "Concrete", "color": {"r": 0.6, "g": 0.6, "b": 0.6}, "density": 2400.0, "conductivity": 1.6, "specific_heat": 880.0},
        "m-wood": {"name": "Oak", "category": "Wood", "color": {"r": 0.55, "g": 0.4, "b": 0.2}, "density": 700.0, "conductivity": 0.18, "specific_heat": 1600.0},
        "m-steel": {"name": "Steel", "category": "Metal", "color": {"r": 0.5, "g": 0.55, "b": 0.6}, "density": 7850.0, "conductivity": 50.0, "specific_heat": 470.0},
    },
    "wall_types": {
        "wt-300": {"name": "Brick 300", "layers": [{"material": "m-brick", "thickness": 0.2, "function": "Structure"}, {"material": "m-wool", "thickness": 0.1, "function": "Insulation"}]},
        "wt-150": {"name": "Concrete 150", "layers": [{"material": "m-conc", "thickness": 0.15, "function": "Structure"}]},
    },
    "slab_types": {"st-floor": {"name": "Floor 250", "layers": [{"material": "m-wood", "thickness": 0.05, "function": "Finish"}, {"material": "m-conc", "thickness": 0.2, "function": "Structure"}]}},
    "roof_types": {"rt-tile": {"name": "Tile roof", "layers": [{"material": "m-wood", "thickness": 0.04, "function": "Finish"}, {"material": "m-wool", "thickness": 0.2, "function": "Insulation"}]}},
    "column_types": {
        "ct-rect": {"name": "Concrete 300x300", "profile": {"Rectangle": {"width": 0.3, "depth": 0.3}}, "material": "m-conc"},
        "ct-round": {"name": "Steel 400", "profile": {"Circle": {"diameter": 0.4}}, "material": "m-steel"},
    },
    "beam_types": {
        "bt-i": {"name": "IPE 300", "profile": {"IShape": {"width": 0.15, "depth": 0.3, "web": 0.0071, "flange": 0.0107}}, "material": "m-steel"},
        "bt-rect": {"name": "Concrete 250x400", "profile": {"Rectangle": {"width": 0.25, "depth": 0.4}}, "material": "m-conc"},
    },
    "window_types": {"wnd-120": {"name": "Window 120", "width": 1.2, "height": 1.2, "sill": 0.9, "frame_width": 0.06, "frame_depth": 0.08, "panes": 2, "material": "m-wood"}},
    "door_types": {
        "dr-90": {"name": "Door 90", "width": 0.9, "height": 2.1, "frame_width": 0.05, "frame_depth": 0.1, "leaves": "Single", "swing": "Left", "material": "m-wood"},
        "dr-180": {"name": "Double door 180", "width": 1.8, "height": 2.1, "frame_width": 0.05, "frame_depth": 0.1, "leaves": "Double", "swing": "Right", "material": "m-wood"},
    },
    "sites": {"site-1": {"name": "Plot", "latitude": 47.3769, "longitude": 8.5417, "elevation": 408.0, "true_north": 0.2, "boundary": [p(-5, -5), p(25, -5), p(25, 15), p(-5, 15)]}},
    "buildings": {"bldg-1": {"site": "site-1", "name": "House", "origin": p(2.0, 3.0), "rotation": 0.1, "elevation": 0.5}},
    "storeys": {
        "st-base": {"building": "bldg-1", "name": "Basement", "level": -1, "height": 2.6},
        "st-ground": {"building": "bldg-1", "name": "Ground", "level": 0, "height": 3.0},
        "st-first": {"building": "bldg-1", "name": "First", "level": 1, "height": 2.8},
        "st-roof": {"building": "bldg-1", "name": "Roof", "level": 2, "height": 0.5},
    },
    "grids": {
        "g-a": {"building": "bldg-1", "label": "A", "start": p(0, -1), "end": p(0, 9)},
        "g-b": {"building": "bldg-1", "label": "B", "start": p(8, -1), "end": p(8, 9)},
        "g-1": {"building": "bldg-1", "label": "1", "start": p(-1, 0), "end": p(9, 0)},
    },
    "walls": {
        "w-south": wall("st-ground", "wt-300", line((0, 0), (8, 0)), name="South"),
        "w-east": wall("st-ground", "wt-300", line((8, 0), (8, 6)), "Exterior", name="East"),
        "w-north": wall("st-ground", "wt-300", line((8, 6), (0, 6)), name="North"),
        "w-west": wall("st-ground", "wt-300", line((0, 6), (0, 0)), "Interior", name="West"),
        "w-free": wall("st-ground", "wt-150", line((2, 2), (5, 2)), top=top_free(2.4), base_offset=0.1, phase="Existing", name="Free"),
        "w-arc": wall("st-ground", "wt-150", arc((10, 1), (10, 5), 0.4), name="Curved"),
        "w-first-south": wall("st-first", "wt-300", line((0, 0), (8, 0)), name="First South"),
    },
    "curtain_walls": {
        "cw-1": {"storey": "st-first", "axis": line((0, 6), (6, 6)), "base_offset": 0.0, "top": top_storey(), "u_spacing": 1.5, "v_spacing": 1.4, "mullion": {"Rectangle": {"width": 0.05, "depth": 0.1}}, "panel_material": "m-wool", "mullion_material": "m-steel", "name": "Facade"},
    },
    "columns": {
        "c-1": {"storey": "st-ground", "column_type": "ct-rect", "position": p(4, 3), "rotation": 0.3, "base_offset": 0.0, "top": top_storey(), "name": "C1"},
        "c-2": {"storey": "st-ground", "column_type": "ct-round", "position": p(6, 3), "rotation": 0.0, "base_offset": 0.2, "top": top_free(2.5), "name": "C2"},
    },
    "beams": {
        "b-1": {"storey": "st-ground", "beam_type": "bt-i", "start": p(4, 3), "end": p(6, 3), "top_offset": 0.0, "name": "B1"},
        "b-2": {"storey": "st-ground", "beam_type": "bt-rect", "start": p(0, 3), "end": p(4, 3), "top_offset": -0.1, "name": "B2"},
    },
    "slabs": {
        "sl-ground": {"storey": "st-ground", "slab_type": "st-floor", "boundary": rectangle(0, 0, 8, 6), "holes": [rectangle(5.5, 4, 6.5, 5)], "offset": 0.0, "name": "Ground slab"},
        "sl-first": {"storey": "st-first", "slab_type": "st-floor", "boundary": rectangle(0, 0, 8, 6), "holes": [], "offset": 0.0, "name": "First slab"},
        "sl-balcony": {"storey": "st-first", "slab_type": "st-floor", "boundary": rectangle(8, 0, 10, 2), "holes": [], "offset": 0.0, "slope": {"direction": 0.0, "angle": 0.05}, "name": "Balcony"},
    },
    "roofs": {
        "r-1": {"storey": "st-roof", "roof_type": "rt-tile", "footprint": rectangle(0, 0, 8, 6), "shape": {"Gable": {"pitch": 0.5, "ridge_direction": 0.0}}, "overhang": 0.3, "base_offset": 0.0, "name": "Roof"},
    },
    "openings": {
        "o-win-1": opening("w-south", {"Window": {"window_type": "wnd-120"}}, 2.0, name="Window 1"),
        "o-door-1": opening("w-south", {"Door": {"door_type": "dr-90"}}, 6.0, flip_hand=True, name="Door 1"),
        "o-win-2": opening("w-north", {"Window": {"window_type": "wnd-120"}}, 4.0, sill=0.1, width=1.0, flip_facing=True, name="Window 2"),
        "o-void-1": opening("w-east", {"Void": {"width": 0.6, "height": 0.8}}, 3.0, sill=1.0, name="Duct"),
        "o-win-arc": opening("w-arc", {"Window": {"window_type": "wnd-120"}}, 2.5, name="Curved window"),
        "o-door-2": opening("w-first-south", {"Door": {"door_type": "dr-180"}}, 4.0, name="Double door"),
    },
    "stairs": {
        "s-1": {"storey": "st-ground", "start": p(1, 1), "direction": 0.0, "width": 1.0, "flight": "Straight", "top": top_storey(), "max_riser": 0.18, "min_tread": 0.27, "name": "Stair 1"},
        "s-2": {"storey": "st-first", "start": p(1, 4), "direction": 0.0, "width": 1.0, "flight": {"LTurn": {"split": 0.5, "turn": "Left"}}, "top": top_storey(), "max_riser": 0.18, "min_tread": 0.27, "name": "Stair 2"},
    },
    "railings": {"rl-1": {"storey": "st-first", "path": [p(0, 0), p(2, 0), p(2, 2)], "height": 1.0, "post_spacing": 1.0, "material": "m-steel", "base_offset": 0.0, "name": "Railing"}},
    "spaces": {
        "sp-1": {"storey": "st-ground", "number": "0.01", "name": "Living room", "boundary": {"Bounded": {"seed": p(6.8, 5.0)}}, "usage": "living"},
        "sp-2": {"storey": "st-first", "number": "1.01", "name": "Bedroom", "boundary": {"Explicit": {"outline": rectangle(0.2, 0.2, 3.8, 5.8)}}, "usage": "bedroom"},
    },
    "properties": {
        "w-south": {"Pset_WallCommon": {"FireRating": text("EI60"), "IsExternal": flag(True), "ThermalTransmittance": number("Real", 0.25)}},
        "c-1": {"Pset_ColumnCommon": {"LoadBearing": flag(True), "Reference": text("C-01")}, "Custom": {"Span": number("Length", 3.0), "Footprint": number("Area", 0.09), "Count": number("Integer", 4), "Tilt": number("Angle", 0.1), "Volume": number("Volume", 0.27)}},
    },
    "classifications": {
        "w-south": {"system": "Uniclass 2015", "code": "EF_25_10", "title": "Walls"},
        "w-east": {"system": "Uniclass 2015", "code": "EF_25_10", "title": "Walls"},
        "c-1": {"system": "Omniclass", "code": "21-02 10 10", "title": "Columns"},
    },
}

with open(sys.argv[1], "w", encoding="utf-8", newline="\n") as handle:
    json.dump(model, handle, indent=2, ensure_ascii=False)
    handle.write("\n")
