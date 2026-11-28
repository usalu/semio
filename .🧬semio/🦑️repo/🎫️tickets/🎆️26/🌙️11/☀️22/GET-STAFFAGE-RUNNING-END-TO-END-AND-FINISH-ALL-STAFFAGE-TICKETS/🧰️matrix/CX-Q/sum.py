"""📊️ Triangle totals per scale and mode before and after, and the assets whose triangles grow with the scale: sum.py <tri-0.json> <tri-1.json>"""
import json, re, sys, collections
d = {}
for f in sys.argv[1:]:
    d.update(json.load(open(f)))
err = {k: v for k, v in d.items() if isinstance(v, str)}
d = {k: v for k, v in d.items() if not isinstance(v, str)}
tot = {w: collections.defaultdict(int) for w in ("before", "after")}
grow = {w: collections.defaultdict(list) for w in ("before", "after")}
for a, v in d.items():
    for w in ("before", "after"):
        rows = collections.defaultdict(dict)
        for sc, n in v[w].items():
            m = re.match(r"s(\d+)-(.*)", sc)
            s, mode = int(m[1]), m[2]
            tot[w][(s, mode)] += n
            rows[mode][s] = n
        for mode, r in rows.items():
            ss = sorted(r)
            if any(r[ss[i]] > r[ss[i - 1]] for i in range(1, len(ss))):
                grow[w][a].append(mode)
print("assets", len(d), "errors", len(err))
for k, v in err.items(): print(" ERR", k, v[:150])
modes = sorted({m for (_, m) in tot["before"]})
print("| scale | mode | before | after | change |\n|---|---|---|---|---|")
for s in (50, 100, 200, 500):
    for m in modes:
        b, a = tot["before"][(s, m)], tot["after"][(s, m)]
        print(f"| 1:{s} | {m} | {b:,} | {a:,} | {100*(a-b)/b:+.1f} % |")
for w in ("before", "after"):
    print(w, "assets with a mode whose triangles grow:", len(grow[w]))
print("still growing after:", sorted(grow["after"].items())[:40])
fine = [a for a, v in d.items() if v["before"] != v["after"] and any(v["after"][s] > v["before"][s] for s in v["after"] if s.startswith("s50-"))]
print("1:50 changed:", len([1 for a, v in d.items() if any(v["after"][s] != v["before"][s] for s in v["after"] if s.startswith("s50-"))]))
print("any scene with more triangles after:", [(a, s, v["before"][s], v["after"][s]) for a, v in d.items() for s in v["after"] if v["after"][s] > v["before"][s]][:20])
print("assets changed:", len([a for a, v in d.items() if v["before"] != v["after"]]))
