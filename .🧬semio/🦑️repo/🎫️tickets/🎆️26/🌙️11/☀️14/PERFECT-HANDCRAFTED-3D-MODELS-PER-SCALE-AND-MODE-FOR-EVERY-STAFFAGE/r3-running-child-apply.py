"""🧩 Writes the round 3 state of `humans/running-child`: the root fixes (`r3-running-child-roots.py`), the redrawn 2D geometry when `r3-running-child-draw2d.ts` has written it, the texts when stated, and with `--candidate` also the `volumes` source, into the candidate document under the generated folder; without it the roots, geometry and texts go into the catalog asset (its `volumes` are spliced by `volumes-insert.py`).

Usage: python3 r3-running-child-apply.py [--candidate]
"""
import importlib.util
import json
import os
import sys

here = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(here, "r3-running-child-%s.py" % name))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


splice = module("splice")
roots = module("roots")
candidate = "--candidate" in sys.argv
geometry_file = os.path.join(here, "🗑️generated", "r3-running-child", "geometry.json")
texts_file = os.path.join(here, "r3-running-child-texts.json")
source = os.path.join(here, "volumes", "humans", "running-child.json")


def change(asset):
    keys = roots.change(asset)
    if os.path.exists(geometry_file):
        for scale, views in json.load(open(geometry_file, encoding="utf-8")).items():
            for view, shapes in views.items():
                asset["representations"][scale]["geometry"][view] = shapes
        keys.append("representations")
    if os.path.exists(texts_file):
        texts = json.load(open(texts_file, encoding="utf-8"))
        for scale, decisions in texts.get("representations", {}).items():
            asset["representations"][scale]["decisions"].update(decisions)
            if "representations" not in keys:
                keys.append("representations")
        if "summary" in texts:
            asset["summary"] = texts["summary"]
            keys.append("summary")
        for language, sentence in texts["provenance"]["notes"].items():
            if not asset["provenance"]["notes"][language].endswith(sentence):
                asset["provenance"]["notes"][language] += sentence
        for reference in texts["provenance"]["references"]:
            if all(r["id"] != reference["id"] for r in asset["provenance"]["references"]):
                asset["provenance"]["references"].append(reference)
        keys.append("provenance")
        for clip in asset["clips"]:
            if clip["id"] in texts.get("clips", {}):
                clip["description"] = texts["clips"][clip["id"]]
        for pose in asset["poses"]:
            if pose["id"] in texts.get("poses", {}):
                pose["description"] = texts["poses"][pose["id"]]
    if candidate:
        asset["volumes"] = json.load(open(source, encoding="utf-8"))["volumes"]
        keys.append("volumes")
    return keys


print(splice.replace_members(change, candidate))
