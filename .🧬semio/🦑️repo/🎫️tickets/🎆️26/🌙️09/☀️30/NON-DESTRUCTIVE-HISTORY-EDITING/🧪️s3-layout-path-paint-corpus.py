"""🧫️ Independent author of the Canvas2d path paint corpus (`📐️Canvas2dHost/🧫️fixtures/🧫️path-paint`).

A second implementation of the `segments` scene node in numpy, independent of React `drawSceneNode` and of the wgpu twin
`canvas2d_paint`: SVG path semantics (a path starts with a move, a segment after a close starts at the closed subpath's
start, an arc follows the SVG endpoint parameterization with radius correction), DENSE sampling instead of adaptive
flattening, fill membership by exact winding number (evenodd / nonzero) and stroke membership by centreline distance with
the dash pattern walked along the arc length. Every probe must keep `MARGIN` screen pixels from every fill boundary,
stroke boundary and dash boundary and `4·halfWidth + MARGIN` from every stroked join or end, so any faithful
rasterization agrees; a probe that does not is refused. The corpus is validated against its schema of record with
`jsonschema` (draft 7) plus two negative documents; `--check` only verifies the committed corpus is current.

Usage: .venv/bin/python 🧪️s3-layout-path-paint-corpus.py [--check]
"""

import json
import math
import pathlib
import sys

import numpy as np
from jsonschema import Draft7Validator

REPO = pathlib.Path(__file__).resolve().parents[7]
HOST = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost"
CORPUS = HOST / "🧫️fixtures/🧫️path-paint/🔣️.json"
SCHEMA = HOST / "🧬️schema/🔣️path-paint/🔣️.json"

MARGIN = 1.0
CURVE_SAMPLES = 1024
VIEWPORT = {"width": 240, "height": 180}


#region 🧮️Geometry
def affine(case):
    a, b, c, d, e, f = case["layer"].get("transform", [1, 0, 0, 1, 0, 0])
    camera, zoom = case["camera"], case["camera"]["zoom"]
    local = np.array([[a, c, e], [b, d, f], [0, 0, 1]], dtype=float)
    view = np.array([[zoom, 0, case["viewport"]["width"] * 0.5 - camera["x"] * zoom], [0, zoom, case["viewport"]["height"] * 0.5 - camera["y"] * zoom], [0, 0, 1]], dtype=float)
    return view @ local


def mapped(matrix, points):
    points = np.asarray(points, dtype=float).reshape(-1, 2)
    return (matrix[:2, :2] @ points.T).T + matrix[:2, 2]


def bezier(points, samples):
    t = np.linspace(0.0, 1.0, samples + 1)[1:, None]
    p = np.asarray(points, dtype=float)
    if len(p) == 3:
        return (1 - t) ** 2 * p[0] + 2 * (1 - t) * t * p[1] + t ** 2 * p[2]
    return (1 - t) ** 3 * p[0] + 3 * (1 - t) ** 2 * t * p[1] + 3 * (1 - t) * t ** 2 * p[2] + t ** 3 * p[3]


