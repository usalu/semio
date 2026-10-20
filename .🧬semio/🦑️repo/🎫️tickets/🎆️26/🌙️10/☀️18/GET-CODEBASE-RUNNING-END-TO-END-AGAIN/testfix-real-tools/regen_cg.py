#!/usr/bin/env python3
from uregen import *
d = load("📋️child-group-owner.json")
res = fnlist([{"id": "mod", "path": d["target"], "mode": "file", "kind": "mod", "name": "child_complete_group_candidate_tests"}] +
             [{"id": "rem%d" % i, "path": r["path"], "mode": "file"} for i, r in enumerate(d["remaining"])])
m = res["mod"]
print("module sha", d["canonicalModuleSha256"][:12], "->", m["sha256"][:12])
d["canonicalModuleSha256"] = m["sha256"]
byname = {f["name"]: f for f in m["fns"]}
tot = 0
for law in d["laws"]:
    f = byname[law["name"]]
    if f["assertions"] < law["assertions"]: raise SystemExit("WEAKENED " + law["name"])
    if f["sha256"] != law["canonicalSha256"]: print("law changed", law["name"][:60], law["assertions"], "->", f["assertions"])
    law["canonicalSha256"] = f["sha256"]; law["assertions"] = f["assertions"]; tot += f["assertions"]
d["newLawAssertions"] = tot; d["oldLawAssertions"] = tot * 2
for i, r in enumerate(d["remaining"]):
    new = [{"name": f["name"], "sha256": f["sha256"]} for f in res["rem%d" % i]["fns"]]
    if new != r["functions"]: print("remaining changed", r["path"][-60:], len(r["functions"]), "->", len(new))
    r["functions"] = new; r["sha256"] = res["rem%d" % i]["sha256"]
paths = [p for p in closure(d["target"]) if p != d["target"]]
old = {c["path"]: c for c in d["closure"]}
for p in paths:
    if p not in old: print("closure gained", p[-70:])
for p in old:
    if p not in paths: print("closure lost", p[-70:])
cl = fnlist([{"id": "c%d" % i, "path": p, "mode": "file"} for i, p in enumerate(paths)])
def nonblank(rel): return sum(1 for l in open(O + "/" + rel, encoding="utf8").read().split("\n") if l.strip())
d["closure"] = [{"path": p, "bytes": cl["c%d" % i]["bytes"], "nonblank": nonblank(p), "sha256": cl["c%d" % i]["sha256"]} for i, p in enumerate(paths)]
save("📋️child-group-owner.json", d)
print("ok")
