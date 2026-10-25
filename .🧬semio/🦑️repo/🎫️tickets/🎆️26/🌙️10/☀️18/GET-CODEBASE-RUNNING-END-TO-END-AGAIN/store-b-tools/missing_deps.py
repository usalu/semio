import os,re,sys,tomllib
PATH=re.compile(r'(?m)^\s*#\[path\s*=\s*"([^"]+)"\s*\]')
INC=re.compile(r'include!\s*\(\s*"([^"]*)"\s*\)')
USE=re.compile(r'\b(semio_framework_[a-z0-9_]+|semio_[a-z0-9_]+)::')
def closure(md,lib):
    seen=set()
    def scan(f):
        f=os.path.normpath(f)
        if f in seen or not os.path.isfile(f):return
        seen.add(f);t=open(f,encoding='utf-8',errors='replace').read();d=os.path.dirname(f)
        for m in PATH.finditer(t):
            p=os.path.normpath(os.path.join(d,m.group(1)))
            if p.endswith('.rs'):scan(p)
        for m in INC.finditer(t):
            p=os.path.normpath(os.path.join(d,m.group(1)))
            if p.endswith('.rs'):scan(p)
    scan(os.path.join(md,lib));return seen
for mf in sys.argv[1:]:
    md=os.path.dirname(mf);c=tomllib.load(open(mf,'rb'))
    names=set()
    for k in ('dependencies','dev-dependencies','build-dependencies'):
        for key,v in c.get(k,{}).items():
            names.add(key.replace('-','_'))
            if isinstance(v,dict) and 'package' in v:names.add(v['package'].replace('-','_'))
    lib=c.get('lib',{}).get('path','src/lib.rs');files=closure(md,lib)
    used={}
    for f in files:
        t=open(f,encoding='utf-8',errors='replace').read()
        t=re.sub(r'(?m)^\s*//.*$','',t)
        for m in USE.finditer(t):
            # skip mod paths inside the crate: heuristic only crates
            used.setdefault(m.group(1),f)
    me=c['package']['name'].replace('-','_')
    miss=sorted(u for u in used if u not in names and u!=me)
    print(c['package']['name'],'MISSING:',miss)
