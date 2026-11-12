"""Timing and frame-hash probe for the Nakagin tower drawing (not a render).

Usage: measure_tower.py <label> [hash-output.json]
Builds the tower the way the Grasshopper tower clip does for every storey (growing, tinted, typed) and records wall times,
template builds, ordering calls and a pixel hash of each drawing rendered at tiny resolution.
"""
import hashlib
import importlib
import json
import os
import sys
import time
from pathlib import Path

import numpy as np
from manim import Camera

visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")

counts = {"template": 0, "order": 0}
_template, _order = visuals.NakaginDrawing._tower_drawing, visuals._section_order


def counted_template(self, *a, **k):
    counts["template"] += 1
    return _template(self, *a, **k)


def counted_order(*a, **k):
    counts["order"] += 1
    return _order(*a, **k)


visuals.NakaginDrawing._tower_drawing = counted_template
visuals._section_order = counted_order


def pixel_hash(drawing):
    camera = Camera(pixel_width=96, pixel_height=54, frame_width=14.222, frame_height=8.0, background_color="#ffffff")
    camera.capture_mobjects([drawing])
    return hashlib.sha256(np.asarray(camera.get_image()).tobytes()).hexdigest()


def main():
    label, out = sys.argv[1], sys.argv[2]
    ks = [int(value) for value in sys.argv[3:]]
    draw = visuals.NakaginDrawing()
    towers = model.towers()
    pods = towers[1].first_capsule_storey
    rows, hashes = {}, {}
    started = time.perf_counter()

    def timed(name, **options):
        t0 = time.perf_counter()
        drawing = draw.tower_axon(at=(0, -1), scale=0.05, **options)
        rows[name] = round(time.perf_counter() - t0, 3)
        hashes[name] = pixel_hash(drawing)
        print(label, name, rows[name], counts, flush=True)

    for k in ks:
        timed(f"growing-{k}", towers=(1,), storeys=k + pods, where=lambda s, k=k: s.storey < k, podium=False, capital=True, types=True)
    for k in ks + ks[:1]:
        timed(f"revisit-{k}", towers=(1,), storeys=k + pods, where=lambda s, k=k: s.storey < k, podium=False, capital=True, types=True)
    result = {"label": label, "profile": os.environ.get("SEMIO_TEACHING_PROFILE"), "totalSeconds": round(time.perf_counter() - started, 3), "seconds": rows, "counts": counts}
    print(json.dumps(result), flush=True)
    Path(out).write_text(json.dumps({"hashes": hashes, "timing": result}, indent=1), encoding="utf-8")


main()
