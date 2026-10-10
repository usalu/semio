#!/usr/bin/env python3
"""🪑️ Third-party ORACLE for the `s.bim.model@1` inferences `🪑️components` and `🌀️mep`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per component (an instance of a parametric family), the family under its parameter overrides, the position and turn in the building frame
(the host fit to a wall, the yaw, the mirror), the footprint, the bounds, the volume and the connector; per MEP element (duct, pipe, cable tray) the path in the building frame, the section, the
length, the closed-form volume and surface; the clashing pairs of different systems; the quantities per element and per group (category, system, size); and the findings of the component and MEP
codes. This file re-derives all of it from the SAME committed snapshot without sharing a line of code with the subject:

* the family under the overrides is evaluated by the independent interpreter of the families oracle (`🧬️infer-bim-1-families`, itself loaded from the Python interpreter of the expression module):
  the overrides replace the formula text of the parameter rows of a copy of the snapshot, the new issues are the ones the plain family does not have;
* the placement is rebuilt with `numpy` (projection of the position on the wall axis, the left or right face by the sign of the cross product, a `+y` pointing away from the wall, mirror then rotation)
  and the footprint, the overlaps with the walls and the distances beyond them are measured by `shapely` 2 (GEOS) on the polygons of the oriented rectangles and of the wall bands;
* the MEP closed forms (length of the path, `area * length`, `perimeter * length`) come from `numpy`; the clashing pairs from an independent segment distance (the minimum over the interior solution and the
  four boundary edges of the unit square) with a sweep over the boxes; the terminals look for a run end of their system within 5 cm.

The audit part proves the closed forms against shapely (footprint polygon area, mitre-free band area `width * length` of a straight horizontal duct) and the metamorphic laws: mirroring a hosted component
keeps its footprint area, adding a constant to the position of a hosted component along the wall moves it by exactly that constant, raising the storey height does not move a component.

The committed expectation under `🧫️fixtures/💡️inferences/🪑️components/<case>/💡️inference/🪑️components/🔣️.json` is WRITTEN by this file (`write`), never by hand, and the Rust subject is compared against it.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🪑️components>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🪑️components>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see https://shapely.readthedocs.io/en/stable/manual.html — polygons, intersections, buffers
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import numpy as np
import shapely
from shapely.geometry import Polygon, box

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
CONNECT = 0.05
FAR = 25.0
HEIGHT_EPS = 1e-6
OVERLAP_EPS = 1e-4
DEPTH_EPS = 1e-3
PALETTE = {
    "Supply": ("supply", "#1f77d4"),
    "Return": ("return", "#f08a24"),
    "Exhaust": ("exhaust", "#8b5a2b"),
    "DomesticWater": ("domestic-water", "#1ec8d6"),
    "Waste": ("waste", "#808000"),
    "Gas": ("gas", "#f2d11e"),
    "Power": ("power", "#d62728"),
    "Data": ("data", "#8e44ad"),
    "Lighting": ("lighting", "#ffbf00"),
}
CATEGORY_KEYS = {"Furniture": "furniture", "Equipment": "equipment", "Casework": "casework", "Plumbing": "plumbing", "Lighting": "lighting", "Mechanical": "mechanical", "Electrical": "electrical", "Generic": "generic", "Profile": "profile"}


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def levels_oracle():
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels")


def families_oracle():
    return load_sibling("🧬️infer-bim-1-families", "families")


def close(a, b, tolerance=1e-9):
    return abs(a - b) <= tolerance * max(1.0, abs(a), abs(b))


# endregion 🔖️Vocabulary


# region 🔖️Family
def instance_family(snapshot, component_id):
    """🧬️ The family of a component under its overrides: its row, the names that were overridden and the issues the overrides add (the plain family's own issues removed once each)."""
    oracle = families_oracle()
    component = snapshot["components"][component_id]
    family_id = component["family"]
    overrides = {row["name"]: row["value"] for row in snapshot.get("component_overrides", {}).values() if row["component"] == component_id}
    plain = oracle.family_row(snapshot, family_id)
    if not overrides:
        return plain, [], []
    changed = copy.deepcopy(snapshot)
    known = [name for name in overrides if "%s.%s" % (family_id, name) in snapshot["family_parameters"]]
    for name in known:
        changed["family_parameters"]["%s.%s" % (family_id, name)]["value"] = overrides[name]
    own = oracle.family_row(changed, family_id)
    remaining = list(plain["issues"])
    added = []
    for issue in own["issues"]:
        if issue in remaining:
            remaining.remove(issue)
        else:
            added.append(issue)
    return own, sorted(known), added


# endregion 🔖️Family


# region 🔖️Placement
def wall_offsets(snapshot, wall):
    """↔️ The distances from the axis of a wall to its left and right faces from its location line and the layers of its type."""
    layers = snapshot["wall_types"].get(wall["wall_type"], {"layers": []})["layers"]
    thickness = math.fsum(layer["thickness"] for layer in layers)
    location = wall["location"]
    if location == "Center":
        left = thickness / 2
    elif location == "Interior":
        left = 0.0
    elif location == "Exterior":
        left = thickness
    else:
        def span(function):
            hit = [index for index, layer in enumerate(layers) if layer["function"] == function]
            if not hit:
                return None
            return (math.fsum(layer["thickness"] for layer in layers[: hit[0]]), math.fsum(layer["thickness"] for layer in layers[: hit[-1] + 1]))
        core = span("Core") or span("Structure")
        left = (core[0] + core[1]) / 2 if core else thickness / 2
    return left, thickness - left


def fit(snapshot, component):
    """🧱️ The fit of the authored position to the host wall, or the reason it cannot be made."""
    wall = snapshot["walls"].get(component["host"])
    if wall is None:
        return None, "host-missing"
    if wall["storey"] != component["storey"]:
        return None, "host-other-storey"
    tag, body = next(iter(wall["axis"].items()))
    if tag != "Line":
        raise NotImplementedError("the corpus has no curved host wall; the oracle audits straight hosts only")
    start, end = np.array([body["start"]["x"], body["start"]["y"]]), np.array([body["end"]["x"], body["end"]["y"]])
    direction = end - start
    length = float(np.hypot(*direction))
    if length <= EXACT:
        return None, "host-degenerate"
    tangent = direction / length
    position = np.array([component["position"]["x"], component["position"]["y"]])
    t = float(np.clip(np.dot(position - start, tangent) / length, 0.0, 1.0))
    on_axis = start + t * direction
    left_normal = np.array([-tangent[1], tangent[0]])
    side = 1.0 if tangent[0] * (position - on_axis)[1] - tangent[1] * (position - on_axis)[0] >= 0 else -1.0
    left, right = wall_offsets(snapshot, wall)
    away = left_normal * side
    face = on_axis + away * (left if side > 0 else right)
    return {"wall": component["host"], "station": t * length, "side": side, "face": face, "normal": away}, None


def transform(origin, yaw, mirrored):
    """🧭️ The map of the family frame into the building frame: mirror x, turn about z, move."""
    sin, cos = math.sin(yaw), math.cos(yaw)

    def apply(point):
        x = -point[0] if mirrored else point[0]
        return (origin[0] + cos * x - sin * point[1], origin[1] + sin * x + cos * point[1], origin[2] + point[2])

    return apply


def levels_of(snapshot):
    return levels_oracle().storey_levels(snapshot)


def component_rows(snapshot):
    """🪑️ The row of every component and the findings about it that need no walls."""
    levels = levels_of(snapshot)
    rows, notes = {}, {}
    for component_id, component in sorted(snapshot.get("components", {}).items()):
        family = snapshot["families"].get(component["family"])
        issues, extra = [], []
        category = family["category"] if family else None
        if family is None:
            issues.append("family-missing")
            row_family, known, added = {"solids": {}, "parameters": {}}, [], []
        elif category == "Profile":
            issues.append("family-profile")
            row_family, known, added = {"solids": {}, "parameters": {}}, [], []
        else:
            row_family, known, added = instance_family(snapshot, component_id)
            issues += ["override"] * len(added)
        level = levels[component["storey"]]
        z = level["elevation"] + component["elevation"]
        hosted, reason = (None, None)
        if component.get("host") is not None:
            hosted, reason = fit(snapshot, component)
            if reason:
                issues.append(reason)
        if hosted is not None:
            origin = (float(hosted["face"][0]), float(hosted["face"][1]), z)
            yaw = math.atan2(-float(hosted["normal"][0]), float(hosted["normal"][1])) + component["rotation"]
        else:
            origin = (component["position"]["x"], component["position"]["y"], z)
            yaw = component["rotation"]
        visible = [solid for solid in row_family["solids"].values() if solid["visible"] and solid["triangles"] > 0]
        footprint, area, bounds, volume = [], 0.0, {"min": {"x": 0.0, "y": 0.0, "z": 0.0}, "max": {"x": 0.0, "y": 0.0, "z": 0.0}}, 0.0
        if visible:
            lo = np.min([solid["min"] for solid in visible], axis=0)
            hi = np.max([solid["max"] for solid in visible], axis=0)
            apply = transform(origin, yaw, component["mirrored"])
            corners = [apply((lo[0], lo[1], 0.0)), apply((hi[0], lo[1], 0.0)), apply((hi[0], hi[1], 0.0)), apply((lo[0], hi[1], 0.0))]
            if Polygon(corners).exterior.is_ccw is False:
                corners.reverse()
            footprint = [{"x": c[0], "y": c[1]} for c in corners]
            area = float((hi[0] - lo[0]) * (hi[1] - lo[1]))
            xs, ys = [c[0] for c in corners], [c[1] for c in corners]
            bounds = {"min": {"x": min(xs), "y": min(ys), "z": z + float(lo[2])}, "max": {"x": max(xs), "y": max(ys), "z": z + float(hi[2])}}
            volume = float(sum(solid["volume"] for solid in visible))
        row = {"storey": component["storey"], "family": component["family"], "x": origin[0], "y": origin[1], "z": z, "yaw_cos": math.cos(yaw), "yaw_sin": math.sin(yaw), "mirrored": component["mirrored"], "footprint": footprint, "footprint_area": area, "bounds": bounds, "volume": volume, "overridden": known, "issues": sorted(issues)}
        if category is not None:
            row["category"] = category
        if hosted is not None:
            row["host"] = {"wall": hosted["wall"], "station": hosted["station"], "side": hosted["side"], "face": {"x": float(hosted["face"][0]), "y": float(hosted["face"][1])}, "normal": {"x": float(hosted["normal"][0]) + 0.0, "y": float(hosted["normal"][1]) + 0.0}}
        if component.get("system") is not None:
            row["connector"] = {"system": component["system"], "colour": PALETTE[component["system"]][1], "position": {"x": origin[0], "y": origin[1], "z": z}}
        rows[component_id] = row
        notes[component_id] = {"added": added, "reason": reason}
    return rows, notes


# endregion 🔖️Placement


# region 🔖️Mep
def millimetres(metres):
    value = metres * 1000.0
    return str(int(round(value))) if abs(value - round(value)) < 1e-6 else "%.1f" % value


def mep_rows(snapshot):
    """🌀️ The closed forms of every MEP element."""
    levels = levels_of(snapshot)
    rows = {}
    for element_id, element in sorted(snapshot.get("mep_elements", {}).items()):
        tag, body = next(iter(element["shape"].items()))
        if tag == "Pipe":
            width = height = body["diameter"]
            area, perimeter, kind, label = math.pi * width * width / 4, math.pi * width, "pipe", "Ø" + millimetres(width)
        else:
            width, height = body["width"], body["height"]
            area, perimeter, kind, label = width * height, 2 * (width + height), tag.lower(), "%sx%s" % (millimetres(width), millimetres(height))
        elevation = levels[element["storey"]]["elevation"]
        issues, path = [], []
        if width > EXACT and height > EXACT:
            valid = True
        else:
            valid = False
            issues.append("section-degenerate")
        repeated = False
        for point in element["path"]:
            at = (point["x"], point["y"], elevation + point["z"])
            if path and math.dist(path[-1], at) <= EXACT:
                repeated = True
            else:
                path.append(at)
        if len(path) < 2 or repeated:
            issues.append("path-degenerate")
        length = float(sum(math.dist(a, b) for a, b in zip(path, path[1:])))
        buildable = valid and len(path) >= 2
        reach = max(width, height) / 2 if valid else 0.0
        arr = np.array(path) if path else np.zeros((1, 3))
        bounds = {"min": {"x": float(arr[:, 0].min() - reach), "y": float(arr[:, 1].min() - reach), "z": float(arr[:, 2].min() - reach)}, "max": {"x": float(arr[:, 0].max() + reach), "y": float(arr[:, 1].max() + reach), "z": float(arr[:, 2].max() + reach)}}
        if not path:
            bounds = {"min": {"x": 0.0, "y": 0.0, "z": 0.0}, "max": {"x": 0.0, "y": 0.0, "z": 0.0}}
        rows[element_id] = {
            "storey": element["storey"], "system": PALETTE[element["system"]][0], "colour": PALETTE[element["system"]][1], "kind": kind, "label": label, "width": width, "height": height, "area": area, "perimeter": perimeter,
            "path": [{"x": p[0], "y": p[1], "z": p[2]} for p in path], "length": length, "volume": area * length if buildable else 0.0, "surface_area": perimeter * length if buildable else 0.0, "bounds": bounds, "issues": sorted(issues),
            "_reach": reach, "_buildable": buildable, "_system": element["system"], "_path": path,
        }
    return rows


def segment_distance(p1, p2, q1, q2):
    """📏️ The shortest distance of two segments: the minimum over the unconstrained solution and the four boundary edges of the parameter square."""
    p1, p2, q1, q2 = (np.array(v, dtype=float) for v in (p1, p2, q1, q2))
    d1, d2, r = p2 - p1, q2 - q1, p1 - q1

    def at(s, t):
        return float(np.linalg.norm((p1 + s * d1) - (q1 + t * d2)))

    candidates = [at(s, t) for s in (0.0, 1.0) for t in (0.0, 1.0)]
    for s in (0.0, 1.0):
        t = float(np.clip(-np.dot(d2, p1 + s * d1 - q1) / np.dot(d2, d2), 0.0, 1.0)) if np.dot(d2, d2) > 0 else 0.0
        candidates.append(at(s, t))
    for t in (0.0, 1.0):
        s = float(np.clip(-np.dot(d1, q1 + t * d2 - p1) / np.dot(d1, d1), 0.0, 1.0)) if np.dot(d1, d1) > 0 else 0.0
        candidates.append(at(s, t))
    matrix = np.array([[np.dot(d1, d1), -np.dot(d1, d2)], [-np.dot(d1, d2), np.dot(d2, d2)]])
    if abs(np.linalg.det(matrix)) > 1e-18:
        s, t = np.linalg.solve(matrix, [-np.dot(d1, r), np.dot(d2, r)])
        if 0.0 <= s <= 1.0 and 0.0 <= t <= 1.0:
            candidates.append(at(float(s), float(t)))
    return min(candidates)


def clashes(rows):
    """💥️ The pairs of different systems whose centre lines come closer than the sum of their reaches."""
    found = []
    ids = sorted(rows)
    for index, a in enumerate(ids):
        for b in ids[index + 1 :]:
            first, second = rows[a], rows[b]
            if not (first["_buildable"] and second["_buildable"]) or first["_system"] == second["_system"]:
                continue
            reach = first["_reach"] + second["_reach"]
            distance = min(segment_distance(p, q, r, s) for p, q in zip(first["_path"], first["_path"][1:]) for r, s in zip(second["_path"], second["_path"][1:]))
            if distance < reach - 1e-9:
                found.append({"code": "mep.clash", "elements": [a, b], "missing": [], "values": {"distance": distance, "reach": reach}})
    return found


# endregion 🔖️Mep


# region 🔖️Findings
def wall_band(snapshot, wall):
    tag, body = next(iter(wall["axis"].items()))
    start, end = np.array([body["start"]["x"], body["start"]["y"]]), np.array([body["end"]["x"], body["end"]["y"]])
    direction = end - start
    tangent = direction / np.hypot(*direction)
    normal = np.array([-tangent[1], tangent[0]])
    left, right = wall_offsets(snapshot, wall)
    return Polygon([start - normal * right, end - normal * right, end + normal * left, start + normal * left]), (start, end)


def findings(snapshot, components, notes, meps):
    levels = levels_of(snapshot)
    found = []
    bands = {}
    for wall_id, wall in snapshot["walls"].items():
        bands[wall_id] = (wall, *wall_band(snapshot, wall))
    for component_id, row in components.items():
        component = snapshot["components"][component_id]
        for issue in row["issues"]:
            if issue in ("family-missing", "family-profile"):
                found.append({"code": "reference.component-family", "elements": [component_id], "missing": [component["family"]], "values": {}})
            elif issue in ("host-missing", "host-other-storey"):
                found.append({"code": "reference.component-host", "elements": [component_id], "missing": [component["host"]], "values": {}})
        added = notes[component_id]["added"]
        if added:
            subjects = sorted({issue.split("|")[2] for issue in added})
            found.append({"code": "component.override", "elements": [component_id], "missing": subjects, "values": {}})
        level = levels[component["storey"]]
        storey_walls = [(wall_id, polygon) for wall_id, (wall, polygon, _) in bands.items() if wall["storey"] == component["storey"]]
        height = level["top_elevation"] - level["elevation"]
        rel = row["z"] - level["elevation"]
        distance = 0.0
        if storey_walls:
            bx0 = min(p.bounds[0] for _, p in storey_walls)
            by0 = min(p.bounds[1] for _, p in storey_walls)
            bx1 = max(p.bounds[2] for _, p in storey_walls)
            by1 = max(p.bounds[3] for _, p in storey_walls)
            distance = math.hypot(max(bx0 - row["x"], row["x"] - bx1, 0.0), max(by0 - row["y"], row["y"] - by1, 0.0))
        if rel < -HEIGHT_EPS or rel > height + HEIGHT_EPS or distance > FAR:
            found.append({"code": "component.outside-storey", "elements": [component_id], "missing": [], "values": {"height": rel, "storey_height": height, "distance": distance}})
        if row["footprint"]:
            shape = Polygon([(p["x"], p["y"]) for p in row["footprint"]])
            hosted = row.get("host")
            candidates = []
            if hosted is None:
                candidates = storey_walls
            else:
                depth = min(0.0, min((p["x"] - hosted["face"]["x"]) * hosted["normal"]["x"] + (p["y"] - hosted["face"]["y"]) * hosted["normal"]["y"] for p in row["footprint"]))
                if -depth > DEPTH_EPS:
                    candidates = [(wall_id, polygon) for wall_id, polygon in storey_walls if wall_id == hosted["wall"]]
            for wall_id, polygon in candidates:
                wall = snapshot["walls"][wall_id]
                low, high = max(row["bounds"]["min"]["z"], levels[wall["storey"]]["elevation"]), min(row["bounds"]["max"]["z"], levels[wall["storey"]]["top_elevation"])
                area = shape.intersection(polygon).area if high - low > HEIGHT_EPS else 0.0
                if area > OVERLAP_EPS:
                    ends = bands[wall_id][2]
                    for end in ends:
                        if shape.distance(shapely.Point(*end)) < 0.3:
                            raise NotImplementedError("a component touches a wall end; the oracle audits bands away from the joins")
                    found.append({"code": "component.in-wall", "elements": [component_id, wall_id], "missing": [], "values": {"overlap_area": float(area), "overlap_height": high - low}})
        if "connector" in row:
            at = row["connector"]["position"]
            system = component["system"]
            ok = False
            for mep in meps.values():
                if mep["storey"] == component["storey"] and mep["_system"] == system and mep["_buildable"]:
                    for end in (mep["_path"][0], mep["_path"][-1]):
                        if math.dist(end, (at["x"], at["y"], at["z"])) <= CONNECT:
                            ok = True
            if not ok:
                found.append({"code": "mep.terminal-unconnected", "elements": [component_id], "missing": [], "values": {"distance": CONNECT}})
    for element_id, mep in meps.items():
        if "section-degenerate" in mep["issues"] or "path-degenerate" in mep["issues"]:
            found.append({"code": "mep.degenerate", "elements": [element_id], "missing": [], "values": {"width": mep["width"], "height": mep["height"], "length": mep["length"]}})
    found += clashes(meps)
    return sorted(found, key=lambda row: (row["code"], row["elements"], row["missing"]))


# endregion 🔖️Findings


# region 🔖️Quantities
def quantities(snapshot, components, meps):
    """🧮️ The closed-form measures per element and the sums per group."""
    rows, groups = {}, {}
    for component_id, row in components.items():
        corners = [(p["x"], p["y"]) for p in row["footprint"]]
        yaw = math.atan2(row["yaw_sin"], row["yaw_cos"])
        across = (-1.0 if row["mirrored"] else 1.0) * np.array([math.cos(yaw), math.sin(yaw)])
        along = np.array([-math.sin(yaw), math.cos(yaw)])
        width = length = 0.0
        if corners:
            width = float(np.ptp([np.dot(c, across) for c in corners]))
            length = float(np.ptp([np.dot(c, along) for c in corners]))
        names = []
        if "category" in row:
            names.append("component-category:" + CATEGORY_KEYS[row["category"]])
        if "connector" in row:
            names.append("component-system:" + PALETTE[row["connector"]["system"]][0])
        rows[component_id] = {"kind": "component", "type_id": row["family"], "length": length, "width": width, "height": max(row["bounds"]["max"]["z"] - row["bounds"]["min"]["z"], 0.0), "perimeter": 2 * (width + length), "gross_area": row["footprint_area"], "net_area": row["footprint_area"], "gross_volume": row["volume"], "groups": names}
    for element_id, row in meps.items():
        rows[element_id] = {"kind": "mep", "type_id": row["system"], "length": row["length"], "width": row["width"], "height": row["height"], "perimeter": row["perimeter"], "gross_area": row["area"], "net_area": row["surface_area"], "gross_volume": row["volume"], "groups": ["mep-system:" + row["system"], "mep-size:" + row["label"]]}
    for element_id, row in rows.items():
        for group in row["groups"]:
            total = groups.setdefault(group, {"count": 0, "length": 0.0, "area": 0.0, "gross_volume": 0.0})
            total["count"] += 1
            total["length"] += row["length"]
            total["area"] += row["net_area"]
            total["gross_volume"] += row["gross_volume"]
    return rows, groups


# endregion 🔖️Quantities


# region 🔖️Table
def table(snapshot):
    components, notes = component_rows(snapshot)
    meps = mep_rows(snapshot)
    measured, groups = quantities(snapshot, components, meps)
    return {
        "components": components,
        "mep": {key: {name: value for name, value in row.items() if not name.startswith("_")} for key, row in meps.items()},
        "quantities": measured,
        "groups": groups,
        "findings": findings(snapshot, components, notes, meps),
    }


def audit(snapshot):
    """🩺️ Closed forms against shapely and the metamorphic laws."""
    problems = []
    computed = table(snapshot)
    for component_id, row in computed["components"].items():
        if row["footprint"]:
            polygon = Polygon([(p["x"], p["y"]) for p in row["footprint"]])
            if not close(polygon.area, row["footprint_area"], 1e-9):
                problems.append("%s: the GEOS area %.12g is not the product of the extents %.12g" % (component_id, polygon.area, row["footprint_area"]))
    for element_id, row in computed["mep"].items():
        if row["issues"]:
            continue
        element = snapshot["mep_elements"][element_id]
        if row["kind"] == "duct" and all(abs(a["z"] - b["z"]) < EXACT for a, b in zip(row["path"], row["path"][1:])) and len(row["path"]) == 2:
            band = shapely.LineString([(p["x"], p["y"]) for p in row["path"]]).buffer(row["width"] / 2, cap_style="flat")
            if not close(band.area, row["width"] * row["length"], 1e-9):
                problems.append("%s: the flat buffer of the centre line covers %.12g, not width x length" % (element_id, band.area))
    for component_id, component in snapshot["components"].items():
        row = computed["components"][component_id]
        if component.get("host") and row.get("host") and not row["issues"]:
            moved = copy.deepcopy(snapshot)
            moved["components"][component_id]["mirrored"] = not component["mirrored"]
            flipped = table(moved)["components"][component_id]
            if not close(flipped["footprint_area"], row["footprint_area"]):
                problems.append("%s: mirroring changed the footprint area" % component_id)
            wall = snapshot["walls"][component["host"]]
            tag, body = next(iter(wall["axis"].items()))
            tangent = np.array([body["end"]["x"] - body["start"]["x"], body["end"]["y"] - body["start"]["y"]])
            tangent = tangent / np.hypot(*tangent)
            shifted = copy.deepcopy(snapshot)
            shifted["components"][component_id]["position"]["x"] += 0.25 * tangent[0]
            shifted["components"][component_id]["position"]["y"] += 0.25 * tangent[1]
            after = table(shifted)["components"][component_id]
            if not (close(after["x"] - row["x"], 0.25 * tangent[0]) and close(after["y"] - row["y"], 0.25 * tangent[1]) and close(after["host"]["station"] - row["host"]["station"], 0.25)):
                problems.append("%s: moving along the wall by 0.25 m did not move the component by 0.25 m" % component_id)
    taller = copy.deepcopy(snapshot)
    for storey in taller["storeys"].values():
        storey["height"] += 1.0
    again = table(taller)
    for component_id, row in computed["components"].items():
        if snapshot["storeys"][snapshot["components"][component_id]["storey"]]["level"] == 0 and not close(again["components"][component_id]["z"], row["z"]):
            problems.append("%s: raising the storey height moved the component on the ground storey" % component_id)
    return problems


def differences(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table, numbers within 1e-9 relative."""
    if isinstance(expected, dict) and isinstance(actual, dict):
        found = ["%s: member %s missing" % (path, key) for key in expected if key not in actual] + ["%s: unexpected member %s" % (path, key) for key in actual if key not in expected]
        for key in expected.keys() & actual.keys():
            found += differences(expected[key], actual[key], path + "/" + key)
        return found
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return ["%s: %d items, expected %d" % (path, len(actual), len(expected))]
        return [d for i, (e, a) in enumerate(zip(expected, actual)) for d in differences(e, a, "%s[%d]" % (path, i))]
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)) and not isinstance(expected, bool):
        return [] if close(float(expected), float(actual)) else ["%s: %r, expected %r" % (path, actual, expected)]
    return [] if expected == actual else ["%s: %r, expected %r" % (path, actual, expected)]


# endregion 🔖️Table


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def components_handler(ctx):
    """🪑️ Oracle answer for `🪑️components`, after the closed forms and the metamorphic laws agreed with it."""
    from semio_repo_test import Outcome

    snapshot = case_snapshot(ctx)
    problems = audit(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    payload = table(snapshot)
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("components-room", components_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in audit(snapshot)]
        computed = table(snapshot)
        target = case / "💡️inference" / "🪑️components" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in differences(json.loads(target.read_text(encoding="utf-8")), computed)]
        print("%s: shapely %s, %d components, %d MEP elements, %d findings" % (case.name, shapely.__version__, len(computed["components"]), len(computed["mep"]), len(computed["findings"])))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
