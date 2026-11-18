import json,sys,collections,urllib.parse
p=json.load(open(sys.argv[1]));target=sys.argv[2]
nodes={n['id']:n for n in p['nodes']};parent={}
for n in p['nodes']:
    for c in n.get('children',[]): parent[c]=n['id']
self_=collections.Counter()
for s,d in zip(p['samples'],p['timeDeltas']): self_[s]+=d
def nm(n):
    cf=n['callFrame'];return f"{cf['functionName'] or '(anon)'} {(urllib.parse.unquote(cf['url']).split('/')+['',''])[-2]}:{cf['lineNumber']}"
chains=collections.Counter()
for nid,t in self_.items():
    x=nid;path=[]
    while x is not None:
        path.append(nm(nodes[x]));x=parent.get(x)
    if any(target in q for q in path):
        i=[j for j,q in enumerate(path) if target in q][-1]
        chains[' < '.join(path[i:i+6])]+=t
for k,v in chains.most_common(8):print(round(v/1000),k[:400])
