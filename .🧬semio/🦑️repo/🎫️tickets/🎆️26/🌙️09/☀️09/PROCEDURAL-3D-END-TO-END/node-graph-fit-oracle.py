#!/usr/bin/env python3
"""Independent oracle of the `fitGraph` action fixture: recomputes every `fitRows` expectation of the shared
camera-fit fixture from the law text alone (graph bounds + viewport -> camera), with no Rust and no TypeScript,
and checks the declared action rows. `--write` regenerates the expectations; the default mode verifies them.
Run: python3 node-graph-fit-oracle.py [--write]"""
import json
import math
import pathlib
import sys

FIXTURE = next(pathlib.Path(__file__).resolve().parents[7].glob("🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🧫️fixtures/📷️camera-fit/🔣️.json"))


def bounds(nodes):
    if not nodes:
        return None
    return (
        min(n["x"] - n["width"] / 2 for n in nodes),
        min(n["y"] - n["height"] / 2 for n in nodes),
        max(n["x"] + n["width"] / 2 for n in nodes),
        max(n["y"] + n["height"] / 2 for n in nodes),
    )


def fit(nodes, width, height, padding, zoom_min, zoom_max):
    box = bounds(nodes)
    if box is None:
        return None
    min_x, min_y, max_x, max_y = box
    usable_w = max(max(width, 1) - 2 * padding, 1)
    usable_h = max(max(height, 1) - 2 * padding, 1)
    content_w, content_h = max(max_x - min_x, 0), max(max_y - min_y, 0)
    zoom_x = usable_w / content_w if content_w > 0 else math.inf
    zoom_y = usable_h / content_h if content_h > 0 else math.inf
    zoom = min(zoom_x, zoom_y)
    zoom = zoom if math.isfinite(zoom) else 1.0
    return {"x": (min_x + max_x) / 2, "y": (min_y + max_y) / 2, "zoom": min(max(zoom, zoom_min), zoom_max)}


def main():
    document = json.loads(FIXTURE.read_text(encoding="utf8"))
    constants = document["provenance"]["constants"]
    failures = []
    for row in document["fitRows"]:
        expected = fit(row["nodes"], row["viewport"]["width"], row["viewport"]["height"], constants["paddingPx"], constants["zoomMin"], constants["zoomMax"])
        if "--write" in sys.argv:
            row["expect"] = {"camera": expected}
        elif row["expect"]["camera"] != expected:
            failures.append((row["name"], row["expect"]["camera"], expected))
    action = document["fitAction"]
    if action["id"] != "fitGraph" or action["key"] != "f" or action["label"] != {"en": "Fit graph", "de": "Graph einpassen"}:
        failures.append(("fitAction", action, "declaration drifted"))
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        print(f"wrote {len(document['fitRows'])} fitRows")
        return 0
    for failure in failures:
        print("FAIL", *failure)
    print(f"{len(document['fitRows']) - len(failures)}/{len(document['fitRows'])} fit rows agree with the independent oracle")
    return 1 if failures else 0


raise SystemExit(main())
