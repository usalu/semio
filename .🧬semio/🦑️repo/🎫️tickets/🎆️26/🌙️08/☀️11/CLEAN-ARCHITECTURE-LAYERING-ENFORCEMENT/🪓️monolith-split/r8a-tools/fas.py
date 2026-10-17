import json,sys,collections
# Eades-Lin-Smyth feedback arc set heuristic on /tmp/r8a/filegraph.json (edges: src depends on dst). Prints back edges.
g=json.load(open('/tmp/r8a/filegraph.json'))
nodes=list(g.keys())
out={n:set(g[n].keys()) for n in nodes}
inn={n:set() for n in nodes}
for a in nodes:
    for b in out[a]: inn[b].add(a)
# order: dependencies first (sinks first). ELS builds order where edges go left->right; we want edges a->b (a depends on b) to go right->left.
# so run ELS on reversed graph.
R_out={n:set(inn[n]) for n in nodes}; R_in={n:set(out[n]) for n in nodes}
alive=set(nodes); s1=[];s2=[]
def deg(n,which): return len([x for x in which[n] if x in alive])
while alive:
    changed=True
    while changed:
        changed=False
        for n in list(alive):
            if deg(n,R_out)==0: s2.insert(0,n);alive.discard(n);changed=True
        for n in list(alive):
            if n in alive and deg(n,R_in)==0: s1.append(n);alive.discard(n);changed=True
    if alive:
        n=max(alive,key=lambda x:deg(x,R_out)-deg(x,R_in)); s1.append(n);alive.discard(n)
order=s1+s2   # edges in reversed graph go left->right: i.e. dependents appear left, dependencies right
pos={n:i for i,n in enumerate(order)}
# in R graph edge u->v means v depends on u; want pos[u]<pos[v] ... compute back edges w.r.t. original: a depends on b, ideally pos[a] < pos[b]
back=[(a,b) for a in nodes for b in out[a] if pos[a]<pos[b]]
print('nodes',len(nodes),'edges',sum(len(v) for v in out.values()),'back edges',len(back))
agg=collections.defaultdict(list)
for a,b in back: agg[b].append((a,sorted(g[a][b])))
for b,l in sorted(agg.items(),key=lambda kv:-len(kv[1])):
    print(f'DEST {b[:70]} <- {len(l)} users')
    for a,ns in l[:6]: print('     ',a[:60],ns[:5])
json.dump(order,open('/tmp/r8a/order.json','w'),ensure_ascii=False)
