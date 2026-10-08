#!/usr/bin/env python3
"""Writes the authored snapshots of the plan and diagnostics oracle case (label i-plan-diagnostics).

`🗺️plan-linework/🏡️house` is a clean two-storey house that exercises every plan primitive; `⚠️diagnostics/💥️defects` is the same house with deliberate defects
(clashes, dangling references, a zero-length wall, an opening outside its wall, duplicate and skipped storey levels, a stair that breaks the
comfort rule). Expected tables are NOT written here: `🐍️.py` of the case writes them with shapely.

Usage: python r4-i-plan-diagnostics-fixtures.py <S>/🧫️fixtures/💡️inferences
"""
import copy
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])


def p(x, y):
    return {"x": float(x), "y": float(y)}


def v(x, y, bulge=0.0):
    return {"point": p(x, y), "bulge": float(bulge)}


def rect(x0, y0, x1, y1):
    return [v(x0, y0), v(x1, y0), v(x1, y1), v(x0, y1)]


def layer(material, thickness, function):
    return {"material": material, "thickness": float(thickness), "function": function}


def wall(storey, wall_type, start, end, name, location="Center", top=None, bulge=None):
    axis = {"Line": {"start": p(*start), "end": p(*end)}} if bulge is None else {"Arc": {"start": p(*start), "end": p(*end), "bulge": float(bulge)}}
    return {"storey": storey, "wall_type": wall_type, "axis": axis, "location": location, "base_offset": 0.0, "top": top or {"StoreyTop": {"offset": 0.0}}, "phase": "New", "name": name}


def opening(host, kind, offset, name, sill=0.0, width=None, height=None):
    row = {"host": host, "kind": kind, "offset": float(offset), "sill": float(sill), "flip_hand": False, "flip_facing": False, "name": name}
    if width is not None:
        row["width"] = float(width)
    if height is not None:
        row["height"] = float(height)
    return row


