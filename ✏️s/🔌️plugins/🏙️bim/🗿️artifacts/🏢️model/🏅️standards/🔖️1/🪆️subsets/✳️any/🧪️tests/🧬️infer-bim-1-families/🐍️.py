#!/usr/bin/env python3
"""🧬️ Third-party ORACLE for the `s.bim.model@1` inference `🧬️families`.

The subject (Rust, `semio-s-artifact-bim-model`) infers, per parametric family, the value of every parameter from its formula, the issues of every formula and solid, and the solids as
meshes (volume, area, triangle count, bounds) plus the outline of a profile family. None of that is stored: the snapshot holds the formulas as text only. This file re-derives the same table from
the SAME committed snapshots without sharing a line of code with the subject:

* the formulas are read by a Python recursive-descent translator of the expression language and evaluated by the independent Python interpreter of the expression framework module
  (`🧰️framework/🔨️modules/🧮️expression/🧪️tests/📜️conformance/🐍️.py`: dimension checking on `ast`, `networkx` strongly connected components and topological generations, `Decimal` rounding);
* the solids are rebuilt with `numpy` (explicit facets, divergence-theorem volume, cross-product area) and `shapely` 2 (GEOS area and perimeter of the section polygons), on the documented
  tessellation contract of the framework geometry: arcs are flattened to a sagitta of 0.1 mm, a revolution turns in steps of the same sagitta;
* a closed-form audit checks the tessellated measures against the analytic ones (a prism is area x height, a cylinder is pi r^2 h, a sweep along a path is the section area x the path length) and the
  parametric laws: the order of the parameters never changes a value, and widening the table by delta widens its top by delta x depth x thickness.

The committed expectation under `🧫️fixtures/💡️inferences/🧬️families/<case>/💡️inference/📏️families/🔣️.json` is WRITTEN by this file (`write`), never by hand, and the Rust subject is compared
against it.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧬️families>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧬️families>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import re
import sys
from pathlib import Path

import numpy as np
import shapely
from shapely.geometry import Polygon

# endregion 🔖️Imports


# region 🔖️Vocabulary
CHORD = 1e-4
EPS = 1e-9
REPO = Path(__file__).resolve().parents[11]
KIND_OF = {"Length": "length", "Angle": "angle", "Real": "number", "Integer": "number", "Boolean": "bool", "Text": "text", "Material": "text"}
TAG_OF = {"number": "Number", "length": "Length", "angle": "Angle", "bool": "Boolean", "text": "Text"}
UNITS = {"mm", "cm", "m", "km", "in", "ft", "deg", "rad", "m2", "cm2", "mm2", "l", "m3", "cm3", "mm3"}
SPELLING = {"°": "deg", "m²": "m2", "cm²": "cm2", "mm²": "mm2", "m³": "m3", "cm³": "cm3", "mm³": "mm3"}
FUNCTIONS = {"sqrt", "abs", "round", "floor", "ceil", "sin", "cos", "tan", "atan2"}
KEYWORDS = {"if", "then", "else", "and", "or", "not", "true", "false"}


def child(parent, suffix):
    """🧭️ The entry of `parent` whose name ends with `suffix` (the owner tree names its folders with emoji)."""
    return next(path for path in sorted(parent.iterdir()) if path.name.endswith(suffix))


def expression_oracle():
    """🧮️ The independent Python interpreter of the expression framework module, loaded by path."""
    if "expression_oracle" not in sys.modules:
        module = child(child(child(child(child(REPO, "framework"), "modules"), "expression"), "tests"), "conformance")
        spec = importlib.util.spec_from_file_location("expression_oracle", child(module, ".py"))
        loaded = importlib.util.module_from_spec(spec)
        sys.modules["expression_oracle"] = loaded
        spec.loader.exec_module(loaded)
    return sys.modules["expression_oracle"]


# endregion 🔖️Vocabulary


# region 🔖️Translator
class Syntax(Exception):
    """🔤️ The text is no formula of the expression language."""


TOKEN = re.compile(r'\s*(?:(?P<number>\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)|(?P<name>[^\W\d]\w*)|`(?P<quoted>[^`]+)`|"(?P<text>(?:[^"\\]|\\.)*)"|(?P<op><=|>=|!=|==|[-+*/^<>=(),×÷−≤≥≠°])|(?P<unit>m²|cm²|mm²|m³|cm³|mm³))')


def tokens(text):
    position, out = 0, []
    while position < len(text):
        if not text[position:].strip():
            break
        match = TOKEN.match(text, position)
        if not match:
            raise Syntax(text)
        position = match.end()
        for kind in ("number", "name", "quoted", "text", "op", "unit"):
            if match.group(kind) is not None:
                out.append((kind, match.group(kind)))
                break
    return out


class Translator:
    """🔁️ Pratt parser from the text of the expression language to the Python rendering the framework interpreter reads (fully parenthesised)."""

    def __init__(self, text):
        self.items = tokens(text)
        self.at = 0

    def peek(self, value=None):
        if self.at >= len(self.items):
            return None
        item = self.items[self.at]
        return item if value is None or item[1] == value else None

    def take(self):
        item = self.items[self.at]
        self.at += 1
        return item

    def expect(self, value):
        if not self.peek(value):
            raise Syntax(value)
        self.take()

    def parse(self):
        result = self.expression()
        if self.at != len(self.items):
            raise Syntax("trailing")
        return result

    def expression(self):
        if self.peek("if"):
            self.take()
            condition = self.expression()
            self.expect("then")
            then = self.expression()
            self.expect("else")
            return "(%s if %s else %s)" % (then, condition, self.expression())
        return self.disjunction()

    def disjunction(self):
        left = self.conjunction()
        while self.peek("or"):
            self.take()
            left = "(%s or %s)" % (left, self.conjunction())
        return left

    def conjunction(self):
        left = self.negation()
        while self.peek("and"):
            self.take()
            left = "(%s and %s)" % (left, self.negation())
        return left

    def negation(self):
        if self.peek("not"):
            self.take()
            return "(not %s)" % self.negation()
        return self.comparison()

    def comparison(self):
        left = self.additive()
        item = self.peek()
        if item and item[0] == "op" and item[1] in ("=", "==", "!=", "≠", "<", "<=", "≤", ">", ">=", "≥"):
            self.take()
            symbol = {"=": "==", "≠": "!=", "≤": "<=", "≥": ">="}.get(item[1], item[1])
            left = "(%s %s %s)" % (left, symbol, self.additive())
            if (self.peek() or ("", ""))[1] in ("=", "==", "!=", "≠", "<", "<=", "≤", ">", ">=", "≥"):
                raise Syntax("chained")
        return left

    def additive(self):
        left = self.multiplicative()
        while (self.peek() or ("", ""))[1] in ("+", "-", "−"):
            symbol = "+" if self.take()[1] == "+" else "-"
            left = "(%s %s %s)" % (left, symbol, self.multiplicative())
        return left

    def multiplicative(self):
        left = self.unary()
        while (self.peek() or ("", ""))[1] in ("*", "/", "×", "÷"):
            symbol = "*" if self.take()[1] in ("*", "×") else "/"
            left = "(%s %s %s)" % (left, symbol, self.unary())
        return left

    def unary(self):
        if (self.peek() or ("", ""))[1] in ("-", "−"):
            self.take()
            return "(-%s)" % self.unary()
        return self.power()

    def power(self):
        base = self.primary()
        if self.peek("^"):
            self.take()
            return "(%s ** %s)" % (base, self.unary())
        return base

    def primary(self):
        if not self.peek():
            raise Syntax("end")
        kind, value = self.take()
        if kind == "number":
            unit = self.peek()
            if unit and (unit[1] in UNITS or unit[1] in SPELLING) and unit[0] in ("name", "unit", "op"):
                self.take()
                return 'Q(%r, "%s")' % (float(value), SPELLING.get(unit[1], unit[1]))
            return repr(float(value))
        if kind == "text":
            return json.dumps(json.loads('"%s"' % value), ensure_ascii=False)
        if kind == "quoted":
            return 'P(%s)' % json.dumps(value, ensure_ascii=False)
        if kind == "op" and value == "(":
            inner = self.expression()
            self.expect(")")
            return inner
        if kind == "name":
            if value == "true":
                return "True"
            if value == "false":
                return "False"
            if value in KEYWORDS:
                raise Syntax(value)
            if self.peek("("):
                if value not in FUNCTIONS and value not in ("min", "max"):
                    raise Syntax(value)
                self.take()
                args = [self.expression()]
                while self.peek(","):
                    self.take()
                    args.append(self.expression())
                self.expect(")")
                return "%s(%s)" % (value, ", ".join(args))
            return value
        raise Syntax(value)


def rendering(text):
    return Translator(text).parse()


# endregion 🔖️Translator


# region 🔖️Evaluation
def classify(code, names, declared):
    """🏷️ The family issue code of an error code of the framework interpreter."""
    if code == "unknown-parameter":
        return "dependency" if any(name in declared for name in names[:1]) else "unknown"
    if code in ("division-by-zero",):
        return "division-by-zero"
    if code in ("domain", "overflow"):
        return "domain"
    if code in ("cycle",):
        return "cycle"
    if code == "failed-dependency":
        return "dependency"
    return "kind"


def unknown_names(source, rows):
    """🔎️ The names a rendering refers to that no row defines, in the order of the text."""
    oracle = expression_oracle()
    used = oracle.dependencies(source)
    seen = []
    for name in re.findall(r'P\("([^"]+)"\)|\b([A-Za-z_]\w*)\b', source):
        name = name[0] or name[1]
        if name in used and name not in rows and name not in seen:
            seen.append(name)
    return seen


def parameters(snapshot, family_id):
    rows = {row["name"]: row for row in snapshot.get("family_parameters", {}).values() if row["family"] == family_id}
    return dict(sorted(rows.items()))


def resolve_parameters(snapshot, family_id):
    """🧮️ Every parameter of a family: its stored form and the issues, from the framework interpreter."""
    oracle = expression_oracle()
    rows = parameters(snapshot, family_id)
    names = set(rows)
    issues, sources = [], {}
    for name, row in rows.items():
        try:
            sources[name] = rendering(row["value"])
        except (Syntax, IndexError, ValueError):
            issues.append("syntax|parameter|%s|value" % name)
    declared = {name: KIND_OF[row["kind"]] for name, row in rows.items() if name in sources}
    resolved = oracle.resolve(sources, {}, declared)
    for name, error in resolved["errors"].items():
        if error["code"] == "unknown-parameter":
            missing = unknown_names(sources[name], resolved["values"])
            issues.append("%s|parameter|%s|value" % (classify(error["code"], missing, names), name))
        else:
            issues.append("%s|parameter|%s|value" % (classify(error["code"], [], names), name))
    stored = {}
    for name, row in rows.items():
        item = {"kind": row["kind"], "formula": row["value"]}
        value = resolved["values"].get(name)
        if value is not None and value["kind"] in TAG_OF:
            item["value"] = {TAG_OF[value["kind"]]: {"value": value["value"]}}
            if row["kind"] == "Integer" and abs(value["value"] - round(value["value"])) > EPS:
                issues.append("kind|parameter|%s|value" % name)
            if row["kind"] == "Material" and value["value"] not in snapshot.get("materials", {}):
                issues.append("unknown|parameter|%s|value" % name)
        stored[name] = item
    return stored, resolved["values"], names, issues


class Slots:
    """🧾️ The evaluation of the formula slots of one solid against the resolved parameters (same order of checks as the subject)."""

    def __init__(self, solid_id, env, names, issues):
        self.solid_id, self.env, self.names, self.issues = solid_id, env, names, issues
        self.oracle = expression_oracle()

    def note(self, code, field):
        self.issues.append("%s|solid|%s|%s" % (code, self.solid_id, field))

    def value(self, field, text, want):
        try:
            source = rendering(text)
        except (Syntax, IndexError, ValueError):
            self.note("syntax", field)
            return None
        result = self.oracle.run(source, self.env)
        if "error" in result:
            code = result["error"]["code"]
            names = unknown_names(source, self.env) if code == "unknown-parameter" else []
            self.note(classify(code, names, self.names), field)
            return None
        if result["kind"] != want:
            self.note("kind", field)
            return None
        return result["value"]

    def positive(self, field, text):
        value = self.value(field, text, "length")
        if value is None:
            return None
        if value > EPS:
            return value
        self.note("negative", field)
        return None


def section(slots, field, profile):
    """▭️ The counter-clockwise vertices of a profile in (u, v), or `None` after noting its issues."""
    tag, body = next(iter(profile.items()))
    if tag == "Rectangle":
        width, depth = slots.positive(field + ".width", body["width"]), slots.positive(field + ".depth", body["depth"])
        if width is None or depth is None:
            return None
        w, d = width / 2, depth / 2
        return [(-w, -d), (w, -d), (w, d), (-w, d)]
    if tag == "Circle":
        diameter = slots.positive(field + ".diameter", body["diameter"])
        return None if diameter is None else circle(diameter / 2)
    if tag == "IShape":
        measures = [slots.positive(field + "." + name, body[name]) for name in ("width", "depth", "web", "flange")]
        if any(measure is None for measure in measures):
            return None
        w, d, t, f = measures
        if t >= w or 2 * f >= d:
            slots.note("outline", field)
            return None
        x, y, h = w / 2, d / 2, t / 2
        return [(-x, -y), (x, -y), (x, -y + f), (h, -y + f), (h, y - f), (x, y - f), (x, y), (-x, y), (-x, y - f), (-h, y - f), (-h, -y + f), (-x, -y + f)]
    points = []
    for index, point in enumerate(body["points"]):
        x, y = slots.value("%s.points[%d].x" % (field, index), point["x"], "length"), slots.value("%s.points[%d].y" % (field, index), point["y"], "length")
        points.append(None if x is None or y is None else (x, y))
    if any(point is None for point in points):
        return None
    if len(points) < 3:
        slots.note("outline", field)
        return None
    polygon = Polygon(points)
    if polygon.area < 1e-12 or not polygon.is_valid:
        slots.note("outline", field)
        return None
    return orient(points)


def orient(points):
    area = sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points)))
    return points if area > 0 else points[::-1]


class Ring(list):
    """⭕️ A flattened round section: the subject keeps it as two half circles."""

    curved = True


def circle(radius):
    """⭕️ The two half circles of the profile flattened to a sagitta of CHORD: the vertices of the flattened loop, counter-clockwise from (r, 0)."""
    step = 2 * math.acos(1 - min(max(CHORD / radius, 1e-12), 1.0))
    n = min(max(math.ceil(math.pi / max(step, 1e-6)), 1), 1 << 16)
    points = []
    for half in (0, 1):
        for i in range(n):
            angle = math.pi * (half + i / n)
            points.append((radius * math.cos(angle), radius * math.sin(angle)))
    points[0] = (radius, 0.0)
    points[n] = (-radius, 0.0)
    return Ring(points)


def mesh_measures(vertices, triangles):
    """📐️ Volume (divergence theorem), area, triangle count and bounds of explicit facets."""
    triangles = [t for t in triangles if np.linalg.norm(np.cross(t[1] - t[0], t[2] - t[0])) > 1e-18]
    cube = np.array(triangles) if triangles else np.zeros((0, 3, 3))
    volume = abs(sum(np.dot(t[0], np.cross(t[1], t[2])) for t in triangles) / 6.0)
    area = sum(0.5 * np.linalg.norm(np.cross(t[1] - t[0], t[2] - t[0])) for t in triangles)
    flat = cube.reshape(-1, 3) if len(triangles) else np.zeros((1, 3))
    return volume, area, len(triangles), flat.min(axis=0), flat.max(axis=0)


def prism(ring, base, height):
    """🧱️ Volume, area, triangles and corners of the prism over a ring: the caps are fans, the sides quads."""
    polygon = Polygon(ring)
    n = len(ring)
    corners = np.array([(x, y, z) for z in (base, base + height) for x, y in ring])
    return polygon.area * height, 2 * polygon.area + polygon.length * height, 4 * n - 4, corners.min(axis=0), corners.max(axis=0)


def sweep(ring, path, base):
    """➰️ A section swept along straight plan segments with mitered corners: volume = section area x path length (a centred section)."""
    points = [np.array(p, dtype=float) for p in path]
    distinct = [points[0]]
    for point in points[1:]:
        if np.linalg.norm(point - distinct[-1]) > EPS:
            distinct.append(point)
    if len(distinct) < 2:
        return None
    segments = len(distinct) - 1
    directions = [(b - a) / np.linalg.norm(b - a) for a, b in zip(distinct, distinct[1:])]
    normals = [np.array([-d[1], d[0]]) for d in directions]
    lateral, scale = [normals[0]], [1.0]
    for i in range(1, segments):
        bisector = normals[i - 1] + normals[i]
        bisector = bisector / np.linalg.norm(bisector)
        lateral.append(bisector)
        scale.append(1.0 / float(np.dot(bisector, normals[i])))
    lateral.append(normals[-1])
    scale.append(1.0)
    polygon = Polygon(ring)
    length = sum(np.linalg.norm(b - a) for a, b in zip(distinct, distinct[1:]))
    corners = np.array([(*(distinct[i] + lateral[i] * (u * scale[i])), base + v) for i in range(segments + 1) for u, v in ring])
    n = len(ring)
    perimeter_area = polygon.length
    mitered = 0.0
    for a in range(segments):
        for k in range(n):
            u0, v0 = ring[k]
            u1, v1 = ring[(k + 1) % n]
            p = [np.append(distinct[a + s] + lateral[a + s] * (u * scale[a + s]), base + v) for s in (0, 1) for u, v in ((u0, v0), (u1, v1))]
            quad = [p[0], p[1], p[3], p[2]]
            mitered += 0.5 * np.linalg.norm(np.cross(quad[1] - quad[0], quad[2] - quad[0])) + 0.5 * np.linalg.norm(np.cross(quad[2] - quad[0], quad[3] - quad[0]))
    area = 2 * polygon.area + mitered
    return polygon.area * length, area, 2 * (n - 2) + 2 * n * segments, corners.min(axis=0), corners.max(axis=0), perimeter_area


def revolution(ring, turn):
    """🌀️ A profile in (radius, height) turned about the Z axis: explicit ring facets like the framework builder (full turns have no caps)."""
    ring = orient(ring)
    clean = [ring[0]]
    for point in ring[1:]:
        if math.dist(point, clean[-1]) > 1e-12:
            clean.append(point)
    while len(clean) > 1 and math.dist(clean[0], clean[-1]) <= 1e-12:
        clean.pop()
    turn = min(turn, 2 * math.pi)
    full = turn >= 2 * math.pi - 1e-12
    reach = max(x for x, _ in clean)
    step = 2 * math.acos(1 - CHORD / reach) if reach > CHORD else turn
    steps = min(max(math.ceil(turn / max(step, 1e-6)), 8 if full else 1), 4096)
    place = lambda p, t: np.array([max(p[0], 0.0) * math.cos(t), max(p[0], 0.0) * math.sin(t), p[1]])
    facets = []
    for i in range(steps):
        t0, t1 = turn * i / steps, turn * (i + 1) / steps
        for j in range(len(clean)):
            a, b = clean[j], clean[(j + 1) % len(clean)]
            a0, a1, b1, b0 = place(a, t0), place(a, t1), place(b, t1), place(b, t0)
            facets += [(a0, a1, b1), (a0, b1, b0)]
    if not full:
        poly = Polygon(clean)
        from shapely.ops import triangulate

        for tri in triangulate(poly):
            if poly.contains(tri.representative_point()):
                a, b, c = [tuple(p) for p in tri.exterior.coords[:3]]
                facets += [(place(a, 0), place(b, 0), place(c, 0)), (place(a, turn), place(c, turn), place(b, turn))]
    return mesh_measures(None, [np.array(f) for f in facets])


def solid_row(snapshot, solid_id, solid, env, names, issues):
    """🧊️ The metrics of one solid and, for an extrusion, its section; issues are appended."""
    slots = Slots(solid_id, env, names, issues)
    visible = slots.value("visible", solid["visible"], "bool")
    visible = True if visible is None else visible
    material = slots.value("material", solid["material"], "text")
    if material is not None and material != "" and material not in snapshot.get("materials", {}):
        slots.note("unknown", "material")
    material = "" if material is None else material
    offset = [slots.value("offset." + axis, solid["offset"][axis], "length") for axis in "xyz"]
    tag, body = next(iter(solid["shape"].items()))
    measures, loop = None, None
    if tag == "Extrusion":
        ring = section(slots, "profile", body["profile"])
        base, height = slots.value("base", body["base"], "length"), slots.positive("height", body["height"])
        if ring is not None and base is not None and height is not None:
            measures, loop = prism(ring, base, height), ring
    elif tag == "Cuboid":
        corner = [slots.value(axis, body[axis], "length") for axis in "xyz"]
        size = [slots.positive(name, body[name]) for name in ("width", "depth", "height")]
        if None not in corner and None not in size:
            ring = [(corner[0], corner[1]), (corner[0] + size[0], corner[1]), (corner[0] + size[0], corner[1] + size[1]), (corner[0], corner[1] + size[1])]
            measures = prism(ring, corner[2], size[2])
    elif tag == "Sweep":
        ring = section(slots, "profile", body["profile"])
        points = []
        for index, point in enumerate(body["path"]):
            x, y = slots.value("path[%d].x" % index, point["x"], "length"), slots.value("path[%d].y" % index, point["y"], "length")
            points.append(None if x is None or y is None else (x, y))
        if ring is not None and None not in points:
            result = sweep(ring, points, 0.0)
            if result is None:
                slots.note("outline", "path")
            else:
                measures = result[:5]
    else:
        ring = section(slots, "profile", body["profile"])
        turn = slots.value("angle", body["angle"], "angle")
        if ring is not None and turn is not None:
            if turn <= EPS or turn > 2 * math.pi + EPS:
                slots.note("negative", "angle")
            elif any(x < -EPS for x, _ in ring):
                slots.note("outline", "profile")
            else:
                measures = revolution(ring, turn)
                if body["axis"] != "Z":
                    permute = {"X": [2, 0, 1], "Y": [1, 2, 0]}[body["axis"]]
                    lo, hi = measures[3], measures[4]
                    corners = np.array([[x, y, z] for x in (lo[0], hi[0]) for y in (lo[1], hi[1]) for z in (lo[2], hi[2])])
                    mapped = np.zeros_like(corners)
                    for source, target in enumerate(permute):
                        mapped[:, target] = corners[:, source]
                    measures = (*measures[:3], mapped.min(axis=0), mapped.max(axis=0))
    if measures is None or None in offset:
        row = {"visible": visible, "material": material, "volume": 0.0, "area": 0.0, "triangles": 0, "min": [0.0, 0.0, 0.0], "max": [0.0, 0.0, 0.0]}
    else:
        volume, area, triangles, lo, hi = measures
        row = {"visible": visible, "material": material, "volume": float(volume), "area": float(area), "triangles": int(triangles), "min": [float(v + o) for v, o in zip(lo, offset)], "max": [float(v + o) for v, o in zip(hi, offset)]}
    return row, (loop, offset) if loop is not None and None not in offset else None


# endregion 🔖️Evaluation


# region 🔖️Table
def family_row(snapshot, family_id):
    family = snapshot["families"][family_id]
    stored, env, names, issues = resolve_parameters(snapshot, family_id)
    solids, sections = {}, []
    for solid_id, solid in sorted(snapshot.get("family_solids", {}).items()):
        if solid["family"] != family_id:
            continue
        row, built = solid_row(snapshot, solid_id, solid, env, names, issues)
        solids[solid_id] = row
        if row["visible"] and built is not None:
            sections.append(built)
    outline_vertices, outline_area = 0, 0.0
    if family["category"] == "Profile":
        if sections:
            loop, offset = sections[0]
            curved = getattr(loop, "curved", False)
            outline_vertices = 2 if curved else len(loop)
            outline_area = math.pi * (loop[0][0] ** 2) if curved else Polygon(loop).area
        else:
            issues.append("outline|family||")
    return {"category": family["category"], "parameters": stored, "issues": sorted(issues), "solids": solids, "outline_area": outline_area, "outline_vertices": outline_vertices}


def table(snapshot):
    return {family_id: family_row(snapshot, family_id) for family_id in sorted(snapshot.get("families", {}))}


# endregion 🔖️Table


# region 🔖️Audit
def close(a, b, tolerance):
    return abs(a - b) <= tolerance * max(1.0, abs(a), abs(b))


def audit(snapshot):
    """🩺️ Closed forms and parametric laws that must hold for the table the oracle computes."""
    problems = []
    computed = table(snapshot)
    for family_id, family in computed.items():
        for solid_id, solid in snapshot.get("family_solids", {}).items():
            if solid["family"] != family_id:
                continue
            row = family["solids"][solid_id]
            tag, body = next(iter(solid["shape"].items()))
            if tag == "Extrusion" and next(iter(body["profile"])) == "Circle" and row["volume"] > 0:
                diameter = family["parameters"]["diameter"]["value"]["Length"]["value"]
                if not close(row["volume"], math.pi * (diameter / 2) ** 2 * 1.0, 5e-3):
                    problems.append("%s: the tessellated pipe differs from pi r^2 h by more than the chord error" % solid_id)
            if tag == "Revolution" and row["volume"] > 0 and not close(row["volume"], analytic_foot(snapshot, solid), 5e-3):
                problems.append("%s: the tessellated revolution differs from Pappus' theorem by more than the chord error" % solid_id)
    for family_id in snapshot.get("families", {}):
        shuffled = copy.deepcopy(snapshot)
        keys = list(shuffled["family_parameters"])
        shuffled["family_parameters"] = {key: shuffled["family_parameters"][key] for key in reversed(keys)}
        if family_row(shuffled, family_id) != computed[family_id]:
            problems.append("%s: the order of the parameters changed the table" % family_id)
    top = snapshot.get("family_solids", {}).get("s-top")
    if top and all("fam-table." + name in snapshot.get("family_parameters", {}) for name in ("width", "depth", "top_thickness")):
        wider = copy.deepcopy(snapshot)
        wider["family_parameters"]["fam-table.width"]["value"] = "1.7 m"
        before, after = computed["fam-table"]["solids"]["s-top"]["volume"], family_row(wider, "fam-table")["solids"]["s-top"]["volume"]
        depth = computed["fam-table"]["parameters"]["depth"]["value"]["Length"]["value"]
        thick = computed["fam-table"]["parameters"]["top_thickness"]["value"]["Length"]["value"]
        if not close(after - before, 0.1 * depth * thick, 1e-9):
            problems.append("fam-table: widening by 0.1 m changed the top by %.12g, expected %.12g" % (after - before, 0.1 * depth * thick))
    return problems


def analytic_foot(snapshot, solid):
    """🌀️ Pappus: the volume of a profile turned once about the axis is 2 pi x centroid radius x area; the foot is a polygon of resolved lengths."""
    ref = expression_oracle()
    env = {}
    family_id = solid["family"]
    _, values, _, _ = resolve_parameters(snapshot, family_id)
    points = []
    for point in solid["shape"]["Revolution"]["profile"]["Polygon"]["points"]:
        x = ref.run(rendering(point["x"]), values)["value"]
        y = ref.run(rendering(point["y"]), values)["value"]
        points.append((x, y))
    polygon = Polygon(points)
    return 2 * math.pi * polygon.centroid.x * polygon.area


# endregion 🔖️Table


# region 🔖️Handlers
def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def families_handler(ctx):
    """🧬️ Oracle answer for `🧬️families`, after the closed forms and the parametric laws agreed with it."""
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

    return Adapter("python").oracle("families-table", families_handler).oracle("families-frame", families_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
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
        return [] if close(float(expected), float(actual), 1e-9) else ["%s: %r, expected %r" % (path, actual, expected)]
    return [] if expected == actual else ["%s: %r, expected %r" % (path, actual, expected)]


def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in audit(snapshot)]
        computed = table(snapshot)
        target = case / "💡️inference" / "📏️families" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in differences(json.loads(target.read_text(encoding="utf-8")), computed, "families")]
        print("%s: shapely %s, numpy %s, %d families, %d solids" % (case.name, shapely.__version__, np.__version__, len(computed), sum(len(f["solids"]) for f in computed.values())))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
