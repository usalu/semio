"""🏗️ Crate generator helpers: manifests, project files, registry rows (writes only into the repo tree the carve owns)."""
import json,os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from manifests import *
PKG="📦️packages/🦀️rust"
class Crate:
    def __init__(s,cid,node,desc,role="product",lib_rel="🦀️.rs"):
        s.id=cid;s.node=node;s.desc=desc;s.role=role;s.lib_rel=lib_rel
        s.name=NAMEPRE+cid;s.project=PROJ(cid)
        s.pkgdir=FL+node+"/"+PKG
        s.deps=[];s.devdeps=[];s.internal=[];s.idev=[];s.targetdeps={};s.features={}
    def depth(s):
        return len(os.path.relpath(s.pkgdir,REPO).split(os.sep))
def fmtval(v):
    if isinstance(v,bool):return "true" if v else "false"
    if isinstance(v,str):return json.dumps(v,ensure_ascii=False)
    if isinstance(v,list):return "["+", ".join(fmtval(x) for x in v)+"]"
    raise ValueError(v)
def old_dep(key,where="dep",cfg=None):
    for d in DEPS:
        if d["key"]==key and d["where"]==where and d["cfg"]==cfg:return d
    raise KeyError((key,where,cfg))
def manifest_text(c:Crate,libname=None):
    out=["[package]",f'workspace = "{"../"*c.depth()}"'.replace('/"','"'),f'name = "{c.name}"',*(['version.workspace = true','edition.workspace = true'] if (old.get("package",{}).get("version") or {}) == {"workspace":True} else ['version = "0.1.0"','edition = "2021"']),'rust-version.workspace = true',f'description = {json.dumps(c.desc,ensure_ascii=False)}',"",
    "[package.metadata.semio]",f'role = "{c.role}"',f'id = "{IDPRE}{c.id}"',"",
    "[package.metadata.semio.rust-source-boundary]","maxSourceBytes = 262144","maxSourceFiles = 32","maxActiveLines = 4000","",
    "[lints]","workspace = true","","[lib]"]
    if libname:out.append(f'name = "{libname}"')
    out.append(f'path = "{c.lib_rel_from_pkg()}"')
    out.append("")
    def block(header,items):
        if not items:return []
        return [header]+items+[""]
    normal=[depline(d,c.pkgdir) for d in c.deps if d["cfg"] is None]
    internal=[f'{n} = {{ path = {json.dumps(os.path.relpath(p,c.pkgdir),ensure_ascii=False)} }}' for n,p in c.internal]
    out+= ["[dependencies]"]+internal+normal+[""]
    dev=[f'{n} = {{ path = {json.dumps(os.path.relpath(p,c.pkgdir),ensure_ascii=False)} }}' for n,p in c.idev]+[depline(d,c.pkgdir) for d in c.devdeps]
    if dev:out+=["[dev-dependencies]"]+dev+[""]
    byc={}
    for d in c.deps:
        if d["cfg"]:byc.setdefault(d["cfg"],[]).append(depline(d,c.pkgdir))
    for cfg,items in byc.items():
        out+= [f"[target.'{cfg}'.dependencies]"]+items+[""]
    for sec,items in c.features.items():
        pass
    return "\n".join(out).rstrip("\n")+"\n"
def _lib_rel_from_pkg(self):
    return os.path.relpath(FL+self.node+"/"+self.lib_rel,self.pkgdir)
Crate.lib_rel_from_pkg=_lib_rel_from_pkg
def write_crate(c:Crate,libname=None):
    os.makedirs(c.pkgdir,exist_ok=True)
    open(c.pkgdir+"/Cargo.toml","w",encoding="utf-8").write(manifest_text(c,libname))
    pj={"name":c.project,"projectType":"library","tags":["scope:framework","language:rust"]}
    open(c.pkgdir+"/📋️project.json","w",encoding="utf-8").write(json.dumps(pj,ensure_ascii=False,indent=2)+"\n")
