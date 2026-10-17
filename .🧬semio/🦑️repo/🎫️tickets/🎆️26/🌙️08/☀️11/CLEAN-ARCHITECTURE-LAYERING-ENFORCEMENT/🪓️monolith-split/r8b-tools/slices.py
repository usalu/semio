import sys,re
lines=open(sys.argv[1],encoding='utf8').read().split('\n')
blk=int(sys.argv[2]) if len(sys.argv)>2 else 0
# find blockers
starts=[i for i,l in enumerate(lines) if l.startswith('blocker:')]
s=starts[blk]; e=starts[blk+1] if blk+1<len(starts) else len(lines)
out=[]
cur=None
for l in lines[s:e]:
    m=re.match(r'^  slice (.*?) \[\{.*?\}\] files=(\d+) bytes=(\d+) lines=(\d+) -> (.*)$',l)
    if m:
        cur=[m.group(1),m.group(2),m.group(3),m.group(4),m.group(5).split(' ; ')]
        out.append(cur)
    elif l.startswith('      ; ') and cur:
        cur[4].append(l[8:])
def sh(x): return x.replace('schema::mutations::','m:').replace('subsets::any::schema','S').replace('standards::v1::','')
for name,f,b,li,es in out:
    print(f"## {sh(name)[:70]} files={f} bytes={b} lines={li}")
    for x in es:
        m=re.match(r'^(\S+) \((.*?)\)(.*)$',x)
        if m: print("     ->",sh(m.group(1)),'|',sh(m.group(2))[:110])
        else: print("     ->",x[:120])

# SCC over slice names
import re as _re
names=[o[0] for o in out]
def resolve(t):
    # edge target name as printed ("v1::subsets::any::schema") equals a slice name possibly with ' + '
    for n in names:
        if n==t or n.split(' + ')[0]==t or t in n.split(' + '): return n
    return None
g={}
for name,f,b,li,es in out:
    g[name]=[]
    for x in es:
        m=_re.match(r'^(\S+) \(',x)
        tgt=m.group(1) if m else x.split(' ')[0]
        r=resolve(tgt)
        if r and r!=name: g[name].append(r)
import sys as _s
_s.setrecursionlimit(10000)
idx={};low={};st=[];on=set();res=[];c=[0]
def sc(v):
    idx[v]=low[v]=c[0];c[0]+=1;st.append(v);on.add(v)
    for w in g[v]:
        if w not in idx: sc(w);low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],idx[w])
    if low[v]==idx[v]:
        comp=[]
        while True:
            w=st.pop();on.discard(w);comp.append(w)
            if w==v:break
        res.append(comp)
for n in names:
    if n not in idx: sc(n)
print("SCCs:")
for comp in res:
    if len(comp)>1:
        files=sum(int(o[1]) for o in out if o[0] in comp)
        print(len(comp),'slices',files,'files:',[sh(x)[:40] for x in comp])
