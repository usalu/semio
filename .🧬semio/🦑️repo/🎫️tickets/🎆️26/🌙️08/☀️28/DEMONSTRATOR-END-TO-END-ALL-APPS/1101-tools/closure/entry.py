import sys,re,collections
lock,root=sys.argv[1],sys.argv[2]
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
s={by[root][0]};q=list(s)
while q:
    k=q.pop()
    for d in P[k]['dependencies']:
        x=res(d)
        if x not in s:s.add(x);q.append(x)
isstd=lambda n:n.startswith('semio-s-artifact-stdio-')
out=collections.defaultdict(list)
for k in s:
    if isstd(k[0]): continue
    for d in P[k]['dependencies']:
        x=res(d)
        if isstd(x[0]) and 'source' not in P[k] or (isstd(x[0])): out[k[0]].append(x[0].replace('semio-s-artifact-stdio-',''))
for k,v in sorted(out.items()): print(k,'->',' '.join(sorted(v)))
