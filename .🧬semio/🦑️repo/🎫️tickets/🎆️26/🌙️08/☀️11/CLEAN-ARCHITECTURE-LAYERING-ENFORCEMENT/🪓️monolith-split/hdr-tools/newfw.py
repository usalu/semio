import json,os,re,sys
# 🏗️ newfw.py <spec.json> [--write] : creates one new framework producer crate and registers it (Cargo.toml, 📋️project.json, 🧰️framework members + workspace dependency, build owner row, six launch routes).
R="/Users/ueli/Documents/semio"; FW=R+"/🧰️framework"; PKG="📦️packages/🦀️rust"
spec=json.load(open(sys.argv[1])); write="--write" in sys.argv
name=spec["pkg"]; ident=spec["id"]; node=spec["dir"]; parent=spec["parent"]
pkgdir=f"{R}/{node}/{PKG}"
assert len(pkgdir.encode())<240+len(R)
depth=os.path.relpath(pkgdir,FW).count("/")+1
ws="/".join([".."]*depth)
def rel(to_abs): return os.path.relpath(to_abs,pkgdir)
def dep(d):
    if isinstance(d,str): return d
    return f'{d["name"]} = {{ path = "{rel(R+"/"+d["path"]+"/"+PKG)}" }}'
def srt(ds): return sorted(ds,key=lambda s:s.split(" ")[0].split(".")[0])
deps=srt([dep(d) for d in spec["deps"]]); dev=srt([dep(d) for d in spec.get("dev",[])])
libpath=spec.get("lib","../../🦀️.rs")
manifest=(f'[package]\nworkspace = "{ws}"\nname = "{name}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "{spec["desc"]}"\n\n'
 f'[package.metadata.semio]\nrole = "{spec.get("role","framework")}"\nid = "{ident}"\n\n[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true\n\n[lib]\npath = "{libpath}"\n\n'
 +("[features]\n"+"".join(f+"\n" for f in spec["features"])+"\n" if spec.get("features") else "")
 +"[dependencies]\n"+"".join(d+"\n" for d in deps)+"\n[dev-dependencies]\n"+"".join(d+"\n" for d in dev))
for extra in spec.get("extra_manifest",[]): manifest+=extra+"\n"
project='{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'%ident
# workspace edits
ftxt=open(FW+"/Cargo.toml").read()
pnode=spec["parent_node"]
member_parent=f'    "{pnode.replace("🧰️framework/","",1)}/{PKG}",\n'
member_new=f'    "{node.replace("🧰️framework/","",1)}/{PKG}",\n'
assert ftxt.count(member_parent)==1,("parent member",member_parent)
assert member_new not in ftxt
ftxt=ftxt.replace(member_parent,member_parent+member_new)
if spec.get("workspace_dep",True):
    pdep=re.search(r'    \[workspace\.dependencies\.%s\]\n    path = "[^"]+"\n'%re.escape(parent),ftxt)
    assert pdep,"parent dependency table"
    newdep=f'\n    [workspace.dependencies.{name}]\n    path = "{node.replace("🧰️framework/","",1)}/{PKG}"\n'
    assert f"[workspace.dependencies.{name}]" not in ftxt
    ftxt=ftxt[:pdep.end()]+newdep+ftxt[pdep.end():]
# owners
otxt=open(R+"/📋️build-owners.json").read()
row=f'    {{\n      "name": "@semio-tech/{ident}-rs",\n      "root": "{node}/{PKG}",\n      "manifest": "{node}/{PKG}/📋️project.json",\n      "packages": []\n    }},\n'
names=[(m.start(),m.group(1)) for m in re.finditer(r'    \{\n      "name": "(@semio-tech/[^"]+)"',otxt)]
assert f'"@semio-tech/{ident}-rs"' not in otxt
pos=next((p for p,n in names if n>f"@semio-tech/{ident}-rs"),None)
assert pos is not None
otxt=otxt[:pos]+row+otxt[pos:]
# launch
ltxt=open(R+"/.vscode/launch.json").read()
k=spec["route_index"]
def route(label,target,conf,group,order):
    return (f'    {{\n      "name": "🦀️ {name} {label}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/{ident}-rs:{target} --configuration={conf} --parallel=1 -- --lib --offline",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {order}\n      }}\n    }},\n')
a,b=2*k-1,2*k
routes=("".join([route("🏗️build 🛠️prototype","build","prototype","4_build",f"900.96{a:03d}"),route("🏗️build 🚀️release","build","release","4_build",f"900.96{b:03d}"),
 route("🔎️check 🛠️prototype","check","prototype","4_gate",f"922.96{a:03d}"),route("🔎️check 🚀️release","check","release","4_gate",f"922.96{b:03d}"),
 route("🧪️test 🛠️prototype","test","prototype","4_test",f"903.96{a:03d}"),route("🧪️test 🚀️release","test","release","4_test",f"903.96{b:03d}")]))
for o in (f"900.96{a:03d}",f"922.96{a:03d}",f"903.96{a:03d}",f"900.96{b:03d}"): assert f'"order": {o}\n' not in ltxt,("order used",o)
assert f'"🦀️ {name} ' not in ltxt
last=[m.end() for m in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n'%re.escape(spec.get('route_after',parent)),ltxt)]
assert last,"parent routes"
ins=last[-1]
ltxt=ltxt[:ins]+routes+ltxt[ins:]
json.loads(ltxt.replace("//","//")) if False else None
print(f"{name}: pkgdir {len(pkgdir.encode())-len(R)-1} bytes; workspace {ws}; routes {k}")
if write:
    assert not os.path.exists(pkgdir)
    os.makedirs(pkgdir)
    open(pkgdir+"/Cargo.toml","w").write(manifest); open(pkgdir+"/📋️project.json","w").write(project)
    open(FW+"/Cargo.toml","w").write(ftxt); open(R+"/📋️build-owners.json","w").write(otxt); open(R+"/.vscode/launch.json","w").write(ltxt)
    json.loads(otxt)
    print("written")
else:
    print(manifest)
