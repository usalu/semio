#!/usr/bin/env python3
"""🧊️ Writes the authored `snapshot` of every `🧊️element-solids` fixture case of the frame, slab, roof, stair and railing families.

Keeps the `expected` and `meshes` members of an existing file (the oracle writes `expected`):

    python r4-i-solids-rest-fixtures.py <repo root>
"""

import json
import math
import sys
from pathlib import Path

SUBSET = Path("✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/🧊️element-solids")


def material(name, category, r, g, b, density):
    return {"name": name, "category": category, "color": {"r": r, "g": g, "b": b}, "density": density, "conductivity": 1.0, "specific_heat": 900.0}


MATERIALS = {
    "m-concrete": material("Concrete", "Concrete", 0.6, 0.6, 0.6, 2400.0),
    "m-steel": material("Steel", "Metal", 0.6, 0.62, 0.65, 7850.0),
    "m-wood": material("Oak", "Wood", 0.55, 0.4, 0.2, 700.0),
    "m-clay": material("Clay Tile", "Masonry", 0.7, 0.3, 0.2, 1900.0),
}


def layer(material_id, thickness, function="Structure"):
    return {"material": material_id, "thickness": thickness, "function": function}


def point(x, y):
    return {"x": x, "y": y}


def vertex(x, y, bulge=0.0):
    return {"point": point(x, y), "bulge": bulge}


def polygon(*pairs):
    return [vertex(x, y) for x, y in pairs]


