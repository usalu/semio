import json, sys, collections
d = json.load(open(sys.argv[1]))
nodes = {n['id']: n for n in d['nodes']}
dt = d['timeDeltas']
samples = d['samples']
self_t = collections.Counter()
for s, t in zip(samples, dt):
    self_t[s] += t
parent = {}
for n in d['nodes']:
    for c in n.get('children', []):
        parent[c] = n['id']
byfn = collections.Counter()
incl = collections.Counter()
for nid, t in self_t.items():
    n = nodes[nid]['callFrame']
    byfn[(n['functionName'], n['url'].split('/')[-1][-40:], n['lineNumber'])] += t
    seen = set()
    cur = nid
    while cur is not None:
        c = nodes[cur]['callFrame']
        key = (c['functionName'], c['url'].split('/')[-1][-40:], c['lineNumber'])
        if key not in seen:
            incl[key] += t
            seen.add(key)
        cur = parent.get(cur)
total = sum(self_t.values())
print('total ms', total / 1000)
print('--- self')
for k, v in byfn.most_common(15):
    print(round(v / 1000), k)
print('--- inclusive')
for k, v in incl.most_common(25):
    print(round(v / 1000), k)
