import json,sys,re,collections
# usage: blk.py <log> -> loads first blocker JSON object and exposes graph analysis
text=open(sys.argv[1],errors='ignore').read()
i=text.index('\n{\n')
dec=json.JSONDecoder()
obj,_=dec.raw_decode(text[i+1:])
print(list(obj.keys()))
print('slices',obj['slices'],'files',obj['files'],'edges',len(obj['edges']))
print(obj['edges'][:3])
print(obj['rings'][:1] if obj.get('rings') else None)
G=collections.defaultdict(set); why={}
for e in obj['edges']:
    m=re.match(r'^(.*?) -> (.*?)( \((.*)\))?( \[test\])?$',e)
    a,b=m.group(1),m.group(2)
    if ' (' in b: b=b.split(' (')[0]
    G[a].add(b); why[(a,b)]=m.group(4)
    G[b]
def scc(G):
    idx={};low={};st=[];on=set();res=[];c=[0]
    sys.setrecursionlimit(100000)
    def v(n):
        idx[n]=low[n]=c[0];c[0]+=1;st.append(n);on.add(n)
        for m in G[n]:
            if m not in idx: v(m);low[n]=min(low[n],low[m])
            elif m in on: low[n]=min(low[n],idx[m])
        if low[n]==idx[n]:
            comp=[]
            while True:
                x=st.pop();on.discard(x);comp.append(x)
                if x==n:break
            res.append(comp)
    for n in list(G):
        if n not in idx:v(n)
    return res
comps=[c for c in scc(G) if len(c)>1]
print('edge-SCCs (no coherence):',[len(c) for c in comps])
for c in comps:
    print('--- SCC',len(c)); 
    for n in sorted(c)[:60]: print('  ',n)
print('=========== inner edges excluding leaf-diff traffic')
for ci,c in enumerate(comps):
    s=set(c)
    print('SCC',ci,len(c))
    for a in sorted(c):
        for b in sorted(G[a]):
            if b in s and not (a.endswith('::diff') or b.endswith('::diff')) and '::commands::' not in a+b:
                print('  ',a,'->',b,' |',(why.get((a,b)) or '')[:120])
print('=========== whys into editor::puzzle5d')
cnt=collections.Counter()
for (a,b),w in why.items():
    if b=='editor::puzzle5d' and w: cnt[w.split(' @ ')[0]]+=1
for k,v in cnt.most_common(40): print(v,k)
