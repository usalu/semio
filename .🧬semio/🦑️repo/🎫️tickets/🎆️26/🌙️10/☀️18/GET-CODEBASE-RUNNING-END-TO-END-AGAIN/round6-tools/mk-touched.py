#!/usr/bin/env python3
"""Writes touched.json (step -> files outside this ticket's tool folders that the step changes, creates or removes) from the last full overlay simulation, and prints the cold-rebuild census per step.
usage: mk-touched.py [sim-dir]   default C/🗑️generated/round6-apply/sim-a"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
sim = sys.argv[1] if len(sys.argv) > 1 else C + "/🗑️generated/round6-apply/sim-a"
rows = json.load(open(sim + "/result.json", encoding="utf8"))["result"]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P = "🧰️framework/🔨️modules/🏃️process"
touched, census = {}, {}
for row in rows:
    files = sorted({f for f in row["changed"] + row.get("removed", []) if not (f == CREL or f.startswith(CREL + "/")) and not f.startswith(".git/") and "/__pycache__/" not in f})
    touched[row["step"]] = files
    census[row["step"]] = {
        "files": len(files),
        "build-library": sum(1 for f in files if f.startswith(L + "/🏗️build/")),
        "process-modules": sum(1 for f in files if f.startswith(P + "/")),
        "manifests": sum(1 for f in files if os.path.basename(f) in ("📋️project.json", "package.json")),
        "locks": sum(1 for f in files if f.endswith(("bun.lock", "Cargo.lock", "uv.lock", "go.work.sum")) or "lock" in os.path.basename(f).lower() and f.endswith(".json")),
        "tests": sum(1 for f in files if "/🧪️tests/" in f),
        "docs": sum(1 for f in files if f.endswith(".md")),
        "removed": len(row.get("removed", [])),
    }
json.dump(touched, open(HERE + "/touched.json", "w", encoding="utf8"), ensure_ascii=False, indent=0)
json.dump(census, open(sim + "/census.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
for step, c in census.items():
    print("%-28s files %5d  build-lib %4d  process %4d  manifests %4d  tests %4d  removed %4d" % (step, c["files"], c["build-library"], c["process-modules"], c["manifests"], c["tests"], c["removed"]))
