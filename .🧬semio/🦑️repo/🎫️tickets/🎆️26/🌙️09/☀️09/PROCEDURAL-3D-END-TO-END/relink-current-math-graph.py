"""🧠 Rebuild the affected native Neural and Flow units against a single dependency closure."""
from pathlib import Path
import json
import os
import subprocess

root=Path.cwd().resolve()
g=Path(__file__).resolve().parent/"🗑️generated/bridge-current"
capture=json.loads((g/"geometry-test-command.json").read_text())
args=capture["args"]
directories=[Path(args[i+1][11:]) for i,flag in enumerate(args[:-1]) if flag=="-L" and args[i+1].startswith("dependency=")]
graph={}
for dest in directories:
    for fp in (dest.parent/"fingerprint").glob("lib-*.json"):
        key=fp.with_suffix("").read_text().strip()
        name=fp.name[4:-5]
        libs=[p for p in dest.glob("lib"+name+"-*") if p.suffix in (".rlib",".dylib")]
        if len(libs)!=1:continue
        record=json.loads(fp.read_text())
        graph[key]=(dest,name,libs[0],record)
out=g/"current-math-graph-variadic"
out.mkdir(exist_ok=True)
target=next(k for k,(p,n,_,_) in graph.items() if n=="semio_os_flow_extensions")
changes={k for k,(d,_,_,_) in graph.items() if d.parent.parent.name in ("semio-os-kernel-neural-engine-model","semio-os-kernel-neural-engine-registry")}
built={}
def build(key):
    if key in built:return built[key]
    dest,name,lib,record=graph[key]
    children={n:build(f.to_bytes(8,"little").hex()) for _,n,_,f in record["deps"] if f.to_bytes(8,"little").hex() in graph}
    if key not in changes and all(p.parent!=out for p in children.values()):
        built[key]=lib
        return lib
    source=Path(next(dest.glob("*.d")).open().readline().split(": ",1)[1].strip().split(" ",1)[0])
    source=Path(str(source).replace(str(root/".cache/g3d-native/s")+"/",str(root)+"/",1)) if dest.parent.parent.name in ("semio-os-kernel-neural-engine-model","semio-os-kernel-neural-engine-registry","semio-os-kernel-neural-engine","semio-os-flow-extensions") else source
    manifest=Path(str(source).split("/📦️packages/🦀️rust/")[0])/"📦️packages/🦀️rust"
    package=dest.parent.parent.name
    cmd=[args[0],"--crate-name",name,"--edition=2021",str(source),"--crate-type=rlib","--emit=metadata,link","-Cdebuginfo=0","-Cmetadata=procedural_math_native","-L","dependency="+str(out),"--out-dir",str(out)]
    for feature in json.loads(record["features"]):cmd+=["--cfg",'feature="'+feature+'"']
    for alias,link in children.items():
        cmd+=["--extern",alias+"="+str(link)]
        meta=link.with_suffix(".rmeta")
        if meta.exists():cmd+=["--extern",alias+"="+str(meta)]
    for folder in directories:cmd+=["-L","dependency="+str(folder)]
    env={**os.environ,**capture["env"],"CARGO_CRATE_NAME":name,"CARGO_MANIFEST_DIR":str(manifest),"CARGO_PKG_NAME":package}
    print("[DEBUG] Building "+package,flush=True)
    subprocess.run(cmd,env=env,check=True)
    built[key]=out/("lib"+name+".rlib")
    return built[key]
assert changes, "Neural engine must be present"
assert build(target).parent==out
