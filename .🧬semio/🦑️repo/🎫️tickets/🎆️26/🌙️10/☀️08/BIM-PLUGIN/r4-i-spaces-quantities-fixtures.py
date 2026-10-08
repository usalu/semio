#!/usr/bin/env python3
"""🏠️ Writes the authored snapshots of the `spaces`, `quantities` and `stair-runs` inference fixtures of `s.bim.model@1`.

Only the snapshots are written here (the ONLY input of an inference oracle); the expected tables are written by the oracles
(`🏠️infer-bim-1-spaces`, `🧮️infer-bim-1-quantities`, `🛤️infer-bim-1-stair-runs` under the subset `🧪️tests/`).

    python r4-i-spaces-quantities-fixtures.py <path to the subset 🧫️fixtures/💡️inferences>
"""

import json
import sys
from pathlib import Path

COLLECTIONS = ["materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "properties", "classifications"]


def snapshot(name, **parts):
    return {"schema": "s.bim.model@1", "project": {"name": name, "description": "", "author": "", "organization": "", "phase_names": []}, **{key: parts.get(key, {}) for key in COLLECTIONS}}


def p(x, y):
    return {"x": float(x), "y": float(y)}


def material(name, density, category="Masonry"):
    return {"name": name, "category": category, "color": {"r": 0.7, "g": 0.35, "b": 0.25}, "density": float(density), "conductivity": 1.0, "specific_heat": 900.0}


def layer(material_id, thickness, function="Structure"):
    return {"material": material_id, "thickness": float(thickness), "function": function}


def line(a, b):
    return {"Line": {"start": p(*a), "end": p(*b)}}


def arc(a, b, bulge):
    return {"Arc": {"start": p(*a), "end": p(*b), "bulge": float(bulge)}}


def wall(storey, wall_type, axis, top=None, location="Center", base_offset=0.0, name="Wall"):
    return {"storey": storey, "wall_type": wall_type, "axis": axis, "location": location, "base_offset": float(base_offset), "top": top or {"StoreyTop": {"offset": 0.0}}, "phase": "New", "name": name}


def vertex(x, y, bulge=0.0):
    return {"point": p(x, y), "bulge": float(bulge)}


def polygon(*points):
    return [vertex(*point) for point in points]


def space(storey, boundary, name):
    return {"storey": storey, "number": name, "name": name, "boundary": boundary, "usage": "room"}


def bounded(x, y):
    return {"Bounded": {"seed": p(x, y)}}


def explicit(outline):
    return {"Explicit": {"outline": outline}}


def column(storey, column_type, x, y, rotation=0.0, top=None):
    return {"storey": storey, "column_type": column_type, "position": p(x, y), "rotation": float(rotation), "base_offset": 0.0, "top": top or {"StoreyTop": {"offset": 0.0}}, "name": "Column"}


def slab(storey, slab_type, boundary, holes=(), offset=0.0, slope=None):
    row = {"storey": storey, "slab_type": slab_type, "boundary": boundary, "holes": [list(hole) for hole in holes], "offset": float(offset), "name": "Slab"}
    if slope:
        row["slope"] = {"direction": float(slope[0]), "angle": float(slope[1])}
    return row


def place():
    return (
        {"site-1": {"name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 100.0, "true_north": 0.0, "boundary": []}},
        {"bldg-1": {"site": "site-1", "name": "House", "origin": p(0, 0), "rotation": 0.0, "elevation": 0.0}},
    )


def storey(name, level, height):
    return {"building": "bldg-1", "name": name, "level": level, "height": float(height)}


def rooms():
    sites, buildings = place()
    ring = lambda storey_id, prefix, x, y, w, d, wall_type, west="Center": {
        f"{prefix}-south": wall(storey_id, wall_type, line((x, y), (x + w, y))),
        f"{prefix}-east": wall(storey_id, wall_type, line((x + w, y), (x + w, y + d))),
        f"{prefix}-north": wall(storey_id, wall_type, line((x + w, y + d), (x, y + d))),
        f"{prefix}-west": wall(storey_id, wall_type, line((x, y + d), (x, y)), location=west),
    }
    walls = {**ring("st-ground", "ground", 0, 0, 8, 6, "wt-200", west="Interior"), **ring("st-first", "first", 0, 0, 4, 3, "wt-200")}
    walls["ground-partition"] = wall("st-ground", "wt-100", line((5, 0), (5, 6)))
    walls["ground-island"] = wall("st-ground", "wt-200", line((1.5, 4.5), (3.0, 4.5)))
    return snapshot(
        "Rooms",
        materials={"m-brick": material("Brick", 1800), "m-concrete": material("Concrete", 2400, "Concrete")},
        wall_types={"wt-200": {"name": "Brick 200", "layers": [layer("m-brick", 0.2)]}, "wt-100": {"name": "Brick 100", "layers": [layer("m-brick", 0.1)]}},
        slab_types={"slt-250": {"name": "Slab 250", "layers": [layer("m-concrete", 0.25)]}, "slt-400": {"name": "Slab 400", "layers": [layer("m-concrete", 0.4)]}},
        column_types={"ct-300": {"name": "Column 300", "profile": {"Rectangle": {"width": 0.3, "depth": 0.3}}, "material": "m-concrete"}},
        sites=sites,
        buildings=buildings,
        storeys={"st-ground": storey("Ground", 0, 3.0), "st-first": storey("First", 1, 2.8)},
        walls=walls,
        columns={"c-living": column("st-ground", "ct-300", 1.5, 1.5, 0.5)},
        slabs={
            "sl-over-living": slab("st-first", "slt-250", polygon((0, 0), (5, 0), (5, 6), (0, 6)), offset=-0.05),
            "sl-over-kitchen": slab("st-first", "slt-400", polygon((5, 0), (8, 0), (8, 6), (5, 6)), holes=[polygon((6.0, 2.5), (7.0, 2.5), (7.0, 3.5), (6.0, 3.5))]),
        },
        spaces={
            "sp-living": space("st-ground", bounded(2.5, 2.5), "Living"),
            "sp-kitchen": space("st-ground", bounded(6.5, 3.0), "Kitchen"),
            "sp-outside": space("st-ground", bounded(12.0, 3.0), "Outside"),
            "sp-in-wall": space("st-ground", bounded(5.0, 3.0), "In wall"),
            "sp-round": space("st-ground", explicit([vertex(10, 1, 1.0), vertex(12, 1, 1.0)]), "Round"),
            "sp-box": space("st-ground", explicit(polygon((10, 3), (10, 5), (12, 5), (12, 3))), "Box"),
            "sp-upper": space("st-first", bounded(2.0, 1.5), "Upper"),
        },
    )


def quantities():
    sites, buildings = place()
    openings = {
        "o-window": {"host": "w-iso", "kind": {"Window": {"window_type": "win"}}, "offset": 1.5, "sill": 0.0, "flip_hand": False, "flip_facing": False, "name": "Window"},
        "o-door": {"host": "w-iso", "kind": {"Door": {"door_type": "door"}}, "offset": 3.8, "sill": 0.0, "flip_hand": False, "flip_facing": False, "name": "Door"},
        "o-void": {"host": "w-arc", "kind": {"Void": {"width": 1.0, "height": 1.0}}, "offset": 3.0, "sill": 0.5, "flip_hand": False, "flip_facing": False, "name": "Void"},
    }
    return snapshot(
        "Quantities",
        materials={"m-brick": material("Brick", 1800), "m-wool": material("Wool", 40, "Insulation"), "m-concrete": material("Concrete", 2400, "Concrete")},
        wall_types={"wt-300": {"name": "Brick 300", "layers": [layer("m-brick", 0.2), layer("m-wool", 0.1, "Insulation")]}, "wt-200": {"name": "Brick 200", "layers": [layer("m-brick", 0.2)]}},
        slab_types={"slt-250": {"name": "Slab 250", "layers": [layer("m-concrete", 0.15), layer("m-concrete", 0.1, "Finish")]}},
        column_types={"ct-sq": {"name": "Square", "profile": {"Rectangle": {"width": 0.3, "depth": 0.3}}, "material": "m-concrete"}, "ct-round": {"name": "Round", "profile": {"Circle": {"diameter": 0.4}}, "material": "m-concrete"}},
        beam_types={"bt-rect": {"name": "Beam", "profile": {"Rectangle": {"width": 0.2, "depth": 0.4}}, "material": "m-concrete"}},
        window_types={"win": {"name": "Window", "width": 1.2, "height": 1.0, "sill": 0.9, "frame_width": 0.06, "frame_depth": 0.1, "panes": 2, "material": "m-wool"}},
        door_types={"door": {"name": "Door", "width": 0.9, "height": 2.1, "frame_width": 0.06, "frame_depth": 0.1, "leaves": "Single", "swing": "Left", "material": "m-wool"}},
        sites=sites,
        buildings=buildings,
        storeys={"st-0": storey("Ground", 0, 2.8), "st-1": storey("First", 1, 3.0)},
        walls={
            "w-iso": wall("st-0", "wt-300", line((0, 0), (5, 0))),
            "w-l1": wall("st-0", "wt-200", line((10, 0), (14, 0))),
            "w-l2": wall("st-0", "wt-200", line((14, 0), (14, 3)), location="Exterior"),
            "w-arc": wall("st-0", "wt-300", arc((20, 0), (16, 0), 1.0), top={"Unconnected": {"height": 2.5}}, base_offset=0.1),
            "w-up": wall("st-1", "wt-200", line((0, 0), (2, 0)), top={"StoreyTop": {"offset": 0.2}}),
        },
        openings=openings,
        slabs={
            "sl-holed": slab("st-0", "slt-250", polygon((0, 10), (6, 10), (6, 14), (0, 14)), holes=[polygon((1, 11), (2, 11), (2, 12), (1, 12))], slope=(0.0, 0.1)),
            "sl-round": slab("st-0", "slt-250", [vertex(20, 10, 1.0), vertex(24, 10, 1.0)], offset=-0.05),
        },
        columns={"c-sq": column("st-0", "ct-sq", 1, 5, 0.4), "c-round": column("st-0", "ct-round", 3, 5, 0.0, top={"Unconnected": {"height": 2.0}}), "c-up": column("st-1", "ct-sq", 1, 5, 0.0, top={"Storey": {"storey": "st-1", "offset": 2.4}})},
        beams={"b-1": {"storey": "st-0", "beam_type": "bt-rect", "start": p(0, 3), "end": p(4, 0), "top_offset": 0.0, "name": "Beam"}},
        spaces={
            "sp-hall": space("st-0", explicit(polygon((0, 20), (5, 20), (5, 24), (0, 24))), "Hall"),
            "sp-round": space("st-0", explicit([vertex(10, 20, 1.0), vertex(12, 20, 1.0)]), "Round"),
        },
    )


def stairs():
    sites, buildings = place()
    stair = lambda storey_id, flight, top, x=0.0, y=0.0, direction=0.0, width=1.0, max_riser=0.1875, min_tread=0.25: {
        "storey": storey_id, "start": p(x, y), "direction": float(direction), "width": float(width), "flight": flight, "top": top, "max_riser": float(max_riser), "min_tread": float(min_tread), "name": "Stair",
    }
    top = lambda offset=0.0: {"StoreyTop": {"offset": float(offset)}}
    return snapshot(
        "Stairs",
        sites=sites,
        buildings=buildings,
        storeys={"st-base": storey("Basement", -1, 2.6), "st-ground": storey("Ground", 0, 3.0), "st-first": storey("First", 1, 2.8), "st-roof": storey("Roof", 2, 0.5)},
        stairs={
            "s-straight": stair("st-ground", "Straight", top()),
            "s-straight-tall": stair("st-first", "Straight", top(), x=2, y=1, direction=0.3, max_riser=0.17, min_tread=0.28),
            "s-lturn": stair("st-ground", {"LTurn": {"split": 0.5, "turn": "Left"}}, top(), x=5, y=0, direction=1.0),
            "s-lturn-right": stair("st-ground", {"LTurn": {"split": 0.3, "turn": "Right"}}, top(), x=9, y=0, direction=-0.5, width=1.2),
            "s-uturn": stair("st-ground", {"UTurn": {"gap": 0.2}}, top(), x=12, y=3, direction=3.141592653589793 / 2),
            "s-spiral": stair("st-ground", {"Spiral": {"radius": 1.5, "sweep": 6.283185307179586}}, top(), x=20, y=0),
            "s-spiral-cw": stair("st-ground", {"Spiral": {"radius": 1.2, "sweep": -4.0}}, top(), x=24, y=0, direction=0.7),
            "s-two-storeys": stair("st-ground", "Straight", {"Storey": {"storey": "st-roof", "offset": -0.5}}, x=30, y=0),
            "s-free": stair("st-ground", "Straight", {"Unconnected": {"height": 2.4}}, x=34, y=0),
            "s-basement": stair("st-base", "Straight", top(), x=38, y=0, max_riser=0.19, min_tread=0.26),
            "s-steep": stair("st-ground", "Straight", top(), x=42, y=0, max_riser=0.25, min_tread=0.2),
            "s-one-riser": stair("st-ground", {"UTurn": {"gap": 0.1}}, {"Unconnected": {"height": 0.15}}, x=46, y=0),
            "s-flat": stair("st-ground", "Straight", {"Unconnected": {"height": 0.0}}, x=50, y=0),
        },
    )


def write(root, slug, case, name, document):
    target = root / slug / case / "📸️snapshot" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote", target)


def main(arguments):
    root = Path(arguments[0])
    write(root, "🏠️spaces", "🏡️rooms", "Rooms", rooms())
    write(root, "🧮️quantities", "🏗️building", "Quantities", quantities())
    write(root, "🪜️stair-runs", "🪜️flights", "Stairs", stairs())
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
