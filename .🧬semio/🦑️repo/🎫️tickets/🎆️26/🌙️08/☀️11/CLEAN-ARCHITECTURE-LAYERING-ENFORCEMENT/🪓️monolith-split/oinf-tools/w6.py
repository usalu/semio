import re, collections, sys
sys.path.insert(0,'.')
from graph2 import *
from wmap import *
tree, mods = load_mods()
nodes = build(mods)
st = [n for n in nodes if n.kind=='struct' and n.name=='World3dState'][0]
body = mods[st.mod].items[st.item_ref[1]].text
fields = re.findall(r'^    (?:pub\s+)?([a-z_][a-z0-9_]*)\s*:', body, re.M)
missing = [f for f in fields if f not in F]; extra = [f for f in F if f not in fields]
print('missing', missing, 'extra', extra)
fs = set(F)
takers = [n for n in nodes if n.mod.startswith('🌍️world') and ('World3dState' in n.plain or n.parent=='World3dState') and n.kind in ('fn','method')]
dist = collections.Counter(); wide = []
for n in takers:
    comps = set(F[f] for f in n.fields & fs)
    dist[len(comps)] += 1
    if len(comps) >= 6: wide.append((len(comps), n))
print(sorted(dist.items()))
for k, n in sorted(wide, key=lambda x:-x[0])[:15]: print(k, n.mod.split('/')[-2], (n.parent+'::' if n.parent else '')+n.name, n.lines)
pair = collections.Counter()
for n in takers:
    comps = sorted(set(F[f] for f in n.fields & fs))
    for i in range(len(comps)):
        for j in range(i+1, len(comps)): pair[(comps[i], comps[j])] += 1
print(pair.most_common(20))