def arc(start, segment, samples):
    """🌙️ SVG F.6.5 endpoint → centre conversion with the F.6.6 radius correction, densely sampled."""
    (x1, y1), (x2, y2) = start, segment["to"]
    if (x1, y1) == (x2, y2):
        return np.empty((0, 2))
    rx, ry = abs(segment["rx"]), abs(segment["ry"])
    if rx == 0 or ry == 0:
        return np.array([[x2, y2]], dtype=float)
    phi = math.radians(segment["rotation"])
    cos, sin = math.cos(phi), math.sin(phi)
    hx, hy = (x1 - x2) / 2, (y1 - y2) / 2
    px, py = cos * hx + sin * hy, -sin * hx + cos * hy
    scale = px * px / (rx * rx) + py * py / (ry * ry)
    if scale > 1:
        rx, ry = rx * math.sqrt(scale), ry * math.sqrt(scale)
    numerator = rx * rx * ry * ry - rx * rx * py * py - ry * ry * px * px
    denominator = rx * rx * py * py + ry * ry * px * px
    root = math.sqrt(max(0.0, numerator / denominator))
    if segment["largeArc"] == segment["sweep"]:
        root = -root
    cpx, cpy = root * rx * py / ry, -root * ry * px / rx
    cx, cy = cos * cpx - sin * cpy + (x1 + x2) / 2, sin * cpx + cos * cpy + (y1 + y2) / 2
    theta = math.atan2((py - cpy) / ry, (px - cpx) / rx)
    end = math.atan2((-py - cpy) / ry, (-px - cpx) / rx)
    delta = end - theta
    if segment["sweep"] and delta < 0:
        delta += 2 * math.pi
    if not segment["sweep"] and delta > 0:
        delta -= 2 * math.pi
    angles = theta + delta * np.linspace(0.0, 1.0, samples + 1)[1:]
    points = np.stack([cx + rx * cos * np.cos(angles) - ry * sin * np.sin(angles), cy + rx * sin * np.cos(angles) + ry * cos * np.sin(angles)], axis=1)
    radial = ((cos * (points[:, 0] - cx) + sin * (points[:, 1] - cy)) / rx) ** 2 + ((-sin * (points[:, 0] - cx) + cos * (points[:, 1] - cy)) / ry) ** 2
    if np.max(np.abs(radial - 1.0)) > 1e-9 or np.hypot(*(points[-1] - np.array([x2, y2]))) > 1e-9:
        raise SystemExit(f"arc sampling left its ellipse: {segment}")
    return points


def contours(case):
    """✏️ Screen-space subpaths `(points, closed, vertices)`; `vertices` are the real joins and ends (segment endpoints)."""
    matrix = affine(case)
    result, current, cursor, start = [], None, None, None
    for segment in case["layer"]["segments"]:
        kind = segment["kind"]
        if kind == "move":
            if current:
                result.append(current)
            current = {"points": [segment["to"]], "closed": False, "vertices": [0]}
            cursor = start = segment["to"]
            continue
        if kind == "close":
            if cursor is None:
                break
            if current:
                current["closed"] = True
                result.append(current)
                current = None
            cursor = start
            continue
        if cursor is None:
            break
        if current is None:
            current = {"points": [cursor], "closed": False, "vertices": [0]}
        if kind == "line":
            local = np.array([segment["to"]], dtype=float)
        elif kind == "quad":
            local = bezier([cursor, segment["ctrl"], segment["to"]], CURVE_SAMPLES)
        elif kind == "cubic":
            local = bezier([cursor, segment["ctrl1"], segment["ctrl2"], segment["to"]], CURVE_SAMPLES)
        else:
            local = arc(cursor, segment, CURVE_SAMPLES)
        current["points"].extend(local.tolist())
        current["vertices"].append(len(current["points"]) - 1)
        cursor = segment["to"]
    if current:
        result.append(current)
    for entry in result:
        entry["points"] = mapped(matrix, entry["points"])
    return result, math.sqrt(abs(np.linalg.det(matrix[:2, :2])))


def edges(points, closed):
    tail = np.roll(points, -1, axis=0) if closed else points[1:]
    head = points if closed else points[:-1]
    return head, tail


def distance(point, head, tail):
    """📏️ Distance from `point` to every edge plus the arc-length position of the nearest point."""
    span = tail - head
    length = np.hypot(span[:, 0], span[:, 1])
    t = np.clip(np.einsum("ij,ij->i", point - head, span) / np.where(length > 0, length * length, 1.0), 0.0, 1.0)
    nearest = head + span * t[:, None]
    gaps = np.hypot(*(point - nearest).T)
    index = int(np.argmin(gaps))
    return float(gaps[index]), float(np.concatenate([[0.0], np.cumsum(length)])[index] + t[index] * length[index])


