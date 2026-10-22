import os,sys,json
sys.path.insert(0,"/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad")
from mkdeps import dep,S,R
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
def manifest(pkgdir,name,id_,desc,libpath,names):
    ups=os.path.relpath(R,pkgdir)
    lines=[]
    for n in names:
        t,rest=dep[n]
        if t=="WS": lines.append(f'{n} = {{ workspace = true }}')
        else: lines.append(f'{n} = {{ path = "{os.path.relpath(t,pkgdir)}" }}')
    return f'''[package]
workspace = "{ups}"
name = "{name}"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "{desc}"

[package.metadata.semio]
role = "framework"
id = "{id_}"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "{libpath}"

[dependencies]
'''+"\n".join(lines)+"\n"
MG=S+"/🧩️composition/📬️publication/🤝️group/📦️packages/🦀️rust"
CL=S+"/⚙️engine/🧯️close/📦️packages/🦀️rust"
pre="semio-framework-"
mg=manifest(MG,"semio-framework-os-store-engine-member-group","os-store-engine-member-group","Common-decision publication of one store's member group over the StoreParts bundle","../../🦀️.rs",
 [pre+x for x in ["os-spr","os-store-one-item-batch","os-store-read","os-store-ledger","os-store-one-item","os-store-one-item-publication","os-store-text-io","os-vcs","value","pack","replication","os-store-engine-stage","os-store-durable-group-wire","os-store-engine-live"]])
cl=manifest(CL,"semio-framework-os-store-engine-close","os-store-engine-close","Close witnesses and cursor admission over the StoreParts bundle","../../🦀️.rs",
 [pre+x for x in ["os-spr","os-store-read","os-store-ledger","os-store-replay","os-vcs","value","replication","os-store-engine-live"]])
pj=lambda id_:'{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'%id_
files={MG+"/Cargo.toml":mg,MG+"/📋️project.json":pj("os-store-engine-member-group"),CL+"/Cargo.toml":cl,CL+"/📋️project.json":pj("os-store-engine-close")}
for k,v in files.items():
    if apply:
        os.makedirs(os.path.dirname(k),exist_ok=True); open(k,"w",encoding='utf8').write(v)
    else: print(k.replace(R,""));print(v)