def base(name, storeys=None):
    return {
        "schema": "s.bim.model@1",
        "project": {"name": name, "description": "", "author": "", "organization": "", "phase_names": []},
        "materials": MATERIALS,
        "sites": {"site-1": {"name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 100.0, "true_north": 0.0, "boundary": []}},
        "buildings": {"bldg-1": {"site": "site-1", "name": "Frame", "origin": point(0.0, 0.0), "rotation": 0.0, "elevation": 2.5}},
        "storeys": storeys
        or {
            "st-ground": {"building": "bldg-1", "name": "Ground", "level": 0, "height": 3.0},
            "st-first": {"building": "bldg-1", "name": "First", "level": 1, "height": 2.8},
        },
    }


def columns():
    snapshot = base("Columns")
    snapshot["column_types"] = {
        "ct-rect": {"name": "Rect 400x300", "profile": {"Rectangle": {"width": 0.4, "depth": 0.3}}, "material": "m-concrete"},
        "ct-circle": {"name": "Round 500", "profile": {"Circle": {"diameter": 0.5}}, "material": "m-concrete"},
        "ct-i": {"name": "I 200x300", "profile": {"IShape": {"width": 0.2, "depth": 0.3, "web": 0.01, "flange": 0.015}}, "material": "m-steel"},
        "ct-custom": {
            "name": "L angle",
            "profile": {"Custom": {"outline": polygon((0.0, 0.0), (0.4, 0.0), (0.4, 0.1), (0.1, 0.1), (0.1, 0.4), (0.0, 0.4))}},
            "material": "m-steel",
        },
        "ct-bad": {"name": "Zero", "profile": {"Rectangle": {"width": 0.0, "depth": 0.3}}, "material": "m-concrete"},
    }

    def column(storey, kind, x, y, rotation, base_offset, top, name):
        return {"storey": storey, "column_type": kind, "position": point(x, y), "rotation": rotation, "base_offset": base_offset, "top": top, "name": name}

    snapshot["columns"] = {
        "c-rect": column("st-ground", "ct-rect", 1.0, 2.0, 0.5, 0.0, {"StoreyTop": {"offset": 0.0}}, "Rect"),
        "c-circle": column("st-ground", "ct-circle", 4.0, 2.0, 0.0, 0.1, {"Storey": {"storey": "st-first", "offset": -0.2}}, "Circle"),
        "c-i": column("st-first", "ct-i", 2.0, 5.0, 1.0, 0.2, {"Unconnected": {"height": 2.0}}, "I"),
        "c-custom": column("st-ground", "ct-custom", 6.0, 1.0, -0.3, 0.0, {"StoreyTop": {"offset": 0.5}}, "Custom"),
        "c-bad-type": column("st-ground", "ct-bad", 0.0, 0.0, 0.0, 0.0, {"StoreyTop": {"offset": 0.0}}, "Bad type"),
        "c-flat": column("st-ground", "ct-rect", 0.0, 4.0, 0.0, 0.0, {"Unconnected": {"height": 0.0}}, "No height"),
        "c-unknown-type": column("st-ground", "ct-none", 0.0, 6.0, 0.0, 0.0, {"StoreyTop": {"offset": 0.0}}, "Unknown type"),
    }
    return snapshot


def beams():
    snapshot = base("Beams")
    snapshot["beam_types"] = {
        "bt-rect": {"name": "Rect 200x400", "profile": {"Rectangle": {"width": 0.2, "depth": 0.4}}, "material": "m-concrete"},
        "bt-i": {"name": "I 150x300", "profile": {"IShape": {"width": 0.15, "depth": 0.3, "web": 0.008, "flange": 0.012}}, "material": "m-steel"},
        "bt-circle": {"name": "Pipe 100", "profile": {"Circle": {"diameter": 0.1}}, "material": "m-steel"},
    }

    def beam(storey, kind, start, end, top_offset, name):
        return {"storey": storey, "beam_type": kind, "start": point(*start), "end": point(*end), "top_offset": top_offset, "name": name}

    snapshot["beams"] = {
        "b-along-x": beam("st-ground", "bt-rect", (0.0, 0.0), (6.0, 0.0), -0.1, "Along x"),
        "b-diagonal": beam("st-first", "bt-rect", (0.0, 0.0), (3.0, 4.0), 0.0, "Diagonal"),
        "b-i": beam("st-ground", "bt-i", (1.0, 1.0), (1.0, 7.0), -0.2, "I beam"),
        "b-pipe": beam("st-ground", "bt-circle", (0.0, 2.0), (4.0, 2.0), 0.0, "Pipe"),
        "b-zero": beam("st-ground", "bt-rect", (2.0, 2.0), (2.0, 2.0), 0.0, "Zero length"),
        "b-unknown-type": beam("st-ground", "bt-none", (0.0, 0.0), (1.0, 0.0), 0.0, "Unknown type"),
    }
    return snapshot


def slabs():
    snapshot = base("Slabs")
    snapshot["slab_types"] = {
        "slt-two": {"name": "Floor", "layers": [layer("m-wood", 0.05, "Finish"), layer("m-concrete", 0.2)]},
        "slt-bare": {"name": "Bare", "layers": []},
        "slt-thin": {"name": "Thin", "layers": [layer("m-concrete", 0.15)]},
    }

    def slab(storey, kind, boundary, holes, offset, slope, name):
        row = {"storey": storey, "slab_type": kind, "boundary": boundary, "holes": holes, "offset": offset, "name": name}
        if slope is not None:
            row["slope"] = slope
        return row

    d_shape = [vertex(0.0, 0.0, 0.0), vertex(4.0, 0.0, 1.0), vertex(4.0, 4.0, 0.0), vertex(0.0, 4.0, 0.0)]
    snapshot["slabs"] = {
        "s-holed": slab("st-ground", "slt-two", polygon((0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)), [polygon((2.0, 1.0), (3.0, 1.0), (3.0, 2.0), (2.0, 2.0))], 0.0, None, "Holed"),
        "s-sloped": slab("st-ground", "slt-thin", polygon((0.0, 0.0), (5.0, 0.0), (5.0, 4.0), (0.0, 4.0)), [], -0.05, {"direction": 0.0, "angle": 0.05}, "Sloped"),
        "s-diagonal-slope": slab("st-first", "slt-thin", polygon((0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)), [], -0.1, {"direction": math.pi / 3, "angle": 0.1}, "Diagonal fall"),
        "s-curved": slab("st-first", "slt-two", d_shape, [[vertex(1.0, 2.0, 1.0), vertex(2.0, 2.0, 1.0)]], -0.1, None, "Curved with round hole"),
        "s-bare": slab("st-ground", "slt-bare", polygon((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)), [], 0.0, None, "No layers"),
        "s-unknown-type": slab("st-ground", "slt-none", polygon((0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)), [], 0.0, None, "Unknown type"),
    }
    return snapshot


def roofs():
    snapshot = base("Roofs")
    snapshot["roof_types"] = {
        "rt-tiles": {"name": "Tiled", "layers": [layer("m-clay", 0.04, "Finish"), layer("m-wood", 0.06)]},
        "rt-flat": {"name": "Flat", "layers": [layer("m-concrete", 0.2)]},
    }
    rectangle = polygon((0.0, 0.0), (8.0, 0.0), (8.0, 6.0), (0.0, 6.0))

    def roof(kind, footprint, shape, overhang, base_offset, name):
        return {"storey": "st-first", "roof_type": kind, "footprint": footprint, "shape": shape, "overhang": overhang, "base_offset": base_offset, "name": name}

    l_shape = polygon((0.0, 0.0), (8.0, 0.0), (8.0, 3.0), (4.0, 3.0), (4.0, 6.0), (0.0, 6.0))
    d_shape = [vertex(0.0, 0.0), vertex(4.0, 0.0, 1.0), vertex(4.0, 4.0), vertex(0.0, 4.0)]
    snapshot["roofs"] = {
        "r-flat": roof("rt-flat", rectangle, "Flat", 0.5, 0.0, "Flat"),
        "r-shed": roof("rt-tiles", rectangle, {"Shed": {"pitch": 0.2, "direction": -math.pi / 2}}, 0.0, 0.1, "Shed"),
        "r-gable": roof("rt-tiles", rectangle, {"Gable": {"pitch": 0.5, "ridge_direction": 0.0}}, 0.5, 0.0, "Gable"),
        "r-gable-rotated": roof("rt-tiles", polygon((0.0, 0.0), (6.0, 1.0), (7.0, 5.0), (1.0, 4.0)), {"Gable": {"pitch": 0.4, "ridge_direction": 0.3}}, 0.0, 0.0, "Gable on a skewed quad"),
        "r-hip": roof("rt-tiles", rectangle, {"Hip": {"pitch": 0.4}}, 0.0, 0.0, "Hip"),
        "r-hip-overhang": roof("rt-tiles", rectangle, {"Hip": {"pitch": 0.6}}, 0.4, 0.0, "Hip with overhang"),
        "r-mansard": roof("rt-tiles", rectangle, {"Mansard": {"lower_pitch": 1.1, "upper_pitch": 0.3, "break_height": 1.0}}, 0.0, 0.0, "Mansard"),
        "r-l-hip": roof("rt-tiles", l_shape, {"Hip": {"pitch": 0.4}}, 0.0, 0.0, "Hip on an L footprint falls back"),
        "r-curved-gable": roof("rt-tiles", d_shape, {"Gable": {"pitch": 0.4, "ridge_direction": 0.0}}, 0.0, 0.0, "Gable on a curved footprint falls back"),
        "r-curved-shed": roof("rt-flat", d_shape, {"Shed": {"pitch": 0.1, "direction": math.pi}}, 0.0, 0.0, "Shed over a curved footprint"),
        "r-bad-pitch": roof("rt-tiles", rectangle, {"Hip": {"pitch": 1.7}}, 0.0, 0.0, "Pitch beyond vertical falls back"),
        "r-unknown-type": roof("rt-none", rectangle, "Flat", 0.0, 0.0, "Unknown type"),
    }
    return snapshot


def stairs():
    snapshot = base("Stairs")

    def stair(flight, top, x, y, direction, width, name, max_riser=0.18, min_tread=0.25):
        return {"storey": "st-ground", "start": point(x, y), "direction": direction, "width": width, "flight": flight, "top": top, "max_riser": max_riser, "min_tread": min_tread, "name": name}

    snapshot["stairs"] = {
        "st-straight": stair("Straight", {"StoreyTop": {"offset": 0.0}}, 0.0, 0.0, 0.0, 1.0, "Straight"),
        "st-straight-free": stair("Straight", {"Unconnected": {"height": 2.4}}, 0.0, 3.0, math.pi / 2, 0.9, "Straight, own height"),
        "st-l-left": stair({"LTurn": {"split": 0.5, "turn": "Left"}}, {"StoreyTop": {"offset": 0.0}}, 10.0, 0.0, 0.0, 1.0, "L turn left"),
        "st-l-right": stair({"LTurn": {"split": 0.4, "turn": "Right"}}, {"Storey": {"storey": "st-first", "offset": -0.1}}, 10.0, 8.0, 0.0, 1.2, "L turn right"),
        "st-u": stair({"UTurn": {"gap": 0.1}}, {"StoreyTop": {"offset": 0.0}}, 20.0, 0.0, math.pi / 2, 1.0, "U turn"),
        "st-spiral": stair({"Spiral": {"radius": 1.5, "sweep": 4.5}}, {"StoreyTop": {"offset": 0.0}}, 30.0, 0.0, 0.0, 0.9, "Spiral", 0.2, 0.2),
        "st-spiral-cw": stair({"Spiral": {"radius": 1.2, "sweep": -4.0}}, {"Unconnected": {"height": 2.6}}, 36.0, 0.0, 0.0, 0.8, "Clockwise spiral", 0.2, 0.2),
        "st-one-riser": stair("Straight", {"Unconnected": {"height": 0.15}}, 40.0, 0.0, 0.0, 1.0, "One riser"),
        "st-no-rise": stair("Straight", {"Unconnected": {"height": 0.0}}, 42.0, 0.0, 0.0, 1.0, "No rise"),
    }
    return snapshot


def railings():
    snapshot = base("Railings")

    def railing(path, height, spacing, base_offset, name):
        return {"storey": "st-ground", "path": [point(x, y) for x, y in path], "height": height, "post_spacing": spacing, "material": "m-steel", "base_offset": base_offset, "name": name}

    snapshot["railings"] = {
        "r-straight": railing([(0.0, 0.0), (4.0, 0.0)], 1.0, 1.0, 0.0, "Straight"),
        "r-l": railing([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0)], 0.9, 0.8, 0.1, "L shaped"),
        "r-u": railing([(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)], 1.1, 1.5, 0.0, "U shaped"),
        "r-no-spacing": railing([(0.0, 5.0), (2.5, 5.0)], 1.0, 0.0, 0.0, "Vertex posts only"),
        "r-point": railing([(1.0, 1.0)], 1.0, 1.0, 0.0, "One point"),
        "r-low": railing([(0.0, 6.0), (1.0, 6.0)], 0.03, 1.0, 0.0, "Lower than the rail"),
    }
    return snapshot


CASES = {
    "columns-profiles": columns,
    "beams-profiles": beams,
    "slabs-holes-slope": slabs,
    "roofs-shapes": roofs,
    "stairs-flights": stairs,
    "railings-posts": railings,
}


def main(argv):
    root = Path(argv[0]) if argv else Path(".")
    for name, build in CASES.items():
        file = root / SUBSET / name / "🔣️.json"
        file.parent.mkdir(parents=True, exist_ok=True)
        previous = json.loads(file.read_text(encoding="utf-8")) if file.exists() else {}
        document = {"snapshot": build(), "expected": previous.get("expected", {}), "meshes": previous.get("meshes", {})}
        file.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print("wrote", file)


if __name__ == "__main__":
    main(sys.argv[1:])
