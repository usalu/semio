#!/usr/bin/env python3.12
"""Static check: every workspace crate's reachable Rust sources must have existing #[path]/include_* targets (mirrors the cargo source admission)."""
import os,re,sys,tomllib
root=sys.argv[1] if len(sys.argv)>1 else '.'
ws=tomllib.load(open(os.path.join(root,'Cargo.toml'),'rb'))['workspace']['members']
PATH=re.compile(r'(?m)^#\[path\s*=\s*"([^"]+)"\s*\]')
INC=re.compile(r'include(?:_str|_bytes)?!\s*\(\s*(concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*)"\s*\)|"([^"]*)")\s*\)')
bad=[]
def scan(crate,manifest_dir,entry,seen):
    if entry in seen: return
    seen.add(entry)
    if not os.path.isfile(entry):
        bad.append((crate,entry,'entry missing'));return
    t=open(entry,encoding='utf-8',errors='replace').read()
    t=re.sub(r'(?m)^\s*//.*$','',t)
    d=os.path.dirname(entry)
    for m in PATH.finditer(t):
        p=os.path.normpath(os.path.join(d,m.group(1)))
        scan(crate,manifest_dir,p,seen) if p.endswith('.rs') else None
        if not os.path.exists(p): bad.append((crate,entry+' -> path',m.group(1)))
    for m in INC.finditer(t):
        if m.group(2) is not None: p=os.path.normpath(manifest_dir+m.group(2))
        else: p=os.path.normpath(os.path.join(d,m.group(3)))
        if not os.path.exists(p): bad.append((crate,entry,m.group(0)[:160]))
        elif p.endswith('.rs') and 'include!' in m.group(0) and not 'include_' in m.group(0): scan(crate,manifest_dir,p,seen)
for member in ws:
    md=os.path.join(root,member)
    mf=os.path.join(md,'Cargo.toml')
    if not os.path.isfile(mf): bad.append((member,mf,'manifest missing'));continue
    c=tomllib.load(open(mf,'rb'))
    name=c.get('package',{}).get('name',member)
    entries=[]
    lib=c.get('lib',{}).get('path') or ('src/lib.rs' if os.path.exists(os.path.join(md,'src/lib.rs')) else None)
    if lib: entries.append(lib)
    for k in ('bin','test','bench','example'):
        for t in c.get(k,[]):
            if 'path' in t: entries.append(t['path'])
    for e in entries:
        scan(name,md,os.path.normpath(os.path.join(md,e)),set())
for b in bad: print(*b,sep=' | ')
print(len(ws),'members,',len(bad),'missing')
