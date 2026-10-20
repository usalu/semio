#!/usr/bin/env python3
"""Scratch experiment: hierarchical directory refinement of a cyclic unit graph. Input: /private/tmp/conc-ts/fedges.json (+ module roots from analyze_units)."""
import json,os,sys,collections
sys.setrecursionlimit(1000000)
g=json.load(open('/private/tmp/conc-ts/fedges.json'))
files=g['files']; fe=g['fedges']
roots=set([''])
fs=set(files)
# module roots as in analyze_units (reconstruct)
for f in files:
    parts=f.split('/')
    for i,p in enumerate(parts[:-1]):
        if p=='📦️packages': roots.add('/'.join(parts[:i]))
        if p=='🔌️plugins' and i+1<len(parts)-1: roots.add('/'.join(parts[:i+2]))
R='/Users/ueli/Documents/semio'
for f in os.popen("cd '%s' && git ls-files -co --exclude-standard -- ':!.🧬semio' '*package.json'"%R).read().split('\n'):
    if f.endswith('package.json'):
        d=os.path.dirname(f)
        if '/📦️packages/' not in d+'/': roots.add(d)
def module_of(f):
    d=os.path.dirname(f)
    while d and d not in roots: d=os.path.dirname(d)
    return d
ISOLATE=os.environ.get('ISOLATE','🧪️tests')
# unit key: (module, tuple(path segments below module))  -> unit identified by prefix path and mode
# initial: unit = module, with isolated child dir 'tests' split off
def initial(f):
    m=module_of(f); rel=f[len(m)+1:] if m else f
    segs=rel.split('/')
    if ISOLATE and ISOLATE in segs[:-1]:
        i=segs.index(ISOLATE); return (m+'/'+'/'.join(segs[:i+1])).strip('/')
    return m
# each file's unit assignment is a path prefix string; refinement extends prefix
assign={f:initial(f) for f in files}
def unitroot(u): return u
# edges file-level
def quotient():
    ed=collections.defaultdict(set)
    for a,b in fe:
        ua,ub=assign[a],assign[b]
        if ua!=ub: ed[ua].add(ub)
    return ed
def scc(nodes,ed):
    idx={};low={};on=set();st=[];out=[];c=[0]
    for s in nodes:
        if s in idx: continue
        stack=[(s,iter(ed.get(s,())))]; idx[s]=low[s]=c[0];c[0]+=1;st.append(s);on.add(s)
        while stack:
            v,it=stack[-1]
            adv=False
            for w in it:
                if w not in idx:
                    idx[w]=low[w]=c[0];c[0]+=1;st.append(w);on.add(w);stack.append((w,iter(ed.get(w,()))));adv=True;break
                elif w in on: low[v]=min(low[v],idx[w])
            if adv: continue
            stack.pop()
            if stack: low[stack[-1][0]]=min(low[stack[-1][0]],low[v])
            if low[v]==idx[v]:
                comp=[]
                while True:
                    w=st.pop();on.discard(w);comp.append(w)
                    if w==v:break
                out.append(comp)
    return out
def membership():
    m=collections.defaultdict(list)
    for f,u in assign.items(): m[u].append(f)
    return m
rounds=0
frozen=set()
while True:
    ed=quotient(); mem=membership()
    comps=[c for c in scc(list(mem),ed) if len(c)>1 and not all(u in frozen for u in c)]
    if not comps: break
    rounds+=1
    split=0
    LIMIT=int(os.environ.get('LIMIT','0')); MAXR=int(os.environ.get('MAXR','999'))
    if rounds>MAXR: LIMIT=10**9
    rest=[]
    for c in comps:
        tot=sum(len(mem[u]) for u in c)
        if LIMIT and tot<=LIMIT:
            mid='merged:'+min(c)
            for u in c:
                for f in mem[u]: assign[f]=mid
            frozen.add(mid)
        else: rest.append(c)
    comps=rest
    for c in comps:
        for u in c:
            # children by next segment
            if u in frozen: continue
            fl=mem[u]
            if len(fl)==1:
                frozen.add(u); continue
            if '#' in u:
                for f in fl: assign[f]='#file:'+f
                continue
            nxt=collections.defaultdict(list)
            for f in fl:
                rest=f[len(u)+1:] if u else f
                segs=rest.split('/')
                key=u+('/' if u else '')+segs[0] if len(segs)>1 else u+'#self'
                nxt[key].append(f)
            if len(nxt)==1:
                for f in fl: assign[f]='#file:'+f
                continue
            for k,v in nxt.items():
                for f in v: assign[f]=k
            split+=1
    sys.stdout.flush(); print('round',rounds,'cyclic comps',len(comps),'sizes',sorted((len(c) for c in comps),reverse=True)[:5],'units now',len(set(assign.values())))
