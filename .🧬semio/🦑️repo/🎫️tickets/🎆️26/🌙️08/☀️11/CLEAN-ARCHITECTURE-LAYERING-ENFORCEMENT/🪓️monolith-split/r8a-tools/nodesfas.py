import json,sys,subprocess
nodes=json.load(open(sys.argv[1])); names=list(nodes)
def find(t):
    if t in nodes: return t
    c=[n for n in names if n.endswith(t) or t.endswith(n)]
    return c[0] if c else None
g={n:{} for n in names}
for n,v in nodes.items():
    for t,w in v['out']:
        m=find(t)
        if m and m!=n: g[n].setdefault(m,[]).append(w.split(' @ ')[0].split('::')[-1] if w and w!='?' else '?')
json.dump(g,open('/tmp/r8a/filegraph.json','w'),ensure_ascii=False)
