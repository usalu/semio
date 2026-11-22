import json, sys, collections
data = json.load(open(sys.argv[1]))
events = data["traceEvents"] if isinstance(data, dict) else data
nodes = {}; samples = collections.defaultdict(list); deltas = collections.defaultdict(list)
for e in events:
    if e.get("name") == "ProfileChunk":
        a = e["args"]["data"]; key = (e["pid"], e["id"] if "id" in e else e.get("tid"))
        for n in a.get("cpuProfile", {}).get("nodes", []):
            nodes[(key, n["id"])] = n
        samples[key] += a.get("cpuProfile", {}).get("samples", [])
        deltas[key] += a.get("timeDeltas", [])
tot = collections.Counter(); per = collections.Counter()
for key, s in samples.items():
    per[key] = len(s)
    for sid, dt in zip(s, deltas[key]):
        n = nodes.get((key, sid))
        if n: tot[(key, n["callFrame"]["functionName"], n["callFrame"].get("url","")[-40:], n["callFrame"].get("lineNumber"), n["callFrame"].get("columnNumber"))] += dt
print("threads", dict(per))
for (k, f, u, l, c), v in tot.most_common(25): print(round(v/1000), "ms", k[1], f, u, l, c)
