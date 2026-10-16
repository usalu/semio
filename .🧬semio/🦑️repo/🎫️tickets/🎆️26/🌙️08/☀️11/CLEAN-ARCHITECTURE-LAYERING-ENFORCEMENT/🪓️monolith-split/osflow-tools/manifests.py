"""📦️ Dependency resolution for the new os-flow crates: idents -> old manifest specs, rebased relative paths."""
import json,os,re,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
REPO="/Users/ueli/Documents/semio/"
OSM=REPO+"🧰️framework/🛍️products/💻️os/🔨️modules/"
FAM=os.environ.get("OSFAM","flow")
FAMS={"flow":(OSM+"🌊️flow/","oldmanifest.json","semio-framework-os-flow-","os-flow-"),
 "workflow":(OSM+"🔁️workflow/🗿️artifacts/🔁️workflow/","oldmanifest-workflow.json","semio-framework-artifact-workflow-workflow-","workflow-workflow-"),
 "flowflow":(OSM+"🌊️flow/🗿️artifacts/🌊️flow/","oldmanifest-flowflow.json","semio-framework-artifact-flow-flow-","flow-flow-")}
FL,_OLDFILE,NAMEPRE,IDPRE=FAMS[FAM]
PROJ={"flow":lambda cid:NAMEPRE+cid,"workflow":lambda cid:f"@semio-tech/framework-workflow-workflow-{cid}-rs","flowflow":lambda cid:f"@semio-tech/framework-flow-flow-{cid}-rs"}[FAM]
OLDPKG=FL+"📦️packages/🦀️rust/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osflow/"
T=os.path.dirname(os.path.abspath(__file__))
old=json.load(open(S+_OLDFILE))
root=json.load(open(S+"rootcargo.json"))
WS=root["workspace"]["dependencies"]
_cache={}
def tomljson(path):
    if path in _cache:return _cache[path]
    out=subprocess.run(["bun",T+"/tomljson.ts",path],capture_output=True,text=True)
    d=json.loads(out.stdout) if out.returncode==0 and out.stdout.strip() else {}
    _cache[path]=d;return d
def libname(depdir,key,spec):
    if "package" in spec:return key.replace("-","_")
    m=tomljson(depdir+"/Cargo.toml") if os.path.exists(depdir+"/Cargo.toml") else {}
    ln=(m.get("lib") or {}).get("name")
    pk=(m.get("package") or {}).get("name",key)
    return (ln or pk).replace("-","_")
def collect():
    """list of dict(key, spec, abs, ident, where, cfg)"""
    out=[]
    def add(tbl,where,cfg):
        for key,spec in (tbl or {}).items():
            sp={"version":spec} if isinstance(spec,str) else dict(spec)
            ab=None
            if sp.get("workspace") is True:
                w=WS.get(sp.get("package",key),{})
                w={"version":w} if isinstance(w,str) else w
                if "path" in w:ab=os.path.normpath(REPO+w["path"])
                base=dict(w);base.pop("path",None)
                sp2=dict(sp);sp2.pop("workspace");base.update(sp2)
                sp_res=base;sp_res["_ws"]=True
            else:
                sp_res=sp
                if "path" in sp:ab=os.path.normpath(os.path.join(OLDPKG,sp["path"]))
            ident=libname(ab,key,sp) if ab else key.replace("-","_")
            out.append(dict(key=key,spec=sp,res=sp_res,abs=ab,ident=ident,where=where,cfg=cfg))
    add(old.get("dependencies"),"dep",None)
    add(old.get("dev-dependencies"),"dev",None)
    for cfg,t in (old.get("target") or {}).items():
        add(t.get("dependencies"),"dep",cfg)
        add(t.get("dev-dependencies"),"dev",cfg)
    return out
DEPS=collect()
def idents_used(paths):
    used=set()
    for p in paths:
        t=open(p,encoding="utf-8").read();L=[l+"\n" for l in t.split("\n")][:-1];I=lex2(L)
        for x,info in enumerate(I):
            line=L[x]
            for n,c in info["toks"]:
                pre=line[:c].rstrip()
                if pre.endswith(".") and not pre.endswith(".."):continue
                if pre.endswith("::") and not pre.endswith("crate::") and True:
                    before=pre[:-2]
                    if not (before=="" or before[-1] in " \t,({[<=&|!;>" or before.endswith("derive")):
                        continue
                used.add(n)
            for st in info["strs"]:
                pass
    return used
def rel(frompkg,ab):
    return os.path.relpath(ab,frompkg)
def depline(d,pkgdir,workspace_ok=True):
    sp=dict(d["res"])
    ws=sp.pop("_ws",False)
    parts=[]
    if d["abs"] and d["spec"].get("workspace") is not True:
        sp["path"]=rel(pkgdir,d["abs"])
    elif d["abs"]:
        pass
    def fmt(v):
        if isinstance(v,bool):return "true" if v else "false"
        if isinstance(v,str):return json.dumps(v,ensure_ascii=False)
        if isinstance(v,list):return "["+", ".join(fmt(x) for x in v)+"]"
        raise ValueError(v)
    if d["spec"].get("workspace") is True:
        items=[("workspace","true")]
        if "features" in d["spec"]:items.append(("features",fmt(d["spec"]["features"])))
        if "optional" in d["spec"]:items.append(("optional",fmt(d["spec"]["optional"])))
        if "package" in d["spec"]:items.append(("package",fmt(d["spec"]["package"])))
        return f"{d['key']} = {{ "+", ".join(f"{k} = {v}" for k,v in items)+" }"
    keys=[k for k in ["path","version","package","features","optional","default-features"] if k in sp]
    if keys==["version"]:return f"{d['key']} = {fmt(sp['version'])}"
    return f"{d['key']} = {{ "+", ".join(f"{k} = {fmt(sp[k])}" for k in keys)+" }"
if __name__=="__main__":
    for d in DEPS:print(d["where"],d["cfg"] or "-",d["key"],"->",d["ident"],d["abs"] and os.path.relpath(d["abs"],REPO))
