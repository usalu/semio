#!/usr/bin/env python3
from uregen import *
d = load("📋️surface-carriers.json")
orig = d["original"]
parent = os.path.dirname(os.path.dirname(O + "/" + orig)); base = os.path.basename(os.path.dirname(orig))
fam = sorted(n for n in os.listdir(parent) if n == base or (n.startswith(base + "-part-") and not n.endswith("-x")))
rels = [os.path.relpath(parent + "/" + n + "/🦀️.rs", O) for n in fam]
res = fnlist([{"id": str(i), "path": r, "mode": "tree"} for i, r in enumerate(rels)] + [{"id": "t", "path": d["target"], "mode": "file"}])
new = []
for i in range(len(rels)): new += [{"name": f["name"], "sha256": f["sha256"], "assertions": f["assertions"], "law": f["isLaw"], "bytes": f["bytes"]} for f in res[str(i)]["fns"]]
key = lambda r: (r["name"], r["sha256"])
old = d["remaining"]
on = {r["name"] for r in old}; nn = {r["name"] for r in new}
print("remaining old", len(old), "new", len(new), "gained", len(nn - on), "lost", sorted(on - nn)[:10])
o = {r["name"]: r["sha256"] for r in old}
print("rehashed", sum(1 for r in new if r["name"] in o and o[r["name"]] != r["sha256"]))
d["remaining"] = sorted(new, key=lambda r: (r["name"], r["sha256"]))
# beforeAssertions = asserts(target) + asserts(joined family texts)
txt = "\n".join(readmod(r) for r in rels) if False else None
import subprocess
tot = res["t"]["assertions"] + sum(res[str(i)]["assertions"] for i in range(len(rels)))
print("beforeAssertions", d["beforeAssertions"], "->", tot)
d["beforeAssertions"] = tot
save("📋️surface-carriers.json", d)
