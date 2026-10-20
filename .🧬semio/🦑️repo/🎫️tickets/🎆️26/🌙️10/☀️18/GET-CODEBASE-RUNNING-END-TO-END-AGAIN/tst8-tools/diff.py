#!/usr/bin/env python3
import json, sys, glob, collections
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/tst8/"
old = {r["file"]: r for r in json.load(open(C + "/🗑️generated/tst7/tst7-census-full.json", encoding="utf-8"))["suites"]}
new = {}
for n in sys.argv[1:] or ["results.jsonl"]:
    for l in open(G + n, encoding="utf-8"):
        r = json.loads(l); new[r["file"]] = r
reg, fix, still, added = [], [], [], []
for f, r in new.items():
    o = old.get(f)
    if o is None: added.append((f, r["rc"])); continue
    if o["rc"] == 0 and r["rc"] != 0: reg.append(f)
    elif o["rc"] != 0 and r["rc"] == 0: fix.append(f)
    elif r["rc"] != 0: still.append(f)
print("done", len(new), "green", sum(r["rc"] == 0 for r in new.values()), "regress", len(reg), "fixed", len(fix), "still red", len(still), "new suites", len(added), "new red", sum(1 for a in added if a[1]))
json.dump(dict(regress=reg, fixed=fix, still=still, added=added), open(G + "diff.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for f in reg: print("REG", f, (new[f].get("errs") or new[f].get("fails") or [""])[0][:200])
