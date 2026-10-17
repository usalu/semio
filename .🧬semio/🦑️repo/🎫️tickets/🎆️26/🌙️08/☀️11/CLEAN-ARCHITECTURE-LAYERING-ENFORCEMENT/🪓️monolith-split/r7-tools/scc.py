# 🕸️ Parses `F3_SLICES=1 --explain` slice lines and prints the strongly connected components of the lib+test slice graph and the cheapest edges (by target subtree) that keep them together: usage `python3 scc.py <log> [--lib]`.
import re,sys,collections
log=sys.argv[1]; libonly='--lib' in sys.argv
lines=[l.rstrip('\n') for l in open(log,encoding='utf8') if l.startswith('  slice ')]
slices=[]
for l in lines:
    m=re.match(r'  slice (.*?) \[\{(.*?)\}\] files=(\d+) bytes=(\d+) lines=(\d+) -> ?(.*)$',l)
    if not m: continue
    names=m.group(1); files=int(m.group(3)); lines_=int(m.group(5)); rest=m.group(6)
    edges=[]
    for part in rest.split(' ; '):
        part=part.strip()
        if not part: continue
        mm=re.match(r'(.*?) \((.*)\)$',part)
        if not mm: continue
        tgt=mm.group(1); why=mm.group(2)
        test=tgt.endswith(' [test]') or ' [test]' in tgt
        tgt=tgt.replace(' [test]','')
        if libonly and test: continue
        edges.append((tgt.strip(),why))
    slices.append((names,files,lines_,edges))
# map names -> idx: a slice is named by its first name; edges target by an eff suffix of any member
idx={}
for i,(n,f,l,e) in enumerate(slices):
    for part in n.split(' + '):
        idx[part.strip()]=i
def find(t):
    if t in idx: return idx[t]
    cands=[k for k in idx if k.endswith(t) or t.endswith(k)]
    if len(cands)>=1:
        cands.sort(key=lambda k:-len(k)); return idx[cands[0]]
    return None
adj=collections.defaultdict(set); lab={}
for i,(n,f,l,e) in enumerate(slices):
    for t,why in e:
        j=find(t)
        if j is None or j==i: continue
        adj[i].add(j); lab[(i,j)]=why
# tarjan
sys.setrecursionlimit(10000)
index={}; low={}; st=[]; on=set(); comps=[]; c=[0]
def sc(v):
    index[v]=low[v]=c[0]; c[0]+=1; st.append(v); on.add(v)
    for w in adj[v]:
        if w not in index: sc(w); low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],index[w])
    if low[v]==index[v]:
        comp=[]
        while True:
            w=st.pop(); on.discard(w); comp.append(w)
            if w==v: break
        comps.append(comp)
for v in range(len(slices)):
    if v not in index: sc(v)
for comp in sorted(comps,key=lambda x:-len(x)):
    if len(comp)<2: continue
    files=sum(slices[i][1] for i in comp); ln=sum(slices[i][2] for i in comp)
    print(f'SCC size {len(comp)} files {files} lines {ln}')
    cs=set(comp)
    for i in comp:
        outs=[(slices[j][0][:70],lab[(i,j)][:60]) for j in adj[i] if j in cs]
        print('  ',slices[i][0][:80],f'[{slices[i][1]}f/{slices[i][2]}l]','->',' | '.join(f'{a} ({b})' for a,b in outs)[:600])
