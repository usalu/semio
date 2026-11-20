"""💾 Writes the root fixes of `humans/kneeling-gardener` into the catalog asset in one step: rig, anchors, poses, dimensions, symbols and the garden reference (`r3-kneeling-gardener-roots.py`) together with the redrawn 2D geometry (`r3-kneeling-gardener-draw2d.py`) and its drawing decisions (`r3-kneeling-gardener-decisions.py`). The file is read again right before writing and every other member stays byte-identical; `volumes` follows with `volumes-insert.py`.

Usage: python3 r3-kneeling-gardener-write.py
"""
import importlib.util
import os

here = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(here, "r3-kneeling-gardener-%s.py" % name))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


roots = load("roots")
draw2d = load("draw2d")
decisions = load("decisions")
splice = load("splice")


def change(asset):
    return roots.change(asset) + list(dict.fromkeys(draw2d.change(asset) + decisions.change(asset)))


print(splice.replace_members(change))
