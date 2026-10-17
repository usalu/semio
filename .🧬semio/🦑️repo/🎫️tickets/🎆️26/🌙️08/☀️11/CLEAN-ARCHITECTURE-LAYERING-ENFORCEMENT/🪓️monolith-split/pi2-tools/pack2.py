#!/usr/bin/env python3
"""🧳️ Packs test roots into companion bins with the real closure (source + embedded + concat includes; shared files counted once per bin). Roots that mention each other's module names stay together.
usage: pack2.py <closures.json> <tops.json> <skip comma list> <max bytes> <max lines> <max files> [<bin keys comma list> <out bins.json>]"""
import json, re, sys
from lib import *
cl = json.load(open(sys.argv[1])); tops = json.load(open(sys.argv[2]))
skip = set(sys.argv[3].split(",")) if sys.argv[3] else set()
MB, ML, MF = int(sys.argv[4]), int(sys.argv[5]), int(sys.argv[6])
keys = sys.argv[7].split(",") if len(sys.argv) > 7 else []
P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
cand = [t for t in tops if t["mod"] not in skip]
texts = [" ".join(open(PLUG + "/" + f, encoding="utf8").read() for f in t["files"]) for t in cand]
par = list(range(len(cand)))
def find(i):
    while par[i] != i:
        par[i] = par[par[i]]; i = par[i]
    return i
for i, t in enumerate(cand):
    for j, u in enumerate(cand):
        if i != j and len(u["mod"]) >= 8 and not u["mod"].startswith("<") and re.search(r"\b" + re.escape(u["mod"]) + r"\b", texts[i]):
            par[find(i)] = find(j)
groups = {}
for i, t in enumerate(cand):
    groups.setdefault(find(i), []).append(t)
items = []
for g in groups.values():
    f = {}
    for t in g:
        for r in cl[P + t["top"]]:
            f[r[0]] = (r[1], r[2])
    items.append(dict(mods=[t["mod"] for t in g], files=f, bytes=sum(v[0] for v in f.values()), lines=sum(v[1] for v in f.values())))
items.sort(key=lambda x: -x["bytes"])
bins = []
def cost(b, it):
    f = dict(b["files"]); f.update(it["files"])
    return sum(v[0] for v in f.values()), sum(v[1] for v in f.values()), len(f), f
for it in items:
    for b in bins:
        by, li, fi, f = cost(b, it)
        if by <= MB and li <= ML and fi <= MF:
            b["files"] = f; b["mods"] += it["mods"]; break
    else:
        bins.append(dict(files=dict(it["files"]), mods=list(it["mods"])))
out = {}
for i, b in enumerate(bins):
    k = keys[i] if i < len(keys) else f"?{i+1}"
    out[k] = b["mods"]
    print(k, sum(v[0] for v in b["files"].values()), sum(v[1] for v in b["files"].values()), len(b["files"]) + 1, b["mods"])
if len(sys.argv) > 8:
    json.dump(out, open(sys.argv[8], "w"), ensure_ascii=False, indent=1)
import os
if os.environ.get("PI2_MAP_OUT"):
    byname = {x["mod"]: x for x in tops}
    m = json.load(open(TOOLS + "/crate_map.json"))
    for k, mods in out.items():
        for md in mods:
            for f in byname[md]["files"]:
                m[f] = k
    json.dump(m, open(os.environ["PI2_MAP_OUT"], "w"), ensure_ascii=False, indent=1)
    json.dump(out, open(os.environ["PI2_MAP_OUT"] + ".bins.json", "w"), ensure_ascii=False, indent=1)
    print("written", os.environ["PI2_MAP_OUT"], len(m))

