#!/usr/bin/env python3
"""📄️ Third-party ORACLE for the `s.bim.model@1` inference `📄️sheet-layout`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, per sheet, where everything goes on the paper in millimetres from the top left corner (x to the right, y downward): the paper as it
lies, the frame (20 mm at the binding edge, 10 mm elsewhere), the title block in the bottom right corner of the frame, the revision table directly above it, the window of every viewport and the
findings about the placement. None of that is stored: the snapshot holds the paper, the orientation, the title block texts, the viewports (position, scale, crop) and the revision rows only.
This file re-derives the table from the SAME committed snapshot without sharing a line of code with the subject, and lets `shapely` 2 (GEOS), a library that has never seen this repository,
adjudicate the geometry: every rectangle is a `box`, a window inside the frame is `covers` of the frame grown by a micrometre, an overlap is the area of an `intersection`, and the title block
and the revision table are boxes anchored to the frame. The parametric laws are metamorphic properties: moving a viewport moves its window by the same vector without resizing it, and halving
the scale denominator doubles both sides of its window. The windows of UNCROPPED drawn views depend on the extent of the linework (owned by the view oracle); the committed room crops every drawn
viewport, and an uncropped viewport of a camera view draws nothing and keeps the minimum window of 10 mm.

The committed expectation under `🧫️fixtures/💡️inferences/📄️sheet-layout/<case>/💡️inference/📄️sheets/🔣️.json` is WRITTEN by this file (`write`), never by hand, and the Rust subject is compared
against it.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/📄️sheet-layout>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/📄️sheet-layout>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import sys
from pathlib import Path

import shapely
from shapely.geometry import box

# endregion 🔖️Imports


# region 🔖️Vocabulary
EXACT = 1e-9
SLACK = 1e-6
BINDING_MARGIN = 20.0
MARGIN = 10.0
MIN_WINDOW = 10.0
TITLE_WIDTH = 180.0
TITLE_ROW = 16.0
REVISION_ROW = 6.0
PAPERS = {"A0": (1189.0, 841.0), "A1": (841.0, 594.0), "A2": (594.0, 420.0), "A3": (420.0, 297.0), "A4": (297.0, 210.0)}
DRAWN = ["Plan", "CeilingPlan", "Section", "Elevation"]


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
    """🧩️ The tag and the payload of an externally tagged value (`{"Iso": {...}}` or `"Landscape"`)."""
    if isinstance(value, str):
        return value, {}
    (tag, payload), = value.items()
    return tag, payload


def rect(shape):
    """▭️ `[x, y, width, height]` of a box."""
    x0, y0, x1, y1 = shape.bounds
    return [x0, y0, x1 - x0, y1 - y0]


# endregion 🔖️Vocabulary


# region 🔖️Geometry
def paper_of(sheet):
    """📐️ The (name, width, height) of the paper as it lies: the long side is the width of a landscape sheet."""
    tag, body = variant(sheet["paper"])
    if tag == "Iso":
        name = body["size"]
        long_side, short_side = PAPERS[name]
    else:
        long_side, short_side = max(body["width"], body["height"]), min(body["width"], body["height"])
        name = "%g × %g" % (long_side, short_side)
    landscape = variant(sheet["orientation"])[0] == "Landscape"
    return name, (long_side if landscape else short_side), (short_side if landscape else long_side)


def scale_text(scales):
    """📏️ The distinct scales from the largest scale to the smallest as one line."""
    return ", ".join("1:%d" % scale for scale in sorted(set(scales)))


def window_of(snapshot, viewport):
    """🪟️ The window of a viewport: its crop at the scale of the viewport, else the minimum window of a view that draws nothing; `None` when the extent of linework would be needed."""
    mm = 1000.0 / max(viewport["scale"], 1)
    x, y = viewport["position"]["x"], viewport["position"]["y"]
    crop = viewport.get("crop")
    if crop is not None:
        width = max((crop["max"]["x"] - crop["min"]["x"]) * mm, MIN_WINDOW)
        height = max((crop["max"]["y"] - crop["min"]["y"]) * mm, MIN_WINDOW)
        return box(x, y, x + width, y + height), mm, True
    if variant(snapshot["views"][viewport["view"]]["kind"])[0] not in DRAWN:
        return box(x, y, x + MIN_WINDOW, y + MIN_WINDOW), mm, False
    return None, mm, False


def sheet_table(snapshot, sheet_id):
    """📄️ The metrics of one sheet."""
    sheet = snapshot["sheets"][sheet_id]
    paper, width, height = paper_of(sheet)
    frame = box(BINDING_MARGIN, MARGIN, max(width - MARGIN, BINDING_MARGIN), max(height - MARGIN, MARGIN))
    frame_width, frame_height = frame.bounds[2] - frame.bounds[0], frame.bounds[3] - frame.bounds[1]
    title_width, row = min(TITLE_WIDTH, frame_width), min(TITLE_ROW, frame_height / 3.0)
    title = box(frame.bounds[2] - title_width, frame.bounds[3] - 3.0 * row, frame.bounds[2], frame.bounds[3])
    rows = sorted(((rev["number"], key, rev) for key, rev in snapshot.get("sheet_revisions", {}).items() if rev["sheet"] == sheet_id), key=lambda item: (item[0], item[1]))
    revisions = box(title.bounds[0], title.bounds[1] - (len(rows) + 1) * REVISION_ROW if rows else title.bounds[1], title.bounds[2], title.bounds[1])
    placed, found = [], []
    for key, viewport in sorted(snapshot.get("viewports", {}).items()):
        if viewport["sheet"] != sheet_id or viewport["view"] not in snapshot["views"]:
            continue
        window, mm, cropped = window_of(snapshot, viewport)
        if window is None:
            raise AssertionError("%s: the window of an uncropped drawn view needs the linework oracle" % key)
        empty = variant(snapshot["views"][viewport["view"]]["kind"])[0] not in DRAWN
        placed.append((key, viewport, window, mm, cropped, empty))
    for key, _, window, _, _, empty in placed:
        if not frame.buffer(SLACK, join_style="mitre").covers(window):
            found.append("viewport-outside|%s" % key)
        if window.intersection(title).area > SLACK or (rows and window.intersection(revisions).area > SLACK):
            found.append("viewport-over-title-block|%s" % key)
        if empty:
            found.append("viewport-empty|%s" % key)
    for index, first in enumerate(placed):
        for second in placed[index + 1:]:
            if first[2].intersection(second[2]).area > SLACK:
                found.append("viewports-overlap|%s,%s" % (first[0], second[0]))
    scales = [viewport["scale"] for _, viewport, *_ in placed]
    last = rows[-1][0] if rows else ""
    texts = {
        "project": sheet["project"],
        "number": sheet["number"],
        "title": sheet["name"],
        "scale": sheet["scale_label"] or scale_text(scales),
        "drawn-by": sheet["drawn_by"],
        "checked-by": sheet["checked_by"],
        "date": sheet["date"],
        "revision": sheet["revision"] or last,
    }
    return {
        "paper": paper,
        "size": [width, height],
        "frame": rect(frame),
        "title": rect(title),
        "title_text": texts,
        "revisions": rect(revisions),
        "revision_rows": ["%s|%s|%s|%s" % (rev["number"], rev["date"], rev["description"], rev["author"]) for _, _, rev in rows],
        "viewports": {key: {"scale": viewport["scale"], "mm": mm, "window": rect(window), "cropped": cropped, "empty": empty} for key, viewport, window, mm, cropped, empty in placed},
        "findings": sorted(found),
    }


def table(snapshot):
    """📏️ The metrics of every sheet that has a layout, by sheet id."""
    return {sheet_id: sheet_table(snapshot, sheet_id) for sheet_id in sorted(snapshot.get("sheets", {}))}


# endregion 🔖️Geometry


# region 🔖️Audit
def audit(snapshot):
    """🩺️ The parametric laws hold on perturbed copies of the model, and the sheet geometry is consistent."""
    problems = []
    base = table(snapshot)
    for sheet_id, metrics in base.items():
        frame = box(metrics["frame"][0], metrics["frame"][1], metrics["frame"][0] + metrics["frame"][2], metrics["frame"][1] + metrics["frame"][3])
        paper = box(0, 0, *metrics["size"])
        if not paper.covers(frame):
            problems.append("%s: the frame leaves the paper" % sheet_id)
        title = box(metrics["title"][0], metrics["title"][1], metrics["title"][0] + metrics["title"][2], metrics["title"][1] + metrics["title"][3])
        if not frame.buffer(SLACK).covers(title):
            problems.append("%s: the title block leaves the frame" % sheet_id)
    for key, viewport in snapshot.get("viewports", {}).items():
        if viewport["view"] not in snapshot["views"] or viewport.get("crop") is None:
            continue
        shifted = copy.deepcopy(snapshot)
        shifted["viewports"][key]["position"]["x"] += 7.5
        shifted["viewports"][key]["position"]["y"] += 2.5
        before, after = base[viewport["sheet"]]["viewports"][key]["window"], table(shifted)[viewport["sheet"]]["viewports"][key]["window"]
        if abs(after[0] - before[0] - 7.5) > EXACT or abs(after[1] - before[1] - 2.5) > EXACT or abs(after[2] - before[2]) > EXACT or abs(after[3] - before[3]) > EXACT:
            problems.append("%s: moving the viewport resized or misplaced its window" % key)
        if viewport["scale"] % 2 == 0 and before[2] > MIN_WINDOW * 2 and before[3] > MIN_WINDOW * 2:
            finer = copy.deepcopy(snapshot)
            finer["viewports"][key]["scale"] = viewport["scale"] // 2
            window = table(finer)[viewport["sheet"]]["viewports"][key]["window"]
            if abs(window[2] - 2.0 * before[2]) > 1e-7 or abs(window[3] - 2.0 * before[3]) > 1e-7:
                problems.append("%s: halving the scale denominator did not double the window" % key)
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


def sheets_handler(ctx):
    """📄️ Oracle answer for `📄️sheet-layout`, after GEOS and the parametric laws agreed with it."""
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

    return Adapter("python").oracle("sheets-room", sheets_handler)


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
        target = case / "💡️inference" / "📄️sheets" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "sheets")]
        print("%s: shapely %s, %d sheets, %d viewports" % (case.name, shapely.__version__, len(snapshot.get("sheets", {})), len(snapshot.get("viewports", {}))))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
