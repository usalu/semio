"""↩️ Author 07: puts the 2D geometry of one scale and view of an asset back from a saved catalog copy (`🗑️generated/rework-07/<slug>.catalog.orig.json`).

Usage: python3 rework-07-2d-restore.py <slug> <scale> [view,view]
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.argv, ARGS = [sys.argv[0], "noop"], sys.argv[1:]
SOURCE = open(os.path.join(HERE, "rework-07-2d.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
SCOPE = {"__file__": os.path.join(HERE, "rework-07-2d.py")}
exec(SOURCE, SCOPE)
slug, scale = ARGS[0], ARGS[1]
views = ARGS[2].split(",") if len(ARGS) > 2 else ["elevation", "front", "plan"]
saved = json.load(open(os.path.join(HERE, "🗑️generated", "rework-07", slug + ".catalog.orig.json"), encoding="utf-8"))["asset"]["representations"][scale]["geometry"]


def mutate(representations, asset):
    for view in views:
        representations[scale]["geometry"][view] = saved[view]


SCOPE["splice"](os.path.join(SCOPE["CATALOG"], slug, "🔣️.json"), "representations", mutate)
print("[DEBUG] restored 1:%s %s" % (scale, ",".join(views)))
