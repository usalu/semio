"""✍️ Round 3 roe deer: puts members into the work document (default) or, with --catalog, writes the finished members of the work document into the catalog asset file in one write (every other byte untouched, the file re-read right before).

Usage: python3 r3-roe-deer-apply.py reps | dims <height> <length> <width> | --catalog
"""
import importlib.util
import json
import sys

spec = importlib.util.spec_from_file_location("io", "r3-roe-deer-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
ROOT = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "clips", "volumes"]
if sys.argv[1] == "reps":
    io.candidate(json.load(open("🗑️generated/r3-roe-deer/reps.json", encoding="utf-8")))
elif sys.argv[1] == "dims":
    d = dict(io.work()["dimensions"])
    d["heightM"], d["widthM"], d["depthM"] = (float(v) for v in sys.argv[2:5])
    io.candidate({"dimensions": d})
elif sys.argv[1] == "volumes":
    io.candidate({"volumes": json.load(open("volumes/adhoc/r3-roe-deer.json", encoding="utf-8"))["volumes"]})
elif sys.argv[1] == "--catalog":
    work, now = io.work(), io.load()
    io.put({k: work[k] for k in ROOT if work[k] != now[k]})
print("ok", sys.argv[1])
