#!/usr/bin/env python3
"""🎨️ Third-party ORACLE (lxml + shapely) for the SVG 1.1 floor plans of the BIM house.

lxml (libxml2) and shapely (GEOS) have never seen this repository's writer. They open the committed file
`🧫️fixtures/🚪️svg/🏠️house/🏠️house.svg`, and:

* parse it as namespaced XML, require the SVG root with version 1.1, a millimetre size equal to its viewBox and the 1:100 scale;
* find one `g.storey` per storey (unique id `storey-<id>`, the model storey in `data-storey`) with a title and its three layers;
* count the regions, lines and texts of every storey, the paths per line-style class (cut, projection, hidden, annotation) and the arc commands;
* read every path's `d` with an own SVG path reader (M, L, A, Z) and measure with GEOS: the area of the straight cut poche regions
  (even-odd, so the holes subtract) and the length of the straight lines per style, in metres at the drawing scale;
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

SCALE = 100.0
"""📐️ The drawing scale denominator: paper millimetres per model metre are 1000 / SCALE."""

MM_PER_METRE = 1000.0 / SCALE
CHORDS = 2048
"""🌀️ Chords per full circle when an arc is sampled."""

TOKEN = re.compile(r"[MLAZ]|-?\d+(?:\.\d+)?")


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
def measure(document):
    """🎨️ The oracle table of an SVG document plus the audit values that are not compared at 1e-9."""
    root = etree.fromstring(document)
    view = [float(part) for part in root.get("viewBox").split()]
    table = {"width": view[2], "height": view[3], "storeys": {}}
    sampled = {}
    for group in root.xpath("svg:g[@class='storey']", namespaces=NS):
        storey = group.get("data-storey")
        paths = group.xpath(".//svg:path", namespaces=NS)
        regions = [path for path in paths if "region" in classes(path)]
        lines = [path for path in paths if "line" in classes(path)]
        texts = group.xpath("svg:g[@class='layer texts']/svg:text", namespaces=NS)
        row = {
            "name": group.get("data-name"),
            "level": int(group.get("data-level")),
            "regions": len(regions),
            "lines": len(lines),
            "texts": len(texts),
            "styles": {style: sum(style in classes(path) for path in paths) for style in STYLES},
            "arcs": sum(path.get("d").count("A") for path in paths),
            "pocheArea": 0.0,
            "lineLength": {style: 0.0 for style in STYLES},
        }
        for path in regions:
            if "cut" in classes(path) and "A" not in path.get("d"):
                row["pocheArea"] += region_area(path.get("d"), False) / (MM_PER_METRE * MM_PER_METRE)
        for path in lines:
            if "A" not in path.get("d"):
                style = next(style for style in STYLES if style in classes(path))
                row["lineLength"][style] += line_length(path.get("d")) / MM_PER_METRE
        sampled[storey] = sum(region_area(path.get("d"), True) for path in regions if "cut" in classes(path)) / (MM_PER_METRE * MM_PER_METRE)
        table["storeys"][storey] = row
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
    if root.get("data-scale") != "100" or root.get("data-unit") != "mm":
        problems.append("the scale is not declared as 1:100 in millimetres")
    groups = root.xpath("svg:g[@class='storey']", namespaces=NS)
    if sorted(group.get("data-storey") for group in groups) != sorted(snapshot["storeys"]):
        problems.append("the storey groups are %s, the model has %s" % (sorted(group.get("data-storey") for group in groups), sorted(snapshot["storeys"])))
    ids = [group.get("id") for group in groups]
    if len(set(ids)) != len(ids):
        problems.append("the storey group ids are not unique")
    for group in groups:
        storey = group.get("data-storey")
        if group.get("id") != "storey-" + re.sub(r"[^A-Za-z0-9_.-]", "_", storey):
            problems.append("%s: the group id is %s" % (storey, group.get("id")))
        if len(group.xpath("svg:title", namespaces=NS)) != 1 or [layer.get("class") for layer in group.xpath("svg:g", namespaces=NS)] != ["layer regions", "layer lines", "layer texts"]:
            problems.append("%s: the group lacks its title or its three layers" % storey)
        if group.get("data-name") != snapshot["storeys"][storey]["name"] or int(group.get("data-level")) != snapshot["storeys"][storey]["level"]:
            problems.append("%s: name or level differ from the model" % storey)
        for path in group.xpath(".//svg:path", namespaces=NS):
            if sum(style in classes(path) for style in STYLES) != 1:
                problems.append("%s: a path has not exactly one style class" % storey)
            subpaths(path.get("d"))
        for text in group.xpath(".//svg:text", namespaces=NS):
            if not (math.isfinite(float(text.get("x"))) and math.isfinite(float(text.get("y")))):
                problems.append("%s: a text has no position" % storey)
        counted = sum(table["storeys"][storey]["styles"].values())
        if counted != table["storeys"][storey]["regions"] + table["storeys"][storey]["lines"]:
            problems.append("%s: %d styled paths for %d regions and lines" % (storey, counted, table["storeys"][storey]["regions"] + table["storeys"][storey]["lines"]))
    return problems


def open_case(root):
    """📂️ The committed snapshot, the committed file and the committed table of the house case."""
    case = Path(root) / "🏠️house"
    snapshot = json.loads((Path(root).parent / "🏗️ifc" / "🏠️house" / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    return case, snapshot, (case / "🏠️house.svg").read_bytes()


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

    return Adapter("python").oracle("export-svg-house", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def committed_form(table, sampled):
    """🧾️ The committed file: the compared table plus the audit values beside it."""
    return {**table, "audit": {"sampledPocheArea": {storey: round(area, 9) for storey, area in sampled.items()}}}


def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], arguments[1]
    case, snapshot, document = open_case(root)
    table, sampled = measure(document)
    problems = audit(document, snapshot, table)
    path = case / "🔬️measure" / "🔣️.json"
    expected = committed_form(table, sampled)
    if command == "write":
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(expected, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        print("wrote the table of %d storeys" % len(table["storeys"]))
    else:
        committed = json.loads(path.read_text(encoding="utf-8"))
        if committed != json.loads(json.dumps(expected)):
            problems.append("the committed table differs from the measurement of the committed file")
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (lxml %s, shapely %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", etree.LXML_VERSION, __import__("shapely").__version__))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
