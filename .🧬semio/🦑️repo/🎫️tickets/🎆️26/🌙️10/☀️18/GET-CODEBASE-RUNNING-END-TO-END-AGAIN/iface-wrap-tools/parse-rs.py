#!/usr/bin/env python3
"""Syntax check of every staged Rust file with rustfmt (children not followed): prints files that do not parse."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
bad = n = 0
for row in manifest["files"]:
    if not row["path"].endswith(".rs"):
        continue
    n += 1
    result = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(HERE, "staged", row["steps"][-1]["after"])], capture_output=True, text=True)
    if any(line.startswith("error") for line in result.stderr.splitlines()):
        bad += 1
        print("PARSE", row["path"][-80:], result.stderr.splitlines()[:2])
print("rust files", n, "parse errors", bad)
sys.exit(1 if bad else 0)
