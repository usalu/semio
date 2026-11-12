"""Export staffage casts at levels of detail into the cast cache and time each batch (a heavy job: queue it).

Run from the repo root with the teaching venv python, PYTHONUTF8=1, PYTHONPATH=<repo root>:
  python -I staffage-export.py <out.json> --lods full,preview,poster [--casts a,b,c | all] [--output-fps 15]
With SEMIO_TEACHING_CACHE_ROOT set to an empty folder the run is a cold baseline; without it the course cache is filled.
Writes seconds per level and per cast (wall clock around each ensure_casts), frames and bytes of each folder.
"""
import argparse
import importlib
import json
import os
import sys
import time
from pathlib import Path

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
parser = argparse.ArgumentParser()
parser.add_argument("out")
parser.add_argument("--lods", default="full,preview,poster")
parser.add_argument("--casts", default="all")
parser.add_argument("--output-fps", type=float, default=15.0)
parser.add_argument("--batch", action="store_true", help="one exporter call per level for all casts (fills the cache; no per-cast seconds)")
arguments = parser.parse_args()
casts = list(S._manifest()["casts"]) if arguments.casts == "all" else arguments.casts.split(",")
report = {"cacheRoot": str(S._cache_root()), "outputFps": arguments.output_fps, "levels": {}}
for name in arguments.lods.split(","):
    lod = S.lod_level(name, arguments.output_fps)
    rows = {}
    started = time.perf_counter()
    if arguments.batch:
        folders = S.ensure_casts(casts, None, lod)
    for cast in casts:
        begin = time.perf_counter()
        folder = folders[cast] if arguments.batch else S.ensure_casts([cast], None, lod)[cast]
        index = S._index(folder)
        rows[cast] = {"seconds": None if arguments.batch else round(time.perf_counter() - begin, 3), "frames": len(index["frames"]), "canonicalFrames": index["lod"]["canonicalFrames"], "maxPaths": index["logical"]["maxPaths"],
                      "bytes": sum(file.stat().st_size for file in folder.iterdir())}
    report["levels"][name] = {"revision": lod.revision, "totalSeconds": round(time.perf_counter() - started, 3), "frames": sum(row["frames"] for row in rows.values()), "bytes": sum(row["bytes"] for row in rows.values()), "casts": rows}
    print(name, report["levels"][name]["totalSeconds"], "s", report["levels"][name]["frames"], "frames", report["levels"][name]["bytes"], "bytes", flush=True)
Path(arguments.out).write_text(json.dumps(report, ensure_ascii=False, indent=1), encoding="utf-8")
