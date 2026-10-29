import os,re,sys
R="/Users/ueli/Documents/semio"
apply="--apply" in sys.argv
def root_of(p):
    s=open(p,encoding="utf-8").read()
    m=re.search(r'^workspace\s*=\s*"([^"]+)"',s,re.M)
    if m:
        r=os.path.normpath(os.path.join(os.path.dirname(p),m.group(1),"Cargo.toml"));return r if os.path.exists(r) else None
    d=os.path.dirname(p)
    while len(d)>len(R):
        c=os.path.join(d,"Cargo.toml")
        if c!=p and os.path.exists(c) and re.search(r'^\[workspace\]',open(c,encoding="utf-8").read(),re.M):return c
        d=os.path.dirname(d)
def wsdeps(c):
    s=open(c,encoding="utf-8").read()
    h=set(re.findall(r'^\[workspace\.dependencies\.([^\]]+)\]',s,re.M))
    m=re.search(r'^\[workspace\.dependencies\]\n(.*?)(?=^\[|\Z)',s,re.M|re.S)
    if m:h|=set(re.findall(r'^([A-Za-z0-9_-]+)\s*=',m.group(1),re.M))
    return h
def defs():
    d={}
    for c in (R+"/🌎️hub/Cargo.toml",R+"/✏️s/Cargo.toml",R+"/🧰️framework/Cargo.toml"):
        if not os.path.exists(c):continue
        s=open(c,encoding="utf-8").read()
        for m in re.finditer(r'^\[workspace\.dependencies\.([^\]]+)\]\n(.*?)(?=^\[|\Z)',s,re.M|re.S):
            pm=re.search(r'^path\s*=\s*"([^"]+)"',m.group(2),re.M)
            if pm:d.setdefault(m.group(1),os.path.normpath(os.path.join(os.path.dirname(c),pm.group(1))))
        m=re.search(r'^\[workspace\.dependencies\]\n(.*?)(?=^\[|\Z)',s,re.M|re.S)
        if m:
            for k,pth in re.findall(r'^([A-Za-z0-9_-]+)\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"',m.group(1),re.M):d.setdefault(k,os.path.normpath(os.path.join(os.path.dirname(c),pth)))
    return d
D=defs();cache={};miss={}
for p in [l.rstrip(chr(10)) for l in open('/private/tmp/claude-501/today.txt')]:
    if True:
        p=os.path.join(R,p)
        s=open(p,encoding="utf-8").read()
        if re.match(r'\s*\[workspace\]',s) or "workspace = true" not in s:continue
        keys=set(re.findall(r'^([A-Za-z0-9_-]+)\s*=\s*\{\s*workspace\s*=\s*true',s,re.M))
        if not keys:continue
        r=root_of(p)
        if not r:continue
        if r not in cache:cache[r]=wsdeps(r)
        for k in keys-cache[r]:miss.setdefault(r,set()).add(k)
for r,ks in miss.items():
    for k in sorted(ks):
        if k in D:
            rel=os.path.relpath(D[k],os.path.dirname(r))
            print("ADD",r.replace(R+"/",""),k,rel[:60])
            if apply:
                with open(r,"a",encoding="utf-8") as f:f.write(f'\n[workspace.dependencies.{k}]\npath = "{rel}"\n')
        else:print("UNKNOWN",r.replace(R+"/",""),k)
