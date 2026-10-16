"""🏛️ Facade API: the public names the old glob-exported modules exposed, located in the new crates. usage: facade.py [--write]"""
import os,re,sys,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from pubnames import names,mod_decls
from families import FL
from build_crates import CR
ORIG="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osflow/flow/"
OLDMODS={"catalogue":"🗂️catalogue/🦀️.rs","registry":"📔️registry/🦀️.rs","host":"🖥️host/🦀️.rs","drawing":"🖍️drawing/🦀️.rs","vcs":"🌿️vcs/🦀️.rs","extensions::wasm":"🧩️extensions/🕸️wasm/🦀️.rs"}
def old_names():
    out={}
    for m,rel in OLDMODS.items():
        out[m]={k:v for k,v in names(ORIG+rel).items() if not k.startswith("EXTGLOB")}
        eg=[k for k in names(ORIG+rel) if k.startswith("EXTGLOB")]
        if eg:out[m]["__extglobs"]=eg
    return out
ROOT={cid:FL+c.node+"/"+c.lib_rel for cid,c in CR.items()}
def new_names(cid,seen=None):
    """public namespace of a new crate root (resolving glob re-exports of sibling new crates)"""
    p=ROOT[cid]
    from pubnames import load
    n=names(p)
    res={k:v for k,v in n.items() if not k.startswith("EXTGLOB")}
    for k in n:
        if k.startswith("EXTGLOB:"):
            g=k[8:]
            first=g.split("::")[0]
            if first.startswith("semio_framework_os_flow_"):
                dep=first[len("semio_framework_os_flow_"):].replace("_","-")
                if dep in CR and len(g.split("::"))==1:
                    for kk,vv in new_names(dep).items():res.setdefault(kk,vv)
    return res
if __name__=="__main__":
    on=old_names()
    newn={cid:new_names(cid) for cid in CR}
    for m,ns in on.items():
        print("==",m,len(ns))
        loc={}
        for n in ns:
            if n=="__extglobs":print("   EXTGLOBS",ns[n]);continue
            found=[cid for cid in CR if n in newn[cid]]
            loc.setdefault(tuple(found),[]).append(n)
        for k,v in loc.items():print("  ",k,len(v),sorted(v)[:12])
