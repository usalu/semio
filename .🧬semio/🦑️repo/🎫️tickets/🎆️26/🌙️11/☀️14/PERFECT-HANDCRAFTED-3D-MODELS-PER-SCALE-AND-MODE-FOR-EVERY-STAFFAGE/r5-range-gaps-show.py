"""🔎 R01: prints the shapes (all views) and the body/thermal/structure solids of the named ids at one scale of an asset, and whether the volumes source equals the catalog volumes.

Usage: python3 r5-range-gaps-show.py <category>/<slug> <scale> <id> [<id> ...]
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-range-gaps-lib.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

spec, scale, ids = sys.argv[1], sys.argv[2], set(sys.argv[3:])
asset = json.load(open(K.asset_path(spec), encoding="utf-8"))["asset"]
source = json.load(open(K.source_path(spec), encoding="utf-8"))["volumes"] if os.path.exists(K.source_path(spec)) else None
print("source == catalog volumes:", source == asset["volumes"], "| range", asset["representations"][scale].get("validRange"))
rep = asset["representations"][scale]
bones = set()
for view in ("elevation", "front", "plan"):
    for s in rep["geometry"][view]:
        if s["id"] in ids:
            bones.add(s.get("bone"))
            print(view, json.dumps(s))
volume = asset["volumes"][scale]
for name in ("body", "thermal", "structure"):
    for s in volume.get(name, []):
        if s["id"] in ids or s.get("of") in ids:
            print(name, json.dumps(s))
print("bones", bones)
