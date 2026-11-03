import sys,re,os,collections
ROOT='/Users/ueli/Documents/semio'
man=[l.strip() for l in open(sys.argv[1],encoding='utf8')]
lock,root,prefix=sys.argv[2],sys.argv[3],sys.argv[4]
name2dir={}
for m in man:
    try: t=open(os.path.join(ROOT,m),encoding='utf8').read()
    except: continue
    mm=re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"',t,re.M|re.S)
    if mm: name2dir[mm.group(1)]=os.path.join(ROOT,os.path.dirname(m))
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
def roots(d,mt):
    r={d}
    for m in re.finditer(r'^path\s*=\s*"([^"]+)"',mt,re.M):
        pth=os.path.normpath(os.path.join(d,m.group(1)))
        if pth.endswith('.rs'): r.add(os.path.dirname(pth))
    return r
_cache={}
def walk(d,top):
    key=(d,top)
    if key in _cache: return _cache[key]
    fl=[]
    for dp,dn,fn in os.walk(d):
        dn[:]=[x for x in dn if not (os.path.exists(os.path.join(dp,x,'Cargo.toml')) and os.path.join(dp,x)!=top) and x not in('target','node_modules')]
        for f in fn:
            if f.endswith('.rs'): fl.append(os.path.join(dp,f))
    _cache[key]=fl
    return fl
_txt={}
def rd(f):
    if f not in _txt:
        try: _txt[f]=open(f,encoding='utf8').read()
        except: _txt[f]=''
    return _txt[f]
def src(d,mt):
    files=set();todo=[]
    for rt in roots(d,mt):
        for f in walk(rt,d): 
            if f not in files: files.add(f);todo.append(f)
    while todo:
        f=todo.pop()
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',rd(f)):
            t=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            if t.endswith('.rs') and os.path.isfile(t):
                for g in [t]+walk(os.path.dirname(t),d):
                    if g not in files: files.add(g);todo.append(g)
    return '\n'.join(rd(f) for f in files)
for k in sorted(s):
    if k[0] not in name2dir: continue
    d=name2dir[k[0]]
    mt=open(os.path.join(d,'Cargo.toml'),encoding='utf8').read()
    S=None;un=[]
    sec=None
    for line in mt.split('\n'):
        h=re.match(r'\[(.*)\]',line)
        if h: sec=h.group(1);continue
        if sec and ('dependencies' in sec):
            m=re.match(r'([A-Za-z0-9_-]+)\s*=',line)
            if m and m.group(1).startswith(prefix):
                if S is None: S=src(d,mt)
                n=m.group(1).replace('-','_')
                if not re.search(r'\b'+n+r'\b',S): un.append((sec,m.group(1)))
    if un: print(k[0],os.path.relpath(d,ROOT)); [print('   ',a,b) for a,b in un]
