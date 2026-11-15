import os,re,csv,sys
ROOT='/Users/Shared/semio'
F=ROOT+'/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-server-canonical-decisions.tsv'
rows=list(csv.reader(open(F,encoding='utf8'),delimiter='\t'))[1:]
targets=[r[0] for r in rows if r[2] in('take','conflict','copy-in') and r[0].endswith(('.ts','.mjs','.tsx','.js'))]
EXT=('.ts','.tsx','.mjs','.js','.mts')
SKIP={'node_modules','target','.git','dist','.cache','.🧬semio'}
def strip(t):
    return t
def strip_old(t):
    t=re.sub(r'/\*.*?\*/','',t,flags=re.S)
    return re.sub(r'(?m)(^|[^:\'"`])//.*$',r'\1',t)
cache={}
def read(p):
    if p not in cache:
        try: cache[p]=strip(open(p,encoding='utf8').read())
        except Exception: cache[p]=None
    return cache[p]
def resolve(frm,spec):
    if not spec.startswith('.'): return None
    b=os.path.normpath(os.path.join(os.path.dirname(frm),spec))
    for c in [b]+[b+e for e in EXT]+[b[:-3]+e for e in EXT if b.endswith(('.js',))]+[os.path.join(b,'index'+e) for e in EXT]:
        if os.path.isfile(c): return c
    return b
imp_re=re.compile(r'(?:^|[\n;])\s*(import|export)\s+(type\s+)?(\{[^}]*\}|\*(?:\s+as\s+\w+)?|\w+(?:\s*,\s*\{[^}]*\})?)\s*from\s*[\'"]([^\'"]+)[\'"]')
def names(block):
    out=[]
    for m in re.finditer(r'\{([^}]*)\}',block):
        for x in m.group(1).split(','):
            x=x.strip()
            if not x: continue
            if re.match(r'type\s+',x): continue
            out.append(x.split(' as ')[0].strip())
    return out
def exports(p,seen=None):
    seen=seen or set()
    if p in seen: return set()
    seen.add(p)
    t=read(p)
    if t is None: return None
    ex=set()
    for m in re.finditer(r'\bexport\s+(?:const|let|var)\s+([^;]*)',t):
        for n in re.findall(r'(?:^|,)\s*(\w+)\s*[:=]',m.group(1))+re.findall(r'^\s*\{([^}]*)\}',m.group(1))[0].replace(' ','').split(',') if re.match(r'\s*\{',m.group(1)) else re.findall(r'(?:^|,)\s*(\w+)\s*[:=]',m.group(1)): ex.add(n)
    for m in re.finditer(r'\bexport\s+(?:declare\s+)?(?:async\s+)?(?:abstract\s+)?(?:function\s*\*?\s*|const\s+|let\s+|var\s+|class\s+|type\s+|interface\s+|enum\s+|namespace\s+)(\w+)',t): ex.add(m.group(1))
    if re.search(r'\bexport\s+default\b',t): ex.add('default')
    for m in re.finditer(r'\bexport\s*(?:type\s*)?\{([^}]*)\}(\s*from\s*[\'"]([^\'"]+)[\'"])?',t):
        for x in m.group(1).split(','):
            x=x.strip()
            if x:
                x=re.sub(r'^type\s+','',x); ex.add(x.split(' as ')[-1].strip())
    for m in re.finditer(r'\bexport\s*\*\s*(?:as\s+(\w+)\s*)?from\s*[\'"]([^\'"]+)[\'"]',t):
        if m.group(1): ex.add(m.group(1)); continue
        r=resolve(p,m.group(2))
        if r and os.path.isfile(r):
            e=exports(r,seen)
            if e is not None: ex|=e
    return ex
def imports(p):
    t=read(p) or ''
    res=[]
    for m in imp_re.finditer(t):
        res.append((m.group(1),[] if m.group(2) else (names(m.group(3)) if '{' in m.group(3) else []),m.group(4),('default' if not m.group(2) else None) if re.match(r'\w+',m.group(3)) and not m.group(3).startswith(('{','*')) else None))
    return res
def check(p):
    bad=[]
    for kind,nm,spec,dflt in imports(p):
        r=resolve(p,spec)
        if r is None: continue
        if not os.path.isfile(r): bad.append((p,spec,'FILE-MISSING',[])); continue
        if r.endswith(('.json','.cjs')): continue
        ex=exports(r)
        if ex is None: continue
        miss=[n for n in nm if n not in ex]
        if dflt and 'default' not in ex: miss.append('default')
        if miss: bad.append((p,spec,'MISSING',miss))
    return bad
if __name__=='__main__':
    mode=sys.argv[1]
    if mode=='targets':
        for t in targets:
            p=os.path.join(ROOT,t)
            print('TARGET',t, 'exists' if os.path.isfile(p) else 'ABSENT')
            if os.path.isfile(p):
                for b in check(p): print('  OUT-MISMATCH',b[1],b[2],b[3])
    elif mode=='importers':
        tset={os.path.join(ROOT,t) for t in targets}
        for base in ('🧰️framework','✏️s','🌎️hub'):
          for dp,dn,fn in os.walk(os.path.join(ROOT,base)):
            dn[:]=[d for d in dn if d not in SKIP]
            for f in fn:
                if f.endswith(EXT):
                    p=os.path.join(dp,f)
                    for b in check(p):
                        r=resolve(p,b[1])
                        if r in tset: print('IN-MISMATCH',p[len(ROOT)+1:],'->',b[1],b[3])
