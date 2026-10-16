import sys, re, collections
from graph2 import *
tree, mods = load_mods()
nodes = build(mods)
# State struct fields
st = [n for n in nodes if n.kind=='struct' and n.name=='World3dState'][0]
m = mods[st.mod]
it = m.items[st.item_ref[1]]
body = it.text
fields = re.findall(r'^\s{4}(?:pub\s+)?([a-z_][a-z0-9_]*)\s*:', body, re.M)
print(len(fields), 'fields')
# every file text of world lane; count `.field` uses per file, and functions taking World3dState
fset = set(fields)
use = collections.defaultdict(collections.Counter)
takers = []
for n in nodes:
    if not n.mod.startswith('🌍️world'): continue
    if 'World3dState' in n.plain or n.parent=='World3dState':
        takers.append(n)
print('nodes mentioning World3dState:', len(takers), sum(n.lines for n in takers))
c = collections.Counter(n.kind for n in takers); print(c)
fieldcount = collections.Counter()
for n in takers:
    for f in n.fields & fset: fieldcount[f]+=1
print(fieldcount.most_common(60))
print('fields never used outside State impl:', [f for f in fields if fieldcount[f]==0])
