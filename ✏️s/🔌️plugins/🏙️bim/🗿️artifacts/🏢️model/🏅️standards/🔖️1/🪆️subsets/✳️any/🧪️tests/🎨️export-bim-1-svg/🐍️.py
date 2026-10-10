#!/usr/bin/env python3
"""🎨️ Third-party ORACLE (lxml + shapely) for the SVG 1.1 views of the BIM house: its plans, sections and elevations.

lxml (libxml2) and shapely (GEOS) have never seen this repository's writer. They open the committed file
`🧫️fixtures/🚪️svg/🏠️house/🏠️house.svg` and `🧫️fixtures/🚪️svg/🪧️notated/🪧️notated.svg`, and:

* parse it as namespaced XML, require the SVG root with version 1.1 and a millimetre size equal to its viewBox;
* find one `g.view` per drawn view (a plan, ceiling plan, section or elevation of the model, never a camera; unique id `view-<id>`, the model view in `data-view`, its kind, scale and storey) with a title and its three layers;
* count the regions, lines and texts of every view (the texts of the text layer and of the annotation layer), the paths per line-style class (cut, projection, hidden, annotation) and the arc commands;
* measure the annotation layer of a notated plan: per kind (dimension, extension and leader lines and marks, the texts of dimensions, tags, notes and leaders) the number of paths or texts
  and the straight length of the paths, and the sorted texts the layer prints;
* read every path's `d` with an own SVG path reader (M, L, A, Z) and measure with GEOS: the area of the straight cut poche regions
  (even-odd, so the holes subtract) and the length of the straight lines per style, in metres at the scale of the view;
* sample every arc (SVG endpoint-to-centre conversion, 2048 chords per full circle) and measure the poche area of the regions that contain arcs;
  that audit value is committed beside the table, never compared at 1e-9.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🚪️svg>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🚪️svg>     # rewrite the measured table from the committed file

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🎨️svg/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import math
import re
import sys
from functools import reduce
from pathlib import Path

from lxml import etree
from shapely.geometry import LineString, LinearRing, Polygon

# endregion 🔖️Imports


# region 🔖️Reading
NS = {"svg": "http://www.w3.org/2000/svg"}
STYLES = ["cut", "projection", "hidden", "annotation"]
"""🖊️ The four line-style classes."""

KINDS = {"Plan": "plan", "CeilingPlan": "ceiling-plan", "Section": "section", "Elevation": "elevation"}
"""🖼️ The kinds of view the sheet draws, by the class of their group; a camera is rendered by the 3D window."""
VIEW = "svg:g[contains(concat(' ', normalize-space(@class), ' '), ' view ')]"
NOTATION = ["dimension-line", "dimension-extension", "dimension-mark", "dimension-text", "tag-text", "note-text", "leader-line", "leader-mark", "leader-text"]
"""🪧️ The kinds of the annotation layer."""
LAYERS = ["layer regions", "layer lines", "layer texts"]
ELEMENTS = ["component-outline", "component-front", "component-connector", "mep-axis", "mep-band", "mep-drop"]
"""🪑️ The kinds of the components and routed MEP elements: the outline of a component with its front tick and connector cross, the centre line, band and drops of a run."""
CASES = {"🏠️house": "🏠️house.svg", "🪧️notated": "🪧️notated.svg", "🪑️components": "🪑️components.svg"}
SNAPSHOTS = {"🏠️house": ("🏗️ifc", "🏠️house", "📸️snapshot"), "🪧️notated": ("💡️inferences", "🪧️annotation-layout", "🏠️room", "📸️snapshot"), "🪑️components": ("🏗️ifc", "🪑️components", "📸️snapshot")}
CHORDS = 2048
"""🌀️ Chords per full circle when an arc is sampled."""

TOKEN = re.compile(r"[MLAZ]|-?\d+(?:\.\d+)?")


def mm_per_metre(group):
    """📐️ Paper millimetres per model metre at the scale `1:n` of a view group."""
    return 1000.0 / int(group.get("data-scale"))


def classes(element):
    """🏷️ The class tokens of an element."""
    return (element.get("class") or "").split()


def subpaths(d):
    """✒️ The subpaths of a path: each `(points, closed, arcs)` where `arcs` are the `(rx, ry, large, sweep, from, to)` of its arc segments."""
    tokens = TOKEN.findall(d)
    if "".join(tokens) != re.sub(r"\s+", "", d):
        raise ValueError("unsupported path data: %s" % d)
    result, at, current = [], 0, None
    while at < len(tokens):
        command = tokens[at]
        at += 1
        if command == "M":
            current = {"points": [(float(tokens[at]), float(tokens[at + 1]))], "closed": False, "arcs": []}
            result.append(current)
            at += 2
        elif command == "L":
            current["points"].append((float(tokens[at]), float(tokens[at + 1])))
            at += 2
        elif command == "A":
            rx, ry, rotation, large, sweep, x, y = (float(token) for token in tokens[at:at + 7])
            if rx != ry or rotation != 0:
                raise ValueError("only circular arcs are written: %s" % d)
            current["arcs"].append((rx, large == 1, sweep == 1, current["points"][-1], (x, y)))
            current["points"].append((x, y))
            at += 7
        elif command == "Z":
            current["closed"] = True
        else:
            raise ValueError("unexpected token %s in %s" % (command, d))
    return result


def arc_points(radius, large, sweep, start, end):
    """🌀️ Points along an SVG circular arc from `start` to `end` (the spec's endpoint-to-centre conversion F.6.5), `start` excluded, `end` included."""
    dx, dy = (start[0] - end[0]) / 2.0, (start[1] - end[1]) / 2.0
    distance = dx * dx + dy * dy
    if distance == 0.0:
        return []
    radius = max(radius, math.sqrt(distance))
    sign = -1.0 if large == sweep else 1.0
    coefficient = sign * math.sqrt(max(0.0, (radius * radius - distance) / distance))
    centre = (coefficient * dy + (start[0] + end[0]) / 2.0, -coefficient * dx + (start[1] + end[1]) / 2.0)
    first = math.atan2(start[1] - centre[1], start[0] - centre[0])
    last = math.atan2(end[1] - centre[1], end[0] - centre[0])
    delta = last - first
    if not sweep and delta > 0:
        delta -= math.tau
    if sweep and delta < 0:
        delta += math.tau
    count = max(2, math.ceil(abs(delta) / math.tau * CHORDS))
    return [(centre[0] + radius * math.cos(first + delta * step / count), centre[1] + radius * math.sin(first + delta * step / count)) for step in range(1, count + 1)]


def ring_points(subpath, sampled):
    """🔢️ The points of a subpath; arcs are replaced by their sampled chords when `sampled`."""
    if not sampled or not subpath["arcs"]:
        return subpath["points"]
    points, arcs = [subpath["points"][0]], iter(subpath["arcs"])
    pending = next(arcs, None)
    for index in range(1, len(subpath["points"])):
        if pending is not None and pending[3] == subpath["points"][index - 1] and pending[4] == subpath["points"][index]:
            points.extend(arc_points(*pending))
            pending = next(arcs, None)
        else:
            points.append(subpath["points"][index])
    return points


def region_area(d, sampled):
    """📐️ The even-odd area of a region path in square paper millimetres."""
    polygons = [Polygon(ring_points(subpath, sampled)) for subpath in subpaths(d)]
    return reduce(lambda a, b: a.symmetric_difference(b), polygons).area


def line_length(d):
    """📏️ The length of a straight line path in paper millimetres (a closed one counts its closing segment)."""
    total = 0.0
    for subpath in subpaths(d):
        total += (LinearRing(subpath["points"]) if subpath["closed"] else LineString(subpath["points"])).length
    return total


# endregion 🔖️Reading


# region 🔖️Measurement
def kind_of(element):
    """🪧️ The annotation kind class of a path or text, or `None` for any other primitive."""
    return next((token for token in classes(element) if token in NOTATION), None)


def element_rows(paths, scale):
    """🪑️ The components and routed elements of a view by model id: the length of a centre line and the area of an outline or band in model units, the paths per kind and the stroke colours the writer gave them (the colour of the service)."""
    rows = {}
    for path in paths:
        names = classes(path)
        kind = next((token for token in ELEMENTS if token in names), None)
        if kind is None:
            continue
        row = rows.setdefault(path.get("data-id"), {"axis": 0.0, "area": 0.0, "count": {}, "stroke": []})
        row["count"][kind] = row["count"].get(kind, 0) + 1
        d = path.get("d")
        if kind == "mep-axis" and "A" not in d:
            row["axis"] += line_length(d) / scale
        if kind in ("mep-band", "component-outline") and "A" not in d:
            row["area"] += region_area(d, False) / (scale * scale)
        for colour in re.findall(r"(?:stroke|fill):(#[0-9a-fA-F]{6})", path.get("style") or ""):
            if colour not in row["stroke"]:
                row["stroke"].append(colour)
    return {identity: {**row, "count": dict(sorted(row["count"].items())), "stroke": sorted(row["stroke"])} for identity, row in sorted(rows.items())}


def measure(document):
    """🎨️ The oracle table of an SVG document plus the audit values that are not compared at 1e-9."""
    root = etree.fromstring(document)
    view = [float(part) for part in root.get("viewBox").split()]
    table = {"width": view[2], "height": view[3], "views": {}}
    sampled = {}
    for group in root.xpath(VIEW, namespaces=NS):
        scale = mm_per_metre(group)
        identity = group.get("data-view")
        paths = group.xpath(".//svg:path", namespaces=NS)
        regions = [path for path in paths if "region" in classes(path)]
        lines = [path for path in paths if "line" in classes(path)]
        texts = group.xpath("svg:g[@class='layer texts' or @class='layer annotations']/svg:text", namespaces=NS)
        row = {
            "name": group.get("data-name"),
            "kind": group.get("data-kind"),
            "scale": int(group.get("data-scale")),
            "regions": len(regions),
            "lines": len(lines),
            "texts": len(texts),
            "styles": {style: sum(style in classes(path) for path in paths) for style in STYLES},
            "arcs": sum(path.get("d").count("A") for path in paths),
            "pocheArea": 0.0,
            "lineLength": {style: 0.0 for style in STYLES},
            "notation": {},
            "printed": sorted("".join(text.itertext()) for text in texts if kind_of(text)),
        }
        for element in paths + texts:
            kind = kind_of(element)
            if kind:
                found = row["notation"].setdefault(kind, {"count": 0, "length": 0.0})
                found["count"] += 1
                if element.tag.endswith("path") and "A" not in element.get("d"):
                    found["length"] += line_length(element.get("d")) / scale
        row["notation"] = dict(sorted(row["notation"].items()))
        elements = element_rows(paths, scale)
        if elements:
            row["elements"] = elements
        for path in regions:
            if "cut" in classes(path) and "A" not in path.get("d"):
                row["pocheArea"] += region_area(path.get("d"), False) / (scale * scale)
        for path in lines:
            if "A" not in path.get("d"):
                style = next(style for style in STYLES if style in classes(path))
                row["lineLength"][style] += line_length(path.get("d")) / scale
        sampled[identity] = sum(region_area(path.get("d"), True) for path in regions if "cut" in classes(path)) / (scale * scale)
        table["views"][identity] = row
    return table, sampled


def audit(document, snapshot, table):
    """⚖️ Every place where the file leaves the structure the export promises."""
    problems = []
    root = etree.fromstring(document)
    if root.tag != "{%s}svg" % NS["svg"]:
        problems.append("the root is %s, not an SVG element" % root.tag)
    if root.get("version") != "1.1":
        problems.append("version is %s, not 1.1" % root.get("version"))
    view = root.get("viewBox").split()
    if view[:2] != ["0", "0"] or root.get("width") != "%smm" % view[2] or root.get("height") != "%smm" % view[3]:
        problems.append("the size %s x %s does not follow the viewBox %s in millimetres" % (root.get("width"), root.get("height"), root.get("viewBox")))
    if root.get("data-unit") != "mm" or "sheet" not in classes(root):
        problems.append("the sheet does not declare paper millimetres")
    groups = root.xpath(VIEW, namespaces=NS)
    drawn = {identity: view for identity, view in snapshot["views"].items() if view["kind"] in KINDS}
    if sorted(group.get("data-view") for group in groups) != sorted(drawn):
        problems.append("the view groups are %s, the model draws %s" % (sorted(group.get("data-view") for group in groups), sorted(drawn)))
    ids = [group.get("id") for group in groups]
    if len(set(ids)) != len(ids):
        problems.append("the view group ids are not unique")
    for group in groups:
        identity = group.get("data-view")
        view = drawn.get(identity)
        if view is None:
            continue
        if group.get("id") != "view-" + re.sub(r"[^A-Za-z0-9_.-]", "_", identity):
            problems.append("%s: the group id is %s" % (identity, group.get("id")))
        layers = [layer.get("class") for layer in group.xpath("svg:g", namespaces=NS)]
        notated = bool(table["views"][identity]["notation"])
        if len(group.xpath("svg:title", namespaces=NS)) != 1 or layers != LAYERS + (["layer annotations"] if notated else []):
            problems.append("%s: the group lacks its title or its layers (%s)" % (identity, layers))
        for element in group.xpath("svg:g[@class!='layer annotations']//*[self::svg:path or self::svg:text]", namespaces=NS):
            if kind_of(element):
                problems.append("%s: an annotation primitive sits outside the annotation layer" % identity)
        for element in group.xpath("svg:g[@class='layer annotations']//*[self::svg:path or self::svg:text]", namespaces=NS):
            if not kind_of(element) or "annotation" not in classes(element):
                problems.append("%s: a primitive of the annotation layer is not an annotation" % identity)
        for text in group.xpath("svg:g[@class='layer annotations']/svg:text", namespaces=NS):
            if len(text) or text.get("text-anchor") != "middle" or not text.get("font-size"):
                problems.append("%s: an annotation text is not one run, middle anchored, with its own size" % identity)
        if group.get("class").split() != ["view", KINDS[view["kind"]]] or group.get("data-kind") != KINDS[view["kind"]]:
            problems.append("%s: the kind differs from the model" % identity)
        if group.get("data-name") != view["name"] or group.get("data-building") != view["building"] or group.get("aria-label") != view["name"]:
            problems.append("%s: name or building differ from the model" % identity)
        if int(group.get("data-scale")) != view["scale"] or group.get("data-storey") != view.get("storey"):
            problems.append("%s: scale or storey differ from the model" % identity)
        for path in group.xpath(".//svg:path", namespaces=NS):
            if sum(style in classes(path) for style in STYLES) != 1:
                problems.append("%s: a path has not exactly one style class" % identity)
            subpaths(path.get("d"))
        for text in group.xpath(".//svg:text", namespaces=NS):
            if not (math.isfinite(float(text.get("x"))) and math.isfinite(float(text.get("y")))):
                problems.append("%s: a text has no position" % identity)
        counted = sum(table["views"][identity]["styles"].values())
        if counted != table["views"][identity]["regions"] + table["views"][identity]["lines"]:
            problems.append("%s: %d styled paths for %d regions and lines" % (identity, counted, table["views"][identity]["regions"] + table["views"][identity]["lines"]))
    return problems


def open_case(root, name):
    """📂️ The committed snapshot, the committed file and the directory of the committed table of a case."""
    case = Path(root) / name
    snapshot = json.loads(Path(root).parent.joinpath(*SNAPSHOTS[name], "🔣️.json").read_text(encoding="utf-8"))
    return case, snapshot, (case / CASES[name]).read_bytes()


# endregion 🔖️Measurement


# region 🔖️Handlers
def export_handler(ctx):
    """🎨️ Oracle answer: the table of the committed file."""
    from semio_repo_test import Outcome

    svg = next(uri for uri in ctx.step_input_uris() if uri.endswith(".svg"))
    table, _ = measure(ctx.input_bytes(svg))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-svg-house", export_handler).oracle("export-svg-notated", export_handler).oracle("export-svg-components", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def committed_form(table, sampled):
    """🧾️ The committed file: the compared table plus the audit values beside it."""
    return {**table, "audit": {"sampledPocheArea": {view: round(area, 9) for view, area in sampled.items()}}}


def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], arguments[1]
    problems = []
    for name in arguments[2:] or CASES:
        case, snapshot, document = open_case(root, name)
        table, sampled = measure(document)
        problems += audit(document, snapshot, table)
        path = case / "🔬️measure" / "🔣️.json"
        expected = committed_form(table, sampled)
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(expected, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote the table of %d views" % (name, len(table["views"])))
        else:
            committed = json.loads(path.read_text(encoding="utf-8"))
            if committed != json.loads(json.dumps(expected)):
                problems.append("%s: the committed table differs from the measurement of the committed file" % name)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (lxml %s, shapely %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", etree.LXML_VERSION, __import__("shapely").__version__))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
