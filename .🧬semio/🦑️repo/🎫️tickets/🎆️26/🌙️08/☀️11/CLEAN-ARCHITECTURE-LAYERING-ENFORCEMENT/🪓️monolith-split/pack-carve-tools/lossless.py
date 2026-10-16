"""🧾 Lossless check: line multiset (trimmed, non-blank) of every touched pack source before and after the carve."""
import os, collections, re, sys
from lib import *
base_lines = collections.Counter(); stage_lines = collections.Counter()
touched = []
for dp, dn, fn in os.walk(STAGE):
    for f in fn:
        if not f.endswith(".rs"): continue
        rel = os.path.relpath(os.path.join(dp, f), STAGE)
        touched.append(rel)
        if os.path.exists(BASE + rel):
            for l in open(BASE + rel, encoding="utf8").read().split("\n"):
                if l.strip(): base_lines[l.strip()] += 1
        for l in open(os.path.join(dp, f), encoding="utf8").read().split("\n"):
            if l.strip(): stage_lines[l.strip()] += 1
only_base = base_lines - stage_lines
only_stage = stage_lines - base_lines
print("touched files:", len(touched), "base lines:", sum(base_lines.values()), "stage lines:", sum(stage_lines.values()))
print("only in base:", sum(only_base.values()), "only in stage:", sum(only_stage.values()))
if len(sys.argv) > 1:
    out = sys.argv[1]
    open(out + ".base", "w").write("\n".join(f"{n}\t{l}" for l, n in sorted(only_base.items())))
    open(out + ".stage", "w").write("\n".join(f"{n}\t{l}" for l, n in sorted(only_stage.items())))
