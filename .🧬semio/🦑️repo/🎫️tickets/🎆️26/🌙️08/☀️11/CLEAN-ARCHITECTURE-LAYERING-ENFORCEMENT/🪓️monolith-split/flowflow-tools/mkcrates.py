"""📦 Step 4: crate registry + Cargo.toml / project.json generation for the flow-flow carve. Work tree only (or the real tree with FF_APPLY=1)."""
import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ffed
from ffed import *
ART=FFREL
ORIGPKG=ART+"/📦️packages/🦀️rust"
FWK="🧰️framework/"
MOD="🧰️framework/🔨️modules/"
OSM="🧰️framework/🛍️products/💻️os/🔨️modules/"
OSK="🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
SPEC={
 "dp":('semio-framework-os-infinite-directed-presentation',"path",OSM+"♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/📦️packages/🦀️rust"),
 "pack":('semio-framework-pack',"path",MOD+"🎒️pack/📦️packages/🦀️rust"),
 "ref":('semio-framework-artifact-reference',"ws",None),
 "drd":('semio-framework-dsl-record-derive',"path",MOD+"🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"),
 "dr":('semio-framework-dsl-record',"path",MOD+"🗣️dsl/🧬️schema/📦️packages/🦀️rust"),
 "pj":('semio-framework-pack-json',"path",MOD+"🎒️pack/🔤️json/📦️packages/🦀️rust"),
 "diag":('semio-framework-diagnostic',"path",MOD+"⚠️diagnostic/📦️packages/🦀️rust"),
 "loc":('semio-framework-ui-locale',"path",MOD+"🖱️ui/🌐️locale/📦️packages/🦀️rust"),
 "ss":('semio-framework-schema-state',"path",MOD+"🧬️schema/📶️state/📦️packages/🦀️rust"),
 "job":('semio-framework-job',"ws",None),
 "async":('semio-framework-async',"ws",None),
 "val":('semio-framework-value',"path",MOD+"🌱️value/📦️packages/🦀️rust"),
 "kernel":('semio-framework-os-kernel',"ws",None),
 "vd":('semio-framework-value-derive',"ws",None),
 "rep":('semio-framework-replication',"ws",None),
 "serde":('serde','raw','{ version = "1", features = ["derive"] }'),
 "sj":('serde_json','raw','"1"'),
 "ne":('neural_engine',"rename",(OSM+"🧠️neural/⚙️engine/📦️packages/🦀️rust","semio-framework-os-kernel-neural-engine")),
 "gm":('semio-framework-graph-manifest',"path",MOD+"🕸️graph/🛂️manifest/📦️packages/🦀️rust"),
 "trace":('semio-framework-trace',"ws",None),
 "amacros":('semio-framework-async-macros',"ws",None),
}
class Crate: pass
def mk(cid,node,deps,dev=(),desc="",kernel_features=None,devkernel=None,libname=None,projname=None):
    c=Crate();c.id=cid;c.node=node;c.name="semio-framework-artifact-"+cid;c.project="@semio-tech/framework-"+cid+"-rs"
    c.pkgdir=ART+"/"+node+"/📦️packages/🦀️rust" if node else ORIGPKG
    c.lib=ART+"/"+node+"/🦀️.rs"
    c.deps=deps;c.dev=dev;c.desc=desc;c.devkernel=devkernel
    return c
CR={}
CR["flow-flow-model"]=mk("flow-flow-model","🧱️model",["dp","dr","drd","pj","val","kernel","vd","rep","serde","ne","gm"],["trace","sj"],"Flow document model: widgets, synapses, GUI projections, typed retirement and the component text codecs")
CR["flow-flow-document"]=mk("flow-flow-document","🧾️document",["flow-flow-model","pack","ref","dr","drd","pj","diag","loc","ss","async","val","kernel","vd","rep","serde","ne"],[],"Flow document: the persisted host snapshot, its mutation catalogue and the native text, binary and SQLite representations",devkernel='{ workspace = true, features = ["mutation-testing"] }')
CR["flow-flow-copy"]=mk("flow-flow-copy","🧵️retained/📑️copy",["flow-flow-model","flow-flow-document","job","val"],["pj","sj"],"Flow retained copy: selected typed scene copying under an immutable rooted borrow witness",devkernel=None)
CR["flow-flow-mutation-laws"]=mk("flow-flow-mutation-laws","🌿️vcs/⚖️laws",["flow-flow-document"],["pack","dr","pj","val","sj"],"Flow mutation catalogue laws: direct-leaf contracts, structural diff ownership and typed mutation retirement over the public document API",devkernel='{ workspace = true, features = ["mutation-testing"] }')
CR["flow-flow-snapshot-laws"]=mk("flow-flow-snapshot-laws","🚪️io/⚖️laws",["flow-flow-document"],["amacros","ref","pj","rep","val","sj"],"Flow snapshot representation laws: relational and native owner capability, package schema oracle and normal Store registration over the public document API",devkernel='{ workspace = true, features = ["mutation-testing"] }')
def relpath(frm,to):return os.path.relpath(REPOROOT+to,REPOROOT+frm)
def depline(c,key):
    if key in CR:
        o=CR[key];return f'{o.name} = {{ path = "{relpath(c.pkgdir,o.pkgdir)}" }}'
    name,kind,val=SPEC[key]
    if kind=="ws":return f"{name} = {{ workspace = true }}"
    if kind=="raw":return f"{name} = {val}"
    if kind=="path":return f'{name} = {{ path = "{relpath(c.pkgdir,val)}" }}'
    if kind=="rename":
        path,pkg=val;return f'{name} = {{ path = "{relpath(c.pkgdir,path)}", package = "{pkg}" }}'
    raise SystemExit(key)
def cargo(c):
    up=os.path.relpath(REPOROOT,REPOROOT+c.pkgdir)
    out=f'''[package]
workspace = "{up}"
name = "{c.name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{c.desc}"

[package.metadata.semio]
role = "artifact"
id = "{c.id}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
'''
    for k in c.deps:out+=depline(c,k)+"\n"
    out+="\n[dev-dependencies]\n"
    for k in c.dev:
        out+=depline(c,k)+"\n"
    if c.devkernel:out+=f"semio-framework-os-kernel = {c.devkernel}\n"
    return out
def project(c):
    return '''{
  "name": "%s",
  "projectType": "library",
  "tags": [
    "scope:framework",
    "language:rust"
  ]
}
''' % c.project
def emit(only=None):
    for cid,c in CR.items():
        if only and cid not in only:continue
        write(c.pkgdir[len(FFREL)+1:]+"/Cargo.toml",cargo(c))
        write(c.pkgdir[len(FFREL)+1:]+"/📋️project.json",project(c))
if __name__=="__main__":
    emit()
    for c in CR.values():print(c.name,c.project,c.pkgdir)
