import os,re,sys
PATH=re.compile(r'(?m)^\s*#\[path\s*=\s*"([^"]+)"\s*\]')
INC=re.compile(r'include(?:_str|_bytes)?!\s*\(\s*(?:concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*)"\s*\)|"([^"]*)")\s*\)')
md=sys.argv[1];lib=sys.argv[2]
seen={}
def scan(f):
    f=os.path.normpath(f)
    if f in seen or not os.path.isfile(f):return
    seen[f]=os.path.getsize(f)
    t=open(f,encoding='utf-8',errors='replace').read();d=os.path.dirname(f)
    for m in PATH.finditer(t):
        p=os.path.normpath(os.path.join(d,m.group(1)))
        if p.endswith('.rs'):scan(p)
    for m in INC.finditer(t):
        p=os.path.normpath(md+m.group(1)) if m.group(1) is not None else os.path.normpath(os.path.join(d,m.group(2)))
        if os.path.isfile(p):
            if p.endswith('.rs') and 'include!' in m.group(0):scan(p)
            else: seen.setdefault(p,os.path.getsize(p))
scan(os.path.join(md,lib))
tot=sum(seen.values());print(tot,len(seen))
for k,v in sorted(seen.items(),key=lambda x:-x[1])[:40]:print(v,k[-110:])
