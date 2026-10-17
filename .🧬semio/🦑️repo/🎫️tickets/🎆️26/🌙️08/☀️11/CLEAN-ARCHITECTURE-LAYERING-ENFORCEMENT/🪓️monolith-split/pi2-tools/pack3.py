#!/usr/bin/env python3
"""🧳️ Packs the test roots of a refresh tag into companion bins with the exact closure (source + embedded; shared files counted once per bin).
usage: pack3.py <tag> <max bytes> <max lines> <max files> <stay comma list> <out bins.json>"""
import json, re, sys, os
from lib import *
tag, MB, ML, MF = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
stay = set(sys.argv[5].split(",")) if sys.argv[5] else set()
out_path = sys.argv[6]
cl = json.load(open(GEN + f"/pi2w7-{tag}-top-closures.json")); tops = json.load(open(GEN + f"/pi2w7-{tag}-tops.json"))
P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
by = {t["mod"]: t for t in tops}
# fixed placements (providers first, lowest numbers) and unions (co-location)
FIX = json.loads(os.environ.get("PACK_FIX", "{}"))      # bin index (1-based) -> [mods]
UNION = json.loads(os.environ.get("PACK_UNION", "[]"))  # list of [mods...]
NOT_IN = json.loads(os.environ.get("PACK_NOT_IN", "{}")) # mod -> [bin indexes it must avoid]
NO_AUTO = set(json.loads(os.environ.get("PACK_NO_AUTO", "[]")))  # support mods consumers mention but must not be co-located with
cand = [t for t in tops if t["mod"] not in stay]
par = {t["mod"]: t["mod"] for t in cand}
def find(a):
    while par[a] != a:
        par[a] = par[par[a]]; a = par[a]
    return a
texts = {t["mod"]: " ".join(open(PLUG + "/" + f, encoding="utf8").read() for f in t["files"]) for t in cand}
for i in cand:
    for j in cand:
        if i is not j and j["mod"] not in NO_AUTO and len(j["mod"]) >= 8 and not j["mod"].startswith("<") and re.search(r"\b" + re.escape(j["mod"]) + r"\b", texts[i["mod"]]):
            par[find(i["mod"])] = find(j["mod"])
for g in UNION:
    for m in g[1:]:
        par[find(m)] = find(g[0])
groups = {}
for t in cand:
    groups.setdefault(find(t["mod"]), []).append(t["mod"])
def files_of(mods):
    f = {}
    for m in mods:
        for r in cl[P + by[m]["top"]]:
            f[r[0]] = (r[1], r[2])
    return f
items = [dict(mods=g, files=files_of(g)) for g in groups.values()]
for it in items:
    it["bytes"] = sum(v[0] for v in it["files"].values())
items.sort(key=lambda x: -x["bytes"])
bins = []
def fits(files):
    return sum(v[0] for v in files.values()) <= MB and sum(v[1] for v in files.values()) <= ML and len(files) <= MF
placed = set()
for k, mods in sorted(FIX.items(), key=lambda kv: int(kv[0])):
    k = int(k)
    while len(bins) < k:
        bins.append(dict(files={}, mods=[]))
    for m in mods:
        for it in items:
            if m in it["mods"] and id(it) not in placed:
                f = dict(bins[k - 1]["files"]); f.update(it["files"])
                assert fits(f), (k, m, sum(v[0] for v in f.values()), len(f))
                bins[k - 1]["files"] = f; bins[k - 1]["mods"] += it["mods"]; placed.add(id(it))
for it in items:
    if id(it) in placed:
        continue
    avoid = set()
    for m in it["mods"]:
        avoid |= set(NOT_IN.get(m, []))
    for bi, b in enumerate(bins):
        if bi + 1 in avoid:
            continue
        f = dict(b["files"]); f.update(it["files"])
        if fits(f):
            b["files"] = f; b["mods"] += it["mods"]; break
    else:
        bins.append(dict(files=dict(it["files"]), mods=list(it["mods"])))
res = {}
for i, b in enumerate(bins, 1):
    res[i] = b["mods"]
    print(i, sum(v[0] for v in b["files"].values()), sum(v[1] for v in b["files"].values()), len(b["files"]) + 1, b["mods"])
json.dump(res, open(out_path, "w"), ensure_ascii=False, indent=1)
mp = os.environ.get("PI2_MAP_OUT")
if mp:
    m = json.load(open(TOOLS + "/crate_map.json"))
    named = {}
    for i, mods in res.items():
        key = f"tests-{int(i):02d}"
        named[key] = mods
        for md in mods:
            for f in by[md]["files"]:
                m[f] = key
    json.dump(m, open(mp, "w"), ensure_ascii=False, indent=1)
    json.dump(named, open(mp + ".bins.json", "w"), ensure_ascii=False, indent=1)
    print("map written", mp, len(m))
