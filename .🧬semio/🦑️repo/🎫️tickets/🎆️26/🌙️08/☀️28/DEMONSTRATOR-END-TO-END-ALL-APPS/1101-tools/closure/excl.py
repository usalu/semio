import sys,re,collections
lock,root=sys.argv[1],sys.argv[2]
depth=int(sys.argv[3]) if len(sys.argv)>3 else 1
pk={};byname=collections.defaultdict(list);cur=None;ind=False
for l in open(lock,encoding='utf8'):
    l=l.rstrip()
    if l=='[[package]]': cur={'dependencies':[]};ind=False;pk_list=None;L=cur;pk[id(cur)]=cur;continue
    if cur is None: continue
    m=re.match(r'(name|version|source) = "(.*)"',l)
    if m: cur[m.group(1)]=m.group(2);continue
    if l.startswith('dependencies = ['): ind=True;continue
    if ind:
        if l==']': ind=False
        else: cur['dependencies'].append(l.strip().strip(',').strip('"'))
P={}
for c in pk.values():
    P[(c['name'],c['version'])]=c;byname[c['name']].append((c['name'],c['version']))
def res(d):
    n=d.split(' ');
    if len(n)>=2: return (n[0],n[1])
    return byname[n[0]][0]
def clo(r,skip=None):
    s={r};q=[r]
    while q:
        k=q.pop()
        for d in P[k]['dependencies']:
            x=res(d)
            if skip and (k,x) in skip: continue
            if x not in s: s.add(x);q.append(x)
    return s
r=byname[root][0]
full=clo(r)
print(f'TOTAL {len(full)} crates; workspace-local {sum(1 for k in full if "source" not in P[k])}')
fam=sorted({k[0] for k in full if k[0].startswith('semio-s-artifact-stdio-')})
fams=collections.Counter()
for k in full:
    m=re.match(r'semio-s-artifact-stdio-([a-z0-9]+)',k[0])
    if m: fams[m.group(1)]+=1
print('stdio families:',dict(fams))
def edges(k): return [(k,res(d)) for d in P[k]['dependencies']]
res_=[]
for e in edges(r):
    s=clo(r,{e}); lost=len(full)-len(s)
    if lost>0: res_.append((lost,e[1][0]))
for lost,n in sorted(res_,reverse=True)[:40]: print(f'  -{lost:4d} if root drops {n}')
