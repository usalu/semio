import sys,re,collections
lock,root,unusedf,mode=sys.argv[1:5]
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
fam=lambda n:(re.match(r'semio-s-artifact-stdio-([a-z0-9]+)',n) or [0,None])[1]
rm=set();own=None
for l in open(unusedf,encoding='utf8'):
    if l.startswith('semio'): own=l.split()[0];continue
    p=l.split()
    if len(p)<2: continue
    dep=p[1]
    ownstd=own.startswith('semio-s-artifact-stdio-')
    if fam(own)=='pdf': continue
    if mode=='A' and ownstd: continue
    if mode=='B' and ownstd and fam(own)==fam(dep): continue
    rm.add((own,dep))
def clo(r):
    s={r};q=[r]
    while q:
        k=q.pop()
        for d in P[k]['dependencies']:
            x=res(d)
            if (k[0],x[0]) in rm: continue
            if x not in s:s.add(x);q.append(x)
    return s
r=by[root][0]
s0=set(); 
rm0=rm;rm=set();s0=clo(r);rm=rm0;s1=clo(r)
f=collections.Counter(fam(k[0]) for k in s1 if fam(k[0]))
print(mode,'before',len(s0),'after',len(s1),dict(f))