def house():
    material = lambda name, category, color, density: {"name": name, "category": category, "color": color, "density": density, "conductivity": 1.0, "specific_heat": 900.0}
    grey = {"r": 0.6, "g": 0.6, "b": 0.6}
    return {
        "schema": "s.bim.model@1",
        "project": {"name": "Plan House", "description": "", "author": "", "organization": "", "phase_names": []},
        "materials": {
            "m-brick": material("Brick", "Masonry", {"r": 0.7, "g": 0.35, "b": 0.25}, 1800.0),
            "m-insulation": material("Mineral Wool", "Insulation", {"r": 0.9, "g": 0.85, "b": 0.3}, 40.0),
            "m-concrete": material("Concrete", "Concrete", grey, 2400.0),
            "m-steel": material("Steel", "Metal", grey, 7800.0),
            "m-glass": material("Glass", "Glass", {"r": 0.6, "g": 0.8, "b": 0.9}, 2500.0),
            "m-wood": material("Wood", "Wood", {"r": 0.6, "g": 0.4, "b": 0.2}, 500.0),
        },
        "wall_types": {
            "wt-300": {"name": "Brick 300", "layers": [layer("m-brick", 0.2, "Structure"), layer("m-insulation", 0.1, "Insulation")]},
            "wt-150": {"name": "Brick 150", "layers": [layer("m-brick", 0.15, "Structure")]},
        },
        "slab_types": {"slt-22": {"name": "Concrete 220", "layers": [layer("m-concrete", 0.22, "Structure")]}},
        "roof_types": {"rt-30": {"name": "Timber 300", "layers": [layer("m-wood", 0.3, "Structure")]}},
        "column_types": {"ct-400": {"name": "Column 400", "profile": {"Rectangle": {"width": 0.4, "depth": 0.4}}, "material": "m-concrete"}},
        "beam_types": {"bt-30x50": {"name": "Beam 30x50", "profile": {"Rectangle": {"width": 0.3, "depth": 0.5}}, "material": "m-concrete"}},
        "window_types": {"wnd-120": {"name": "Window 120", "width": 1.2, "height": 1.2, "sill": 0.9, "frame_width": 0.05, "frame_depth": 0.08, "panes": 2, "material": "m-glass"}},
        "door_types": {"dr-100": {"name": "Door 100", "width": 1.0, "height": 2.1, "frame_width": 0.05, "frame_depth": 0.1, "leaves": "Single", "swing": "Left", "material": "m-wood"}},
        "sites": {"site-1": {"name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 0.0, "true_north": 0.0, "boundary": []}},
        "buildings": {"bldg-1": {"site": "site-1", "name": "House", "origin": p(0, 0), "rotation": 0.0, "elevation": 0.0}},
        "storeys": {
            "st-ground": {"building": "bldg-1", "name": "Ground", "level": 0, "height": 3.0},
            "st-first": {"building": "bldg-1", "name": "First", "level": 1, "height": 2.8},
        },
        "grids": {
            "g-A": {"building": "bldg-1", "label": "A", "start": p(0, -1), "end": p(0, 7)},
            "g-1": {"building": "bldg-1", "label": "1", "start": p(-1, 0), "end": p(9, 0)},
        },
        "walls": {
            "w-south": wall("st-ground", "wt-300", (0, 0), (8, 0), "South"),
            "w-east": wall("st-ground", "wt-300", (8, 0), (8, 6), "East"),
            "w-north": wall("st-ground", "wt-300", (8, 6), (0, 6), "North"),
            "w-west": wall("st-ground", "wt-300", (0, 6), (0, 0), "West"),
            "w-part": wall("st-ground", "wt-150", (4, 0), (4, 3), "Partition"),
            "w-first-south": wall("st-first", "wt-150", (0, 0), (8, 0), "First South"),
            "w-first-east": wall("st-first", "wt-150", (8, 0), (8, 6), "First East"),
            "w-first-west": wall("st-first", "wt-150", (0, 6), (0, 0), "First West"),
        },
        "curtain_walls": {
            "cw-first-north": {"storey": "st-first", "axis": {"Line": {"start": p(8, 6), "end": p(0, 6)}}, "base_offset": 0.0, "top": {"StoreyTop": {"offset": 0.0}}, "u_spacing": 2.0, "v_spacing": 1.0, "mullion": {"Rectangle": {"width": 0.06, "depth": 0.1}}, "panel_material": "m-glass", "mullion_material": "m-steel", "name": "North Facade"},
        },
        "columns": {"c-1": {"storey": "st-ground", "column_type": "ct-400", "position": p(6, 3), "rotation": 0.0, "base_offset": 0.0, "top": {"StoreyTop": {"offset": 0.0}}, "name": "C1"}},
        "beams": {"b-1": {"storey": "st-ground", "beam_type": "bt-30x50", "start": p(0, 4.5), "end": p(8, 4.5), "top_offset": -0.25, "name": "B1"}},
        "slabs": {
            "sl-ground": {"storey": "st-ground", "slab_type": "slt-22", "boundary": rect(0, 0, 8, 6), "holes": [], "offset": 0.0, "name": "Ground Slab"},
            "sl-first": {"storey": "st-first", "slab_type": "slt-22", "boundary": rect(0, 0, 8, 6), "holes": [rect(1, 4.7, 5.5, 5.7)], "offset": 0.0, "name": "First Slab"},
        },
        "roofs": {"rf-1": {"storey": "st-first", "roof_type": "rt-30", "footprint": rect(0, 0, 8, 6), "shape": {"Gable": {"pitch": 0.5, "ridge_direction": 0.0}}, "overhang": 0.3, "base_offset": 2.8, "name": "Roof"}},
        "openings": {
            "o-door": opening("w-south", {"Door": {"door_type": "dr-100"}}, 2.0, "Door"),
            "o-win-south": opening("w-south", {"Window": {"window_type": "wnd-120"}}, 5.5, "Window South"),
            "o-win-north": opening("w-north", {"Window": {"window_type": "wnd-120"}}, 4.0, "Window North High", sill=0.6),
        },
        "stairs": {"s-1": {"storey": "st-ground", "start": p(1, 5.2), "direction": 0.0, "width": 1.0, "flight": "Straight", "top": {"StoreyTop": {"offset": 0.0}}, "max_riser": 0.18, "min_tread": 0.25, "name": "Stair"}},
        "railings": {"r-1": {"storey": "st-ground", "path": [p(1, 4.65), p(5.2, 4.65)], "height": 1.0, "post_spacing": 1.2, "material": "m-steel", "base_offset": 0.0, "name": "Rail"}},
        "spaces": {
            "sp-living": {"storey": "st-ground", "number": "G.01", "name": "Living", "boundary": {"Explicit": {"outline": rect(0.15, 0.15, 7.85, 5.85)}}, "usage": "Living"},
            "sp-hall": {"storey": "st-ground", "number": "G.02", "name": "Hall", "boundary": {"Bounded": {"seed": p(2, 2)}}, "usage": "Circulation"},
        },
    }


def defects():
    model = copy.deepcopy(house())
    model["project"]["name"] = "Plan Defects"
    model["walls"]["w-missing-type"] = wall("st-ground", "wt-missing", (1, 2), (3, 2), "No Type")
    model["walls"]["w-zero"] = wall("st-ground", "wt-150", (2, 2), (2, 2), "Zero")
    model["walls"]["w-into-column"] = wall("st-ground", "wt-150", (6, 0), (6, 3), "Into Column")
    model["columns"]["c-2"] = {**model["columns"]["c-1"], "position": p(6.2, 3.1), "name": "C2"}
    model["openings"]["o-outside"] = opening("w-south", {"Door": {"door_type": "dr-100"}}, 7.9, "Outside")
    model["openings"]["o-orphan"] = opening("w-nowhere", {"Door": {"door_type": "dr-100"}}, 1.0, "Orphan")
    model["storeys"]["st-dup"] = {"building": "bldg-1", "name": "Duplicate", "level": 1, "height": 3.0}
    model["storeys"]["st-far"] = {"building": "bldg-1", "name": "Far", "level": 4, "height": 3.0}
    model["stairs"]["s-comfort"] = {"storey": "st-ground", "start": p(1, 1), "direction": 0.0, "width": 1.0, "flight": "Straight", "top": {"StoreyTop": {"offset": 0.0}}, "max_riser": 0.18, "min_tread": 0.4, "name": "Steep"}
    model["spaces"]["sp-twin"] = {"storey": "st-ground", "number": "G.01", "name": "Twin", "boundary": {"Bounded": {"seed": p(7, 5)}}, "usage": "Living"}
    return model


def curved():
    model = copy.deepcopy(house())
    model["project"]["name"] = "Plan Curved"
    for collection in ("wall_types", "slab_types", "roof_types", "beam_types", "curtain_walls", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "grids", "columns"):
        model[collection] = {key: value for key, value in model[collection].items() if collection in ("wall_types", "slab_types", "roof_types", "beam_types")}
    model["storeys"] = {"st-ground": model["storeys"]["st-ground"]}
    model["walls"] = {"w-arc": wall("st-ground", "wt-300", (0, 0), (8, 0), "Arc", bulge=0.5)}
    model["column_types"]["ct-round"] = {"name": "Round 500", "profile": {"Circle": {"diameter": 0.5}}, "material": "m-concrete"}
    model["columns"] = {"c-round": {"storey": "st-ground", "column_type": "ct-round", "position": p(4, 5), "rotation": 0.0, "base_offset": 0.0, "top": {"StoreyTop": {"offset": 0.0}}, "name": "Round"}}
    model["openings"] = {"o-arc-win": opening("w-arc", {"Window": {"window_type": "wnd-120"}}, 4.0, "Arc Window")}
    return model


def write(name, snapshot):
    target = root / name / "📸️snapshot" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(snapshot, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    print("wrote", target)


write("🗺️plan-linework/🏡️house", house())
write("🗺️plan-linework/🌀️curved", curved())
write("⚠️diagnostics/🏡️clean", house())
write("⚠️diagnostics/💥️defects", defects())
