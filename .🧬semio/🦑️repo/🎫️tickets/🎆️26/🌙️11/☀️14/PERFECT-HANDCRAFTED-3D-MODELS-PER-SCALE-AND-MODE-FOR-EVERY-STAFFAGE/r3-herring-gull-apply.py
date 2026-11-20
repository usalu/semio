"""📌 Round 3 herring gull: writes the corrected roots (rig, poses, anchors, provenance, and when present the redrawn representations, dimensions and thermal anchors of `r3-herring-gull-2d.json`) into the trial document (default) or the catalog file (`--catalog`); the target is re-read right before writing and only the named members change.

Usage: python3 r3-herring-gull-apply.py [--catalog]
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


io = module("io_", "r3-herring-gull-io.py")
roots = module("roots", "r3_herring_gull_roots.py")
notes = []
members = roots.members(notes)
extra = os.path.join(HERE, "r3-herring-gull-2d.json")
if os.path.exists(extra):
    drawn = json.load(open(extra, encoding="utf-8"))
    for key in ("representations", "dimensions", "summary"):
        if key in drawn:
            members[key] = drawn[key]
    for a in members["anchors"]:
        if a["id"] in drawn.get("anchors", {}):
            a.update(drawn["anchors"][a["id"]])
else:
    reps = json.loads(json.dumps(roots.ORIG["representations"]).replace('"arm-n"', '"humerus-n"').replace('"arm-f"', '"humerus-f"'))
    members["representations"] = reps
text = io.put(open(io.CATALOG, encoding="utf-8").read(), members)
target = io.CATALOG if "--catalog" in sys.argv else io.CANDIDATE
os.makedirs(os.path.dirname(target), exist_ok=True)
open(target, "w", encoding="utf-8").write(text)
print("wrote", target, "members", ", ".join(members), "|", "; ".join(notes) or "poses inside the joint limits")
