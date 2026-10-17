#!/usr/bin/env python3
"""🗺️ Assigns test roots to companion bins (first-fit decreasing, 3,300 lines / 215,000 bytes / 26 files) and writes file-level `crate_map` entries into the map file named by env PI2_MAP_OUT.
usage: mapbins.py <tops.json> <bin keys comma list> <skip mods comma list> [--write]"""
import json, os, re, sys
from lib import *
tops = json.load(open(sys.argv[1]))
keys = sys.argv[2].split(",")
skip = set(sys.argv[3].split(",")) if len(sys.argv) > 3 and sys.argv[3] else set()
write = "--write" in sys.argv
LINES = {}
for line in open(GEN + "/pi2w7-closure-test.tsv"):
    if not line.startswith("TOTAL"):
        b, l, k, p = line.rstrip("\n").split("\t")
        LINES[p.replace("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", "")] = (int(b), int(l))
for t in tops:
    t["lines"] = sum(LINES.get(f, (0, 0))[1] for f in t["files"])
    t["bytes"] = sum(LINES.get(f, (0, 0))[0] for f in t["files"]) or t["bytes"]
cand = [t for t in tops if t["mod"] not in skip]
parent_ = list(range(len(cand)))
def find(i):
    while parent_[i] != i:
        parent_[i] = parent_[parent_[i]]
        i = parent_[i]
    return i
texts = [" ".join(open(PLUG + "/" + f, encoding="utf8").read() for f in t["files"]) for t in cand]
for i, t in enumerate(cand):
    for j, u in enumerate(cand):
        if i != j and len(u["mod"]) >= 8 and not u["mod"].startswith("<") and re.search(r"\b" + re.escape(u["mod"]) + r"\b", texts[i]):
            parent_[find(i)] = find(j)
groups = {}
for i, t in enumerate(cand):
    groups.setdefault(find(i), []).append(t)
items = sorted((dict(mod="+".join(x["mod"] for x in g), files=[f for x in g for f in x["files"]], bytes=sum(x["bytes"] for x in g), lines=sum(x["lines"] for x in g), members=g) for g in groups.values()), key=lambda t: -t["bytes"])
bins = {k: dict(bytes=0, lines=0, files=0, tops=[]) for k in keys}
unplaced = []
for t in items:
    for k in keys:
        b = bins[k]
        if b["bytes"] + t["bytes"] <= 215000 and b["lines"] + t["lines"] <= 3300 and b["files"] + len(t["files"]) <= 26:
            b["bytes"] += t["bytes"]; b["lines"] += t["lines"]; b["files"] += len(t["files"]); b["tops"].append(t)
            break
    else:
        unplaced.append(t)
for k, b in bins.items():
    print(k, b["bytes"], b["lines"], b["files"], [t["mod"] for t in b["tops"]])
print("unplaced", [(t["mod"], t["bytes"], t["lines"]) for t in unplaced])
if write:
    out = os.environ["PI2_MAP_OUT"]
    m = json.load(open(TOOLS + "/crate_map.json"))
    for k, b in bins.items():
        for t in b["tops"]:
            for f in t["files"]:
                m[f] = k
    json.dump(m, open(out, "w"), ensure_ascii=False, indent=1)
    json.dump({k: [x["mod"] for t in b["tops"] for x in t["members"]] for k, b in bins.items()}, open(out + ".bins.json", "w"), ensure_ascii=False, indent=1)
    print("written", out, len(m))
