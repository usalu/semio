"""🖋 Splices the redrawn 2D geometry of `humans/jogger` (written by `r3-jogger-draw2d.ts`) into the three views of every scale of the asset file; only `representations` changes, and in it only `geometry`.

Usage: python3 r3-jogger-draw2d.py
"""
import importlib.util
import json
import os

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("splice", os.path.join(here, "r3-jogger-splice.py"))
splice = importlib.util.module_from_spec(spec)
spec.loader.exec_module(splice)
geometry = json.load(open(os.path.join(here, "🗑️generated", "r3-jogger", "geometry.json"), encoding="utf-8"))


def change(asset):
    for scale, views in geometry.items():
        for view, shapes in views.items():
            asset["representations"][scale]["geometry"][view] = shapes
    return ["representations"]


print(splice.replace_members(change))