def winding(point, subpaths):
    total = 0
    for entry in subpaths:
        head, tail = edges(entry["points"], True)
        upward = (head[:, 1] <= point[1]) & (tail[:, 1] > point[1])
        downward = (head[:, 1] > point[1]) & (tail[:, 1] <= point[1])
        side = (tail[:, 0] - head[:, 0]) * (point[1] - head[:, 1]) - (point[0] - head[:, 0]) * (tail[:, 1] - head[:, 1])
        total += int(np.sum(upward & (side > 0))) - int(np.sum(downward & (side < 0)))
    return total


def dash_pattern(dash, scale):
    pattern = [value * scale for value in (dash * 2 if len(dash) % 2 else dash)]
    return pattern, sum(pattern)


def on_dash(position, pattern, period):
    offset = position % period
    for index, value in enumerate(pattern):
        if offset < value:
            return index % 2 == 0, min(offset, value - offset)
        offset -= value
    return True, 0.0
#endregion 🧮️Geometry


#region 🎯️Probes
def classify(case, probe):
    subpaths, scale = contours(case)
    layer = case["layer"]
    point = np.asarray(probe, dtype=float)
    stroke = layer.get("stroke")
    on_stroke, dash_gap = False, False
    if stroke and subpaths:
        half = stroke["width"] * scale * 0.5
        for entry in subpaths:
            if len(entry["points"]) < 2:
                continue
            head, tail = edges(entry["points"], entry["closed"])
            gap, position = distance(point, head, tail)
            if abs(gap - half) < MARGIN:
                raise SystemExit(f"{case['name']} probe {probe}: {gap - half:+.3f} px from the stroke boundary")
            corners = entry["points"][entry["vertices"]]
            if np.min(np.hypot(*(corners - point).T)) < 4 * half + MARGIN:
                raise SystemExit(f"{case['name']} probe {probe}: within a join or end of the stroke")
            if gap >= half:
                continue
            if "dash" in stroke:
                pattern, period = dash_pattern(stroke["dash"], scale)
                inside, clearance = on_dash(position, pattern, period)
                if clearance < MARGIN:
                    raise SystemExit(f"{case['name']} probe {probe}: {clearance:.3f} px from a dash boundary")
                if not inside:
                    dash_gap = True
                    continue
            on_stroke = True
    if on_stroke:
        return "stroke", "on-stroke"
    if "fill" in layer and subpaths:
        for entry in subpaths:
            head, tail = edges(entry["points"], True)
            if len(entry["points"]) > 1 and distance(point, head, tail)[0] < MARGIN:
                raise SystemExit(f"{case['name']} probe {probe}: within {MARGIN} px of the fill boundary")
        turns = winding(point, subpaths)
        inside = turns % 2 != 0 if layer.get("fillRule", "evenodd") == "evenodd" else turns != 0
        if inside:
            return "fill", "inside"
        if any(winding(point, [entry]) != 0 for entry in subpaths):
            return "none", "hole"
    if dash_gap:
        return "none", "dash-gap"
    return "none", "outside" if subpaths else "no-paint"
#endregion 🎯️Probes


