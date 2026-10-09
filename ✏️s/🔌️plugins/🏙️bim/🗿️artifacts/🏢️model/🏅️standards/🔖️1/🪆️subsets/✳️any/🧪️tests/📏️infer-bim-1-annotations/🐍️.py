#!/usr/bin/env python3
"""📏️ Third-party ORACLE for the `s.bim.model@1` inference `🪧️annotation-layout`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per annotated storey, the distance every dimension prints (the difference of the positions of its
anchors along its measuring direction), the text every tag reads from its element, and the findings about the annotations: anchors that no longer have geometry,
locks that are not kept, tags that print nothing. None of that is stored: the snapshot holds the anchors, the direction, the offset, the style and the lock only.
This file re-derives the same table from the SAME committed snapshots without sharing a line of code with the subject, and lets `shapely` 2 (GEOS), a library
that has never seen this repository, adjudicate the geometry: every wall face is an `offset_curve` of the axis LineString, the centre of an opening is
`LineString.interpolate` at its offset, and the clear distance between two parallel faces must equal the GEOS `distance` between them. The parametric law is a
metamorphic property: lengthening a wall by `delta` lengthens its end-to-end dimension by `delta`, and moving a column by a vector moves its tag by the same vector.

The committed expectation under `🧫️fixtures/💡️inferences/🪧️annotation-layout/<case>/💡️inference/📏️annotations/🔣️.json` is WRITTEN by this file (`write`), never by hand,
and the Rust subject is compared against it.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🪧️annotation-layout>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🪧️annotation-layout>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import sys
from pathlib import Path

import shapely
from shapely.geometry import LineString, Point

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
LOCK_TOLERANCE = 1e-6
PARALLEL = 1e-9
UNITS = {"Metre": 1.0, "Centimetre": 100.0, "Millimetre": 1000.0}


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


def variant(value):
    """🧩️ The tag and the payload of an externally tagged value (`{"Line": {...}}` or `"Start"`)."""
    if isinstance(value, str):
        return value, {}
    (tag, payload), = value.items()
    return tag, payload


def printed(metres, unit, precision):
    """🔢️ A length in metres printed in a unit with a number of decimals; negative zero prints as zero."""
    text = "%.*f" % (precision, metres * UNITS[unit])
    return text[1:] if text.startswith("-") and set(text[1:]) <= set("0.") else text


# endregion 🔖️Vocabulary


# region 🔖️Geometry
def axis_line(axis):
    tag, body = variant(axis)
    return LineString([(body["start"]["x"], body["start"]["y"]), (body["end"]["x"], body["end"]["y"])]) if tag == "Line" else None


def wall_offsets(snapshot, wall):
    """↔️ The distances from the axis to the left and the right face from the layers of the type and the location line."""
    layers = snapshot["wall_types"][wall["wall_type"]]["layers"]
    thickness = math.fsum(layer["thickness"] for layer in layers)
    left = {"Center": thickness / 2, "Interior": 0.0, "Exterior": thickness}.get(wall["location"])
    if left is None:
        cores = [layer for layer in layers if layer["function"] == "Core"] or [layer for layer in layers if layer["function"] == "Structure"]
        if cores:
            first = layers.index(cores[0])
            last = len(layers) - 1 - [layer["function"] for layer in reversed(layers)].index(cores[0]["function"])
            left = (math.fsum(layer["thickness"] for layer in layers[:first]) + math.fsum(layer["thickness"] for layer in layers[: last + 1])) / 2
        else:
            left = thickness / 2
    return left, thickness - left, thickness


def face_line(snapshot, wall_id, side):
    """〰️ The straight face of a wall as a GEOS offset curve of the axis (left is positive); `None` for a curved wall."""
    wall = snapshot["walls"][wall_id]
    axis = axis_line(wall["axis"])
    if axis is None or axis.length < EXACT:
        return None
    left, right, _ = wall_offsets(snapshot, wall)
    return axis.offset_curve(left if side == "Left" else -right)


def reference(snapshot, anchor):
    """⚓️ The geometry of an anchor: ("point", (x, y)), ("line", LineString), or ("none", reason)."""
    tag, body = variant(anchor)
    if tag == "Point":
        return "point", (body["point"]["x"], body["point"]["y"])
    if tag in ("WallFace", "WallAxis", "WallEnd"):
        if body["wall"] not in snapshot["walls"]:
            return "none", "missing"
        wall = snapshot["walls"][body["wall"]]
        if tag == "WallEnd":
            _, axis = variant(wall["axis"])
            end = axis["start"] if body["end"] == "Start" else axis["end"]
            return "point", (end["x"], end["y"])
        line = face_line(snapshot, body["wall"], body["side"]) if tag == "WallFace" else axis_line(wall["axis"])
        if line is None:
            return "none", "curved"
        return ("line", line) if line.length > EXACT else ("none", "degenerate")
    if tag == "OpeningCentre":
        opening = snapshot["openings"].get(body["opening"])
        host = opening and (snapshot["walls"].get(opening["host"]) or snapshot["curtain_walls"].get(opening["host"]))
        if not host:
            return "none", "missing"
        line = LineString([(variant(host["axis"])[1]["start"]["x"], variant(host["axis"])[1]["start"]["y"]), (variant(host["axis"])[1]["end"]["x"], variant(host["axis"])[1]["end"]["y"])])
        centre = line.interpolate(opening["offset"])
        return "point", (centre.x, centre.y)
    if tag == "Grid":
        grid = snapshot["grids"].get(body["grid"])
        if not grid:
            return "none", "missing"
        return "line", LineString([(grid["start"]["x"], grid["start"]["y"]), (grid["end"]["x"], grid["end"]["y"])])
    column = snapshot["columns"].get(body["column"])
    return ("point", (column["position"]["x"], column["position"]["y"])) if column else ("none", "missing")


def pick(kind, value):
    """👆️ The point a dimension picks its base by: the point, or the middle of the line."""
    return value if kind == "point" else value.interpolate(0.5, normalized=True).coords[0]


def positions(snapshot, dimension):
    """📏️ The positions of the anchors along the measuring direction, or the reasons they have none."""
    direction = (math.cos(dimension["angle"]), math.sin(dimension["angle"]))
    normal = (-direction[1], direction[0])
    refs = [reference(snapshot, anchor) for anchor in dimension["anchors"]]
    reasons = [value for kind, value in refs if kind == "none"]
    if reasons:
        return None, reasons, direction
    base = pick(*refs[0])
    line = base[0] * normal[0] + base[1] * normal[1] + dimension["offset"]
    found = []
    for kind, value in refs:
        if kind == "point":
            found.append(value[0] * direction[0] + value[1] * direction[1])
            continue
        (ax, ay), (bx, by) = value.coords[0], value.coords[-1]
        span = (bx - ax, by - ay)
        across = span[0] * normal[0] + span[1] * normal[1]
        if abs(across) <= PARALLEL * max(math.hypot(*span), 1.0):
            return None, ["parallel"], direction
        t = (line - (ax * normal[0] + ay * normal[1])) / across
        crossing = (ax + t * span[0], ay + t * span[1])
        found.append(crossing[0] * direction[0] + crossing[1] * direction[1])
    return found, [], direction


# endregion 🔖️Geometry


# region 🔖️Texts
def element_name(snapshot, element):
    for collection in ("walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces"):
        if element in snapshot[collection]:
            return snapshot[collection][element]["name"]
    return snapshot["grids"][element]["label"] if element in snapshot["grids"] else None


def element_type(snapshot, element):
    for collection, field, library in (("walls", "wall_type", "wall_types"), ("columns", "column_type", "column_types"), ("beams", "beam_type", "beam_types"), ("slabs", "slab_type", "slab_types"), ("roofs", "roof_type", "roof_types")):
        if element in snapshot[collection]:
            return snapshot[library].get(snapshot[collection][element][field], {}).get("name")
    if element in snapshot["openings"]:
        tag, body = variant(snapshot["openings"][element]["kind"])
        library = {"Window": ("window_types", "window_type"), "Door": ("door_types", "door_type")}.get(tag)
        return snapshot[library[0]].get(body[library[1]], {}).get("name") if library else None
    return snapshot["spaces"][element]["usage"] or None if element in snapshot["spaces"] else None


def element_size(snapshot, element, unit, precision):
    pair = lambda a, b: "%s × %s" % (printed(a, unit, precision), printed(b, unit, precision))
    if element in snapshot["openings"]:
        opening = snapshot["openings"][element]
        tag, body = variant(opening["kind"])
        kind = {"Window": snapshot["window_types"].get(body.get("window_type"), {}), "Door": snapshot["door_types"].get(body.get("door_type"), {}), "Void": body}[tag]
        return pair(opening.get("width") if opening.get("width") is not None else kind.get("width", 0.0), opening.get("height") if opening.get("height") is not None else kind.get("height", 0.0))
    if element in snapshot["walls"]:
        return printed(wall_offsets(snapshot, snapshot["walls"][element])[2], unit, precision)
    for collection, field, library in (("columns", "column_type", "column_types"), ("beams", "beam_type", "beam_types")):
        if element in snapshot[collection]:
            profile = snapshot[library][snapshot[collection][element][field]]["profile"]
            tag, body = variant(profile)
            if tag == "Rectangle":
                return pair(body["width"], body["depth"])
    return None


def tag_text(snapshot, tag, style):
    category, element = tag["category"], tag["element"]
    unit, precision = style["unit"], style["precision"]
    text = {"Name": lambda: element_name(snapshot, element), "Type": lambda: element_type(snapshot, element), "Number": lambda: snapshot["spaces"].get(element, {}).get("number"), "Size": lambda: element_size(snapshot, element, unit, precision)}[category]()
    return text or ""


# endregion 🔖️Texts


# region 🔖️Table
def element_exists(snapshot, element):
    return any(element in snapshot[collection] for collection in ("walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "grids"))


def storey_table(snapshot, storey):
    """📏️ The oracle table of one storey: the printed distances, the tag texts and the findings."""
    dimensions, tags, findings = {}, {}, []
    for dimension_id, dimension in sorted(snapshot.get("dimensions", {}).items()):
        if dimension["storey"] != storey:
            continue
        style = snapshot["annotation_styles"].get(dimension["style"])
        if style is None:
            findings.append("annotation.style-missing|" + dimension_id)
        found, reasons, _ = positions(snapshot, dimension)
        for reason in reasons:
            findings.append(("annotation.anchor-missing|" if reason == "missing" else "annotation.anchor-unresolved|") + dimension_id)
        if found is None:
            continue
        segments = [abs(b - a) for a, b in zip(found, found[1:])]
        total = abs(found[-1] - found[0])
        dimensions[dimension_id] = {"segments": segments, "total": total}
        if any(segment <= EXACT for segment in segments):
            findings.append("annotation.dimension-zero|" + dimension_id)
        if dimension.get("lock") is not None and abs(total - dimension["lock"]) > LOCK_TOLERANCE:
            findings.append("annotation.dimension-lock-violated|" + dimension_id)
    for tag_id, tag in sorted(snapshot.get("tags", {}).items()):
        if tag["storey"] != storey:
            continue
        style = snapshot["annotation_styles"].get(tag["style"])
        if style is None:
            findings.append("annotation.style-missing|" + tag_id)
        if not element_exists(snapshot, tag["element"]):
            findings.append("annotation.anchor-missing|" + tag_id)
            continue
        text = tag_text(snapshot, tag, style or {"unit": "Metre", "precision": 2})
        tags[tag_id] = text
        if not text:
            findings.append("annotation.tag-empty|" + tag_id)
    for collection in ("text_notes", "leaders"):
        for item_id, item in sorted(snapshot.get(collection, {}).items()):
            if item["storey"] == storey and item["style"] not in snapshot["annotation_styles"]:
                findings.append("annotation.style-missing|" + item_id)
            if collection == "leaders" and item["storey"] == storey:
                kind, value = reference(snapshot, item["anchor"])
                if kind == "none":
                    findings.append(("annotation.anchor-missing|" if value == "missing" else "annotation.anchor-unresolved|") + item_id)
    return {"dimensions": dimensions, "tags": tags, "findings": sorted(set(findings))}


def table(snapshot):
    """📏️ The oracle table of every annotated storey."""
    annotated = sorted({item["storey"] for collection in ("dimensions", "tags", "text_notes", "leaders") for item in snapshot.get(collection, {}).values()})
    return {storey: storey_table(snapshot, storey) for storey in annotated}


# endregion 🔖️Table


# region 🔖️Audit
def audit(snapshot):
    """🩺️ GEOS adjudicates the faces, and the parametric laws hold on perturbed copies of the model."""
    problems = []
    for dimension_id, dimension in snapshot.get("dimensions", {}).items():
        refs = [reference(snapshot, anchor) for anchor in dimension["anchors"]]
        found, reasons, direction = positions(snapshot, dimension)
        if found is None or len(refs) != 2 or any(kind != "line" for kind, _ in refs):
            continue
        first, second = refs[0][1], refs[1][1]
        u = (first.coords[-1][0] - first.coords[0][0], first.coords[-1][1] - first.coords[0][1])
        v = (second.coords[-1][0] - second.coords[0][0], second.coords[-1][1] - second.coords[0][1])
        cross = abs(u[0] * v[1] - u[1] * v[0]) / (math.hypot(*u) * math.hypot(*v))
        along = abs(u[0] * direction[0] + u[1] * direction[1]) / math.hypot(*u)
        if cross < EXACT and along < EXACT and abs(first.distance(second) - abs(found[1] - found[0])) > 1e-7:
            problems.append("%s: GEOS measures %.9g between the faces, the dimension %.9g" % (dimension_id, first.distance(second), abs(found[1] - found[0])))
    for dimension_id, dimension in snapshot.get("dimensions", {}).items():
        anchors = [variant(anchor) for anchor in dimension["anchors"]]
        ends = [(tag, body) for tag, body in anchors if tag == "WallEnd"]
        if len(anchors) == 2 and len(ends) == 2 and ends[0][1]["wall"] == ends[1][1]["wall"] and ends[0][1]["end"] != ends[1][1]["end"] and abs(math.sin(dimension["angle"])) < EXACT:
            wall = snapshot["walls"][ends[0][1]["wall"]]
            tag, body = variant(wall["axis"])
            if tag != "Line" or abs(body["end"]["y"] - body["start"]["y"]) > EXACT:
                continue
            before = table(snapshot)[dimension["storey"]]["dimensions"][dimension_id]["total"]
            lengthened = copy.deepcopy(snapshot)
            moved = lengthened["walls"][ends[0][1]["wall"]]["axis"]["Line"]
            moved["end"]["x"] += 0.75 if body["end"]["x"] >= body["start"]["x"] else -0.75
            after = table(lengthened)[dimension["storey"]]["dimensions"][dimension_id]["total"]
            if abs(after - before - 0.75) > EXACT:
                problems.append("%s: lengthening %s by 0.75 changed the dimension by %.12g" % (dimension_id, ends[0][1]["wall"], after - before))
    for tag_id, tag in snapshot.get("tags", {}).items():
        if tag["element"] in snapshot["columns"]:
            moved = copy.deepcopy(snapshot)
            moved["columns"][tag["element"]]["position"]["x"] += 1.0
            if table(moved)[tag["storey"]]["tags"].get(tag_id) != table(snapshot)[tag["storey"]]["tags"].get(tag_id):
                problems.append("%s: moving the column changed what its tag prints" % tag_id)
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


def annotations_handler(ctx):
    """📏️ Oracle answer for `🪧️annotation-layout`, after GEOS and the parametric laws agreed with it."""
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

    return Adapter("python").oracle("annotations-room", annotations_handler)


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
        target = case / "💡️inference" / "📏️annotations" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "annotations")]
        print("%s: shapely %s, %d dimensions, %d tags" % (case.name, shapely.__version__, len(snapshot.get("dimensions", {})), len(snapshot.get("tags", {}))))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
