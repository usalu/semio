import sys,subprocess
units=subprocess.run(["node","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/units.cjs",sys.argv[1]],capture_output=True,text=True).stdout
rows={}
for l in units.split("\n"):
    p=l.split("\t")
    if len(p)<5: continue
    i=int(p[0]); a,b=map(int,p[1].split("-")); kind=p[2]; name=p[3].replace("export ",""); edges=[int(x) for x in p[4].replace("-> ","").split(",") if x.strip()]
    rows[i]=(a,b,kind,name,edges)
g={i:[e for e in r[4] if e in rows] for i,r in rows.items() if r[2]!="ImportDeclaration"}
sys.setrecursionlimit(100000)
index=0;stack=[];on=set();ids={};low={};sccs=[]
def sc(v):
    global index
    ids[v]=low[v]=index;index+=1;stack.append(v);on.add(v)
    for w in g[v]:
        if w not in g: continue
        if w not in ids: sc(w);low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],ids[w])
    if low[v]==ids[v]:
        comp=[]
        while True:
            w=stack.pop();on.discard(w);comp.append(w)
            if w==v:break
        sccs.append(comp)
for v in g:
    if v not in ids: sc(v)
for c in sccs:
    if len(c)>1:
        print("SCC size",len(c),"lines",sum(rows[x][1]-rows[x][0]+1 for x in c),[rows[x][3] for x in sorted(c)][:40])
print("total statements",len(g))
