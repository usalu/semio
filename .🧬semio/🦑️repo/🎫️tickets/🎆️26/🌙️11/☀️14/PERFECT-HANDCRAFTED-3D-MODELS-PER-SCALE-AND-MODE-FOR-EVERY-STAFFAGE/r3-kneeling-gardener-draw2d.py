"""🖋 Splices the redrawn 2D geometry of `humans/kneeling-gardener` (written by `r3-kneeling-gardener-draw2d.ts`) into the three views of every authored scale: with `--candidate` into the candidate document under `🗑️generated/r3-kneeling-gardener/`, else as a function for `r3-kneeling-gardener-write.py`.

Usage: python3 r3-kneeling-gardener-draw2d.py --candidate
"""
import json
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))
generated = os.path.join(here, "🗑️generated", "r3-kneeling-gardener")


def change(asset):
    geometry = json.load(open(os.path.join(generated, "geometry.json"), encoding="utf-8"))
    for scale, views in geometry.items():
        for view, shapes in views.items():
            asset["representations"][scale]["geometry"][view] = shapes
    return ["representations"]


if __name__ == "__main__" and "--candidate" in sys.argv:
    target = os.path.join(generated, "🔣️.json")
    document = json.load(open(target, encoding="utf-8"))
    change(document["asset"])
    json.dump(document, open(target, "w", encoding="utf-8"), ensure_ascii=False)
