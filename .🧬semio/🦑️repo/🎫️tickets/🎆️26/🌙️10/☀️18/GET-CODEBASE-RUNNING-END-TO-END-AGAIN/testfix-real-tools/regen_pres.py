#!/usr/bin/env python3
from uregen import *
d = load("📋️preserved.json")
paths = [d["original"]] + sorted({r["source"] for r in d["laws"] if r["source"] != d["original"]})
res = fnlist([{"id": str(i), "path": p, "mode": "tree" if p == d["original"] else "file"} for i, p in enumerate(paths)])
actual = {}
total = 0
for i, p in enumerate(paths):
    total += res[str(i)]["assertions"]
    for f in res[str(i)]["fns"]:
        if f["isLaw"]: actual[f["name"]] = (p, f)
declared = {r["name"]: r for r in d["laws"]}
changed = new = 0
for name, (p, f) in actual.items():
    r = declared.get(name)
    if r is None:
        d["laws"].append({"name": name, "source": p, "sha256": f["sha256"], "assertions": f["assertions"]}); new += 1; print("NEW", name, f["assertions"]); continue
    if r["source"] != p: raise SystemExit("MOVED " + name)
    if f["assertions"] < r["assertions"]: raise SystemExit("WEAKENED " + name)
    if f["sha256"] != r["sha256"]: changed += 1
    r["sha256"] = f["sha256"]; r["assertions"] = f["assertions"]
for name in declared:
    if name not in actual: raise SystemExit("REMOVED " + name)
d["addedAssertions"] = total - d["originalAssertions"]
save("📋️preserved.json", d)
print("rehashed", changed, "new", new, "addedAssertions", d["addedAssertions"])
