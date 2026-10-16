#!/usr/bin/env python3
# 📋️ Homes residue units that crates outside the Shell use (`shell::NAME`): plan_extra.py <out.json> NAME...
import sys, json, os, collections
HERE = os.path.dirname(os.path.abspath(__file__))
from units import *
out_path, names = sys.argv[1], sys.argv[2:]
ms, its, units, edges, defs, loose = build()
home = {u["id"]: u["crate"] for u in units}
rev = collections.defaultdict(set)
for u, vs in edges.items():
    for v in vs: rev[v].add(u)
def crate_at(rk):
    for c in SHELL_ORDER:
        if RANK[c] == rk: return c
def lo(uid, seen=()):
    best = -1
    for v in edges[uid]:
        if v in seen: continue
        if home[v] is not None: best = max(best, RANK[home[v]])
        else: best = max(best, lo(v, seen + (uid,)))
    return best
chosen = []
def place(uid, floor=-1):
    if home[uid] is not None: return
    rk = max(lo(uid), floor)
    if rk < 0: rk = RANK["shell-async"]
    home[uid] = crate_at(rk)
    chosen.append(uid)
    for v in edges[uid]:
        if home[v] is None: place(v)
for n in names:
    cands = [u["id"] for u in units if u["mk"] == "chunk" and any(its[m]["name"] == n for m in u["members"])]
    assert cands, n
    for uid in cands: place(uid)
out = []
for uid in chosen:
    u = units[uid]
    for m in u["members"]:
        it = its[m]
        text = open(it["file"], encoding="utf8").read().split("\n")
        seg = text[it["start"] - 1:it["end"]]
        head = next(l for l in seg if not l.lstrip().startswith(("///", "#[", "//"))).strip()
        out.append(dict(src=it["file"], src_crate=None, src_slug=it["slug"], kind=it["kind"], name=it["name"], head=head, dest=home[uid], slug=it["slug"], unit=uid))
json.dump(out, open(out_path, "w"), ensure_ascii=False, indent=1)
print(len(out), collections.Counter(o["dest"] for o in out))
for o in out: print("  ", o["dest"], o["name"][:50], o["slug"][:18])
