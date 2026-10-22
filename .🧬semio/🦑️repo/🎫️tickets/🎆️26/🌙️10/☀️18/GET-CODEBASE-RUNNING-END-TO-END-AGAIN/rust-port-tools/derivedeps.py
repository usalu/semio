#!/usr/bin/env python3
"""Adds the path dependencies `derive(Mutations)` expands to (os-command, os-store-one-item, ui-locale, schema-state, value) to every non-plugin crate that mounts an aggregate. Idempotent. usage: derivedeps.py @sites-list [--dry]"""
import os,re,sys
ROOT='/Users/ueli/Documents/semio'
os.chdir(ROOT)
dry='--dry' in sys.argv
sites=[]
for a in sys.argv[1:]:
    if a.startswith('@'): sites+=[l for l in open(a[1:],encoding='utf-8').read().split('\n') if l]
NEED=['semio-framework-os-command','semio-framework-os-store-one-item','semio-framework-ui-locale','semio-framework-schema-state','semio-framework-value']
DEV={'semio-framework-os-store-one-item'}
import subprocess
manifests=[l for l in subprocess.run(['git','ls-files','-z','--','*Cargo.toml'],capture_output=True,text=True).stdout.split('\0') if l and '/node_modules/' not in l and '/🔌️plugins/' not in l and os.path.isfile(l)]
pk={}
for p in manifests:
    t=open(p,encoding='utf-8').read()
    m=re.search(r'^name = "([^"]+)"',t,re.M)
    if m and m.group(1) in NEED: pk[m.group(1)]=os.path.dirname(p)
missing=[n for n in NEED if n not in pk]
assert not missing,missing
def libof(r,t):
    m=re.search(r'^\[lib\][^\[]*?path\s*=\s*"([^"]+)"',t,re.M|re.S)
    return os.path.normpath(os.path.join(r,m.group(1))) if m else os.path.join(r,'src/lib.rs')
def mounted(libfile):
    seen=set();stack=[libfile]
    while stack:
        fp=os.path.realpath(stack.pop())
        if fp in seen or not os.path.isfile(fp): continue
        seen.add(fp)
        try:s=open(fp,encoding='utf-8').read()
        except:continue
        d=os.path.dirname(fp)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',s): stack.append(os.path.join(d,m.group(1)))
        for m in re.finditer(r'include!\(\s*"([^"]+)"',s): stack.append(os.path.join(d,m.group(1)))
    return seen
crates={}
want={os.path.realpath(f) for f in sites}
for p in manifests:
    t=open(p,encoding='utf-8').read()
    if '[package]' not in t: continue
    r=os.path.dirname(p)
    tests=[os.path.normpath(os.path.join(r,m)) for m in re.findall(r'^\[\[test\]\][^\[]*?path\s*=\s*"([^"]+)"',t,re.M|re.S)]
    files=set()
    for x in [libof(r,t)]+tests: files|=mounted(x)
    if files&want and '/✨️derive/' not in p: crates[p]=(r,t)
def insert(lines,section,add):
    try: i=lines.index(section)
    except ValueError:
        lines+=['',section]+add;return
    j=i+1
    while j<len(lines) and not lines[j].startswith('['): j+=1
    k=j
    while k>i+1 and lines[k-1].strip()=='': k-=1
    lines[k:k]=add
n=0
for p,(r,t) in sorted(crates.items()):
    lines=t.split('\n');normal=[];dev=[]
    for dep in NEED:
        if re.search(r'^'+re.escape(dep)+r'\s*=',t,re.M) or re.search(r'^\[(?:dev-)?dependencies\.'+re.escape(dep)+r'\]',t,re.M): continue
        if re.search(r'^name = "'+re.escape(dep)+r'"',t,re.M): continue
        rel=os.path.relpath(pk[dep],r)
        (dev if dep in DEV else normal).append(f'{dep} = {{ path = "{rel}" }}')
    if not normal and not dev: continue
    if normal: insert(lines,'[dependencies]',normal)
    if dev: insert(lines,'[dev-dependencies]',dev)
    n+=1;print(p[-90:],[a.split(' ')[0] for a in normal],'dev',[a.split(' ')[0] for a in dev])
    if not dry: open(p,'w',encoding='utf-8').write('\n'.join(lines))
print('crates',len(crates),'updated',n)
