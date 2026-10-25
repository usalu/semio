#!/usr/bin/env python3
"""usage: modscan2.py LISTFILE [--apply]  Walks the #[path] module tree of every [lib]/[[test]]/[[bench]]/[[example]] target of the listed Cargo.toml files,
compares the referenced semio_* crates with the declared [dependencies]/[dev-dependencies]/[build-dependencies], reports (and with --apply adds) the missing path dependencies."""
import os,re,sys,subprocess
os.chdir("/Users/ueli/Documents/semio")
LIST=sys.argv[1]; APPLY="--apply" in sys.argv
NAMES={}
for root in ("🧰️framework","✏️s"):
    for cf in subprocess.run(["rg","--files","-g","Cargo.toml",root],capture_output=True,text=True).stdout.split("\n"):
        if not cf: continue
        try: t=open(cf,encoding="utf-8").read()
        except: continue
        if "[package]" not in t: continue
        m=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M)
        if m: NAMES.setdefault(m.group(1).replace("-","_"),(m.group(1),os.path.dirname(cf)))
MOD=re.compile(r'((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)')
def strip(s):
    s=re.sub(r'/\*.*?\*/','',s,flags=re.S)
    return re.sub(r'(?m)(^|[^:"\w])//.*','\\1',s)
def walk(f,seen):
    f=os.path.normpath(f)
    if f in seen or not os.path.isfile(f): return
    seen.add(f)
    try: c=strip(open(f,encoding="utf-8").read())
    except: return
    base=os.path.dirname(f)
    events=list(MOD.finditer(c)); d=0; last=0; inl=[]
    for m in events:
        d+=c[last:m.start()].count('{')-c[last:m.start()].count('}'); last=m.start()
        while inl and d<=inl[-1][1]: inl.pop()
        cur=inl[-1][0] if inl else base
        pm=re.search(r'#\[path\s*=\s*"([^"]*)"\]',m.group(1)); path=pm.group(1) if pm else None
        name,kind=m.group(2),m.group(3)
        if kind==';':
            if path is not None: walk(os.path.join(cur,path),seen)
            else: walk(os.path.join(cur,name+".rs"),seen); walk(os.path.join(cur,name,"mod.rs"),seen)
        else: inl.append((os.path.join(cur,path) if path is not None else os.path.join(cur,name),d))
def used(f):
    s=strip(open(f,encoding="utf-8").read())
    return set(re.findall(r'(?<![\w:])(semio_[a-z0-9_]+)(?=::)',s))|set(re.findall(r'(?<![\w:])::(semio_[a-z0-9_]+)(?=::)',s))|set(re.findall(r'extern crate (semio_[a-z0-9_]+)',s))
def targets(cf,t):
    out=[]
    m=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"',t,re.S)
    out.append(("lib",os.path.join(os.path.dirname(cf),m.group(1) if m else "src/lib.rs")))
    for kind in ("test","bench","example","bin"):
        for blk in re.finditer(r'\[\['+kind+r'\]\](.*?)(?=\n\[|\Z)',t,re.S):
            p=re.search(r'path\s*=\s*"([^"]+)"',blk.group(1))
            if p: out.append((kind,os.path.join(os.path.dirname(cf),p.group(1))))
    return out
def add(cf,t,section,name,rel):
    line=f'{name} = {{ path = "{rel}" }}'
    lines=t.split("\n")
    hdr="["+section+"]"
    if hdr in lines:
        i=lines.index(hdr); j=i+1
        while j<len(lines) and not lines[j].startswith("["): j+=1
        k=j
        while k>i+1 and lines[k-1].strip()=="": k-=1
        ins=k
        for q in range(i+1,k):
            nm=lines[q].split("=")[0].strip()
            if nm and not nm.startswith("#") and nm>name: ins=q; break
        lines.insert(ins,line)
    else: lines+=["",hdr,line]
    return "\n".join(lines)
tot=0
for cf in [l.strip() for l in open(LIST) if l.strip()]:
    if cf.startswith("./"): cf=cf[2:]
    try: t=open(cf,encoding="utf-8").read()
    except: continue
    if "[package]" not in t: continue
    nm=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M).group(1)
    declared=set()
    for m in re.finditer(r'^([\w-]+)\s*(?:=|\.)',t,re.M): declared.add(m.group(1).replace("-","_"))
    for p in re.findall(r'package\s*=\s*"([^"]+)"',t): declared.add(p.replace("-","_"))
    declared.add(nm.replace("-","_"))
    miss={}
    for kind,tp in targets(cf,t):
        seen=set(); walk(tp,seen)
        for f in seen:
            for c in used(f):
                if c not in declared: miss.setdefault(c,[]).append((kind,f))
    if not miss: continue
    print(cf[-100:])
    for c,fs in sorted(miss.items()):
        known=c in NAMES and (c.startswith('semio_framework_') or '--all' in sys.argv)
        onlytest=all(("/🧪️tests/" in f or k!="lib") for k,f in fs)
        print("   ",c,len(fs),"dev" if onlytest else "dep","" if known else "UNKNOWN",min(f for k,f in fs)[-70:])
        if APPLY and known:
            pn,pd=NAMES[c]
            rel=os.path.relpath(pd,os.path.dirname(cf))
            t=add(cf,t,"dev-dependencies" if onlytest else "dependencies",pn,rel); tot+=1
            declared.add(c)
    if APPLY: open(cf,"w",encoding="utf-8").write(t)
print("added",tot)
