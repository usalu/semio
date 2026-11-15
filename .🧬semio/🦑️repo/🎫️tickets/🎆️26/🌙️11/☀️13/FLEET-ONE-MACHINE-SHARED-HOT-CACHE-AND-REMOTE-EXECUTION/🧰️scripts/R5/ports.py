#!/usr/bin/env python3
import json,os,re,sys,collections
sys.path.insert(0,os.path.dirname(__file__))
R='/Users/Shared/semio/'
LIB='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
SPEC=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+|require\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
EXT=re.compile(r'\.(ts|mjs|js|cjs|mts|tsx)$')
def tree():
    out=[]
    for base in (LIB+'🏗️build',LIB+'⚡️caching'):
        for d,ds,fs in os.walk(R+base):
            ds[:]=[x for x in ds if x!='node_modules']
            for f in fs: out.append(os.path.relpath(os.path.join(d,f),R))
    return out
def graph(files):
    fs=set(files); g={}
    for f in files:
        if not EXT.search(f): continue
        try: src=open(R+f,encoding='utf8').read()
        except Exception: continue
        dst=set()
        for m in SPEC.finditer(src):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            for c in (p,re.sub(r'\.js$','.ts',p),re.sub(r'\.mjs$','.d.mts',p)):
                if c in fs: dst.add(c); break
        g[f]=dst
    return g
def ports(prefixes,g):
    inside=lambda f:any(f.startswith(LIB+p) for p in prefixes)
    c=collections.Counter()
    for f,ds in g.items():
        if inside(f): continue
        if '/🧪️tests/' in f: continue
        for t in ds:
            if inside(t): c[t[len(LIB):]]+=1
    return c
if __name__=='__main__':
    files=tree(); g=graph(files)
    groups=json.loads(sys.argv[1])
    for name,prefixes in groups.items():
        c=ports(prefixes,g)
        n=sum(1 for f in files if any(f.startswith(LIB+p) for p in prefixes))
        print(name,'files',n,'ports',len(c))
        for p,k in sorted(c.items()): print('  ',k,p)