#region 🧫️Cases
def rect(x, y, w, h, rotation=0.0):
    cx, cy = x + w * 0.5, y + h * 0.5
    sin, cos = math.sin(rotation), math.cos(rotation)
    corners = [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
    if rotation:
        corners = [(cx + (px - cx) * cos - (py - cy) * sin, cy + (px - cx) * sin + (py - cy) * cos) for px, py in corners]
    return [{"kind": "move", "to": list(corners[0])}] + [{"kind": "line", "to": list(point)} for point in corners[1:]] + [{"kind": "close"}]


def square(cx, cy, half, clockwise=True):
    corners = [(cx - half, cy - half), (cx + half, cy - half), (cx + half, cy + half), (cx - half, cy + half)]
    if not clockwise:
        corners.reverse()
    return [{"kind": "move", "to": list(corners[0])}] + [{"kind": "line", "to": list(point)} for point in corners[1:]] + [{"kind": "close"}]


def star(cx, cy, radius):
    points = [(cx + radius * math.sin(2 * math.pi * k * 2 / 5), cy - radius * math.cos(2 * math.pi * k * 2 / 5)) for k in range(5)]
    return [{"kind": "move", "to": [round(points[0][0], 6), round(points[0][1], 6)]}] + [{"kind": "line", "to": [round(x, 6), round(y, 6)]} for x, y in points[1:]] + [{"kind": "close"}]


INK = [0.1, 0.45, 0.95, 1]
PAPER = [0.97, 0.97, 0.98, 1]
GUIDE = [0.75, 0.2, 0.2, 0.35]
CAMERA = {"x": 60, "y": 45, "zoom": 2}


def case(name, story, layer, probes, camera=CAMERA):
    entry = {"name": name, "story": story, "viewport": VIEWPORT, "camera": camera, "layer": layer}
    return dict(entry, probes=[probe(entry) if callable(probe) else probe for probe in probes])


def along(fraction, offset=0.0):
    """📍️ The screen point at `fraction` of the first subpath's arc length, `offset` pixels along its left normal."""
    def at(entry):
        points = contours(entry)[0][0]["points"]
        head, tail = edges(points, False)
        lengths = np.hypot(*(tail - head).T)
        target = fraction * lengths.sum()
        index = int(np.searchsorted(np.cumsum(lengths), target))
        t = (target - np.cumsum(lengths)[index] + lengths[index]) / lengths[index]
        direction = (tail[index] - head[index]) / lengths[index]
        point = head[index] + (tail[index] - head[index]) * t + np.array([-direction[1], direction[0]]) * offset
        return [round(float(point[0]), 3), round(float(point[1]), 3)]
    return at


CASES = [
    case("layout-selected-frame", "A layout frame as the Blueprint paints it (`host_layer`: rect segments, solid fill, the 2.5 selection stroke) at zoom 2: the centre is fill, the middle of every edge is stroke, the paper beside it is empty.",
         {"id": "frame-rect", "segments": rect(10, 20, 100, 60), "fill": {"color": [0.85, 0.88, 0.92, 1]}, "stroke": {"color": INK, "width": 2.5}},
         [[120, 90], [120, 40], [20, 90], [220, 90], [120, 140], [120, 30], [10, 10]]),
    case("layout-rotated-frame", "`rotated_rect_segments` of a frame turned by half a radian, fill only: the turned body is fill, the bounding-box corners it no longer covers are empty.",
         {"id": "frame-turned", "segments": rect(20, 20, 80, 50, 0.5), "fill": {"color": [0.85, 0.85, 0.85, 1]}},
         [[120, 90], [150, 110], [90, 60], [45, 45], [195, 135], [195, 45], [60, 120]]),
    case("layout-margin-guide", "An open margin guide (`line_segments`, stroke width 1, no fill) at zoom 3: three pixels on the line are stroke, beside it nothing.",
         {"id": "layout.guide.margin", "segments": [{"kind": "move", "to": [0, 30]}, {"kind": "line", "to": [120, 30]}], "stroke": {"color": GUIDE, "width": 1}},
         [[120, 90], [120, 84], [120, 96], [60, 90]], {"x": 60, "y": 30, "zoom": 3}),
    case("layout-inherited-dash", "An inherited frame's outline dashed `[4, 3]` at zoom 2 (dash 8 px, gap 6 px): the dash is stroke, the gap on the centreline and the unfilled interior are empty.",
         {"id": "frame-inherited", "segments": rect(10, 20, 100, 60), "stroke": {"color": [0.3, 0.3, 0.3, 1], "width": 1, "dash": [4, 3]}},
         [[66, 40], [60, 40], [120, 90], [20, 80]]),
    case("evenodd-hole", "Two nested squares wound the same way under the default `evenodd` rule: the ring is fill, the inner square is a hole.",
         {"id": "ring-evenodd", "segments": square(60, 45, 40) + square(60, 45, 15), "fill": {"color": PAPER}},
         [[120, 90], [60, 90], [170, 50], [10, 10]]),
    case("nonzero-same-winding", "The same nested squares under `nonzero`: both wind the same way, so the inner square is fill too.",
         {"id": "ring-nonzero", "segments": square(60, 45, 40) + square(60, 45, 15), "fill": {"color": PAPER}, "fillRule": "nonzero"},
         [[120, 90], [60, 90], [10, 10]]),
    case("nonzero-opposite-winding", "Under `nonzero` an inner square wound the other way cancels the outer one: a hole again.",
         {"id": "ring-opposite", "segments": square(60, 45, 40) + square(60, 45, 15, clockwise=False), "fill": {"color": PAPER}, "fillRule": "nonzero"},
         [[120, 90], [60, 90], [10, 10]]),
    case("star-evenodd", "A self-intersecting pentagram under `evenodd`: the points are fill, the pentagon in the middle winds twice and is empty.",
         {"id": "star-evenodd", "segments": star(60, 45, 40), "fill": {"color": INK}},
         [[120, 90], [120, 20], [10, 10]]),
    case("star-nonzero", "The same pentagram under `nonzero`: the middle pentagon is fill.",
         {"id": "star-nonzero", "segments": star(60, 45, 40), "fill": {"color": INK}, "fillRule": "nonzero"},
         [[120, 90], [120, 20], [10, 10]]),
    case("quarter-pie-arc", "A quarter pie (`arc` of radius 40): a probe between the chord and the arc is fill — the arc is never flattened to its chord — and beyond the arc nothing.",
         {"id": "pie", "segments": [{"kind": "move", "to": [40, 25]}, {"kind": "line", "to": [80, 25]}, {"kind": "arc", "rx": 40, "ry": 40, "rotation": 0, "largeArc": False, "sweep": True, "to": [40, 65]}, {"kind": "close"}], "fill": {"color": INK}},
         [[100, 80], [129.5, 99.5], [165, 135], [60, 140]]),
    case("rotated-large-arc-stroke", "A large counter-clockwise arc of a rotated ellipse (radii 30/15, 35°), stroked 3 wide at zoom 2: points on the far side of the arc are stroke, the centre of the ellipse is empty.",
         {"id": "arc-stroke", "segments": [{"kind": "move", "to": [45, 40]}, {"kind": "arc", "rx": 30, "ry": 15, "rotation": 35, "largeArc": True, "sweep": False, "to": [75, 50]}], "stroke": {"color": INK, "width": 3}},
         [along(0.5), along(0.25), along(0.8), along(0.5, 12.0), [10, 10]]),
    case("quad-and-cubic-bulges", "A closed outline with a quadratic and a cubic bulge: probes inside each bulge, past the chords, are fill.",
         {"id": "curves", "segments": [{"kind": "move", "to": [20, 20]}, {"kind": "line", "to": [100, 20]}, {"kind": "quad", "ctrl": [130, 45], "to": [100, 70]}, {"kind": "cubic", "ctrl1": [80, 95], "ctrl2": [40, 95], "to": [20, 70]}, {"kind": "close"}], "fill": {"color": PAPER}},
         [[120, 90], [215, 90], [120, 160], [235, 90], [60, 175]]),
    case("transformed-frame", "A frame with a local transform (turned 30° and scaled 1.5 about a shifted origin) stroked 2 wide: the transform moves the fill and widens the stroke to 6 px.",
         {"id": "frame-transformed", "segments": rect(0, 0, 40, 20), "transform": [1.299038105676658, 0.75, -0.75, 1.299038105676658, 40, 20], "fill": {"color": [0.85, 0.88, 0.92, 1]}, "stroke": {"color": INK, "width": 2}},
         [[117, 96], [131.96, 70.0], [10, 10], [230, 170]]),
    case("subpath-after-close", "A segment after `close` starts a new subpath at the closed subpath's start: both triangles are fill.",
         {"id": "after-close", "segments": [{"kind": "move", "to": [60, 45]}, {"kind": "line", "to": [100, 45]}, {"kind": "line", "to": [100, 85]}, {"kind": "close"}, {"kind": "line", "to": [20, 45]}, {"kind": "line", "to": [20, 5]}, {"kind": "close"}], "fill": {"color": INK}},
         [[180, 110], [60, 70], [120, 160], [100, 30]]),
    case("open-subpath-fill", "An open subpath is filled as if closed: the triangle is fill although no `close` ends it.",
         {"id": "open", "segments": [{"kind": "move", "to": [30, 20]}, {"kind": "line", "to": [90, 20]}, {"kind": "line", "to": [60, 70]}], "fill": {"color": INK}},
         [[120, 70], [120, 160], [40, 90]]),
    case("thick-stroke-over-fill", "A 10 wide stroke paints over the fill it borders: a probe inside the shape but within half the width of an edge is stroke.",
         {"id": "thick", "segments": rect(20, 20, 80, 50), "fill": {"color": PAPER}, "stroke": {"color": INK, "width": 10, "join": "miter"}},
         [[120, 90], [120, 52], [120, 46], [120, 42], [235, 175]]),
]
#endregion 🧫️Cases


def corpus():
    cases = []
    for entry in CASES:
        probes = []
        for probe in entry["probes"]:
            paint, reason = classify(entry, probe)
            probes.append({"at": probe, "paint": paint, "reason": reason})
        cases.append(dict(entry, probes=probes))
    return {"margin": MARGIN, "cases": cases}


def validate(document):
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    validator = Draft7Validator(schema)
    errors = sorted(validator.iter_errors(document), key=lambda error: list(error.path))
    if errors:
        raise SystemExit("corpus invalid: " + "; ".join(f"{list(error.path)}: {error.message}" for error in errors[:5]))
    stray = json.loads(json.dumps(document))
    stray["cases"][0]["layer"]["segments"][1]["ctrl"] = [0, 0]
    if not list(validator.iter_errors(stray)):
        raise SystemExit("the schema accepts a line segment with a control point")
    rule = json.loads(json.dumps(document))
    rule["cases"][0]["layer"]["fillRule"] = "winding"
    if not list(validator.iter_errors(rule)):
        raise SystemExit("the schema accepts an unknown fill rule")


def render(document):
    return json.dumps(document, ensure_ascii=False, indent=2) + "\n"


def main():
    document = corpus()
    validate(document)
    text = render(document)
    counts = {paint: sum(1 for entry in document["cases"] for probe in entry["probes"] if probe["paint"] == paint) for paint in ("stroke", "fill", "none")}
    summary = f"{len(document['cases'])} cases, {sum(counts.values())} probes ({counts['stroke']} stroke, {counts['fill']} fill, {counts['none']} none)"
    if "--check" in sys.argv:
        current = CORPUS.read_text(encoding="utf-8") if CORPUS.exists() else ""
        if current != text:
            raise SystemExit(f"{CORPUS} is stale: re-run without --check")
        print(f"corpus current: {summary}")
        return
    CORPUS.parent.mkdir(parents=True, exist_ok=True)
    CORPUS.write_text(text, encoding="utf-8")
    print(f"wrote {CORPUS.relative_to(REPO)}: {summary}")
    for entry in document["cases"]:
        print(f"  {entry['name']}: " + ", ".join(f"{probe['at']}={probe['paint']}/{probe['reason']}" for probe in entry["probes"]))


if __name__ == "__main__":
    main()
