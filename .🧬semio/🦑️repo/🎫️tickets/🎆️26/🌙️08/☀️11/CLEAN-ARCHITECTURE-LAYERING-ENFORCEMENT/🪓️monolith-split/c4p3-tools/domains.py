# usage: domains.py <file> <domains.json>  where domains.json = [[label, [[a,b],...]], ...] (statement start-line ranges); prints cross-domain edges and cycles
import sys,subprocess,json,collections
U="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/units.cjs"
units=subprocess.run(["node",U,sys.argv[1]],capture_output=True,text=True).stdout
rows={}
for l in units.split("\n"):
    p=l.split("\t")
    if len(p)<5: continue
    i=int(p[0]); a,b=map(int,p[1].split("-")); kind=p[2]; name=p[3].replace("export ",""); edges=[int(x) for x in p[4].replace("-> ","").split(",") if x.strip()]
    rows[i]=(a,b,kind,name,edges)
doms=json.load(open(sys.argv[2],encoding="utf8"))
def lab(a):
    for name,rs in doms:
        for x,y in rs:
            if x<=a<=y: return name
    return "parent"
size=collections.Counter(); 
L={}
for i,r in rows.items():
    if r[2]=="ImportDeclaration": continue
    L[i]=lab(r[0]); size[L[i]]+=r[1]-r[0]+1
print({k:v for k,v in size.items()})
E=collections.defaultdict(list)
for i,r in rows.items():
    if i not in L: continue
    for e in r[4]:
        if e in L and L[e]!=L[i]: E[(L[i],L[e])].append((r[3],rows[e][3]))
for (a,b),v in sorted(E.items()): print(f"{a} -> {b}: {len(v)}  e.g. {v[0]}")
# cycles
G=collections.defaultdict(set)
for (a,b) in E: G[a].add(b)
def cyc():
    seen={};path=[]
    out=[]
    def dfs(u):
        seen[u]=1;path.append(u)
        for v in G[u]:
            if seen.get(v)==1: out.append(path[path.index(v):]+[v])
            elif v not in seen: dfs(v)
        path.pop();seen[u]=2
    for u in list(G): 
        if u not in seen: dfs(u)
    return out
for c in cyc():
    print("CYCLE"," -> ".join(c))
    for a,b in zip(c,c[1:]):
        print("    ",a,"->",b,E[(a,b)][:6])
