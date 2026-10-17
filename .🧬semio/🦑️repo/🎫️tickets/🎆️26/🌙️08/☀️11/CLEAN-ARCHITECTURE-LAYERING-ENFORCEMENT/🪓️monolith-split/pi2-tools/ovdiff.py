#!/usr/bin/env python3
"""🔍️ Prints a unified diff of every file of an overlay against the live tree: ovdiff.py <overlay.json> [path-substring]"""
import json, sys, difflib, os
from lib import *
ov = json.load(open(sys.argv[1]))
flt = sys.argv[2] if len(sys.argv) > 2 else ""
stat = []
for relp, t in sorted(ov.items()):
    if flt and flt not in relp:
        continue
    f = REPO + "/" + relp
    old = open(f, encoding="utf8").read() if os.path.exists(f) else ""
    new = t or ""
    d = list(difflib.unified_diff(old.split("\n"), new.split("\n"), "a/" + relp.replace(PLUG_REL + "/", "P/"), "b/" + relp.replace(PLUG_REL + "/", "P/"), lineterm="", n=1))
    add = sum(1 for l in d if l.startswith("+") and not l.startswith("+++"))
    rem = sum(1 for l in d if l.startswith("-") and not l.startswith("---"))
    stat.append((relp.replace(PLUG_REL + "/", "P/"), add, rem, t is None))
    if flt:
        print("\n".join(d))
for s in stat:
    print("STAT", s)
