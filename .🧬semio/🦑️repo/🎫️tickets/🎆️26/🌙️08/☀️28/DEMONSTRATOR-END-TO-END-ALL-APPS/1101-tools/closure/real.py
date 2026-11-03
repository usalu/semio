import sys,re,os,collections
ROOT='/Users/ueli/Documents/semio'
manf,lock,root=sys.argv[1:4]
removed=set()
if len(sys.argv)>4:
    for l in open(sys.argv[4],encoding='utf8'):
        p=l.split()
        if len(p)==2: removed.add((p[0],p[1]))
name2dir={}
for m in open(manf,encoding='utf8'):
    m=m.strip()
    try: t=open(os.path.join(ROOT,m),encoding='utf8').read()
    except: continue
    mm=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',t,re.M|re.S)
    if mm: name2dir[mm.group(1)]=os.path.join(ROOT,m)
pk=[];cur=None;ind=False
for l in open(lock,encoding='utf8'):
    l=l.rstrip()
    if l=='[[package]]': cur={'dependencies':[]};pk.append(cur);ind=False;continue
    if cur is None: continue
    m=re.match(r'(name|version|source) = "(.*)"',l)
    if m: cur[m.group(1)]=m.group(2);continue
    if l.startswith('dependencies = ['): ind=True;continue
    if ind:
        if l==']': ind=False
        else: cur['dependencies'].append(l.strip().strip(',').strip('"'))
P={(c['name'],c['version']):c for c in pk};by=collections.defaultdict(list)
for k in P: by[k[0]].append(k)
def res(d):
    n=d.split(' ');return (n[0],n[1]) if len(n)>=2 else by[n[0]][0]
def mdeps(name,isroot):
    t=open(name2dir[name],encoding='utf8').read()
    sec=None;out=set()
    for ln in t.split('\n'):
        h=re.match(r'\[(.*)\]\s*$',ln)
        if h: sec=h.group(1);continue
        if not sec or 'dependencies' not in sec: continue
        if 'dev-dependencies' in sec and not isroot: continue
        m=re.match(r'([A-Za-z0-9_-]+)\s*=(.*)',ln)
        if m:
            pn=re.search(r'package\s*=\s*"([^"]+)"',m.group(2))
            out.add(pn.group(1) if pn else m.group(1))
    return out
ws={k for k in P if 'source' not in P[k]}
r=by[root][0]
seen={r};q=[r];wsset=set()
while q:
    k=q.pop()
    if k in ws and k[0] in name2dir:
        deps=mdeps(k[0],k==r)
        nxt=[res(d) for d in P[k]['dependencies'] if res(d)[0] in deps]
        for n in nxt:
            pass
    else: nxt=[res(d) for d in P[k]['dependencies']]
    for x in nxt:
        if (k[0],x[0]) in removed: continue
        if x not in seen: seen.add(x);q.append(x)
fam=collections.Counter()
for k in seen:
    m=re.match(r'semio-s-artifact-stdio-([a-z0-9]+)',k[0])
    if m: fam[m.group(1)]+=1
print(len(seen),'crates; ws',sum(1 for k in seen if k in ws),dict(fam))
