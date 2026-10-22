#!/usr/bin/env python3
# records the sha256 of every live file the staged set edits or replaces (run once at staging time; apply.py refuses replaced files that drifted since)
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
edits = load("edits").EDITS
targets = load("targets")
files = sorted({e["file"] for e in edits} | set(targets.REPLACE.values()))
base = {f: sha(read(os.path.join(ROOT, f))) for f in files}
json.dump(base, open(os.path.join(TOOLS, "base-hashes.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(base), "files stamped")
