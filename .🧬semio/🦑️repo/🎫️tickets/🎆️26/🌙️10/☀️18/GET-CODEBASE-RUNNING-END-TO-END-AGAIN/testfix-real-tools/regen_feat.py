#!/usr/bin/env python3
from uregen import *
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
d = load("📋️feature-owner.json")
start = PL + "🧪️testing/🧪️artifact-app-laws/🦀️.rs"
files = [PL + "🧪️testing/🦀️.rs"] + closure(start)
res = fnlist([{"id": str(i), "path": p, "mode": "file"} for i, p in enumerate(files)])
new = []
for i, p in enumerate(files): new += [{"name": f["name"], "sha256": f["sha256"]} for f in res[str(i)]["fns"]]
old = d["remainingFunctions"]
on = {r["name"] for r in old}; nn = {r["name"] for r in new}
print("remaining: old", len(old), "new", len(new), "gained", sorted(nn - on)[:40], "lost", sorted(on - nn))
osha = {r["name"]: r["sha256"] for r in old}
print("rehashed", sum(1 for r in new if r["name"] in osha and osha[r["name"]] != r["sha256"]))
d["remainingFunctions"] = new
save("📋️feature-owner.json", d)
