A="semio-framework-value-testing-allocator"
import tomllib,subprocess,os,re,sys,json
root=os.getcwd()
tomls=subprocess.run(["find","✏️s","🧰️framework","🌎️hub","🧪️tests","🧫️fixtures","-name","Cargo.toml","-not","-path","*/target/*","-not","-path","*/node_modules/*","-not","-path","*/.cache/*"],capture_output=True,text=True).stdout.split("\n")
pk={}
for t in tomls:
    if not t:continue
    try:d=tomllib.load(open(t,"rb"))
    except Exception:continue
    if "package" not in d:continue
    n=d["package"]["name"]
    pk[n]=(os.path.abspath(t),d)
wscache={}
def wsdeps(p,d):
    w=d["package"].get("workspace")
    if not isinstance(w,str):return {}
    r=os.path.normpath(os.path.join(os.path.dirname(p),w,"Cargo.toml"))
    if r not in wscache:
        try:wscache[r]=tomllib.load(open(r,"rb")).get("workspace",{}).get("dependencies",{})
        except Exception:wscache[r]={}
    return wscache[r]
def spec(k,v,ws):
    if isinstance(v,dict) and v.get("workspace"):
        b=ws.get(k,{});b={"version":b} if isinstance(b,str) else dict(b)
        feats=list(b.get("features",[]))+list(v.get("features",[]))
        b["features"]=feats;b["optional"]=v.get("optional",False)
        return b
    return {"version":v} if isinstance(v,str) else v
def deps_of(d,dev,p=None):
    ws=wsdeps(p,d) if p else {}
    out=[]
    secs=["dependencies"]+(["dev-dependencies"] if dev else [])
    for tb in [d]+list(d.get("target",{}).values()):
        for s in secs:
            for k,v in tb.get(s,{}).items():
                v=spec(k,v,ws)
                out.append((v.get("package",k),v.get("optional",False),v.get("features",[]),v.get("default-features",True),k,s=="dev-dependencies"))
    return out
edges={}
def resolve(root):
    on={};work=[(root,"default",True)];seen=set()
    def want(n,f):
        if (n,f) not in seen:seen.add((n,f));work.append((n,f,False))
    seen.add((root,"default"))
    on.setdefault(root,set())
    while work:
        n,f,isroot=work.pop()
        if n not in pk:continue
        p,d=pk[n];on.setdefault(n,set())
        fe=d.get("features",{})
        if f!="" and f not in on[n] :
            on[n].add(f)
        todo=[]
        if f in fe:todo=fe[f]
        for it in todo:
            if it.startswith("dep:"):
                dn=it[4:];want_dep(n,dn,None,work,seen,on);continue
            if "/" in it:
                dn,ff=it.split("/",1);dn=dn.rstrip("?")
                want_dep(n,dn,ff,work,seen,on);continue
            if it in fe:want(n,it)
            else:want_dep(n,it,None,work,seen,on)
        if f=="default" and f not in fe:pass
        for (dn,opt,feats,dflt,key,isdev) in deps_of(d,isroot and True,p):
            if isdev and not isroot:continue
            if opt and not ((n,"dep:"+key) in seen or (n,"dep:"+dn) in seen):continue
            if (dn,"@") in seen and False:pass
            seen.add((n,"edge:"+key))
            if dn not in pk:continue
            if dflt:
                if (dn,"default") not in seen:seen.add((dn,"default"));work.append((dn,"default",False))
            for ff in feats:
                if (dn,ff) not in seen:seen.add((dn,ff));work.append((dn,ff,False))
            on.setdefault(dn,set())
            edges.setdefault(n,set()).add(dn)
    return on
def want_dep(n,dn,ff,work,seen,on):
    seen.add((n,"dep:"+dn))
    for (m,opt,feats,dflt,key,isdev) in deps_of(pk[n][1],False,pk[n][0]):
        if key==dn or m==dn:
            seen.add((n,"dep:"+key));seen.add((n,"dep:"+m))
            work.append((n,"",False))
            if ff and (m,ff) not in seen:seen.add((m,ff));work.append((m,ff,False))
            if (m,"default") not in seen and dflt:seen.add((m,"default"));work.append((m,"default",False))
            for f2 in feats:
                if (m,f2) not in seen:seen.add((m,f2));work.append((m,f2,False))
            edges.setdefault(n,set()).add(m)
def targets(p,d):
    base=os.path.dirname(p);out=[]
    l=d.get("lib",{})
    if "path" in l:out.append(l["path"])
    else:out.append("src/lib.rs")
    for k in("bin","test","example","bench"):
        for t in d.get(k,[]):
            if "path" in t:out.append(t["path"])
    return [os.path.normpath(os.path.join(base,x)) for x in out]
pat=re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
inc=re.compile(r'\binclude!\(\s*"([^"]+\.rs)"\s*\)')
files=[os.path.abspath(f) for f in subprocess.run(["grep","-rl","--include=*.rs","global_allocator","✏️s","🧰️framework","🌎️hub","🧪️tests","🧫️fixtures","--exclude-dir=target","--exclude-dir=.cache","--exclude-dir=node_modules"],capture_output=True,text=True).stdout.split("\n") if f]
owner={}
for n,(p,d) in pk.items():
    stack=targets(p,d);seen=set()
    while stack:
        f=stack.pop()
        if f in seen or not os.path.isfile(f):continue
        seen.add(f);owner.setdefault(f,set()).add(n)
        s=open(f,errors="ignore").read()
        for q in pat.findall(s)+inc.findall(s):stack.append(os.path.normpath(os.path.join(os.path.dirname(f),q)))
for f in files:
    rel=os.path.relpath(f,root)
    ow=owner.get(f)
    if not ow:print("UNMAPPED",rel);continue
    for n in sorted(ow):
        on=resolve(n)
        print("CONFLICT" if (A in on or n==A) else "ok",n,rel)
