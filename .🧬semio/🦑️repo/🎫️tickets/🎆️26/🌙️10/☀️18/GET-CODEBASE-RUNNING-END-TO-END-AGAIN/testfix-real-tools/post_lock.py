#!/usr/bin/env python3
"""After the stale Cargo.lock files are regenerated (cargo offline): refreshes the lock-derived plugin-units data on the live checkout. Dry output by default; --write saves. TFR_ROOT=/Users/ueli/Documents/semio python3 post_lock.py [--write]"""
import sys
from uregen import *
import json, subprocess
write = "--write" in sys.argv
d = load("📋️feature-consumers.json")
variants = {}
for row in d["roots"]:
    e = lock_packages(row["path"]).get(d["consumer"], [])
    row["present"] = bool(e)
    if e: variants.setdefault(tuple(e[0]["dependencies"]), []).append(row["path"])
print("consumer present in", sum(1 for r in d["roots"] if r["present"]), "locks;", len(variants), "dependency variant(s)")
if len(variants) == 1:
    d["dependencies"] = list(next(iter(variants))); d["absentRootCount"] = sum(1 for r in d["roots"] if not r["present"])
    if write: save("📋️feature-consumers.json", d)
else: print("locks still disagree; regenerate the stale ones first:", [(len(v), ps[0]) for v, ps in variants.items()])
if write: subprocess.run([sys.executable, os.path.dirname(os.path.abspath(__file__)) + "/regen_pins.py"], check=True)
