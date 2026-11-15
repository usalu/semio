import json, sys, collections, statistics
r = json.load(open(sys.argv[1], encoding="utf-8"))
S = ["50", "100", "200", "500"]
cats = collections.OrderedDict()
for x in sorted(r, key=lambda x: x["cat"]):
    cats.setdefault(x["cat"], []).append(x)
print("| category | assets | " + " | ".join(f"1:{s} authored / unavail." for s in S) + " | " + " | ".join(f"mean shapes 1:{s}" for s in S) + " | mean elev/front/plan (all scales) |")
print("|" + "---|" * (2 + 4 + 4 + 1))
tot = collections.Counter()
allmean = {s: [] for s in S}
for c, xs in cats.items():
    row = [c, str(len(xs))]
    for s in S:
        a = sum(1 for x in xs if x["scales"][s]["status"] == "authored")
        row.append(f"{a} / {len(xs)-a}")
        tot[("a", s)] += a; tot[("u", s)] += len(xs) - a
    means = []
    for s in S:
        v = [x["scales"][s]["total"] for x in xs if x["scales"][s]["status"] == "authored"]
        allmean[s] += v
        means.append(f"{statistics.mean(v):.1f}" if v else "-")
    row += means
    vps = {vp: [x["scales"][s]["vp"][vp] for x in xs for s in S if x["scales"][s]["status"] == "authored"] for vp in ["elevation", "front", "plan"]}
    row.append("/".join(f"{statistics.mean(vps[vp]):.1f}" for vp in ["elevation", "front", "plan"]))
    print("| " + " | ".join(row) + " |")
print("| ALL | " + str(len(r)) + " | " + " | ".join(f"{tot[('a',s)]} / {tot[('u',s)]}" for s in S) + " | " + " | ".join(f"{statistics.mean(allmean[s]):.1f}" for s in S) + " | |")
print()
print("Unavailable causes:", collections.Counter(x["scales"][s]["cause"] for x in r for s in S if x["scales"][s]["status"] == "unavailable"))
print("Unavailable by scale and cause:")
for s in S:
    print(" 1:" + s, dict(collections.Counter(x["scales"][s]["cause"] for x in r if x["scales"][s]["status"] == "unavailable")))
print("Alternatives kinds:", collections.Counter(a for x in r for s in S if x["scales"][s]["status"] == "unavailable" for a in x["scales"][s]["alternatives"]))
print("Assets with all 4 authored:", sum(1 for x in r if all(x["scales"][s]["status"] == "authored" for s in S)))
print("Assets with omitted combos:", collections.Counter(tuple(s for s in S if x["scales"][s]["status"] == "unavailable") for x in r).most_common())
print("Default scale:", collections.Counter(x["presentation_defaultScale"] for x in r))
print()
print("Per category omitted lists:")
for c, xs in cats.items():
    print(c, [(x["id"], [s for s in S if x["scales"][s]["status"]=="unavailable"]) for x in xs if any(x["scales"][s]["status"]=="unavailable" for s in S)])
