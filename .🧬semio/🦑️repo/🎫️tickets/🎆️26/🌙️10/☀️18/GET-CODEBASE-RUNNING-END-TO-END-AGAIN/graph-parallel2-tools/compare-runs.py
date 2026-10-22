#!/usr/bin/env python3
"""🔬️ Compares the canonical project values two harness runs wrote (OUT.projects.json): prints how many owners are equal, equal after the closure tokens are normalised, and which owners differ. usage: compare-runs.py <a.projects.json> <b.projects.json>"""
import json, re, sys
a = json.load(open(sys.argv[1], encoding="utf-8")); b = json.load(open(sys.argv[2], encoding="utf-8"))
loose = lambda value: re.sub(r"SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[0-9a-f]{64}", "SEMIO_*", json.dumps(value, sort_keys=True))
names = sorted(set(a) | set(b))
equal = sum(1 for n in names if a.get(n) == b.get(n)); same = sum(1 for n in names if n in a and n in b and loose(a[n]) == loose(b[n]))
print(f"owners {len(names)} equal {equal} equal-after-token-normalisation {same}")
for n in names:
    if n not in a or n not in b or loose(a[n]) != loose(b[n]):
        print("DIFF", n, "only-a" if n not in b else "only-b" if n not in a else "")
