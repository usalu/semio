#!/usr/bin/env python3
# 🔬️ Cross-reference analysis of the Shell tree: for every staging module the names defined elsewhere in the Shell namespace it mentions, grouped by provider crate / residue chunk.
import json, sys, collections
from model import *
ms = parent_mounts()
defs = {}
for m in ms:
    if m["kind"] in ("staging", "chunk"):
        m["defs"] = defined(m["file"])
        for it in m["defs"]:
            defs.setdefault(it["name"], []).append((m, it))
ptext = read(PARENT)
pdefs = {it["name"]: it for it in __import__("items").table(ptext) if it["kind"] not in ("impl", "use")}
print("names", len(defs), "parent defs", list(pdefs))
dups = {n: [(m["crate"] or m["slug"], m["slug"]) for m, it in v] for n, v in defs.items() if len(v) > 1 and n not in ("tests",)}
print("duplicate defined names:", len(dups))
for n, v in list(dups.items())[:40]:
    print("  ", n, v)
out = collections.defaultdict(lambda: collections.defaultdict(set))
for m in ms:
    if m["kind"] != "staging":
        continue
    toks = tokens(read(m["file"]))
    own = {it["name"] for it in m["defs"]}
    for t in toks:
        if t in own or t not in defs:
            continue
        for pm, it in defs[t]:
            if pm is m:
                continue
            key = pm["crate"] if pm["kind"] == "staging" else "RESIDUE:" + pm["slug"]
            out[m["crate"]][key].add(t)
res = {}
for c, d in out.items():
    for k, names in d.items():
        res.setdefault(c, {})[k] = sorted(names)
json.dump(res, open(GEN + "/ri2-xref1.json", "w"), ensure_ascii=False, indent=1)
for c in SHELL_ORDER:
    if c not in out:
        continue
    print("==", c, RANK[c])
    for k, names in sorted(out[c].items(), key=lambda kv: (RANK.get(kv[0], 999), kv[0])):
        flag = ""
        if k in RANK and RANK[k] > RANK[c]:
            flag = "  UPWARD"
        if k == c:
            continue
        print(f"   -> {k} ({len(names)}){flag}", sorted(names)[:8])
