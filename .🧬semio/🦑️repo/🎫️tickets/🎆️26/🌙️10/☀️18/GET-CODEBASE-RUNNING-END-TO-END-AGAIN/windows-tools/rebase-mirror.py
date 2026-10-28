#!/usr/bin/env python3
"""🔄 Puts the live base of every file of a batch into a mirror (removing files the batch creates), so `stage.py apply --root <mirror>` starts from exactly what the gate will see: python3 -I rebase-mirror.py <batch> <mirror-root> [--live ROOT]."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
name, mirror = sys.argv[1], os.path.abspath(sys.argv[2])
live = os.path.abspath(sys.argv[sys.argv.index("--live") + 1]) if "--live" in sys.argv else "/Users/ueli/Documents/semio"
for rel, row in json.load(open(os.path.join(HERE, "staged", name, "manifest.json"), encoding="utf-8")).items():
    target = os.path.join(mirror, rel)
    if row["base"] is None:
        if os.path.exists(target): os.remove(target)
        continue
    os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copyfile(os.path.join(live, rel), target)
print("rebased", name, "in", mirror)
