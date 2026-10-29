import json,os,re,sys
R="/Users/ueli/Documents/semio"
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
IX=json.load(open(os.path.join(R,C,"1029-tools/policy/crateindex.json")))
CACHE={}
def _read(p):
    if p not in CACHE: CACHE[p]=open(p,encoding="utf-8").read()
    return CACHE[p]
def _write(p,t): CACHE[p]=t
def flush():
    for p,t in CACHE.items(): open(p,"w",encoding="utf-8").write(t)
    print("flushed",len(CACHE))
def sub(path,old,new,count=1):
    p=os.path.join(R,path)
    t=_read(p)
    n=t.count(old)
    assert n==count,(path[-60:],old[:90],n,count)
    _write(p,t.replace(old,new))
def subre(path,pattern,new,count=None):
    p=os.path.join(R,path)
    t=_read(p)
    t2,n=re.subn(pattern,new,t)
    assert n>0 and (count is None or n==count),(path[-60:],pattern,n,count)
    _write(p,t2)
    return n
def add_dep(crate,dep):
    d=IX[crate]; tgt=IX[dep]
    p=os.path.join(d,"Cargo.toml")
    t=_read(p)
    if re.search(r'^'+re.escape(dep)+r'\s*=',t,re.M): return "has"
    rel=os.path.relpath(tgt,d)
    line=f'{dep} = {{ path = "{rel}" }}\n'
    lines=t.split("\n")
    start=None
    for i,l in enumerate(lines):
        if l.strip()=="[dependencies]": start=i;break
    assert start is not None,(crate,"no [dependencies]")
    end=len(lines)
    for j in range(start+1,len(lines)):
        if lines[j].startswith("["): end=j;break
    last=None
    for j in range(start+1,end):
        if re.match(r'^[A-Za-z0-9_\-]+\s*=\s*\{.*path\s*=',lines[j]) or re.match(r'^[A-Za-z0-9_\-]+\s*=\s*\{\s*workspace',lines[j]): last=j
    if last is None: last=start
    lines.insert(last+1,line.rstrip("\n"))
    _write(p,"\n".join(lines))
    return "added"
