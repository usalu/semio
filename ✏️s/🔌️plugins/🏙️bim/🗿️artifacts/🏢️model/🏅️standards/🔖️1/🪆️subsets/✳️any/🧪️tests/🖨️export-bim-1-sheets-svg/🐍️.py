#!/usr/bin/env python3
"""🖨️ Third-party ORACLE (lxml + shapely) for the SVG 1.1 sheets of the BIM room.

lxml (libxml2) and shapely (GEOS) have never seen this repository's writer. They open the committed files `🧫️fixtures/🚪️sheets/🏠️room/<number>.svg` and:

* parse each as namespaced XML, require the SVG root with version 1.1, a millimetre size equal to its viewBox, and read the sheet id and the paper the root names;
* find one `g.viewport` per viewport (its id in `data-viewport`, its scale in `data-scale`), follow its `clip-path` to the `clipPath` rectangle and measure that window as a shapely `box`;
* list the text runs of the title block and of the revision table, sorted;
* audit that the window of a cropped viewport is exactly the crop of the committed model at the scale of the viewport (`crop size in metres * 1000 / scale`, at least 10 mm)
  and that the drawing of a viewport (every straight vertex of its paths, read by the own path reader of the sibling SVG oracle) lands on its window: a mapping or scale error
  would put the vertices elsewhere.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures/🚪️sheets>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🚪️sheets>     # rewrite the measured table from the committed files

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/📄️sheets/🎨️svg/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import importlib.util
import json
import re
import sys
from pathlib import Path

from lxml import etree
from shapely.geometry import MultiPoint, box

# endregion 🔖️Imports


# region 🔖️Reading
NS = {"svg": "http://www.w3.org/2000/svg"}
MIN_WINDOW = 10.0
SLACK = 1e-6
URL = re.compile(r"url\(#([^)]+)\)")


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def subpaths(d):
    """✒️ The subpaths of a path (the path reader of the SVG view oracle)."""
    return load_sibling("🎨️export-bim-1-svg", "svg").subpaths(d)


def millimetres(text):
    """📏️ A length like `420mm` in millimetres."""
    if not text.endswith("mm"):
        raise AssertionError("the size %s is not in millimetres" % text)
    return float(text[:-2])


def group_with(root, name):
    """🔎️ The group of a class directly below the root."""
    found = root.xpath("svg:g[contains(concat(' ', normalize-space(@class), ' '), ' %s ')]" % name, namespaces=NS)
    return found[0] if found else None


def texts_of(group):
    """🔤️ The sorted text runs of a group."""
    if group is None:
        return []
    return sorted(text.text for text in group.xpath(".//svg:text", namespaces=NS) if text.text)


def window_of(root, viewport):
    """🪟️ The clip window of a viewport group as a shapely box."""
    match = URL.fullmatch(viewport.get("clip-path") or "")
    if match is None:
        raise AssertionError("%s has no clip-path" % viewport.get("data-viewport"))
    clip = root.xpath("//svg:clipPath[@id='%s']/svg:rect" % match.group(1), namespaces=NS)
    if len(clip) != 1:
        raise AssertionError("%s: the clip-path %s names no single rectangle" % (viewport.get("data-viewport"), match.group(1)))
    x, y, width, height = (float(clip[0].get(name)) for name in ("x", "y", "width", "height"))
    return box(x, y, x + width, y + height)


def vertices(viewport):
    """📍️ Every vertex of the paths of a viewport group."""
    points = []
    for path in viewport.xpath(".//svg:path", namespaces=NS):
        for subpath in subpaths(path.get("d")):
            points.extend(subpath["points"])
    return points


def measure_sheet(document):
    """📄️ The table of one sheet file and its geometry for the audit."""
    root = etree.fromstring(document)
    if root.tag != "{%s}svg" % NS["svg"] or root.get("version") != "1.1":
        raise AssertionError("not an SVG 1.1 root")
    width, height = millimetres(root.get("width")), millimetres(root.get("height"))
    view_box = [float(item) for item in root.get("viewBox").split()]
    if view_box != [0.0, 0.0, width, height]:
        raise AssertionError("the viewBox %s is not the size %sx%s mm" % (view_box, width, height))
    viewports, geometry = {}, {}
    for group in root.xpath("svg:g[contains(concat(' ', normalize-space(@class), ' '), ' viewport ')]", namespaces=NS):
        identity = group.get("data-viewport")
        window = window_of(root, group)
        x0, y0, x1, y1 = window.bounds
        viewports[identity] = {"scale": int(group.get("data-scale")), "window": [x0, y0, x1 - x0, y1 - y0]}
        geometry[identity] = (window, vertices(group))
    table = {
        "paper": root.get("data-paper"),
        "size": [width, height],
        "viewports": viewports,
        "title_texts": texts_of(group_with(root, "title-block")),
        "revision_texts": texts_of(group_with(root, "revision-table")),
    }
    return root.get("data-sheet"), table, geometry


def measure(documents):
    """🖨️ The table of every sheet file, by sheet id, and the geometry for the audit."""
    tables, geometry = {}, {}
    for document in documents:
        identity, table, found = measure_sheet(document)
        tables[identity] = table
        geometry[identity] = found
    return tables, geometry


def audit(snapshot, tables, geometry):
    """🩺️ Windows equal the crops at their scales and hold the drawing of their viewport."""
    problems = []
    for sheet_id, table in tables.items():
        for viewport_id, row in table["viewports"].items():
            window = geometry[sheet_id][viewport_id][0]
            model = snapshot["viewports"][viewport_id]
            crop = model.get("crop")
            if crop is not None:
                mm = 1000.0 / model["scale"]
                expected = [max((crop["max"]["x"] - crop["min"]["x"]) * mm, MIN_WINDOW), max((crop["max"]["y"] - crop["min"]["y"]) * mm, MIN_WINDOW)]
                if abs(row["window"][2] - expected[0]) > 1e-9 or abs(row["window"][3] - expected[1]) > 1e-9:
                    problems.append("%s: the window %s is not the crop at 1:%d (%s)" % (viewport_id, row["window"][2:], model["scale"], expected))
            points = geometry[sheet_id][viewport_id][1]
            if points and not MultiPoint(points).intersects(window.buffer(SLACK)):
                problems.append("%s: the drawing does not land on its window" % viewport_id)
    return problems


# endregion 🔖️Reading


# region 🔖️Handlers
def export_handler(ctx):
    """🖨️ Oracle answer: the table of the committed files, after the audit."""
    from semio_repo_test import Outcome

    snapshot = json.loads(ctx.input_bytes(next(uri for uri in ctx.step_input_uris() if "📸️snapshot" in uri)).decode("utf-8"))
    tables, geometry = measure([ctx.input_bytes(uri) for uri in ctx.step_input_uris() if uri.endswith(".svg")])
    problems = audit(snapshot, tables, geometry)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(tables, raw=json.dumps(tables, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-sheets-svg-room", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed files against the committed table; `write` rewrites the table from the files."""
    command, root = arguments[0], Path(arguments[1])
    problems = []
    for case in sorted(path for path in root.iterdir() if path.is_dir()):
        snapshot = json.loads(root.parent.joinpath("💡️inferences", "📄️sheet-layout", case.name, "📸️snapshot", "🔣️.json").read_text(encoding="utf-8"))
        tables, geometry = measure([path.read_bytes() for path in sorted(case.glob("*.svg"))])
        problems += audit(snapshot, tables, geometry)
        path = case / "🔬️measure-svg" / "🔣️.json"
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(tables, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote the table of %d sheets" % (case.name, len(tables)))
        elif json.loads(path.read_text(encoding="utf-8")) != json.loads(json.dumps(tables)):
            problems.append("%s: the committed table differs from the measurement of the committed files" % case.name)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (lxml %s, shapely %s)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", etree.LXML_VERSION, __import__("shapely").__version__))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
