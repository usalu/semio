"""[gram3-g] attribute self time of native frames (by name) to the nearest JS caller (file:line) in a .cpuprofile. usage: prof_callers.py <profile> <name>[,<name>...] [top]"""
import json, sys, collections, resource
prof = json.load(open(sys.argv[1]))
names = set(sys.argv[2].split(","))
top = int(sys.argv[3]) if len(sys.argv) > 3 else 12
nodes = {n["id"]: n for n in prof["nodes"]}
parent = {}
for n in prof["nodes"]:
    for c in n.get("children", []):
        parent[c] = n["id"]
dt = prof.get("timeDeltas", [])
self_us = collections.Counter()
for sid, d in zip(prof["samples"], dt):
    self_us[sid] += d
def label(n):
    cf = n["callFrame"]
    u = cf["url"].split("🏗️build/")[-1].split("TO-END-AGAIN/")[-1]
    return f'{cf["functionName"] or "(anon)"} {u[-70:]}:{cf["lineNumber"]+1}'
out = collections.defaultdict(collections.Counter)
for nid, us in self_us.items():
    n = nodes[nid]
    fn = n["callFrame"]["functionName"]
    if fn not in names: continue
    p = parent.get(nid)
    while p and nodes[p]["callFrame"]["url"] in ("", ) and p in parent: p = parent[p]
    chain = []
    q = nid; 
    # nearest JS frames: first two with a url
    k = 0
    q = parent.get(nid)
    while q and k < 2:
        if nodes[q]["callFrame"]["url"]:
            chain.append(label(nodes[q])); k += 1
        q = parent.get(q)
    out[fn][" <- ".join(chain)] += us
for fn, c in out.items():
    tot = sum(c.values()) / 1e6
    print(f"== {fn} total self {tot:.1f}s")
    for k, v in c.most_common(top):
        print(f"  {v/1e6:7.2f}s {k}")
print("rss MB", resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // 1048576)
