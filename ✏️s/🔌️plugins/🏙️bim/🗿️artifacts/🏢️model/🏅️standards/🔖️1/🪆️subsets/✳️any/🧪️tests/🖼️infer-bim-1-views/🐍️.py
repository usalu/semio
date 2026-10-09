#!/usr/bin/env python3
"""🖼️ Third-party ORACLE for the `s.bim.model@1` inference `🖼️view-linework`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per authored view, the drawing of a section or elevation: the solids of the building are cut by the vertical plane of the view (the
section cut, a filled region per solid) and projected behind it (the silhouettes), both in `(u, z)` coordinates with `u` along the plane and `z` above the building datum, and the storey
levels run across the drawing as datum lines. None of that is stored: the snapshot holds the plane, the depth, the crop, the hidden categories and the phase of the view.
This file re-derives three measures per section and elevation from the SAME committed snapshot without sharing a line of code with the subject, and lets `shapely` 2 (GEOS), a library that
has never seen this repository, adjudicate the geometry:

* `SectionCut.area`: the plane is a `LineString`; the footprint of every wall (`buffer` of its axis with flat caps, or the mitred ring of a closed room), column (`box` rotated and translated with
  `shapely.affinity`) is a `Polygon`; the cut of a vertical prism is the length of `plane.intersection(footprint)` times its height, clipped to the crop rectangle with `shapely.geometry.box`.
* `Silhouette.union_area`: the part of the footprint inside the slab of the view (`along` the plane, `depth` behind it) is a `Polygon` intersection; a vertical prism projects to the rectangle
  `[u_min, u_max] x [base, top]` of the extent of every connected piece along the plane; the silhouette of the view is the `unary_union` of those rectangles, clipped to the crop.
* `Datum.length`: every storey level and the top of the highest storey is a horizontal line across the plane (clipped to the crop rectangle).

Hidden categories and the phase filter drop elements before anything is measured. The edges of a view are decided by hidden-line removal, which no third-party library offers; they are pinned by the
unit tests of the subject. The parametric law is a metamorphic property: moving the whole scene along the plane or turning plane and scene together by one angle leaves every measure unchanged.

The committed expectation under `🧫️fixtures/💡️inferences/🖼️view-linework/<case>/💡️inference/📐️view-metrics/🔣️.json` is WRITTEN by this file (`write`), never by hand, and the Rust subject is compared
against it.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🖼️view-linework>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🖼️view-linework>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import shapely
from shapely import affinity
from shapely.geometry import LineString, Polygon, box
from shapely.ops import unary_union

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
MEASURES = ["Silhouette.union_area", "SectionCut.area", "Datum.length"]
DRAWN = ["Section", "Elevation"]
CATEGORY = {"wall": "Walls", "column": "Columns"}


def variant(value):
    """🧩️ The tag and the payload of an externally tagged value (`{"Line": {...}}` or `"Start"`)."""
    if isinstance(value, str):
        return value, {}
    (tag, payload), = value.items()
    return tag, payload


def elevations(snapshot, building):
    """📏️ The elevation of every storey of a building and the top of the highest: sums of storey heights, exactly rounded."""
    rows = sorted(((storey["level"], key, storey) for key, storey in snapshot["storeys"].items() if storey["building"] == building), key=lambda row: row[0])
    base, top = {}, 0.0
    for level, key, storey in rows:
        if level >= 0:
            base[key] = math.fsum(row[2]["height"] for row in rows if 0 <= row[0] < level)
        else:
            base[key] = -math.fsum(row[2]["height"] for row in rows if level <= row[0] < 0)
        top = max(top, base[key] + storey["height"])
    return base, top


# endregion 🔖️Vocabulary


# region 🔖️Solids
def wall_footprints(snapshot, building):
    """🧱️ The footprints of the straight walls of a building as shapely polygons: free walls are flat-capped bands of their axis, walls that close a ring are the mitred band of the ring.

    A wall is a prism from its storey elevation to the storey top; walls of one storey that share their end points in a closed chain are one mitred ring (`buffer(join_style=mitre)`), whose cut and projection
    equal those of the four join-trimmed solids because those partition it.
    """
    base, _ = elevations(snapshot, building)
    rows = []
    walls = {key: wall for key, wall in snapshot["walls"].items() if snapshot["storeys"][wall["storey"]]["building"] == building}
    for key, wall in walls.items():
        tag, axis = variant(wall["axis"])
        assert tag == "Line", "the oracle measures straight walls"
        thickness = math.fsum(layer["thickness"] for layer in snapshot["wall_types"][wall["wall_type"]]["layers"])
        rows.append((key, wall, (axis["start"]["x"], axis["start"]["y"]), (axis["end"]["x"], axis["end"]["y"]), thickness))
    pieces, taken = [], set()
    for key, wall, start, end, thickness in rows:
        if key in taken:
            continue
        chain = [(key, wall, start, end)]
        taken.add(key)
        while chain[-1][3] != chain[0][2]:
            nxt = next((row for row in rows if row[0] not in taken and row[1]["storey"] == wall["storey"] and row[2] == chain[-1][3]), None)
            if nxt is None:
                break
            taken.add(nxt[0])
            chain.append((nxt[0], nxt[1], nxt[2], nxt[3]))
        storey = snapshot["storeys"][wall["storey"]]
        if chain[-1][3] == chain[0][2] and len(chain) >= 3:
            ring = [row[2] for row in chain] + [chain[0][2]]
            shape = LineString(ring).buffer(thickness / 2.0, cap_style="flat", join_style="mitre")
            members = [row[0] for row in chain]
        else:
            members = [row[0] for row in chain]
            shape = unary_union([LineString([row[2], row[3]]).buffer(thickness / 2.0, cap_style="flat") for row in chain])
        pieces.append({"family": "wall", "members": members, "phase": wall["phase"], "shape": shape, "base": base[wall["storey"]] + wall["base_offset"], "top": base[wall["storey"]] + storey["height"]})
    return pieces


def column_footprints(snapshot, building):
    """🏛️ The footprints of the columns of a building: the profile rectangle rotated by the column angle and moved to its position."""
    base, _ = elevations(snapshot, building)
    pieces = []
    for key, column in snapshot["columns"].items():
        storey = snapshot["storeys"][column["storey"]]
        if storey["building"] != building:
            continue
        tag, profile = variant(snapshot["column_types"][column["column_type"]]["profile"])
        assert tag == "Rectangle", "the oracle measures rectangular columns"
        shape = box(-profile["width"] / 2.0, -profile["depth"] / 2.0, profile["width"] / 2.0, profile["depth"] / 2.0)
        shape = affinity.rotate(shape, column["rotation"], origin=(0.0, 0.0), use_radians=True)
        shape = affinity.translate(shape, column["position"]["x"], column["position"]["y"])
        pieces.append({"family": "column", "members": [key], "phase": column["phase"], "shape": shape, "base": base[column["storey"]] + column["base_offset"], "top": base[column["storey"]] + storey["height"]})
    return pieces


def solids(snapshot, building):
    """📦️ Every prism of the building."""
    return wall_footprints(snapshot, building) + column_footprints(snapshot, building)


# endregion 🔖️Solids


# region 🔖️Views
def frame(plane):
    """🖼️ Origin, unit vector along the plane, unit vector it looks along (the left normal) and its length."""
    dx, dy = plane["end"]["x"] - plane["start"]["x"], plane["end"]["y"] - plane["start"]["y"]
    length = math.hypot(dx, dy)
    return (plane["start"]["x"], plane["start"]["y"]), (dx / length, dy / length), (-dy / length, dx / length), length


def parts(geometry):
    """🧩️ The polygons a geometry is made of."""
    if geometry.is_empty:
        return []
    if geometry.geom_type == "Polygon":
        return [geometry]
    return [part for member in geometry.geoms for part in parts(member)]


def crop_box(view):
    """✂️ The crop rectangle of a view in `(u, z)`, or none."""
    crop = view.get("crop")
    return box(crop["min"]["x"], crop["min"]["y"], crop["max"]["x"], crop["max"]["y"]) if crop else None


def measure_view(snapshot, view):
    """🖼️ The three measures of one section or elevation."""
    origin, along, look, length = frame(view["plane"])
    crop = crop_box(view)
    line = LineString([origin, (origin[0] + along[0] * length, origin[1] + along[1] * length)])
    strip = Polygon([origin, (origin[0] + along[0] * length, origin[1] + along[1] * length), (origin[0] + along[0] * length + look[0] * view["depth"], origin[1] + along[1] * length + look[1] * view["depth"]), (origin[0] + look[0] * view["depth"], origin[1] + look[1] * view["depth"])])
    shown = [piece for piece in solids(snapshot, view["building"]) if CATEGORY[piece["family"]] not in view["hidden"] and (view.get("phase") is None or piece["phase"] == view["phase"])]

    def u_of(x, y):
        return (x - origin[0]) * along[0] + (y - origin[1]) * along[1]

    cut = 0.0
    if view["kind"] == "Section":
        for piece in shown:
            for start, end in runs_along(line, piece["shape"], origin, along):
                region = box(start, piece["base"], end, piece["top"])
                cut += (region if crop is None else region.intersection(crop)).area
    rectangles = []
    for piece in shown:
        for part in parts(piece["shape"].intersection(strip)):
            us = [u_of(x, y) for x, y in part.exterior.coords]
            rectangle = box(max(min(us), 0.0), piece["base"], min(max(us), length), piece["top"])
            if not rectangle.is_empty:
                rectangles.append(rectangle if crop is None else rectangle.intersection(crop))
    silhouette = unary_union(rectangles).area if rectangles else 0.0
    base, top = elevations(snapshot, view["building"])
    levels = list(base.values()) + [top]
    datum = 0.0
    for level in levels:
        segment = LineString([(0.0, level), (length, level)])
        datum += (segment.intersection(crop).length if crop is not None else segment.length)
    return {"Silhouette.union_area": silhouette, "SectionCut.area": cut, "Datum.length": datum}


def runs_along(line, shape, origin, along):
    """➡️ The `(u_from, u_to)` runs where the plane line lies inside a footprint: what GEOS gives for `line ∩ shape`, ordered along the plane."""
    inside = line.intersection(shape)
    pieces = [inside] if inside.geom_type == "LineString" else list(getattr(inside, "geoms", []))
    runs = []
    for piece in pieces:
        if piece.geom_type != "LineString" or piece.is_empty:
            continue
        us = [(x - origin[0]) * along[0] + (y - origin[1]) * along[1] for x, y in piece.coords]
        runs.append((min(us), max(us)))
    return sorted(runs)


def metrics(snapshot):
    """📊️ The oracle table `view → measure → value` of every section and elevation."""
    return {key: measure_view(snapshot, view) for key, view in sorted(snapshot["views"].items()) if view["kind"] in DRAWN}


# endregion 🔖️Views


# region 🔖️Audit
def turned(snapshot, angle, shift):
    """🔄️ The snapshot with every plan point rotated by `angle` about the origin and then moved by `shift`: the oracle table must not change."""
    def point(p):
        x, y = p["x"] * math.cos(angle) - p["y"] * math.sin(angle) + shift[0], p["x"] * math.sin(angle) + p["y"] * math.cos(angle) + shift[1]
        return {"x": x, "y": y}

    moved = json.loads(json.dumps(snapshot))
    for wall in moved["walls"].values():
        tag, axis = variant(wall["axis"])
        wall["axis"] = {tag: {**axis, "start": point(axis["start"]), "end": point(axis["end"])}}
    for column in moved["columns"].values():
        column["position"] = point(column["position"])
        column["rotation"] += angle
    for view in moved["views"].values():
        if "plane" in view:
            view["plane"] = {"start": point(view["plane"]["start"]), "end": point(view["plane"]["end"])}
    return moved


def audit(snapshot, table):
    """⚖️ Every place where the measures break a law shapely can state."""
    problems = []
    for key, row in table.items():
        view = snapshot["views"][key]
        if view["kind"] == "Elevation" and row["SectionCut.area"] != 0.0:
            problems.append("%s: an elevation cuts nothing" % key)
        if row["Silhouette.union_area"] < 0 or row["SectionCut.area"] < 0:
            problems.append("%s: a negative area" % key)
    moved = metrics(turned(snapshot, 0.7, (3.25, -1.5)))
    for key, row in table.items():
        for measure in MEASURES:
            if abs(moved[key][measure] - row[measure]) > 1e-7:
                problems.append("%s.%s changes when scene and planes turn together: %s vs %s" % (key, measure, row[measure], moved[key][measure]))
    return problems


# endregion 🔖️Audit


# region 🔖️Handlers
def table_json(table):
    """🧾️ The canonical bytes of a table."""
    return json.dumps(table, separators=(",", ":"), ensure_ascii=False, sort_keys=True).encode("utf-8")


def metrics_handler(ctx):
    """🖼️ Oracle answer: the table of the committed snapshot."""
    from semio_repo_test import Outcome

    uri = next(uri for uri in ctx.step_input_uris() if "snapshot" in uri)
    table = metrics(json.loads(ctx.input_bytes(uri)))
    return Outcome(table, raw=table_json(table))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("view-metrics-room", metrics_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the laws and compares the committed table with the measurement; `write` rewrites the table from the snapshot."""
    command, root = arguments[0], Path(arguments[1])
    problems = []
    for case in sorted(path for path in root.iterdir() if path.is_dir()):
        snapshot = json.loads((case / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
        table = metrics(snapshot)
        problems += audit(snapshot, table)
        target = case / "💡️inference" / "📐️view-metrics" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(table, indent=2, ensure_ascii=False, sort_keys=True) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote %d views" % (case.name, len(table)))
        else:
            committed = json.loads(target.read_text(encoding="utf-8"))
            for key in sorted(set(committed) | set(table)):
                for measure in MEASURES:
                    left, right = committed.get(key, {}).get(measure), table.get(key, {}).get(measure)
                    if left is None or right is None or abs(left - right) > EXACT:
                        problems.append("%s: %s.%s committed %s, measured %s" % (case.name, key, measure, left, right))
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (shapely %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", shapely.__version__))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
