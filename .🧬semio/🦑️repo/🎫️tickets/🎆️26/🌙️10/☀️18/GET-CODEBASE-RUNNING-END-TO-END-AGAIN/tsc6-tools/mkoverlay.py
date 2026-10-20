import json,os,re,sys
R="/Users/ueli/Documents/semio"
def lenient(p):
    s=open(p,encoding="utf8").read()
    s=re.sub(r'(?m)^\s*//.*$',"",s)
    s=re.sub(r',(\s*[}\]])',r'\1',s)
    return json.loads(s)
def chain_paths(cfg):
    cfg=os.path.abspath(cfg)
    while True:
        j=lenient(cfg)
        co=j.get("compilerOptions",{})
        if "paths" in co:
            base=os.path.join(os.path.dirname(cfg),co.get("baseUrl","."))
            return {k:[os.path.normpath(os.path.join(base,t)) for t in v] for k,v in co["paths"].items()}
        ext=j.get("extends")
        if not ext: return {}
        cfg=os.path.normpath(os.path.join(os.path.dirname(cfg),ext))
def pick(v):
    if isinstance(v,str): return v
    if isinstance(v,dict):
        for k in ("types","bun","import","default","development","node"):
            if k in v:
                r=pick(v[k])
                if r: return r
    if isinstance(v,list):
        for x in v:
            r=pick(x)
            if r: return r
def unlinked():
    out={}
    for base in [".","✏️s","♻️mit-bestand","🌎️hub","🎓️teaching"]:
        top=os.path.join(R,base,"package.json")
        if not os.path.exists(top): continue
        ws=json.load(open(top)).get("workspaces")
        if not isinstance(ws,list): continue
        nm=os.path.join(R,base,"node_modules")
        for w in ws:
            d=os.path.join(R,base,w); pj=os.path.join(d,"package.json")
            if not os.path.exists(pj): continue
            j=json.load(open(pj)); n=j.get("name")
            if not n or os.path.exists(os.path.join(nm,n)) or n in out: continue
            ex=j.get("exports") or {".":j.get("main","./index.js")}
            m={}
            for k,v in ex.items():
                t=pick(v)
                if not t: continue
                key=n if k=="." else n+k[1:]
                m[key]=[os.path.normpath(os.path.join(d,t))]
            out[n]=m
    return out
if __name__=="__main__":
    cfg,out=sys.argv[1],sys.argv[2]
    paths=chain_paths(cfg)
    for n,m in unlinked().items():
        for k,v in m.items(): paths.setdefault(k,v)
    json.dump({"extends":os.path.abspath(cfg),"compilerOptions":{"paths":paths}},open(out,"w"),ensure_ascii=False,indent=1)
    print(len(paths),"paths",len(unlinked()),"unlinked packages")
