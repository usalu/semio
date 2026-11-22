import json, sys, collections
data = json.load(open(sys.argv[1])); events = data["traceEvents"] if isinstance(data, dict) else data
nodes = {}; samples = collections.defaultdict(list); deltas = collections.defaultdict(list)
for e in events:
    if e.get("name") == "ProfileChunk":
        a = e["args"]["data"]; key = e["id"]
        for n in a.get("cpuProfile", {}).get("nodes", []): nodes[(key, n["id"])] = n
        samples[key] += a.get("cpuProfile", {}).get("samples", []); deltas[key] += a.get("timeDeltas", [])
parent = {}
for (k, i), n in nodes.items():
    for c in n.get("children", []): parent[(k, c)] = i
name = lambda n: n["callFrame"]["functionName"]
incl = collections.Counter(); selfc = collections.Counter(); depthmax = 0
thread = sys.argv[2]
for sid, dt in zip(samples[thread], deltas[thread]):
    cur = sid; seen = set()
    while cur is not None:
        n = nodes[(thread, cur)]; f = name(n)
        if f not in seen: incl[f] += dt; seen.add(f)
        cur = parent.get((thread, cur))
    selfc[name(nodes[(thread, sid)])] += dt
for f, v in incl.most_common(45): print(round(v/1000), "ms incl", f, "| self", round(selfc[f]/1000))
