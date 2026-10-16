"""🔗 Dependency idents of a crate manifest and which closure files mention them (analysis only)."""
import os,re,subprocess,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from common import *
import toml as tomllib
ROOT_TOML=tomllib.load(REPO+"Cargo.toml")
WSDEPS=ROOT_TOML.get("workspace",{}).get("dependencies",{})
def dep_ident(pkgdir,key,spec):
    if isinstance(spec,str):spec={}
    if spec.get("workspace"):
        base=WSDEPS.get(key,{})
        if isinstance(base,str):base={}
        path=base.get("path");pkg=base.get("package",key);rel=REPO
        spec2=dict(base);spec2["package"]=base.get("package")
        spec=spec2;pkgdir=REPO
    else:path=spec.get("path");pkg=spec.get("package")
    if pkg is not None and pkg!=key:return key.replace("-","_")
    name=key
    if path:
        m=os.path.join(pkgdir,path,"Cargo.toml")
        if os.path.exists(m):
            t=tomllib.load(m)
            name=t.get("lib",{}).get("name") or t["package"]["name"]
    return name.replace("-","_")
def idents(manifest):
    d=os.path.dirname(manifest);t=tomllib.load(manifest)
    out={}
    def take(tbl,kind):
        for k,v in (tbl or {}).items():out[dep_ident(d,k,v)]=(k,kind)
    take(t.get("dependencies"),"dep");take(t.get("dev-dependencies"),"dev");take(t.get("build-dependencies"),"build")
    for tgt,body in (t.get("target") or {}).items():
        take(body.get("dependencies"),"dep:"+tgt);take(body.get("dev-dependencies"),"dev:"+tgt)
    return out
def used(files):
    from rlex2 import lex2
    toks=set()
    for f in files:
        t=open(f if os.path.isabs(f) else REPO+f,encoding="utf-8",errors="ignore").read()
        L=[l+"\n" for l in t.split("\n")]
        for info in lex2(L):
            for n,_ in info["toks"]:toks.add(n)
    return toks
if __name__=="__main__":
    man=sys.argv[1]
    d=idents(man)
    for i,(k,kind) in sorted(d.items()):print(i,k,kind)
