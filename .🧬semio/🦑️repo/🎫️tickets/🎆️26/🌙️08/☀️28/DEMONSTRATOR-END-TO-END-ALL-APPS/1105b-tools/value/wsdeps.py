import subprocess,os,re,collections
files=[f for f in subprocess.run(["git","ls-files","--","*Cargo.toml",":!.🧬semio"],capture_output=True,text=True).stdout.split('\n') if f]
roots={}
def keys_of(p):
    if p in roots:return roots[p]
    try:t=open(p).read()
    except Exception:roots[p]=None;return None
    ks=set();on=False
    for l in t.split('\n'):
        s=l.strip()
        if s.startswith('['):
            on=s=='[workspace.dependencies]'
            m=re.match(r'\[workspace\.dependencies\.([^\]]+)\]',s)
            if m:ks.add(m.group(1).strip('"'))
        elif on:
            m=re.match(r'([A-Za-z0-9_\-]+)\s*[=.]',s)
            if m:ks.add(m.group(1))
    roots[p]=ks;return ks
bad=collections.defaultdict(list)
for f in [x for x in files if os.path.exists(x)]:
    t=open(f).read()
    m=re.search(r'^workspace\s*=\s*"([^"]+)"',t,re.M)
    if not m:continue
    rp=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1),'Cargo.toml'))
    ks=keys_of(rp)
    if ks is None:print("NOROOT",f[-100:],rp[-60:]);continue
    for l in t.split('\n'):
        mm=re.match(r'([A-Za-z0-9_\-]+)\s*=\s*\{[^}]*workspace\s*=\s*true',l) or re.match(r'([A-Za-z0-9_\-]+)\.workspace\s*=\s*true',l)
        if mm and mm.group(1) not in ks and mm.group(1) not in('version','edition','rust-version','license','authors','description','repository','homepage','readme','documentation','keywords','categories','publish'):bad[f].append(mm.group(1))
for f,l in bad.items():print(f[-110:],l)
print(len(bad),"manifests with unresolved workspace deps")
