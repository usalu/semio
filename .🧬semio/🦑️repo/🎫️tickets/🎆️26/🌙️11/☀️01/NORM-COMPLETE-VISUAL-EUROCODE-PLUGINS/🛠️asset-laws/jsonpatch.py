import sys,json,os,re,shutil
sys.path.insert(0,".")
src=open("patch.py").read().split("def patch(")[0]
sys.argv=["x","--none"]
exec(compile(src,"patch","exec"))
def camel(n):
    p=n.split("_");return p[0]+"".join(x[:1].upper()+x[1:] for x in p[1:])
def jdef(f,owner,structs,enums):
    t,x,opt=tag(f,structs,enums)
    o=OVR_ASSET.get((CTX["art"],CTX["asset"],f.name)) or OVR_FIELD.get((CTX["art"],f.name))
    d=DEFAULTS.get(owner,{}).get(f.name)
    if o is not None:
        return o if t=="TEXT" else (float(o) if t in("NUM","QTY") else o)
    if d is not None:
        v=lit(d,t,x,enums)
        if v is not None:
            if t=="TEXT": return v.strip('"')
            if t in("NUM","QTY"): return float(re.sub(r'[A-Za-z]+$','',v))
            if t in("UINT","INT"): return int(float(v))
            if t=="BOOL": return v=="true"
            if t=="ENUM": return camel(v.replace("-","_"))
            if t in("TABLE","LIST"): return []
    if opt: return None
    if t=="TEXT": return ""
    if t in("NUM","QTY"): return 0.0
    if t in("UINT","INT"): return 0
    if t=="BOOL": return False
    if t=="ENUM":
        return camel(pascal_kebab(enums[x][1]).replace("-","_"))
    if t in("TABLE","LIST"): return []
    if t=="REC": return {camel(g.name):jdef(g,x,structs,enums) for g in getstruct(structs,x)}
def fix(d,sname,structs,enums):
    fl=getstruct(structs,sname)
    out={}
    for f in fl:
        k=[k for k in d if norm(k)==norm(f.name)] if isinstance(d,dict) else []
        t,x,o=tag(f,structs,enums)
        if k:
            v=d[k[0]]
            if t in("TABLE","LIST") and x in structs and isinstance(v,list): v=[fix(e,x,structs,enums) for e in v]
            elif t=="REC" and isinstance(v,dict): v=fix(v,x,structs,enums)
            out[camel(f.name)]=v
        else: out[camel(f.name)]=jdef(f,sname,structs,enums)
    return out
for art in ["🪶️en1999"]:
    base=f"{R}/{art}/{SUB}"
    structs,enums=parse_rust(base)
    DEFAULTS.clear();DEFAULTS.update(parse_defaults(base))
    for p,fl in parse_rust(SHARED)[0].items(): structs.setdefault(p,[]).extend(fl)
    sch=schema_of(art)
    for root,ds,fs in os.walk(base+"/🖼️assets"):
        for f in fs:
            if f.endswith(".json"):
                p=os.path.join(root,f)
                CTX["art"]=re.search(r'en\d+',art).group(0);CTX["asset"]=re.sub(r'^[^a-z0-9]+','',p.split("🖼️assets/")[1].split("/")[0])
                old=open(p).read();new=json.dumps(fix(json.loads(old),sch["title"],structs,enums),indent=2,ensure_ascii=False)+("\n" if old.endswith("\n") else "")
                print(art,p.split("🖼️assets/")[1],"CHANGED" if json.loads(old)!=json.loads(new) or old!=new else "same")
                if "--apply" in sys.argv_orig if False else False: pass
                if os.environ.get("APPLY") and old!=new:
                    dst=os.path.join(BACKUP,art,p.split("🖼️assets/")[1]);os.makedirs(os.path.dirname(dst),exist_ok=True)
                    if not os.path.exists(dst): shutil.copy2(p,dst)
                    open(p,"w").write(new)
