#!/usr/bin/env python3
from uregen import *
d = load("📋️feature-consumers.json")
deps = None; present = 0; variants = {}
for row in d["roots"]:
    p = O + "/" + row["path"]
    if not os.path.exists(p): print("MISSING lock", row["path"]); continue
    e = lock_packages(row["path"]).get(d["consumer"], [])
    row["present"] = bool(e)
    if e:
        v = tuple(e[0]["dependencies"]); variants.setdefault(v, []).append(row["path"])
print("present", sum(1 for r in d["roots"] if r["present"]), "variants", len(variants))
for v, ps in variants.items(): print(len(v), len(ps), ps[0][-60:])
if len(variants) == 1:
    d["dependencies"] = list(next(iter(variants)))
    d["absentRootCount"] = sum(1 for r in d["roots"] if not r["present"])
    save("📋️feature-consumers.json", d)
