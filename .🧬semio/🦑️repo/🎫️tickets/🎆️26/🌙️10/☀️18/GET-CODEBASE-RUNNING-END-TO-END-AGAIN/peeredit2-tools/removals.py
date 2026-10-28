#!/usr/bin/env python3
"""🧮 Compares the line removals (base → new) of every file of two cuts of one staged batch: a re-cut onto a moved base must remove exactly the lines the earlier cut removed (peer lines must survive). usage: python3 -I removals.py <old-stage-dir> <new-stage-dir>"""
import difflib, json, os, sys
def removed(stage, row):
    if row["base"] is None: return []
    base = open(os.path.join(stage, "base", row["file"]), encoding="utf-8").read().splitlines()
    new = open(os.path.join(stage, "files", row["file"]), encoding="utf-8").read().splitlines()
    return [line[1:] for line in difflib.unified_diff(base, new, lineterm="", n=0) if line.startswith("-") and not line.startswith("---")]
old, new = sys.argv[1], sys.argv[2]
mo, mn = json.load(open(os.path.join(old, "manifest.json"))), json.load(open(os.path.join(new, "manifest.json")))
bad = 0
for rel, row in mn.items():
    a, b = removed(old, mo[rel]) if rel in mo else None, removed(new, row)
    if a is None: print("NEW-IN-CUT", rel[-70:]); continue
    if sorted(a) != sorted(b):
        bad += 1; print("REMOVALS DIFFER", rel[-70:]); [print("  only-new -", x[:160]) for x in sorted(set(b) - set(a))]; [print("  only-old -", x[:160]) for x in sorted(set(a) - set(b))]
print("files", len(mn), "removal sets differ in", bad)
