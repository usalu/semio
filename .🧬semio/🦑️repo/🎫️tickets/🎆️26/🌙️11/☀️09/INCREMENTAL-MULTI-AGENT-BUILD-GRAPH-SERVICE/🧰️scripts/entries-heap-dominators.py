import json,sys,collections
d=json.load(open(sys.argv[1],encoding="utf8"))
meta=d["snapshot"]["meta"];nf=meta["node_fields"];ef=meta["edge_fields"];NF=len(nf);EF=len(ef)
nodes=d["nodes"];edges=d["edges"];strings=d["strings"]
ni={n:i for i,n in enumerate(nf)};ei={n:i for i,n in enumerate(ef)}
types=meta["node_types"][0];etypes=meta["edge_types"][0]
N=len(nodes)//NF
first=[0]*(N+1);c=0
for n in range(N):
    first[n]=c;c+=nodes[n*NF+ni["edge_count"]]
first[N]=c
selfs=[nodes[n*NF+ni["self_size"]] for n in range(N)]
def label(n):
    t=types[nodes[n*NF+ni["type"]]];nm=strings[nodes[n*NF+ni["name"]]]
    return t+":"+(nm[:60] if t!="string" else "(string)")
succ=[[] for _ in range(N)]
ename={}
for u in range(N):
    for k in range(first[u],first[u+1]):
        e=k*EF;t=etypes[edges[e+ei["type"]]]
        if t=="weak":continue
        to=edges[e+ei["to_node"]]//NF
        succ[u].append(to)
        if t in("property","internal","context","shortcut","variable"):
            ename[(u,to)]=strings[edges[e+ei["name_or_index"]]] if isinstance(edges[e+ei["name_or_index"]],int) and edges[e+ei["name_or_index"]]<len(strings) else str(edges[e+ei["name_or_index"]])
# iterative DFS postorder from 0
order=[];seen=[False]*N;stack=[(0,0)];seen[0]=True
while stack:
    u,i=stack.pop()
    if i<len(succ[u]):
        stack.append((u,i+1));v=succ[u][i]
        if not seen[v]:seen[v]=True;stack.append((v,0))
    else:order.append(u)
rpo=order[::-1];po={u:i for i,u in enumerate(order)}
preds=[[] for _ in range(N)]
for u in order:
    for v in succ[u]:preds[v].append(u)
idom=[-1]*N;idom[0]=0
def intersect(a,b):
    while a!=b:
        while po[a]<po[b]:a=idom[a]
        while po[b]<po[a]:b=idom[b]
    return a
changed=True
while changed:
    changed=False
    for u in rpo[1:]:
        new=-1
        for p in preds[u]:
            if idom[p]!=-1:new=p if new==-1 else intersect(p,new)
        if idom[u]!=new:idom[u]=new;changed=True
ret=selfs[:]
for u in order:
    if u!=0 and idom[u]>=0:ret[idom[u]]+=ret[u]
top=sorted(range(N),key=lambda u:-ret[u])[:60]
print("total",sum(selfs)/1048576,"MB")
for u in top:
    p=idom[u];print(round(ret[u]/1048576,1),"MB",label(u),"<- idom",label(p),"via",ename.get((p,u),"?"))
print("---- children of top dominators")
for u in top[:6]:
    print("##",round(ret[u]/1048576,1),"MB",label(u))
    kids=sorted([v for v in succ[u] if idom[v]==u],key=lambda v:-ret[v])[:8]
    for v in kids:
        print("   ",round(ret[v]/1048576,2),"MB",label(v),"via",ename.get((u,v),"?"))
        kk=sorted([w for w in succ[v] if idom[w]==v],key=lambda w:-ret[w])[:3]
        for w in kk:print("        ",round(ret[w]/1048576,2),"MB",label(w),"via",ename.get((v,w),"?"))
print("---- big map samples")
for u in top[:6]:
    if label(u).startswith("object:Map") and ret[u]>8*1048576:
        bf=[v for v in succ[u]]
        print("Map node",u,"succ",len(bf))
        for b in bf[:1]:
            print(" butterfly succ",len(succ[b]))
            import collections
            cc=collections.Counter(label(v) for v in succ[b][:20000]);print(cc.most_common(8))
            for v in succ[b][:6]:
                print("   entry",label(v),"succ",[label(w) for w in succ[v][:4]],[ (strings[nodes[w*NF+ni['name']]][:100] if types[nodes[w*NF+ni['type']]]=='string' else '') for w in succ[v][:3]])