mem=membership()
print('indivisible cyclic comps (genuine file-level cycles):',len([c for c in scc(list(mem),quotient()) if len(c)>1]))
sizes=sorted((len(v) for v in mem.values()),reverse=True)
print('final units',len(mem),'max',sizes[:10],'files',len(files))
tot=sum(sizes); 
for t in (50,100,200,500,1000): print('files in units >',t,sum(s for s in sizes if s>t))
ed=quotient()
print('unit edges',sum(len(v) for v in ed.values()))
import statistics
print('median unit',statistics.median(sizes))
json.dump({'assign':assign},open('/private/tmp/conc-ts/assign.json','w'),ensure_ascii=False)

# ---- coarsening: merge small units under one parent when the quotient stays acyclic
MIN=int(os.environ.get('MIN','0'))
if MIN:
    mem=membership(); ed=quotient()
    out={u:set(v) for u,v in ed.items()}; inn=collections.defaultdict(set)
    for u,vs in out.items():
        for v in vs: inn[v].add(u)
    for u in mem: out.setdefault(u,set())
    def reach(starts,adj):
        seen=set(starts); st=list(starts)
        while st:
            x=st.pop()
            for y in adj.get(x,()):
                if y not in seen: seen.add(y); st.append(y)
        return seen
    def parent(u):
        p=u.split('#')[0] if not u.startswith('#file:') else u[6:]
        if u.startswith('merged:'): p=u[7:]
        return os.path.dirname(p)
    groups=collections.defaultdict(list)
    for u,fl in mem.items():
        if len(fl)<MIN: groups[parent(u)].append(u)
    merges=0
    for p,us in sorted(groups.items()):
        us.sort(key=lambda u:(len(mem[u]),u))
        cluster=[];size=0
        def flush():
            global cluster,size
            cluster=[];size=0
        for u in sorted(us):
            trial=cluster+[u]
            M=set(trial)
            d=reach(M,out)-M; a=reach(M,inn)-M
            if d&a: 
                continue
            cluster=trial
            if sum(len(mem[x]) for x in cluster)>=MIN:
                if len(cluster)>1:
                    name='cluster:'+cluster[0]
                    for x in cluster:
                        for f in mem[x]: assign[f]=name
                    # update graph
                    newout=set();newin=set()
                    for x in cluster:
                        newout|=out.pop(x,set());newin|=inn.pop(x,set())
                    newout-=set(cluster);newin-=set(cluster)
                    out[name]=newout;inn[name]=newin
                    for w in newout: inn[w]=(inn[w]-set(cluster))|{name}
                    for w in newin: out[w]=(out[w]-set(cluster))|{name}
                    merges+=1
                cluster=[];size=0
        if len(cluster)>1:
            name='cluster:'+cluster[0]
            for x in cluster:
                for f in mem[x]: assign[f]=name
            newout=set();newin=set()
            for x in cluster:
                newout|=out.pop(x,set());newin|=inn.pop(x,set())
            newout-=set(cluster);newin-=set(cluster)
            out[name]=newout;inn[name]=newin
            for w in newout: inn[w]=(inn[w]-set(cluster))|{name}
            for w in newin: out[w]=(out[w]-set(cluster))|{name}
            merges+=1
    mem=membership(); sizes=sorted((len(v) for v in mem.values()),reverse=True)
    print('after coarsening: merges',merges,'units',len(mem),'max',sizes[:6],'median',statistics.median(sizes))
    for t in (25,50,100,200,500): print(' units <=',t,sum(1 for s in sizes if s<=t))
    print('residual cyclic comps',len([c for c in scc(list(mem),quotient()) if len(c)>1]),'unit edges',sum(len(v) for v in quotient().values()))
sing=collections.Counter()
for u,fl in membership().items():
    if len(fl)==1:
        f=fl[0]; sing['/'.join(f.split('/')[:4])]+=1
print(sing.most_common(15))
