"""🧾 Lossless check: trimmed non-blank line multiset of every touched Rust source before and after the carve."""
import os, collections, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
base = collections.Counter(); stage = collections.Counter(); touched = 0
for dp, dn, fn in os.walk(STAGE):
    for f in fn:
        rel = os.path.relpath(os.path.join(dp, f), STAGE)
        if f.endswith(".deleted"):
            r = rel[:-8]
            if r.endswith(".rs"):
                for l in open(BASE + r, encoding="utf8").read().split("\n"):
                    if l.strip(): base[l.strip()] += 1
            continue
        if not f.endswith(".rs"): continue
        touched += 1
        if os.path.exists(BASE + rel):
            for l in open(BASE + rel, encoding="utf8").read().split("\n"):
                if l.strip(): base[l.strip()] += 1
        for l in open(os.path.join(dp, f), encoding="utf8").read().split("\n"):
            if l.strip(): stage[l.strip()] += 1
ob = base - stage; os_ = stage - base
print("touched", touched, "base lines", sum(base.values()), "stage lines", sum(stage.values()), "only base", sum(ob.values()), "only stage", sum(os_.values()))
open(SP + "/lossless.base", "w").write("\n".join(f"{n}\t{l}" for l, n in sorted(ob.items())))
open(SP + "/lossless.stage", "w").write("\n".join(f"{n}\t{l}" for l, n in sorted(os_.items())))
